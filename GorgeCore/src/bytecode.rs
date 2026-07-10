#![allow(dead_code)]

use crate::ir::*;
use crate::types::{GorgeType, TypeCount};

/// 编译模块
///
/// 一个 .gorge 文件中所有类及其方法的编译产物。
#[derive(Debug, Clone)]
pub struct CompiledModule {
    pub version: u16,
    pub classes: Vec<CompiledClass>,
}

/// 编译后的类
///
/// 包含类元数据和编译后的方法 IR。
#[derive(Debug, Clone)]
pub struct CompiledClass {
    pub class_type: GorgeType,
    pub is_native: bool,
    pub super_class_name: Option<String>,
    pub super_interfaces: Vec<String>,
    pub field_counts: TypeCount,
    pub methods: Vec<CompiledMethod>,
    pub constructors: Vec<CompiledMethod>,
    pub injector_fields: Vec<InjectorFieldDef>,
    pub delegate_impls: Vec<DelegateImpl>,
}

/// 委托实现元数据
#[derive(Debug, Clone)]
pub struct DelegateImpl {
    pub param_types: Vec<ValueType>,
    pub return_type: ValueType,
    pub body_ir: Vec<CodeWithSpan>,
    pub captured_var_names: Vec<String>,
    pub outer_value_count: usize,
}

/// 注入器字段定义（序列化用）
#[derive(Debug, Clone)]
pub struct InjectorFieldDef {
    pub name: String,
    pub value_type: ValueType,
    pub has_default: bool,
}

/// 字节码魔数："GORG"
const MAGIC: [u8; 4] = [b'G', b'O', b'R', b'G'];
/// 字节码格式版本
const VERSION: u16 = 1;

/// 操作码 → u16 编号的双向映射
fn opcode_to_u16(op: &IntermediateOperator) -> u16 {
    match op {
        IntermediateOperator::IntAssign => 0,
        IntermediateOperator::FloatAssign => 1,
        IntermediateOperator::BoolAssign => 2,
        IntermediateOperator::StringAssign => 3,
        IntermediateOperator::ObjectAssign => 4,
        IntermediateOperator::LoadIntField(_) => 10,
        IntermediateOperator::LoadFloatField(_) => 11,
        IntermediateOperator::LoadBoolField(_) => 12,
        IntermediateOperator::LoadStringField(_) => 13,
        IntermediateOperator::LoadObjectField(_) => 14,
        IntermediateOperator::SetIntField(_) => 15,
        IntermediateOperator::SetFloatField(_) => 16,
        IntermediateOperator::SetBoolField(_) => 17,
        IntermediateOperator::SetStringField(_) => 18,
        IntermediateOperator::SetObjectField(_) => 19,
        IntermediateOperator::IntAdd => 20,
        IntermediateOperator::IntSub => 21,
        IntermediateOperator::IntMul => 22,
        IntermediateOperator::IntDiv => 23,
        IntermediateOperator::IntMod => 24,
        IntermediateOperator::FloatAdd => 25,
        IntermediateOperator::FloatSub => 26,
        IntermediateOperator::FloatMul => 27,
        IntermediateOperator::FloatDiv => 28,
        IntermediateOperator::IntLess => 30,
        IntermediateOperator::IntLessEqual => 31,
        IntermediateOperator::IntGreater => 32,
        IntermediateOperator::IntGreaterEqual => 33,
        IntermediateOperator::FloatLess => 34,
        IntermediateOperator::FloatLessEqual => 35,
        IntermediateOperator::FloatGreater => 36,
        IntermediateOperator::FloatGreaterEqual => 37,
        IntermediateOperator::IntEqual => 38,
        IntermediateOperator::FloatEqual => 39,
        IntermediateOperator::BoolEqual => 40,
        IntermediateOperator::StringEqual => 41,
        IntermediateOperator::ObjectEqual => 42,
        IntermediateOperator::IntNotEqual => 43,
        IntermediateOperator::FloatNotEqual => 44,
        IntermediateOperator::BoolNotEqual => 45,
        IntermediateOperator::StringNotEqual => 46,
        IntermediateOperator::ObjectNotEqual => 47,
        IntermediateOperator::LogicalAnd => 50,
        IntermediateOperator::LogicalOr => 51,
        IntermediateOperator::LogicalNot => 52,
        IntermediateOperator::IntToFloat => 60,
        IntermediateOperator::FloatToInt => 61,
        IntermediateOperator::IntToBool => 62,
        IntermediateOperator::BoolToInt => 63,
        IntermediateOperator::IntToString => 64,
        IntermediateOperator::FloatToString => 65,
        IntermediateOperator::BoolToString => 66,
        IntermediateOperator::Jump(_) => 70,
        IntermediateOperator::JumpIfFalse(_) => 71,
        IntermediateOperator::JumpIfTrue(_) => 72,
        IntermediateOperator::InvokeInstance(_) => 80,
        IntermediateOperator::InvokeStatic(_) => 81,
        IntermediateOperator::InvokeInterface(_) => 82,
        IntermediateOperator::InvokeDelegate(_) => 83,
        IntermediateOperator::InvokeConstructor(_) => 84,
        IntermediateOperator::DoConstruct(_) => 90,
        IntermediateOperator::ReturnInt => 100,
        IntermediateOperator::ReturnFloat => 101,
        IntermediateOperator::ReturnBool => 102,
        IntermediateOperator::ReturnString => 103,
        IntermediateOperator::ReturnObject => 104,
        IntermediateOperator::ReturnVoid => 105,
        IntermediateOperator::StringAddition => 200,
        IntermediateOperator::LoadThis => 201,
        IntermediateOperator::SetIntParameter => 202,
        IntermediateOperator::SetFloatParameter => 203,
        IntermediateOperator::SetBoolParameter => 204,
        IntermediateOperator::SetStringParameter => 205,
        IntermediateOperator::SetObjectParameter => 206,
        IntermediateOperator::LoadIntParameter => 207,
        IntermediateOperator::LoadFloatParameter => 208,
        IntermediateOperator::LoadBoolParameter => 209,
        IntermediateOperator::LoadStringParameter => 210,
        IntermediateOperator::LoadObjectParameter => 211,
        IntermediateOperator::GetReturnInt => 212,
        IntermediateOperator::GetReturnFloat => 213,
        IntermediateOperator::GetReturnBool => 214,
        IntermediateOperator::GetReturnString => 215,
        IntermediateOperator::GetReturnObject => 216,
        IntermediateOperator::IntCastToString => 217,
        IntermediateOperator::FloatCastToString => 218,
        IntermediateOperator::BoolCastToString => 219,
        IntermediateOperator::LoadInjector => 220,
        IntermediateOperator::SetInjector => 221,
        IntermediateOperator::ConstructDelegate(_) => 222,
        IntermediateOperator::LoadStaticIntField(_) => 223,
        IntermediateOperator::LoadStaticFloatField(_) => 224,
        IntermediateOperator::LoadStaticBoolField(_) => 225,
        IntermediateOperator::LoadStaticStringField(_) => 226,
        IntermediateOperator::LoadStaticObjectField(_) => 227,
        IntermediateOperator::SetStaticIntField(_) => 228,
        IntermediateOperator::SetStaticFloatField(_) => 229,
        IntermediateOperator::SetStaticBoolField(_) => 230,
        IntermediateOperator::SetStaticStringField(_) => 231,
        IntermediateOperator::SetStaticObjectField(_) => 232,
        IntermediateOperator::Nop => 255,
        _ => 254, // fallback
    }
}

