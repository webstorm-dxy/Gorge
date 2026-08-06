#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;

use gorge_core::diagnostics::{Diagnostics, Span};
use gorge_core::virtual_machine::ir::*;
use gorge_core::objective::bytecode::{DelegateImpl, InjectorConstField, InjectorConstantDef};
use crate::compiler::InjectorFieldDef;

use crate::frontend::ast::*;
use crate::compile_context::symbol::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockKind { For, While, DoWhile, Switch, If, Else }

struct PendingLeave { code_index: usize, targets: std::collections::VecDeque<BreakTarget>, is_break: bool, done: bool }

struct BlockCtx { kind: BlockKind, is_else: bool }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatchLevel { Exact, Castable, None }


#[derive(Debug, Default)]
struct ParamIndexCounters { int: usize, float: usize, bool: usize, string: usize, object: usize }
impl ParamIndexCounters {
    fn reset(&mut self) { *self = Self::default(); }
    fn next(&mut self, vt: ValueType) -> usize {
        let c = match vt {
            ValueType::Int => &mut self.int, ValueType::Float => &mut self.float,
            ValueType::Bool => &mut self.bool, ValueType::String => &mut self.string,
            ValueType::Object => &mut self.object,
        };
        let cur = *c; *c += 1; cur
    }
}

/// 代码生成器
///
/// 将 AST 表达式和语句转换为三地址码（IntermediateCode）序列。
/// 每个方法体/构造方法体对应一个独立的 CodeGenerator 实例。
pub struct CodeGenerator<'a> {
    /// 对符号表的不可变引用
    pub symbol_table: &'a SymbolTable,
    /// 诊断收集器
    pub diagnostics: &'a mut Diagnostics,
    /// 委托实现列表（Lambda 编译产生）
    pub delegate_impls: &'a mut Vec<DelegateImpl>,
    /// 各类注入器字段定义（类名 → 字段列表），供 `obj.^field` 按对象类型解析
    /// （对齐 C# 按对象声明类型查找注入器字段，而非仅当前类）
    pub injector_fields: &'a HashMap<String, Vec<InjectorFieldDef>>,
    /// 生成的 IR 指令序列
    pub codes: Vec<CodeWithSpan>,

    /// 局部变量名 → Address 映射
    local_vars: HashMap<String, Address>,
    /// 参数名 → Address 映射（参数使用固定偏移）
    param_vars: HashMap<String, Address>,
    /// 每种值类型的下一个局部变量索引（临时变量共用此计数器）
    next_local: HashMap<ValueType, usize>,
    /// 委托变量名 → delegate_impl 索引
    delegate_vars: HashMap<String, usize>,
    var_class: HashMap<String, String>,
    var_types: HashMap<String, TypeInfo>,
    /// 注入器数组变量名 → 元素类型名（如 "listB" → "int"），用于 `new listB[n]` 数组构造
    var_injector_array_elem: HashMap<String, String>,
    /// 注入器数组变量名 → 常量池索引，用于 `new listB[n]` 的数组元素初始化
    var_injector_const_idx: HashMap<String, usize>,
    field_info: HashMap<String, (usize, ValueType)>,
    field_types: HashMap<String, TypeInfo>,
    injector_field_info: HashMap<String, (usize, ValueType)>,
    param_counters: ParamIndexCounters,
    pub current_class_name: Option<String>,
    /// 当前类的作用域（含正确的 using_scopes），用于跨命名空间类型查找
    pub current_class_scope: Option<ScopeId>,
    /// 当前类的泛型参数名列表（J1）
    current_generic_params: Vec<String>,
    /// 泛型参数 → 具体类型替换映射（T6 实例化）
    /// 如 `class Foo<T>` 实例化为 `Foo<int>` 时映射 { "T" → TypeInfo::Int }
    generic_substitutions: HashMap<String, TypeInfo>,
    block_stack: Vec<BlockCtx>,
    pending_leaves: Vec<PendingLeave>,
    pub injector_constants: Vec<InjectorConstantDef>,
    /// 构造时注入的种子常量数（类级池既有条目），`take_injector_constants`
    /// 只回收该长度之后的**新增**条目，避免把种子重复合并进类级池
    seed_injector_constants_len: usize,

    /// 当前代码块上下文（B-4），影响注入器字段读写权限校验
    pub current_block_context: BlockContext,
}

impl<'a> CodeGenerator<'a> {
    /// 创建新的代码生成器实例
    ///
    /// `seed_injector_constants` 为该类此前代码生成单元（方法/构造/字段初始化器/
    /// 隐藏方法）已积累的注入器常量，新单元从该长度续写索引——注入器常量的
    /// 索引在**类内**连续分配，保证类内各方法的 `LoadInjectorConstant` 引用
    /// 与最终合并进 `CompiledClass.injector_constants` 的条目一一对应
    /// （P0-7 按类持有）。
    pub fn new(
        symbol_table: &'a SymbolTable,
        diagnostics: &'a mut Diagnostics,
        delegate_impls: &'a mut Vec<DelegateImpl>,
        injector_fields: &'a HashMap<String, Vec<InjectorFieldDef>>,
        seed_injector_constants: Vec<InjectorConstantDef>,
    ) -> Self {
        let mut next_local = HashMap::new();
        next_local.insert(ValueType::Int, 0);
        next_local.insert(ValueType::Float, 0);
        next_local.insert(ValueType::Bool, 0);
        next_local.insert(ValueType::String, 0);
        next_local.insert(ValueType::Object, 1); // 0 保留给 this 指针

        let seed_injector_constants_len = seed_injector_constants.len();
        Self {
            symbol_table,
            diagnostics,
            delegate_impls,
            injector_fields,
            codes: Vec::new(),
            local_vars: HashMap::new(),
            param_vars: HashMap::new(),
            next_local,
            delegate_vars: HashMap::new(),
            var_class: HashMap::new(),
            var_types: HashMap::new(),
            var_injector_array_elem: HashMap::new(),
            var_injector_const_idx: HashMap::new(),
            field_info: HashMap::new(),
            field_types: HashMap::new(),
            injector_field_info: HashMap::new(),
            param_counters: ParamIndexCounters::default(),
            current_class_name: None,
            current_class_scope: None,
            current_generic_params: Vec::new(),
            generic_substitutions: HashMap::new(),
            block_stack: Vec::new(),
            pending_leaves: Vec::new(),
            injector_constants: seed_injector_constants,
            seed_injector_constants_len,
            current_block_context: BlockContext::Instance,
        }
    }

    /// 取出本代码生成单元**新增**的注入器常量（供调用方合并进类级常量池）。
    ///
    /// 构造时注入的种子常量（类级池既有条目）不取出，只回收本单元新分配的
    /// 尾部条目，避免种子被重复合并进类级池；与 `into_codes` 配合使用：
    /// 先取常量、再 `into_codes` 消费自身（P0-7 按类持有）。
    pub fn take_injector_constants(&mut self) -> Vec<InjectorConstantDef> {
        let seed_len = self.seed_injector_constants_len;
        if self.injector_constants.len() <= seed_len {
            return Vec::new();
        }
        self.injector_constants.split_off(seed_len)
    }

    /// 获取类类型查找的作用域
    ///
    /// 优先使用当前类作用域（含正确的跨命名空间 using_scopes），
    /// 回退到 global_scope。
    fn class_lookup_scope(&self) -> ScopeId {
        self.current_class_scope.unwrap_or(self.symbol_table.global_scope)
    }

    /// 为方法参数注册地址
    pub fn register_parameters(&mut self, params: &[(String, ValueType)]) {
        for (name, vt) in params {
            let addr = self.alloc_local(*vt);
            self.param_vars.insert(name.clone(), addr);
            self.local_vars.insert(name.clone(), addr);
        }
    }

    /// 记录变量/参数的类名，用于实例方法解析
    pub fn register_var_class(&mut self, name: &str, class_name: &str) {
        self.var_class.insert(name.to_string(), class_name.to_string());
    }

    /// 记录变量/参数的完整类型信息
    pub fn register_var_type(&mut self, name: &str, ty: TypeInfo) {
        self.var_types.insert(name.to_string(), ty);
    }

    /// 布置一个调用参数：按值类型分组分配索引
    fn emit_set_param(&mut self, arg_op: Operand, span: Span) {
        let arg_vt = Self::operand_value_type(&arg_op);
        let index = self.param_counters.next(arg_vt);
        let param_addr = Address::new(ValueType::Int, index);
        let op = match arg_vt {
            ValueType::Int => IntermediateOperator::SetIntParameter,
            ValueType::Float => IntermediateOperator::SetFloatParameter,
            ValueType::Bool => IntermediateOperator::SetBoolParameter,
            ValueType::String => IntermediateOperator::SetStringParameter,
            ValueType::Object => IntermediateOperator::SetObjectParameter,
        };
        self.emit(IntermediateCode::new(op, arg_op, None, Some(param_addr)), span);
    }

    /// 获取方法体中的所有 IR 指令
    pub fn into_codes(self) -> Vec<CodeWithSpan> {
        self.codes
    }

    pub fn emit_super_constructor_call(&mut self, super_class_name: &str, base_arguments: &[Expression], span: Span) {
        let arg_ops: Vec<Operand> = base_arguments.iter().map(|a| self.generate_expression(a)).collect();
        self.param_counters.reset();
        for arg_op in &arg_ops {
            self.emit_set_param(arg_op.clone(), span);
        }
        self.emit(IntermediateCode::new(
            IntermediateOperator::InvokeSuperConstructor(0),
            Operand::int(base_arguments.len() as i64),
            Some(Operand::string(super_class_name.to_string())),
            None,
        ), span);
    }

    fn unresolved_leaves(&self) -> Vec<Span> {
        self.pending_leaves.iter().filter(|l| !l.done).filter_map(|l| self.codes.get(l.code_index).map(|c| c.span)).collect()
    }

    pub fn report_unresolved_leaves(&mut self) {
        for span in self.unresolved_leaves() {
            self.diagnostics.emit_error(span, "break/continue 的离块目标非法。".to_string());
        }
    }

    // ==================== break/continue 离块回填 ====================
    //
    // 对齐 C# `LeaveBlockBackPatchTask` + `CodeBlockScope.BackPatch` 语义：
    // - `break/continue` 生成占位 `Jump(0)` 并登记为待回填任务；
    // - 每个控制流块（while/for/do-while/switch/if/else）生成结束时调用
    //   `backpatch_block`，对所有未完成任务尝试消解一层；
    // - 队列式多目标（如 `break for for`）：队首目标满足即出队，继续用下一目标；
    // - `if`/`else` 也算一层（与 C# 一致），plain `break` 会被最内层 if 捕获；
    // - 方法体生成结束后仍未回填的任务由 `report_unresolved_leaves` 报编译错误。

    /// 发出一条离块占位指令（break/continue）。
    ///
    /// 生成占位 `Jump(0)`，并把该指令连同离块目标队列登记为待回填任务，
    /// 具体跳转目标在外层控制流块生成结束时由 `backpatch_block` 回填。
    ///
    /// # 参数
    /// - `is_break`：`true` 为 break，`false` 为 continue
    /// - `targets`：离块目标序列（层数或关键字）
    /// - `span`：源码位置，用于错误报告
    fn emit_leave(&mut self, is_break: bool, targets: &[BreakTarget], span: Span) {
        let code_index = self.codes.len();
        self.emit(IntermediateCode::jump(0), span);
        self.pending_leaves.push(PendingLeave {
            code_index,
            targets: targets.iter().cloned().collect(),
            is_break,
            done: false,
        });
    }

    /// 在一个控制流块生成结束时尝试回填该块内登记的离块任务。
    ///
    /// 只处理 `pending_leaves` 中下标位于 `[since, until)` 的任务（即本块内部
    /// 产生的 break/continue），块外及兄弟块的任务不受影响。对每个未完成任务的
    /// 队首目标消解一层：
    /// - `ByLayer(n)`：无论块类型，层数一律减 1，减到 0 则本块即目标；
    /// - `ByKeyword(k)`：块类型（含 else 判定）匹配时消解，否则跳过本块、等待外层；
    /// 队列清空后，按任务是 break 还是 continue 回填到相应落点。
    ///
    /// # 参数
    /// - `kind`：当前块的种类
    /// - `is_else`：当前块是否为 else 块（仅 If 相关块有意义）
    /// - `break_index`：break 应跳转到的代码索引（通常为块尾）
    /// - `continue_index`：continue 应跳转到的代码索引（循环续点）；
    ///   对非循环块（switch/if/else）传入 `None`，此时 continue 命中本块时
    ///   退化为跳到块尾（fall-through，对齐 C# 非 else 块行为）。
    /// - `since`/`until`：只处理 `pending_leaves[since..until]` 范围的任务
    ///   （进入本块前记录 `pending_leaves.len()` 作为 `since`，
    ///   离开时用 `pending_leaves.len()` 作为 `until`；兄弟块用各自区间避免重复消解）。
    fn backpatch_block(
        &mut self,
        kind: BlockKind,
        is_else: bool,
        break_index: usize,
        continue_index: Option<usize>,
        since: usize,
        until: usize,
    ) {
        let upper = until.min(self.pending_leaves.len());
        for idx in since..upper {
            let leave = &mut self.pending_leaves[idx];
            if leave.done {
                continue;
            }
            let matched = match leave.targets.front_mut() {
                Some(BreakTarget::ByLayer(n)) => {
                    // 层数目标：无论块类型，每经过一个块都消解一层
                    *n = n.saturating_sub(1);
                    if *n == 0 {
                        leave.targets.pop_front();
                        true
                    } else {
                        false
                    }
                }
                Some(BreakTarget::ByKeyword(k)) => {
                    // 关键字目标：仅当块类型匹配才消解，否则跳过本块等待外层
                    let hit = Self::keyword_matches_block(k, kind, is_else);
                    if hit {
                        leave.targets.pop_front();
                    }
                    hit
                }
                None => false,
            };
            if matched && leave.targets.is_empty() {
                // 所有目标已消解，回填占位跳转
                let target = if leave.is_break {
                    break_index
                } else {
                    // continue 命中：循环块跳到续点，非循环块退化为跳到块尾
                    continue_index.unwrap_or(break_index)
                };
                if let Some(code) = self.codes.get_mut(leave.code_index) {
                    code.code.operator = IntermediateOperator::Jump(target);
                }
                leave.done = true;
            }
        }
    }

    /// 判断关键字离块目标是否匹配当前块类型。
    ///
    /// `else` 关键字匹配 else 块；`if` 关键字匹配非 else 的 If 块；
    /// 其余关键字按块种类一一对应。
    fn keyword_matches_block(keyword: &str, kind: BlockKind, is_else: bool) -> bool {
        match keyword {
            "for" => kind == BlockKind::For,
            "while" => kind == BlockKind::While,
            "do" => kind == BlockKind::DoWhile,
            "switch" => kind == BlockKind::Switch,
            "else" => is_else,
            "if" => kind == BlockKind::If && !is_else,
            _ => false,
        }
    }

    // ==================== 临时变量 / 局部变量管理 ====================

    /// 分配一个临时变量地址（与局部变量共享计数器，确保不冲突）
    pub(crate) fn alloc_temp(&mut self, value_type: ValueType) -> Address {
        self.alloc_local(value_type)
    }

    /// 分配一个局部变量地址
    fn alloc_local(&mut self, value_type: ValueType) -> Address {
        let next = self.next_local.get_mut(&value_type).unwrap();
        let index = *next;
        *next += 1;
        Address::new(value_type, index)
    }

    /// 为局部变量声明分配地址
    fn declare_local(&mut self, name: &str, value_type: ValueType) -> Address {
        let addr = self.alloc_local(value_type);
        self.local_vars.insert(name.to_string(), addr);
        addr
    }

    /// 查找变量地址（先查局部变量，再查参数）
    fn lookup_var(&self, name: &str) -> Option<Address> {
        self.local_vars.get(name).copied()
    }

    /// 发出一条 IR 指令
    pub(crate) fn emit(&mut self, code: IntermediateCode, span: Span) {
        self.codes.push(CodeWithSpan::new(code, span));
    }

    /// 设置类上下文，填充字段名→(偏移,类型) 映射（包含继承字段）
    ///
    /// 同时填充 `field_types`（字段名→完整类型信息），供「this 字段再取其成员」
    /// （如 `lane.noteReferenceNode`）的接收者类型推导使用。
    pub fn set_class_context(&mut self, class_name: &str) {
        self.current_class_name = Some(class_name.to_string());
        self.current_class_scope = None;
        self.field_info.clear();
        self.field_types.clear();
        self.current_generic_params.clear();
        let starting_class_id = match self.symbol_table.find_class_by_name(class_name) {
            Some((cid, scope)) => {
                self.current_class_scope = Some(scope);
                Some(cid)
            }
            None => self.symbol_table.lookup_class(self.symbol_table.global_scope, class_name),
        };
        if let Some(mut class_id) = starting_class_id {
            loop {
                let class_info = self.symbol_table.classes.get(class_id.0);
                for &field_id in &class_info.fields {
                    let fi = self.symbol_table.fields.get(field_id.0);
                    let vt = Self::type_to_value_type(&fi.field_type);
                    let offset = fi.offset.unwrap_or(0);
                    self.field_info.entry(fi.name.clone()).or_insert((offset, vt));
                    self.field_types
                        .entry(fi.name.clone())
                        .or_insert_with(|| fi.field_type.clone());
                }
                // J1: 收集泛型参数名
                for gp in &class_info.generic_params {
                    if !self.current_generic_params.contains(gp) {
                        self.current_generic_params.push(gp.clone());
                    }
                }
                // 继续处理父类
                class_id = match class_info.super_class {
                    Some(super_id) => super_id,
                    None => break,
                };
            }
        }
    }

    /// 生成读取字段的 IR 操作码
    fn load_field_op(value_type: ValueType, field_index: usize) -> IntermediateOperator {
        match value_type {
            ValueType::Int => IntermediateOperator::LoadIntField(field_index),
            ValueType::Float => IntermediateOperator::LoadFloatField(field_index),
            ValueType::Bool => IntermediateOperator::LoadBoolField(field_index),
            ValueType::String => IntermediateOperator::LoadStringField(field_index),
            ValueType::Object => IntermediateOperator::LoadObjectField(field_index),
        }
    }

    /// 生成写入字段的 IR 操作码
    pub(crate) fn set_field_op(value_type: ValueType, field_index: usize) -> IntermediateOperator {
        match value_type {
            ValueType::Int => IntermediateOperator::SetIntField(field_index),
            ValueType::Float => IntermediateOperator::SetFloatField(field_index),
            ValueType::Bool => IntermediateOperator::SetBoolField(field_index),
            ValueType::String => IntermediateOperator::SetStringField(field_index),
            ValueType::Object => IntermediateOperator::SetObjectField(field_index),
        }
    }

    /// 设置注入器上下文，填充注入器字段名→(字段索引,值类型) 映射
    pub fn set_injector_context(&mut self, fields: &[(String, ValueType)]) {
        self.injector_field_info.clear();
        for (i, (name, vt)) in fields.iter().enumerate() {
            self.injector_field_info.insert(name.clone(), (i, *vt));
        }
    }

    /// 生成读取注入器字段的 IR 操作码
    fn load_injector_field_op(value_type: ValueType, field_index: usize) -> IntermediateOperator {
        match value_type {
            ValueType::Int => IntermediateOperator::LoadIntInjectorField(field_index),
            ValueType::Float => IntermediateOperator::LoadFloatInjectorField(field_index),
            ValueType::Bool => IntermediateOperator::LoadBoolInjectorField(field_index),
            ValueType::String => IntermediateOperator::LoadStringInjectorField(field_index),
            ValueType::Object => IntermediateOperator::LoadObjectInjectorField(field_index),
        }
    }

    /// 生成写入注入器字段的 IR 操作码（调用者负责构造正确的 left/right 操作数）
    fn set_injector_field_op(value_type: ValueType, field_index: usize) -> IntermediateOperator {
        match value_type {
            ValueType::Int => IntermediateOperator::SetIntInjectorField(field_index),
            ValueType::Float => IntermediateOperator::SetFloatInjectorField(field_index),
            ValueType::Bool => IntermediateOperator::SetBoolInjectorField(field_index),
            ValueType::String => IntermediateOperator::SetStringInjectorField(field_index),
            ValueType::Object => IntermediateOperator::SetObjectInjectorField(field_index),
        }
    }

    // ==================== 类型推导 ====================

    /// 编译期类型自动转换判定
    fn can_auto_cast(&self, from: &TypeInfo, to: &TypeInfo) -> bool {
        if from == to { return true; }
        if matches!(from, TypeInfo::Int) && matches!(to, TypeInfo::Float) { return true; }
        matches!(from, TypeInfo::Unresolved) || matches!(to, TypeInfo::Unresolved)
    }

    /// 将一组方法参数与实参类型做三级匹配
    fn match_params(&self, params: &[ParameterId], arg_types: &[TypeInfo]) -> MatchLevel {
        if params.len() != arg_types.len() { return MatchLevel::None; }
        let mut all_exact = true;
        for (pid, at) in params.iter().zip(arg_types.iter()) {
            let pt = &self.symbol_table.parameters.get(pid.0).param_type;
            if pt == at || matches!(at, TypeInfo::Unresolved) { continue; }
            all_exact = false;
            if !self.can_auto_cast(at, pt) { return MatchLevel::None; }
        }
        if all_exact { MatchLevel::Exact } else { MatchLevel::Castable }
    }

    /// 在指定类中解析实例方法（含重载）
    fn resolve_instance_method(&self, class_name: &str, method: &str, arg_types: &[TypeInfo]) -> Result<Option<(usize, TypeInfo)>, ()> {
        let scope = self.class_lookup_scope();
        let mut class_id = match self.symbol_table.lookup_class(scope, class_name) { Some(c) => c, None => return Ok(None) };
        loop {
            let class_info = self.symbol_table.classes.get(class_id.0);
            let mut exact = Vec::new(); let mut castable = Vec::new();
            for (i, &method_id) in class_info.methods.iter().enumerate() {
                let mi = self.symbol_table.methods.get(method_id.0);
                if mi.name != method || mi.is_static { continue; }
                // native 类的方法编号按同类方法独立计数（仅统计实例方法）
                let global_id = if class_info.is_native {
                    class_info.methods.iter().take(i).filter(|&&mid| {
                        !self.symbol_table.methods.get(mid.0).is_static
                    }).count()
                } else {
                    class_info.method_start_id + i
                };
                let return_type = mi.return_type.clone();
                match self.match_params(&mi.parameters, arg_types) {
                    MatchLevel::Exact => exact.push((global_id, return_type)),
                    MatchLevel::Castable => castable.push((global_id, return_type)),
                    MatchLevel::None => {}
                }
            }
            if let Some(hit) = exact.first() { return Ok(Some(hit.clone())); }
            if castable.len() == 1 { return Ok(Some(castable[0].clone())); }
            if castable.len() > 1 { return Err(()); }
            match class_info.super_class { Some(sid) => class_id = sid, None => return Ok(None) }
        }
    }

    /// 编译期强制转换判定：双向任一可自动转换即可
    fn can_cast(&self, from: &TypeInfo, to: &TypeInfo) -> bool {
        self.can_auto_cast(from, to) || self.can_auto_cast(to, from)
    }

    /// 校验方法调用的实参数量是否与某个同名重载匹配（对齐 C# `UnexpectedParameterCountException`）。
    ///
    /// 沿类的继承链查找名为 `method` 的方法（按 `want_static` 过滤静态/实例）：
    /// - 若存在同名方法但**没有任何重载**的形参个数等于 `arg_count`，报编译错误并返回 `true`；
    /// - 若无同名方法（可能是 native/未解析，交由运行时分派）或存在数量匹配的重载，返回 `false`。
    fn check_method_arg_count(&mut self, class_name: &str, method: &str, want_static: bool, arg_count: usize, span: Span) -> bool {
        let scope = self.class_lookup_scope();
        let mut class_id = match self.symbol_table.lookup_class(scope, class_name) { Some(c) => c, None => return false };
        let mut name_found = false;
        let mut arities: Vec<usize> = Vec::new();
        loop {
            let class_info = self.symbol_table.classes.get(class_id.0);
            for &method_id in &class_info.methods {
                let mi = self.symbol_table.methods.get(method_id.0);
                if mi.name == method && mi.is_static == want_static {
                    name_found = true;
                    arities.push(mi.parameters.len());
                }
            }
            match class_info.super_class { Some(sid) => class_id = sid, None => break }
        }
        if name_found && !arities.contains(&arg_count) {
            let expected = arities[0];
            self.diagnostics.emit_error(
                span,
                format!("方法 `{}` 参数数量错误，期望 {} 个，实际 {} 个。", method, expected, arg_count),
            );
            return true;
        }
        false
    }

