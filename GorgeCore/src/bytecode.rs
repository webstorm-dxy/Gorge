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
    /// 继承编号冻结（B-3）：本类方法（混合空间）的全局起始编号
    pub method_start_id: usize,
    /// 含继承的方法总数
    pub method_count_total: usize,
    /// 本类构造方法的全局起始编号
    pub constructor_start_id: usize,
    /// 重写映射：被重写父类方法全局 ID → 本类重写方法全局 ID
    pub method_override_id: Vec<(usize, usize)>,
    /// 本类实例字段各值类型的起始索引（父类字段总数），顺序 int/float/bool/string/object
    pub field_start_counts: [usize; 5],
    /// 接口方法实现映射（F1）：(接口全名, [接口方法本地ID → 类方法全局ID])
    pub interface_method_impl_id: Vec<(String, Vec<usize>)>,
    /// 注入器常量池（G2）：编译时求值的注入器字面量
    pub injector_constants: Vec<InjectorConstantDef>,
    /// 注入器构造方法实现映射（G3）：注入器构造方法本地ID → 全局构造方法ID
    pub injector_constructor_impl_id: Vec<usize>,
    /// 字段初始化器（Phase P）：每个有初始值的字段编译为独立的 IR 可执行体
    /// 构造流程中先于构造方法体执行，对齐 C# CompiledGorgeClass.FieldInitializerImplementations
    pub field_initializers: Vec<CompiledFieldInitializer>,
    /// 类注解（Phase Q3）：(注解名, 可选的泛型类型字符串)
    /// 对齐 C# ClassDeclaration.Annotations，存储类声明的所有注解信息
    pub annotations: Vec<(String, Option<String>)>,
}

/// 字段初始化器编译产物（Phase P）
///
/// 对齐 C# CompiledFieldInitializerImplementation，每个有初始值的非 native 字段
/// 编译为一个独立的可执行体，在构造方法体之前由 VM 执行。
#[derive(Debug, Clone)]
pub struct CompiledFieldInitializer {
    /// 目标字段在各值类型分组内的索引
    pub field_index: usize,
    /// 字段的值类型
    pub value_type: ValueType,
    /// 初始化器 IR 所需的局部变量数
    pub local_count: usize,
    /// 初始化器 IR 代码
    pub codes: Vec<CodeWithSpan>,
}

/// 委托实现元数据
#[derive(Debug, Clone)]
pub struct DelegateImpl {
    pub param_types: Vec<ValueType>,
    pub return_type: ValueType,
    pub body_ir: Vec<CodeWithSpan>,
    pub captured_var_names: Vec<String>,
    pub outer_value_count: usize,
    /// 静态委托标记（I-B）：无自由变量时为 true，可编译时常量化
    pub is_static: bool,
}

/// 注入器字段定义（序列化用）
#[derive(Debug, Clone)]
pub struct InjectorFieldDef {
    pub name: String,
    pub value_type: ValueType,
    pub has_default: bool,
    /// 默认值常量（G4）：@Inject(default = expr) 的编译时常量值
    pub default_value: Option<InjectorConstField>,
}

/// 注入器常量定义（编译时构造的注入器常量，G2）
///
/// 每个常量对应源码中的一个注入器字面量（如 `Vector2:{x:1.0,y:2.0}`），
/// 字段值已求值为常量化，运行时从常量池实例化。
#[derive(Debug, Clone)]
pub struct InjectorConstantDef {
    /// 注入目标的类全名（如 "Vector2"）
    pub class_name: String,
    /// 字段名 → 常量值
    pub fields: Vec<InjectorConstField>,
}

/// 注入器常量字段
///
/// 表示注入器常量中的一个字段值，支持基本类型字面量和嵌套的注入器对象/数组。
/// `Object` 变体存储运行时对象 ID（填充阶段由 VM 分配）。
#[derive(Debug, Clone)]
pub enum InjectorConstField {
    Int(String, i64),
    Float(String, f64),
    Bool(String, bool),
    String(String, String),
    /// 对象引用（运行时对象 ID，由 VM 在常量实例化时填充）
    Object(String, usize),
    /// 嵌套的注入器对象常量 `{ field: val, ... }`
    InjectObject(String, Vec<InjectorConstField>),
    /// 注入器数组常量 `[elem1, elem2, ...]`
    Array(Vec<InjectorConstField>),
}

