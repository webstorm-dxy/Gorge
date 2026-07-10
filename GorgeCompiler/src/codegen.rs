#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;

use gorge_core::diagnostics::{Diagnostics, Span};
use gorge_core::ir::*;
use gorge_core::bytecode::DelegateImpl;

use crate::ast::*;
use crate::symbol::*;

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
    /// 类字段名 → (偏移, 值类型) 映射
    field_info: HashMap<String, (usize, ValueType)>,
    /// 参数索引计数器（每次方法调用前重置）
    param_index: usize,
}

impl<'a> CodeGenerator<'a> {
    /// 创建新的代码生成器实例
    pub fn new(
        symbol_table: &'a SymbolTable,
        diagnostics: &'a mut Diagnostics,
        delegate_impls: &'a mut Vec<DelegateImpl>,
    ) -> Self {
        let mut next_local = HashMap::new();
        next_local.insert(ValueType::Int, 0);
        next_local.insert(ValueType::Float, 0);
        next_local.insert(ValueType::Bool, 0);
        next_local.insert(ValueType::String, 0);
        next_local.insert(ValueType::Object, 0);

        Self {
            symbol_table,
            diagnostics,
            delegate_impls,
            codes: Vec::new(),
            local_vars: HashMap::new(),
            param_vars: HashMap::new(),
            next_local,
            delegate_vars: HashMap::new(),
            field_info: HashMap::new(),
            param_index: 0,
        }
    }

    /// 为方法参数注册地址
    pub fn register_parameters(&mut self, params: &[(String, ValueType)]) {
        for (name, vt) in params {
            let addr = self.alloc_local(*vt);
            self.param_vars.insert(name.clone(), addr);
            self.local_vars.insert(name.clone(), addr);
        }
    }

    /// 获取方法体中的所有 IR 指令
    pub fn into_codes(self) -> Vec<CodeWithSpan> {
        self.codes
    }

    // ==================== 临时变量 / 局部变量管理 ====================