    /// 把 AST 的 TypeRef 解析为 TypeInfo
    fn resolve_type_ref(&self, tr: &TypeRef) -> TypeInfo {
        match tr {
            TypeRef::Simple { name, .. } => {
                // J1: 检测泛型参数名 → 若有实例化替换则返回具体类型
                if self.current_generic_params.contains(name) {
                    return self.generic_substitutions.get(name)
                        .cloned()
                        .unwrap_or_else(|| TypeInfo::GenericParam(name.clone()));
                }
                match name.as_str() {
                    "int" => TypeInfo::Int, "float" => TypeInfo::Float, "bool" => TypeInfo::Bool,
                    "string" => TypeInfo::String, "void" => TypeInfo::Void,
                    // lookup_qualified 支持 `GorgeFramework.Element` 限定名
                    _ => match self.symbol_table.lookup_qualified(self.class_lookup_scope(), name) {
                        Some((SymbolEntry::Class(cid), _)) => TypeInfo::Object(*cid),
                        _ => TypeInfo::Unresolved,
                    },
                }
            },
            // J1: 泛型实例化 `Foo<int>`
            TypeRef::Generic { name, type_args, .. } => {
                let base = self.resolve_type_ref(&TypeRef::simple(name, Span::new(0, 0, 0, 0, 0)));
                let args: Vec<TypeInfo> = type_args.iter()
                    .map(|t| self.resolve_type_ref(t))
                    .collect();
                TypeInfo::GenericInstance { base: Box::new(base), type_args: args }
            },
            _ => TypeInfo::Unresolved,
        }
    }

    /// 推导表达式的完整类型
    fn infer_type(&self, expr: &Expression) -> TypeInfo {
        match expr {
            Expression::Literal(Literal::Int(_), _) => TypeInfo::Int,
            Expression::Literal(Literal::Float(_), _) => TypeInfo::Float,
            Expression::Literal(Literal::Bool(_), _) => TypeInfo::Bool,
            Expression::Literal(Literal::String(_), _) => TypeInfo::String,
            Expression::Identifier(name, _) => self.var_types.get(name).cloned().or_else(|| self.field_types.get(name).cloned()).unwrap_or(TypeInfo::Unresolved),
            Expression::This(_) => self.current_class_name.as_ref().and_then(|n| self.symbol_table.lookup_class(self.class_lookup_scope(), n)).map(TypeInfo::Object).unwrap_or(TypeInfo::Unresolved),
            Expression::New { class_type, .. } => self.resolve_type_ref(class_type),
            Expression::Cast { target_type, .. } => self.resolve_type_ref(target_type),
            Expression::Binary { left, operator, .. } => {
                use BinaryOp::*;
                match operator { Less|LessEqual|Greater|GreaterEqual|Equal|NotEqual|LogicAnd|LogicOr => TypeInfo::Bool, _ => self.infer_type(left) }
            }
            // 委托调用 d1(arg) → 从委托变量推导返回类型
            Expression::MethodCall { receiver, method, arguments, .. } => {
                if let Expression::Identifier(name, _) = receiver.as_ref() {
                    if let Some(idx) = self.delegate_vars.get(name) {
                        if let Some(di) = self.delegate_impls.get(*idx) {
                            return match di.return_type {
                                ValueType::Int => TypeInfo::Int,
                                ValueType::Float => TypeInfo::Float,
                                ValueType::Bool => TypeInfo::Bool,
                                ValueType::String => TypeInfo::String,
                                ValueType::Object => TypeInfo::Unresolved,
                            };
                        }
                    }
                }
                let argument_types: Vec<TypeInfo> = arguments.iter()
                    .map(|argument| self.infer_type(argument))
                    .collect();
                self.resolve_instance_method_return_type(receiver, method, &argument_types)
                    .unwrap_or(TypeInfo::Unresolved)
            }
            // 直接委托调用 d1(arg)（语法为 StaticMethodCall，class_name 为空）
            Expression::StaticMethodCall { method, .. } => {
                if let Some(idx) = self.delegate_vars.get(method) {
                    if let Some(di) = self.delegate_impls.get(*idx) {
                        return match di.return_type {
                            ValueType::Int => TypeInfo::Int,
                            ValueType::Float => TypeInfo::Float,
                            ValueType::Bool => TypeInfo::Bool,
                            ValueType::String => TypeInfo::String,
                            ValueType::Object => TypeInfo::Unresolved,
                        };
                    }
                }
                TypeInfo::Unresolved
            }
            // 成员访问优先按接收者字段类型推导；枚举成员访问保留专门回退逻辑。
            Expression::MemberAccess { object, .. } => {
                if let Some(ty) = self.resolve_object_type(expr) {
                    return ty;
                }
                if let Expression::Identifier(name, _) = object.as_ref() {
                    if self.lookup_var(name).is_none() && !self.field_info.contains_key(name) {
                        if let Some(enum_id) =
                            self.symbol_table.find_enum_by_name(self.class_lookup_scope(), name)
                        {
                            return TypeInfo::Enum(enum_id);
                        }
                    }
                }
                TypeInfo::Unresolved
            }
            // 数组访问的表达式类型就是数组元素类型（对齐 C# ArrayAccessExpression）。
            Expression::ArrayAccess { array, .. } => self.resolve_object_type(array)
                .and_then(|ty| match ty {
                    TypeInfo::Array(element_type) => Some(*element_type),
                    _ => None,
                })
                .unwrap_or(TypeInfo::Unresolved),
            _ => TypeInfo::Unresolved,
        }
    }

    /// 解析实例方法调用的返回类型，支持成员链、数组元素等任意可推导接收者。
    fn resolve_instance_method_return_type(
        &self,
        receiver: &Expression,
        method: &str,
        argument_types: &[TypeInfo],
    ) -> Option<TypeInfo> {
        let receiver_type = self.resolve_object_type(receiver).or_else(|| match receiver {
            Expression::New { class_type, .. } => Some(self.resolve_type_ref(class_type)),
            _ => None,
        })?;

        match receiver_type {
            TypeInfo::Object(class_id) => {
                let class_name = &self.symbol_table.classes.get(class_id.0).name;
                self.resolve_instance_method(class_name, method, argument_types)
                    .ok()
                    .flatten()
                    .map(|(_, return_type)| return_type)
            }
            TypeInfo::Interface(interface_id) => self
                .resolve_interface_method(interface_id, method, argument_types)
                .map(|(_, _, return_type)| return_type),
            _ => None,
        }
    }

    /// 在类中查找字段类型（含继承链）
    fn lookup_field_type_in(&self, obj_ty: &TypeInfo, field: &str) -> Option<TypeInfo> {
        let mut class_id = match obj_ty { TypeInfo::Object(cid) => Some(*cid), _ => None };
        while let Some(cid) = class_id {
            let ci = self.symbol_table.classes.get(cid.0);
            for &fid in &ci.fields { let fi = self.symbol_table.fields.get(fid.0); if fi.name == field { return Some(fi.field_type.clone()); } }
            class_id = ci.super_class;
        }
        None
    }

    /// 在接口中解析方法
    fn resolve_interface_method(&self, iface_id: InterfaceId, method: &str, arg_types: &[TypeInfo]) -> Option<(usize, String, TypeInfo)> {
        let iface = self.symbol_table.interfaces.get(iface_id.0);
        for (local_idx, &mid) in iface.methods.iter().enumerate() {
            let mi = self.symbol_table.methods.get(mid.0);
            if mi.name == method && self.match_params(&mi.parameters, arg_types) != MatchLevel::None {
                return Some((local_idx, iface.name.clone(), mi.return_type.clone()));
            }
        }
        None
    }

    /// 生成强制转换代码
    fn generate_cast(&mut self, target_type: &TypeRef, expression: &Expression, span: Span) -> Operand {
        let src_ty = self.infer_type(expression);
        let dst_ty = self.resolve_type_ref(target_type);
        let src_op = self.generate_expression(expression);
        if !matches!(src_ty, TypeInfo::Unresolved) && !matches!(dst_ty, TypeInfo::Unresolved) && !self.can_cast(&src_ty, &dst_ty) {
            self.diagnostics.emit_error(span, "非法的强制类型转换".to_string()); return src_op;
        }
        let sv = Self::type_to_value_type(&src_ty); let dv = Self::type_to_value_type(&dst_ty);
        if sv == dv { return src_op; }
        let (op, result) = match (sv, dv) {
            (ValueType::Int, ValueType::Float) => (IntermediateOperator::IntToFloat, self.alloc_temp(ValueType::Float)),
            (ValueType::Float, ValueType::Int) => (IntermediateOperator::FloatToInt, self.alloc_temp(ValueType::Int)),
            (ValueType::Int, ValueType::String) => (IntermediateOperator::IntCastToString, self.alloc_temp(ValueType::String)),
            (ValueType::Float, ValueType::String) => (IntermediateOperator::FloatCastToString, self.alloc_temp(ValueType::String)),
            (ValueType::Bool, ValueType::String) => (IntermediateOperator::BoolCastToString, self.alloc_temp(ValueType::String)),
            (ValueType::Object, ValueType::Object) => (IntermediateOperator::ObjectCastToObject, self.alloc_temp(ValueType::Object)),
            _ => return src_op,
        };
        self.emit(IntermediateCode::new(op, src_op, None, Some(result)), span);
        Operand::Address(result)
    }