/// 字节码魔数："GORG"
const MAGIC: [u8; 4] = [b'G', b'O', b'R', b'G'];
/// 字节码格式版本
/// V1: 基础方法列表
/// V2: 类元数据（字段计数/父类/接口/注入器字段/委托/方法重写/接口映射）
/// V3: 注入器构造方法映射 + 字段初始化器
const VERSION: u16 = 3;

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
        IntermediateOperator::IntOpposite => 29,
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
        IntermediateOperator::ObjectCastToObject => 67,
        IntermediateOperator::Jump(_) => 70,
        IntermediateOperator::JumpIfFalse(_) => 71,
        IntermediateOperator::JumpIfTrue(_) => 72,
        IntermediateOperator::InvokeInstance(_) => 80,
        IntermediateOperator::InvokeStatic(_) => 81,
        IntermediateOperator::InvokeInterface(_) => 82,
        IntermediateOperator::InvokeDelegate(_) => 83,
        IntermediateOperator::InvokeConstructor(_) => 84,
        IntermediateOperator::DoConstruct(_) => 90,
        IntermediateOperator::InvokeSuperConstructor(_) => 91,
        IntermediateOperator::LoadInjectorConstant(_) => 92,
        IntermediateOperator::InvokeArrayConstructor => 93,
        IntermediateOperator::InvokeInjectorConstructor(_) => 94,
        IntermediateOperator::FloatOpposite => 95,
        IntermediateOperator::FloatMod => 96,
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
         29 => IntermediateOperator::IntOpposite,
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
        67 => IntermediateOperator::ObjectCastToObject,
        70 => IntermediateOperator::Jump(extra as usize),
        71 => IntermediateOperator::JumpIfFalse(extra as usize),
        72 => IntermediateOperator::JumpIfTrue(extra as usize),
        80 => IntermediateOperator::InvokeInstance(extra as usize),
        81 => IntermediateOperator::InvokeStatic(extra as usize),
        82 => IntermediateOperator::InvokeInterface(extra as usize),
        83 => IntermediateOperator::InvokeDelegate(extra as usize),
        84 => IntermediateOperator::InvokeConstructor(extra as usize),
        90 => IntermediateOperator::DoConstruct(extra as usize),
        91 => IntermediateOperator::InvokeSuperConstructor(extra as usize),
         92 => IntermediateOperator::LoadInjectorConstant(extra as usize),
         93 => IntermediateOperator::InvokeArrayConstructor,
         94 => IntermediateOperator::InvokeInjectorConstructor(extra as usize),
         95 => IntermediateOperator::FloatOpposite,
         96 => IntermediateOperator::FloatMod,
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

    // 写入文件头（V1 格式始终用版本 1）
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&1u16.to_le_bytes());
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
    buf.extend_from_slice(&3u16.to_le_bytes());
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
        // G4: 若存在默认值，序列化常量字段
        if let Some(dv) = &field.default_value {
            serialize_const_fields(&[dv.clone()], buf);
        } else {
            buf.extend_from_slice(&0u16.to_le_bytes());
        }
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

    // 继承编号冻结（B-3）
    buf.extend_from_slice(&(class.method_start_id as u32).to_le_bytes());
    buf.extend_from_slice(&(class.method_count_total as u32).to_le_bytes());
    buf.extend_from_slice(&(class.constructor_start_id as u32).to_le_bytes());
    buf.extend_from_slice(&(class.method_override_id.len() as u16).to_le_bytes());
    for (from, to) in &class.method_override_id {
        buf.extend_from_slice(&(*from as u32).to_le_bytes());
        buf.extend_from_slice(&(*to as u32).to_le_bytes());
    }
    for c in &class.field_start_counts {
        buf.extend_from_slice(&(*c as u32).to_le_bytes());
    }

    // 接口方法实现映射（F1）
    buf.extend_from_slice(&(class.interface_method_impl_id.len() as u16).to_le_bytes());
    for (iface_name, impl_ids) in &class.interface_method_impl_id {
        let name_bytes = iface_name.as_bytes();
        buf.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        buf.extend_from_slice(name_bytes);
        buf.extend_from_slice(&(impl_ids.len() as u16).to_le_bytes());
        for id in impl_ids {
            buf.extend_from_slice(&(*id as u32).to_le_bytes());
        }
    }

    // 注入器常量池（G2）
    buf.extend_from_slice(&(class.injector_constants.len() as u16).to_le_bytes());
    for c in &class.injector_constants {
        let cn = c.class_name.as_bytes();
        buf.extend_from_slice(&(cn.len() as u16).to_le_bytes());
        buf.extend_from_slice(cn);
        serialize_const_fields(&c.fields, buf);
    }

    // 注入器构造方法实现映射（G3）
    buf.extend_from_slice(&(class.injector_constructor_impl_id.len() as u16).to_le_bytes());
    for id in &class.injector_constructor_impl_id {
        buf.extend_from_slice(&(*id as u32).to_le_bytes());
    }

    // 字段初始化器（Phase P）
    buf.extend_from_slice(&(class.field_initializers.len() as u16).to_le_bytes());
    for init in &class.field_initializers {
        buf.extend_from_slice(&(init.field_index as u32).to_le_bytes());
        buf.push(value_type_to_u8(init.value_type));
        buf.extend_from_slice(&(init.local_count as u32).to_le_bytes());
        serialize_method(&CompiledMethod {
            name: String::new(),
            codes: init.codes.clone(),
            local_count: init.local_count,
        }, buf)?;
    }

    // 类注解（Phase Q3）
    buf.extend_from_slice(&(class.annotations.len() as u16).to_le_bytes());
    for (name, generic) in &class.annotations {
        let nb = name.as_bytes();
        buf.extend_from_slice(&(nb.len() as u16).to_le_bytes());
        buf.extend_from_slice(nb);
        if let Some(gt) = generic {
            let gb = gt.as_bytes();
            buf.extend_from_slice(&(gb.len() as u16).to_le_bytes());
            buf.extend_from_slice(gb);
        } else {
            buf.extend_from_slice(&0u16.to_le_bytes());
        }
    }

    Ok(())
}