fn u16_to_opcode(code: u16, extra: u16) -> IntermediateOperator {
    match code {
        0 => IntermediateOperator::IntAssign,
        1 => IntermediateOperator::FloatAssign,
        2 => IntermediateOperator::BoolAssign,
        3 => IntermediateOperator::StringAssign,
        4 => IntermediateOperator::ObjectAssign,
        10 => IntermediateOperator::LoadIntField(extra as usize),
        11 => IntermediateOperator::LoadFloatField(extra as usize),
        12 => IntermediateOperator::LoadBoolField(extra as usize),
        13 => IntermediateOperator::LoadStringField(extra as usize),
        14 => IntermediateOperator::LoadObjectField(extra as usize),
        15 => IntermediateOperator::SetIntField(extra as usize),
        16 => IntermediateOperator::SetFloatField(extra as usize),
        17 => IntermediateOperator::SetBoolField(extra as usize),
        18 => IntermediateOperator::SetStringField(extra as usize),
        19 => IntermediateOperator::SetObjectField(extra as usize),
        20 => IntermediateOperator::IntAdd,
        21 => IntermediateOperator::IntSub,
        22 => IntermediateOperator::IntMul,
        23 => IntermediateOperator::IntDiv,
        24 => IntermediateOperator::IntMod,
        25 => IntermediateOperator::FloatAdd,
        26 => IntermediateOperator::FloatSub,
        27 => IntermediateOperator::FloatMul,
        28 => IntermediateOperator::FloatDiv,
        30 => IntermediateOperator::IntLess,
        31 => IntermediateOperator::IntLessEqual,
        32 => IntermediateOperator::IntGreater,
        33 => IntermediateOperator::IntGreaterEqual,
        34 => IntermediateOperator::FloatLess,
        35 => IntermediateOperator::FloatLessEqual,
        36 => IntermediateOperator::FloatGreater,
        37 => IntermediateOperator::FloatGreaterEqual,
        38 => IntermediateOperator::IntEqual,
        39 => IntermediateOperator::FloatEqual,
        40 => IntermediateOperator::BoolEqual,
        41 => IntermediateOperator::StringEqual,
        42 => IntermediateOperator::ObjectEqual,
        43 => IntermediateOperator::IntNotEqual,
        44 => IntermediateOperator::FloatNotEqual,
        45 => IntermediateOperator::BoolNotEqual,
        46 => IntermediateOperator::StringNotEqual,
        47 => IntermediateOperator::ObjectNotEqual,
        50 => IntermediateOperator::LogicalAnd,
        51 => IntermediateOperator::LogicalOr,
        52 => IntermediateOperator::LogicalNot,
        60 => IntermediateOperator::IntToFloat,
        61 => IntermediateOperator::FloatToInt,
        62 => IntermediateOperator::IntToBool,
        63 => IntermediateOperator::BoolToInt,
        64 => IntermediateOperator::IntToString,
        65 => IntermediateOperator::FloatToString,
        66 => IntermediateOperator::BoolToString,
        70 => IntermediateOperator::Jump(extra as usize),
        71 => IntermediateOperator::JumpIfFalse(extra as usize),
        72 => IntermediateOperator::JumpIfTrue(extra as usize),
        80 => IntermediateOperator::InvokeInstance(extra as usize),
        81 => IntermediateOperator::InvokeStatic(extra as usize),
        82 => IntermediateOperator::InvokeInterface(extra as usize),
        83 => IntermediateOperator::InvokeDelegate(extra as usize),
        84 => IntermediateOperator::InvokeConstructor(extra as usize),
        90 => IntermediateOperator::DoConstruct(extra as usize),
        100 => IntermediateOperator::ReturnInt,
        101 => IntermediateOperator::ReturnFloat,
        102 => IntermediateOperator::ReturnBool,
        103 => IntermediateOperator::ReturnString,
        104 => IntermediateOperator::ReturnObject,
        105 => IntermediateOperator::ReturnVoid,
        200 => IntermediateOperator::StringAddition,
        201 => IntermediateOperator::LoadThis,
        202 => IntermediateOperator::SetIntParameter,
        203 => IntermediateOperator::SetFloatParameter,
        204 => IntermediateOperator::SetBoolParameter,
        205 => IntermediateOperator::SetStringParameter,
        206 => IntermediateOperator::SetObjectParameter,
        207 => IntermediateOperator::LoadIntParameter,
        208 => IntermediateOperator::LoadFloatParameter,
        209 => IntermediateOperator::LoadBoolParameter,
        210 => IntermediateOperator::LoadStringParameter,
        211 => IntermediateOperator::LoadObjectParameter,
        212 => IntermediateOperator::GetReturnInt,
        213 => IntermediateOperator::GetReturnFloat,
        214 => IntermediateOperator::GetReturnBool,
        215 => IntermediateOperator::GetReturnString,
        216 => IntermediateOperator::GetReturnObject,
        217 => IntermediateOperator::IntCastToString,
        218 => IntermediateOperator::FloatCastToString,
        219 => IntermediateOperator::BoolCastToString,
        220 => IntermediateOperator::LoadInjector,
        221 => IntermediateOperator::SetInjector,
        222 => IntermediateOperator::ConstructDelegate(extra as usize),
        223 => IntermediateOperator::LoadStaticIntField(extra as usize),
        224 => IntermediateOperator::LoadStaticFloatField(extra as usize),
        225 => IntermediateOperator::LoadStaticBoolField(extra as usize),
        226 => IntermediateOperator::LoadStaticStringField(extra as usize),
        227 => IntermediateOperator::LoadStaticObjectField(extra as usize),
        228 => IntermediateOperator::SetStaticIntField(extra as usize),
        229 => IntermediateOperator::SetStaticFloatField(extra as usize),
        230 => IntermediateOperator::SetStaticBoolField(extra as usize),
        231 => IntermediateOperator::SetStaticStringField(extra as usize),
        232 => IntermediateOperator::SetStaticObjectField(extra as usize),
        255 => IntermediateOperator::Nop,
        _ => IntermediateOperator::Nop,
    }
}

