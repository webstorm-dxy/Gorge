#![allow(dead_code)]

use crate::diagnostics::Span;

/// 值类型
///
/// 虚拟机为每种值类型维护独立的栈，因此操作码也按值类型区分。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueType {
    Int,
    Float,
    Bool,
    String,
    Object,
}

/// 栈变量地址
///
/// 索引指向对应类型栈中的位置。局部变量和临时变量都通过 Address 访问。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address {
    pub value_type: ValueType,
    pub index: usize,
}

impl Address {
    pub fn new(value_type: ValueType, index: usize) -> Self {
        Self { value_type, index }
    }
}

/// 立即数（编译时常量）
#[derive(Debug, Clone)]
pub enum ImmediateValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

/// 操作数
///
/// 可以是地址（栈变量）或立即数（编译时常量）。
#[derive(Debug, Clone)]
pub enum Operand {
    Address(Address),
    Immediate(ImmediateValue),
}

impl Operand {
    /// 从地址创建操作数
    pub fn addr(addr: Address) -> Self {
        Operand::Address(addr)
    }

    /// 从整数立即数创建操作数
    pub fn int(v: i64) -> Self {
        Operand::Immediate(ImmediateValue::Int(v))
    }

    /// 从浮点数立即数创建操作数
    pub fn float(v: f64) -> Self {
        Operand::Immediate(ImmediateValue::Float(v))
    }

    /// 从布尔立即数创建操作数
    pub fn boolean(v: bool) -> Self {
        Operand::Immediate(ImmediateValue::Bool(v))
    }

    /// 从字符串立即数创建操作数
    pub fn string(v: impl Into<String>) -> Self {
        Operand::Immediate(ImmediateValue::String(v.into()))
    }

    /// 尝试获取操作数中的地址引用
    pub fn as_address(&self) -> Option<&Address> {
        match self {
            Operand::Address(addr) => Some(addr),
            _ => None,
        }
    }