/// 序列化常量字段列表（含嵌套注入器对象和数组的递归序列化）
fn serialize_const_fields(fields: &[InjectorConstField], buf: &mut Vec<u8>) {
    buf.extend_from_slice(&(fields.len() as u16).to_le_bytes());
    for f in fields {
        match f {
            InjectorConstField::Int(name, v) => {
                buf.push(0); let nb = name.as_bytes(); buf.extend_from_slice(&(nb.len() as u16).to_le_bytes()); buf.extend_from_slice(nb);
                buf.extend_from_slice(&v.to_le_bytes());
            }
            InjectorConstField::Float(name, v) => {
                buf.push(1); let nb = name.as_bytes(); buf.extend_from_slice(&(nb.len() as u16).to_le_bytes()); buf.extend_from_slice(nb);
                buf.extend_from_slice(&v.to_le_bytes());
            }
            InjectorConstField::Bool(name, v) => {
                buf.push(2); let nb = name.as_bytes(); buf.extend_from_slice(&(nb.len() as u16).to_le_bytes()); buf.extend_from_slice(nb);
                buf.push(if *v { 1u8 } else { 0u8 });
            }
            InjectorConstField::String(name, v) => {
                buf.push(3); let nb = name.as_bytes(); buf.extend_from_slice(&(nb.len() as u16).to_le_bytes()); buf.extend_from_slice(nb);
                let vb = v.as_bytes(); buf.extend_from_slice(&(vb.len() as u16).to_le_bytes()); buf.extend_from_slice(vb);
            }
            InjectorConstField::Object(name, v) => {
                buf.push(4); let nb = name.as_bytes(); buf.extend_from_slice(&(nb.len() as u16).to_le_bytes()); buf.extend_from_slice(nb);
                buf.extend_from_slice(&(*v as u32).to_le_bytes());
            }
            InjectorConstField::InjectObject(name, nested) => {
                buf.push(5); let nb = name.as_bytes(); buf.extend_from_slice(&(nb.len() as u16).to_le_bytes()); buf.extend_from_slice(nb);
                serialize_const_fields(nested, buf);
            }
            InjectorConstField::Array(elements) => {
                buf.push(6);
                serialize_const_fields(elements, buf);
            }
        }
    }
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
        | IntermediateOperator::InvokeSuperConstructor(v)
        | IntermediateOperator::LoadInjectorConstant(v)
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
        | IntermediateOperator::InvokeDelegate(v)
        | IntermediateOperator::InvokeInjectorConstructor(v) => *v as u16,
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

/// 反序列化常量字段列表（支持嵌套注入器对象和数组的递归解析）
fn deserialize_const_fields(data: &[u8], pos: &mut usize) -> BytecodeResult<Vec<InjectorConstField>> {
    if *pos + 2 > data.len() { return Err("读取字段计数越界".into()); }
    let count = u16::from_le_bytes([data[*pos], data[*pos+1]]) as usize;
    *pos += 2;
    let mut fields = Vec::with_capacity(count);
    for _ in 0..count {
        if *pos + 1 > data.len() { return Err("读取字段类型标签越界".into()); }
        let tag = data[*pos]; *pos += 1;
        // Array 类型的元素没有字段名，tag 6 跳过字段名读取
        let fname = if tag != 6 {
            if *pos + 2 > data.len() { return Err("读取字段名长度越界".into()); }
            let fnlen = u16::from_le_bytes([data[*pos], data[*pos+1]]) as usize; *pos += 2;
            if *pos + fnlen > data.len() { return Err("读取字段名越界".into()); }
            let name = String::from_utf8_lossy(&data[*pos..*pos+fnlen]).to_string();
            *pos += fnlen;
            name
        } else {
            String::new()
        };
        match tag {
            0 => { if *pos + 8 > data.len() { return Err("读取int字段值越界".into()); } let v = i64::from_le_bytes([data[*pos],data[*pos+1],data[*pos+2],data[*pos+3],data[*pos+4],data[*pos+5],data[*pos+6],data[*pos+7]]); *pos += 8; fields.push(InjectorConstField::Int(fname, v)); }
            1 => { if *pos + 8 > data.len() { return Err("读取float字段值越界".into()); } let v = f64::from_le_bytes([data[*pos],data[*pos+1],data[*pos+2],data[*pos+3],data[*pos+4],data[*pos+5],data[*pos+6],data[*pos+7]]); *pos += 8; fields.push(InjectorConstField::Float(fname, v)); }
            2 => { if *pos + 1 > data.len() { return Err("读取bool字段值越界".into()); } let v = data[*pos] != 0; *pos += 1; fields.push(InjectorConstField::Bool(fname, v)); }
            3 => { if *pos + 2 > data.len() { return Err("读取string字段长度越界".into()); } let slen = u16::from_le_bytes([data[*pos],data[*pos+1]]) as usize; *pos += 2; if *pos + slen > data.len() { return Err("读取string字段越界".into()); } let sv = String::from_utf8_lossy(&data[*pos..*pos+slen]).to_string(); *pos += slen; fields.push(InjectorConstField::String(fname, sv)); }
            4 => { if *pos + 4 > data.len() { return Err("读取object字段值越界".into()); } let v = u32::from_le_bytes([data[*pos],data[*pos+1],data[*pos+2],data[*pos+3]]) as usize; *pos += 4; fields.push(InjectorConstField::Object(fname, v)); }
            5 => { let nested = deserialize_const_fields(data, pos)?; fields.push(InjectorConstField::InjectObject(fname, nested)); }
            6 => { let elements = deserialize_const_fields(data, pos)?; fields.push(InjectorConstField::Array(elements)); }
            _ => return Err("未知注入器常量字段类型".into()),
        }
    }
    Ok(fields)
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
            method_start_id: 0,
            method_count_total: 0,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],
            injector_constants: vec![],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![],
        };
        Ok(CompiledModule { version: 1, classes: vec![class] })
    } else {
        let class_count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;
        let mut classes = Vec::with_capacity(class_count);
        for _ in 0..class_count {
            let (class, new_pos) = deserialize_compiled_class(data, pos, version)?;
            classes.push(class);
            pos = new_pos;
        }
        Ok(CompiledModule { version, classes })
    }
}

