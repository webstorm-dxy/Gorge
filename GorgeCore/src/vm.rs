#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;
use crate::ir::*;
use crate::object::{RuntimeObject, GorgeObject};
use crate::types::TypeCount;
use crate::class::{RuntimeClass, GorgeClass};
use crate::injector::{RuntimeInjector, Injector};
use crate::value_pool::FixedFieldValuePool;

/// 值类型的栈
///
/// 每种值类型维护独立的栈容器，同时追踪调用帧边界。
/// `frames` 记录每帧的起始位置，`PushFrame` 分配局部变量空间，
/// `PopFrame` 释放当前帧所有变量。
#[derive(Debug, Clone)]
pub struct VmStack<T: Clone + Default> {
    data: Vec<T>,
    frames: Vec<usize>,
}

impl<T: Clone + Default> VmStack<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            frames: vec![0], // 根帧起始于 0
        }
    }

    /// 推入一个调用帧，分配 `local_count` 个局部变量空间
    pub fn push_frame(&mut self, local_count: usize) {
        let start = self.data.len();
        self.frames.push(start);
        self.data.resize(start + local_count, T::default());
    }

    /// 弹出当前调用帧，释放局部变量空间
    pub fn pop_frame(&mut self) {
        if self.frames.len() <= 1 {
            return; // 保留根帧
        }
        let start = self.frames.pop().unwrap();
        self.data.truncate(start);
    }

    /// 读取地址处的值
    pub fn read(&self, index: usize) -> &T {
        &self.data[index]
    }

    /// 写入地址处的值
    pub fn write(&mut self, index: usize, value: T) {
        if index >= self.data.len() {
            self.data.resize(index + 1, T::default());
        }
        self.data[index] = value;
    }

    /// 栈深度（用于调试）
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 当前帧数量
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}

impl<T: Clone + Default> Default for VmStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 虚拟机执行结果
pub type VmResult<T> = Result<T, String>;

/// 类型分离栈的 Gorge 解释型虚拟机
///
/// 采用类型分离栈设计——int、float、bool、string、object 各使用独立栈。
/// 这种设计避免了类型标记开销，与 C# 参考实现保持一致。
#[derive(Debug)]
pub struct VirtualMachine {
    pub int_stack: VmStack<i64>,
    pub float_stack: VmStack<f64>,
    pub bool_stack: VmStack<bool>,
    pub string_stack: VmStack<String>,
    /// object_stack 存储对象 ID（在运行时对象表中的索引）
    pub object_stack: VmStack<usize>,

    /// 对象 ID 分配器
    next_object_id: usize,

    /// 调用参数池（用于方法调用参数传递和返回值）
    pub param_pool: crate::param_pool::InvokeParameterPool,
    /// 正式对象表：对象ID → RuntimeObject 实例
    pub objects: HashMap<usize, RuntimeObject>,
    /// 注入器对象表：注入器ID → RuntimeInjector 实例
    pub injectors: HashMap<usize, RuntimeInjector>,
    /// 类字段数量注册表：类全名 → 字段类型计数（供 DoConstruct 使用）
    class_field_counts: HashMap<String, TypeCount>,
    /// 类静态字段注册表：类全名 → 静态字段值池
    pub class_static_fields: HashMap<String, FixedFieldValuePool>,
    /// 类委托实现表：类全名 → 委托列表
    class_delegate_impls: HashMap<String, Vec<(CompiledMethod, Vec<ValueType>)>>,
    /// 类注册表：类全名 → RuntimeClass（供方法分派使用）
    pub class_table: HashMap<String, Arc<RuntimeClass>>,

    /// Native 类注册表：类全名 → NativeClass（供 native 方法/构造分派）
    pub native_class_table: HashMap<String, Arc<dyn crate::native::NativeClass>>,
    /// 类 → 父类名映射（供跨 native/compiled 边界的祖先查找，F2）
    pub class_super_name: HashMap<String, String>,
    /// 注入器常量池（G2）：由 runner 反序列化后注册，运行时通过 LoadInjectorConstant 访问
    pub injector_constants: Vec<crate::bytecode::InjectorConstantDef>,

    /// 按类名索引的静态方法表
    class_static_methods: HashMap<String, Vec<(CompiledMethod, Vec<ValueType>)>>,
    /// 当前执行上下文所属的类名
    current_class: String,
    /// 当前调用上下文中的注入器对象 ID（对应 C# InvokeParameterPool.Injector）
    pub current_injector: Option<usize>,