/// 字节码序列化/反序列化错误
pub type BytecodeResult<T> = Result<T, String>;

/// 将编译后的方法列表序列化为字节码
pub fn serialize(methods: &[CompiledMethod]) -> BytecodeResult<Vec<u8>> {
    let mut buf = Vec::new();

    // 写入文件头
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&VERSION.to_le_bytes());
    buf.extend_from_slice(&(methods.len() as u16).to_le_bytes());

    for method in methods {
        serialize_method(method, &mut buf)?;
    }

    Ok(buf)
}

/// 将编译模块序列化为字节码
pub fn serialize_module(module: &CompiledModule) -> BytecodeResult<Vec<u8>> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&2u16.to_le_bytes());
    buf.extend_from_slice(&(module.classes.len() as u16).to_le_bytes());

    for class in &module.classes {
        serialize_compiled_class(class, &mut buf)?;
    }

    Ok(buf)
}

fn serialize_compiled_class(class: &CompiledClass, buf: &mut Vec<u8>) -> BytecodeResult<()> {
    let full_name = class.class_type.full_name();
    let name_bytes = full_name.as_bytes();
    buf.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
    buf.extend_from_slice(name_bytes);

    buf.push(if class.is_native { 1 } else { 0 });

    buf.extend_from_slice(&(class.field_counts.int_count as u32).to_le_bytes());
    buf.extend_from_slice(&(class.field_counts.float_count as u32).to_le_bytes());
    buf.extend_from_slice(&(class.field_counts.bool_count as u32).to_le_bytes());
    buf.extend_from_slice(&(class.field_counts.string_count as u32).to_le_bytes());
    buf.extend_from_slice(&(class.field_counts.object_count as u32).to_le_bytes());

    match &class.super_class_name {
        Some(name) => {
            let bytes = name.as_bytes();
            buf.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
            buf.extend_from_slice(bytes);
        }
        None => {
            buf.extend_from_slice(&0u16.to_le_bytes());
        }
    }

    buf.extend_from_slice(&(class.super_interfaces.len() as u16).to_le_bytes());
    for iface in &class.super_interfaces {
        let bytes = iface.as_bytes();
        buf.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
        buf.extend_from_slice(bytes);
    }

    // 注入器字段
    buf.extend_from_slice(&(class.injector_fields.len() as u16).to_le_bytes());
    for field in &class.injector_fields {
        let name_bytes = field.name.as_bytes();
        buf.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        buf.extend_from_slice(name_bytes);
        buf.push(value_type_to_u8(field.value_type));
        buf.push(if field.has_default { 1 } else { 0 });
    }

    // 委托实现
    buf.extend_from_slice(&(class.delegate_impls.len() as u16).to_le_bytes());
    for delegate in &class.delegate_impls {
        buf.extend_from_slice(&(delegate.param_types.len() as u16).to_le_bytes());
        for pt in &delegate.param_types {
            buf.push(value_type_to_u8(*pt));
        }
        buf.push(value_type_to_u8(delegate.return_type));
        buf.extend_from_slice(&(delegate.captured_var_names.len() as u16).to_le_bytes());
        for name in &delegate.captured_var_names {
            let bytes = name.as_bytes();
            buf.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
            buf.extend_from_slice(bytes);
        }
        buf.extend_from_slice(&(delegate.body_ir.len() as u32).to_le_bytes());
        for code_span in &delegate.body_ir {
            serialize_code(&code_span.code, buf)?;
        }
    }

    buf.extend_from_slice(&(class.methods.len() as u16).to_le_bytes());
    for method in &class.methods {
        serialize_method(method, buf)?;
    }

    buf.extend_from_slice(&(class.constructors.len() as u16).to_le_bytes());
    for ctor in &class.constructors {
        serialize_method(ctor, buf)?;
    }

    Ok(())
}

fn serialize_method(method: &CompiledMethod, buf: &mut Vec<u8>) -> BytecodeResult<()> {
    // 方法名
    let name_bytes = method.name.as_bytes();
    buf.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
    buf.extend_from_slice(name_bytes);

    // 局部变量数
    buf.extend_from_slice(&(method.local_count as u32).to_le_bytes());

    // 指令数量
    buf.extend_from_slice(&(method.codes.len() as u32).to_le_bytes());

    for code_span in &method.codes {
        serialize_code(&code_span.code, buf)?;
    }

    Ok(())
}

fn serialize_code(code: &IntermediateCode, buf: &mut Vec<u8>) -> BytecodeResult<()> {
    // 操作码编号 + 额外数据
    let extra = get_extra_field(&code.operator);
    let opcode = opcode_to_u16(&code.operator);
    buf.extend_from_slice(&opcode.to_le_bytes());
    buf.extend_from_slice(&extra.to_le_bytes());

    // result
    write_optional_address(code.result, buf);

    // left operand
    write_operand(&code.left, buf)?;

    // right operand
    write_optional_operand(code.right.as_ref(), buf)?;

    Ok(())
}

fn get_extra_field(op: &IntermediateOperator) -> u16 {
    match op {
        IntermediateOperator::Jump(v)
        | IntermediateOperator::JumpIfFalse(v)
        | IntermediateOperator::JumpIfTrue(v)
        | IntermediateOperator::InvokeInstance(v)
        | IntermediateOperator::InvokeStatic(v)
        | IntermediateOperator::InvokeInterface(v)
        | IntermediateOperator::InvokeConstructor(v)
        | IntermediateOperator::DoConstruct(v)
        | IntermediateOperator::LoadIntField(v)
        | IntermediateOperator::LoadFloatField(v)
        | IntermediateOperator::LoadBoolField(v)
        | IntermediateOperator::LoadStringField(v)
        | IntermediateOperator::LoadObjectField(v)
        | IntermediateOperator::SetIntField(v)
        | IntermediateOperator::SetFloatField(v)
        | IntermediateOperator::SetBoolField(v)
        | IntermediateOperator::SetStringField(v)
        | IntermediateOperator::SetObjectField(v)
        | IntermediateOperator::LoadIntInjectorField(v)
        | IntermediateOperator::LoadFloatInjectorField(v)
        | IntermediateOperator::LoadBoolInjectorField(v)
        | IntermediateOperator::LoadStringInjectorField(v)
        | IntermediateOperator::LoadObjectInjectorField(v)
        | IntermediateOperator::SetIntInjectorField(v)
        | IntermediateOperator::SetFloatInjectorField(v)
        | IntermediateOperator::SetBoolInjectorField(v)
        | IntermediateOperator::SetStringInjectorField(v)
        | IntermediateOperator::SetObjectInjectorField(v)
        | IntermediateOperator::LoadStaticIntField(v)
        | IntermediateOperator::LoadStaticFloatField(v)
        | IntermediateOperator::LoadStaticBoolField(v)
        | IntermediateOperator::LoadStaticStringField(v)
        | IntermediateOperator::LoadStaticObjectField(v)
        | IntermediateOperator::SetStaticIntField(v)
        | IntermediateOperator::SetStaticFloatField(v)
        | IntermediateOperator::SetStaticBoolField(v)
        | IntermediateOperator::SetStaticStringField(v)
        | IntermediateOperator::SetStaticObjectField(v)
        | IntermediateOperator::ConstructDelegate(v)
        | IntermediateOperator::InvokeDelegate(v) => *v as u16,
        _ => 0,
    }
}