fn deserialize_compiled_class(data: &[u8], mut pos: usize, _version: u16) -> BytecodeResult<(CompiledClass, usize)> {
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
        // G4: 读取默认值常量字段
        let default_value = if has_default {
            let dfs = deserialize_const_fields(data, &mut pos)?;
            dfs.into_iter().next()
        } else {
            // 跳过字段计数
            if pos + 2 <= data.len() { pos += 2; }
            None
        };
        injector_fields.push(InjectorFieldDef { name, value_type: vt, has_default, default_value });
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
            is_static: false,
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

    // 继承编号冻结（B-3）
    if pos + 4 > data.len() { return Err("读取 method_start_id 越界".into()); }
    let method_start_id = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
    pos += 4;
    if pos + 4 > data.len() { return Err("读取 method_count_total 越界".into()); }
    let method_count_total = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
    pos += 4;
    if pos + 4 > data.len() { return Err("读取 constructor_start_id 越界".into()); }
    let constructor_start_id = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
    pos += 4;
    if pos + 2 > data.len() { return Err("读取重写映射计数越界".into()); }
    let override_count = u16::from_le_bytes([data[pos], data[pos+1]]) as usize;
    pos += 2;
    let mut method_override_id = Vec::with_capacity(override_count);
    for _ in 0..override_count {
        if pos + 8 > data.len() { return Err("读取重写映射项越界".into()); }
        let from = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
        let to = u32::from_le_bytes([data[pos+4], data[pos+5], data[pos+6], data[pos+7]]) as usize;
        method_override_id.push((from, to));
        pos += 8;
    }
    let mut field_start_counts = [0usize; 5];
    for slot in &mut field_start_counts {
        if pos + 4 > data.len() { return Err("读取字段起始计数越界".into()); }
        *slot = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
        pos += 4;
    }

    // 接口方法实现映射（F1）
    if pos + 2 > data.len() { return Err("读取接口映射计数越界".into()); }
    let iface_map_count = u16::from_le_bytes([data[pos], data[pos+1]]) as usize;
    pos += 2;
    let mut interface_method_impl_id = Vec::with_capacity(iface_map_count);
    for _ in 0..iface_map_count {
        if pos + 2 > data.len() { return Err("读取接口名长度越界".into()); }
        let nlen = u16::from_le_bytes([data[pos], data[pos+1]]) as usize;
        pos += 2;
        if pos + nlen > data.len() { return Err("读取接口名越界".into()); }
        let iface_name = String::from_utf8_lossy(&data[pos..pos+nlen]).to_string();
        pos += nlen;
        if pos + 2 > data.len() { return Err("读取接口方法数越界".into()); }
        let mcount = u16::from_le_bytes([data[pos], data[pos+1]]) as usize;
        pos += 2;
        let mut impl_ids = Vec::with_capacity(mcount);
        for _ in 0..mcount {
            if pos + 4 > data.len() { return Err("读取接口方法实现ID越界".into()); }
            impl_ids.push(u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize);
            pos += 4;
        }
        interface_method_impl_id.push((iface_name, impl_ids));
    }

    // 注入器常量池（G2）
    if pos + 2 > data.len() { return Err("读取注入器常量计数越界".into()); }
    let inj_const_count = u16::from_le_bytes([data[pos], data[pos+1]]) as usize;
    pos += 2;
    let mut injector_constants = Vec::with_capacity(inj_const_count);
    for _ in 0..inj_const_count {
        if pos + 2 > data.len() { return Err("读取常量类名长度越界".into()); }
        let cnlen = u16::from_le_bytes([data[pos], data[pos+1]]) as usize;
        pos += 2;
        if pos + cnlen > data.len() { return Err("读取常量类名越界".into()); }
        let class_name = String::from_utf8_lossy(&data[pos..pos+cnlen]).to_string();
        pos += cnlen;
        let fields = deserialize_const_fields(data, &mut pos)?;
        injector_constants.push(InjectorConstantDef { class_name, fields });
    }

    // 注入器构造方法实现映射（G3）—— 仅 V3 及以上格式有此段
    let mut injector_constructor_impl_id = Vec::new();
    if _version >= 3 {
        let ic_count = u16::from_le_bytes([data[pos], data[pos+1]]) as usize;
        pos += 2;
        for _ in 0..ic_count {
            if pos + 4 > data.len() { break; }
            let id = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
            pos += 4;
            injector_constructor_impl_id.push(id);
        }
    }

    // 字段初始化器（Phase P）—— 仅 V3 及以上格式有此段
    let mut field_initializers = Vec::new();
    if _version >= 3 {
        if pos + 2 > data.len() { return Err("读取字段初始化器计数越界".into()); }
        let fi_count = u16::from_le_bytes([data[pos], data[pos+1]]) as usize;
        pos += 2;
        for _ in 0..fi_count {
            if pos + 9 > data.len() { return Err("读取字段初始化器头部越界".into()); }
            let field_index = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
            pos += 4;
            let value_type = u8_to_value_type(data[pos]);
            pos += 1;
            let local_count = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
            pos += 4;
            let (method, new_pos) = deserialize_method(data, pos)?;
            pos = new_pos;
            field_initializers.push(CompiledFieldInitializer {
                field_index,
                value_type,
                local_count,
                codes: method.codes,
            });
        }
    }

    // 类注解（Phase Q3）—— 仅 V3 及以上格式有此段
    let mut annotations = Vec::new();
    if _version >= 3 {
        if pos + 2 <= data.len() {
            let ann_count = u16::from_le_bytes([data[pos], data[pos+1]]) as usize;
            pos += 2;
            for _ in 0..ann_count {
                if pos + 2 > data.len() { break; }
                let nlen = u16::from_le_bytes([data[pos], data[pos+1]]) as usize;
                pos += 2;
                if pos + nlen > data.len() { break; }
                let name = String::from_utf8_lossy(&data[pos..pos+nlen]).to_string();
                pos += nlen;
                if pos + 2 > data.len() { break; }
                let glen = u16::from_le_bytes([data[pos], data[pos+1]]) as usize;
                pos += 2;
                let generic = if glen > 0 {
                    if pos + glen > data.len() { break; }
                    let gt = String::from_utf8_lossy(&data[pos..pos+glen]).to_string();
                    pos += glen;
                    Some(gt)
                } else {
                    None
                };
                annotations.push((name, generic));
            }
        }
    }

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
        method_start_id,
        method_count_total,
        constructor_start_id,
        method_override_id,
        field_start_counts,
        interface_method_impl_id,
        injector_constants,
        injector_constructor_impl_id,
        field_initializers,
        annotations,
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
    fn test_roundtrip_super_constructor() {
        // 验证 InvokeSuperConstructor 操作码与 right（父类名）序列化往返
        let codes = vec![CodeWithSpan::new(
            IntermediateCode::new(
                IntermediateOperator::InvokeSuperConstructor(0),
                Operand::int(1),
                Some(Operand::string("Animal")),
                None,
            ),
            crate::diagnostics::Span::dummy(),
        )];
        let method = CompiledMethod {
            name: "constructor".into(),
            codes,
            local_count: 2,
        };
        let data = serialize(&[method]).unwrap();
        let deserialized = deserialize(&data).unwrap();
        assert_eq!(deserialized[0].codes.len(), 1);
        let code = &deserialized[0].codes[0].code;
        assert!(matches!(code.operator, IntermediateOperator::InvokeSuperConstructor(0)));
        match &code.right {
            Some(Operand::Immediate(ImmediateValue::String(s))) => assert_eq!(s, "Animal"),
            _ => panic!("right 应为父类名字符串"),
        }
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
            method_start_id: 0,
            method_count_total: 0,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],                    injector_constants: vec![],                    injector_constructor_impl_id: vec![],                    field_initializers: vec![],                    annotations: vec![],        };
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
                InjectorFieldDef { name: "hitTime".into(), value_type: ValueType::Float, has_default: true, default_value: None },
                InjectorFieldDef { name: "position".into(), value_type: ValueType::Object, has_default: false, default_value: None },
            ],
            delegate_impls: vec![],
            method_start_id: 0,
            method_count_total: 0,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],                    injector_constants: vec![],                    injector_constructor_impl_id: vec![],                    field_initializers: vec![],                    annotations: vec![],        };
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
                    is_static: false,
                },
            ],
            method_start_id: 0,
            method_count_total: 0,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],                    injector_constants: vec![],                    injector_constructor_impl_id: vec![],                    field_initializers: vec![],                    annotations: vec![],        };
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