    /// 将操作数从源值类型隐式转换到目标值类型，返回转换后的操作数。
    ///
    /// 用于二元运算的操作数类型提升（对齐 C# `CommonImmediateCodes.AppendAutoCastCode` +
    /// 加法级的 int/float/bool→string 扩展）。支持的隐式转换：
    /// - Int→Float（数值提升）
    /// - Int→String / Float→String / Bool→String（拼接场景）
    /// 源类型与目标类型相同则原样返回；无对应转换操作码时也原样返回（由调用方保证类型合法）。
    fn emit_cast_operand(&mut self, op: Operand, from: ValueType, to: ValueType, span: Span) -> Operand {
        if from == to {
            return op;
        }
        let (cast_op, result) = match (from, to) {
            (ValueType::Int, ValueType::Float) => {
                (IntermediateOperator::IntToFloat, self.alloc_temp(ValueType::Float))
            }
            (ValueType::Int, ValueType::String) => {
                (IntermediateOperator::IntCastToString, self.alloc_temp(ValueType::String))
            }
            (ValueType::Float, ValueType::String) => {
                (IntermediateOperator::FloatCastToString, self.alloc_temp(ValueType::String))
            }
            (ValueType::Bool, ValueType::String) => {
                (IntermediateOperator::BoolCastToString, self.alloc_temp(ValueType::String))
            }
            // 其余组合无隐式转换，原样返回（调用方已做类型校验）
            _ => return op,
        };
        self.emit(IntermediateCode::new(cast_op, op, None, Some(result)), span);
        Operand::Address(result)
    }
    /// 尝试将表达式求值为编译时常量（G2）
    fn try_eval_const(&self, expr: &Expression) -> Option<InjectorConstField> {
        match expr {
            Expression::Literal(Literal::Int(v), _) => Some(InjectorConstField::Int(String::new(), *v)),
            Expression::Literal(Literal::Float(v), _) => Some(InjectorConstField::Float(String::new(), *v)),
            Expression::Literal(Literal::Bool(v), _) => Some(InjectorConstField::Bool(String::new(), *v)),
            Expression::Literal(Literal::String(v), _) => Some(InjectorConstField::String(String::new(), v.clone())),
            Expression::Null(_) => {
                // null 是编译时常量，映射为 Object 引用 ID 0（VM 约定 0 表示 null），
                // 使含 null 字段的注入器对象/数组整体可折叠（H3A 真实谱面
                // `progressCurve : null` 场景）
                Some(InjectorConstField::Object(String::new(), 0))
            }
            Expression::InjectorObject { class_name, fields, .. } => {
                let nested: Vec<InjectorConstField> = fields.iter()
                    .filter_map(|(name, val_expr)| {
                        self.try_eval_const(val_expr).map(|mut cf| {
                            // 将字段名写入常量字段（标量变体的首槽位即字段名）；
                            // InjectObject 首槽位是类名，不可覆写，否则嵌套类型信息丢失
                            match &mut cf {
                                InjectorConstField::Int(n, _) | InjectorConstField::Float(n, _)
                                | InjectorConstField::Bool(n, _) | InjectorConstField::String(n, _)
                                | InjectorConstField::Object(n, _) => *n = name.clone(),
                                _ => {}
                            }
                            cf
                        })
                    })
                    .collect();
                if nested.len() == fields.len() {
                    Some(InjectorConstField::InjectObject(class_name.clone(), nested))
                } else {
                    None
                }
            }
            Expression::InjectorArray { elements, .. } => {
                let nested: Vec<InjectorConstField> = elements.iter()
                    .filter_map(|e| self.try_eval_const(e))
                    .collect();
                if nested.len() == elements.len() {
                    Some(InjectorConstField::Array(nested))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// 从 TypeInfo 推导 ValueType
    fn type_to_value_type(type_info: &TypeInfo) -> ValueType {
        match type_info {
            TypeInfo::Int => ValueType::Int,
            TypeInfo::Float => ValueType::Float,
            TypeInfo::Bool => ValueType::Bool,
            TypeInfo::String => ValueType::String,
            // 枚举在 VM 中以整数存储（Enum → Int）
            TypeInfo::Enum(_) => ValueType::Int,
            // 泛型参数保持 Object（字段偏移不展开），但让调用方能区分具体类型
            TypeInfo::GenericParam(_) | TypeInfo::GenericInstance { .. } => ValueType::Object,
            _ => ValueType::Object,
        }
    }

    /// 设置泛型参数→具体类型的替换映射（T6 实例化）
    ///
    /// 当 `class Foo<T>` 被实例化为 `Foo<int>` 时，将 `"T"` 映射到 `TypeInfo::Int`。
    /// `resolve_type_ref` 遇到泛型参数名时会查此映射返回具体类型。
    pub fn set_generic_substitutions(&mut self, subs: HashMap<String, TypeInfo>) {
        self.generic_substitutions = subs;
    }

    /// 从字面量推导 ValueType
    fn literal_value_type(literal: &Literal) -> ValueType {
        match literal {
            Literal::Int(_) => ValueType::Int,
            Literal::Float(_) => ValueType::Float,
            Literal::Bool(_) => ValueType::Bool,
            Literal::String(_) => ValueType::String,
        }
    }

    /// 从操作数推导 ValueType
    fn operand_value_type(operand: &Operand) -> ValueType {
        match operand {
            Operand::Address(addr) => addr.value_type,
            Operand::Immediate(val) => match val {
                ImmediateValue::Int(_) => ValueType::Int,
                ImmediateValue::Float(_) => ValueType::Float,
                ImmediateValue::Bool(_) => ValueType::Bool,
                ImmediateValue::String(_) => ValueType::String,
            },
        }
    }

    /// 从 TypeRef 推导 ValueType
    fn type_ref_to_value_type(tr: &TypeRef) -> ValueType {
        match tr {
            TypeRef::Simple { name, .. } => match name.as_str() {
                "int" => ValueType::Int,
                "float" => ValueType::Float,
                "bool" => ValueType::Bool,
                "string" => ValueType::String,
                _ => ValueType::Object,
            },
            _ => ValueType::Object,
        }
    }

    // ==================== 表达式代码生成 ====================

    /// 为表达式生成代码，返回结果操作数
    pub fn generate_expression(&mut self, expr: &Expression) -> Operand {
        match expr {
            Expression::Literal(lit, _span) => {
                match lit {
                    Literal::Int(v) => Operand::int(*v),
                    Literal::Float(v) => Operand::float(*v),
                    Literal::Bool(v) => Operand::boolean(*v),
                    Literal::String(v) => Operand::string(v),
                }
            }
            Expression::Identifier(name, span) => {
                match self.lookup_var(name) {
                    Some(addr) => Operand::Address(addr),
                    None => {
                        // 回退：在类字段中查找（this.field 隐式访问）
                        // 对齐 C# FieldReferenceExpression：先 LoadThis 再 LoadField
                        if let Some(&(offset, vt)) = self.field_info.get(name) {
                            let this_temp = self.alloc_temp(ValueType::Object);
                            self.emit(
                                IntermediateCode::new(
                                    IntermediateOperator::LoadThis,
                                    Operand::int(0), None, Some(this_temp),
                                ),
                                *span,
                            );
                            let temp = self.alloc_temp(vt);
                            let load_op = Self::load_field_op(vt, offset);
                            self.emit(
                                IntermediateCode::new(
                                    load_op,
                                    Operand::Address(this_temp),
                                    None,
                                    Some(temp),
                                ),
                                *span,
                            );
                            return Operand::Address(temp);
                        }
                        self.diagnostics.emit_error(
                            *span,
                            format!("未定义的变量 `{}`", name),
                        );
                        Operand::int(0)
                    }
                }
            }
            Expression::Binary { left, operator, right, span } => {
                self.generate_binary(left, *operator, right, *span)
            }
            Expression::Unary { operator, operand, span } => {
                self.generate_unary(*operator, operand, *span)
            }
            Expression::Assignment { target, operator: _, value, span } => {
                self.generate_assignment(target, value, *span)
            }
            Expression::MemberAccess { object, member, span: _ } => {
                self.generate_member_access(object, member)
            }
            Expression::MethodCall { receiver, method, arguments, span } => {
                self.generate_method_call(receiver, method, arguments, *span)
            }
            Expression::StaticMethodCall { class_name, method, arguments, span } => {
                // 数组构造 `new int[size]` / `new injVar[size]` (H3) 或
                // `new Type^[N]{ elem1, ... }` (H3A，带内联注入器元素)
                if method == "new_array" {
                    if arguments.len() > 1 {
                        // 带内联元素：要么折叠为常量数组，要么逐元素写回，绝不静默丢弃
                        return self.generate_array_constructor_with_elements(class_name, arguments, *span);
                    }
                    let size_op = if let Some(arg) = arguments.get(0) { self.generate_expression(arg) } else { Operand::int(0) };
                    let temp = self.alloc_temp(ValueType::Object);
                    // 若 class_name 是注入器数组变量，解析其元素类型（如 listB → int）
                    let elem_type = self.var_injector_array_elem.get(class_name)
                        .cloned()
                        .unwrap_or_else(|| class_name.clone());
                    self.emit(IntermediateCode::new(
                        IntermediateOperator::InvokeArrayConstructor,
                        size_op,
                        Some(Operand::string(elem_type.clone())),
                        Some(temp),
                    ), *span);
                    // 若 class_name 对应已知注入器常量，生成数组元素初始化指令
                    if let Some(&inj_idx) = self.var_injector_const_idx.get(class_name) {
                        let fields = self.injector_constants[inj_idx].fields.clone();
                        for (i, field) in fields.iter().enumerate() {
                            let val = match field {
                                InjectorConstField::Int(_, v) => Operand::int(*v),
                                InjectorConstField::Float(_, v) => Operand::float(*v),
                                InjectorConstField::Bool(_, v) => Operand::boolean(*v),
                                InjectorConstField::String(_, v) => Operand::string(v.clone()),
                                InjectorConstField::Object(_, _) | InjectorConstField::InjectObject(_, _) | InjectorConstField::Array(_) => continue,
                            };
                            self.param_counters.reset();
                            self.emit_set_param(Operand::int(i as i64), *span);
                            self.emit_set_param(val, *span);
                            self.emit(
                                IntermediateCode::new(
                                    IntermediateOperator::InvokeInstance(1), // array set 方法
                                    Operand::Address(temp),
                                    None,
                                    None,
                                ),
                                *span,
                            );
                        }
                    }
                    return Operand::Address(temp);
                }
                self.generate_delegate_call(method, arguments, *span)
            }
            Expression::New { class_type, arguments, injector, span } => {
                self.generate_new(class_type, arguments, injector.as_deref(), *span)
            }
            Expression::Conditional { condition, then_branch, else_branch, span } => {
                self.generate_conditional(condition, then_branch, else_branch.as_deref(), *span)
            }
            Expression::This(_span) => {
                // this 指向对象自身，地址 0 的 Object
                Operand::Address(Address::new(ValueType::Object, 0))
            }
            Expression::Null(_span) => {
                // Object 地址 0 固定保留给 this；null 必须使用默认值为 0 的独立临时槽位。
                Operand::Address(self.alloc_temp(ValueType::Object))
            }
            Expression::InjectorObject { class_name, fields, span } => {
                // 注入器对象：将字段值求值为编译时常量，存入常量池（G2）
                let const_fields: Vec<InjectorConstField> = fields
                    .iter()
                    .filter_map(|(name, val_expr)| {
                        self.try_eval_const(val_expr).map(|mut cf| {
                            // 将字段名写入常量字段（InjectObject 保留类名，见 try_eval_const）
                            match &mut cf {
                                InjectorConstField::Int(n, _) | InjectorConstField::Float(n, _)
                                | InjectorConstField::Bool(n, _) | InjectorConstField::String(n, _)
                                | InjectorConstField::Object(n, _) => *n = name.clone(),
                                _ => {}
                            }
                            cf
                        })
                    })
                    .collect();
                let idx = self.injector_constants.len();
                self.injector_constants.push(InjectorConstantDef { class_name: class_name.clone(), fields: const_fields });
                let temp = self.alloc_temp(ValueType::Object);
                self.emit(IntermediateCode::new(IntermediateOperator::LoadInjectorConstant(idx), Operand::int(0), None, Some(temp)), *span);
                Operand::Address(temp)
            }
            Expression::InjectorArray { elements, span } => {
                // 注入器数组：将所有元素求值为编译时常量，存入常量池（G2）
                let const_fields: Vec<InjectorConstField> = elements.iter()
                    .filter_map(|e| self.try_eval_const(e))
                    .collect();
                if const_fields.len() == elements.len() {
                    // 数组用特殊的类名标记，含元素类型前缀
                    let array_class = "Array".to_string();
                    let idx = self.injector_constants.len();
                    self.injector_constants.push(InjectorConstantDef { class_name: array_class, fields: const_fields });
                    let temp = self.alloc_temp(ValueType::Object);
                    self.emit(IntermediateCode::new(IntermediateOperator::LoadInjectorConstant(idx), Operand::int(0), None, Some(temp)), *span);
                    Operand::Address(temp)
                } else {
                    self.diagnostics.emit_error(*span, "注入器数组元素必须是编译时常量");
                    Operand::Address(Address::new(ValueType::Object, 0))
                }
            }
            Expression::InjectorFieldRef(name, span) => {
                // 校验当前类是否有注入器字段定义
                if self.injector_field_info.is_empty() {
                    self.diagnostics.emit_error(*span, &format!("未定义的注入器字段 `^{}`（当前类未声明注入器字段）", name));
                    return Operand::Address(Address::new(ValueType::Int, 0));
                }
                // 从 VM 上下文加载当前注入器对象 ID
                let temp_inj = self.alloc_temp(ValueType::Object);
                self.emit(IntermediateCode::new(
                    IntermediateOperator::LoadInjector,
                    Operand::int(0),
                    None,
                    Some(temp_inj),
                ), *span);
                // 按字段名查找注入器字段的索引和值类型
                if let Some(&(field_idx, vt)) = self.injector_field_info.get(name) {
                    let temp_result = self.alloc_temp(vt);
                    let load_op = Self::load_injector_field_op(vt, field_idx);
                    // left 操作数传递注入器对象地址
                    self.emit(IntermediateCode::new(
                        load_op,
                        Operand::Address(temp_inj),
                        None,
                        Some(temp_result),
                    ), *span);
                    Operand::Address(temp_result)
                } else {
                    self.diagnostics.emit_error(*span, &format!("未定义的注入器字段 `^{}`", name));
                    Operand::Address(Address::new(ValueType::Int, 0))
                }
            }
            Expression::InjectorNew { injector_field, args, span } => {
                self.generate_injector_new(injector_field, args, *span)
            }
            Expression::Lambda { parameters, body, span } => {
                // 1. 自由变量分析（须在生成子代码之前，以便注册捕获变量到 sub_cg）
                let param_names: HashSet<String> = parameters.iter()
                    .map(|p| p.name.clone()).collect();
                let free_vars = Self::analyze_free_vars_lambda_body(body, &param_names);

                // 2. 推导捕获变量的值类型（从父上下文的 var_types / field_info 查）
                let captured_var_types: Vec<ValueType> = free_vars.iter().map(|name| {
                    if let Some(ti) = self.var_types.get(name) {
                        return Self::type_to_value_type(ti);
                    }
                    if let Some(&(_, vt)) = self.field_info.get(name) {
                        return vt;
                    }
                    ValueType::Int // 兜底（不应到达）
                }).collect();

                // 3. 创建子生成器，注册 Lambda 参数与捕获变量
                let mut sub_diags = Diagnostics::new();
                let mut dummy_delegates = Vec::new();
                // Lambda 方法体是独立的执行单元，不从父上下文的注入器常量池
                // 续写索引（Lambda 内不允许出现注入器字面量）
                let mut sub_cg = CodeGenerator::new(self.symbol_table, &mut sub_diags, &mut dummy_delegates, self.injector_fields, Vec::new());

                for param in parameters {
                    let vt = Self::type_ref_to_value_type(&param.param_type);
                    sub_cg.declare_local(&param.name, vt);
                }
                // 注册捕获变量到 sub_cg（使 body_ir 中自由变量引用可解析为局部地址）
                for (name, vt) in free_vars.iter().zip(captured_var_types.iter()) {
                    sub_cg.declare_local(name, *vt);
                }

                let return_type = match body {
                    LambdaBody::Expression(ref expr) => {
                        let r = sub_cg.generate_expression(expr);
                        let vt = Self::operand_value_type(&r);
                        let ret_addr = Address::new(vt, 0);
                        sub_cg.emit(IntermediateCode::assign(ret_addr, r), *span);
                        sub_cg.emit(IntermediateCode::return_value(vt), *span);
                        vt
                    }
                    LambdaBody::Block(ref stmts) => {
                        for s in stmts {
                            sub_cg.generate_statement(s);
                        }
                        // 从子生成器的 IR 中推导返回类型（查找最后一条 Return* 指令）
                        let mut ret_type = ValueType::Object;
                        for cws in sub_cg.codes.iter().rev() {
                            match &cws.code.operator {
                                IntermediateOperator::ReturnInt => { ret_type = ValueType::Int; break; }
                                IntermediateOperator::ReturnFloat => { ret_type = ValueType::Float; break; }
                                IntermediateOperator::ReturnBool => { ret_type = ValueType::Bool; break; }
                                IntermediateOperator::ReturnString => { ret_type = ValueType::String; break; }
                                IntermediateOperator::ReturnObject => { ret_type = ValueType::Object; break; }
                                IntermediateOperator::ReturnVoid => { ret_type = ValueType::Object; break; }
                                _ => {}
                            }
                        }
                        ret_type
                    }
                };

                // 4. 获取 body IR，将嵌套委托转移到父级
                let body_ir = sub_cg.into_codes();
                self.delegate_impls.append(&mut dummy_delegates);
                let delegate_idx = self.delegate_impls.len();
                let param_types: Vec<ValueType> = parameters.iter()
                    .map(|p| Self::type_ref_to_value_type(&p.param_type)).collect();

                let free_vars_count = free_vars.len();
                self.delegate_impls.push(DelegateImpl {
                    param_types,
                    return_type,
                    body_ir,
                    captured_var_names: free_vars,
                    captured_var_types: captured_var_types.clone(),
                    outer_value_count: free_vars_count,
                    is_static: free_vars_count == 0,
                });

                // 5. 发射 SetXxxParameter 将捕获变量值写入参数池（供 ConstructDelegate 读取）
                self.param_counters.reset();
                let cv_names = self.delegate_impls[delegate_idx].captured_var_names.clone();
                for name in &cv_names {
                    // 局部变量/参数：直接从父上下文查找地址
                    if let Some(addr) = self.local_vars.get(name).copied() {
                        self.emit_set_param(Operand::Address(addr), *span);
                    }
                    // 正则变量取不到则看 field_info（字段捕获：LoadThis + LoadField + SetParam）
                    else if let Some(&(field_offset, field_vt)) = self.field_info.get(name) {
                        let this_temp = self.alloc_temp(ValueType::Object);
                        self.emit(
                            IntermediateCode::new(
                                IntermediateOperator::LoadThis,
                                Operand::int(0), None, Some(this_temp),
                            ),
                            *span,
                        );
                        let field_temp = self.alloc_temp(field_vt);
                        let load_op = Self::load_field_op(field_vt, field_offset);
                        self.emit(
                            IntermediateCode::new(
                                load_op,
                                Operand::Address(this_temp),
                                None,
                                Some(field_temp),
                            ),
                            *span,
                        );
                        self.emit_set_param(Operand::Address(field_temp), *span);
                    }
                }

                // 6. 生成 ConstructDelegate(idx)
                let temp = self.alloc_temp(ValueType::Object);
                self.emit(
                    IntermediateCode::new(
                        IntermediateOperator::ConstructDelegate(delegate_idx),
                        Operand::int(delegate_idx as i64),
                        None,
                        Some(temp),
                    ),
                    *span,
                );
                Operand::Address(temp)
            }
            // 数组元素访问 a[i] → 调用 native Array 的 get 方法（Phase H3）
            Expression::ArrayAccess { array, index, span } => {
                // `new (^field)[size]` — 注入器数组构造（对齐 C# ArrayConstructorInvocationExpression）：
                // parser 将其解析为 InjectorNew 上的 ArrayAccess；
                // 元素类型取注入器字段同名类字段声明类型（`FunctionCurve^[]`）的数组元素。
                // 注意：size 表达式必须先于 SetInjector 求值，
                // 否则 `new (^laneLines)[^laneLines.length]` 中的 `^laneLines` 会读到被切换后的注入器。
                if let Expression::InjectorNew { injector_field, .. } = array.as_ref() {
                    if let Some(TypeInfo::Array(elem)) = self.field_types.get(injector_field) {
                        let elem_name = match elem.as_ref() {
                            TypeInfo::Object(cid) => self.symbol_table.classes.get(cid.0).name.clone(),
                            TypeInfo::Int | TypeInfo::Enum(_) => "int".to_string(),
                            TypeInfo::Float => "float".to_string(),
                            TypeInfo::Bool => "bool".to_string(),
                            TypeInfo::String => "string".to_string(),
                            _ => "object".to_string(),
                        };
                        let size_op = self.generate_expression(index);
                        // 加载注入器字段值并设为当前注入器上下文（与 generate_injector_new 一致）
                        if let Some(&(field_idx, _)) = self.injector_field_info.get(injector_field) {
                            let temp_inj = self.alloc_temp(ValueType::Object);
                            self.emit(IntermediateCode::new(
                                IntermediateOperator::LoadInjector,
                                Operand::int(0), None, Some(temp_inj),
                            ), *span);
                            let temp_field = self.alloc_temp(ValueType::Object);
                            self.emit(IntermediateCode::new(
                                IntermediateOperator::LoadObjectInjectorField(field_idx),
                                Operand::Address(temp_inj), None, Some(temp_field),
                            ), *span);
                            self.emit(IntermediateCode::new(
                                IntermediateOperator::SetInjector,
                                Operand::Address(temp_field), None, None,
                            ), *span);
                        }
                        let temp = self.alloc_temp(ValueType::Object);
                        self.emit(IntermediateCode::new(
                            IntermediateOperator::InvokeArrayConstructor,
                            size_op,
                            Some(Operand::string(elem_name)),
                            Some(temp),
                        ), *span);
                        return Operand::Address(temp);
                    }
                }
                let arr_op = self.generate_expression(array);
                let idx_op = self.generate_expression(index);
                self.param_counters.reset();
                self.emit_set_param(idx_op, *span);
                let element_value_type = self.resolve_object_type(array)
                    .and_then(|array_type| match array_type {
                        TypeInfo::Array(element_type) => Some(Self::type_to_value_type(&element_type)),
                        _ => None,
                    })
                    .unwrap_or(ValueType::Int);
                let temp = self.alloc_temp(element_value_type);
                self.emit(
                    IntermediateCode::new(
                        IntermediateOperator::InvokeInstance(0),
                        arr_op,
                        None,
                        Some(temp),
                    ),
                    *span,
                );
                Operand::Address(temp)
            }
            // 类型转换表达式 (TargetType)expr
            Expression::Cast { target_type, expression, span } => {
                self.generate_cast(target_type, expression, *span)
            }
            // super 关键字，生成 LoadThis（VM 层面 super 与 this 等同，运行时分派由父类方法表完成）
            Expression::Super(span) => {
                let this_temp = self.alloc_temp(ValueType::Object);
                self.emit(
                    IntermediateCode::new(
                        IntermediateOperator::LoadThis,
                        Operand::int(0), None, Some(this_temp),
                    ),
                    *span,
                );
                Operand::Address(this_temp)
            }
        }
    }

    /// 处理 `new Type^[N]{ elem1, elem2, ... }` 数组构造（带内联元素，H3A）。
    ///
    /// 修复真实谱面 `@Chart` 方法 `return new Element^[8]{ ... };` 内联元素被
    /// 静默丢弃、导致 `score_elements=0` 的 bug：
    /// - **全部元素可折叠为编译期常量**：生成 `class_name="Array"` 的元素注入器
    ///   数组常量（对齐 `Expression::InjectorArray` 分支），发射 `LoadInjectorConstant`
    ///   返回常量数组地址——框架 `fill_element_period_from_method` 据此从常量池
    ///   恢复每个元素，`@Chart` 不再返回空数组。
    /// - **元素含运行时会变表达式**：退化为逐元素 `SetArrayElement` 写入，保证
    ///   任何元素都不丢失（不静默丢弃）。
    fn generate_array_constructor_with_elements(
        &mut self,
        class_name: &str,
        arguments: &[Expression],
        span: Span,
    ) -> Operand {
        // 尝试将所有内联元素折叠为编译期常量
        let const_fields: Vec<InjectorConstField> = arguments[1..]
            .iter()
            .filter_map(|e| self.try_eval_const(e))
            .collect();
        if const_fields.len() == arguments.len() - 1 {
            // 全部可折叠：生成常量数组，对齐 Expression::InjectorArray 路径
            let idx = self.injector_constants.len();
            self.injector_constants.push(InjectorConstantDef {
                class_name: "Array".to_string(),
                fields: const_fields,
            });
            let temp = self.alloc_temp(ValueType::Object);
            self.emit(IntermediateCode::new(
                IntermediateOperator::LoadInjectorConstant(idx),
                Operand::int(0),
                None,
                Some(temp),
            ), span);
            return Operand::Address(temp);
        }

        // 部分元素无法折叠：退化为逐元素写入，保证元素不丢失
        let size_op = if let Some(arg) = arguments.get(0) {
            self.generate_expression(arg)
        } else {
            Operand::int(0)
        };
        let elem_type = if class_name.is_empty() { "Object" } else { class_name };
        let temp = self.alloc_temp(ValueType::Object);
        self.emit(IntermediateCode::new(
            IntermediateOperator::InvokeArrayConstructor,
            size_op,
            Some(Operand::string(elem_type.to_string())),
            Some(temp),
        ), span);
        for (i, elem) in arguments[1..].iter().enumerate() {
            let elem_op = self.generate_expression(elem);
            self.param_counters.reset();
            self.emit_set_param(Operand::int(i as i64), span);
            self.emit_set_param(elem_op, span);
            self.emit(
                IntermediateCode::new(
                    IntermediateOperator::InvokeInstance(1), // array set 方法
                    Operand::Address(temp),
                    None,
                    None,
                ),
                span,
            );
        }
        Operand::Address(temp)
    }

    /// 生成二元运算代码
    ///
    /// 对齐 C# 三级运算符语义（AdditionExpression / CalculateExpression /
    /// ComparisonExpression / EqualityExpression）：
    /// - **加法 `+`**：int+int=int；含 float→float；含 string→string（另一操作数 int/float/bool 自动 cast 成 string）
    /// - **减乘除模 `- * / %`**：int+int=int；否则 float（操作数 int→float 提升）
    /// - **比较 `< > <= >=`**：结果恒 bool；运算类型 int+int=int，否则 float
    /// - **相等 `== !=`**：结果恒 bool；运算类型取可互相自动转换的类型（int↔float 提升，其余需同类型）
    /// - **逻辑 `&& ||`**：结果恒 bool；操作数须为 bool
    ///
    /// 操作数先按运算类型自动 cast（`emit_cast_operand`），再按运算类型选具体操作码；
    /// 非法类型组合报编译错误（对齐 `ExpressionOperandWrongTypeException`）。
    fn generate_binary(
        &mut self,
        left: &Expression,
        op: BinaryOp,
        right: &Expression,
        span: Span,
    ) -> Operand {
        let left_op = self.generate_expression(left);
        let right_op = self.generate_expression(right);
        let lvt = Self::operand_value_type(&left_op);
        let rvt = Self::operand_value_type(&right_op);

        use BinaryOp::*;
        match op {
            // ===== 加法：支持数值相加与字符串拼接 =====
            Add => {
                // 结果类型：含 string→string；含 float→float；全 int→int
                let operand_vt = match (lvt, rvt) {
                    (ValueType::String, _) | (_, ValueType::String) => ValueType::String,
                    (ValueType::Float, ValueType::Int)
                    | (ValueType::Int, ValueType::Float)
                    | (ValueType::Float, ValueType::Float) => ValueType::Float,
                    (ValueType::Int, ValueType::Int) => ValueType::Int,
                    _ => {
                        self.diagnostics.emit_error(span, "加法运算的操作数类型必须为 int、float 或 string。".to_string());
                        return Operand::Address(self.alloc_temp(ValueType::Int));
                    }
                };
                let l = self.emit_cast_operand(left_op, lvt, operand_vt, span);
                let r = self.emit_cast_operand(right_op, rvt, operand_vt, span);
                let ir_op = match operand_vt {
                    ValueType::Int => IntermediateOperator::IntAdd,
                    ValueType::Float => IntermediateOperator::FloatAdd,
                    ValueType::String => IntermediateOperator::StringAddition,
                    _ => {
                        self.diagnostics.emit_error(span, "内部错误：非预期的操作数类型".to_string());
                        return Operand::Address(self.alloc_temp(ValueType::Int));
                    }
                };
                let result = self.alloc_temp(operand_vt);
                self.emit(IntermediateCode::binary(ir_op, l, r, result), span);
                Operand::Address(result)
            }
            // ===== 减乘除模：仅数值，int+int=int 否则 float =====
            Subtract | Multiply | Divide | Modulo => {
                let operand_vt = match (lvt, rvt) {
                    (ValueType::Int, ValueType::Int) => ValueType::Int,
                    (ValueType::Int, ValueType::Float)
                    | (ValueType::Float, ValueType::Int)
                    | (ValueType::Float, ValueType::Float) => ValueType::Float,
                    _ => {
                        self.diagnostics.emit_error(span, "算术运算的操作数类型必须为 int 或 float。".to_string());
                        return Operand::Address(self.alloc_temp(ValueType::Int));
                    }
                };
                let l = self.emit_cast_operand(left_op, lvt, operand_vt, span);
                let r = self.emit_cast_operand(right_op, rvt, operand_vt, span);
                let ir_op = match (op, operand_vt) {
                    (Subtract, ValueType::Int) => IntermediateOperator::IntSub,
                    (Subtract, ValueType::Float) => IntermediateOperator::FloatSub,
                    (Multiply, ValueType::Int) => IntermediateOperator::IntMul,
                    (Multiply, ValueType::Float) => IntermediateOperator::FloatMul,
                    (Divide, ValueType::Int) => IntermediateOperator::IntDiv,
                    (Divide, ValueType::Float) => IntermediateOperator::FloatDiv,
                    (Modulo, ValueType::Int) => IntermediateOperator::IntMod,
                    (Modulo, ValueType::Float) => IntermediateOperator::FloatMod,
                    _ => {
                        self.diagnostics.emit_error(span, "内部错误：非预期的操作数类型".to_string());
                        return Operand::Address(self.alloc_temp(ValueType::Int));
                    }
                };
                let result = self.alloc_temp(operand_vt);
                self.emit(IntermediateCode::binary(ir_op, l, r, result), span);
                Operand::Address(result)
            }
            // ===== 比较：仅数值，结果恒 bool；运算类型 int+int=int 否则 float =====
            Less | LessEqual | Greater | GreaterEqual => {
                let operand_vt = match (lvt, rvt) {
                    (ValueType::Int, ValueType::Int) => ValueType::Int,
                    (ValueType::Int, ValueType::Float)
                    | (ValueType::Float, ValueType::Int)
                    | (ValueType::Float, ValueType::Float) => ValueType::Float,
                    _ => {
                        self.diagnostics.emit_error(span, "比较运算的操作数类型必须为 int 或 float。".to_string());
                        return Operand::Address(self.alloc_temp(ValueType::Bool));
                    }
                };
                let l = self.emit_cast_operand(left_op, lvt, operand_vt, span);
                let r = self.emit_cast_operand(right_op, rvt, operand_vt, span);
                let ir_op = match (op, operand_vt) {
                    (Less, ValueType::Int) => IntermediateOperator::IntLess,
                    (Less, ValueType::Float) => IntermediateOperator::FloatLess,
                    (LessEqual, ValueType::Int) => IntermediateOperator::IntLessEqual,
                    (LessEqual, ValueType::Float) => IntermediateOperator::FloatLessEqual,
                    (Greater, ValueType::Int) => IntermediateOperator::IntGreater,
                    (Greater, ValueType::Float) => IntermediateOperator::FloatGreater,
                    (GreaterEqual, ValueType::Int) => IntermediateOperator::IntGreaterEqual,
                    (GreaterEqual, ValueType::Float) => IntermediateOperator::FloatGreaterEqual,
                    _ => {
                        self.diagnostics.emit_error(span, "内部错误：非预期的操作数类型".to_string());
                        return Operand::Address(self.alloc_temp(ValueType::Bool));
                    }
                };
                let result = self.alloc_temp(ValueType::Bool);
                self.emit(IntermediateCode::binary(ir_op, l, r, result), span);
                Operand::Address(result)
            }
            // ===== 相等：结果恒 bool；运算类型取可互相自动转换的类型（int↔float 提升） =====
            Equal | NotEqual => {
                // 运算类型判定：int↔float 提升为 float；同类型直接用；其余按左类型（可能是 Object/Bool/String）
                let operand_vt = match (lvt, rvt) {
                    (a, b) if a == b => a,
                    (ValueType::Int, ValueType::Float)
                    | (ValueType::Float, ValueType::Int) => ValueType::Float,
                    _ => {
                        self.diagnostics.emit_error(span, "相等运算的两操作数类型不同且无法互相转换。".to_string());
                        return Operand::Address(self.alloc_temp(ValueType::Bool));
                    }
                };
                let l = self.emit_cast_operand(left_op, lvt, operand_vt, span);
                let r = self.emit_cast_operand(right_op, rvt, operand_vt, span);
                let ir_op = match (op, operand_vt) {
                    (Equal, ValueType::Int) => IntermediateOperator::IntEqual,
                    (Equal, ValueType::Float) => IntermediateOperator::FloatEqual,
                    (Equal, ValueType::Bool) => IntermediateOperator::BoolEqual,
                    (Equal, ValueType::String) => IntermediateOperator::StringEqual,
                    (Equal, ValueType::Object) => IntermediateOperator::ObjectEqual,
                    (NotEqual, ValueType::Int) => IntermediateOperator::IntNotEqual,
                    (NotEqual, ValueType::Float) => IntermediateOperator::FloatNotEqual,
                    (NotEqual, ValueType::Bool) => IntermediateOperator::BoolNotEqual,
                    (NotEqual, ValueType::String) => IntermediateOperator::StringNotEqual,
                    (NotEqual, ValueType::Object) => IntermediateOperator::ObjectNotEqual,
                    _ => {
                        self.diagnostics.emit_error(span, "内部错误：非预期的操作数类型".to_string());
                        return Operand::Address(self.alloc_temp(ValueType::Bool));
                    }
                };
                let result = self.alloc_temp(ValueType::Bool);
                self.emit(IntermediateCode::binary(ir_op, l, r, result), span);
                Operand::Address(result)
            }
            // ===== 逻辑：结果恒 bool；操作数须为 bool =====
            LogicAnd | LogicOr => {
                if lvt != ValueType::Bool || rvt != ValueType::Bool {
                    self.diagnostics.emit_error(span, "逻辑运算的操作数类型必须为 bool。".to_string());
                    return Operand::Address(self.alloc_temp(ValueType::Bool));
                }
                let ir_op = if op == LogicAnd {
                    IntermediateOperator::LogicalAnd
                } else {
                    IntermediateOperator::LogicalOr
                };
                let result = self.alloc_temp(ValueType::Bool);
                self.emit(IntermediateCode::binary(ir_op, left_op, right_op, result), span);
                Operand::Address(result)
            }
        }
    }

    /// 生成一元运算代码
    fn generate_unary(
        &mut self,
        op: UnaryOp,
        operand: &Expression,
        span: Span,
    ) -> Operand {
        let inner = self.generate_expression(operand);
        let vt = Self::operand_value_type(&inner);
        let result = self.alloc_temp(vt);

        match op {
            UnaryOp::Negate => {
                let ir_op = match vt {
                    ValueType::Int => IntermediateOperator::IntOpposite,
                    ValueType::Float => IntermediateOperator::FloatOpposite,
                    _ => IntermediateOperator::IntOpposite,
                };
                self.emit(
                    IntermediateCode::new(ir_op, inner, None, Some(result)),
                    span,
                );
            }
            UnaryOp::Not => {
                self.emit(
                    IntermediateCode::new(
                        IntermediateOperator::LogicalNot,
                        inner,
                        None,
                        Some(result),
                    ),
                    span,
                );
            }
            UnaryOp::PreIncrement | UnaryOp::PreDecrement | UnaryOp::PostIncrement | UnaryOp::PostDecrement => {
                let is_pre = matches!(op, UnaryOp::PreIncrement | UnaryOp::PreDecrement);
                let is_inc = matches!(op, UnaryOp::PreIncrement | UnaryOp::PostIncrement);

                // 仅支持 int 和 float 类型的自增/自减
                let operand_vt = match vt {
                    ValueType::Int | ValueType::Float => vt,
                    _ => {
                        self.diagnostics.emit_error(span, "自增/自减操作仅支持 int 或 float 类型");
                        self.emit(IntermediateCode::assign(result, inner), span);
                        return Operand::Address(result);
                    }
                };

                // 保存原值到临时变量（后置操作返回原值）
                let orig_temp = self.alloc_temp(operand_vt);
                self.emit(IntermediateCode::assign(orig_temp, inner.clone()), span);

                // 计算新值：原值 ± 1
                let one_op = match operand_vt {
                    ValueType::Int => Operand::int(1),
                    ValueType::Float => Operand::float(1.0),
                    _ => unreachable!(),
                };
                let arith_op = match (operand_vt, is_inc) {
                    (ValueType::Int, true) => IntermediateOperator::IntAdd,
                    (ValueType::Int, false) => IntermediateOperator::IntSub,
                    (ValueType::Float, true) => IntermediateOperator::FloatAdd,
                    (ValueType::Float, false) => IntermediateOperator::FloatSub,
                    _ => unreachable!(),
                };
                let new_val = self.alloc_temp(operand_vt);
                self.emit(IntermediateCode::binary(arith_op, Operand::Address(orig_temp), one_op, new_val), span);

                // 写回操作数：区分局部变量 / this.field / obj.field
                match operand {
                    Expression::Identifier(name, var_span) => {
                        if let Some(addr) = self.lookup_var(name) {
                            self.emit(IntermediateCode::assign(addr, Operand::Address(new_val)), *var_span);
                        } else if let Some(&(offset, _)) = self.field_info.get(name) {
                            let this_temp = self.alloc_temp(ValueType::Object);
                            self.emit(IntermediateCode::new(
                                IntermediateOperator::LoadThis,
                                Operand::int(0), None, Some(this_temp),
                            ), *var_span);
                            let set_op = Self::set_field_op(operand_vt, offset);
                            self.emit(IntermediateCode::new(
                                set_op,
                                Operand::Address(this_temp),
                                Some(Operand::Address(new_val)),
                                None,
                            ), *var_span);
                        }
                    }
                    Expression::MemberAccess { object, member, .. } => {
                        let obj_op = self.generate_expression(object);
                        let field_lookup = self.lookup_field_for_object(object, member)
                            .or_else(|| self.field_info.get(member).copied());
                        if let Some((offset, _)) = field_lookup {
                            let set_op = Self::set_field_op(operand_vt, offset);
                            self.emit(IntermediateCode::new(
                                set_op,
                                obj_op,
                                Some(Operand::Address(new_val)),
                                None,
                            ), span);
                        }
                    }
                    _ => {
                        self.diagnostics.emit_error(span, "自增/自减操作的目标必须为变量或字段");
                    }
                }

                // 前置返回新值，后置返回原值
                self.emit(IntermediateCode::assign(result, if is_pre { Operand::Address(new_val) } else { Operand::Address(orig_temp) }), span);
            }
        }

        Operand::Address(result)
    }

    /// 生成赋值代码
    fn generate_assignment(
        &mut self,
        target: &AssignmentTarget,
        value: &Expression,
        span: Span,
    ) -> Operand {
        let result_op = self.generate_expression(value);

        match target {
            AssignmentTarget::Variable(name, _) => {
                let vt = Self::operand_value_type(&result_op);
                let addr = match self.lookup_var(name) {
                    Some(a) => a,
                    None => {
                // 回退：检查是否为实例字段（this.field = val 隐式写法）
                if let Some(&(offset, field_vt)) = self.field_info.get(name) {
                    let this_temp = self.alloc_temp(ValueType::Object);
                    self.emit(
                        IntermediateCode::new(
                            IntermediateOperator::LoadThis,
                            Operand::int(0), None, Some(this_temp),
                        ),
                        span,
                    );
                    let set_op = Self::set_field_op(field_vt, offset);
                    self.emit(
                        IntermediateCode::new(
                            set_op,
                            Operand::Address(this_temp),
                            Some(result_op.clone()),
                            None,
                        ),
                        span,
                    );
                    return result_op;
                }
                        // 隐式声明变量
                        self.declare_local(name, vt)
                    }
                };
                self.emit(IntermediateCode::assign(addr, result_op), span);
                Operand::Address(addr)
            }
            AssignmentTarget::Field { object, field, span: _ } => {
                // 对齐 C# FieldAssignmentExpression：求值右侧值，再求值接收器对象，SetField
                if matches!(**object, Expression::This(_)) {
                    if let Some(&(offset, vt)) = self.field_info.get(field) {
                        let this_temp = self.alloc_temp(ValueType::Object);
                        self.emit(
                            IntermediateCode::new(
                                IntermediateOperator::LoadThis,
                                Operand::int(0), None, Some(this_temp),
                            ),
                            span,
                        );
                        let set_op = Self::set_field_op(vt, offset);
                        self.emit(
                            IntermediateCode::new(
                                set_op,
                                Operand::Address(this_temp),
                                Some(result_op.clone()),
                                None,
                            ),
                            span,
                        );
                        return result_op;
                    }
                }
                // obj.field = val（非 this）
                let obj_op = self.generate_expression(object);
                let field_lookup = self.lookup_field_for_object(object, field);
                if let Some((offset, vt)) = field_lookup.or_else(|| self.field_info.get(field).copied()) {
                    let set_op = Self::set_field_op(vt, offset);
                    self.emit(
                        IntermediateCode::new(
                            set_op,
                            obj_op,
                            Some(result_op.clone()),
                            None,
                        ),
                        span,
                    );
                    return result_op;
                }
                self.diagnostics.emit_error(span, format!("未定义的字段 `{}`", field));
                result_op
            }
            // 数组元素赋值 a[i] = val → 调用 native Array 的 set 方法（Phase H3）
            AssignmentTarget::ArrayElement { array, index, span } => {
                let arr_op = self.generate_expression(array);
                let idx_op = self.generate_expression(index);
                self.param_counters.reset();
                self.emit_set_param(idx_op, *span);
                self.emit_set_param(result_op.clone(), *span);
                self.emit(
                    IntermediateCode::new(
                        IntermediateOperator::InvokeInstance(1),
                        arr_op,
                        None,
                        None,
                    ),
                    *span,
                );
                result_op
            }
            AssignmentTarget::InjectorField { object, field, span: field_span } => {
                // 对齐 C# InjectorFieldAssignmentExpression
                // B-4: FieldInjecting 上下文中注入器字段只读，不可写
                if self.current_block_context == BlockContext::FieldInjecting {
                    self.diagnostics.emit_error(
                        *field_span,
                        format!("注入器字段 `^{}` 在注入中（FieldInjecting）上下文中不可写", field),
                    );
                    return result_op;
                }
                if matches!(**object, Expression::This(_)) {
                    if let Some(&(field_idx, _vt)) = self.injector_field_info.get(field) {
                        let temp_inj = self.alloc_temp(ValueType::Object);
                        self.emit(IntermediateCode::new(
                            IntermediateOperator::LoadInjector,
                            Operand::int(0), None, Some(temp_inj),
                        ), span);
                        let set_op = Self::set_injector_field_op(_vt, field_idx);
                        self.emit(IntermediateCode::new(
                            set_op,
                            result_op.clone(),
                            Some(Operand::Address(temp_inj)),
                            None,
                        ), span);
                        return result_op;
                    }
                    self.diagnostics.emit_error(span, &format!("未定义的注入器字段 `^{}`", field));
                    return result_op;
                }
                // obj.^field = val（非 this）→ 求值对象表达式，结果作为注入器引用
                let obj_op = self.generate_expression(object);
                if let Some(&(field_idx, _vt)) = self.injector_field_info.get(field) {
                    let set_op = Self::set_injector_field_op(_vt, field_idx);
                    self.emit(IntermediateCode::new(
                        set_op,
                        result_op.clone(),
                        Some(obj_op),
                        None,
                    ), span);
                    return result_op;
                }
                // 字段不在当前类注入器字段中：按对象声明类型解析（同读取路径，
                // 支持 `@PeriodModifier` 的 `laneInjector.^generateTime = ...`）
                if let Some((field_idx, vt)) = self.resolve_object_injector_field(object, field) {
                    let set_op = Self::set_injector_field_op(vt, field_idx);
                    self.emit(IntermediateCode::new(
                        set_op,
                        result_op.clone(),
                        Some(obj_op),
                        None,
                    ), span);
                    return result_op;
                }
                self.diagnostics.emit_error(span, &format!("未定义的注入器字段 `^{}`", field));
                result_op
            }
        }
    }

    /// 从对象表达式的声明类型中查找字段（跨类字段访问）。
    ///
    /// 当前 `self.field_info` 仅包含正在编译的类的字段，而 `obj.field` 中的
    /// `obj` 可能属于其他类。此方法根据变量声明的类，沿继承链查找字段偏移和类型。
    pub(crate) fn lookup_field_for_object(&self, object: &Expression, field_name: &str) -> Option<(usize, ValueType)> {
        // 数组类型对象的成员（如 `arr.length`）→ 在对应 native 数组类中查找
        // （对齐 C#：数组即 IntArray/ObjectArray 等 native 类的实例，`length` 是其 int 字段）
        if let Some(ti) = self.resolve_object_type(object) {
            if let Some(array_class) = Self::native_array_class_name(&ti) {
                let (cid, _) = self.symbol_table.find_class_by_name(array_class)?;
                let ci = self.symbol_table.classes.get(cid.0);
                for &fid in &ci.fields {
                    let fi = self.symbol_table.fields.get(fid.0);
                    if fi.name == field_name {
                        let vt = Self::type_to_value_type(&fi.field_type);
                        return Some((fi.offset.unwrap_or(0), vt));
                    }
                }
                return None;
            }
        }
        // 从变量声明中获取类名
        let class_name = match object {
            Expression::Identifier(name, _) => self.var_class.get(name).cloned(),
            Expression::This(_) => self.current_class_name.clone(),
            // 成员链访问 t.fieldA.fieldB：递归解析外侧字段类型所属的类
            Expression::MemberAccess { object: inner, member: inner_field, .. } => {
                let inner_type = self.resolve_object_type(inner)?;
                let field_type = self.lookup_field_type_in(&inner_type, inner_field)?;
                match field_type {
                    TypeInfo::Object(cid) => Some(self.symbol_table.classes.get(cid.0).name.clone()),
                    _ => None,
                }
            }
            _ => None,
        };
        // 从 var_types 中获取类 ID（更精确的路径）
        // 局部变量未命中时回退到当前类字段（隐式 this.field 作为接收者的情形，
        // 如 `lane.noteReferenceNode` 中的 `lane`）
        let class_id_from_type = self.resolve_object_type(object).and_then(|ty| {
            if let TypeInfo::Object(class_id) = ty {
                Some(class_id)
            } else {
                None
            }
        });
        // 优先用 var_class 查找，回退到 var_types
        let class_name = class_name.or_else(|| {
            class_id_from_type.map(|cid| self.symbol_table.classes.get(cid.0).name.clone())
        });
        let class_name = match class_name {
            Some(cn) => cn,
            None => return None,
        };
        // 沿继承链查找字段
        let scope_id = self.class_lookup_scope();
        let mut class_id = self.symbol_table.lookup_class(scope_id, &class_name)?;
        loop {
            let class_info = self.symbol_table.classes.get(class_id.0);
            for &field_id in &class_info.fields {
                let fi = self.symbol_table.fields.get(field_id.0);
                if fi.name == field_name {
                    let vt = Self::type_to_value_type(&fi.field_type);
                    let offset = fi.offset.unwrap_or(0);
                    return Some((offset, vt));
                }
            }
            class_id = match class_info.super_class {
                Some(super_id) => super_id,
                None => break,
            };
        }
        None
    }

    /// 递归解析表达式的对象类型，用于成员链访问的类型推导
    ///
    /// 标识符优先查局部变量，未命中时回退到当前类字段（隐式 `this.field`），
    /// 使 `lane.noteReferenceNode.x` 这类以 this 字段开头的成员链可以推导。
    pub(crate) fn resolve_object_type(&self, expr: &Expression) -> Option<TypeInfo> {
        match expr {
            Expression::Identifier(name, _) => self
                .var_types
                .get(name)
                .or_else(|| self.field_types.get(name))
                .cloned(),
            // 注入器字段引用 `^field` 与同名类字段类型一致（如 `^laneLines` ↔ `FunctionCurve^[]`）
            Expression::InjectorFieldRef(name, _) => self.field_types.get(name).cloned(),
            Expression::This(_) => self.current_class_name.as_ref()
                .and_then(|n| self.symbol_table.lookup_class(self.class_lookup_scope(), n))
                .map(TypeInfo::Object),
            Expression::MemberAccess { object, member, .. } => {
                let inner_type = self.resolve_object_type(object)?;
                self.lookup_field_type_in(&inner_type, member)
            }
            Expression::ArrayAccess { array, .. } => {
                let array_type = self.resolve_object_type(array)?;
                match array_type {
                    TypeInfo::Array(element_type) => Some(*element_type),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// 数组类型对应的 native 数组类名（对齐 C#：`int[]` ↔ `IntArray`，对象数组 ↔ `ObjectArray`）
    pub(crate) fn native_array_class_name(ti: &TypeInfo) -> Option<&'static str> {
        if let TypeInfo::Array(elem) = ti {
            Some(match elem.as_ref() {
                TypeInfo::Int | TypeInfo::Enum(_) => "IntArray",
                TypeInfo::Float => "FloatArray",
                TypeInfo::Bool => "BoolArray",
                TypeInfo::String => "StringArray",
                _ => "ObjectArray",
            })
        } else {
            None
        }
    }

    /// 生成成员访问代码
    fn generate_member_access(
        &mut self,
        object: &Expression,
        member: &str,
    ) -> Operand {
        // this.^field → 读取注入器字段（对齐 C# InjectorFieldReferenceExpression）
        if let Some(field_name) = member.strip_prefix('^') {
            if matches!(object, Expression::This(_)) {
                if let Some(&(field_idx, vt)) = self.injector_field_info.get(field_name) {
                    let temp_inj = self.alloc_temp(ValueType::Object);
                    self.emit(IntermediateCode::new(
                        IntermediateOperator::LoadInjector,
                        Operand::int(0), None, Some(temp_inj),
                    ), object.span());
                    let temp = self.alloc_temp(vt);
                    let load_op = Self::load_injector_field_op(vt, field_idx);
                    self.emit(
                        IntermediateCode::new(load_op, Operand::Address(temp_inj), None, Some(temp)),
                        object.span(),
                    );
                    return Operand::Address(temp);
                }
                self.diagnostics.emit_error(object.span(), &format!("未定义的注入器字段 `^{}`", field_name));
                return Operand::Address(Address::new(ValueType::Int, 0));
            }
            // obj.^field（非 this）→ 求值对象表达式的注入器，再 LoadInjectorField
            let obj_op = self.generate_expression(object);
            if let Some(&(field_idx, vt)) = self.injector_field_info.get(field_name) {
                let temp = self.alloc_temp(vt);
                let load_op = Self::load_injector_field_op(vt, field_idx);
                self.emit(
                    IntermediateCode::new(load_op, obj_op, None, Some(temp)),
                    object.span(),
                );
                return Operand::Address(temp);
            }
            // 字段不在当前类注入器字段中：按对象的声明类型解析（对齐 C# 按
            // 对象声明类型查找注入器字段）。如 `@PeriodModifier` 的
            // `laneInjector.^generateTime` 中 laneInjector 是 `DremuLane^` 参数，
            // 字段 `generateTime` 属于 DremuLane 而非当前类。
            if let Some(field_info) = self.resolve_object_injector_field(object, field_name) {
                let (field_idx, vt) = field_info;
                let temp = self.alloc_temp(vt);
                let load_op = Self::load_injector_field_op(vt, field_idx);
                self.emit(
                    IntermediateCode::new(load_op, obj_op, None, Some(temp)),
                    object.span(),
                );
                return Operand::Address(temp);
            }
            self.diagnostics.emit_error(object.span(), &format!("未定义的注入器字段 `^{}`", field_name));
            return Operand::Address(Address::new(ValueType::Int, 0));
        }
        // 枚举成员访问 Enum.Value → 直接替换为枚举整数值（对齐 C# 枚举即整数）
        // 仅当标识符不是局部变量/当前类字段时按枚举解析，避免遮蔽
        if let Expression::Identifier(name, _) = object {
            if self.lookup_var(name).is_none() && !self.field_info.contains_key(name) {
                let scope = self.class_lookup_scope();
                if let Some(enum_id) = self.symbol_table.find_enum_by_name(scope, name) {
                    let enum_info = self.symbol_table.enums.get(enum_id.0);
                    for (i, &vid) in enum_info.values.iter().enumerate() {
                        let vi = self.symbol_table.enum_values.get(vid.0);
                        if vi.name == member {
                            // 显式值优先，缺省按声明序号（对齐 C# 枚举默认值规则）
                            let v = vi.value.unwrap_or(i as i64);
                            return Operand::int(v);
                        }
                    }
                    self.diagnostics.emit_error(
                        object.span(),
                        format!("枚举 `{}` 中未定义的值 `{}`", name, member),
                    );
                    return Operand::int(0);
                }
            }
        }
        // this.field → 生成 LoadField（对齐 C#：显式 LoadThis 再 LoadField）
        if matches!(object, Expression::This(_)) {
            if let Some(&(offset, vt)) = self.field_info.get(member) {
                let this_temp = self.alloc_temp(ValueType::Object);
                self.emit(
                    IntermediateCode::new(
                        IntermediateOperator::LoadThis,
                        Operand::int(0), None, Some(this_temp),
                    ),
                    object.span(),
                );
                let temp = self.alloc_temp(vt);
                let load_op = Self::load_field_op(vt, offset);
                self.emit(
                    IntermediateCode::new(load_op, Operand::Address(this_temp), None, Some(temp)),
                    object.span(),
                );
                return Operand::Address(temp);
            }
        }
        // obj.field（非 this）→ 求值接收器对象，再 LoadField
        let obj_op = self.generate_expression(object);
        // 尝试从变量声明的类中查找字段（而非当前类的 field_info）
        let field_lookup = self.lookup_field_for_object(object, member);
        if let Some((offset, vt)) = field_lookup.or_else(|| self.field_info.get(member).copied()) {
            let temp = self.alloc_temp(vt);
            let load_op = Self::load_field_op(vt, offset);
            self.emit(
                IntermediateCode::new(load_op, obj_op, None, Some(temp)),
                object.span(),
            );
            return Operand::Address(temp);
        }
        self.diagnostics.emit_error(object.span(), format!("未定义的字段 `{}`", member));
        Operand::Address(Address::new(ValueType::Int, 0))
    }

    /// 按对象的声明类型解析注入器字段（对齐 C# 按对象声明类型查找注入器字段）。
    ///
    /// 用于 `obj.^field`（非 this）且字段不属于当前类注入器字段的场景，如
    /// `@PeriodModifier` 的 `laneInjector.^generateTime`（laneInjector 是
    /// `DremuLane^` 参数）。运行时注入器字段布局 = 祖先类字段 + 自有字段，
    /// 故字段索引须沿继承链从根类开始累加偏移。
    fn resolve_object_injector_field(&self, object: &Expression, field_name: &str) -> Option<(usize, ValueType)> {
        // 解析对象声明的类名（参数/局部变量经 var_class 记录）
        let class_name = match object {
            Expression::Identifier(name, _) => self.var_class.get(name).cloned(),
            Expression::This(_) => self.current_class_name.clone(),
            _ => None,
        }?;
        // 收集 类 → 祖先 链（本类在前，根类在后）
        let mut chain: Vec<String> = vec![class_name.clone()];
        loop {
            let scope = self.class_lookup_scope();
            let next = self.symbol_table
                .lookup_class(scope, chain.last().unwrap())
                .map(|cid| self.symbol_table.classes.get(cid.0).super_class)
                .flatten()
                .map(|cid| self.symbol_table.classes.get(cid.0).name.clone());
            match next {
                Some(n) => chain.push(n),
                None => break,
            }
        }
        // 从根类向下累加注入器字段偏移（运行时布局 = 祖先字段 + 自有字段）
        let mut offset = 0usize;
        for cls in chain.iter().rev() {
            if let Some(fields) = self.injector_fields.get(cls) {
                for (i, f) in fields.iter().enumerate() {
                    if f.name == field_name {
                        return Some((offset + i, f.value_type));
                    }
                }
                offset += fields.len();
            }
        }
        None
    }

    /// 生成方法调用代码
    fn generate_method_call(
        &mut self,
        receiver: &Expression,
        method: &str,
        arguments: &[Expression],
        span: Span,
    ) -> Operand {
        // 检查是否为静态方法调用 ClassName.method(args)
        if let Expression::Identifier(class_name, _) = receiver {
            let scope = self.class_lookup_scope();
            if let Some(class_id) = self.symbol_table.lookup_class(scope, class_name) {
                let class_info = self.symbol_table.classes.get(class_id.0);
                // 在类方法中查找匹配的静态方法
                for (i, &method_id) in class_info.methods.iter().enumerate() {
                    let mi = self.symbol_table.methods.get(method_id.0);
                    if mi.name == method && mi.is_static {
                        // 参数数量校验：同名静态方法存在但无匹配 arity 时报错
                        self.check_method_arg_count(class_name, method, true, arguments.len(), span);
                        // 先求值所有实参表达式（避免嵌套调用的 emit_set_param 覆写参数池）
                        let arg_ops: Vec<Operand> = arguments.iter().map(|a| self.generate_expression(a)).collect();
                        self.param_counters.reset();
                        for arg_op in &arg_ops {
                            self.emit_set_param(arg_op.clone(), span);
                        }
                        // 生成 InvokeStatic（left 操作数存参数计数）
                        // native 类的方法编号按同类方法独立计数（静态方法与实例方法分开编号）
                        let idx = if class_info.is_native {
                            class_info.methods.iter().take(i).filter(|&&mid| {
                                self.symbol_table.methods.get(mid.0).is_static
                            }).count()
                        } else {
                            i
                        };
                        let result = self.alloc_temp(Self::type_to_value_type(&mi.return_type));
                        self.emit(
                            IntermediateCode::new(
                                IntermediateOperator::InvokeStatic(idx),
                                Operand::int(arguments.len() as i64),
                                Some(Operand::string(class_name.clone())),
                                Some(result),
                            ),
                            span,
                        );
                        return Operand::Address(result);
                    }
                }
            }
        }

        // 原有的委托/实例调用逻辑
        let delegate_idx = match receiver {
            Expression::Identifier(name, _) => self.delegate_vars.get(name).copied(),
            _ => None,
        };

        // 非委托调用：尝试查找实例方法
        if delegate_idx.is_none() {
            if let Expression::Identifier(class_name, _) = receiver {
                let scope = self.class_lookup_scope();
                if let Some(class_id) = self.symbol_table.lookup_class(scope, class_name) {
                    let class_info = self.symbol_table.classes.get(class_id.0);
                    for (i, &method_id) in class_info.methods.iter().enumerate() {
                        let mi = self.symbol_table.methods.get(method_id.0);
                        if mi.name == method && !mi.is_static {
                            let arg_ops: Vec<Operand> = arguments.iter().map(|a| self.generate_expression(a)).collect();
                            self.param_counters.reset();
                            for arg_op in &arg_ops {
                                self.emit_set_param(arg_op.clone(), span);
                            }
                            let result = self.alloc_temp(Self::type_to_value_type(&mi.return_type));
                            // native 类的方法编号按同类方法独立计数
                            let idx = if class_info.is_native {
                                class_info.methods.iter().take(i).filter(|&&mid| {
                                    !self.symbol_table.methods.get(mid.0).is_static
                                }).count()
                            } else {
                                i
                            };
                            self.emit(
                                IntermediateCode::new(
                                    IntermediateOperator::InvokeInstance(idx),
                                    Operand::int(arguments.len() as i64),
                                    None,
                                    Some(result),
                                ),
                                span,
                            );
                            return Operand::Address(result);
                        }
                    }
                }
            }
            // 变量.方法() 或 new Class().方法()：参数与接收者各只求值一次，
            // 避免重复发射求值 IR（副作用重复执行）与重复诊断
            let arg_ops: Vec<Operand> = arguments.iter().map(|a| self.generate_expression(a)).collect();
            let recv_op = self.generate_expression(receiver);
            // 解析实例方法编号与返回类型（含成员链、数组元素等接收者）。
            let receiver_type = self.resolve_object_type(receiver).or_else(|| match receiver {
                Expression::New { class_type, .. } => Some(self.resolve_type_ref(class_type)),
                _ => None,
            });
            let arg_types: Vec<TypeInfo> = arguments.iter()
                .map(|argument| self.infer_type(argument))
                .collect();
            match receiver_type {
                Some(TypeInfo::Object(class_id)) => {
                    let class_name = self.symbol_table.classes.get(class_id.0).name.clone();
                    self.check_method_arg_count(&class_name, method, false, arguments.len(), span);
                    if let Ok(Some((method_idx, return_type))) =
                        self.resolve_instance_method(&class_name, method, &arg_types)
                    {
                        self.param_counters.reset();
                        for arg_op in &arg_ops {
                            self.emit_set_param(arg_op.clone(), span);
                        }
                        let result = self.alloc_temp(Self::type_to_value_type(&return_type));
                        self.emit(IntermediateCode::new(
                            IntermediateOperator::InvokeInstance(method_idx),
                            recv_op,
                            Some(Operand::string(class_name)),
                            Some(result),
                        ), span);
                        return Operand::Address(result);
                    }
                }
                Some(TypeInfo::Interface(interface_id)) => {
                    if let Some((method_idx, interface_name, return_type)) =
                        self.resolve_interface_method(interface_id, method, &arg_types)
                    {
                        self.param_counters.reset();
                        for arg_op in &arg_ops {
                            self.emit_set_param(arg_op.clone(), span);
                        }
                        let result = self.alloc_temp(Self::type_to_value_type(&return_type));
                        self.emit(IntermediateCode::new(
                            IntermediateOperator::InvokeInterface(method_idx),
                            recv_op,
                            Some(Operand::string(interface_name)),
                            Some(result),
                        ), span);
                        return Operand::Address(result);
                    }
                }
                _ => {}
            }
            // 无法静态解析（如变量调用），发出 InvokeInstance(0) 让 VM 运行时分派
            // 注意：先求值接收者再布置参数，避免接收者中的嵌套调用覆写参数池
            self.param_counters.reset();
            for arg_op in &arg_ops {
                self.emit_set_param(arg_op.clone(), span);
            }
            let result = self.alloc_temp(ValueType::Int);
            self.emit(
                IntermediateCode::new(
                    IntermediateOperator::InvokeInstance(0),
                    recv_op,
                    None,
                    Some(result),
                ),
                span,
            );
            return Operand::Address(result);
        }

        // 委托调用
        let recv_op = self.generate_expression(receiver);
        let arg_ops: Vec<Operand> = arguments.iter().map(|a| self.generate_expression(a)).collect();
        self.param_counters.reset();
        for arg_op in &arg_ops {
            self.emit_set_param(arg_op.clone(), span);
        }

        let result_vt = delegate_idx
            .and_then(|idx| self.delegate_impls.get(idx))
            .map(|d| d.return_type)
            .unwrap_or(ValueType::Int);
        let result = self.alloc_temp(result_vt);
        let op = match delegate_idx {
            Some(idx) => IntermediateOperator::InvokeDelegate(idx),
            None => IntermediateOperator::Nop,
        };
        self.emit(
            IntermediateCode::new(op, recv_op, None, Some(result)),
            span,
        );
        Operand::Address(result)
    }

    /// 生成委托调用代码（本地变量调用 d1(1)）
    fn generate_delegate_call(
        &mut self,
        var_name: &str,
        arguments: &[Expression],
        span: Span,
    ) -> Operand {
        let recv_op = match self.lookup_var(var_name) {
            Some(addr) => Operand::Address(addr),
            None => {
                // 隐式 this 调用：无接收者的 `Method(args)` 回退为 `this.Method(args)`
                // （对齐 C# 成员方法可省略 this 直接调用）
                if let Some(result) = self.try_generate_implicit_this_call(var_name, arguments, span) {
                    return result;
                }
                self.diagnostics.emit_error(span, format!("未定义的变量 `{}`", var_name));
                return Operand::Address(self.alloc_temp(ValueType::Int));
            }
        };

        let delegate_idx = self.delegate_vars.get(var_name).copied();

        let arg_ops: Vec<Operand> = arguments.iter().map(|a| self.generate_expression(a)).collect();
        self.param_counters.reset();
        for arg_op in &arg_ops {
            self.emit_set_param(arg_op.clone(), span);
        }

        let result_vt = delegate_idx
            .and_then(|idx| self.delegate_impls.get(idx))
            .map(|d| d.return_type)
            .unwrap_or(ValueType::Int);
        let result = self.alloc_temp(result_vt);
        let op = match delegate_idx {
            Some(idx) => IntermediateOperator::InvokeDelegate(idx),
            None => IntermediateOperator::InvokeDelegate(0),
        };
        self.emit(
            IntermediateCode::new(op, recv_op, None, Some(result)),
            span,
        );
        Operand::Address(result)
    }

    /// 尝试将无接收者调用 `Method(args)` 解析为隐式 this 调用 `this.Method(args)`
    ///
    /// 查找顺序：当前类及父类的实例方法（含重载解析）→ 当前类的静态方法。
    /// 均不匹配时返回 None，由调用方报「未定义的变量」。
    fn try_generate_implicit_this_call(
        &mut self,
        method: &str,
        arguments: &[Expression],
        span: Span,
    ) -> Option<Operand> {
        let class_name = self.current_class_name.clone()?;
        let arg_types: Vec<TypeInfo> = arguments.iter().map(|a| {
            match a { Expression::Literal(Literal::Int(_), _) => TypeInfo::Int, Expression::Literal(Literal::Float(_), _) => TypeInfo::Float, Expression::Literal(Literal::Bool(_), _) => TypeInfo::Bool, Expression::Literal(Literal::String(_), _) => TypeInfo::String, _ => TypeInfo::Unresolved }
        }).collect();

        // 1. 实例方法（含继承链与重载解析，与 var.Method() 路径一致）
        if let Ok(Some((method_idx, ret_vt))) =
            self.resolve_instance_method(&class_name, method, &arg_types)
        {
            let arg_ops: Vec<Operand> =
                arguments.iter().map(|a| self.generate_expression(a)).collect();
            self.param_counters.reset();
            for arg_op in &arg_ops {
                self.emit_set_param(arg_op.clone(), span);
            }
            // this 固定为地址 0 的 Object（与 This 表达式一致）
            let this_op = Operand::Address(Address::new(ValueType::Object, 0));
            let result = self.alloc_temp(Self::type_to_value_type(&ret_vt));
            self.emit(
                IntermediateCode::new(
                    IntermediateOperator::InvokeInstance(method_idx),
                    this_op,
                    Some(Operand::string(class_name)),
                    Some(result),
                ),
                span,
            );
            return Some(Operand::Address(result));
        }

        // 2. 当前类的静态方法（分派逻辑与 ClassName.Method() 静态分支一致）
        let scope = self.class_lookup_scope();
        let class_id = self.symbol_table.lookup_class(scope, &class_name)?;
        let class_info = self.symbol_table.classes.get(class_id.0);
        for (i, &method_id) in class_info.methods.iter().enumerate() {
            let mi = self.symbol_table.methods.get(method_id.0);
            if mi.name == method && mi.is_static {
                self.check_method_arg_count(&class_name, method, true, arguments.len(), span);
                let arg_ops: Vec<Operand> =
                    arguments.iter().map(|a| self.generate_expression(a)).collect();
                self.param_counters.reset();
                for arg_op in &arg_ops {
                    self.emit_set_param(arg_op.clone(), span);
                }
                // native 类的方法编号按同类方法独立计数（静态方法与实例方法分开编号）
                let idx = if class_info.is_native {
                    class_info.methods.iter().take(i).filter(|&&mid| {
                        self.symbol_table.methods.get(mid.0).is_static
                    }).count()
                } else {
                    i
                };
                let result = self.alloc_temp(Self::type_to_value_type(&mi.return_type));
                self.emit(
                    IntermediateCode::new(
                        IntermediateOperator::InvokeStatic(idx),
                        Operand::int(arguments.len() as i64),
                        Some(Operand::string(class_name.clone())),
                        Some(result),
                    ),
                    span,
                );
                return Some(Operand::Address(result));
            }
        }
        None
    }

    /// 生成 new 表达式代码
    fn generate_new(
        &mut self,
        class_type: &TypeRef,
        arguments: &[Expression],
        injector: Option<&[(String, Expression)]>,
        span: Span,
    ) -> Operand {
        // 若有注入器字段，先创建注入器常量并用 SetInjector 激活（G3）
        if let Some(fields) = injector {
            let class_name = match class_type {
                TypeRef::Simple { name, .. } => name.clone(),
                _ => String::new(),
            };
            let const_fields: Vec<InjectorConstField> = fields
                .iter()
                .filter_map(|(name, val_expr)| {
                    self.try_eval_const(val_expr).map(|mut cf| {
                        // 将字段名写入常量字段（InjectObject 保留类名，见 try_eval_const）
                        match &mut cf {
                            InjectorConstField::Int(n, _) | InjectorConstField::Float(n, _)
                            | InjectorConstField::Bool(n, _) | InjectorConstField::String(n, _)
                            | InjectorConstField::Object(n, _) => *n = name.clone(),
                            _ => {}
                        }
                        cf
                    })
                })
                .collect();
            let idx = self.injector_constants.len();
            self.injector_constants.push(InjectorConstantDef { class_name: class_name.clone(), fields: const_fields });
            // 创建注入器对象
            let inj_temp = self.alloc_temp(ValueType::Object);
            self.emit(IntermediateCode::new(IntermediateOperator::LoadInjectorConstant(idx), Operand::int(0), None, Some(inj_temp)), span);
            // SetInjector 使当前构造过程使用此注入器
            self.emit(IntermediateCode::new(IntermediateOperator::SetInjector, Operand::Address(inj_temp), None, None), span);
        }
        // 设置构造参数
        let arg_ops: Vec<Operand> = arguments.iter().map(|a| self.generate_expression(a)).collect();
        self.param_counters.reset();
        for arg_op in &arg_ops {
            self.emit_set_param(arg_op.clone(), span);
        }
        // 调用构造方法（B-5: 根据构造方法是否为 injector 选择操作码）
        let target = match class_type { TypeRef::Simple { name, .. } => Some(name.clone()), _ => None };
        let scope = self.class_lookup_scope();
        let (is_injector, injector_local_id, global_ctor_id) = target.as_ref().and_then(|name| {
            self.symbol_table.lookup_class(scope, name).map(|cid| {
                let ci = self.symbol_table.classes.get(cid.0);
                let start = ci.constructor_start_id;
                for (i, ctor_cid) in ci.constructors.iter().enumerate() {
                    let ct = self.symbol_table.constructors.get(ctor_cid.0);
                    if ct.parameters.len() == arguments.len() {
                        return (ct.is_injector, ct.injector_local_id, start + i);
                    }
                }
                (false, None, start)
            })
        }).unwrap_or((false, None, 0));
        let temp = self.alloc_temp(ValueType::Object);
        if is_injector {
            if let Some(local_id) = injector_local_id {
                self.emit(
                    IntermediateCode::new(
                        IntermediateOperator::InvokeInjectorConstructor(local_id),
                        Operand::int(arguments.len() as i64),
                        target.map(Operand::string),
                        Some(temp),
                    ),
                    span,
                );
            } else {
                self.emit(
                    IntermediateCode::new(
                        IntermediateOperator::InvokeConstructor(global_ctor_id),
                        Operand::int(arguments.len() as i64),
                        target.map(Operand::string),
                        Some(temp),
                    ),
                    span,
                );
            }
        } else {
            self.emit(
                IntermediateCode::new(
                    IntermediateOperator::InvokeConstructor(global_ctor_id),
                    Operand::int(arguments.len() as i64),
                    target.map(Operand::string),
                    Some(temp),
                ),
                span,
            );
        }
        Operand::Address(temp)
    }

    /// 生成注入器字段构造表达式 `new ^field(args)` 代码
    ///
    /// 从当前类的字段声明中反查注入器字段对应的目标类名，
    /// 加载注入器字段值作为注入器上下文，然后委托给 `generate_new` 完成构造。
    fn generate_injector_new(
        &mut self,
        injector_field: &str,
        args: &[Expression],
        span: Span,
    ) -> Operand {
        // 从当前类的字段中查找注入器字段对应的目标类名
        let scope = self.class_lookup_scope();
        let target_class_name = self.current_class_name.as_ref().and_then(|cn| {
            self.symbol_table.lookup_class(scope, cn).and_then(|class_id| {
                let ci = self.symbol_table.classes.get(class_id.0);
                ci.fields.iter().find_map(|&fid| {
                    let fi = self.symbol_table.fields.get(fid.0);
                    if fi.name == injector_field {
                        match &fi.field_type {
                            TypeInfo::Object(cls_id) => {
                                Some(self.symbol_table.classes.get(cls_id.0).name.clone())
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
            })
        });

        // 加载注入器字段值并设置为当前注入器上下文
        if let Some(&(field_idx, _vt)) = self.injector_field_info.get(injector_field) {
            let temp_inj = self.alloc_temp(ValueType::Object);
            self.emit(IntermediateCode::new(
                IntermediateOperator::LoadInjector,
                Operand::int(0),
                None,
                Some(temp_inj),
            ), span);
            let temp_field = self.alloc_temp(ValueType::Object);
            self.emit(IntermediateCode::new(
                IntermediateOperator::LoadObjectInjectorField(field_idx),
                Operand::Address(temp_inj),
                None,
                Some(temp_field),
            ), span);
            self.emit(IntermediateCode::new(
                IntermediateOperator::SetInjector,
                Operand::Address(temp_field),
                None,
                None,
            ), span);
        }

        if let Some(ref class_name) = target_class_name {
            let class_type = TypeRef::Simple { name: class_name.clone(), span };
            self.generate_new(&class_type, args, None, span)
        } else {
            self.diagnostics.emit_error(span, &format!("无法确定注入器字段 `^{}` 的对应类型", injector_field));
            Operand::Address(self.alloc_temp(ValueType::Object))
        }
    }

    /// 生成条件表达式 `?:` 代码
    fn generate_conditional(
        &mut self,
        condition: &Expression,
        then_branch: &Expression,
        else_branch: Option<&Expression>,
        span: Span,
    ) -> Operand {
        let cond_op = self.generate_expression(condition);
        let vt = Self::operand_value_type(&cond_op);

        // 跳转到 else 分支的占位符
        let jump_to_else_index = self.codes.len();
        self.emit(IntermediateCode::jump_if_false(cond_op, 0), span);

        let result = self.alloc_temp(vt);
        let then_op = self.generate_expression(then_branch);
        self.emit(IntermediateCode::assign(result, then_op), span);

        let jump_to_end_index = self.codes.len();
        self.emit(IntermediateCode::jump(0), span);

        // 回填 else 跳转目标
        let else_start = self.codes.len();
        if let Some(code) = self.codes.get_mut(jump_to_else_index) {
            code.code.operator = IntermediateOperator::JumpIfFalse(else_start);
        }

        if let Some(else_expr) = else_branch {
            let else_op = self.generate_expression(else_expr);
            self.emit(IntermediateCode::assign(result, else_op), span);
        }

        // 回填结束跳转目标
        let end = self.codes.len();
        if let Some(code) = self.codes.get_mut(jump_to_end_index) {
            code.code.operator = IntermediateOperator::Jump(end);
        }

        Operand::Address(result)
    }

    /// 分析表达式中的自由变量（非参数、非局部变量的标识符）
    fn analyze_free_vars(
        expr: &Expression,
        params: &HashSet<String>,
    ) -> Vec<String> {
        let mut vars = Vec::new();
        Self::collect_free_vars(expr, params, &mut vars);
        vars
    }

    fn collect_free_vars(
        expr: &Expression,
        params: &HashSet<String>,
        vars: &mut Vec<String>,
    ) {
        match expr {
            Expression::Identifier(name, _) => {
                if !params.contains(name) && !vars.contains(name) {
                    vars.push(name.clone());
                }
            }
            Expression::Binary { left, right, .. } => {
                Self::collect_free_vars(left, params, vars);
                Self::collect_free_vars(right, params, vars);
            }
            Expression::Unary { operand, .. } => {
                Self::collect_free_vars(operand, params, vars);
            }
            Expression::Conditional { condition, then_branch, else_branch, .. } => {
                Self::collect_free_vars(condition, params, vars);
                Self::collect_free_vars(then_branch, params, vars);
                if let Some(else_e) = else_branch {
                    Self::collect_free_vars(else_e, params, vars);
                }
            }
            Expression::Assignment { value, .. } => {
                Self::collect_free_vars(value, params, vars);
            }
            Expression::MethodCall { receiver, arguments, .. } => {
                Self::collect_free_vars(receiver, params, vars);
                for arg in arguments {
                    Self::collect_free_vars(arg, params, vars);
                }
            }
            Expression::MemberAccess { object, .. } => {
                Self::collect_free_vars(object, params, vars);
            }
            Expression::New { arguments, .. } => {
                for arg in arguments {
                    Self::collect_free_vars(arg, params, vars);
                }
            }
            Expression::InjectorNew { args, .. } => {
                for arg in args {
                    Self::collect_free_vars(arg, params, vars);
                }
            }
            _ => {}
        }
    }

    /// 分析 LambdaBody 中的自由变量
    fn analyze_free_vars_lambda_body(
        body: &LambdaBody,
        params: &HashSet<String>,
    ) -> Vec<String> {
        match body {
            LambdaBody::Expression(expr) => Self::analyze_free_vars(expr, params),
            LambdaBody::Block(stmts) => {
                let mut vars = Vec::new();
                for stmt in stmts {
                    Self::collect_free_vars_from_stmt(stmt, params, &mut vars);
                }
                vars
            }
        }
    }

    /// 从语句中递归收集自由变量
    fn collect_free_vars_from_stmt(
        stmt: &Statement,
        params: &HashSet<String>,
        vars: &mut Vec<String>,
    ) {
        match stmt {
            Statement::Expression(expr, _) => Self::collect_free_vars(expr, params, vars),
            Statement::VariableDeclaration { initializer, .. } => {
                if let Some(init) = initializer {
                    Self::collect_free_vars(init, params, vars);
                }
            }
            Statement::Return { value, .. } => {
                if let Some(v) = value {
                    Self::collect_free_vars(v, params, vars);
                }
            }
            Statement::If { condition, then_branch, else_branch, .. } => {
                Self::collect_free_vars(condition, params, vars);
                Self::collect_free_vars_from_stmt(then_branch, params, vars);
                if let Some(else_s) = else_branch {
                    Self::collect_free_vars_from_stmt(else_s, params, vars);
                }
            }
            Statement::While { condition, body, .. } => {
                Self::collect_free_vars(condition, params, vars);
                Self::collect_free_vars_from_stmt(body, params, vars);
            }
            Statement::For { initializer, condition, update, body, .. } => {
                if let Some(init) = initializer { Self::collect_free_vars_from_stmt(init, params, vars); }
                if let Some(cond) = condition { Self::collect_free_vars(cond, params, vars); }
                if let Some(upd) = update { Self::collect_free_vars(upd, params, vars); }
                Self::collect_free_vars_from_stmt(body, params, vars);
            }
            Statement::Block { statements, .. } => {
                for s in statements {
                    Self::collect_free_vars_from_stmt(s, params, vars);
                }
            }
            Statement::Switch { expression, cases, default_body, .. } => {
                Self::collect_free_vars(expression, params, vars);
                for case in cases {
                    for s in &case.body {
                        Self::collect_free_vars_from_stmt(s, params, vars);
                    }
                }
                if let Some(def) = default_body {
                    Self::collect_free_vars_from_stmt(def, params, vars);
                }
            }
            Statement::DoWhile { condition, body, .. } => {
                Self::collect_free_vars_from_stmt(body, params, vars);
                Self::collect_free_vars(condition, params, vars);
            }
            _ => {} // Break/Continue 无表达式子结构
        }
    }

    // ==================== 语句代码生成 ====================

    /// 为语句生成代码
    pub fn generate_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Expression(expr, _span) => {
                let _ = self.generate_expression(expr);
            }
            Statement::VariableDeclaration { var_type, name, initializer, span } => {
                let _vt = if let Some(init) = initializer {
                    let prev_count = self.delegate_impls.len();
                    let prev_inj_count = self.injector_constants.len();
                    let result = self.generate_expression(init);
                    let vt = Self::operand_value_type(&result);
                    let addr = self.declare_local(name, vt);
                    self.emit(IntermediateCode::assign(addr, result), *span);
                    // 如果初始化表达式生成了新委托，记录变量映射
                    if self.delegate_impls.len() > prev_count {
                        let delegate_idx = self.delegate_impls.len() - 1;
                        self.delegate_vars.insert(name.clone(), delegate_idx);
                    }
                    // 记录变量的完整类型与类名，供后续 `v.method()` 解析实例方法编号/返回类型。
                    // 显式类型优先按声明解析；`auto` 时从初始化表达式推导。
                    let declared = self.resolve_type_ref(var_type);
                    let inferred = match declared {
                        TypeInfo::Unresolved | TypeInfo::Void => self.infer_type(init),
                        other => other,
                    };
                    if let TypeInfo::Object(cid) = &inferred {
                        let cname = self.symbol_table.classes.get(cid.0).name.clone();
                        self.register_var_class(name, &cname);
                    }
                    if !matches!(inferred, TypeInfo::Unresolved) {
                        self.register_var_type(name, inferred);
                    }
                    // 追踪注入器数组变量（如 `int[]^ listB = ...`），
                    // 记录元素类型供 `new listB[n]` 数组构造解析
                    if let TypeRef::Injector { base_type, .. } = var_type {
                        if let TypeRef::Array { element_type, .. } = base_type.as_ref() {
                            if let TypeRef::Simple { name: elem_name, .. } = element_type.as_ref() {
                                self.var_injector_array_elem.insert(name.clone(), elem_name.clone());
                                // 记录对应注入器常量的索引（刚由 generate_expression 创建）
                                if self.injector_constants.len() > prev_inj_count {
                                    self.var_injector_const_idx.insert(name.clone(), prev_inj_count);
                                }
                            }
                        }
                    }
                    vt
                } else {
                    // 无初始化器的变量声明，默认值
                    let vt = ValueType::Int; // 简化：需要从 var_type 推导
                    let _addr = self.declare_local(name, vt);
                    vt
                };
            }
            Statement::Block { statements, span: _ } => {
                for s in statements {
                    self.generate_statement(s);
                }
            }
            Statement::If { condition, then_branch, else_branch, span } => {
                self.generate_if_statement(condition, then_branch, else_branch.as_deref(), *span);
            }
            Statement::While { condition, body, span } => {
                self.generate_while_statement(condition, body, *span);
            }
            Statement::Return { value, span } => {
                if let Some(expr) = value {
                    let result = self.generate_expression(expr);
                    let vt = Self::operand_value_type(&result);
                    let ret_addr = Address::new(vt, 0);
                    self.emit(IntermediateCode::assign(ret_addr, result), *span);
                    self.emit(IntermediateCode::return_value(vt), *span);
                } else {
                    self.emit(IntermediateCode::return_void(), *span);
                }
            }
            Statement::For { initializer, condition, update, body, span: _ } => {
                // for (init; cond; update) { body }
                if let Some(init) = initializer {
                    self.generate_statement(init);
                }

                let leaves_since = self.pending_leaves.len();
                let loop_start = self.codes.len();

                let mut jump_to_end_index = None;
                if let Some(cond) = condition {
                    let cond_op = self.generate_expression(cond);
                    jump_to_end_index = Some(self.codes.len());
                    self.emit(IntermediateCode::jump_if_false(cond_op, 0), cond.span());
                }

                self.generate_statement(body);

                // continue 落点：update 段起始处
                let update_start = self.codes.len();
                if let Some(upd) = update {
                    let _ = self.generate_expression(upd);
                }

                self.emit(IntermediateCode::jump(loop_start), Span::dummy());

                // 回填结束跳转
                let end = self.codes.len();
                if let Some(idx) = jump_to_end_index {
                    if let Some(code) = self.codes.get_mut(idx) {
                        code.code.operator = IntermediateOperator::JumpIfFalse(end);
                    }
                }
                // 回填本块内的 break（→end）/continue（→update 段）
                self.backpatch_block(BlockKind::For, false, end, Some(update_start), leaves_since, self.pending_leaves.len());
            }
            Statement::DoWhile { body, condition, span } => {
                let leaves_since = self.pending_leaves.len();
                let loop_start = self.codes.len();
                self.generate_statement(body);
                // continue 落点：条件判断处
                let cond_start = self.codes.len();
                let cond_op = self.generate_expression(condition);
                self.emit(IntermediateCode::jump_if_true(cond_op, loop_start), *span);
                let end = self.codes.len();
                // 回填本块内的 break（→end）/continue（→条件判断处）
                self.backpatch_block(BlockKind::DoWhile, false, end, Some(cond_start), leaves_since, self.pending_leaves.len());
            }
            Statement::Switch { expression, cases, default_body, span } => {
                self.generate_switch(expression, cases, default_body.as_deref(), *span);
            }
            Statement::Break { targets, span } => {
                self.emit_leave(true, targets, *span);
            }
            Statement::Continue { targets, span } => {
                self.emit_leave(false, targets, *span);
            }
        }
    }

    /// 生成 if 语句代码
    fn generate_if_statement(
        &mut self,
        condition: &Expression,
        then_branch: &Statement,
        else_branch: Option<&Statement>,
        span: Span,
    ) {
        let cond_op = self.generate_expression(condition);

        let jump_to_else_index = self.codes.len();
        self.emit(IntermediateCode::jump_if_false(cond_op, 0), span);

        // then 分支内的离块任务区间起点
        let then_since = self.pending_leaves.len();
        self.generate_statement(then_branch);
        let then_until = self.pending_leaves.len();

        if else_branch.is_some() {
            let jump_to_end_index = self.codes.len();
            self.emit(IntermediateCode::jump(0), span);

            let else_start = self.codes.len();
            if let Some(code) = self.codes.get_mut(jump_to_else_index) {
                code.code.operator = IntermediateOperator::JumpIfFalse(else_start);
            }

            // else 分支内的离块任务区间
            let else_since = self.pending_leaves.len();
            self.generate_statement(else_branch.unwrap());
            let else_until = self.pending_leaves.len();

            let end = self.codes.len();
            if let Some(code) = self.codes.get_mut(jump_to_end_index) {
                code.code.operator = IntermediateOperator::Jump(end);
            }

            // then 分支离块任务：作为 if 块（is_else=false）消解，落点为块尾
            self.backpatch_block(BlockKind::If, false, end, None, then_since, then_until);
            // else 分支离块任务：作为 else 块（is_else=true）消解，落点为块尾
            self.backpatch_block(BlockKind::If, true, end, None, else_since, else_until);
        } else {
            let end = self.codes.len();
            if let Some(code) = self.codes.get_mut(jump_to_else_index) {
                code.code.operator = IntermediateOperator::JumpIfFalse(end);
            }
            // then 分支离块任务：作为 if 块（is_else=false）消解，落点为块尾
            self.backpatch_block(BlockKind::If, false, end, None, then_since, then_until);
        }
    }

    /// 生成 while 语句代码
    fn generate_while_statement(
        &mut self,
        condition: &Expression,
        body: &Statement,
        span: Span,
    ) {
        let leaves_since = self.pending_leaves.len();
        let loop_start = self.codes.len();

        let cond_op = self.generate_expression(condition);

        let jump_to_end_index = self.codes.len();
        self.emit(IntermediateCode::jump_if_false(cond_op, 0), span);

        self.generate_statement(body);

        self.emit(IntermediateCode::jump(loop_start), span);

        let end = self.codes.len();
        if let Some(code) = self.codes.get_mut(jump_to_end_index) {
            code.code.operator = IntermediateOperator::JumpIfFalse(end);
        }
        // 回填本块内的 break（→end）/continue（→条件复检处 loop_start）
        self.backpatch_block(BlockKind::While, false, end, Some(loop_start), leaves_since, self.pending_leaves.len());
    }

    /// 生成 switch 语句代码
    fn generate_switch(
        &mut self,
        expression: &Expression,
        cases: &[CaseBlock],
        default_body: Option<&Statement>,
        span: Span,
    ) {
        let switch_val = self.generate_expression(expression);
        let vt = Self::operand_value_type(&switch_val);

        // switch 条件类型校验（对齐 C# SwitchBlock）：只允许 int/float/bool/string。
        // 对象/其他类型无法用相等运算判定，报编译错误而非静默回退。
        if !matches!(vt, ValueType::Int | ValueType::Float | ValueType::Bool | ValueType::String) {
            self.diagnostics.emit_error(span, "switch 条件表达式类型必须为 int、float、bool 或 string。".to_string());
        }

        let leaves_since = self.pending_leaves.len();

        // 为每个 case 生成比较+条件跳转
        let mut case_jumps: Vec<usize> = Vec::new(); // 记录每个 case body 的跳转位置
        let mut case_bodies: Vec<usize> = Vec::new(); // 每个 case body 的起始位置

        for case in cases {
            for value in &case.values {
                let val_op = self.generate_expression(value);
                let case_vt = Self::operand_value_type(&val_op);
                // case 值类型须与 switch 类型兼容：相同，或 int→float 数值提升（对齐 C#）
                let compatible = case_vt == vt
                    || (vt == ValueType::Float && case_vt == ValueType::Int);
                if !compatible {
                    self.diagnostics.emit_error(value.span(), "case 值类型与 switch 条件类型不兼容。".to_string());
                }
                // case 值提升到 switch 类型后再比较（int→float）
                let val_op = self.emit_cast_operand(val_op, case_vt, vt, value.span());
                let cmp_result = self.alloc_temp(ValueType::Bool);
                let eq_op = match vt {
                    ValueType::Int => IntermediateOperator::IntEqual,
                    ValueType::Float => IntermediateOperator::FloatEqual,
                    ValueType::Bool => IntermediateOperator::BoolEqual,
                    ValueType::String => IntermediateOperator::StringEqual,
                    _ => IntermediateOperator::IntEqual,
                };
                self.emit(
                    IntermediateCode::binary(eq_op, switch_val.clone(), val_op, cmp_result),
                    span,
                );
                // 如果匹配，跳转到 case body（位置稍后回填）
                case_jumps.push(self.codes.len());
                self.emit(
                    IntermediateCode::jump_if_false(Operand::Address(cmp_result), 0),
                    span,
                );
            }

            case_bodies.push(self.codes.len());
            for stmt in &case.body {
                self.generate_statement(stmt);
            }
        }

        // default
        let default_start = self.codes.len();
        if let Some(stmt) = default_body {
            self.generate_statement(stmt);
        }

        // 回填所有 case 跳转到对应 body
        for &jump_idx in &case_jumps {
            // 计算距离：找到大于 jump_idx 的最小 case_body 位置
            let target = case_bodies
                .iter()
                .find(|&&b| b > jump_idx)
                .copied()
                .unwrap_or(default_start);
            if let Some(code) = self.codes.get_mut(jump_idx) {
                code.code.operator = IntermediateOperator::JumpIfTrue(target);
            }
        }

        // 回填本块内的 break（→switch 块尾）；continue 透传给外层循环（None）
        let end = self.codes.len();
        self.backpatch_block(BlockKind::Switch, false, end, None, leaves_since, self.pending_leaves.len());
    }

    /// 获取已使用局部变量的总数（用于栈帧大小计算）
    pub fn total_locals(&self) -> usize {
        self.next_local.values().sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile_context::symbol::SymbolTable;

    fn dummy_span() -> Span {
        Span::new(0, 1, 1, 1, 0)
    }

    /// 测试用空注入器字段表（泄漏以匹配 CodeGenerator 的借用生命周期）
    fn empty_injector_fields<'a>() -> &'a HashMap<String, Vec<InjectorFieldDef>> {
        Box::leak(Box::new(HashMap::new()))
    }

    fn make_codegen<'a>(
        st: &'a SymbolTable,
        diags: &'a mut Diagnostics,
        delegates: &'a mut Vec<DelegateImpl>,
    ) -> CodeGenerator<'a> {
        CodeGenerator::new(st, diags, delegates, empty_injector_fields(), Vec::new())
    }

    #[test]
    fn test_generate_literal_int() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let expr = Expression::Literal(Literal::Int(42), dummy_span());
        let result = cg.generate_expression(&expr);

        assert!(matches!(
            result,
            Operand::Immediate(ImmediateValue::Int(42))
        ));
        assert!(cg.codes.is_empty()); // 字面量不产生任何指令
    }

    #[test]
    fn test_generate_binary_add() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Int(1), dummy_span())),
            operator: BinaryOp::Add,
            right: Box::new(Expression::Literal(Literal::Int(2), dummy_span())),
            span: dummy_span(),
        };

        let _result = cg.generate_expression(&expr);
        assert_eq!(cg.codes.len(), 1); // 产生一条 IntAdd 指令
        assert!(matches!(
            cg.codes[0].code.operator,
            IntermediateOperator::IntAdd
        ));
    }

    #[test]
    fn test_generate_assignment() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let expr = Expression::Assignment {
            target: AssignmentTarget::Variable("x".into(), dummy_span()),
            operator: AssignmentOp::Assign,
            value: Box::new(Expression::Literal(Literal::Int(10), dummy_span())),
            span: dummy_span(),
        };

        let _result = cg.generate_expression(&expr);
        assert_eq!(cg.codes.len(), 1);
        assert!(matches!(
            cg.codes[0].code.operator,
            IntermediateOperator::IntAssign
        ));
    }

    #[test]
    fn test_generate_if_statement() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let stmt = Statement::If {
            condition: Expression::Literal(Literal::Bool(true), dummy_span()),
            then_branch: Box::new(Statement::Return {
                value: Some(Expression::Literal(Literal::Int(1), dummy_span())),
                span: dummy_span(),
            }),
            else_branch: None,
            span: dummy_span(),
        };

        cg.generate_statement(&stmt);
        assert!(cg.codes.len() >= 2);
        // 至少有条件跳转 + return
    }

    #[test]
    fn test_generate_return_int() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let stmt = Statement::Return {
            value: Some(Expression::Literal(Literal::Int(99), dummy_span())),
            span: dummy_span(),
        };

        cg.generate_statement(&stmt);
        assert!(cg.codes.len() >= 2);
        assert!(matches!(
            cg.codes.last().unwrap().code.operator,
            IntermediateOperator::ReturnInt
        ));
    }

    #[test]
    fn test_generate_while_loop() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let stmt = Statement::While {
            condition: Expression::Identifier("flag".into(), dummy_span()),
            body: Box::new(Statement::Block {
                statements: vec![],
                span: dummy_span(),
            }),
            span: dummy_span(),
        };

        cg.generate_statement(&stmt);
        // 应该生成 while 循环的代码（含跳转指令）
        assert!(cg.codes.iter().any(|c| matches!(
            c.code.operator,
            IntermediateOperator::JumpIfFalse(_) | IntermediateOperator::Jump(_)
        )));
    }