fn write_optional_address(addr: Option<Address>, buf: &mut Vec<u8>) {
    match addr {
        Some(a) => {
            buf.push(1); // has address
            buf.push(value_type_to_u8(a.value_type));
            buf.extend_from_slice(&(a.index as u32).to_le_bytes());
        }
        None => {
            buf.push(0); // no address
        }
    }
}

fn write_operand(op: &Operand, buf: &mut Vec<u8>) -> BytecodeResult<()> {
    match op {
        Operand::Address(addr) => {
            buf.push(0); // address kind
            buf.push(value_type_to_u8(addr.value_type));
            buf.extend_from_slice(&(addr.index as u32).to_le_bytes());
        }
        Operand::Immediate(val) => match val {
            ImmediateValue::Int(v) => {
                buf.push(1); // int immediate kind
                buf.extend_from_slice(&v.to_le_bytes());
            }
            ImmediateValue::Float(v) => {
                buf.push(2); // float immediate kind
                buf.extend_from_slice(&v.to_le_bytes());
            }
            ImmediateValue::Bool(v) => {
                buf.push(3); // bool immediate kind
                buf.push(if *v { 1 } else { 0 });
            }
            ImmediateValue::String(s) => {
                buf.push(4); // string immediate kind
                let bytes = s.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                buf.extend_from_slice(bytes);
            }
        },
    }
    Ok(())
}

fn write_optional_operand(op: Option<&Operand>, buf: &mut Vec<u8>) -> BytecodeResult<()> {
    match op {
        Some(o) => {
            buf.push(1); // has operand
            write_operand(o, buf)
        }
        None => {
            buf.push(0); // no operand
            Ok(())
        }
    }
}

// ==================== 反序列化 ====================

/// 从字节码反序列化为方法列表
pub fn deserialize(data: &[u8]) -> BytecodeResult<Vec<CompiledMethod>> {
    let module = deserialize_module(data)?;
    let mut methods = Vec::new();
    for class in &module.classes {
        methods.extend(class.methods.clone());
    }
    Ok(methods)
}

