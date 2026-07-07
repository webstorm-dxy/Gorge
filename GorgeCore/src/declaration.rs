use std::collections::HashMap;
use crate::types::{GorgeType, TypeCount};

/// 类声明的编译时元数据
#[derive(Debug, Clone)]
pub struct ClassDeclaration {
    pub class_type: GorgeType,
    pub is_native: bool,
    pub annotations: Vec<Annotation>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub static_methods: Vec<MethodInfo>,
    pub constructors: Vec<ConstructorInfo>,
    pub injector_fields: Vec<InjectorFieldInfo>,
    pub super_class: Option<Box<ClassDeclaration>>,
    pub super_interfaces: Vec<String>,
    pub field_type_count: TypeCount,
    pub method_count: usize,
    pub static_method_count: usize,
    pub constructor_count: usize,
    pub injector_field_type_count: TypeCount,
    pub injector_field_default_value_type_count: TypeCount,
    pub method_start_id: usize,
    pub constructor_start_id: usize,
    pub interface_method_impl_id: HashMap<String, Vec<usize>>,
    pub method_override_id: HashMap<usize, usize>,
}

/// 字段声明信息
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: GorgeType,
    pub is_static: bool,
    pub is_native: bool,
}

/// 方法声明信息
#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub return_type: GorgeType,
    pub parameters: Vec<ParameterInfo>,
    pub is_static: bool,
    pub is_native: bool,
    pub is_override: bool,
    pub is_abstract: bool,
}

/// 构造方法声明信息
#[derive(Debug, Clone)]
pub struct ConstructorInfo {
    pub parameters: Vec<ParameterInfo>,
    pub is_native: bool,
    pub is_injector: bool,
}

/// 参数信息
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub param_type: GorgeType,
}

/// 注入器字段信息
#[derive(Debug, Clone)]
pub struct InjectorFieldInfo {
    pub name: String,
    pub field_type: GorgeType,
    pub has_default_value: bool,
}

/// 注解信息
#[derive(Debug, Clone)]
pub struct Annotation {
    pub name: String,
    pub generic_type: Option<GorgeType>,
    pub arguments: Vec<String>,
}

/// 枚举定义
#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: String,
    pub full_name: String,
    pub values: Vec<EnumValue>,
}

/// 枚举值
#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: String,
    pub value: i64,
}

impl EnumDef {
    /// 按名称查找枚举值
    pub fn name_to_value(&self, name: &str) -> Option<i64> {
        self.values.iter().find(|v| v.name == name).map(|v| v.value)
    }

    /// 按值查找枚举名
    pub fn value_to_name(&self, value: i64) -> Option<&str> {
        self.values.iter().find(|v| v.value == value).map(|v| v.name.as_str())
    }
}

/// 接口定义
#[derive(Debug, Clone)]
pub struct InterfaceDef {
    pub name: String,
    pub full_name: String,
    pub methods: Vec<MethodInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::BasicType;

    #[test]
    fn test_class_declaration_basic() {
        let decl = ClassDeclaration {
            class_type: GorgeType::class("Test", None),
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
            method_count: 0,
            static_method_count: 0,
            constructor_count: 0,
            injector_field_type_count: TypeCount::zero(),
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0,
            constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
        };
        assert_eq!(decl.class_type.full_name(), "Test");
    }

    #[test]
    fn test_method_info() {
        let method = MethodInfo {
            name: "add".into(),
            return_type: GorgeType::new(BasicType::Int),
            parameters: vec![
                ParameterInfo { name: "a".into(), param_type: GorgeType::new(BasicType::Int) },
                ParameterInfo { name: "b".into(), param_type: GorgeType::new(BasicType::Int) },
            ],
            is_static: false,
            is_native: false,
            is_override: false,
            is_abstract: false,
        };
        assert_eq!(method.parameters.len(), 2);
    }

    #[test]
    fn test_enum_name_to_value() {
        let e = EnumDef {
            name: "Color".into(),
            full_name: "Color".into(),
            values: vec![
                EnumValue { name: "Red".into(), value: 1 },
                EnumValue { name: "Green".into(), value: 2 },
            ],
        };
        assert_eq!(e.name_to_value("Red"), Some(1));
        assert_eq!(e.value_to_name(2), Some("Green"));
        assert_eq!(e.name_to_value("Blue"), None);
    }
}
