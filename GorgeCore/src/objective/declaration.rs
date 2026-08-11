use std::collections::HashMap;
use crate::objective::types::{GorgeType, TypeCount};

/// 注解参数值类型（S3）
#[derive(Debug, Clone)]
pub enum AnnotationValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    /// 隐藏静态方法的全局方法 ID
    Delegate(usize),
}

/// 方法级注解信息（S3）
#[derive(Debug, Clone)]
pub struct MethodAnnotation {
    pub name: String,
    pub parameters: Vec<(String, AnnotationValue)>,
}

impl MethodAnnotation {
    pub fn find_parameter(&self, name: &str) -> Option<&AnnotationValue> {
        self.parameters.iter().find(|(k, _)| k == name).map(|(_, v)| v)
    }
}

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
    /// 注入器构造方法实现映射（G3）：注入器构造方法本地ID → 全局构造方法ID
    pub injector_constructor_impl_id: Vec<usize>,
    /// 方法注解映射表（S3）：方法全局 ID → 注解列表
    pub method_annotations: HashMap<usize, Vec<MethodAnnotation>>,
    /// 构造方法注解映射表（S3）：构造方法全局 ID → 注解列表
    pub constructor_annotations: HashMap<usize, Vec<MethodAnnotation>>,
}

impl ClassDeclaration {
    /// 创建一个仅含名字的最小声明（用于注入器常量构造等场景）
    pub fn dummy(name: String) -> Self {
        Self {
            class_type: crate::objective::types::GorgeType::class(name, None),
            is_native: false, annotations: vec![], fields: vec![], methods: vec![],
            static_methods: vec![], constructors: vec![], injector_fields: vec![],
            super_class: None, super_interfaces: vec![],
            field_type_count: crate::objective::types::TypeCount::zero(),
            method_count: 0, static_method_count: 0, constructor_count: 0,
            injector_field_type_count: crate::objective::types::TypeCount::zero(),
            injector_field_default_value_type_count: crate::objective::types::TypeCount::zero(),
            method_start_id: 0, constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
            injector_constructor_impl_id: vec![],
            method_annotations: HashMap::new(),
            constructor_annotations: HashMap::new(),
        }
    }

    /// 按注解名查找含该注解的方法（S3）
    pub fn methods_with_annotation(&self, annotation_name: &str) -> Vec<(usize, &MethodAnnotation)> {
        let mut result = Vec::new();
        for (method_id, annotations) in &self.method_annotations {
            for ann in annotations {
                if ann.name == annotation_name {
                    result.push((*method_id, ann));
                }
            }
        }
        result
    }

    /// 按注解名查找含该注解的构造方法（S3）
    pub fn constructors_with_annotation(&self, annotation_name: &str) -> Vec<(usize, &MethodAnnotation)> {
        let mut result = Vec::new();
        for (ctor_id, annotations) in &self.constructor_annotations {
            for ann in annotations {
                if ann.name == annotation_name {
                    result.push((*ctor_id, ann));
                }
            }
        }
        result
    }

    /// 按注解名查找指定方法的注解（S3）
    pub fn find_annotation(&self, method_id: usize, annotation_name: &str) -> Option<&MethodAnnotation> {
        self.method_annotations
            .get(&method_id)
            .and_then(|anns| anns.iter().find(|a| a.name == annotation_name))
    }
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
    /// 是否为数组字段（如 `FunctionCurve^[]`），供物化端单对象自动包装
    pub is_array: bool,
    pub has_default_value: bool,
    /// 字段默认值常量（`auto defaultValue = ...` 的编译产物）；
    /// 物化注入器时对谱面未提供的字段应用此默认值。
    pub default_value: Option<crate::objective::bytecode::InjectorConstField>,
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
    use crate::objective::types::BasicType;

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
            injector_constructor_impl_id: vec![],
            method_annotations: HashMap::new(),
            constructor_annotations: HashMap::new(),
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

    #[test]
    fn test_method_annotations_query() {
        let mut decl = ClassDeclaration::dummy("Ann".into());
        let ann = MethodAnnotation {
            name: "ForwardTimedDestroy".into(),
            parameters: vec![("time".into(), AnnotationValue::Float(2.5))],
        };
        decl.method_annotations.insert(0, vec![ann]);
        let results = decl.methods_with_annotation("ForwardTimedDestroy");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 0);
        let param = results[0].1.find_parameter("time").unwrap();
        assert!(matches!(param, AnnotationValue::Float(2.5)));
    }

    #[test]
    fn test_annotation_delegate() {
        let mut decl = ClassDeclaration::dummy("DelTest".into());
        let ann = MethodAnnotation {
            name: "ForwardTimedGenerate".into(),
            parameters: vec![("time".into(), AnnotationValue::Delegate(42))],
        };
        decl.constructor_annotations.insert(0, vec![ann]);
        let results = decl.constructors_with_annotation("ForwardTimedGenerate");
        assert_eq!(results.len(), 1);
        let param = results[0].1.find_parameter("time").unwrap();
        assert!(matches!(param, AnnotationValue::Delegate(42)));
    }
}