    /// 尝试获取操作数中的整数立即数
    pub fn as_immediate_int(&self) -> Option<i64> {
        match self {
            Operand::Immediate(ImmediateValue::Int(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn value_type(&self) -> ValueType {
        match self {
            Operand::Address(addr) => addr.value_type,
            Operand::Immediate(val) => match val {
                ImmediateValue::Int(_) => ValueType::Int,
                ImmediateValue::Float(_) => ValueType::Float,
                ImmediateValue::Bool(_) => ValueType::Bool,
                ImmediateValue::String(_) => ValueType::String,
            },
        }
    }
}

/// 三地址码操作符
///
/// 参考 C# 版本的 IntermediateOperator 设计，每种类型组合有独立操作码。
#[derive(Debug, Clone)]
pub enum IntermediateOperator {
    // === 本地变量赋值 ===
    IntAssign,
    FloatAssign,
    BoolAssign,
    StringAssign,
    ObjectAssign,

    // === 对象字段读取 ===
    LoadIntField(usize),     // 字段索引
    LoadFloatField(usize),
    LoadBoolField(usize),
    LoadStringField(usize),
    LoadObjectField(usize),

    // === 对象字段写入 ===
    SetIntField(usize),
    SetFloatField(usize),
    SetBoolField(usize),
    SetStringField(usize),
    SetObjectField(usize),

    // === 注入器字段读取 ===
    LoadIntInjectorField(usize),
    LoadFloatInjectorField(usize),
    LoadBoolInjectorField(usize),
    LoadStringInjectorField(usize),
    LoadObjectInjectorField(usize),

    // === 注入器字段写入 ===
    SetIntInjectorField(usize),
    SetFloatInjectorField(usize),
    SetBoolInjectorField(usize),
    SetStringInjectorField(usize),
    SetObjectInjectorField(usize),

    // === 静态字段读取 ===
    LoadStaticIntField(usize),
    LoadStaticFloatField(usize),
    LoadStaticBoolField(usize),
    LoadStaticStringField(usize),
    LoadStaticObjectField(usize),

    // === 静态字段写入 ===
    SetStaticIntField(usize),
    SetStaticFloatField(usize),
    SetStaticBoolField(usize),
    SetStaticStringField(usize),
    SetStaticObjectField(usize),

    // === this 加载 ===
    LoadThis,

    // === 参数设置 ===
    SetIntParameter,
    SetFloatParameter,
    SetBoolParameter,
    SetStringParameter,
    SetObjectParameter,

    // === 参数加载 ===
    LoadIntParameter,
    LoadFloatParameter,
    LoadBoolParameter,
    LoadStringParameter,
    LoadObjectParameter,

    // === 注入器 ===
    LoadInjector,
    SetInjector,

    // === 算术运算 ===
    IntAdd,
    IntSub,
    IntMul,
    IntDiv,
    IntMod,
    FloatAdd,
    FloatSub,
    FloatMul,
    FloatDiv,

    // === 字符串加法 ===
    StringAddition,

    // === 比较 ===
    IntLess,
    IntLessEqual,
    IntGreater,
    IntGreaterEqual,
    FloatLess,
    FloatLessEqual,
    FloatGreater,
    FloatGreaterEqual,
    IntEqual,
    FloatEqual,
    BoolEqual,
    StringEqual,
    ObjectEqual,
    IntNotEqual,
    FloatNotEqual,
    BoolNotEqual,
    StringNotEqual,
    ObjectNotEqual,

    // === 逻辑 ===
    LogicalAnd,
    LogicalOr,
    LogicalNot,

    // === 类型转换 ===
    IntToFloat,
    FloatToInt,
    IntToBool,
    BoolToInt,
    IntToString,
    FloatToString,
    BoolToString,

    // === 转字符串 ===
    IntCastToString,
    FloatCastToString,
    BoolCastToString,

    // === 控制流 ===
    Jump(usize),       // 无条件跳转（目标代码索引）
    JumpIfFalse(usize), // 条件为假跳转
    JumpIfTrue(usize),  // 条件为真跳转

    // === 方法调用 ===
    InvokeInstance(usize),  // 方法 ID
    InvokeStatic(usize),
    InvokeInterface(usize),
    InvokeDelegate(usize),     // 委托实现索引
    InvokeConstructor(usize), // 类 ID + 构造方法 ID

    // === 对象创建 ===
    DoConstruct(usize), // 类 ID

    // === 构造委托 ===
    ConstructDelegate(usize), // 委托实现索引

    // === 返回值获取 ===
    GetReturnInt,
    GetReturnFloat,
    GetReturnBool,
    GetReturnString,
    GetReturnObject,

    // === 返回 ===
    ReturnInt,
    ReturnFloat,
    ReturnBool,
    ReturnString,
    ReturnObject,
    ReturnVoid,

    /// 空操作（占位符，优化后消除）
    Nop,
}

/// 三地址码中间表示
///
/// 每条指令格式：`result = left op right`
#[derive(Debug, Clone)]
pub struct IntermediateCode {
    /// 结果存储的目标地址
    pub result: Option<Address>,
    /// 操作符
    pub operator: IntermediateOperator,
    /// 左操作数
    pub left: Operand,
    /// 右操作数（可选）
    pub right: Option<Operand>,
}

impl IntermediateCode {
    /// 创建一条三地址码指令
    pub fn new(
        operator: IntermediateOperator,
        left: Operand,
        right: Option<Operand>,
        result: Option<Address>,
    ) -> Self {
        Self {
            operator,
            left,
            right,
            result,
        }
    }

    /// 创建赋值指令 `result = left`
    pub fn assign(result: Address, value: Operand) -> Self {
        let op = match result.value_type {
            ValueType::Int => IntermediateOperator::IntAssign,
            ValueType::Float => IntermediateOperator::FloatAssign,
            ValueType::Bool => IntermediateOperator::BoolAssign,
            ValueType::String => IntermediateOperator::StringAssign,
            ValueType::Object => IntermediateOperator::ObjectAssign,
        };
        Self::new(op, value, None, Some(result))
    }

    /// 创建二元运算指令 `result = left op right`
    pub fn binary(
        op: IntermediateOperator,
        left: Operand,
        right: Operand,
        result: Address,
    ) -> Self {
        Self::new(op, left, Some(right), Some(result))
    }

    /// 创建返回指令
    pub fn return_value(value_type: ValueType) -> Self {
        let op = match value_type {
            ValueType::Int => IntermediateOperator::ReturnInt,
            ValueType::Float => IntermediateOperator::ReturnFloat,
            ValueType::Bool => IntermediateOperator::ReturnBool,
            ValueType::String => IntermediateOperator::ReturnString,
            ValueType::Object => IntermediateOperator::ReturnObject,
        };
        Self::new(op, Operand::Address(Address::new(value_type, 0)), None, None)
    }

    pub fn return_void() -> Self {
        Self::new(
            IntermediateOperator::ReturnVoid,
            Operand::Address(Address::new(ValueType::Int, 0)),
            None,
            None,
        )
    }

    /// 创建无条件跳转
    pub fn jump(target: usize) -> Self {
        Self::new(
            IntermediateOperator::Jump(target),
            Operand::int(target as i64),
            None,
            None,
        )
    }

    /// 创建条件跳转（条件为假时跳转）
    pub fn jump_if_false(condition: Operand, target: usize) -> Self {
        Self::new(
            IntermediateOperator::JumpIfFalse(target),
            condition,
            None,
            None,
        )
    }

    /// 创建条件跳转（条件为真时跳转）
    pub fn jump_if_true(condition: Operand, target: usize) -> Self {
        Self::new(
            IntermediateOperator::JumpIfTrue(target),
            condition,
            None,
            None,
        )
    }

    /// 创建 Nop
    pub fn nop() -> Self {
        Self::new(
            IntermediateOperator::Nop,
            Operand::int(0),
            None,
            None,
        )
    }
}

/// 带源码位置的 IR 指令
#[derive(Debug, Clone)]
pub struct CodeWithSpan {
    pub code: IntermediateCode,
    pub span: Span,
}

impl CodeWithSpan {
    pub fn new(code: IntermediateCode, span: Span) -> Self {
        Self { code, span }
    }
}

/// 编译后的方法实现
#[derive(Debug, Clone)]
pub struct CompiledMethod {
    pub name: String,
    pub codes: Vec<CodeWithSpan>,
    /// 局部变量数量（每个值类型分别计数）
    pub local_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ir_instruction() {
        let addr = Address::new(ValueType::Int, 0);
        let code = IntermediateCode::assign(addr, Operand::int(42));
        assert!(matches!(
            code.operator,
            IntermediateOperator::IntAssign
        ));
    }

    #[test]
    fn test_binary_operation() {
        let a = Address::new(ValueType::Int, 0);
        let b = Address::new(ValueType::Int, 1);
        let r = Address::new(ValueType::Int, 2);
        let code = IntermediateCode::binary(
            IntermediateOperator::IntAdd,
            Operand::Address(a),
            Operand::Address(b),
            r,
        );
        assert!(matches!(code.operator, IntermediateOperator::IntAdd));
        assert!(code.result.is_some());
        assert!(code.right.is_some());
    }

    #[test]
    fn test_return_instruction() {
        let code = IntermediateCode::return_void();
        assert!(matches!(code.operator, IntermediateOperator::ReturnVoid));
    }

    #[test]
    fn test_jump_instruction() {
        let code = IntermediateCode::jump(42);
        assert!(matches!(code.operator, IntermediateOperator::Jump(42)));
    }
}