    /// 程序计数器
    pc: usize,
    /// 委托实现表（Lambda 编译产物），含参数类型用于参数传递
    pub delegate_impls: Vec<(CompiledMethod, Vec<ValueType>)>,
    /// 返回值存储（按类型分）
    return_int: Option<i64>,
    return_float: Option<f64>,
    return_bool: Option<bool>,
    return_string: Option<String>,
    return_object: Option<usize>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            int_stack: VmStack::new(),
            float_stack: VmStack::new(),
            bool_stack: VmStack::new(),
            string_stack: VmStack::new(),
            object_stack: VmStack::new(),
            next_object_id: 1, // 0 保留给 null/默认
            param_pool: crate::param_pool::InvokeParameterPool::new(),
            objects: HashMap::new(),
            injectors: HashMap::new(),
            class_field_counts: HashMap::new(),
            class_static_fields: HashMap::new(),
            class_delegate_impls: HashMap::new(),
            class_table: HashMap::new(),
            native_class_table: HashMap::new(),
            class_super_name: HashMap::new(),
            injector_constants: Vec::new(),
            class_static_methods: HashMap::new(),
            current_class: String::new(),
            current_injector: None,
            delegate_impls: Vec::new(),
            pc: 0,
            return_int: None,
            return_float: None,
            return_bool: None,
            return_string: None,
            return_object: None,
        }
    }

    /// 为类注册静态方法表
    pub fn register_class_methods(
        &mut self,
        class_name: &str,
        methods: Vec<(CompiledMethod, Vec<ValueType>)>,
    ) {
        self.class_static_methods.insert(class_name.to_string(), methods);
    }

    /// 注册类的字段类型计数（供 DoConstruct 使用）
    pub fn register_class_field_counts(&mut self, class_name: &str, counts: TypeCount) {
        self.class_field_counts.insert(class_name.to_string(), counts);
    }

    /// 注册运行时类（供方法分派使用）
    pub fn register_runtime_class(&mut self, class_name: &str, cls: Arc<RuntimeClass>) {
        self.class_table.insert(class_name.to_string(), cls);
    }

    /// 注册 native 类（供 native 方法/构造分派使用）
    pub fn register_native_class(
        &mut self,
        class_name: &str,
        cls: Arc<dyn crate::native::NativeClass>,
    ) {
        self.native_class_table.insert(class_name.to_string(), cls);
    }

    /// 注册类的委托实现（供 InvokeDelegate 按类查找）
    pub fn register_class_delegates(&mut self, class_name: &str, delegates: Vec<(CompiledMethod, Vec<ValueType>)>) {
        self.class_delegate_impls.insert(class_name.to_string(), delegates);
    }

    /// 设置当前执行上下文所属的类名
    pub fn set_current_class(&mut self, class_name: &str) {
        self.current_class = class_name.to_string();
    }

    // ==================== Native 分派 ====================

    /// 把调用参数池中的参数按值类型分组复制到 callee 的局部槽位。
    ///
    /// 参数按值类型分组存储（B-2），方法参数占据每种类型最低的连续局部索引。
    /// 对每种类型从 0 起连续复制「已设置」的参数，遇首个未设置即停，`bound` 为上界。
    fn copy_params_to_locals(&mut self, bound: usize) {
        // 逐类型复制（借用 param_pool 的各数组判断 is_set）
        {
            let mut j = 0;
            while j < bound && self.param_pool.int_params.borrow()[j].is_set {
                let v = self.param_pool.get_int_param(j);
                self.int_stack.write(j, v);
                j += 1;
            }
        }
        {
            let mut j = 0;
            while j < bound && self.param_pool.float_params.borrow()[j].is_set {
                let v = self.param_pool.get_float_param(j);
                self.float_stack.write(j, v);
                j += 1;
            }
        }
        {
            let mut j = 0;
            while j < bound && self.param_pool.bool_params.borrow()[j].is_set {
                let v = self.param_pool.get_bool_param(j);
                self.bool_stack.write(j, v);
                j += 1;
            }
        }
        {
            let mut j = 0;
            while j < bound && self.param_pool.string_params.borrow()[j].is_set {
                let v = self.param_pool.get_string_param(j);
                self.string_stack.write(j, v);
                j += 1;
            }
        }
        {
            let mut j = 0;
            while j < bound && self.param_pool.object_params.borrow()[j].is_set {
                let v = self.param_pool.get_object_param(j);
                self.object_stack.write(j, v);
                j += 1;
            }
        }
        // 注意：不重置参数池——编译方法可能通过 LoadParameter 再次读取参数池。
    }


    /// 沿编译类的 super_class 链查找最近的 native 祖先类名（F2）
    ///
    /// 用于编译子类调用继承自 native 父类的方法时定位 native 实现。
    /// 通过 `class_super_name` 映射逐级上溯（含 native 父类，class_table 不含 native）。
    fn find_native_ancestor(&self, class_name: &str) -> Option<String> {
        let mut current = self.class_super_name.get(class_name).cloned();
        let mut guard = 0;
        while let Some(name) = current {
            if self.native_class_table.contains_key(&name) {
                return Some(name);
            }
            current = self.class_super_name.get(&name).cloned();
            guard += 1;
            if guard > 1000 { break; }
        }
        None
    }

    /// 注册类的父类名（供 F2 跨边界祖先查找）
    pub fn register_class_super(&mut self, class_name: &str, super_name: &str) {
        self.class_super_name.insert(class_name.to_string(), super_name.to_string());
    }

    /// 分派 native 静态方法：参数已由 SetXxxParameter 写入参数池，返回值写回参数池。
    fn dispatch_native_static(&mut self, class_name: &str, method_id: usize) {
        if let Some(cls) = self.native_class_table.get(class_name).cloned() {
            let mut ctx = crate::native::NativeContext::new(
                &self.param_pool,
                &mut self.objects,
                &mut self.next_object_id,
            );
            cls.invoke_native_static(&mut ctx, method_id);
        }
    }

    /// 构造 native 上下文并分派实例方法
    fn dispatch_native_method(&mut self, class_name: &str, obj_id: usize, method_id: usize) {
        if let Some(cls) = self.native_class_table.get(class_name).cloned() {
            let mut ctx = crate::native::NativeContext::new(
                &self.param_pool,
                &mut self.objects,
                &mut self.next_object_id,
            );
            cls.invoke_native_method(&mut ctx, obj_id, method_id);
        }
    }

    /// 构造 native 上下文并分派构造方法，返回新对象 ID
    ///
    /// 注入器专用位在调用前应已由 SetInjector 逻辑写入参数池。
    fn dispatch_native_construct(
        &mut self,
        class_name: &str,
        target: Option<usize>,
        ctor_id: usize,
    ) -> usize {
        if let Some(cls) = self.native_class_table.get(class_name).cloned() {
            let mut ctx = crate::native::NativeContext::new(
                &self.param_pool,
                &mut self.objects,
                &mut self.next_object_id,
            );
            let obj_id = cls.do_construct_native(&mut ctx, target, ctor_id);
            // 仅当新建对象（target=None）时才归一化类名为注册键，确保后续 InvokeInstance
            // 能按同一键找到 native 类。若 target=Some（编译子类 super 调用到 native 父类），
            // 对象是子类实例，绝不能把其类名改成 native 父类名。
            if target.is_none() {
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.class_name = class_name.to_string();
                }
            }
            obj_id
        } else {
            0
        }
    }

    /// 把 native 方法写入参数池的返回值按结果地址类型落到对应栈
    ///
    /// native 桥接层将返回值写入参数池的返回位，本方法据结果地址的值类型
    /// 取出并写入相应类型栈。
    fn write_native_return_to_result(&mut self, result: Option<&Address>) {
        if let Some(addr) = result {
            match addr.value_type {
                ValueType::Int => {
                    let v = self.param_pool.get_int_return();
                    self.int_stack.write(addr.index, v);
                }
                ValueType::Float => {
                    let v = self.param_pool.get_float_return();
                    self.float_stack.write(addr.index, v);
                }
                ValueType::Bool => {
                    let v = self.param_pool.get_bool_return();
                    self.bool_stack.write(addr.index, v);
                }
                ValueType::String => {
                    let v = self.param_pool.get_string_return();
                    self.string_stack.write(addr.index, v);
                }
                ValueType::Object => {
                    let v = self.param_pool.get_object_return();
                    self.object_stack.write(addr.index, v);
                }
            }
        }
    }

    /// 建立编译对象与其 native 子对象之间的双向引用
    ///
    /// 对应 C# 中 `CompiledGorgeObject.NativeObject` 与 native 对象
    /// `OuterCompiledObject` 的互指。用于编译类继承 native 类的场景：
    /// - `compiled_id`：外层编译对象 ID
    /// - `native_id`：内层 native 子对象 ID
    ///
    /// 建立后：
    /// - 编译对象的 `native_object_id` = native_id
    /// - native 对象的 `outer_compiled_id` = compiled_id
    ///
    /// 注意：本机制的真正触发依赖“编译类继承 native 类”，需要编译器实现继承
    /// 编号冻结（Backlog B-3）与 native 类导入（Backlog B-4）后才会在端到端流程中
    /// 被使用；当前通过单元测试直接构造验证。
    pub fn link_native_and_compiled(&mut self, compiled_id: usize, native_id: usize) {
        if let Some(compiled) = self.objects.get_mut(&compiled_id) {
            compiled.native_object_id = Some(native_id);
        }
        if let Some(native) = self.objects.get_mut(&native_id) {
            native.outer_compiled_id = Some(compiled_id);
        }
    }

    /// 解析对象的“真实对象” ID（对应 C# `GorgeObject.RealObject`）
    ///
    /// 若对象是被编译类包裹的 native 子对象（存在 `outer_compiled_id`），
    /// 返回外层编译对象 ID；否则返回自身 ID。
    pub fn resolve_real_object_id(&self, obj_id: usize) -> usize {
        self.objects
            .get(&obj_id)
            .and_then(|o| o.outer_compiled_id)
            .unwrap_or(obj_id)
    }

    /// 从指令的 right 操作数解析目标类名（跨类静态调用/构造用）
    ///
    /// codegen 把 `InvokeStatic`/`InvokeConstructor` 的目标类名写入 right 操作数
    /// （String 立即数）。若无 right 或非字符串，返回 None（回退到 current_class）。
    fn read_target_class(right: Option<&Operand>) -> Option<String> {
        match right {
            Some(Operand::Immediate(crate::ir::ImmediateValue::String(s))) => Some(s.clone()),
            _ => None,
        }
    }

    /// 执行已编译方法的 IR 指令序列
    ///
    /// 调用前需先通过 `push_frame` 分配局部变量空间，调用后手动 `pop_frame` 释放。
    pub fn execute(&mut self, method: &CompiledMethod) -> VmResult<()> {
        self.pc = 0;

        let code_count = method.codes.len();
        while self.pc < code_count {
            let code_span = &method.codes[self.pc];
            let advance = self.execute_one(&code_span.code)?;
            if !advance {
                continue;
            }
            self.pc += 1;
        }

        Ok(())
    }

    /// 推入调用帧
    pub fn push_frame(&mut self, local_count: usize) {
        self.int_stack.push_frame(local_count);
        self.float_stack.push_frame(local_count);
        self.bool_stack.push_frame(local_count);
        self.string_stack.push_frame(local_count);
        self.object_stack.push_frame(local_count);
    }

    /// 弹出调用帧
    pub fn pop_frame(&mut self) {
        self.int_stack.pop_frame();
        self.float_stack.pop_frame();
        self.bool_stack.pop_frame();
        self.string_stack.pop_frame();
        self.object_stack.pop_frame();
    }

    /// 执行单条 IR 指令
    ///
    /// 返回 false 表示 pc 已被控制流指令修改（如 Jump），调用者不应自增 pc。
    fn execute_one(&mut self, code: &IntermediateCode) -> VmResult<bool> {
        match &code.operator {
            // === 本地变量赋值 ===
            IntermediateOperator::IntAssign => {
                let val = self.read_int_operand(&code.left);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::FloatAssign => {
                let val = self.read_float_operand(&code.left);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::BoolAssign => {
                let val = self.read_bool_operand(&code.left);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::StringAssign => {
                let val = self.read_string_operand(&code.left);
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::ObjectAssign => {
                let val = self.read_object_operand(&code.left);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }

            // === this 加载 ===
            IntermediateOperator::LoadThis => {
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, *self.object_stack.read(0));
            }

            // === 参数设置 ===
            IntermediateOperator::SetIntParameter => {
                // 完整实现：从 left 操作数读取值，以 result 地址为参数索引写入参数池
                let val = self.read_int_operand(&code.left);
                let index = self.get_int_addr(code.result);
                self.param_pool.set_int_param(index, val);
            }
            IntermediateOperator::SetFloatParameter => {
                let val = self.read_float_operand(&code.left);
                let index = self.get_int_addr(code.result);
                self.param_pool.set_float_param(index, val);
            }
            IntermediateOperator::SetBoolParameter => {
                let val = self.read_bool_operand(&code.left);
                let index = self.get_int_addr(code.result);
                self.param_pool.set_bool_param(index, val);
            }
            IntermediateOperator::SetStringParameter => {
                let val = self.read_string_operand(&code.left);
                let index = self.get_int_addr(code.result);
                self.param_pool.set_string_param(index, val);
            }
            IntermediateOperator::SetObjectParameter => {
                let val = self.read_object_operand(&code.left);
                let index = self.get_object_addr(code.result);
                self.param_pool.set_object_param(index, val);
            }

            // === 参数加载 ===
            IntermediateOperator::LoadIntParameter => {
                // 完整实现：以 left 操作数为参数索引从参数池读取，写入 result 地址
                let index = self.read_int_operand(&code.left) as usize;
                let val = self.param_pool.get_int_param(index);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::LoadFloatParameter => {
                let index = self.read_int_operand(&code.left) as usize;
                let val = self.param_pool.get_float_param(index);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::LoadBoolParameter => {
                let index = self.read_int_operand(&code.left) as usize;
                let val = self.param_pool.get_bool_param(index);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::LoadStringParameter => {
                let index = self.read_int_operand(&code.left) as usize;
                let val = self.param_pool.get_string_param(index);
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::LoadObjectParameter => {
                let index = self.read_int_operand(&code.left) as usize;
                let val = self.param_pool.get_object_param(index);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }

            // === 注入器 ===
            IntermediateOperator::LoadInjector => {
                // 将当前注入器对象 ID 加载到栈（对应 C# InvokeParameterPool.Injector）
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, self.current_injector.unwrap_or(0));
            }
            IntermediateOperator::SetInjector => {
                // 从 left 操作数读取对象 ID 并设为当前注入器
                self.current_injector = Some(self.read_object_operand(&code.left));
            }

            // === 算术运算 ===
            IntermediateOperator::IntAdd => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, lhs + rhs);
            }
            IntermediateOperator::IntSub => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, lhs - rhs);
            }
            IntermediateOperator::IntMul => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, lhs * rhs);
            }
            IntermediateOperator::IntDiv => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                if rhs == 0 {
                    return Err("除零错误".into());
                }
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, lhs / rhs);
            }
            IntermediateOperator::IntMod => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                if rhs == 0 {
                    return Err("取模除零错误".into());
                }
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, lhs % rhs);
            }

            IntermediateOperator::FloatAdd => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, lhs + rhs);
            }
            IntermediateOperator::FloatSub => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, lhs - rhs);
            }
            IntermediateOperator::FloatMul => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, lhs * rhs);
            }
            IntermediateOperator::FloatDiv => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, lhs / rhs);
            }

            // === 字符串加法 ===
            IntermediateOperator::StringAddition => {
                let lhs = self.read_string_operand(&code.left);
                let rhs = self.read_string_operand(code.right.as_ref().unwrap());
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, lhs + &rhs);
            }

            // === 比较运算 ===
            IntermediateOperator::IntLess => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs < rhs);
            }
            IntermediateOperator::IntLessEqual => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs <= rhs);
            }
            IntermediateOperator::IntGreater => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs > rhs);
            }
            IntermediateOperator::IntGreaterEqual => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs >= rhs);
            }
            IntermediateOperator::IntEqual => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs == rhs);
            }
            IntermediateOperator::IntNotEqual => {
                let lhs = self.read_int_operand(&code.left);
                let rhs = self.read_int_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs != rhs);
            }

            IntermediateOperator::FloatLess => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs < rhs);
            }
            IntermediateOperator::FloatLessEqual => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs <= rhs);
            }
            IntermediateOperator::FloatGreater => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs > rhs);
            }
            IntermediateOperator::FloatGreaterEqual => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs >= rhs);
            }
            IntermediateOperator::FloatEqual => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs == rhs);
            }
            IntermediateOperator::FloatNotEqual => {
                let lhs = self.read_float_operand(&code.left);
                let rhs = self.read_float_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs != rhs);
            }

            IntermediateOperator::BoolEqual => {
                let lhs = self.read_bool_operand(&code.left);
                let rhs = self.read_bool_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs == rhs);
            }
            IntermediateOperator::BoolNotEqual => {
                let lhs = self.read_bool_operand(&code.left);
                let rhs = self.read_bool_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs != rhs);
            }

            // === 逻辑运算 ===
            IntermediateOperator::LogicalAnd => {
                let lhs = self.read_bool_operand(&code.left);
                let rhs = self.read_bool_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs && rhs);
            }
            IntermediateOperator::LogicalOr => {
                let lhs = self.read_bool_operand(&code.left);
                let rhs = self.read_bool_operand(code.right.as_ref().unwrap());
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, lhs || rhs);
            }
            IntermediateOperator::LogicalNot => {
                let val = self.read_bool_operand(&code.left);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, !val);
            }

            // === 类型转换 ===
            IntermediateOperator::IntToFloat => {
                let val = self.read_int_operand(&code.left);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val as f64);
            }
            IntermediateOperator::FloatToInt => {
                let val = self.read_float_operand(&code.left);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val as i64);
            }
            IntermediateOperator::IntToBool => {
                let val = self.read_int_operand(&code.left);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val != 0);
            }
            IntermediateOperator::BoolToInt => {
                let val = self.read_bool_operand(&code.left);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, if val { 1 } else { 0 });
            }

            // === 转字符串 ===
            IntermediateOperator::IntCastToString => {
                let val = self.read_int_operand(&code.left);
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val.to_string());
            }
            IntermediateOperator::FloatCastToString => {
                let val = self.read_float_operand(&code.left);
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val.to_string());
            }
            IntermediateOperator::BoolCastToString => {
                let val = self.read_bool_operand(&code.left);
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val.to_string());
            }
            IntermediateOperator::ObjectCastToObject => {
                // 对象以 ID 传递，转型仅复制对象 ID（运行期不做类型检查，对齐 C#）
                let val = self.read_object_operand(&code.left);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }

            // === 控制流 ===
            IntermediateOperator::Jump(target) => {
                self.pc = *target;
                return Ok(false); // pc 已修改
            }
            IntermediateOperator::JumpIfFalse(target) => {
                let cond = self.read_bool_operand(&code.left);
                if !cond {
                    self.pc = *target;
                    return Ok(false);
                }
            }
            IntermediateOperator::JumpIfTrue(target) => {
                let cond = self.read_bool_operand(&code.left);
                if cond {
                    self.pc = *target;
                    return Ok(false);
                }
            }

            // === 返回值获取 ===
            IntermediateOperator::GetReturnInt => {
                // 完整实现：从参数池获取整数返回值，写入 result 地址
                let val = self.param_pool.get_int_return();
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::GetReturnFloat => {
                let val = self.param_pool.get_float_return();
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::GetReturnBool => {
                let val = self.param_pool.get_bool_return();
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::GetReturnString => {
                let val = self.param_pool.get_string_return();
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::GetReturnObject => {
                let val = self.param_pool.get_object_return();
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }

            // === 构造委托 ===
            IntermediateOperator::ConstructDelegate(idx) => {
                if let Some(ref result) = code.result {
                    self.object_stack.write(result.index, *idx);
                }
            }

            // === 返回 ===
            IntermediateOperator::ReturnInt => {
                let val = self.read_int_operand(&code.left);
                self.return_int = Some(val);
                self.pc = usize::MAX; // 退出循环
                return Ok(false);
            }
            IntermediateOperator::ReturnFloat => {
                let val = self.read_float_operand(&code.left);
                self.return_float = Some(val);
                self.pc = usize::MAX;
                return Ok(false);
            }
            IntermediateOperator::ReturnBool => {
                let val = self.read_bool_operand(&code.left);
                self.return_bool = Some(val);
                self.pc = usize::MAX;
                return Ok(false);
            }
            IntermediateOperator::ReturnString => {
                let val = self.read_string_operand(&code.left);
                self.return_string = Some(val);
                self.pc = usize::MAX;
                return Ok(false);
            }
            IntermediateOperator::ReturnObject => {
                let val = self.read_object_operand(&code.left);
                self.return_object = Some(val);
                self.pc = usize::MAX;
                return Ok(false);
            }
            IntermediateOperator::ReturnVoid => {
                self.pc = usize::MAX;
                return Ok(false);
            }

            // === 对象字段读取 ===
            IntermediateOperator::LoadIntField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.objects
                    .get(&obj_id)
                    .map(|obj| obj.get_int_field(*field_idx))
                    .unwrap_or(0);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::LoadFloatField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.objects
                    .get(&obj_id)
                    .map(|obj| obj.get_float_field(*field_idx))
                    .unwrap_or(0.0);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::LoadBoolField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.objects
                    .get(&obj_id)
                    .map(|obj| obj.get_bool_field(*field_idx))
                    .unwrap_or(false);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::LoadStringField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.objects
                    .get(&obj_id)
                    .map(|obj| obj.get_string_field(*field_idx))
                    .unwrap_or_default();
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::LoadObjectField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.objects
                    .get(&obj_id)
                    .map(|obj| obj.get_object_field(*field_idx))
                    .unwrap_or(0);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }

            // === 对象字段写入 ===
            IntermediateOperator::SetIntField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_int_operand(&code.left);
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_int_field(*field_idx, val);
                }
            }
            IntermediateOperator::SetFloatField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_float_operand(&code.left);
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_float_field(*field_idx, val);
                }
            }
            IntermediateOperator::SetBoolField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_bool_operand(&code.left);
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_bool_field(*field_idx, val);
                }
            }
            IntermediateOperator::SetStringField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_string_operand(&code.left);
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_string_field(*field_idx, val);
                }
            }
            IntermediateOperator::SetObjectField(field_idx) => {
                let obj_id = *self.object_stack.read(0);
                let val = self.read_object_operand(&code.left);
                if let Some(obj) = self.objects.get_mut(&obj_id) {
                    obj.set_object_field(*field_idx, val);
                }
            }

            // === 静态字段读写 ===
            IntermediateOperator::LoadStaticIntField(field_idx) => {
                let val = self.class_static_fields
                    .get(&self.current_class)
                    .map(|pool| pool.get_int(*field_idx))
                    .unwrap_or(0);
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::LoadStaticFloatField(field_idx) => {
                let val = self.class_static_fields
                    .get(&self.current_class)
                    .map(|pool| pool.get_float(*field_idx))
                    .unwrap_or(0.0);
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::LoadStaticBoolField(field_idx) => {
                let val = self.class_static_fields
                    .get(&self.current_class)
                    .map(|pool| pool.get_bool(*field_idx))
                    .unwrap_or(false);
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::LoadStaticStringField(field_idx) => {
                let val = self.class_static_fields
                    .get(&self.current_class)
                    .map(|pool| pool.get_string(*field_idx).to_string())
                    .unwrap_or_default();
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::LoadStaticObjectField(field_idx) => {
                let val = self.class_static_fields
                    .get(&self.current_class)
                    .map(|pool| pool.get_object(*field_idx))
                    .unwrap_or(0);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }
            IntermediateOperator::SetStaticIntField(field_idx) => {
                let val = self.read_int_operand(&code.left);
                if let Some(pool) = self.class_static_fields.get_mut(&self.current_class) {
                    pool.set_int(*field_idx, val);
                }
            }
            IntermediateOperator::SetStaticFloatField(field_idx) => {
                let val = self.read_float_operand(&code.left);
                if let Some(pool) = self.class_static_fields.get_mut(&self.current_class) {
                    pool.set_float(*field_idx, val);
                }
            }
            IntermediateOperator::SetStaticBoolField(field_idx) => {
                let val = self.read_bool_operand(&code.left);
                if let Some(pool) = self.class_static_fields.get_mut(&self.current_class) {
                    pool.set_bool(*field_idx, val);
                }
            }
            IntermediateOperator::SetStaticStringField(field_idx) => {
                let val = self.read_string_operand(&code.left);
                if let Some(pool) = self.class_static_fields.get_mut(&self.current_class) {
                    pool.set_string(*field_idx, val);
                }
            }
            IntermediateOperator::SetStaticObjectField(field_idx) => {
                let val = self.read_object_operand(&code.left);
                if let Some(pool) = self.class_static_fields.get_mut(&self.current_class) {
                    pool.set_object(*field_idx, val);
                }
            }

            // === 注入器字段读写 ===
            IntermediateOperator::LoadIntInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = if let Some(inj) = self.injectors.get(&inj_id) {
                    if inj.get_injector_int_default_value(*field_idx) {
                        // 默认值：从类级默认值池查找
                        self.lookup_injector_default_int(inj_id, *field_idx).unwrap_or(0)
                    } else {
                        inj.get_injector_int(*field_idx)
                    }
                } else { 0 };
                let addr = self.get_int_addr(code.result);
                self.int_stack.write(addr, val);
            }
            IntermediateOperator::LoadFloatInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = if let Some(inj) = self.injectors.get(&inj_id) {
                    if inj.get_injector_float_default_value(*field_idx) { 0.0 }
                    else { inj.get_injector_float(*field_idx) }
                } else { 0.0 };
                let addr = self.get_float_addr(code.result);
                self.float_stack.write(addr, val);
            }
            IntermediateOperator::LoadBoolInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = if let Some(inj) = self.injectors.get(&inj_id) {
                    if inj.get_injector_bool_default_value(*field_idx) { false }
                    else { inj.get_injector_bool(*field_idx) }
                } else { false };
                let addr = self.get_bool_addr(code.result);
                self.bool_stack.write(addr, val);
            }
            IntermediateOperator::LoadStringInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = if let Some(inj) = self.injectors.get(&inj_id) {
                    if inj.get_injector_string_default_value(*field_idx) { String::new() }
                    else { inj.get_injector_string(*field_idx) }
                } else { String::new() };
                let addr = self.get_string_addr(code.result);
                self.string_stack.write(addr, val);
            }
            IntermediateOperator::LoadObjectInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = if let Some(inj) = self.injectors.get(&inj_id) {
                    if inj.get_injector_object_default_value(*field_idx) { 0 }
                    else { inj.get_injector_object(*field_idx) }
                } else { 0 };
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, val);
            }
            IntermediateOperator::SetIntInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = self.read_int_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_int(*field_idx, val);
                }
            }
            IntermediateOperator::SetFloatInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = self.read_float_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_float(*field_idx, val);
                }
            }
            IntermediateOperator::SetBoolInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = self.read_bool_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_bool(*field_idx, val);
                }
            }
            IntermediateOperator::SetStringInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = self.read_string_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_string(*field_idx, val);
                }
            }
            IntermediateOperator::SetObjectInjectorField(field_idx) => {
                let inj_id = *self.object_stack.read(0);
                let val = self.read_object_operand(&code.left);
                if let Some(inj) = self.injectors.get_mut(&inj_id) {
                    inj.set_injector_object(*field_idx, val);
                }
            }

            // === 方法调用 ===
            IntermediateOperator::InvokeInstance(method_id) => {
                // 从 left 操作数获取目标对象 ID
                let target_obj_id = self.read_object_operand(&code.left);
                if target_obj_id == 0 {
                    return Err("调用目标对象为空".into());
                }
                // 从对象表获取 RuntimeObject
                let class_name = self.objects
                    .get(&target_obj_id)
                    .map(|obj| obj.class_name.clone())
                    .unwrap_or_default();
                if class_name.is_empty() {
                    return Err("无法确定目标对象的类".into());
                }
                // 若目标对象属于 native 类，分派到 native 桥接层
                if self.native_class_table.contains_key(&class_name) {
                    self.dispatch_native_method(&class_name, target_obj_id, *method_id);
                    self.write_native_return_to_result(code.result.as_ref());
                    return Ok(true);
                }
                // 查找类的方法实现
                let method = match self.class_table
                    .get(&class_name)
                    .and_then(|cls| cls.find_method(*method_id))
                {
                    Some(m) => m,
                    None => {
                        // 编译子类继承 native 类：方法可能属于 native 祖先（F2）。
                        // 沿 super_class 链找到 native 祖先，方法全局 ID 直接作为其方法索引分派。
                        if let Some(native_anc) = self.find_native_ancestor(&class_name) {
                            self.dispatch_native_method(&native_anc, target_obj_id, *method_id);
                            self.write_native_return_to_result(code.result.as_ref());
                        }
                        return Ok(true);
                    }
                };
                // 保存栈状态并执行
                let saved_pc = self.pc;
                let save_len = self.int_stack.data.len();
                let saved_ints: Vec<i64> = (0..save_len).map(|i| *self.int_stack.read(i)).collect();
                let saved_floats: Vec<f64> = (0..save_len).map(|i| *self.float_stack.read(i)).collect();
                let saved_bools: Vec<bool> = (0..save_len).map(|i| *self.bool_stack.read(i)).collect();
                let saved_objects: Vec<usize> = (0..save_len).map(|i| *self.object_stack.read(i)).collect();
                let max_locals = method.local_count;
                self.int_stack.write(max_locals.saturating_sub(1), 0);
                self.float_stack.write(max_locals.saturating_sub(1), 0.0);
                self.bool_stack.write(max_locals.saturating_sub(1), false);
                self.string_stack.write(max_locals.saturating_sub(1), String::new());
                self.object_stack.write(max_locals.saturating_sub(1), 0);
                // 将参数从 param_pool 复制到 callee 的局部槽位。
                // 参数按值类型分组存于池中（B-2），方法参数占据每种类型最低的连续局部索引，
                // 因此对每种类型从 0 起连续复制已设置的参数，遇首个未设置即停。
                self.copy_params_to_locals(method.local_count);
                // 将目标对象放在 object_stack[0] 作为 callee 的 this
                self.object_stack.write(0, target_obj_id);
                // 执行子方法
                self.pc = 0;
                let count = method.codes.len();
                while self.pc < count {
                    let cs = &method.codes[self.pc];
                    let advance = self.execute_one(&cs.code)?;
                    if !advance {
                        if self.pc >= count { break; }
                        continue;
                    }
                    self.pc += 1;
                }
                // 将返回值写入 result
                let result_index = code.result.as_ref().map(|r| r.index);
                if let Some(ri) = result_index {
                    if let Some(v) = self.return_int {
                        self.int_stack.write(ri, v);
                    }
                }
                // 恢复调用者栈
                for (i, v) in saved_ints.iter().enumerate() {
                    if Some(i) != result_index {
                        self.int_stack.write(i, *v);
                    }
                }
                for (i, v) in saved_floats.iter().enumerate() {
                    self.float_stack.write(i, *v);
                }
                for (i, v) in saved_bools.iter().enumerate() {
                    self.bool_stack.write(i, *v);
                }
                for (i, v) in saved_objects.iter().enumerate() {
                    if Some(i) != result_index {
                        self.object_stack.write(i, *v);
                    }
                }
                self.pc = saved_pc;
                return Ok(true);
            }
            IntermediateOperator::InvokeStatic(idx) => {
                // 解析目标类：优先用 right 携带的类名（跨类调用），否则回退当前类
                let target_class = Self::read_target_class(code.right.as_ref())
                    .unwrap_or_else(|| self.current_class.clone());
                // 若目标类是 native 类，分派到 native 桥接层
                if self.native_class_table.contains_key(&target_class) {
                    self.dispatch_native_static(&target_class, *idx);
                    // 将返回值写回 result 地址（按结果地址类型）
                    self.write_native_return_to_result(code.result.as_ref());
                    return Ok(true);
                }
                let methods = self.class_static_methods
                    .get(&target_class)
                    .ok_or_else(|| format!("类 `{}` 未注册方法表", target_class))?;
                if *idx >= methods.len() {
                    return Err(format!("静态方法索引 {} 越界", idx));
                }
                let (static_method, _param_types) = methods[*idx].clone();
                // left 操作数存参数计数
                let saved_pc = self.pc;
                let save_len = self.int_stack.data.len();
                let saved_ints: Vec<i64> = (0..save_len).map(|i| *self.int_stack.read(i)).collect();
                let saved_floats: Vec<f64> = (0..save_len).map(|i| *self.float_stack.read(i)).collect();
                let saved_bools: Vec<bool> = (0..save_len).map(|i| *self.bool_stack.read(i)).collect();
                let max_locals = static_method.local_count;
                self.int_stack.write(max_locals.saturating_sub(1), 0);
                self.float_stack.write(max_locals.saturating_sub(1), 0.0);
                self.bool_stack.write(max_locals.saturating_sub(1), false);
                self.string_stack.write(max_locals.saturating_sub(1), String::new());
                self.object_stack.write(max_locals.saturating_sub(1), 0);
                // 将参数从 param_pool 复制到 callee 局部槽位（按类型分组，消费后重置池）
                self.copy_params_to_locals(max_locals);
                // 执行子方法（切换 current_class 到目标类，执行完恢复）
                let saved_class = self.current_class.clone();
                self.current_class = target_class.clone();
                self.pc = 0;
                let count = static_method.codes.len();
                while self.pc < count {
                    let cs = &static_method.codes[self.pc];
                    let advance = self.execute_one(&cs.code)?;
                    if !advance {
                        if self.pc >= count { break; }
                        continue;
                    }
                    self.pc += 1;
                }
                self.current_class = saved_class;
                let result_index = code.result.as_ref().map(|r| r.index);
                if let Some(ri) = result_index {
                    if let Some(v) = self.return_int {
                        self.int_stack.write(ri, v);
                    }
                }
                for (i, v) in saved_ints.iter().enumerate() {
                    if Some(i) != result_index {
                        self.int_stack.write(i, *v);
                    }
                }
                for (i, v) in saved_floats.iter().enumerate() {
                    self.float_stack.write(i, *v);
                }
                for (i, v) in saved_bools.iter().enumerate() {
                    self.bool_stack.write(i, *v);
                }
                self.pc = saved_pc;
                return Ok(true);
            }
            IntermediateOperator::InvokeInterface(iface_method_id) => {
                // 目标对象在 left；接口全名在 right
                let target_obj_id = self.read_object_operand(&code.left);
                if target_obj_id == 0 {
                    return Err("接口方法调用目标对象为空".into());
                }
                let iface_name = match Self::read_target_class(code.right.as_ref()) {
                    Some(n) => n,
                    None => return Ok(true),
                };
                // 取目标对象的运行时类
                let class_name = self.objects
                    .get(&target_obj_id)
                    .map(|o| o.class_name.clone())
                    .unwrap_or_default();
                if class_name.is_empty() {
                    return Err("无法确定接口调用目标对象的类".into());
                }
                // 通过类的接口方法实现映射把接口方法本地ID解析为类方法全局ID
                let global_method_id = self.class_table
                    .get(&class_name)
                    .and_then(|cls| cls.declaration.interface_method_impl_id.get(&iface_name))
                    .and_then(|ids| ids.get(*iface_method_id))
                    .copied();
                let global_method_id = match global_method_id {
                    Some(id) if id != usize::MAX => id,
                    _ => return Ok(true), // 未实现或无映射，静默跳过
                };
                // 查类方法实现并执行（与 InvokeInstance 相同的执行流程）
                let method = match self.class_table
                    .get(&class_name)
                    .and_then(|cls| cls.find_method(global_method_id))
                {
                    Some(m) => m,
                    None => return Ok(true),
                };
                let saved_pc = self.pc;
                let save_len = self.int_stack.data.len();
                let saved_ints: Vec<i64> = (0..save_len).map(|i| *self.int_stack.read(i)).collect();
                let saved_floats: Vec<f64> = (0..save_len).map(|i| *self.float_stack.read(i)).collect();
                let saved_bools: Vec<bool> = (0..save_len).map(|i| *self.bool_stack.read(i)).collect();
                let saved_objects: Vec<usize> = (0..save_len).map(|i| *self.object_stack.read(i)).collect();
                let max_locals = method.local_count;
                self.int_stack.write(max_locals.saturating_sub(1), 0);
                self.float_stack.write(max_locals.saturating_sub(1), 0.0);
                self.bool_stack.write(max_locals.saturating_sub(1), false);
                self.string_stack.write(max_locals.saturating_sub(1), String::new());
                self.object_stack.write(max_locals.saturating_sub(1), 0);
                self.copy_params_to_locals(max_locals);
                self.object_stack.write(0, target_obj_id);
                let saved_class = self.current_class.clone();
                self.current_class = class_name.clone();
                self.pc = 0;
                let count = method.codes.len();
                while self.pc < count {
                    let cs = &method.codes[self.pc];
                    let advance = self.execute_one(&cs.code)?;
                    if !advance {
                        if self.pc >= count { break; }
                        continue;
                    }
                    self.pc += 1;
                }
                self.current_class = saved_class;
                // 写回返回值
                if let Some(addr) = code.result.as_ref() {
                    match addr.value_type {
                        ValueType::Int => { if let Some(v) = self.return_int { self.int_stack.write(addr.index, v); } }
                        ValueType::Float => { if let Some(v) = self.return_float { self.float_stack.write(addr.index, v); } }
                        ValueType::Bool => { if let Some(v) = self.return_bool { self.bool_stack.write(addr.index, v); } }
                        ValueType::String => { if let Some(v) = self.return_string.clone() { self.string_stack.write(addr.index, v); } }
                        ValueType::Object => { if let Some(v) = self.return_object { self.object_stack.write(addr.index, v); } }
                    }
                }
                // 恢复调用者栈
                let result_index = code.result.as_ref().map(|r| r.index);
                for (i, v) in saved_ints.iter().enumerate() {
                    if Some(i) != result_index { self.int_stack.write(i, *v); }
                }
                for (i, v) in saved_floats.iter().enumerate() {
                    if Some(i) != result_index { self.float_stack.write(i, *v); }
                }
                for (i, v) in saved_bools.iter().enumerate() {
                    if Some(i) != result_index { self.bool_stack.write(i, *v); }
                }
                for (i, v) in saved_objects.iter().enumerate() {
                    if Some(i) != result_index { self.object_stack.write(i, *v); }
                }
                self.pc = saved_pc;
                return Ok(true);
            }
            IntermediateOperator::InvokeDelegate(idx) => {
                let delegates = self.class_delegate_impls
                    .get(&self.current_class)
                    .ok_or_else(|| format!("类 `{}` 未注册委托表", self.current_class))?;
                if *idx >= delegates.len() {
                    return Err(format!("类 `{}` 委托索引 {} 越界（共 {} 个）", self.current_class, idx, delegates.len()));
                }
                let (delegate_method, param_types) = delegates[*idx].clone();
                let saved_pc = self.pc;
                let saved_return_int = self.return_int;
                // 保存父帧索引范围的旧值
                let save_len = self.int_stack.data.len();
                let saved_ints: Vec<i64> = (0..save_len).map(|i| *self.int_stack.read(i)).collect();
                let saved_floats: Vec<f64> = (0..save_len).map(|i| *self.float_stack.read(i)).collect();
                let saved_bools: Vec<bool> = (0..save_len).map(|i| *self.bool_stack.read(i)).collect();
                // 确保栈包含委托需要的所有局部变量空间
                let max_locals = delegate_method.local_count;
                self.int_stack.write(max_locals.saturating_sub(1), 0);
                self.float_stack.write(max_locals.saturating_sub(1), 0.0);
                self.bool_stack.write(max_locals.saturating_sub(1), false);
                self.string_stack.write(max_locals.saturating_sub(1), String::new());
                self.object_stack.write(max_locals.saturating_sub(1), 0);
                // 将参数从 param_pool 复制到栈前 max_locals 位置
                for (i, pt) in param_types.iter().enumerate() {
                    match pt {
                        ValueType::Int => {
                            let val = self.param_pool.get_int_param(i);
                            self.int_stack.write(i, val);
                        }
                        ValueType::Float => {
                            let val = self.param_pool.get_float_param(i);
                            self.float_stack.write(i, val);
                        }
                        ValueType::Bool => {
                            let val = self.param_pool.get_bool_param(i);
                            self.bool_stack.write(i, val);
                        }
                        ValueType::String => {
                            let val = self.param_pool.get_string_param(i);
                            self.string_stack.write(i, val);
                        }
                        ValueType::Object => {
                            let val = self.param_pool.get_object_param(i);
                            self.object_stack.write(i, val);
                        }
                    }
                }
                self.pc = 0;
                let count = delegate_method.codes.len();
                while self.pc < count {
                    let cs = &delegate_method.codes[self.pc];
                    let advance = self.execute_one(&cs.code)?;
                    if !advance {
                        if self.pc >= count { break; }
                        continue;
                    }
                    self.pc += 1;
                }
                if let Some(ref r) = code.result {
                    if let Some(v) = self.return_int {
                        self.int_stack.write(r.index, v);
                    }
                }
                // 恢复父帧值（除返回结果位置外）
                let result_index = code.result.as_ref().map(|r| r.index);
                for (i, v) in saved_ints.iter().enumerate() {
                    if Some(i) != result_index {
                        self.int_stack.write(i, *v);
                    }
                }
                for (i, v) in saved_floats.iter().enumerate() {
                    self.float_stack.write(i, *v);
                }
                for (i, v) in saved_bools.iter().enumerate() {
                    self.bool_stack.write(i, *v);
                }
                self.return_int = saved_return_int;
                self.pc = saved_pc;
                return Ok(true);
            }

            // === 构造 ===
            IntermediateOperator::InvokeConstructor(ctor_id) => {
                // 解析目标类：优先用 right 携带的类名（跨类构造），否则回退当前类
                let target_class = Self::read_target_class(code.right.as_ref())
                    .unwrap_or_else(|| self.current_class.clone());
                // 若目标类是 native 类，分派到 native 构造桥接层
                if self.native_class_table.contains_key(&target_class) {
                    let new_id = self.dispatch_native_construct(&target_class, None, *ctor_id);
                    // native 构造把对象 ID 也写入返回位，统一以返回位为准
                    let obj_id = if new_id != 0 {
                        new_id
                    } else {
                        self.param_pool.get_object_return()
                    };
                    let result_addr = self.get_object_addr(code.result);
                    self.object_stack.write(result_addr, obj_id);
                    return Ok(true);
                }
                // 1. 创建对象
                let obj_id = self.next_object_id;
                self.next_object_id += 1;
                let field_counts = self.class_field_counts
                    .get(&target_class)
                    .cloned()
                    .unwrap_or_default();
                let obj = RuntimeObject::new_simple(target_class.clone(), &field_counts);
                self.objects.insert(obj_id, obj);
                self.object_stack.write(0, obj_id);

                // 2. 写入结果地址
                let result_addr = self.get_object_addr(code.result);
                self.object_stack.write(result_addr, obj_id);

                // 3. 查找并执行构造方法（若无显式构造方法则仅创建对象）
                if let Some(ctor_method) = self.class_table
                    .get(&target_class)
                    .and_then(|cls| cls.find_constructor(*ctor_id))
                {
                    let saved_pc = self.pc;
                    let save_len = self.int_stack.data.len();
                    let saved_ints: Vec<i64> = (0..save_len).map(|i| *self.int_stack.read(i)).collect();
                    let saved_floats: Vec<f64> = (0..save_len).map(|i| *self.float_stack.read(i)).collect();
                    let saved_bools: Vec<bool> = (0..save_len).map(|i| *self.bool_stack.read(i)).collect();
                    let saved_objects: Vec<usize> = (0..save_len).map(|i| *self.object_stack.read(i)).collect();
                    let max_locals = ctor_method.local_count;
                    self.int_stack.write(max_locals.saturating_sub(1), 0);
                    self.float_stack.write(max_locals.saturating_sub(1), 0.0);
                    self.bool_stack.write(max_locals.saturating_sub(1), false);
                    self.string_stack.write(max_locals.saturating_sub(1), String::new());
                    self.object_stack.write(max_locals.saturating_sub(1), 0);
                    self.copy_params_to_locals(max_locals);
                    // 重新确立 this（栈准备/参数复制可能覆盖 object_stack[0]）
                    self.object_stack.write(0, obj_id);
                    self.pc = 0;
                    let count = ctor_method.codes.len();
                    let saved_ctor_class = self.current_class.clone();
                    self.current_class = target_class.clone();
                    while self.pc < count {
                        let cs = &ctor_method.codes[self.pc];
                        let advance = self.execute_one(&cs.code)?;
                        if !advance {
                            if self.pc >= count { break; }
                            continue;
                        }
                        self.pc += 1;
                    }
                    self.current_class = saved_ctor_class;
                    let result_index = code.result.as_ref().map(|r| r.index);
                    for (i, v) in saved_ints.iter().enumerate() {
                        if Some(i) != result_index {
                            self.int_stack.write(i, *v);
                        }
                    }
                    for (i, v) in saved_floats.iter().enumerate() {
                        self.float_stack.write(i, *v);
                    }
                    for (i, v) in saved_bools.iter().enumerate() {
                        self.bool_stack.write(i, *v);
                    }
                    for (i, v) in saved_objects.iter().enumerate() {
                        if Some(i) != result_index {
                            self.object_stack.write(i, *v);
                        }
                    }
                    // 恢复后重新写入结果（避免被 max_locals 准备覆盖）
                    let addr = self.get_object_addr(code.result);
                    self.object_stack.write(addr, obj_id);
                    self.pc = saved_pc;
                }
                return Ok(true);
            }
            IntermediateOperator::DoConstruct(_) => {
                // 创建正式运行时对象
                let obj_id = self.next_object_id;
                self.next_object_id += 1;
                // 从类字段计数表中获取字段数量
                let field_counts = self.class_field_counts
                    .get(&self.current_class)
                    .cloned()
                    .unwrap_or_default();
                let obj = RuntimeObject::new_simple(self.current_class.clone(), &field_counts);
                // 存入对象表，并将 this(=0) 更新为当前对象 ID
                self.objects.insert(obj_id, obj);
                self.object_stack.write(0, obj_id);
                // 返回新对象 ID
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, obj_id);
            }

            // === 父类构造调用（super）===
            IntermediateOperator::InvokeSuperConstructor(ctor_id) => {
                // 目标父类名经 right 操作数传递
                let super_class = match Self::read_target_class(code.right.as_ref()) {
                    Some(c) => c,
                    None => return Ok(true),
                };
                // 当前 this 对象（object_stack[0]），父类构造在同一对象上执行
                let this_id = *self.object_stack.read(0);

                // native 父类：在已有 this 上执行 native 构造
                if self.native_class_table.contains_key(&super_class) {
                    let _ = self.dispatch_native_construct(&super_class, Some(this_id), *ctor_id);
                    return Ok(true);
                }

                // 编译父类：查找父类构造方法并在 this 上执行其体
                let ctor_method = match self.class_table
                    .get(&super_class)
                    .and_then(|cls| cls.find_constructor(*ctor_id))
                {
                    Some(m) => m,
                    None => return Ok(true),
                };
                let param_count = match &code.left {
                    Operand::Immediate(crate::ir::ImmediateValue::Int(v)) => *v as usize,
                    _ => 0,
                };
                let saved_pc = self.pc;
                let save_len = self.int_stack.data.len();
                let saved_ints: Vec<i64> = (0..save_len).map(|i| *self.int_stack.read(i)).collect();
                let saved_floats: Vec<f64> = (0..save_len).map(|i| *self.float_stack.read(i)).collect();
                let saved_bools: Vec<bool> = (0..save_len).map(|i| *self.bool_stack.read(i)).collect();
                let saved_objects: Vec<usize> = (0..save_len).map(|i| *self.object_stack.read(i)).collect();
                let max_locals = ctor_method.local_count;
                self.int_stack.write(max_locals.saturating_sub(1), 0);
                self.float_stack.write(max_locals.saturating_sub(1), 0.0);
                self.bool_stack.write(max_locals.saturating_sub(1), false);
                self.string_stack.write(max_locals.saturating_sub(1), String::new());
                self.object_stack.write(max_locals.saturating_sub(1), 0);
                // 复制父类构造参数（按类型分组从参数池取）
                for i in 0..param_count {
                    self.int_stack.write(i, self.param_pool.get_int_param(i));
                    self.float_stack.write(i, self.param_pool.get_float_param(i));
                    self.bool_stack.write(i, self.param_pool.get_bool_param(i));
                    self.string_stack.write(i, self.param_pool.get_string_param(i));
                    self.object_stack.write(i, self.param_pool.get_object_param(i));
                }
                // this 保持为当前对象
                self.object_stack.write(0, this_id);
                // 执行父类构造体（切换 current_class 到父类）
                let saved_class = self.current_class.clone();
                self.current_class = super_class.clone();
                self.pc = 0;
                let count = ctor_method.codes.len();
                while self.pc < count {
                    let cs = &ctor_method.codes[self.pc];
                    let advance = self.execute_one(&cs.code)?;
                    if !advance {
                        if self.pc >= count { break; }
                        continue;
                    }
                    self.pc += 1;
                }
                self.current_class = saved_class;
                // 恢复调用者栈（this 已写入对象字段，无需保留父构造的局部）
                for (i, v) in saved_ints.iter().enumerate() { self.int_stack.write(i, *v); }
                for (i, v) in saved_floats.iter().enumerate() { self.float_stack.write(i, *v); }
                for (i, v) in saved_bools.iter().enumerate() { self.bool_stack.write(i, *v); }
                for (i, v) in saved_objects.iter().enumerate() { self.object_stack.write(i, *v); }
                self.pc = saved_pc;
            }

            // === Nop ===
            IntermediateOperator::Nop => {}

            // 注入器常量加载（G2）
            IntermediateOperator::LoadInjectorConstant(idx) => {
                let constant = match self.injector_constants.get(*idx) {
                    Some(c) => c.clone(),
                    None => {
                        if std::env::var("GORGE_VM_DEBUG").is_ok() {
                            eprintln!("[vm] LoadInjectorConstant({}) 越界 (总 {})", idx, self.injector_constants.len());
                        }
                        return Ok(true);
                    }
                };
                let inj = crate::injector::RuntimeInjector::from_constant(&constant);
                let inj_id = self.next_object_id;
                self.next_object_id += 1;
                if std::env::var("GORGE_VM_DEBUG").is_ok() {
                    eprintln!("[vm] LoadInjectorConstant({}) class={} id={}", idx, constant.class_name, inj_id);
                }
                self.injectors.insert(inj_id, inj);
                let addr = self.get_object_addr(code.result);
                self.object_stack.write(addr, inj_id);
            }

            // 未实现的操作码
            _ => {
                return Err(format!("未实现的操作码: {:?}", code.operator));
            }
        }

        Ok(true) // pc 正常自增
    }

    /// 获取 int 类型的返回值
    pub fn get_return_int(&self) -> Option<i64> {
        self.return_int
    }

    /// 获取 float 类型的返回值
    pub fn get_return_float(&self) -> Option<f64> {
        self.return_float
    }

    /// 获取 bool 类型的返回值
    pub fn get_return_bool(&self) -> Option<bool> {
        self.return_bool
    }

    /// 获取执行后的所有栈数据（调试用）
    pub fn dump_stacks(&self) -> String {
        format!(
            "int: {:?}\nfloat: {:?}\nbool: {:?}",
            self.int_stack.data,
            self.float_stack.data,
            self.bool_stack.data,
        )
    }

    /// 从类级注入器默认值池查找整数默认值
    fn lookup_injector_default_int(&self, inj_id: usize, field_idx: usize) -> Option<i64> {
        let obj = self.objects.get(&inj_id)?;
        let cls = self.class_table.get(&obj.class_name)?;
        Some(cls.get_injector_int_default(field_idx))
    }

    // === 操作数读取辅助方法 ===

    fn read_int_operand(&self, op: &Operand) -> i64 {
        match op {
            Operand::Immediate(ImmediateValue::Int(v)) => *v,
            Operand::Address(addr) => *self.int_stack.read(addr.index),
            _ => 0,
        }
    }

    fn read_float_operand(&self, op: &Operand) -> f64 {
        match op {
            Operand::Immediate(ImmediateValue::Float(v)) => *v,
            Operand::Address(addr) => *self.float_stack.read(addr.index),
            _ => 0.0,
        }
    }

    fn read_bool_operand(&self, op: &Operand) -> bool {
        match op {
            Operand::Immediate(ImmediateValue::Bool(v)) => *v,
            Operand::Address(addr) => *self.bool_stack.read(addr.index),
            _ => false,
        }
    }

    fn read_string_operand(&self, op: &Operand) -> String {
        match op {
            Operand::Immediate(ImmediateValue::String(s)) => s.clone(),
            Operand::Address(addr) => self.string_stack.read(addr.index).clone(),
            _ => String::new(),
        }
    }

    fn read_object_operand(&self, op: &Operand) -> usize {
        match op {
            Operand::Address(addr) => *self.object_stack.read(addr.index),
            _ => 0,
        }
    }

    fn get_int_addr(&self, addr: Option<Address>) -> usize {
        addr.map(|a| a.index).unwrap_or(0)
    }

    fn get_float_addr(&self, addr: Option<Address>) -> usize {
        addr.map(|a| a.index).unwrap_or(0)
    }

    fn get_bool_addr(&self, addr: Option<Address>) -> usize {
        addr.map(|a| a.index).unwrap_or(0)
    }

    fn get_string_addr(&self, addr: Option<Address>) -> usize {
        addr.map(|a| a.index).unwrap_or(0)
    }

    fn get_object_addr(&self, addr: Option<Address>) -> usize {
        addr.map(|a| a.index).unwrap_or(0)
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for VirtualMachine {
    fn clone(&self) -> Self {
        Self {
            int_stack: self.int_stack.clone(),
            float_stack: self.float_stack.clone(),
            bool_stack: self.bool_stack.clone(),
            string_stack: self.string_stack.clone(),
            object_stack: self.object_stack.clone(),
            next_object_id: self.next_object_id,
            param_pool: self.param_pool.clone(),
            objects: HashMap::new(),
            injectors: HashMap::new(),
            class_field_counts: self.class_field_counts.clone(),
            class_static_fields: self.class_static_fields.clone(),
            class_delegate_impls: self.class_delegate_impls.clone(),
            class_table: self.class_table.clone(),
            native_class_table: self.native_class_table.clone(),
            class_super_name: self.class_super_name.clone(),
            injector_constants: self.injector_constants.clone(),
            class_static_methods: self.class_static_methods.clone(),
            current_class: self.current_class.clone(),
            current_injector: self.current_injector,
            delegate_impls: self.delegate_impls.clone(),
            pc: self.pc,
            return_int: self.return_int,
            return_float: self.return_float,
            return_bool: self.return_bool,
            return_string: self.return_string.clone(),
            return_object: self.return_object,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_int_addr(index: usize) -> Address {
        Address::new(ValueType::Int, index)
    }

    fn make_bool_addr(index: usize) -> Address {
        Address::new(ValueType::Bool, index)
    }

    fn make_float_addr(index: usize) -> Address {
        Address::new(ValueType::Float, index)
    }

    #[test]
    fn test_vm_stack_push_pop() {
        let mut stack: VmStack<i64> = VmStack::new();
        stack.push_frame(3);
        stack.write(0, 10);
        stack.write(1, 20);
        stack.write(2, 30);
        assert_eq!(*stack.read(0), 10);
        assert_eq!(*stack.read(1), 20);
        assert_eq!(stack.len(), 3);

        stack.pop_frame();
        assert_eq!(stack.len(), 0); // 退回根帧
    }

    #[test]
    fn test_execute_int_add() {
        let a = make_int_addr(0);
        let b = make_int_addr(1);
        let r = make_int_addr(2);

        let codes = vec![
            CodeWithSpan::new(
                IntermediateCode::assign(a, Operand::int(1)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::assign(b, Operand::int(2)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::binary(
                    IntermediateOperator::IntAdd,
                    Operand::Address(a),
                    Operand::Address(b),
                    r,
                ),
                crate::diagnostics::Span::dummy(),
            ),
        ];

        let method = CompiledMethod {
            name: "test".into(),
            codes,
            local_count: 3,
        };

        let mut vm = VirtualMachine::new();
        vm.push_frame(method.local_count);
        vm.execute(&method).unwrap();
        assert_eq!(*vm.int_stack.read(2), 3);
    }

    #[test]
    fn test_execute_return_int() {
        let r = make_int_addr(0);

        let codes = vec![
            CodeWithSpan::new(
                IntermediateCode::assign(r, Operand::int(42)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::return_value(ValueType::Int),
                crate::diagnostics::Span::dummy(),
            ),
        ];

        let method = CompiledMethod {
            name: "test".into(),
            codes,
            local_count: 1,
        };

        let mut vm = VirtualMachine::new();
        vm.push_frame(method.local_count);
        vm.execute(&method).unwrap();
        assert_eq!(vm.get_return_int(), Some(42));
    }

    #[test]
    fn test_execute_comparison() {
        let a = make_int_addr(0);
        let b = make_int_addr(1);
        let r = make_bool_addr(0);

        // 先初始化 a=5, b=10，然后比较
        let codes = vec![
            CodeWithSpan::new(
                IntermediateCode::assign(a, Operand::int(5)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::assign(b, Operand::int(10)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::binary(
                    IntermediateOperator::IntLess,
                    Operand::Address(a),
                    Operand::Address(b),
                    r,
                ),
                crate::diagnostics::Span::dummy(),
            ),
        ];

        let method = CompiledMethod {
            name: "test".into(),
            codes,
            local_count: 3,
        };

        let mut vm = VirtualMachine::new();
        vm.push_frame(method.local_count);
        vm.execute(&method).unwrap();
        assert_eq!(*vm.bool_stack.read(0), true);
    }

    #[test]
    fn test_execute_jump_if_false() {
        let cond = make_bool_addr(0);

        // cond = false, 如果 !cond 则跳转到 end
        let codes = vec![
            CodeWithSpan::new(
                IntermediateCode::assign(cond, Operand::boolean(false)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::jump_if_false(Operand::Address(cond), 3),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::assign(make_int_addr(1), Operand::int(1)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::nop(),
                crate::diagnostics::Span::dummy(),
            ),
        ];

        let method = CompiledMethod {
            name: "test".into(),
            codes,
            local_count: 2,
        };

        let mut vm = VirtualMachine::new();
        vm.push_frame(method.local_count);
        vm.execute(&method).unwrap();
        assert_eq!(vm.int_stack.len(), 2);
    }

    #[test]
    fn test_vm_arithmetic_sequence() {
        let x = make_int_addr(0);
        let y = make_int_addr(1);
        let tmp = make_int_addr(2);
        let result = make_int_addr(3);

        // result = x * 2 + y
        let codes = vec![
            CodeWithSpan::new(
                IntermediateCode::assign(x, Operand::int(3)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::assign(y, Operand::int(4)),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::binary(
                    IntermediateOperator::IntMul,
                    Operand::Address(x),
                    Operand::int(2),
                    tmp,
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::binary(
                    IntermediateOperator::IntAdd,
                    Operand::Address(tmp),
                    Operand::Address(y),
                    result,
                ),
                crate::diagnostics::Span::dummy(),
            ),
        ];

        let method = CompiledMethod {
            name: "compute".into(),
            codes,
            local_count: 4,
        };

        let mut vm = VirtualMachine::new();
        vm.push_frame(method.local_count);
        vm.execute(&method).unwrap();
        assert_eq!(*vm.int_stack.read(3), 10); // 3*2 + 4 = 10
    }

    #[test]
    fn test_invoke_instance_method() {
        use std::sync::Arc;
        use crate::class::RuntimeClass;
        use crate::declaration::ClassDeclaration;
        use crate::types::GorgeType;

        // 构造一个简单的 RuntimeMethod，返回 int 42
        let callee_method = CompiledMethod {
            name: "getValue".into(),
            codes: vec![
                CodeWithSpan::new(
                    IntermediateCode::assign(
                        Address::new(ValueType::Int, 0),
                        Operand::int(42),
                    ),
                    crate::diagnostics::Span::dummy(),
                ),
                CodeWithSpan::new(
                    IntermediateCode::return_value(ValueType::Int),
                    crate::diagnostics::Span::dummy(),
                ),
            ],
            local_count: 1,
        };

        let decl = ClassDeclaration {
            class_type: GorgeType::class("Widget", None),
            is_native: false,
            annotations: vec![],
            fields: vec![],
            methods: vec![],
            static_methods: vec![],
            constructors: vec![],
            injector_fields: vec![],
            super_class: None,
            super_interfaces: vec![],
            field_type_count: TypeCount::zero(),
            method_count: 1,
            static_method_count: 0,
            constructor_count: 0,
            injector_field_type_count: TypeCount::zero(),
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0,
            constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
        };
        let mut cls = RuntimeClass::new(decl, None);
        cls.register_method(0, callee_method);

        let cls_arc = Arc::new(cls);

        // 创建 VM 并注册类
        let mut vm = VirtualMachine::new();
        vm.register_runtime_class("Widget", cls_arc.clone());
        vm.register_class_field_counts("Widget", TypeCount::zero());

        // 创建对象并存入对象表
        let obj_id = vm.next_object_id;
        vm.next_object_id += 1;
        let obj = RuntimeObject::new_simple("Widget".into(), &TypeCount::zero());
        vm.objects.insert(obj_id, obj);
        vm.object_stack.push_frame(3);
        vm.int_stack.push_frame(3);
        vm.float_stack.push_frame(3);
        vm.bool_stack.push_frame(3);
        vm.string_stack.push_frame(3);

        // 调用 InvokeInstance(method_id=0)
        let result_addr = Address::new(ValueType::Int, 1);
        let invoke_code = IntermediateCode::new(
            IntermediateOperator::InvokeInstance(0),
            Operand::Address(Address::new(ValueType::Object, 0)), // target object 在 slot 0
            None,
            Some(result_addr),
        );

        vm.object_stack.write(0, obj_id); // this = 目标对象
        let _advance = vm.execute_one(&invoke_code).unwrap();

        // 返回值写入 result_addr
        assert_eq!(*vm.int_stack.read(1), 42);
    }

    #[test]
    fn test_invoke_constructor() {
        use std::sync::Arc;
        use crate::class::RuntimeClass;
        use crate::declaration::ClassDeclaration;
        use crate::types::GorgeType;

        // 构造方法：将参数 a 存入 this.counter (field 0)
        let ctor_codes = vec![
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::LoadIntParameter,
                    Operand::int(0),
                    None,
                    Some(Address::new(ValueType::Int, 0)),
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::SetIntField(0),
                    Operand::Address(Address::new(ValueType::Int, 0)),
                    None,
                    None,
                ),
                crate::diagnostics::Span::dummy(),
            ),
            CodeWithSpan::new(
                IntermediateCode::return_void(),
                crate::diagnostics::Span::dummy(),
            ),
        ];

        let ctor = CompiledMethod { name: "Widget".into(), codes: ctor_codes, local_count: 2 };
        let field_counts = TypeCount { int_count: 1, ..TypeCount::zero() };

        let decl = ClassDeclaration {
            class_type: GorgeType::class("Widget", None),
            is_native: false, annotations: vec![], fields: vec![],
            methods: vec![], static_methods: vec![],
            constructors: vec![], injector_fields: vec![],
            super_class: None, super_interfaces: vec![],
            field_type_count: field_counts.clone(),
            method_count: 0, static_method_count: 0,
            constructor_count: 1,
            injector_field_type_count: TypeCount::zero(),
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0, constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
        };
        let mut cls = RuntimeClass::new(decl, None);
        cls.register_constructor(0, ctor);

        let cls_arc = Arc::new(cls);

        let mut vm = VirtualMachine::new();
        vm.int_stack.push_frame(4);
        vm.float_stack.push_frame(4);
        vm.bool_stack.push_frame(4);
        vm.string_stack.push_frame(4);
        vm.object_stack.push_frame(4);

        vm.register_runtime_class("Widget", cls_arc.clone());
        vm.register_class_field_counts("Widget", field_counts);
        vm.set_current_class("Widget");

        // 设置构造参数
        vm.param_pool.set_int_param(0, 99);

        // InvokeConstructor(0), 1 个参数, result → object addr 1
        let result_obj = Address::new(ValueType::Object, 1);
        let invoke_ctor = IntermediateCode::new(
            IntermediateOperator::InvokeConstructor(0),
            Operand::int(1),
            None,
            Some(result_obj),
        );

        let _advance = vm.execute_one(&invoke_ctor).unwrap();

        // 结果放在 object_stack[1]
        let obj_id = *vm.object_stack.read(1);
        assert_ne!(obj_id, 0, "应创建非空对象: obj_id={}", obj_id);
        let obj = vm.objects.get(&obj_id).expect("对象表中应有新对象");
        assert_eq!(obj.get_int_field(0), 99, "counter 应为 99");
    }

    #[test]
    fn test_set_and_load_injector() {
        let mut vm = VirtualMachine::new();
        vm.object_stack.push_frame(4);

        // 创建一个 RuntimeObject 模拟注入器对象
        let inj_id = 42;
        let obj = RuntimeObject::new_simple("Injector".into(), &TypeCount::zero());
        vm.objects.insert(inj_id, obj);

        // SetInjector: 将对象 42 设为当前注入器
        let set_code = IntermediateCode::new(
            IntermediateOperator::SetInjector,
            Operand::Address(Address::new(ValueType::Object, 0)), // 读取 object_stack[0]
            None,
            None,
        );
        vm.object_stack.write(0, inj_id);
        let _ = vm.execute_one(&set_code).unwrap();
        assert_eq!(vm.current_injector, Some(42), "SetInjector 应设置注入器 ID");

        // LoadInjector: 加载注入器到结果地址
        let result_addr = Address::new(ValueType::Object, 2);
        let load_code = IntermediateCode::new(
            IntermediateOperator::LoadInjector,
            Operand::int(0),
            None,
            Some(result_addr),
        );
        let _ = vm.execute_one(&load_code).unwrap();
        assert_eq!(*vm.object_stack.read(2), 42, "LoadInjector 应加载注入器 ID");
    }

    #[test]
    fn test_injector_field_set_and_load() {
        use std::sync::Arc;
        use crate::class::RuntimeClass;
        use crate::declaration::ClassDeclaration;
        use crate::types::GorgeType;

        // 创建带注入器字段的类
        let field_counts = TypeCount { int_count: 1, ..TypeCount::zero() };
        let decl = ClassDeclaration {
            class_type: GorgeType::class("Scene", None),
            is_native: false, annotations: vec![], fields: vec![],
            methods: vec![], static_methods: vec![],
            constructors: vec![], injector_fields: vec![],
            super_class: None, super_interfaces: vec![],
            field_type_count: field_counts.clone(),
            method_count: 0, static_method_count: 0, constructor_count: 0,
            injector_field_type_count: TypeCount { int_count: 2, ..TypeCount::zero() },
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0, constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(), method_override_id: HashMap::new(),
        };
        let cls = Arc::new(RuntimeClass::new(decl, None));

        // 创建注入器，含 2 个 int 字段
        let inj = RuntimeInjector::new(Arc::new(cls.declaration().clone()));
        let inj_id = 100;
        let mut vm = VirtualMachine::new();
        vm.int_stack.push_frame(4);
        vm.object_stack.push_frame(4);

        // 将注入器存入 VM
        vm.injectors.insert(inj_id, inj);
        // 在 object_stack[0] 设为当前注入器
        vm.object_stack.write(0, inj_id);

        // SetIntInjectorField(0, 77)
        let set_code = IntermediateCode::new(
            IntermediateOperator::SetIntInjectorField(0),
            Operand::int(77),
            None, None,
        );
        let _ = vm.execute_one(&set_code).unwrap();

        // LoadIntInjectorField(0) → result int[1]
        let result = Address::new(ValueType::Int, 1);
        let load_code = IntermediateCode::new(
            IntermediateOperator::LoadIntInjectorField(0),
            Operand::int(0),
            None,
            Some(result),
        );
        let _ = vm.execute_one(&load_code).unwrap();
        assert_eq!(*vm.int_stack.read(1), 77, "注入器字段值应为 77");

        // 验证默认值标记：未设置的字段 1 应返回默认值
        let load_default = IntermediateCode::new(
            IntermediateOperator::LoadIntInjectorField(1),
            Operand::int(0),
            None,
            Some(Address::new(ValueType::Int, 2)),
        );
        let _ = vm.execute_one(&load_default).unwrap();
        assert_eq!(*vm.int_stack.read(2), 0, "未设置的注入器字段默认值应为 0");
    }

    #[test]
    fn test_static_field_set_and_load() {
        let mut vm = VirtualMachine::new();
        vm.int_stack.push_frame(4);
        vm.string_stack.push_frame(4);
        vm.set_current_class("Config");

        // 注册静态字段池
        let counts = TypeCount { int_count: 1, string_count: 1, ..TypeCount::zero() };
        vm.class_static_fields.insert("Config".into(), FixedFieldValuePool::new(&counts));

        // SetStaticIntField(0, 100)
        let set_int = IntermediateCode::new(
            IntermediateOperator::SetStaticIntField(0),
            Operand::int(100),
            None, None,
        );
        let _ = vm.execute_one(&set_int).unwrap();

        // LoadStaticIntField(0) → result
        let ri = Address::new(ValueType::Int, 1);
        let load_int = IntermediateCode::new(
            IntermediateOperator::LoadStaticIntField(0),
            Operand::int(0), None, Some(ri),
        );
        let _ = vm.execute_one(&load_int).unwrap();
        assert_eq!(*vm.int_stack.read(1), 100, "静态 int 字段应为 100");

        // SetStaticStringField(0, "hello")
        let set_str = IntermediateCode::new(
            IntermediateOperator::SetStaticStringField(0),
            Operand::string("hello"),
            None, None,
        );
        let _ = vm.execute_one(&set_str).unwrap();

        // LoadStaticStringField(0)
        let rs = Address::new(ValueType::String, 0);
        let load_str = IntermediateCode::new(
            IntermediateOperator::LoadStaticStringField(0),
            Operand::int(0), None, Some(rs),
        );
        let _ = vm.execute_one(&load_str).unwrap();
        assert_eq!(vm.string_stack.read(0), "hello");
    }

    // ==================== Phase A: Native 互操作验收测试 ====================

    /// 手写的最小 native 类，用于验收 VM 的 native 分派链路
    ///
    /// 提供：
    /// - 0 号静态方法 `addOne(int) -> int`：参数 +1
    /// - 0 号构造方法：创建一个带 1 个 float 字段的对象，字段值取自参数池 float[0]
    /// - 0 号实例方法 `getValue() -> float`：读取对象 float 字段 0
    #[derive(Debug)]
    struct DemoNativeClass {
        name: String,
        counts: TypeCount,
    }

    impl crate::native::NativeClass for DemoNativeClass {
        fn full_name(&self) -> &str {
            &self.name
        }

        fn field_type_count(&self) -> &TypeCount {
            &self.counts
        }

        fn invoke_native_method(
            &self,
            ctx: &mut crate::native::NativeContext,
            obj_id: usize,
            method_id: usize,
        ) {
            // 0 号实例方法：getValue() -> float
            if method_id == 0 {
                let v = ctx.get_object_float_field(obj_id, 0);
                ctx.set_float_return(v);
            }
        }

        fn invoke_native_static(
            &self,
            ctx: &mut crate::native::NativeContext,
            method_id: usize,
        ) {
            // 0 号静态方法：addOne(int) -> int
            if method_id == 0 {
                let a = ctx.get_int_param(0);
                ctx.set_int_return(a + 1);
            }
        }

        fn do_construct_native(
            &self,
            ctx: &mut crate::native::NativeContext,
            target: Option<usize>,
            _ctor_id: usize,
        ) -> usize {
            // 从参数池读取 float[0] 作为字段初值
            let init = ctx.get_float_param(0);
            let id = match target {
                Some(id) => id,
                None => {
                    let obj = RuntimeObject::new_simple(self.name.clone(), &self.counts);
                    ctx.register_object(obj)
                }
            };
            ctx.set_object_float_field(id, 0, init);
            id
        }
    }

    /// 构造一个带单个 float 字段的 demo native 类
    fn make_demo_native() -> std::sync::Arc<DemoNativeClass> {
        std::sync::Arc::new(DemoNativeClass {
            name: "Demo.Native".into(),
            counts: TypeCount { float_count: 1, ..TypeCount::zero() },
        })
    }

    #[test]
    fn test_native_invoke_static_via_ir() {
        // 验收核心场景：一段 IR 调用 native 静态方法并取回返回值
        let mut vm = VirtualMachine::new();
        vm.register_native_class("Demo.Native", make_demo_native());
        vm.set_current_class("Demo.Native");
        vm.push_frame(4);

        // 准备参数：param[0] = 41（int）
        let arg = Address::new(ValueType::Int, 0);
        vm.int_stack.write(0, 41);
        let set_param = IntermediateCode::new(
            IntermediateOperator::SetIntParameter,
            Operand::Address(arg),
            None,
            Some(Address::new(ValueType::Int, 0)),
        );
        vm.execute_one(&set_param).unwrap();

        // InvokeStatic(0)，结果写入 int 地址 1
        let result = Address::new(ValueType::Int, 1);
        let invoke = IntermediateCode::new(
            IntermediateOperator::InvokeStatic(0),
            Operand::int(1),
            None,
            Some(result),
        );
        vm.execute_one(&invoke).unwrap();

        // native addOne(41) = 42
        assert_eq!(*vm.int_stack.read(1), 42);
    }

    #[test]
    fn test_native_construct_and_instance_method_via_ir() {
        // 验收：native 构造 + native 实例方法读字段
        let mut vm = VirtualMachine::new();
        vm.register_native_class("Demo.Native", make_demo_native());
        vm.set_current_class("Demo.Native");
        vm.push_frame(4);

        // 准备构造参数：float param[0] = 3.5
        vm.float_stack.write(0, 3.5);
        let set_fp = IntermediateCode::new(
            IntermediateOperator::SetFloatParameter,
            Operand::Address(Address::new(ValueType::Float, 0)),
            None,
            Some(Address::new(ValueType::Int, 0)),
        );
        vm.execute_one(&set_fp).unwrap();

        // InvokeConstructor(0)，对象 ID 写入 object 地址 1
        let obj_addr = Address::new(ValueType::Object, 1);
        let construct = IntermediateCode::new(
            IntermediateOperator::InvokeConstructor(0),
            Operand::int(1),
            None,
            Some(obj_addr),
        );
        vm.execute_one(&construct).unwrap();
        let obj_id = *vm.object_stack.read(1);
        assert!(obj_id != 0, "构造应返回有效对象 ID");
        assert!(vm.objects.contains_key(&obj_id), "对象应进入对象表");

        // 调用实例方法 getValue()：InvokeInstance(0)，left = 目标对象 ID
        let ret = Address::new(ValueType::Float, 2);
        let invoke = IntermediateCode::new(
            IntermediateOperator::InvokeInstance(0),
            Operand::Address(obj_addr),
            None,
            Some(ret),
        );
        vm.execute_one(&invoke).unwrap();

        // 字段初值 3.5 应被读回
        assert_eq!(*vm.float_stack.read(2), 3.5);
    }

    #[test]
    fn test_native_compiled_bidirectional_link() {
        // 验收 A5：native 对象与编译对象的双向引用与 RealObject 解析
        let mut vm = VirtualMachine::new();

        // 手工放入两个对象：一个编译对象、一个 native 子对象
        let compiled_id = 1usize;
        let native_id = 2usize;
        vm.objects.insert(
            compiled_id,
            RuntimeObject::new_simple("Compiled.Sub".into(), &TypeCount::zero()),
        );
        vm.objects.insert(
            native_id,
            RuntimeObject::new_simple("Demo.Native".into(), &TypeCount::zero()),
        );

        vm.link_native_and_compiled(compiled_id, native_id);

        // 双向引用建立
        assert_eq!(vm.objects.get(&compiled_id).unwrap().native_object_id, Some(native_id));
        assert_eq!(vm.objects.get(&native_id).unwrap().outer_compiled_id, Some(compiled_id));

        // native 子对象的真实对象应解析为外层编译对象
        assert_eq!(vm.resolve_real_object_id(native_id), compiled_id);
        // 普通对象的真实对象是自身
        assert_eq!(vm.resolve_real_object_id(compiled_id), compiled_id);
    }
}