/// 从字节码反序列化为编译模块
pub fn deserialize_module(data: &[u8]) -> BytecodeResult<CompiledModule> {
    let mut pos = 0;
    if data.len() < 8 {
        return Err("字节码数据太短".into());
    }
    let magic = &data[pos..pos + 4];
    if magic != MAGIC {
        return Err(format!("无效的魔数: {:?}", magic));
    }
    pos += 4;

    let version = u16::from_le_bytes([data[pos], data[pos + 1]]);
    pos += 2;

    if version == 1 {
        let method_count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;
        let mut methods = Vec::with_capacity(method_count);
        for _ in 0..method_count {
            let (method, new_pos) = deserialize_method(data, pos)?;
            methods.push(method);
            pos = new_pos;
        }
        let class = CompiledClass {
            class_type: GorgeType::class("Module", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods,
            constructors: vec![],
            injector_fields: vec![],
            delegate_impls: vec![],
        };
        Ok(CompiledModule { version: 1, classes: vec![class] })
    } else {
        let class_count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;
        let mut classes = Vec::with_capacity(class_count);
        for _ in 0..class_count {
            let (class, new_pos) = deserialize_compiled_class(data, pos)?;
            classes.push(class);
            pos = new_pos;
        }
        Ok(CompiledModule { version, classes })
    }
}

fn deserialize_compiled_class(data: &[u8], mut pos: usize) -> BytecodeResult<(CompiledClass, usize)> {
    if pos + 2 > data.len() { return Err("读取类名长度越界".into()); }
    let name_len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;
    if pos + name_len > data.len() { return Err("读取类名越界".into()); }
    let full_name = String::from_utf8_lossy(&data[pos..pos + name_len]).into_owned();
    pos += name_len;

    if pos >= data.len() { return Err("读取 is_native 越界".into()); }
    let is_native = data[pos] == 1;
    pos += 1;

    if pos + 20 > data.len() { return Err("读取字段计数越界".into()); }
    let ic = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
    pos += 4;
    let fc = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
    pos += 4;
    let bc = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
    pos += 4;
    let sc = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
    pos += 4;
    let oc = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
    pos += 4;

    if pos + 2 > data.len() { return Err("读取父类名长度越界".into()); }
    let super_len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;
    let super_class_name = if super_len > 0 {
        if pos + super_len > data.len() { return Err("读取父类名越界".into()); }
        let name = String::from_utf8_lossy(&data[pos..pos + super_len]).into_owned();
        pos += super_len;
        Some(name)
    } else {
        None
    };

    if pos + 2 > data.len() { return Err("读取接口计数越界".into()); }
    let iface_count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;
    let mut super_interfaces = Vec::with_capacity(iface_count);
    for _ in 0..iface_count {
        if pos + 2 > data.len() { return Err("读取接口名长度越界".into()); }
        let ilen = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;
        if pos + ilen > data.len() { return Err("读取接口名越界".into()); }
        super_interfaces.push(String::from_utf8_lossy(&data[pos..pos + ilen]).into_owned());
        pos += ilen;
    }

    // 注入器字段
    if pos + 2 > data.len() { return Err("读取注入器字段计数越界".into()); }
    let injector_count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;
    let mut injector_fields = Vec::with_capacity(injector_count);
    for _ in 0..injector_count {
        if pos + 2 > data.len() { return Err("读取注入器字段名长度越界".into()); }
        let nlen = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;
        if pos + nlen > data.len() { return Err("读取注入器字段名越界".into()); }
        let name = String::from_utf8_lossy(&data[pos..pos + nlen]).into_owned();
        pos += nlen;
        if pos + 2 > data.len() { return Err("读取注入器字段类型越界".into()); }
        let vt = u8_to_value_type(data[pos]);
        pos += 1;
        let has_default = data[pos] == 1;
        pos += 1;
        injector_fields.push(InjectorFieldDef { name, value_type: vt, has_default });
    }

    // 委托实现
    if pos + 2 > data.len() { return Err("读取委托计数越界".into()); }
    let delegate_count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;
    let mut delegate_impls = Vec::with_capacity(delegate_count);
    for _ in 0..delegate_count {
        if pos + 2 > data.len() { return Err("读取委托参数类型计数越界".into()); }
        let param_count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;
        let mut param_types = Vec::with_capacity(param_count);
        for _ in 0..param_count {
            if pos >= data.len() { return Err("读取委托参数类型越界".into()); }
            param_types.push(u8_to_value_type(data[pos]));
            pos += 1;
        }
        if pos >= data.len() { return Err("读取委托返回值类型越界".into()); }
        let return_type = u8_to_value_type(data[pos]);
        pos += 1;
        if pos + 2 > data.len() { return Err("读取委托捕获变量计数越界".into()); }
        let cv_count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;
        let mut captured_var_names = Vec::with_capacity(cv_count);
        for _ in 0..cv_count {
            if pos + 2 > data.len() { return Err("读取委托捕获变量名长度越界".into()); }
            let clen = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
            pos += 2;
            if pos + clen > data.len() { return Err("读取委托捕获变量名越界".into()); }
            captured_var_names.push(String::from_utf8_lossy(&data[pos..pos + clen]).into_owned());
            pos += clen;
        }
        if pos + 4 > data.len() { return Err("读取委托 IR 计数越界".into()); }
        let ir_count = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;
        let mut body_ir = Vec::with_capacity(ir_count);
        for _ in 0..ir_count {
            let (code_span, new_pos) = deserialize_code(data, pos)?;
            body_ir.push(code_span);
            pos = new_pos;
        }
        delegate_impls.push(DelegateImpl {
            param_types,
            return_type,
            body_ir,
            captured_var_names,
            outer_value_count: 0,
        });
    }

    if pos + 2 > data.len() { return Err("读取方法计数越界".into()); }
    let method_count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;
    let mut methods = Vec::with_capacity(method_count);
    for _ in 0..method_count {
        let (method, new_pos) = deserialize_method(data, pos)?;
        methods.push(method);
        pos = new_pos;
    }

    // 构造方法
    if pos + 2 > data.len() { return Err("读取构造方法计数越界".into()); }
    let ctor_count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;
    let mut constructors = Vec::with_capacity(ctor_count);
    for _ in 0..ctor_count {
        let (ctor, new_pos) = deserialize_method(data, pos)?;
        constructors.push(ctor);
        pos = new_pos;
    }

    let class_name = full_name.rsplit('.').next().unwrap_or(&full_name).to_string();

    Ok((CompiledClass {
        class_type: GorgeType::class(class_name, None),
        is_native,
        super_class_name,
        super_interfaces,
        field_counts: TypeCount {
            int_count: ic, float_count: fc, bool_count: bc,
            string_count: sc, object_count: oc,
        },
        methods,
        constructors,
        injector_fields,
        delegate_impls,
    }, pos))
}

fn deserialize_method(data: &[u8], mut pos: usize) -> BytecodeResult<(CompiledMethod, usize)> {
    // 方法名
    if pos + 2 > data.len() {
        return Err("读取方法名长度越界".into());
    }
    let name_len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;
    if pos + name_len > data.len() {
        return Err("读取方法名越界".into());
    }
    let name = String::from_utf8_lossy(&data[pos..pos + name_len]).into_owned();
    pos += name_len;

    // 局部变量数
    if pos + 4 > data.len() {
        return Err("读取局部变量数越界".into());
    }
    let local_count = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
    pos += 4;

    // 指令数量
    if pos + 4 > data.len() {
        return Err("读取指令数量越界".into());
    }
    let code_count = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
    pos += 4;

    let mut codes = Vec::with_capacity(code_count);
    for _ in 0..code_count {
        let (code, new_pos) = deserialize_code(data, pos)?;
        codes.push(code);
        pos = new_pos;
    }

    Ok((CompiledMethod {
        name,
        codes,
        local_count,
    }, pos))
}

fn deserialize_code(data: &[u8], mut pos: usize) -> BytecodeResult<(CodeWithSpan, usize)> {
    if pos + 4 > data.len() {
        return Err("读取操作码越界".into());
    }
    let opcode = u16::from_le_bytes([data[pos], data[pos + 1]]);
    let extra = u16::from_le_bytes([data[pos + 2], data[pos + 3]]);
    pos += 4;

    let operator = u16_to_opcode(opcode, extra);

    // result address
    let (result, new_pos) = read_optional_address(data, pos)?;
    pos = new_pos;

    // left operand
    let (left, new_pos) = read_operand(data, pos)?;
    pos = new_pos;

    // right operand
    let (right, new_pos) = read_optional_operand(data, pos)?;
    pos = new_pos;

    Ok((CodeWithSpan::new(
        IntermediateCode::new(operator, left, right, result),
        crate::diagnostics::Span::dummy(),
    ), pos))
}

fn read_optional_address(data: &[u8], pos: usize) -> BytecodeResult<(Option<Address>, usize)> {
    if pos >= data.len() {
        return Err("读取 address flag 越界".into());
    }
    let has = data[pos] == 1;
    let mut pos = pos + 1;

    if !has {
        return Ok((None, pos));
    }

    if pos + 5 > data.len() {
        return Err("读取 address 越界".into());
    }
    let vt = u8_to_value_type(data[pos]);
    pos += 1;
    let index = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
    pos += 4;

    Ok((Some(Address::new(vt, index)), pos))
}

fn read_operand(data: &[u8], pos: usize) -> BytecodeResult<(Operand, usize)> {
    if pos >= data.len() {
        return Err("读取 operand kind 越界".into());
    }
    let kind = data[pos];
    let pos = pos + 1;

    match kind {
        0 => {
            // address
            if pos + 5 > data.len() {
                return Err("读取 address operand 越界".into());
            }
            let vt = u8_to_value_type(data[pos]);
            let index = u32::from_le_bytes([data[pos + 1], data[pos + 2], data[pos + 3], data[pos + 4]]) as usize;
            Ok((Operand::Address(Address::new(vt, index)), pos + 5))
        }
        1 => {
            // int immediate
            if pos + 8 > data.len() {
                return Err("读取 int immediate 越界".into());
            }
            let val = i64::from_le_bytes([
                data[pos], data[pos + 1], data[pos + 2], data[pos + 3],
                data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7],
            ]);
            Ok((Operand::int(val), pos + 8))
        }
        2 => {
            // float immediate
            if pos + 8 > data.len() {
                return Err("读取 float immediate 越界".into());
            }
            let val = f64::from_le_bytes([
                data[pos], data[pos + 1], data[pos + 2], data[pos + 3],
                data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7],
            ]);
            Ok((Operand::float(val), pos + 8))
        }
        3 => {
            // bool immediate
            if pos >= data.len() {
                return Err("读取 bool immediate 越界".into());
            }
            let val = data[pos] != 0;
            Ok((Operand::boolean(val), pos + 1))
        }
        4 => {
            // string immediate
            if pos + 2 > data.len() {
                return Err("读取 string length 越界".into());
            }
            let len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
            let pos = pos + 2;
            if pos + len > data.len() {
                return Err("读取 string 越界".into());
            }
            let s = String::from_utf8_lossy(&data[pos..pos + len]).into_owned();
            Ok((Operand::string(s), pos + len))
        }
        _ => Err(format!("未知的 operand kind: {}", kind)),
    }
}