    // ==================== 比较运算结果类型修复测试 ====================

    #[test]
    fn test_float_comparison_result_is_bool() {
        // `1.5 > 0.5` 的结果应为 Bool 类型（此前误用操作数类型 Float，导致 bool 读错栈槽）
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Float(1.5), dummy_span())),
            operator: BinaryOp::Greater,
            right: Box::new(Expression::Literal(Literal::Float(0.5), dummy_span())),
            span: dummy_span(),
        };
        let result = cg.generate_expression(&expr);

        // 结果操作数应为 Bool 类型的地址
        assert_eq!(CodeGenerator::operand_value_type(&result), ValueType::Bool);
        // 生成的比较操作码为 FloatGreater
        assert!(cg.codes.iter().any(|c| matches!(
            c.code.operator,
            IntermediateOperator::FloatGreater
        )));
        // 比较指令的结果地址值类型为 Bool
        let cmp = cg.codes.iter().find(|c| matches!(c.code.operator, IntermediateOperator::FloatGreater)).unwrap();
        assert_eq!(cmp.code.result.unwrap().value_type, ValueType::Bool);
    }

    #[test]
    fn test_arithmetic_result_keeps_operand_type() {
        // `1.5 + 0.5` 的结果应仍为 Float（算术运算不受比较结果类型修复影响）
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Float(1.5), dummy_span())),
            operator: BinaryOp::Add,
            right: Box::new(Expression::Literal(Literal::Float(0.5), dummy_span())),
            span: dummy_span(),
        };
        let result = cg.generate_expression(&expr);
        assert_eq!(CodeGenerator::operand_value_type(&result), ValueType::Float);
    }

    #[test]
    fn test_generic_substitution_replaces_param_with_int() {
        // class Foo<T> 实例化为 Foo<int> 时，T 应被替换为 TypeInfo::Int
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.current_generic_params = vec!["T".to_string()];
        let mut subs = HashMap::new();
        subs.insert("T".to_string(), TypeInfo::Int);
        cg.set_generic_substitutions(subs);

        let tr = TypeRef::simple("T", dummy_span());
        let resolved = cg.resolve_type_ref(&tr);
        let vt = CodeGenerator::type_to_value_type(&resolved);
        assert_eq!(vt, ValueType::Int, "泛型参数 T 应被替换为 Int（值类型为 Int 而非 Object）");
    }

    #[test]
    fn test_generic_param_without_substitution_remains_object() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.current_generic_params = vec!["T".to_string()];

        let tr = TypeRef::simple("T", dummy_span());
        let resolved = cg.resolve_type_ref(&tr);
        let vt = CodeGenerator::type_to_value_type(&resolved);
        assert_eq!(vt, ValueType::Object, "无替换时泛型参数应映射为 Object");
    }

    // ==================== 二元运算类型提升 / StringAddition / FloatMod 测试 ====================

    /// 构造一个二元表达式并返回 (codegen 结果值类型, 生成的操作码是否含指定判定)。
    /// 由于 IntermediateOperator 未派生 PartialEq，用闭包判定代替 contains。
    fn gen_binary_ops(st: &SymbolTable, l: Literal, op: BinaryOp, r: Literal)
        -> (ValueType, Vec<IntermediateOperator>) {
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = CodeGenerator::new(st, &mut diags, &mut delegates, empty_injector_fields(), Vec::new());
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(l, dummy_span())),
            operator: op,
            right: Box::new(Expression::Literal(r, dummy_span())),
            span: dummy_span(),
        };
        let result = cg.generate_expression(&expr);
        let ops: Vec<IntermediateOperator> = cg.codes.iter().map(|c| c.code.operator.clone()).collect();
        (CodeGenerator::operand_value_type(&result), ops)
    }

    fn has_op(ops: &[IntermediateOperator], pred: impl Fn(&IntermediateOperator) -> bool) -> bool {
        ops.iter().any(pred)
    }

    #[test]
    fn test_int_plus_float_promotes_to_float() {
        // int + float → 结果 Float，int 操作数先 IntToFloat 提升，运算用 FloatAdd
        let st = SymbolTable::new();
        let (vt, ops) = gen_binary_ops(&st, Literal::Int(1), BinaryOp::Add, Literal::Float(2.0));
        assert_eq!(vt, ValueType::Float);
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::IntToFloat)));
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::FloatAdd)));
    }

    #[test]
    fn test_int_plus_string_is_string_concat() {
        // int + string → 字符串拼接，int 操作数 IntCastToString，运算用 StringAddition
        let st = SymbolTable::new();
        let (vt, ops) = gen_binary_ops(&st, Literal::Int(7), BinaryOp::Add, Literal::String("x".into()));
        assert_eq!(vt, ValueType::String);
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::IntCastToString)));
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::StringAddition)));
    }

    #[test]
    fn test_string_plus_bool_is_string_concat() {
        // string + bool → 拼接，bool 操作数 BoolCastToString
        let st = SymbolTable::new();
        let (vt, ops) = gen_binary_ops(&st, Literal::String("v=".into()), BinaryOp::Add, Literal::Bool(true));
        assert_eq!(vt, ValueType::String);
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::BoolCastToString)));
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::StringAddition)));
    }

    #[test]
    fn test_float_modulo_emits_float_mod() {
        // float % float → FloatMod（此前恒发 IntMod）
        let st = SymbolTable::new();
        let (vt, ops) = gen_binary_ops(&st, Literal::Float(5.0), BinaryOp::Modulo, Literal::Float(2.0));
        assert_eq!(vt, ValueType::Float);
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::FloatMod)));
        assert!(!has_op(&ops, |o| matches!(o, IntermediateOperator::IntMod)));
    }

    #[test]
    fn test_int_modulo_emits_int_mod() {
        // int % int → IntMod
        let st = SymbolTable::new();
        let (vt, ops) = gen_binary_ops(&st, Literal::Int(5), BinaryOp::Modulo, Literal::Int(2));
        assert_eq!(vt, ValueType::Int);
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::IntMod)));
    }

    #[test]
    fn test_int_equals_float_promotes() {
        // int == float → 结果 Bool，int 操作数提升为 float，用 FloatEqual
        let st = SymbolTable::new();
        let (vt, ops) = gen_binary_ops(&st, Literal::Int(1), BinaryOp::Equal, Literal::Float(1.0));
        assert_eq!(vt, ValueType::Bool);
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::IntToFloat)));
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::FloatEqual)));
    }

    #[test]
    fn test_int_less_than_float_promotes() {
        // int < float → 结果 Bool，用 FloatLess
        let st = SymbolTable::new();
        let (vt, ops) = gen_binary_ops(&st, Literal::Int(1), BinaryOp::Less, Literal::Float(2.0));
        assert_eq!(vt, ValueType::Bool);
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::FloatLess)));
    }

    #[test]
    fn test_string_equals_emits_string_equal() {
        // "a" == "b" → 结果 Bool，用 StringEqual
        let st = SymbolTable::new();
        let (vt, ops) = gen_binary_ops(&st, Literal::String("a".into()), BinaryOp::Equal, Literal::String("b".into()));
        assert_eq!(vt, ValueType::Bool);
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::StringEqual)));
    }

    #[test]
    fn test_string_not_equals_emits_string_not_equal() {
        // "a" != "b" → 结果 Bool，用 StringNotEqual
        let st = SymbolTable::new();
        let (vt, ops) = gen_binary_ops(&st, Literal::String("a".into()), BinaryOp::NotEqual, Literal::String("b".into()));
        assert_eq!(vt, ValueType::Bool);
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::StringNotEqual)));
    }

    #[test]
    fn test_null_equals_null_emits_object_equal() {
        // null == null → 结果 Bool，用 ObjectEqual
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = CodeGenerator::new(&st, &mut diags, &mut delegates, empty_injector_fields(), Vec::new());
        let expr = Expression::Binary {
            left: Box::new(Expression::Null(dummy_span())),
            operator: BinaryOp::Equal,
            right: Box::new(Expression::Null(dummy_span())),
            span: dummy_span(),
        };
        let result = cg.generate_expression(&expr);
        let ops: Vec<IntermediateOperator> = cg.codes.iter().map(|c| c.code.operator.clone()).collect();
        assert_eq!(CodeGenerator::operand_value_type(&result), ValueType::Bool);
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::ObjectEqual)));
    }

    #[test]
    fn test_null_uses_an_object_slot_other_than_this() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = CodeGenerator::new(&st, &mut diags, &mut delegates, empty_injector_fields(), Vec::new());

        let result = cg.generate_expression(&Expression::Null(dummy_span()));

        let Operand::Address(address) = result else {
            panic!("null 应生成对象地址");
        };
        assert_eq!(address.value_type, ValueType::Object);
        assert_ne!(address.index, 0, "对象地址 0 保留给 this，不可代表 null");
    }

    #[test]
    fn test_object_not_equals_emits_object_not_equal() {
        // null != null → 结果 Bool，用 ObjectNotEqual
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = CodeGenerator::new(&st, &mut diags, &mut delegates, empty_injector_fields(), Vec::new());
        let expr = Expression::Binary {
            left: Box::new(Expression::Null(dummy_span())),
            operator: BinaryOp::NotEqual,
            right: Box::new(Expression::Null(dummy_span())),
            span: dummy_span(),
        };
        let result = cg.generate_expression(&expr);
        let ops: Vec<IntermediateOperator> = cg.codes.iter().map(|c| c.code.operator.clone()).collect();
        assert_eq!(CodeGenerator::operand_value_type(&result), ValueType::Bool);
        assert!(has_op(&ops, |o| matches!(o, IntermediateOperator::ObjectNotEqual)));
    }

    #[test]
    fn test_illegal_arithmetic_operand_reports_error() {
        // string - int → 减法不支持字符串，应报编译错误
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = CodeGenerator::new(&st, &mut diags, &mut delegates, empty_injector_fields(), Vec::new());
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::String("a".into()), dummy_span())),
            operator: BinaryOp::Subtract,
            right: Box::new(Expression::Literal(Literal::Int(1), dummy_span())),
            span: dummy_span(),
        };
        let _ = cg.generate_expression(&expr);
        assert!(diags.has_errors());
    }

    // ==================== switch 条件/case 类型校验测试 ====================

    #[test]
    fn test_switch_int_condition_ok() {
        // switch(1) { case 1: } —— int 条件合法，不报错
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = CodeGenerator::new(&st, &mut diags, &mut delegates, empty_injector_fields(), Vec::new());
        let stmt = Statement::Switch {
            expression: Expression::Literal(Literal::Int(1), dummy_span()),
            cases: vec![CaseBlock {
                values: vec![Expression::Literal(Literal::Int(1), dummy_span())],
                body: vec![],
                span: dummy_span(),
            }],
            default_body: None,
            span: dummy_span(),
        };
        cg.generate_statement(&stmt);
        assert!(!diags.has_errors());
    }

    #[test]
    fn test_switch_incompatible_case_reports_error() {
        // switch(1) { case "x": } —— case string 与 int 条件不兼容，应报错
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = CodeGenerator::new(&st, &mut diags, &mut delegates, empty_injector_fields(), Vec::new());
        let stmt = Statement::Switch {
            expression: Expression::Literal(Literal::Int(1), dummy_span()),
            cases: vec![CaseBlock {
                values: vec![Expression::Literal(Literal::String("x".into()), dummy_span())],
                body: vec![],
                span: dummy_span(),
            }],
            default_body: None,
            span: dummy_span(),
        };
        cg.generate_statement(&stmt);
        assert!(diags.has_errors());
    }

    #[test]
    fn test_switch_int_to_float_case_promotes_ok() {
        // switch(1.5) { case 1: } —— case int 提升到 float 条件，合法不报错
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = CodeGenerator::new(&st, &mut diags, &mut delegates, empty_injector_fields(), Vec::new());
        let stmt = Statement::Switch {
            expression: Expression::Literal(Literal::Float(1.5), dummy_span()),
            cases: vec![CaseBlock {
                values: vec![Expression::Literal(Literal::Int(1), dummy_span())],
                body: vec![],
                span: dummy_span(),
            }],
            default_body: None,
            span: dummy_span(),
        };
        cg.generate_statement(&stmt);
        assert!(!diags.has_errors());
    }

    // ==================== 方法参数数量校验测试 ====================

    #[test]
    fn test_static_method_arg_count_mismatch_reports_error() {
        // Widget.make(int) 静态方法存在，但用 2 个参数调用 → 报参数数量错误
        use crate::compile_context::symbol::TypeInfo;
        let mut st = SymbolTable::new();
        let global = st.global_scope;
        let cid = st.declare_class("Widget", global, None, vec![], false, dummy_span());
        // 声明静态方法 make(int) → int
        let pid = st.declare_parameter("n", TypeInfo::Int, 0, dummy_span());
        st.declare_method(
            "make",
            Some(cid),
            None,
            TypeInfo::Int,
            vec![pid],
            true,  // is_static
            false, // is_native
            dummy_span(),
        );

        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = CodeGenerator::new(&st, &mut diags, &mut delegates, empty_injector_fields(), Vec::new());
        cg.set_class_context("Widget");

        // Widget.make(1, 2) —— 期望 1 个参数，实际 2 个
        let call = Expression::MethodCall {
            receiver: Box::new(Expression::Identifier("Widget".into(), dummy_span())),
            method: "make".into(),
            arguments: vec![
                Expression::Literal(Literal::Int(1), dummy_span()),
                Expression::Literal(Literal::Int(2), dummy_span()),
            ],
            span: dummy_span(),
        };
        let _ = cg.generate_expression(&call);
        assert!(diags.has_errors());
    }

    #[test]
    fn test_new_local_registers_var_class() {
        // `Widget w = new Widget();` 应把变量 w 的类名登记到 var_class，
        // 使后续 `w.method()` 能解析到正确的实例方法（而非回退 InvokeInstance(0)）
        let mut st = SymbolTable::new();
        let global = st.global_scope;
        st.declare_class("Widget", global, None, vec![], false, dummy_span());

        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let stmt = Statement::VariableDeclaration {
            var_type: TypeRef::simple("Widget", dummy_span()),
            name: "w".into(),
            initializer: Some(Expression::New {
                class_type: TypeRef::simple("Widget", dummy_span()),
                arguments: vec![],
                injector: None,
                span: dummy_span(),
            }),
            span: dummy_span(),
        };
        cg.generate_statement(&stmt);

        assert_eq!(cg.var_class.get("w").map(|s| s.as_str()), Some("Widget"));
    }

    // ==================== break/continue 离块回填测试（Phase D 真正落地） ====================

    /// 统计已完成的离块任务数量与未回填数量。
    fn leave_stats(cg: &CodeGenerator) -> (usize, usize) {
        let done = cg.pending_leaves.iter().filter(|l| l.done).count();
        let pending = cg.pending_leaves.iter().filter(|l| !l.done).count();
        (done, pending)
    }

    #[test]
    fn test_break_while_backpatched_to_end() {
        // while (flag) { break; } —— break 应回填为跳到循环块尾
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let stmt = Statement::While {
            condition: Expression::Identifier("flag".into(), dummy_span()),
            body: Box::new(Statement::Block {
                statements: vec![Statement::Break {
                    targets: vec![BreakTarget::ByLayer(1)],
                    span: dummy_span(),
                }],
                span: dummy_span(),
            }),
            span: dummy_span(),
        };
        cg.generate_statement(&stmt);

        // break 任务已回填完成，无残留
        let (done, pending) = leave_stats(&cg);
        assert_eq!(done, 1);
        assert_eq!(pending, 0);
        // 占位 Jump 已被回填为指向循环末尾（= codes.len()）
        let end = cg.codes.len();
        let leave = &cg.pending_leaves[0];
        assert!(matches!(
            cg.codes[leave.code_index].code.operator,
            IntermediateOperator::Jump(t) if t == end
        ));
    }

    #[test]
    fn test_continue_while_backpatched_to_loop_start() {
        // while (flag) { continue; } —— continue 应回填为跳回条件复检处（loop_start=0）
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let stmt = Statement::While {
            condition: Expression::Identifier("flag".into(), dummy_span()),
            body: Box::new(Statement::Block {
                statements: vec![Statement::Continue {
                    targets: vec![BreakTarget::ByLayer(1)],
                    span: dummy_span(),
                }],
                span: dummy_span(),
            }),
            span: dummy_span(),
        };
        cg.generate_statement(&stmt);

        let (done, pending) = leave_stats(&cg);
        assert_eq!(done, 1);
        assert_eq!(pending, 0);
        // continue 回填指向循环起始（条件复检处）——该 while 是方法体首语句，loop_start=0
        let leave = &cg.pending_leaves[0];
        assert!(matches!(
            cg.codes[leave.code_index].code.operator,
            IntermediateOperator::Jump(0)
        ));
    }

    #[test]
    fn test_break_two_layers_crosses_inner_loop() {
        // while(a) { while(b) { break 2; } } —— break 2 应穿越内层 while，回填到外层块尾
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let inner = Statement::While {
            condition: Expression::Identifier("b".into(), dummy_span()),
            body: Box::new(Statement::Block {
                statements: vec![Statement::Break {
                    targets: vec![BreakTarget::ByLayer(2)],
                    span: dummy_span(),
                }],
                span: dummy_span(),
            }),
            span: dummy_span(),
        };
        let outer = Statement::While {
            condition: Expression::Identifier("a".into(), dummy_span()),
            body: Box::new(Statement::Block {
                statements: vec![inner],
                span: dummy_span(),
            }),
            span: dummy_span(),
        };
        cg.generate_statement(&outer);

        // break 2 最终被外层 while 回填完成
        let (done, pending) = leave_stats(&cg);
        assert_eq!(done, 1);
        assert_eq!(pending, 0);
        // 回填目标应为外层循环末尾（= 全部 codes 末尾）
        let end = cg.codes.len();
        let leave = &cg.pending_leaves[0];
        assert!(matches!(
            cg.codes[leave.code_index].code.operator,
            IntermediateOperator::Jump(t) if t == end
        ));
    }

    #[test]
    fn test_break_by_keyword_while_from_inside_if() {
        // while(flag) { if(cond) { break while; } } ——
        // 关键字 break while 应穿越内层 if（不匹配），由 while 块回填
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let if_stmt = Statement::If {
            condition: Expression::Identifier("cond".into(), dummy_span()),
            then_branch: Box::new(Statement::Block {
                statements: vec![Statement::Break {
                    targets: vec![BreakTarget::ByKeyword("while".into())],
                    span: dummy_span(),
                }],
                span: dummy_span(),
            }),
            else_branch: None,
            span: dummy_span(),
        };
        let stmt = Statement::While {
            condition: Expression::Identifier("flag".into(), dummy_span()),
            body: Box::new(Statement::Block {
                statements: vec![if_stmt],
                span: dummy_span(),
            }),
            span: dummy_span(),
        };
        cg.generate_statement(&stmt);

        let (done, pending) = leave_stats(&cg);
        assert_eq!(done, 1);
        assert_eq!(pending, 0);
    }

    #[test]
    fn test_plain_break_captured_by_inner_if() {
        // while(flag) { if(cond) { break; } } ——
        // if/else 计入层数（对齐 C#），plain break（ByLayer(1)）应被内层 if 捕获，
        // 回填到 if 块尾（而非跳出 while）
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let if_stmt = Statement::If {
            condition: Expression::Identifier("cond".into(), dummy_span()),
            then_branch: Box::new(Statement::Block {
                statements: vec![Statement::Break {
                    targets: vec![BreakTarget::ByLayer(1)],
                    span: dummy_span(),
                }],
                span: dummy_span(),
            }),
            else_branch: None,
            span: dummy_span(),
        };
        let stmt = Statement::While {
            condition: Expression::Identifier("flag".into(), dummy_span()),
            body: Box::new(Statement::Block {
                statements: vec![if_stmt],
                span: dummy_span(),
            }),
            span: dummy_span(),
        };
        cg.generate_statement(&stmt);

        // 被内层 if 捕获并回填完成（if/else 计入层数），不残留未回填任务
        let (done, pending) = leave_stats(&cg);
        assert_eq!(done, 1);
        assert_eq!(pending, 0);
    }

    #[test]
    fn test_break_switch_backpatched() {
        // switch(x) { case 1: break; } —— break 应回填到 switch 块尾
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let stmt = Statement::Switch {
            expression: Expression::Literal(Literal::Int(1), dummy_span()),
            cases: vec![CaseBlock {
                values: vec![Expression::Literal(Literal::Int(1), dummy_span())],
                body: vec![Statement::Break {
                    targets: vec![BreakTarget::ByLayer(1)],
                    span: dummy_span(),
                }],
                span: dummy_span(),
            }],
            default_body: None,
            span: dummy_span(),
        };
        cg.generate_statement(&stmt);

        let (done, pending) = leave_stats(&cg);
        assert_eq!(done, 1);
        assert_eq!(pending, 0);
    }

    #[test]
    fn test_unresolved_break_reports_error() {
        // 方法体顶层 break（无外层块）应残留未回填 → report 后报编译错误
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let stmt = Statement::Break {
            targets: vec![BreakTarget::ByLayer(1)],
            span: dummy_span(),
        };
        cg.generate_statement(&stmt);

        let (done, pending) = leave_stats(&cg);
        assert_eq!(done, 0);
        assert_eq!(pending, 1);

        cg.report_unresolved_leaves();
        assert!(diags.has_errors());
    }

    #[test]
    fn test_break_layer_too_deep_reports_error() {
        // while(flag) { break 3; } —— break 3 超过可用层数，应残留并报错
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let stmt = Statement::While {
            condition: Expression::Identifier("flag".into(), dummy_span()),
            body: Box::new(Statement::Block {
                statements: vec![Statement::Break {
                    targets: vec![BreakTarget::ByLayer(3)],
                    span: dummy_span(),
                }],
                span: dummy_span(),
            }),
            span: dummy_span(),
        };
        cg.generate_statement(&stmt);

        // while 只消解一层，还剩 2 层未满足
        let (done, pending) = leave_stats(&cg);
        assert_eq!(done, 0);
        assert_eq!(pending, 1);

        let before = cg.diagnostics.error_count();
        cg.report_unresolved_leaves();
        // report 应针对未回填的 break 追加至少一条错误
        assert!(cg.diagnostics.error_count() > before);
    }

    // ==================== G1 注入器字段访问测试 ====================

    #[test]
    fn test_set_injector_context() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let fields = vec![
            ("x".to_string(), ValueType::Float),
            ("y".to_string(), ValueType::Float),
            ("count".to_string(), ValueType::Int),
            ("label".to_string(), ValueType::String),
        ];
        cg.set_injector_context(&fields);

        assert_eq!(cg.injector_field_info.len(), 4);
        assert_eq!(cg.injector_field_info.get("x"), Some(&(0, ValueType::Float)));
        assert_eq!(cg.injector_field_info.get("y"), Some(&(1, ValueType::Float)));
        assert_eq!(cg.injector_field_info.get("count"), Some(&(2, ValueType::Int)));
        assert_eq!(cg.injector_field_info.get("label"), Some(&(3, ValueType::String)));
    }

    #[test]
    fn test_generate_injector_field_ref_int() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        // 设置注入器字段上下文
        cg.set_injector_context(&[("count".to_string(), ValueType::Int)]);

        let expr = Expression::InjectorFieldRef("count".into(), dummy_span());
        let _result = cg.generate_expression(&expr);

        // 应生成两条指令：LoadInjector + LoadIntInjectorField(0)
        assert_eq!(cg.codes.len(), 2);
        assert!(matches!(cg.codes[0].code.operator, IntermediateOperator::LoadInjector));
        assert!(matches!(cg.codes[1].code.operator, IntermediateOperator::LoadIntInjectorField(0)));
    }

    #[test]
    fn test_generate_injector_field_ref_float() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        cg.set_injector_context(&[
            ("x".to_string(), ValueType::Float),
            ("y".to_string(), ValueType::Float),
            ("speed".to_string(), ValueType::Float),
        ]);

        let expr = Expression::InjectorFieldRef("speed".into(), dummy_span());
        let _result = cg.generate_expression(&expr);

        assert_eq!(cg.codes.len(), 2);
        assert!(matches!(cg.codes[0].code.operator, IntermediateOperator::LoadInjector));
        assert!(matches!(cg.codes[1].code.operator, IntermediateOperator::LoadFloatInjectorField(2)));
    }

    #[test]
    fn test_generate_injector_field_ref_undefined() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        // 未设置 injector_field_info
        let expr = Expression::InjectorFieldRef("unknown".into(), dummy_span());
        let _result = cg.generate_expression(&expr);

        // 应报诊断错误
        assert!(diags.has_errors());
    }

    #[test]
    fn test_generate_injector_field_assignment() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        cg.set_injector_context(&[("count".to_string(), ValueType::Int)]);

        // this.^count = 5
        let expr = Expression::Assignment {
            target: AssignmentTarget::InjectorField {
                object: Box::new(Expression::This(dummy_span())),
                field: "count".into(),
                span: dummy_span(),
            },
            operator: AssignmentOp::Assign,
            value: Box::new(Expression::Literal(Literal::Int(5), dummy_span())),
            span: dummy_span(),
        };

        let _result = cg.generate_expression(&expr);
        // 应生成：LoadInjector + SetIntInjectorField(0)
        assert_eq!(cg.codes.len(), 2);
        assert!(matches!(cg.codes[0].code.operator, IntermediateOperator::LoadInjector));
        assert!(matches!(cg.codes[1].code.operator, IntermediateOperator::SetIntInjectorField(0)));
    }

    #[test]
    fn test_generate_member_access_injector_field() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        cg.set_injector_context(&[("name".to_string(), ValueType::String)]);

        // this.^name → 读取注入器字段
        let obj = Expression::This(dummy_span());
        let result = cg.generate_member_access(&obj, "^name");

        // 应生成：LoadInjector + LoadStringInjectorField(0)，返回结果地址
        assert!(matches!(result, Operand::Address(_)));
        assert_eq!(cg.codes.len(), 2);
        assert!(matches!(cg.codes[0].code.operator, IntermediateOperator::LoadInjector));
        assert!(matches!(cg.codes[1].code.operator, IntermediateOperator::LoadStringInjectorField(0)));
    }

    #[test]
    fn test_injector_load_set_field_op() {
        // 验证辅助方法按值类型生成正确操作码
        let load_int = CodeGenerator::load_injector_field_op(ValueType::Int, 3);
        assert!(matches!(load_int, IntermediateOperator::LoadIntInjectorField(3)));

        let load_float = CodeGenerator::load_injector_field_op(ValueType::Float, 1);
        assert!(matches!(load_float, IntermediateOperator::LoadFloatInjectorField(1)));

        let load_bool = CodeGenerator::load_injector_field_op(ValueType::Bool, 0);
        assert!(matches!(load_bool, IntermediateOperator::LoadBoolInjectorField(0)));

        let set_str = CodeGenerator::set_injector_field_op(ValueType::String, 2);
        assert!(matches!(set_str, IntermediateOperator::SetStringInjectorField(2)));

        let set_obj = CodeGenerator::set_injector_field_op(ValueType::Object, 0);
        assert!(matches!(set_obj, IntermediateOperator::SetObjectInjectorField(0)));
    }

    // ==================== G2 编译时常量求值与注入器字面量测试 ====================

    #[test]
    fn test_try_eval_const_nested_injector() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let cg = make_codegen(&st, &mut diags, &mut delegates);

        // Vector2:{x:1.0, y:2.0} → 嵌套注入器常量
        let expr = Expression::InjectorObject {
            class_name: "Vector2".into(),
            fields: vec![
                ("x".into(), Expression::Literal(Literal::Float(1.0), dummy_span())),
                ("y".into(), Expression::Literal(Literal::Float(2.0), dummy_span())),
            ],
            span: dummy_span(),
        };
        let result = cg.try_eval_const(&expr);
        assert!(result.is_some());
        if let Some(InjectorConstField::InjectObject(name, fields)) = result {
            assert_eq!(name, "Vector2");
            assert_eq!(fields.len(), 2);
        } else {
            panic!("应为 InjectObject");
        }
    }

    #[test]
    fn test_try_eval_const_array() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let cg = make_codegen(&st, &mut diags, &mut delegates);

        // [1, 2, 3] → 数组常量
        let expr = Expression::InjectorArray {
            elements: vec![
                Expression::Literal(Literal::Int(1), dummy_span()),
                Expression::Literal(Literal::Int(2), dummy_span()),
                Expression::Literal(Literal::Int(3), dummy_span()),
            ],
            span: dummy_span(),
        };
        let result = cg.try_eval_const(&expr);
        assert!(result.is_some());
        if let Some(InjectorConstField::Array(elements)) = result {
            assert_eq!(elements.len(), 3);
        } else {
            panic!("应为 Array");
        }
    }

    #[test]
    fn test_try_eval_const_deeply_nested() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let cg = make_codegen(&st, &mut diags, &mut delegates);

        // Point:{pos: Vector2:{x:1.0, y:2.0}, label: "A"}
        let inner = Expression::InjectorObject {
            class_name: "Vector2".into(),
            fields: vec![
                ("x".into(), Expression::Literal(Literal::Float(1.0), dummy_span())),
                ("y".into(), Expression::Literal(Literal::Float(2.0), dummy_span())),
            ],
            span: dummy_span(),
        };
        let expr = Expression::InjectorObject {
            class_name: "Point".into(),
            fields: vec![
                ("pos".into(), inner),
                ("label".into(), Expression::Literal(Literal::String("A".into()), dummy_span())),
            ],
            span: dummy_span(),
        };
        let result = cg.try_eval_const(&expr);
        assert!(result.is_some());
    }

    #[test]
    fn test_try_eval_const_nested_inject_object_preserves_class_name() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let cg = make_codegen(&st, &mut diags, &mut delegates);

        // Point:{pos: Vector2:{x:1.0, y:2.0}, label: "A"}
        // 嵌套 InjectObject 首槽位必须保留类名（供下游按类型恢复注入器），
        // 字段名由父级常量字段的位置对齐恢复。
        let inner = Expression::InjectorObject {
            class_name: "Vector2".into(),
            fields: vec![
                ("x".into(), Expression::Literal(Literal::Float(1.0), dummy_span())),
                ("y".into(), Expression::Literal(Literal::Float(2.0), dummy_span())),
            ],
            span: dummy_span(),
        };
        let expr = Expression::InjectorObject {
            class_name: "Point".into(),
            fields: vec![
                ("pos".into(), inner),
                ("label".into(), Expression::Literal(Literal::String("A".into()), dummy_span())),
            ],
            span: dummy_span(),
        };
        let result = cg.try_eval_const(&expr).expect("应成功常量化");
        let InjectorConstField::InjectObject(class_name, fields) = result else {
            panic!("应为 InjectObject");
        };
        assert_eq!(class_name, "Point");
        assert_eq!(fields.len(), 2);

        // 嵌套注入器对象：首槽位保留类名 Vector2，而不是被覆写为字段名 pos
        match &fields[0] {
            InjectorConstField::InjectObject(nested_class, nested_fields) => {
                assert_eq!(nested_class, "Vector2", "嵌套注入器必须保留类名");
                assert_eq!(nested_fields.len(), 2);
                // 内层标量字段仍带字段名
                assert!(matches!(&nested_fields[0], InjectorConstField::Float(n, _) if n == "x"));
                assert!(matches!(&nested_fields[1], InjectorConstField::Float(n, _) if n == "y"));
            }
            other => panic!("fields[0] 应为 InjectObject，实际为 {:?}", other),
        }
        // 标量字段名保持不变
        assert!(matches!(&fields[1], InjectorConstField::String(n, _) if n == "label"));
    }

    #[test]
    fn test_generate_injector_object_with_nested() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        // Point:{pos: Vector2:{x:1.0, y:2.0}}
        let inner = Expression::InjectorObject {
            class_name: "Vector2".into(),
            fields: vec![
                ("x".into(), Expression::Literal(Literal::Float(1.0), dummy_span())),
                ("y".into(), Expression::Literal(Literal::Float(2.0), dummy_span())),
            ],
            span: dummy_span(),
        };
        let expr = Expression::InjectorObject {
            class_name: "Point".into(),
            fields: vec![("pos".into(), inner)],
            span: dummy_span(),
        };

        let _result = cg.generate_expression(&expr);
        // 应生成一条 LoadInjectorConstant 指令
        assert_eq!(cg.codes.len(), 1);
        assert!(matches!(cg.codes[0].code.operator, IntermediateOperator::LoadInjectorConstant(_)));
        // 常量池应有一个条目
        assert_eq!(cg.injector_constants.len(), 1);
        assert_eq!(cg.injector_constants[0].class_name, "Point");
    }

    #[test]
    fn test_generate_injector_array_codes() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        // [10, 20, 30]
        let expr = Expression::InjectorArray {
            elements: vec![
                Expression::Literal(Literal::Int(10), dummy_span()),
                Expression::Literal(Literal::Int(20), dummy_span()),
                Expression::Literal(Literal::Int(30), dummy_span()),
            ],
            span: dummy_span(),
        };

        let _result = cg.generate_expression(&expr);
        // 应生成一条 LoadInjectorConstant 指令
        assert_eq!(cg.codes.len(), 1);
        assert!(matches!(cg.codes[0].code.operator, IntermediateOperator::LoadInjectorConstant(_)));
        // 常量池应有一个数组条目
        assert_eq!(cg.injector_constants.len(), 1);
        assert_eq!(cg.injector_constants[0].class_name, "Array");
        assert_eq!(cg.injector_constants[0].fields.len(), 3);
    }

    /// `new Element^[N]{ ... }` 且内联元素为常量字面量时，应折叠为
    /// `class_name="Array"` 的常量并发射 `LoadInjectorConstant`（H3A 修复）。
    #[test]
    fn test_new_array_inline_literal_elements_build_constant_array() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        // `new int^[3]{ 10, 20, 30 }` → arguments = [size, elem1, elem2, elem3]
        let expr = Expression::StaticMethodCall {
            class_name: "array".into(),
            method: "new_array".into(),
            arguments: vec![
                Expression::Literal(Literal::Int(3), dummy_span()),
                Expression::Literal(Literal::Int(10), dummy_span()),
                Expression::Literal(Literal::Int(20), dummy_span()),
                Expression::Literal(Literal::Int(30), dummy_span()),
            ],
            span: dummy_span(),
        };

        let result = cg.generate_expression(&expr);
        // 应生成一条 LoadInjectorConstant 指令
        assert_eq!(cg.codes.len(), 1);
        assert!(matches!(cg.codes[0].code.operator, IntermediateOperator::LoadInjectorConstant(0)));
        // 常量池应有一个 Array 条目，含全部 3 个元素
        assert_eq!(cg.injector_constants.len(), 1);
        assert_eq!(cg.injector_constants[0].class_name, "Array");
        assert_eq!(cg.injector_constants[0].fields.len(), 3);
        assert!(matches!(result, Operand::Address(_)));
        assert!(!diags.has_errors(), "H3A 常量数组路径不应报错");
    }

    /// `new Element^[N]{ ... }` 且内联元素为注入器对象时，应保留每个元素的
    /// 注入器字段数据（嵌套 InjectObject），使框架能按元素类型逐字段恢复。
    #[test]
    fn test_new_array_inline_injector_objects_build_constant_array() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        // 两个 Element 注入器对象元素
        let make_elem = |label: &str| Expression::InjectorObject {
            class_name: "Element".into(),
            fields: vec![("label".into(), Expression::Literal(Literal::String(label.into()), dummy_span()))],
            span: dummy_span(),
        };
        let expr = Expression::StaticMethodCall {
            class_name: "array".into(),
            method: "new_array".into(),
            arguments: vec![
                Expression::Literal(Literal::Int(2), dummy_span()),
                make_elem("A"),
                make_elem("B"),
            ],
            span: dummy_span(),
        };

        let _result = cg.generate_expression(&expr);
        assert_eq!(cg.codes.len(), 1);
        assert!(matches!(cg.codes[0].code.operator, IntermediateOperator::LoadInjectorConstant(0)));
        assert_eq!(cg.injector_constants.len(), 1);
        assert_eq!(cg.injector_constants[0].class_name, "Array");
        let fields = &cg.injector_constants[0].fields;
        assert_eq!(fields.len(), 2);
        // 每个元素都是 InjectObject，类名保留为 Element，字段数据完整
        for (i, f) in fields.iter().enumerate() {
            let expected = if i == 0 { "A" } else { "B" };
            match f {
                InjectorConstField::InjectObject(class_name, nested) => {
                    assert_eq!(class_name, "Element");
                    assert!(matches!(&nested[0], InjectorConstField::String(n, v) if n == "label" && v == expected));
                }
                other => panic!("元素 {} 应为 InjectObject，实际为 {:?}", i, other),
            }
        }
        assert!(!diags.has_errors(), "H3A 常量数组路径不应报错");
    }

    /// `new Element^[N]{ ... }` 且内联元素含运行时会变表达式时，应退化为
    /// 逐元素写入（InvokeArrayConstructor + SetArrayElement），不静默丢弃。
    #[test]
    fn test_new_array_inline_runtime_elements_are_not_dropped() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        // 元素含运行时会变表达式（二元运算非编译期常量）→ 走逐元素写回路径
        let runtime_elem = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Int(10), dummy_span())),
            operator: BinaryOp::Add,
            right: Box::new(Expression::Literal(Literal::Int(20), dummy_span())),
            span: dummy_span(),
        };
        let expr = Expression::StaticMethodCall {
            class_name: "array".into(),
            method: "new_array".into(),
            arguments: vec![
                Expression::Literal(Literal::Int(2), dummy_span()),
                runtime_elem,
                Expression::Literal(Literal::Int(30), dummy_span()),
            ],
            span: dummy_span(),
        };

        let _result = cg.generate_expression(&expr);
        let emitted = cg.codes.iter().map(|c| c.code.operator.clone()).collect::<Vec<_>>();
        // 首条为 InvokeArrayConstructor，随后每个元素一条 InvokeInstance(1)（array set）
        assert!(matches!(emitted[0], IntermediateOperator::InvokeArrayConstructor));
        let set_count = emitted.iter().filter(|op| matches!(op, IntermediateOperator::InvokeInstance(1))).count();
        assert_eq!(set_count, 2, "两个内联元素都应被写入，不得静默丢弃");
        // 常量数组路径不应被使用（无 LoadInjectorConstant）
        assert!(!emitted.iter().any(|op| matches!(op, IntermediateOperator::LoadInjectorConstant(_))));
        assert!(cg.injector_constants.is_empty());
        assert!(!diags.has_errors(), "H3A 逐元素写回路径不应报错");
    }

    #[test]
    fn test_try_eval_const_non_const_returns_none() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let cg = make_codegen(&st, &mut diags, &mut delegates);

        // 变量引用不是编译时常量
        let expr = Expression::Identifier("x".into(), dummy_span());
        assert!(cg.try_eval_const(&expr).is_none());

        // 二元表达式不是编译时常量
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Int(1), dummy_span())),
            operator: BinaryOp::Add,
            right: Box::new(Expression::Literal(Literal::Int(2), dummy_span())),
            span: dummy_span(),
        };
        assert!(cg.try_eval_const(&expr).is_none());
    }

    // ==================== Phase O: 非 this 字段读写测试 ====================

    #[test]
    fn test_generate_this_field_read() {
        // 验证 this.field 读生成 LoadThis + LoadField（对齐 C# FieldReferenceExpression）
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        // 设置字段信息：类有一个 int 字段"count"位于 offset 0
        cg.field_info.insert("count".into(), (0, ValueType::Int));

        let expr = Expression::MemberAccess {
            object: Box::new(Expression::This(dummy_span())),
            member: "count".into(),
            span: dummy_span(),
        };

        let _result = cg.generate_expression(&expr);
        // 应生成 2 条指令：LoadThis + LoadIntField
        assert_eq!(cg.codes.len(), 2);
        assert!(matches!(cg.codes[0].code.operator, IntermediateOperator::LoadThis));
        assert!(matches!(cg.codes[1].code.operator, IntermediateOperator::LoadIntField(0)));
        // LoadField 的 left 应为 this 的临时地址
        match &cg.codes[1].code.left {
            Operand::Address(a) => assert_eq!(a.value_type, ValueType::Object),
            _ => panic!("LoadField left 操作数应为地址"),
        }
    }

    #[test]
    fn test_generate_non_this_field_read() {
        // 验证 obj.field 读取（obj 是非 this 变量）
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        cg.field_info.insert("value".into(), (1, ValueType::Float));

        // obj.value（obj 是局部变量，非 this）
        let expr = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("obj".into(), dummy_span())),
            member: "value".into(),
            span: dummy_span(),
        };

        // 先声明 obj 为局部变量
        cg.declare_local("obj", ValueType::Object);
        let _result = cg.generate_expression(&expr);
        // 应生成 1 条指令：LoadFloatField
        assert_eq!(cg.codes.len(), 1);
        assert!(matches!(cg.codes[0].code.operator, IntermediateOperator::LoadFloatField(1)));
        // left 应为 obj 的地址
        match &cg.codes[0].code.left {
            Operand::Address(a) => assert_eq!(a.value_type, ValueType::Object),
            _ => panic!("LoadField left 操作数应为地址"),
        }
    }

    #[test]
    fn test_generate_this_field_write() {
        // 验证 this.field = val 生成 LoadThis + SetField（左=对象, 右=值）
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        cg.field_info.insert("count".into(), (0, ValueType::Int));

        let expr = Expression::Assignment {
            target: AssignmentTarget::Field {
                object: Box::new(Expression::This(dummy_span())),
                field: "count".into(),
                span: dummy_span(),
            },
            operator: AssignmentOp::Assign,
            value: Box::new(Expression::Literal(Literal::Int(99), dummy_span())),
            span: dummy_span(),
        };

        let _result = cg.generate_expression(&expr);
        // 应生成 2 条指令：LoadThis + SetIntField
        assert_eq!(cg.codes.len(), 2);
        assert!(matches!(cg.codes[0].code.operator, IntermediateOperator::LoadThis));
        assert!(matches!(cg.codes[1].code.operator, IntermediateOperator::SetIntField(0)));
        // SetField 的 right 应为值 99，left 应为对象地址
        assert!(cg.codes[1].code.right.is_some());
        match &cg.codes[1].code.left {
            Operand::Address(a) => assert_eq!(a.value_type, ValueType::Object),
            _ => panic!("SetField left 操作数应为地址"),
        }
    }

    #[test]
    fn test_generate_non_this_field_write() {
        // 验证 obj.field = val 生成接收器求值 + SetField（无 LoadThis）
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        cg.field_info.insert("value".into(), (2, ValueType::Float));

        cg.declare_local("obj", ValueType::Object);
        let expr = Expression::Assignment {
            target: AssignmentTarget::Field {
                object: Box::new(Expression::Identifier("obj".into(), dummy_span())),
                field: "value".into(),
                span: dummy_span(),
            },
            operator: AssignmentOp::Assign,
            value: Box::new(Expression::Literal(Literal::Float(3.14), dummy_span())),
            span: dummy_span(),
        };

        let _result = cg.generate_expression(&expr);
        // 应生成 1 条 SetFloatField（左侧 obj 是已声明的变量，无需额外指令求值）
        assert!(cg.codes.iter().any(|c| matches!(c.code.operator, IntermediateOperator::SetFloatField(2))));
        // 找到 SetFloatField 验证布局
        let set_code = cg.codes.iter().find(|c| matches!(c.code.operator, IntermediateOperator::SetFloatField(2))).unwrap();
        assert!(set_code.code.right.is_some(), "SetField 应有 right 操作数（值）");
    }

    // ============== B-4 FieldInjecting 上下文测试 ==============

    /// B-4: FieldInjecting 上下文中写注入器字段应报编译错误
    #[test]
    fn test_b4_field_injecting_write_rejected() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = CodeGenerator::new(&st, &mut diags, &mut delegates, empty_injector_fields(), Vec::new());
        cg.set_injector_context(&[("speed".to_string(), ValueType::Float)]);

        // 设置为 FieldInjecting 上下文
        cg.current_block_context = BlockContext::FieldInjecting;

        let expr = Expression::Assignment {
            target: AssignmentTarget::InjectorField {
                object: Box::new(Expression::This(dummy_span())),
                field: "speed".into(),
                span: dummy_span(),
            },
            operator: AssignmentOp::Assign,
            value: Box::new(Expression::Literal(Literal::Float(10.0), dummy_span())),
            span: dummy_span(),
        };
        let _result = cg.generate_expression(&expr);
        // 应产生诊断错误（FieldInjecting 上下文中注入器字段不可写）
        assert!(diags.has_errors());
    }

    // ============== B-5 InvokeInjectorConstructor 测试 ==============

    /// B-5: 注入器构造方法 new 应发射 InvokeInjectorConstructor
    #[test]
    fn test_b5_injector_constructor_emits_invoke_injector() {
        let mut st = SymbolTable::new();
        let global = st.global_scope;
        let spam = Span::new(0, 1, 1, 1, 0);

        let class_id = st.declare_class("Foo", global, None, vec![], false, spam);

        let ctor_params = vec![];
        // 声明注入器构造方法（is_injector=true, injector_local_id=0）
        let _ctor_id = st.declare_constructor(class_id, ctor_params, false, true, Some(0), spam);

        // 模拟 freeze 后的状态（在创建 CodeGenerator 前设置，避免 borrow 冲突）
        {
            let ci = st.classes.get_mut(class_id.0);
            ci.constructor_start_id = 0;
            ci.constructor_count_total = 1;
            ci.method_start_id = 0;
            ci.method_count_total = 0;
            ci.inheritance_frozen = true;
            ci.declaration_frozen = true;
        }

        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = CodeGenerator::new(&st, &mut diags, &mut delegates, empty_injector_fields(), Vec::new());
        cg.current_class_name = Some("Foo".into());

        // 使用 new 表达式
        let new_expr = Expression::New {
            class_type: TypeRef::Simple { name: "Foo".into(), span: spam },
            arguments: vec![],
            injector: None,
            span: spam,
        };
        let _result = cg.generate_expression(&new_expr);

        // 应生成 InvokeInjectorConstructor
        let has_injector = cg.codes.iter().any(|c| matches!(c.code.operator, IntermediateOperator::InvokeInjectorConstructor(0)));
        assert!(has_injector, "注入器构造方法应发射 InvokeInjectorConstructor 而非 InvokeConstructor");
    }

    /// B-5: 普通构造方法 new 应发射 InvokeConstructor
    #[test]
    fn test_b5_normal_constructor_emits_invoke_constructor() {
        let mut st = SymbolTable::new();
        let global = st.global_scope;
        let spam = Span::new(0, 1, 1, 1, 0);

        let class_id = st.declare_class("Foo", global, None, vec![], false, spam);

        let ctor_params = vec![];
        let _ctor_id = st.declare_constructor(class_id, ctor_params, false, false, None, spam);

        // 模拟 freeze 后的状态
        {
            let ci = st.classes.get_mut(class_id.0);
            ci.constructor_start_id = 0;
            ci.constructor_count_total = 1;
            ci.method_start_id = 0;
            ci.method_count_total = 0;
            ci.inheritance_frozen = true;
            ci.declaration_frozen = true;
        }

        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = CodeGenerator::new(&st, &mut diags, &mut delegates, empty_injector_fields(), Vec::new());
        cg.current_class_name = Some("Foo".into());

        let new_expr = Expression::New {
            class_type: TypeRef::Simple { name: "Foo".into(), span: spam },
            arguments: vec![],
            injector: None,
            span: spam,
        };
        let _result = cg.generate_expression(&new_expr);

        let has_invoke = cg.codes.iter().any(|c| matches!(c.code.operator, IntermediateOperator::InvokeConstructor(_)));
        assert!(has_invoke, "普通构造方法应发射 InvokeConstructor");
    }

    // ==================== 修复 1: Cast 表达式测试 ====================

    #[test]
    fn test_generate_cast_int_to_float() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let expr = Expression::Cast {
            target_type: TypeRef::simple("float", dummy_span()),
            expression: Box::new(Expression::Literal(Literal::Int(42), dummy_span())),
            span: dummy_span(),
        };
        let result = cg.generate_expression(&expr);

        assert_eq!(CodeGenerator::operand_value_type(&result), ValueType::Float);
        assert!(cg.codes.iter().any(|c| matches!(c.code.operator, IntermediateOperator::IntToFloat)));
    }

    #[test]
    fn test_generate_cast_float_to_int() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let expr = Expression::Cast {
            target_type: TypeRef::simple("int", dummy_span()),
            expression: Box::new(Expression::Literal(Literal::Float(3.14), dummy_span())),
            span: dummy_span(),
        };
        let result = cg.generate_expression(&expr);

        assert_eq!(CodeGenerator::operand_value_type(&result), ValueType::Int);
        assert!(cg.codes.iter().any(|c| matches!(c.code.operator, IntermediateOperator::FloatToInt)));
    }

    #[test]
    fn test_generate_cast_same_type_no_op() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        // int → int 不产生转换指令
        let expr = Expression::Cast {
            target_type: TypeRef::simple("int", dummy_span()),
            expression: Box::new(Expression::Literal(Literal::Int(10), dummy_span())),
            span: dummy_span(),
        };
        let _result = cg.generate_expression(&expr);
        assert!(cg.codes.is_empty(), "同类型转换不应生成指令");
    }

    // ==================== 修复 2: Super 关键字测试 ====================

    #[test]
    fn test_generate_super_emits_load_this() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let expr = Expression::Super(dummy_span());
        let result = cg.generate_expression(&expr);

        assert_eq!(CodeGenerator::operand_value_type(&result), ValueType::Object);
        assert_eq!(cg.codes.len(), 1);
        assert!(matches!(cg.codes[0].code.operator, IntermediateOperator::LoadThis));
    }

    // ==================== 修复 3: 自增自减测试 ====================

    #[test]
    fn test_generate_pre_increment_int() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        // 声明局部变量 x
        cg.declare_local("x", ValueType::Int);
        let expr = Expression::Unary {
            operator: UnaryOp::PreIncrement,
            operand: Box::new(Expression::Identifier("x".into(), dummy_span())),
            span: dummy_span(),
        };
        let result = cg.generate_expression(&expr);

        // 应包含 IntAdd 指令
        assert!(cg.codes.iter().any(|c| matches!(c.code.operator, IntermediateOperator::IntAdd)));
        // 返回值应为 Int 类型
        assert_eq!(CodeGenerator::operand_value_type(&result), ValueType::Int);
    }

    #[test]
    fn test_generate_pre_decrement_float() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        cg.declare_local("f", ValueType::Float);
        let expr = Expression::Unary {
            operator: UnaryOp::PreDecrement,
            operand: Box::new(Expression::Identifier("f".into(), dummy_span())),
            span: dummy_span(),
        };
        let result = cg.generate_expression(&expr);

        assert_eq!(CodeGenerator::operand_value_type(&result), ValueType::Float);
        assert!(cg.codes.iter().any(|c| matches!(c.code.operator, IntermediateOperator::FloatSub)));
    }

    #[test]
    fn test_generate_post_increment_int() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        cg.declare_local("n", ValueType::Int);
        let expr = Expression::Unary {
            operator: UnaryOp::PostIncrement,
            operand: Box::new(Expression::Identifier("n".into(), dummy_span())),
            span: dummy_span(),
        };
        let result = cg.generate_expression(&expr);

        assert_eq!(CodeGenerator::operand_value_type(&result), ValueType::Int);
        // post-increment: 应包含原值保存 + 加法 + 写回
        assert!(cg.codes.iter().any(|c| matches!(c.code.operator, IntermediateOperator::IntAdd)));
    }

    #[test]
    fn test_generate_increment_on_non_numeric_reports_error() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        cg.declare_local("s", ValueType::String);
        let expr = Expression::Unary {
            operator: UnaryOp::PreIncrement,
            operand: Box::new(Expression::Identifier("s".into(), dummy_span())),
            span: dummy_span(),
        };
        let _result = cg.generate_expression(&expr);
        assert!(diags.has_errors(), "对 string 类型自增应报错");
    }

    // ==================== 修复 4a: 跨对象字段访问 / resolve_object_type 测试 ====================

    #[test]
    fn test_resolve_object_type_identifier() {
        let mut st = SymbolTable::new();
        let global = st.global_scope;
        let class_id = st.declare_class("Point", global, None, vec![], false, dummy_span());
        // 声明 int 字段 x
        st.declare_field("x", class_id, TypeInfo::Int, false, dummy_span());
        // 分配偏移
        st.allocate_field_offset(FieldId(0), 0);

        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.register_var_type("p", TypeInfo::Object(class_id));

        let ty = cg.resolve_object_type(&Expression::Identifier("p".into(), dummy_span()));
        assert!(matches!(ty, Some(TypeInfo::Object(_))));
    }

    #[test]
    fn test_resolve_object_type_member_access() {
        let mut st = SymbolTable::new();
        let global = st.global_scope;

        // 创建类 Point { x: int }
        let point_id = st.declare_class("Point", global, None, vec![], false, dummy_span());
        st.declare_field("x", point_id, TypeInfo::Int, false, dummy_span());
        st.allocate_field_offset(FieldId(0), 0);

        // 创建类 Container { pos: Point }
        let container_id = st.declare_class("Container", global, None, vec![], false, dummy_span());
        st.declare_field("pos", container_id, TypeInfo::Object(point_id), false, dummy_span());
        st.allocate_field_offset(FieldId(1), 0);

        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.register_var_type("c", TypeInfo::Object(container_id));

        // c.pos 的类型应为 Point (Object)
        let member = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("c".into(), dummy_span())),
            member: "pos".into(),
            span: dummy_span(),
        };
        let ty = cg.resolve_object_type(&member);
        assert!(matches!(ty, Some(TypeInfo::Object(cid)) if cid == point_id));
    }

    #[test]
    fn test_lookup_field_for_object_chained() {
        // 测试 t.nativeObjectField.innerField → 跨成员链字段查找
        let mut st = SymbolTable::new();
        let global = st.global_scope;

        // NativeObject 类：有 innerField (int)
        let native_id = st.declare_class("NativeObject", global, None, vec![], false, dummy_span());
        st.declare_field("innerField", native_id, TypeInfo::Int, false, dummy_span());
        st.allocate_field_offset(FieldId(0), 0);

        // Target 类：有 nativeObjectField (NativeObject)
        let target_id = st.declare_class("Target", global, None, vec![], false, dummy_span());
        st.declare_field("nativeObjectField", target_id, TypeInfo::Object(native_id), false, dummy_span());
        st.allocate_field_offset(FieldId(1), 0);

        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.register_var_type("t", TypeInfo::Object(target_id));
        cg.register_var_class("t", "Target");

        // t.nativeObjectField → 应能查到 innerField
        let member_access = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("t".into(), dummy_span())),
            member: "nativeObjectField".into(),
            span: dummy_span(),
        };
        let result = cg.lookup_field_for_object(&member_access, "innerField");
        assert!(result.is_some(), "跨成员链应能查找到 innerField");
        let (offset, vt) = result.unwrap();
        assert_eq!(offset, 0);
        assert_eq!(vt, ValueType::Int);
    }

    // ==================== 修复 6: 注入器数组非编译时常量测试 ====================

    #[test]
    fn test_injector_array_non_const_returns_zero_address() {
        let st = SymbolTable::new();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        // 数组元素含变量引用（非编译时常量）→ 应报错并返回 Object(0)
        let expr = Expression::InjectorArray {
            elements: vec![
                Expression::Identifier("x".into(), dummy_span()),
            ],
            span: dummy_span(),
        };
        let result = cg.generate_expression(&expr);

        assert!(diags.has_errors());
        match result {
            Operand::Address(a) => {
                assert_eq!(a.value_type, ValueType::Object);
                assert_eq!(a.index, 0);
            }
            _ => panic!("应返回 Address(0) 而非 Nop"),
        }
    }

    // ==================== this 字段成员访问推导（field_types 回退）测试 ====================

    /// 构造 Node/Lane/Note 三层类结构的符号表：
    /// `Node { x: float }`、`Lane { noteReferenceNode: Node }`、`Note { lane: Lane }`
    fn make_nested_field_table() -> SymbolTable {
        let mut st = SymbolTable::new();
        let g = st.global_scope;
        let node_id = st.declare_class("Node", g, None, vec![], false, dummy_span());
        let fx = st.declare_field("x", node_id, TypeInfo::Float, false, dummy_span());
        st.allocate_field_offset(fx, 0);
        let lane_id = st.declare_class("Lane", g, None, vec![], false, dummy_span());
        let fref = st.declare_field(
            "noteReferenceNode",
            lane_id,
            TypeInfo::Object(node_id),
            false,
            dummy_span(),
        );
        st.allocate_field_offset(fref, 0);
        let note_id = st.declare_class("Note", g, None, vec![], false, dummy_span());
        let flane = st.declare_field("lane", note_id, TypeInfo::Object(lane_id), false, dummy_span());
        st.allocate_field_offset(flane, 0);
        st
    }

    /// 两级访问：this 字段 `lane` 的成员 `noteReferenceNode` 应解析为 Lane 的字段
    #[test]
    fn test_this_field_member_access_two_level() {
        let st = make_nested_field_table();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.set_class_context("Note");

        let expr = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("lane".into(), dummy_span())),
            member: "noteReferenceNode".into(),
            span: dummy_span(),
        };
        cg.generate_expression(&expr);

        // 应发射 LoadObjectField(0)（noteReferenceNode 在 Lane 中的偏移）
        let emitted = cg.codes.iter().any(|c| matches!(
            c.code.operator,
            IntermediateOperator::LoadObjectField(0)
        ));
        assert!(!diags.has_errors(), "不应报错: {:?}", diags);
        assert!(emitted);
    }

    /// 三级成员链：`lane.noteReferenceNode.x` 应沿字段类型逐级推导
    #[test]
    fn test_this_field_member_chain_three_level() {
        let st = make_nested_field_table();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.set_class_context("Note");

        let expr = Expression::MemberAccess {
            object: Box::new(Expression::MemberAccess {
                object: Box::new(Expression::Identifier("lane".into(), dummy_span())),
                member: "noteReferenceNode".into(),
                span: dummy_span(),
            }),
            member: "x".into(),
            span: dummy_span(),
        };
        cg.generate_expression(&expr);

        // 最后应发射 LoadFloatField(0)（x 在 Node 中的偏移）
        let emitted = cg.codes.iter().any(|c| matches!(
            c.code.operator,
            IntermediateOperator::LoadFloatField(0)
        ));
        assert!(!diags.has_errors(), "不应报错: {:?}", diags);
        assert!(emitted);
    }

    /// 以 this 字段为接收者的字段赋值：`lane.noteReferenceNode = val` 应解析成功
    #[test]
    fn test_this_field_member_assignment() {
        let st = make_nested_field_table();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.set_class_context("Note");

        let expr = Expression::Assignment {
            target: AssignmentTarget::Field {
                object: Box::new(Expression::Identifier("lane".into(), dummy_span())),
                field: "noteReferenceNode".into(),
                span: dummy_span(),
            },
            operator: AssignmentOp::Assign,
            value: Box::new(Expression::Literal(Literal::Int(0), dummy_span())),
            span: dummy_span(),
        };
        cg.generate_expression(&expr);

        let emitted = cg.codes.iter().any(|c| matches!(
            c.code.operator,
            IntermediateOperator::SetObjectField(0)
        ));
        assert!(!diags.has_errors(), "不应报错: {:?}", diags);
        assert!(emitted);
    }

    /// 继承场景：SubLane 继承 Lane，`subLane.noteReferenceNode` 沿父类链解析
    #[test]
    fn test_this_field_member_access_inherited() {
        let mut st = make_nested_field_table();
        let g = st.global_scope;
        let lane_id = st.lookup_class(g, "Lane").unwrap();
        let sub_lane_id = st.declare_class("SubLane", g, Some(lane_id), vec![], false, dummy_span());
        let note2_id = st.declare_class("Note2", g, None, vec![], false, dummy_span());
        let fsub = st.declare_field(
            "subLane",
            note2_id,
            TypeInfo::Object(sub_lane_id),
            false,
            dummy_span(),
        );
        st.allocate_field_offset(fsub, 0);

        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.set_class_context("Note2");

        let expr = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("subLane".into(), dummy_span())),
            member: "noteReferenceNode".into(),
            span: dummy_span(),
        };
        cg.generate_expression(&expr);

        let emitted = cg.codes.iter().any(|c| matches!(
            c.code.operator,
            IntermediateOperator::LoadObjectField(0)
        ));
        assert!(!diags.has_errors(), "不应报错: {:?}", diags);
        assert!(emitted);
    }

    /// 局部变量优先级：同名局部变量的类型应优先于 this 字段
    #[test]
    fn test_local_var_takes_precedence_over_field() {
        let st = make_nested_field_table();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.set_class_context("Note");
        // 局部变量 lane 声明为 Node 类型（与字段 lane: Lane 同名不同类）
        let node_id = st.lookup_class(st.global_scope, "Node").unwrap();
        cg.register_var_class("lane", "Node");
        cg.var_types.insert("lane".into(), TypeInfo::Object(node_id));

        let expr = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("lane".into(), dummy_span())),
            member: "x".into(),
            span: dummy_span(),
        };
        cg.generate_expression(&expr);

        // x 是 Node 的字段：若错误地用了字段类型 Lane 则会报「未定义的字段 x」
        let emitted = cg.codes.iter().any(|c| matches!(
            c.code.operator,
            IntermediateOperator::LoadFloatField(0)
        ));
        assert!(!diags.has_errors(), "不应报错: {:?}", diags);
        assert!(emitted);
    }

    // ==================== 枚举支持测试 ====================

    /// 构造含枚举 TimeMode { CatchBefore, KeepUntil } 的符号表
    fn make_enum_table() -> (SymbolTable, EnumId) {
        let mut st = SymbolTable::new();
        let g = st.global_scope;
        let eid = st.declare_enum("TimeMode", g, dummy_span());
        st.declare_enum_value("CatchBefore", eid, None, dummy_span());
        st.declare_enum_value("KeepUntil", eid, None, dummy_span());
        (st, eid)
    }

    /// 枚举成员访问：缺省值按声明序号（CatchBefore=0, KeepUntil=1）
    #[test]
    fn test_enum_member_access_ordinal() {
        let (st, _eid) = make_enum_table();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let first = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("TimeMode".into(), dummy_span())),
            member: "CatchBefore".into(),
            span: dummy_span(),
        };
        let r0 = cg.generate_expression(&first);
        let second = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("TimeMode".into(), dummy_span())),
            member: "KeepUntil".into(),
            span: dummy_span(),
        };
        let r1 = cg.generate_expression(&second);

        assert!(matches!(r0, Operand::Immediate(ImmediateValue::Int(0))));
        assert!(matches!(r1, Operand::Immediate(ImmediateValue::Int(1))));
        assert!(!diags.has_errors(), "不应报错: {:?}", diags);
    }

    /// 枚举成员访问：显式值优先于声明序号
    #[test]
    fn test_enum_member_access_explicit_value() {
        let mut st = SymbolTable::new();
        let g = st.global_scope;
        let eid = st.declare_enum("RespondResult", g, dummy_span());
        st.declare_enum_value("Miss", eid, Some(10), dummy_span());
        st.declare_enum_value("Perfect", eid, Some(30), dummy_span());

        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let expr = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("RespondResult".into(), dummy_span())),
            member: "Perfect".into(),
            span: dummy_span(),
        };
        let r = cg.generate_expression(&expr);

        assert!(matches!(r, Operand::Immediate(ImmediateValue::Int(30))));
        assert!(!diags.has_errors(), "不应报错: {:?}", diags);
    }

    /// 访问未定义的枚举值应报错
    #[test]
    fn test_enum_undefined_value_reports_error() {
        let (st, _eid) = make_enum_table();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);

        let expr = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("TimeMode".into(), dummy_span())),
            member: "NoSuchValue".into(),
            span: dummy_span(),
        };
        cg.generate_expression(&expr);

        assert!(diags.has_errors(), "应报未定义的枚举值错误");
    }

    /// 枚举类型字段：赋值枚举值 → SetIntField；相等比较 → IntEqual
    #[test]
    fn test_enum_field_assignment_and_equality() {
        let (mut st, eid) = make_enum_table();
        let g = st.global_scope;
        let foo_id = st.declare_class("Foo", g, None, vec![], false, dummy_span());
        let fmode = st.declare_field("mode", foo_id, TypeInfo::Enum(eid), false, dummy_span());
        st.allocate_field_offset(fmode, 0);

        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.set_class_context("Foo");

        // mode = TimeMode.CatchBefore
        let assign = Expression::Assignment {
            target: AssignmentTarget::Variable("mode".into(), dummy_span()),
            operator: AssignmentOp::Assign,
            value: Box::new(Expression::MemberAccess {
                object: Box::new(Expression::Identifier("TimeMode".into(), dummy_span())),
                member: "CatchBefore".into(),
                span: dummy_span(),
            }),
            span: dummy_span(),
        };
        cg.generate_expression(&assign);

        // mode == TimeMode.KeepUntil
        let eq = Expression::Binary {
            left: Box::new(Expression::Identifier("mode".into(), dummy_span())),
            operator: BinaryOp::Equal,
            right: Box::new(Expression::MemberAccess {
                object: Box::new(Expression::Identifier("TimeMode".into(), dummy_span())),
                member: "KeepUntil".into(),
                span: dummy_span(),
            }),
            span: dummy_span(),
        };
        cg.generate_expression(&eq);

        let has_set = cg
            .codes
            .iter()
            .any(|c| matches!(c.code.operator, IntermediateOperator::SetIntField(0)));
        let has_eq = cg
            .codes
            .iter()
            .any(|c| matches!(c.code.operator, IntermediateOperator::IntEqual));
        assert!(!diags.has_errors(), "不应报错: {:?}", diags);
        assert!(has_set, "应发射 SetIntField(0)");
        assert!(has_eq, "枚举相等比较应发射 IntEqual");
    }

    /// 同名局部变量优先于枚举：`TimeMode.CatchBefore` 中 TimeMode 为变量时不按枚举解析
    #[test]
    fn test_local_var_shadows_enum_name() {
        let (st, _eid) = make_enum_table();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        // 注册名为 TimeMode 的局部变量（参数注册会写入 local_vars 地址表）
        cg.register_parameters(&[("TimeMode".into(), ValueType::Int)]);
        cg.var_types.insert("TimeMode".into(), TypeInfo::Int);

        let expr = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("TimeMode".into(), dummy_span())),
            member: "CatchBefore".into(),
            span: dummy_span(),
        };
        let r = cg.generate_expression(&expr);

        // 不应命中枚举路径（不会返回枚举序数 0 的立即数语义之外的值），
        // 变量无 CatchBefore 字段 → 报「未定义的字段」而非按枚举静默通过
        assert!(diags.has_errors(), "变量遮蔽枚举时不应按枚举解析");
        assert!(!matches!(r, Operand::Immediate(ImmediateValue::Int(0))));
    }

    // ==================== 隐式 this 方法调用测试 ====================

    /// 构造类 Foo：实例方法 `float EvaluateLineY(float x, float now)` + 静态方法 `int Double(int n)`
    fn make_method_table() -> SymbolTable {
        let mut st = SymbolTable::new();
        let g = st.global_scope;
        let foo = st.declare_class("Foo", g, None, vec![], false, dummy_span());
        let p1 = st.declare_parameter("x", TypeInfo::Float, 0, dummy_span());
        let p2 = st.declare_parameter("now", TypeInfo::Float, 1, dummy_span());
        st.declare_method(
            "EvaluateLineY",
            Some(foo),
            None,
            TypeInfo::Float,
            vec![p1, p2],
            false,
            false,
            dummy_span(),
        );
        let p3 = st.declare_parameter("n", TypeInfo::Int, 0, dummy_span());
        st.declare_method(
            "Double",
            Some(foo),
            None,
            TypeInfo::Int,
            vec![p3],
            true,
            false,
            dummy_span(),
        );
        st
    }

    /// 无接收者调用 `EvaluateLineY(1.0, 2.0)` 应解析为 this 的实例方法调用
    #[test]
    fn test_implicit_this_instance_method_call() {
        let st = make_method_table();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.set_class_context("Foo");

        let expr = Expression::StaticMethodCall {
            class_name: String::new(),
            method: "EvaluateLineY".into(),
            arguments: vec![
                Expression::Literal(Literal::Float(1.0), dummy_span()),
                Expression::Literal(Literal::Float(2.0), dummy_span()),
            ],
            span: dummy_span(),
        };
        cg.generate_expression(&expr);

        // Foo 为根类（method_start_id=0），EvaluateLineY 是第 0 个方法
        let emitted = cg.codes.iter().any(|c| matches!(
            c.code.operator,
            IntermediateOperator::InvokeInstance(0)
        ));
        assert!(!diags.has_errors(), "不应报错: {:?}", diags);
        assert!(emitted, "应发射 InvokeInstance(0)");
    }

    /// 无接收者调用 `Double(21)` 应回退为当前类的静态方法调用
    #[test]
    fn test_implicit_this_static_method_call() {
        let st = make_method_table();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.set_class_context("Foo");

        let expr = Expression::StaticMethodCall {
            class_name: String::new(),
            method: "Double".into(),
            arguments: vec![Expression::Literal(Literal::Int(21), dummy_span())],
            span: dummy_span(),
        };
        cg.generate_expression(&expr);

        // Double 在方法表中索引 1（非 native 静态分支按位置编号）
        let emitted = cg.codes.iter().any(|c| matches!(
            c.code.operator,
            IntermediateOperator::InvokeStatic(1)
        ));
        assert!(!diags.has_errors(), "不应报错: {:?}", diags);
        assert!(emitted, "应发射 InvokeStatic(1)");
    }

    /// 不存在的方法仍应报「未定义的变量」
    #[test]
    fn test_implicit_this_unknown_method_still_errors() {
        let st = make_method_table();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.set_class_context("Foo");

        let expr = Expression::StaticMethodCall {
            class_name: String::new(),
            method: "NoSuchMethod".into(),
            arguments: vec![Expression::Literal(Literal::Int(1), dummy_span())],
            span: dummy_span(),
        };
        cg.generate_expression(&expr);

        assert!(diags.has_errors(), "不存在的方法应报错");
    }

    /// 委托变量优先级：同名委托变量存在时不触发 this 方法回退
    #[test]
    fn test_delegate_var_takes_precedence_over_implicit_this() {
        let st = make_method_table();
        let mut diags = Diagnostics::new();
        let mut delegates = Vec::new();
        let mut cg = make_codegen(&st, &mut diags, &mut delegates);
        cg.set_class_context("Foo");
        // 注册名为 EvaluateLineY 的委托变量
        cg.register_parameters(&[("EvaluateLineY".into(), ValueType::Int)]);
        cg.delegate_vars.insert("EvaluateLineY".into(), 0);

        let expr = Expression::StaticMethodCall {
            class_name: String::new(),
            method: "EvaluateLineY".into(),
            arguments: vec![
                Expression::Literal(Literal::Float(1.0), dummy_span()),
                Expression::Literal(Literal::Float(2.0), dummy_span()),
            ],
            span: dummy_span(),
        };
        cg.generate_expression(&expr);

        let emitted = cg.codes.iter().any(|c| matches!(
            c.code.operator,
            IntermediateOperator::InvokeDelegate(0)
        ));
        assert!(!diags.has_errors(), "不应报错: {:?}", diags);
        assert!(emitted, "应走委托调用路径 InvokeDelegate(0)");
    }
}