    /// 分配一个临时变量地址（与局部变量共享计数器，确保不冲突）
    fn alloc_temp(&mut self, value_type: ValueType) -> Address {
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
    fn emit(&mut self, code: IntermediateCode, span: Span) {
        self.codes.push(CodeWithSpan::new(code, span));
    }

    /// 设置类上下文，填充字段名→(偏移,类型) 映射（包含继承字段）
    pub fn set_class_context(&mut self, class_name: &str) {
        self.field_info.clear();
        let scope_id = self.symbol_table.global_scope;
        if let Some(mut class_id) = self.symbol_table.lookup_class(scope_id, class_name) {
            loop {
                let class_info = self.symbol_table.classes.get(class_id.0);
                for &field_id in &class_info.fields {
                    let fi = self.symbol_table.fields.get(field_id.0);
                    let vt = Self::type_to_value_type(&fi.field_type);
                    let offset = fi.offset.unwrap_or(0);
                    self.field_info.entry(fi.name.clone()).or_insert((offset, vt));
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
    fn set_field_op(value_type: ValueType, field_index: usize) -> IntermediateOperator {
        match value_type {
            ValueType::Int => IntermediateOperator::SetIntField(field_index),
            ValueType::Float => IntermediateOperator::SetFloatField(field_index),
            ValueType::Bool => IntermediateOperator::SetBoolField(field_index),
            ValueType::String => IntermediateOperator::SetStringField(field_index),
            ValueType::Object => IntermediateOperator::SetObjectField(field_index),
        }
    }

    // ==================== 类型推导 ====================

    /// 从 TypeInfo 推导 ValueType
    fn type_to_value_type(type_info: &TypeInfo) -> ValueType {
        match type_info {
            TypeInfo::Int => ValueType::Int,
            TypeInfo::Float => ValueType::Float,
            TypeInfo::Bool => ValueType::Bool,
            TypeInfo::String => ValueType::String,
            _ => ValueType::Object,
        }
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
                        // 回退：在类字段中查找
                        if let Some(&(offset, vt)) = self.field_info.get(name) {
                            let temp = self.alloc_temp(vt);
                            let load_op = Self::load_field_op(vt, offset);
                            self.emit(
                                IntermediateCode::new(load_op, Operand::int(0), None, Some(temp)),
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
            Expression::StaticMethodCall { class_name: _, method, arguments, span } => {
                // 数组构造占位符 `new_array`
                if method == "new_array" {
                    for arg in arguments {
                        let _ = self.generate_expression(arg);
                    }
                    let temp = self.alloc_temp(ValueType::Object);
                    self.emit(IntermediateCode::nop(), *span);
                    return Operand::Address(temp);
                }
                self.generate_delegate_call(method, arguments, *span)
            }
            Expression::New { class_type, arguments, span } => {
                self.generate_new(class_type, arguments, *span)
            }
            Expression::Conditional { condition, then_branch, else_branch, span } => {
                self.generate_conditional(condition, then_branch, else_branch.as_deref(), *span)
            }
            Expression::This(_span) => {
                // this 指向对象自身，地址 0 的 Object
                Operand::Address(Address::new(ValueType::Object, 0))
            }
            Expression::Null(_span) => {
                Operand::Address(Address::new(ValueType::Object, 0))
            }
            Expression::InjectorObject { fields, span } => {
                // 注入器对象：为每个字段生成值表达式
                for (_key, value) in fields {
                    let _val = self.generate_expression(value);
                }
                let temp = self.alloc_temp(ValueType::Object);
                self.emit(
                    IntermediateCode::new(
                        IntermediateOperator::DoConstruct(0),
                        Operand::Address(Address::new(ValueType::Object, 0)),
                        None,
                        Some(temp),
                    ),
                    *span,
                );
                Operand::Address(temp)
            }
            Expression::InjectorArray { elements, span } => {
                for elem in elements {
                    let _val = self.generate_expression(elem);
                }
                let temp = self.alloc_temp(ValueType::Object);
                self.emit(IntermediateCode::nop(), *span);
                Operand::Address(temp)
            }
            Expression::InjectorFieldRef(_name, span) => {
                let temp = self.alloc_temp(ValueType::Object);
                self.emit(IntermediateCode::nop(), *span);
                Operand::Address(temp)
            }
            Expression::Lambda { parameters, body, span } => {
                // 1. 创建子生成器编译 Lambda body
                let mut sub_diags = Diagnostics::new();
                let mut dummy_delegates = Vec::new();
                let mut sub_cg = CodeGenerator::new(self.symbol_table, &mut sub_diags, &mut dummy_delegates);

                for param in parameters {
                    let vt = Self::type_ref_to_value_type(&param.param_type);
                    sub_cg.declare_local(&param.name, vt);
                }

                match body {
                    LambdaBody::Expression(expr) => {
                        let r = sub_cg.generate_expression(&expr);
                        let vt = Self::operand_value_type(&r);
                        let ret_addr = Address::new(vt, 0);
                        sub_cg.emit(IntermediateCode::assign(ret_addr, r), *span);
                        sub_cg.emit(IntermediateCode::return_value(vt), *span);
                    }
                    LambdaBody::Block(stmts) => {
                        for s in stmts {
                            sub_cg.generate_statement(&s);
                        }
                    }
                }

                // 2. 自由变量分析
                let param_names: HashSet<String> = parameters.iter()
                    .map(|p| p.name.clone()).collect();
                let free_vars = Self::analyze_free_vars_lambda_body(body, &param_names);

                // 3. 获取 body IR（释放对 dummy_delegates 的借出）
                let body_ir = sub_cg.into_codes();

                // 4. 将内部嵌套委托转移到父级，再注册本委托
                self.delegate_impls.append(&mut dummy_delegates);
                let delegate_idx = self.delegate_impls.len();
                let param_types: Vec<ValueType> = parameters.iter()
                    .map(|p| Self::type_ref_to_value_type(&p.param_type)).collect();

                self.delegate_impls.push(DelegateImpl {
                    param_types,
                    return_type: ValueType::Int,
                    body_ir,
                    captured_var_names: free_vars,
                    outer_value_count: 0,
                });

                // 4. 生成 ConstructDelegate(idx)
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
            // 暂未实现的其他表达式类型
            _ => {
                let temp = self.alloc_temp(ValueType::Int);
                self.emit(
                    IntermediateCode::assign(temp, Operand::int(0)),
                    expr.span(),
                );
                Operand::Address(temp)
            }
        }
    }

    /// 生成二元运算代码
    fn generate_binary(
        &mut self,
        left: &Expression,
        op: BinaryOp,
        right: &Expression,
        span: Span,
    ) -> Operand {
        let left_op = self.generate_expression(left);
        let right_op = self.generate_expression(right);
        let vt = Self::operand_value_type(&left_op);
        let result = self.alloc_temp(vt);

        let ir_op = match op {
            BinaryOp::Add => match vt {
                ValueType::Int => IntermediateOperator::IntAdd,
                ValueType::Float => IntermediateOperator::FloatAdd,
                _ => IntermediateOperator::IntAdd,
            },
            BinaryOp::Subtract => match vt {
                ValueType::Int => IntermediateOperator::IntSub,
                ValueType::Float => IntermediateOperator::FloatSub,
                _ => IntermediateOperator::IntSub,
            },
            BinaryOp::Multiply => match vt {
                ValueType::Int => IntermediateOperator::IntMul,
                ValueType::Float => IntermediateOperator::FloatMul,
                _ => IntermediateOperator::IntMul,
            },
            BinaryOp::Divide => match vt {
                ValueType::Int => IntermediateOperator::IntDiv,
                ValueType::Float => IntermediateOperator::FloatDiv,
                _ => IntermediateOperator::IntDiv,
            },
            BinaryOp::Modulo => IntermediateOperator::IntMod,
            BinaryOp::Less => match vt {
                ValueType::Int => IntermediateOperator::IntLess,
                ValueType::Float => IntermediateOperator::FloatLess,
                _ => IntermediateOperator::IntLess,
            },
            BinaryOp::LessEqual => match vt {
                ValueType::Int => IntermediateOperator::IntLessEqual,
                ValueType::Float => IntermediateOperator::FloatLessEqual,
                _ => IntermediateOperator::IntLessEqual,
            },
            BinaryOp::Greater => match vt {
                ValueType::Int => IntermediateOperator::IntGreater,
                ValueType::Float => IntermediateOperator::FloatGreater,
                _ => IntermediateOperator::IntGreater,
            },
            BinaryOp::GreaterEqual => match vt {
                ValueType::Int => IntermediateOperator::IntGreaterEqual,
                ValueType::Float => IntermediateOperator::FloatGreaterEqual,
                _ => IntermediateOperator::IntGreaterEqual,
            },
            BinaryOp::Equal => match vt {
                ValueType::Int => IntermediateOperator::IntEqual,
                ValueType::Float => IntermediateOperator::FloatEqual,
                ValueType::Bool => IntermediateOperator::BoolEqual,
                ValueType::String => IntermediateOperator::StringEqual,
                ValueType::Object => IntermediateOperator::ObjectEqual,
            },
            BinaryOp::NotEqual => match vt {
                ValueType::Int => IntermediateOperator::IntNotEqual,
                ValueType::Float => IntermediateOperator::FloatNotEqual,
                ValueType::Bool => IntermediateOperator::BoolNotEqual,
                ValueType::String => IntermediateOperator::StringNotEqual,
                ValueType::Object => IntermediateOperator::ObjectNotEqual,
            },
            BinaryOp::LogicAnd => IntermediateOperator::LogicalAnd,
            BinaryOp::LogicOr => IntermediateOperator::LogicalOr,
        };

        self.emit(
            IntermediateCode::binary(ir_op, left_op, right_op, result),
            span,
        );
        Operand::Address(result)
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
                    ValueType::Int => IntermediateOperator::IntSub,
                    ValueType::Float => IntermediateOperator::FloatSub,
                    _ => IntermediateOperator::IntSub,
                };
                self.emit(
                    IntermediateCode::binary(
                        ir_op,
                        Operand::int(0),
                        inner,
                        result,
                    ),
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
            _ => {
                // 前置/后置自增自减暂未实现
                self.emit(IntermediateCode::assign(result, inner), span);
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
                        // 回退：检查是否为实例字段
                        if let Some(&(offset, field_vt)) = self.field_info.get(name) {
                            let set_op = Self::set_field_op(field_vt, offset);
                            self.emit(IntermediateCode::new(set_op, result_op.clone(), None, None), span);
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
                // this.field = expr → 生成 SetField
                if matches!(**object, Expression::This(_)) {
                    if let Some(&(offset, vt)) = self.field_info.get(field) {
                        let set_op = Self::set_field_op(vt, offset);
                        self.emit(IntermediateCode::new(set_op, result_op.clone(), None, None), span);
                        return result_op;
                    }
                }
                // 其他对象字段赋值，暂为占位
                let _obj_op = self.generate_expression(object);
                let temp = self.alloc_temp(ValueType::Object);
                self.emit(IntermediateCode::assign(temp, result_op), span);
                Operand::Address(temp)
            }
            AssignmentTarget::ArrayElement { array: _, index: _, span: _ } => {
                let temp = self.alloc_temp(ValueType::Object);
                self.emit(IntermediateCode::assign(temp, result_op), span);
                Operand::Address(temp)
            }
            AssignmentTarget::InjectorField { object, field: _, span: _ } => {
                let _obj_op = self.generate_expression(object);
                let temp = self.alloc_temp(ValueType::Object);
                self.emit(IntermediateCode::assign(temp, result_op), span);
                Operand::Address(temp)
            }
        }
    }

    /// 生成成员访问代码
    fn generate_member_access(
        &mut self,
        object: &Expression,
        member: &str,
    ) -> Operand {
        // this.field → 生成 LoadField
        if matches!(object, Expression::This(_)) {
            if let Some(&(offset, vt)) = self.field_info.get(member) {
                let temp = self.alloc_temp(vt);
                let load_op = Self::load_field_op(vt, offset);
                self.emit(
                    IntermediateCode::new(load_op, Operand::int(0), None, Some(temp)),
                    object.span(),
                );
                return Operand::Address(temp);
            }
        }
        // 其他对象字段读取，暂为占位
        let _obj_op = self.generate_expression(object);
        let temp = self.alloc_temp(ValueType::Int);
        self.emit(
            IntermediateCode::assign(temp, Operand::int(0)),
            object.span(),
        );
        Operand::Address(temp)
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
            let scope = self.symbol_table.global_scope;
            if let Some(class_id) = self.symbol_table.lookup_class(scope, class_name) {
                let class_info = self.symbol_table.classes.get(class_id.0);
                // 在类方法中查找匹配的静态方法
                for (i, &method_id) in class_info.methods.iter().enumerate() {
                    let mi = self.symbol_table.methods.get(method_id.0);
                    if mi.name == method && mi.is_static {
                        // 设置调用参数（分配参数索引）
                        self.param_index = 0;
                        for arg in arguments {
                            let arg_op = self.generate_expression(arg);
                            let arg_vt = Self::operand_value_type(&arg_op);
                            let param_addr = Address::new(ValueType::Int, self.param_index);
                            self.param_index += 1;
                            match arg_vt {
                                ValueType::Int => {
                                    self.emit(IntermediateCode::new(
                                        IntermediateOperator::SetIntParameter, arg_op, None, Some(param_addr)), span);
                                }
                                ValueType::Float => {
                                    self.emit(IntermediateCode::new(
                                        IntermediateOperator::SetFloatParameter, arg_op, None, Some(param_addr)), span);
                                }
                                ValueType::Bool => {
                                    self.emit(IntermediateCode::new(
                                        IntermediateOperator::SetBoolParameter, arg_op, None, Some(param_addr)), span);
                                }
                                ValueType::String => {
                                    self.emit(IntermediateCode::new(
                                        IntermediateOperator::SetStringParameter, arg_op, None, Some(param_addr)), span);
                                }
                                ValueType::Object => {
                                    self.emit(IntermediateCode::new(
                                        IntermediateOperator::SetObjectParameter, arg_op, None, Some(param_addr)), span);
                                }
                            }
                        }
                        // 生成 InvokeStatic（left 操作数存参数计数）
                        let idx = i;
                        let result = self.alloc_temp(Self::type_to_value_type(&mi.return_type));
                        self.emit(
                            IntermediateCode::new(
                                IntermediateOperator::InvokeStatic(idx),
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

        // 原有的委托/实例调用逻辑
        let recv_op = self.generate_expression(receiver);
        let delegate_idx = match receiver {
            Expression::Identifier(name, _) => self.delegate_vars.get(name).copied(),
            _ => None,
        };

        // 非委托调用：尝试查找实例方法
        if delegate_idx.is_none() {
            if let Expression::Identifier(class_name, _) = receiver {
                let scope = self.symbol_table.global_scope;
                if let Some(class_id) = self.symbol_table.lookup_class(scope, class_name) {
                    let class_info = self.symbol_table.classes.get(class_id.0);
                    for (i, &method_id) in class_info.methods.iter().enumerate() {
                        let mi = self.symbol_table.methods.get(method_id.0);
                        if mi.name == method && !mi.is_static {
                            self.param_index = 0;
                            for arg in arguments {
                                let arg_op = self.generate_expression(arg);
                                let arg_vt = Self::operand_value_type(&arg_op);
                                let param_addr = Address::new(ValueType::Int, self.param_index);
                                self.param_index += 1;
                                let set_param = match arg_vt {
                                    ValueType::Int => IntermediateOperator::SetIntParameter,
                                    ValueType::Float => IntermediateOperator::SetFloatParameter,
                                    ValueType::Bool => IntermediateOperator::SetBoolParameter,
                                    ValueType::String => IntermediateOperator::SetStringParameter,
                                    ValueType::Object => IntermediateOperator::SetObjectParameter,
                                };
                                self.emit(IntermediateCode::new(set_param, arg_op, None, Some(param_addr)), span);
                            }
                            let result = self.alloc_temp(Self::type_to_value_type(&mi.return_type));
                            self.emit(
                                IntermediateCode::new(
                                    IntermediateOperator::InvokeInstance(i),
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
            // 无法静态解析（如变量调用），发出 InvokeInstance(0) 让 VM 运行时分派
            self.param_index = 0;
            for arg in arguments {
                let arg_op = self.generate_expression(arg);
                let arg_vt = Self::operand_value_type(&arg_op);
                let param_addr = Address::new(ValueType::Int, self.param_index);
                self.param_index += 1;
                let set_param = match arg_vt {
                    ValueType::Int => IntermediateOperator::SetIntParameter,
                    ValueType::Float => IntermediateOperator::SetFloatParameter,
                    ValueType::Bool => IntermediateOperator::SetBoolParameter,
                    ValueType::String => IntermediateOperator::SetStringParameter,
                    ValueType::Object => IntermediateOperator::SetObjectParameter,
                };
                self.emit(IntermediateCode::new(set_param, arg_op, None, Some(param_addr)), span);
            }
            // 生成 receiver 的代码并将地址作为 InvokeInstance 的 left（目标对象引用）
            let recv_addr_op = self.generate_expression(receiver);
            let result = self.alloc_temp(ValueType::Int);
            self.emit(
                IntermediateCode::new(
                    IntermediateOperator::InvokeInstance(0),
                    recv_addr_op,
                    None,
                    Some(result),
                ),
                span,
            );
            return Operand::Address(result);
        }

        // 委托调用
        for (_i, arg) in arguments.iter().enumerate() {
            let arg_op = self.generate_expression(arg);
            let arg_vt = Self::operand_value_type(&arg_op);
            match arg_vt {
                ValueType::Int => {
                    self.emit(IntermediateCode::new(
                        IntermediateOperator::SetIntParameter, arg_op, None, None), span);
                }
                ValueType::Float => {
                    self.emit(IntermediateCode::new(
                        IntermediateOperator::SetFloatParameter, arg_op, None, None), span);
                }
                _ => {}
            }
        }

        let result = self.alloc_temp(ValueType::Int);
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
                self.diagnostics.emit_error(span, format!("未定义的变量 `{}`", var_name));
                return Operand::Address(self.alloc_temp(ValueType::Int));
            }
        };

        let delegate_idx = self.delegate_vars.get(var_name).copied();

        self.param_index = 0;
        for (_i, arg) in arguments.iter().enumerate() {
            let arg_op = self.generate_expression(arg);
            let arg_vt = Self::operand_value_type(&arg_op);
            let param_addr = Address::new(ValueType::Int, self.param_index);
            self.param_index += 1;
            match arg_vt {
                ValueType::Int => {
                    self.emit(IntermediateCode::new(
                        IntermediateOperator::SetIntParameter, arg_op, None, Some(param_addr)), span);
                }
                ValueType::Float => {
                    self.emit(IntermediateCode::new(
                        IntermediateOperator::SetFloatParameter, arg_op, None, Some(param_addr)), span);
                }
                ValueType::Bool => {
                    self.emit(IntermediateCode::new(
                        IntermediateOperator::SetBoolParameter, arg_op, None, Some(param_addr)), span);
                }
                ValueType::String => {
                    self.emit(IntermediateCode::new(
                        IntermediateOperator::SetStringParameter, arg_op, None, Some(param_addr)), span);
                }
                ValueType::Object => {
                    self.emit(IntermediateCode::new(
                        IntermediateOperator::SetObjectParameter, arg_op, None, Some(param_addr)), span);
                }
            }
        }

        let result = self.alloc_temp(ValueType::Int);
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

    /// 生成 new 表达式代码
    fn generate_new(
        &mut self,
        _class_type: &TypeRef,
        arguments: &[Expression],
        span: Span,
    ) -> Operand {
        // 设置构造参数
        self.param_index = 0;
        for arg in arguments {
            let arg_op = self.generate_expression(arg);
            let arg_vt = Self::operand_value_type(&arg_op);
            let param_addr = Address::new(ValueType::Int, self.param_index);
            self.param_index += 1;
            let set_param = match arg_vt {
                ValueType::Int => IntermediateOperator::SetIntParameter,
                ValueType::Float => IntermediateOperator::SetFloatParameter,
                ValueType::Bool => IntermediateOperator::SetBoolParameter,
                ValueType::String => IntermediateOperator::SetStringParameter,
                ValueType::Object => IntermediateOperator::SetObjectParameter,
            };
            self.emit(
                IntermediateCode::new(set_param, arg_op, None, Some(param_addr)),
                span,
            );
        }
        // 调用构造方法
        let temp = self.alloc_temp(ValueType::Object);
        self.emit(
            IntermediateCode::new(
                IntermediateOperator::InvokeConstructor(0),
                Operand::int(arguments.len() as i64),
                None,
                Some(temp),
            ),
            span,
        );
        Operand::Address(temp)
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
            _ => {}
        }
    }

    // ==================== 语句代码生成 ====================

    /// 为语句生成代码
    pub fn generate_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Expression(expr, _span) => {
                let _ = self.generate_expression(expr);
            }
            Statement::VariableDeclaration { var_type: _, name, initializer, span } => {
                let _vt = if let Some(init) = initializer {
                    let prev_count = self.delegate_impls.len();
                    let result = self.generate_expression(init);
                    let vt = Self::operand_value_type(&result);
                    let addr = self.declare_local(name, vt);
                    self.emit(IntermediateCode::assign(addr, result), *span);
                    // 如果初始化表达式生成了新委托，记录变量映射
                    if self.delegate_impls.len() > prev_count {
                        let delegate_idx = self.delegate_impls.len() - 1;
                        self.delegate_vars.insert(name.clone(), delegate_idx);
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

                let loop_start = self.codes.len();

                let mut jump_to_end_index = None;
                if let Some(cond) = condition {
                    let cond_op = self.generate_expression(cond);
                    jump_to_end_index = Some(self.codes.len());
                    self.emit(IntermediateCode::jump_if_false(cond_op, 0), cond.span());
                }

                self.generate_statement(body);

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
            }
            Statement::DoWhile { body, condition, span } => {
                let loop_start = self.codes.len();
                self.generate_statement(body);
                let cond_op = self.generate_expression(condition);
                self.emit(IntermediateCode::jump_if_true(cond_op, loop_start), *span);
            }
            Statement::Switch { expression, cases, default_body, span } => {
                self.generate_switch(expression, cases, default_body.as_deref(), *span);
            }
            Statement::Break { span, .. } | Statement::Continue { span, .. } => {
                // break/continue 需要记录循环上下文才能正确跳转
                // 暂时生成 Nop
                self.emit(IntermediateCode::nop(), *span);
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

        self.generate_statement(then_branch);

        if else_branch.is_some() {
            let jump_to_end_index = self.codes.len();
            self.emit(IntermediateCode::jump(0), span);

            let else_start = self.codes.len();
            if let Some(code) = self.codes.get_mut(jump_to_else_index) {
                code.code.operator = IntermediateOperator::JumpIfFalse(else_start);
            }

            self.generate_statement(else_branch.unwrap());

            let end = self.codes.len();
            if let Some(code) = self.codes.get_mut(jump_to_end_index) {
                code.code.operator = IntermediateOperator::Jump(end);
            }
        } else {
            let end = self.codes.len();
            if let Some(code) = self.codes.get_mut(jump_to_else_index) {
                code.code.operator = IntermediateOperator::JumpIfFalse(end);
            }
        }
    }

    /// 生成 while 语句代码
    fn generate_while_statement(
        &mut self,
        condition: &Expression,
        body: &Statement,
        span: Span,
    ) {
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

        // 为每个 case 生成比较+条件跳转
        let mut case_jumps: Vec<usize> = Vec::new(); // 记录每个 case body 的跳转位置
        let mut case_bodies: Vec<usize> = Vec::new(); // 每个 case body 的起始位置

        for case in cases {
            for value in &case.values {
                let val_op = self.generate_expression(value);
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
    }

    /// 获取已使用局部变量的总数（用于栈帧大小计算）
    pub fn total_locals(&self) -> usize {
        self.next_local.values().sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::SymbolTable;

    fn dummy_span() -> Span {
        Span::new(0, 1, 1, 1, 0)
    }

    fn make_codegen<'a>(
        st: &'a SymbolTable,
        diags: &'a mut Diagnostics,
        delegates: &'a mut Vec<DelegateImpl>,
    ) -> CodeGenerator<'a> {
        CodeGenerator::new(st, diags, delegates)
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
}