fn read_optional_operand(data: &[u8], pos: usize) -> BytecodeResult<(Option<Operand>, usize)> {
    if pos >= data.len() {
        return Err("读取 operand flag 越界".into());
    }
    let has = data[pos] == 1;
    let pos = pos + 1;

    if !has {
        return Ok((None, pos));
    }

    let (op, pos) = read_operand(data, pos)?;
    Ok((Some(op), pos))
}

fn value_type_to_u8(vt: ValueType) -> u8 {
    match vt {
        ValueType::Int => 0,
        ValueType::Float => 1,
        ValueType::Bool => 2,
        ValueType::String => 3,
        ValueType::Object => 4,
    }
}

fn u8_to_value_type(v: u8) -> ValueType {
    match v {
        0 => ValueType::Int,
        1 => ValueType::Float,
        2 => ValueType::Bool,
        3 => ValueType::String,
        4 => ValueType::Object,
        _ => ValueType::Int,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::Span;

    #[test]
    fn test_roundtrip_single_method() {
        let codes = vec![
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
        ];

        let method = CompiledMethod {
            name: "test".into(),
            codes,
            local_count: 1,
        };

        let data = serialize(&[method.clone()]).unwrap();
        let deserialized = deserialize(&data).unwrap();

        assert_eq!(deserialized.len(), 1);
        assert_eq!(deserialized[0].name, "test");
        assert_eq!(deserialized[0].codes.len(), 2);
        assert_eq!(deserialized[0].local_count, 1);
    }

    #[test]
    fn test_roundtrip_binary_operation() {
        let codes = vec![CodeWithSpan::new(
            IntermediateCode::binary(
                IntermediateOperator::IntAdd,
                Operand::Address(Address::new(ValueType::Int, 0)),
                Operand::int(10),
                Address::new(ValueType::Int, 2),
            ),
            crate::diagnostics::Span::dummy(),
        )];

        let method = CompiledMethod {
            name: "add".into(),
            codes,
            local_count: 3,
        };

        let data = serialize(&[method]).unwrap();
        let deserialized = deserialize(&data).unwrap();
        assert_eq!(deserialized[0].codes.len(), 1);
    }

    #[test]
    fn test_roundtrip_multiple_methods() {
        let m1 = CompiledMethod {
            name: "foo".into(),
            codes: vec![],
            local_count: 0,
        };
        let m2 = CompiledMethod {
            name: "bar".into(),
            codes: vec![],
            local_count: 0,
        };

        let data = serialize(&[m1, m2]).unwrap();
        let deserialized = deserialize(&data).unwrap();
        assert_eq!(deserialized.len(), 2);
    }

    #[test]
    fn test_invalid_magic() {
        let data = vec![0, 0, 0, 0, 1, 0, 0, 0];
        assert!(deserialize(&data).is_err());
    }

    #[test]
    fn test_roundtrip_module() {
        let class = CompiledClass {
            class_type: GorgeType::class("MyClass", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec!["IFoo".into()],
            field_counts: TypeCount { int_count: 2, ..TypeCount::zero() },
            methods: vec![CompiledMethod {
                name: "test".into(),
                codes: vec![],
                local_count: 0,
            }],
            constructors: vec![],
            injector_fields: vec![],
            delegate_impls: vec![],
        };
        let module = CompiledModule { version: 2, classes: vec![class] };
        let data = serialize_module(&module).unwrap();
        let deserialized = deserialize_module(&data).unwrap();
        assert_eq!(deserialized.classes.len(), 1);
        assert_eq!(deserialized.classes[0].methods.len(), 1);
        assert_eq!(deserialized.classes[0].super_interfaces.len(), 1);
    }

    /// 验证包含注入器字段的类可正确序列化/反序列化
    #[test]
    fn test_roundtrip_with_injector_fields() {
        let class = CompiledClass {
            class_type: GorgeType::class("WithInjector", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount { int_count: 1, ..TypeCount::zero() },
            methods: vec![CompiledMethod {
                name: "test".into(),
                codes: vec![],
                local_count: 0,
            }],
            constructors: vec![],
            injector_fields: vec![
                InjectorFieldDef { name: "hitTime".into(), value_type: ValueType::Float, has_default: true },
                InjectorFieldDef { name: "position".into(), value_type: ValueType::Object, has_default: false },
            ],
            delegate_impls: vec![],
        };
        let module = CompiledModule { version: 2, classes: vec![class] };
        let data = serialize_module(&module).unwrap();
        let deserialized = deserialize_module(&data).unwrap();
        assert_eq!(deserialized.classes.len(), 1);
        assert_eq!(deserialized.classes[0].injector_fields.len(), 2);
        assert_eq!(deserialized.classes[0].injector_fields[0].name, "hitTime");
        assert!(deserialized.classes[0].injector_fields[0].has_default);
        assert_eq!(deserialized.classes[0].injector_fields[1].name, "position");
        assert!(!deserialized.classes[0].injector_fields[1].has_default);
    }

    /// 验证含委托实现的类可正确序列化/反序列化
    #[test]
    fn test_roundtrip_with_delegate_impls() {
        let delegate_ir = vec![
            CodeWithSpan::new(
                IntermediateCode::new(
                    IntermediateOperator::ReturnInt,
                    Operand::int(42),
                    None, None,
                ),
                Span::new(0, 10, 1, 1, 0),
            ),
        ];
        let class = CompiledClass {
            class_type: GorgeType::class("LambdaHost", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![],
            constructors: vec![],
            injector_fields: vec![],
            delegate_impls: vec![
                DelegateImpl {
                    param_types: vec![ValueType::Int],
                    return_type: ValueType::Int,
                    body_ir: delegate_ir.clone(),
                    captured_var_names: vec!["x".into()],
                    outer_value_count: 1,
                },
            ],
        };
        let module = CompiledModule { version: 2, classes: vec![class] };
        let data = serialize_module(&module).unwrap();
        let deserialized = deserialize_module(&data).unwrap();
        assert_eq!(deserialized.classes.len(), 1);
        assert_eq!(deserialized.classes[0].delegate_impls.len(), 1);
        assert_eq!(deserialized.classes[0].delegate_impls[0].param_types.len(), 1);
        assert_eq!(deserialized.classes[0].delegate_impls[0].return_type, ValueType::Int);
        assert_eq!(deserialized.classes[0].delegate_impls[0].captured_var_names, vec!["x"]);
        assert_eq!(deserialized.classes[0].delegate_impls[0].body_ir.len(), 1);
    }

    /// 验证实际 .gorge 文件产物可正确反序列化
    #[test]
    fn test_deserialize_compiled_test1() {
        let data = std::fs::read("../test_output/Test1.gorge").unwrap();
        let module = deserialize_module(&data).unwrap();
        assert_eq!(module.classes.len(), 1, "Test1 应有 1 个类");
        let cls = &module.classes[0];
        assert_eq!(cls.class_type.full_name(), "Test1");
        assert!(!cls.is_native);
        assert_eq!(cls.methods.len(), 1);
        assert_eq!(cls.methods[0].name, "DoTest");
        assert!(!cls.methods[0].codes.is_empty(), "DoTest 应有 IR 指令");
    }

    #[test]
    fn test_deserialize_compiled_test9() {
        let data = std::fs::read("../test_output/Test9.gorge").unwrap();
        let module = deserialize_module(&data).unwrap();
        assert_eq!(module.classes.len(), 5, "Test9 应有 5 个类: Test9, Test9NInner, Test9N, Test9Inner, Test9A");
        let names: Vec<String> = module.classes.iter().map(|c| c.class_type.full_name()).collect();
        assert!(names.iter().any(|n| n == "Test9"), "应包含 Test9");
        assert!(names.iter().any(|n| n == "Test9A"), "应包含 Test9A");
        assert!(names.iter().any(|n| n == "Test9N"), "应包含 Test9N");
        assert!(names.iter().any(|n| n == "Test9Inner"), "应包含 Test9Inner");
        assert!(names.iter().any(|n| n == "Test9NInner"), "应包含 Test9NInner");
    }

    #[test]
    fn test_deserialize_compiled_test10() {
        let data = std::fs::read("../test_output/Test10.gorge").unwrap();
        let module = deserialize_module(&data).unwrap();
        assert_eq!(module.classes.len(), 1);
        assert_eq!(module.classes[0].class_type.full_name(), "Test10");
        assert_eq!(module.classes[0].methods.len(), 1);
    }

    #[test]
    fn test_deserialize_compiled_test12() {
        let data = std::fs::read("../test_output/Test12.gorge").unwrap();
        let module = deserialize_module(&data).unwrap();
        assert_eq!(module.classes.len(), 1);
        assert_eq!(module.classes[0].class_type.full_name(), "Test12");
        assert!(module.classes[0].delegate_impls.len() >= 1, "应有至少 1 个委托");
    }

    #[test]
    fn test_deserialize_compiled_test7() {
        let data = std::fs::read("../test_output/Test7.gorge").unwrap();
        let module = deserialize_module(&data).unwrap();
        assert_eq!(module.classes.len(), 1);
        assert_eq!(module.classes[0].class_type.full_name(), "Test7");
        assert_eq!(module.classes[0].methods.len(), 2, "Test7: DoTest + InstanceDoTest");
        // InstanceDoTest 内的 3 个 lambda → 3 个委托
        assert_eq!(module.classes[0].delegate_impls.len(), 3, "Test7 应有 3 个委托");
    }

    /// Test4: 单类 + 构造函数 + this.field + 实例方法 + 静态方法
    #[test]
    fn test_deserialize_compiled_test4() {
        let data = std::fs::read("../test_output/Test4.gorge").unwrap();
        let module = deserialize_module(&data).unwrap();
        assert_eq!(module.classes.len(), 1);
        let cls = &module.classes[0];
        assert_eq!(cls.class_type.full_name(), "Test4");
        assert!(!cls.is_native);
        assert_eq!(cls.field_counts.int_count, 3, "Test4: counter + increasment + selfIncreasement");
        // Test4(): constructor + SelfIncreasement() + DoTest() → 3 方法
        assert!(cls.methods.len() >= 2, "至少应生成 constructor + DoTest");
    }

    /// Test5: 多类 + 继承 + super()
    #[test]
    fn test_deserialize_compiled_test5() {
        let data = std::fs::read("../test_output/Test5.gorge").unwrap();
        let module = deserialize_module(&data).unwrap();
        assert_eq!(module.classes.len(), 4, "Test5: Test5 + Test5A + Test5B + Test5C");
        let test5a = module.classes.iter().find(|c| c.class_type.full_name() == "Test5A").unwrap();
        assert_eq!(test5a.field_counts.int_count, 1, "Test5A: valueA");
        let test5b = module.classes.iter().find(|c| c.class_type.full_name() == "Test5B").unwrap();
        assert_eq!(test5b.super_class_name, Some("Test5A".into()));
        // Test5B 继承 Test5A 的 valueA，自身 valueB
        assert!(test5b.field_counts.int_count >= 1);
    }

    /// Test6: 接口 + 多继承
    #[test]
    fn test_deserialize_compiled_test6() {
        let data = std::fs::read("../test_output/Test6.gorge").unwrap();
        let module = deserialize_module(&data).unwrap();
        assert_eq!(module.classes.len(), 4, "Test6: Test6 + Test6I(接口?) + Test6A + Test6B + Test6C");

        let test6a = module.classes.iter().find(|c| c.class_type.full_name() == "Test6A").unwrap();
        assert!(!test6a.super_interfaces.is_empty(), "Test6A 应实现 Test6I");
        assert!(test6a.super_interfaces.iter().any(|i| i == "Test6I"));
    }

    /// Test11: 注入器构造 + 多类继承
    #[test]
    fn test_deserialize_compiled_test11() {
        let data = std::fs::read("../test_output/Test11.gorge").unwrap();
        let module = deserialize_module(&data).unwrap();
        assert_eq!(module.classes.len(), 4, "Test11: Test11 + Test11A + Test11B + Test11C");
        let test11a = module.classes.iter().find(|c| c.class_type.full_name() == "Test11A").unwrap();
        assert_eq!(test11a.field_counts.int_count, 1, "Test11A: value");
        let test11b = module.classes.iter().find(|c| c.class_type.full_name() == "Test11B").unwrap();
        assert_eq!(test11b.super_class_name, Some("Test11A".into()));
        let test11c = module.classes.iter().find(|c| c.class_type.full_name() == "Test11C").unwrap();
        assert_eq!(test11c.super_class_name, Some("Test11A".into()));
    }

    // ==================== 全量产物结构验证 ====================

    macro_rules! verify_class {
        ($module:expr, $name:expr, $fields:expr, $super:expr) => {
            let cls = $module.classes.iter()
                .find(|c| c.class_type.full_name() == $name)
                .unwrap_or_else(|| panic!("未找到类 `{}`", $name));
            assert_eq!(cls.field_counts.int_count, $fields.0, "{}: int 字段数", $name);
            assert_eq!(cls.field_counts.float_count, $fields.1, "{}: float 字段数", $name);
            assert_eq!(cls.field_counts.string_count, $fields.2, "{}: string 字段数", $name);
            assert_eq!(cls.field_counts.object_count, $fields.3, "{}: object 字段数", $name);
            assert!(cls.methods.len() >= 1, "{}: 应有至少 1 个方法", $name);
            assert_eq!(cls.super_class_name.as_deref(), $super, "{}: 父类", $name);
        };
    }

    #[test]
    fn test_all_compiled_artifacts() {
        // Test1: 单类 + 静态方法
        let m1 = deserialize_module(&std::fs::read("../test_output/Test1.gorge").unwrap()).unwrap();
        assert_eq!(m1.classes.len(), 1);
        verify_class!(m1, "Test1", (0,0,0,0), None);
        assert_eq!(m1.classes[0].methods[0].name, "DoTest");
        assert!(!m1.classes[0].methods[0].codes.is_empty());

        // Test2: 单类 + 静态方法 + 静态调用
        let m2 = deserialize_module(&std::fs::read("../test_output/Test2.gorge").unwrap()).unwrap();
        assert_eq!(m2.classes.len(), 1);
        verify_class!(m2, "Test2", (0,0,0,0), None);

        // Test3: 单类 + 静态方法 + 递归静态调用
        let m3 = deserialize_module(&std::fs::read("../test_output/Test3.gorge").unwrap()).unwrap();
        assert_eq!(m3.classes.len(), 1);
        verify_class!(m3, "Test3", (0,0,0,0), None);

        // Test4: 单类 + 3 int 字段 + 构造函数（验证构造方法序列化）
        let m4 = deserialize_module(&std::fs::read("../test_output/Test4.gorge").unwrap()).unwrap();
        assert_eq!(m4.classes.len(), 1);
        assert_eq!(m4.classes[0].field_counts.int_count, 3);
        assert!(m4.classes[0].methods.len() >= 1, "Test4 应有普通方法");
        assert!(m4.classes[0].constructors.len() >= 1, "Test4 应有构造方法");
        for c in &m4.classes[0].constructors {
            assert!(!c.codes.is_empty(), "构造方法 {} 应有 IR 指令", c.name);
        }

        // Test5: 4 类 + 继承链 Test5C → Test5B → Test5A
        let m5 = deserialize_module(&std::fs::read("../test_output/Test5.gorge").unwrap()).unwrap();
        assert_eq!(m5.classes.len(), 4);
        verify_class!(m5, "Test5", (0,0,0,0), None);
        verify_class!(m5, "Test5A", (1,0,0,0), None);
        verify_class!(m5, "Test5B", (1,0,0,0), Some("Test5A"));
        verify_class!(m5, "Test5C", (1,0,0,0), Some("Test5B"));

        // Test6: 接口 Test6I + 多继承
        let m6 = deserialize_module(&std::fs::read("../test_output/Test6.gorge").unwrap()).unwrap();
        assert_eq!(m6.classes.len(), 4);
        let test6a = m6.classes.iter().find(|c| c.class_type.full_name() == "Test6A").unwrap();
        assert!(test6a.super_interfaces.iter().any(|i| i == "Test6I"), "Test6A 应实现 Test6I");

        // Test7: Lambda + delegate: 3 个委托
        let m7 = deserialize_module(&std::fs::read("../test_output/Test7.gorge").unwrap()).unwrap();
        assert_eq!(m7.classes.len(), 1);
        assert_eq!(m7.classes[0].field_counts.int_count, 1);
        assert_eq!(m7.classes[0].delegate_impls.len(), 3, "Test7 应有 3 个委托");

        // Test8: 3 类（Test8 + native Test8N + Test8A 继承）
        let m8 = deserialize_module(&std::fs::read("../test_output/Test8.gorge").unwrap()).unwrap();
        assert_eq!(m8.classes.len(), 3, "Test8: Test8 + Test8N + Test8A");
        let test8n = m8.classes.iter().find(|c| c.class_type.full_name() == "Test8N").unwrap();
        assert!(test8n.is_native, "Test8N 应为 native");
        let test8a = m8.classes.iter().find(|c| c.class_type.full_name() == "Test8A").unwrap();
        assert_eq!(test8a.super_class_name.as_deref(), Some("Test8N"), "Test8A 父类为 Test8N");

        // Test9: 5 类 + Test9A → Test9N 继承
        let m9 = deserialize_module(&std::fs::read("../test_output/Test9.gorge").unwrap()).unwrap();
        assert_eq!(m9.classes.len(), 5);
        let test9a = m9.classes.iter().find(|c| c.class_type.full_name() == "Test9A").unwrap();
        assert_eq!(test9a.super_class_name, Some("Test9N".into()));

        // Test10: 数组构造 + 注入器数组
        let m10 = deserialize_module(&std::fs::read("../test_output/Test10.gorge").unwrap()).unwrap();
        assert_eq!(m10.classes.len(), 1);
        verify_class!(m10, "Test10", (0,0,0,0), None);

        // Test11: 4 类 + injector 构造 + Test11B/11C → Test11A
        // 注：构造函数在字节码中尚未序列化（collect_classes 仅导出 Method，不含 Constructor）
        let m11 = deserialize_module(&std::fs::read("../test_output/Test11.gorge").unwrap()).unwrap();
        assert_eq!(m11.classes.len(), 4);
        let test11a = m11.classes.iter().find(|c| c.class_type.full_name() == "Test11A").unwrap();
        assert_eq!(test11a.field_counts.int_count, 1);
        let test11b = m11.classes.iter().find(|c| c.class_type.full_name() == "Test11B").unwrap();
        assert_eq!(test11b.super_class_name.as_deref(), Some("Test11A"));
        let test11c = m11.classes.iter().find(|c| c.class_type.full_name() == "Test11C").unwrap();
        assert_eq!(test11c.super_class_name.as_deref(), Some("Test11A"));

        // Test12: 嵌套 Lambda
        let m12 = deserialize_module(&std::fs::read("../test_output/Test12.gorge").unwrap()).unwrap();
        assert_eq!(m12.classes.len(), 1);
        verify_class!(m12, "Test12", (0,0,0,0), None);
        assert!(m12.classes[0].delegate_impls.len() >= 1, "Test12 应有委托");
    }
}
