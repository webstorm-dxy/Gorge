//! 运行时模态容器（对应 C# `Runtime/RuntimeFormContainer.cs`）。
//!
//! 存储运行时使用的模态解析信息、Element 修改器表和即时音效方法表。

use std::collections::HashMap;

// ==================== 模态信息 ====================

/// 模态信息（对应 C# `FormInformation`）
#[derive(Debug, Clone)]
pub struct FormInformation {
    /// 模态名
    pub name: String,
    /// 模态版本
    pub version: String,
    /// 元素类型列表
    pub element_types: Vec<String>,
}

impl FormInformation {
    pub fn new(name: String, version: String, element_types: Vec<String>) -> Self {
        Self { name, version, element_types }
    }
}

// ==================== 运行时 Element 容器 ====================

/// Element 修改器描述（数据化存储，替代 C# 的 `MethodInformation` 委托引用）。
///
/// Rust 侧无法直接持有 Gorge 方法引用，因此使用 (class_name, method_id) 二元素标识。
#[derive(Debug, Clone)]
pub struct ElementModifierRef {
    /// Element 所属类全名
    pub class_name: String,
    /// 静态方法在类中的局部 ID
    pub method_id: usize,
}

impl ElementModifierRef {
    pub fn new(class_name: String, method_id: usize) -> Self {
        Self { class_name, method_id }
    }
}

/// 运行时 Element 容器（对应 C# `RuntimeElementContainer`）。
///
/// 存储运行时所需的 Element 类信息：修改器列表和定时创生构造器。
/// 委托/方法引用全部用数据化 ID 代替 C# 的直接对象引用。
#[derive(Debug, Clone)]
pub struct RuntimeElementContainer {
    /// 运行时修改器列表（标有 `@PeriodModifier` 的静态方法）
    pub modifiers: Vec<ElementModifierRef>,
    /// 初始创生构造器（标有 `@InitializeGenerate`），None 表示不进行初始创生
    pub initialize_generate_constructor: Option<usize>,
    /// 正转定时创生构造器（标有 `@ForwardTimedGenerate`），None 表示不进行正转定时创生
    pub forward_timed_generate_constructor: Option<usize>,
    /// 正转定时创生时间计算委托全局方法 ID（标有 `@ForwardTimedGenerate(time=...)` 的元数据）
    pub forward_generate_time_delegate_id: Option<usize>,
    /// 反转定时创生构造器（标有 `@BackwardTimedGenerate`），None 表示不进行反转定时创生
    pub backward_timed_generate_constructor: Option<usize>,
    /// 反转定时创生时间计算委托全局方法 ID
    pub backward_generate_time_delegate_id: Option<usize>,
}

impl RuntimeElementContainer {
    pub fn new() -> Self {
        Self {
            modifiers: Vec::new(),
            initialize_generate_constructor: None,
            forward_timed_generate_constructor: None,
            forward_generate_time_delegate_id: None,
            backward_timed_generate_constructor: None,
            backward_generate_time_delegate_id: None,
        }
    }
}

impl Default for RuntimeElementContainer {
    fn default() -> Self { Self::new() }
}

// ==================== 配置引用 ====================

/// 静态方法引用（数据化存储，用于 InstantAudioMethods 表）。
#[derive(Debug, Clone)]
pub struct StaticMethodRef {
    /// 所属类全名
    pub class_name: String,
    /// 方法在类中的局部 ID
    pub method_id: usize,
}

impl StaticMethodRef {
    pub fn new(class_name: String, method_id: usize) -> Self {
        Self { class_name, method_id }
    }
}

// ==================== 运行时模态容器 ====================

/// 运行时模态容器（对应 C# `RuntimeFormContainer`）。
///
/// 存储运行时使用的模态解析信息、Element 修改器表和即时音效方法表。
/// 以纯数据形式组织，由调用方持有并传递。
///
/// # 与 C# 的差异
///
/// C# 的构造函数接收 `GorgeLanguageRuntime` 并遍历已编译类的注解完成初始化。
/// Rust 侧框架尚未实现完整的 Gorge 反射系统，因此采用 `new_empty()` 创建空容器，
/// 由调用方（如 RuntimeManager）在编译完成后通过 `scan_from_runtime()` 填充。
#[derive(Debug, Clone)]
pub struct RuntimeFormContainer {
    /// 模态表（模态名 → 模态信息）
    pub forms: HashMap<String, FormInformation>,
    /// Element 修改器表（Element 类名 → 修改器容器）
    pub element_modifiers: HashMap<String, RuntimeElementContainer>,
    /// 即时音效方法表（音效名 → 静态方法引用）
    pub instant_audio_methods: HashMap<String, StaticMethodRef>,
}

impl RuntimeFormContainer {
    /// 创建空的运行时模态容器
    pub fn new_empty() -> Self {
        Self {
            forms: HashMap::new(),
            element_modifiers: HashMap::new(),
            instant_audio_methods: HashMap::new(),
        }
    }

    /// 从编译类列表中扫描模态信息（对应 C# `RuntimeFormContainer` 构造方法）
    ///
    /// 遍历编译类，检查静态方法的注解：
    /// 1. **`@Form` 注解** — 标记模态入口方法，提取 `name`、`version` 参数
    ///    和返回的元素类型列表（`String[]`）
    /// 2. **`@InstantAudio` 注解** — 标记即时音效方法，提取 `name` 参数
    ///
    /// # 参数
    /// - `compiled_classes`: 编译后的类列表，须已包含方法注解信息
    pub fn scan_forms_from_compiled(
        &mut self,
        compiled_classes: &[gorge_core::objective::bytecode::CompiledClass],
    ) {
        for cc in compiled_classes {
            let class_name = cc.class_type.full_name().to_string();

            // 扫描静态方法的注解
            for (method_id, annotations) in &cc.method_annotations {
                for ann in annotations {
                    match ann.name.as_str() {
                        "Form" => {
                            // 提取 name 参数（模态名）
                            let form_name = ann.find_parameter("name")
                                .and_then(annotation_value_to_string)
                                .unwrap_or_else(|| format!("Form_{}", method_id));

                            // 提取 version 参数（模态版本）
                            let version = ann.find_parameter("version")
                                .and_then(annotation_value_to_string)
                                .unwrap_or_else(|| "1.0".to_string());

                            // 元素类型列表：
                            // C# 中通过调用该静态方法获取 String[] 返回值
                            // Rust 侧从方法的 injector_constants 推导
                            // TODO: 完整实现需注入器实例化系统调用静态方法
                            let element_types = extract_element_types_from_method(cc, *method_id);

                            let form_info = FormInformation::new(
                                form_name,
                                version,
                                element_types,
                            );
                            self.forms.insert(form_info.name.clone(), form_info);
                        }
                        "InstantAudio" => {
                            // 提取 name 参数（音效名）
                            let audio_name = ann.find_parameter("name")
                                .and_then(annotation_value_to_string)
                                .unwrap_or_default();

                            // 获取方法局部 ID
                            let local_id = if *method_id >= cc.method_start_id {
                                *method_id - cc.method_start_id
                            } else {
                                *method_id
                            };

                            self.instant_audio_methods.insert(
                                audio_name,
                                StaticMethodRef::new(class_name.clone(), local_id),
                            );
                        }
                        _ => {}
                    }
                }
            }

            // 扫描构造方法注解（为 Element 子类）
            // TODO: 检测类是否继承自 GorgeFramework.Element
            self.scan_element_container_from_class(cc, &class_name);
        }
    }

    /// 扫描编译类中与 Element 相关的构造方法注解
    fn scan_element_container_from_class(
        &mut self,
        cc: &gorge_core::objective::bytecode::CompiledClass,
        class_name: &str,
    ) {
        // 检查是否存在任何 Element 相关的构造方法注解
        let has_element_annotations = cc.constructor_annotations.values()
            .any(|anns| anns.iter().any(|a| matches!(
                a.name.as_str(),
                "PeriodModifier" | "InitializeGenerate" | "ForwardTimedGenerate" | "BackwardTimedGenerate"
            )));

        if !has_element_annotations {
            return;
        }

        let mut elem_container = RuntimeElementContainer::new();

        // 扫描静态方法中的 @PeriodModifier
        for (method_id, _annotations) in &cc.method_annotations {
            for ann in _annotations {
                if ann.name == "PeriodModifier" {
                    let local_id = if *method_id >= cc.method_start_id {
                        *method_id - cc.method_start_id
                    } else {
                        *method_id
                    };
                    elem_container.modifiers.push(
                        ElementModifierRef::new(class_name.to_string(), local_id)
                    );
                }
            }
        }

        // 扫描构造方法中的 @InitializeGenerate / @ForwardTimedGenerate / @BackwardTimedGenerate
        for (ctor_id, annotations) in &cc.constructor_annotations {
            for ann in annotations {
                match ann.name.as_str() {
                    "InitializeGenerate" => {
                        elem_container.initialize_generate_constructor = Some(*ctor_id);
                    }
                    "ForwardTimedGenerate" => {
                        elem_container.forward_timed_generate_constructor = Some(*ctor_id);
                        // 提取 time 委托
                        if let Some(time_val) = ann.find_parameter("time") {
                            if let gorge_core::objective::declaration::AnnotationValue::Delegate(d) = time_val {
                                elem_container.forward_generate_time_delegate_id = Some(*d);
                            }
                        }
                    }
                    "BackwardTimedGenerate" => {
                        elem_container.backward_timed_generate_constructor = Some(*ctor_id);
                        if let Some(time_val) = ann.find_parameter("time") {
                            if let gorge_core::objective::declaration::AnnotationValue::Delegate(d) = time_val {
                                elem_container.backward_generate_time_delegate_id = Some(*d);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        self.element_modifiers.insert(class_name.to_string(), elem_container);
    }
}

// ==================== 辅助函数 ====================

/// 将注解参数值转换为字符串
fn annotation_value_to_string(value: &gorge_core::objective::declaration::AnnotationValue) -> Option<String> {
    match value {
        gorge_core::objective::declaration::AnnotationValue::String(s) => Some(s.clone()),
        gorge_core::objective::declaration::AnnotationValue::Int(v) => Some(v.to_string()),
        gorge_core::objective::declaration::AnnotationValue::Float(v) => Some(v.to_string()),
        gorge_core::objective::declaration::AnnotationValue::Bool(v) => Some(v.to_string()),
        gorge_core::objective::declaration::AnnotationValue::Delegate(_) => None,
    }
}

/// 从编译类中提取 @Form 方法返回的元素类型列表（骨架实现）
///
/// 完整实现需调用静态方法获取 `String[]` 返回值，
/// 当前从方法签名和注入器常量推导。
fn extract_element_types_from_method(
    _cc: &gorge_core::objective::bytecode::CompiledClass,
    _method_id: usize,
) -> Vec<String> {
    // TODO: 完整实现需通过 VM 调用该方法获取 String[] 返回值
    // 当前返回空列表作为占位
    Vec::new()
}

impl Default for RuntimeFormContainer {
    fn default() -> Self { Self::new_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f2_form_information_new() {
        let form = FormInformation::new(
            "NoteForm".into(),
            "1.0".into(),
            vec!["Tap".into(), "Hold".into()],
        );
        assert_eq!(form.name, "NoteForm");
        assert_eq!(form.version, "1.0");
        assert_eq!(form.element_types.len(), 2);
    }

    #[test]
    fn test_f2_runtime_form_container_empty() {
        let container = RuntimeFormContainer::new_empty();
        assert!(container.forms.is_empty());
        assert!(container.element_modifiers.is_empty());
        assert!(container.instant_audio_methods.is_empty());
    }

    #[test]
    fn test_f2_runtime_form_container_with_data() {
        let mut container = RuntimeFormContainer::new_empty();
        let form = FormInformation::new("F".into(), "v1".into(), vec!["E1".into()]);
        container.forms.insert("F".into(), form);

        let mut elem_container = RuntimeElementContainer::new();
        elem_container.modifiers.push(ElementModifierRef::new("GorgeFramework.TapNote".into(), 0));
        elem_container.initialize_generate_constructor = Some(1);
        container.element_modifiers.insert("TapNote".into(), elem_container);

        container.instant_audio_methods.insert(
            "hit".into(),
            StaticMethodRef::new("GorgeFramework.Audio".into(), 0),
        );

        assert_eq!(container.forms.len(), 1);
        assert_eq!(container.element_modifiers.len(), 1);
        assert_eq!(container.instant_audio_methods.len(), 1);
    }

    #[test]
    fn test_f2_runtime_element_container_default() {
        let container = RuntimeElementContainer::default();
        assert!(container.modifiers.is_empty());
        assert!(container.initialize_generate_constructor.is_none());
        assert!(container.forward_timed_generate_constructor.is_none());
        assert!(container.backward_timed_generate_constructor.is_none());
    }

    // ==================== F-3: scan_forms_from_compiled 测试 ====================

    /// 构造带 @Form 和 @InstantAudio 注解的编译类
    fn make_compiled_class_with_forms() -> gorge_core::objective::bytecode::CompiledClass {
        use gorge_core::objective::bytecode::CompiledClass;
        use gorge_core::objective::declaration::{MethodAnnotation, AnnotationValue};
        use gorge_core::objective::types::{GorgeType, TypeCount};
        use gorge_core::virtual_machine::ir::CompiledMethod;
        use std::collections::HashMap;

        let mut method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        // 方法 0: @Form 注解
        method_annotations.insert(0, vec![
            MethodAnnotation {
                name: "Form".into(),
                parameters: vec![
                    ("name".into(), AnnotationValue::String("NoteForm".into())),
                    ("version".into(), AnnotationValue::String("2.0".into())),
                ],
            },
        ]);
        // 方法 1: @InstantAudio 注解
        method_annotations.insert(1, vec![
            MethodAnnotation {
                name: "InstantAudio".into(),
                parameters: vec![
                    ("name".into(), AnnotationValue::String("hit".into())),
                ],
            },
        ]);

        CompiledClass {
            class_type: GorgeType::class("GorgeFramework.FormDef", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![
                CompiledMethod { name: "ElementTypeList".into(), codes: vec![], local_count: 0 },
                CompiledMethod { name: "PlayHit".into(), codes: vec![], local_count: 0 },
            ],
            constructors: vec![],
            injector_fields: vec![],
            delegate_impls: vec![],
            method_start_id: 0,
            method_count_total: 2,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],
            injector_constants: vec![],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![],
            method_annotations,
            constructor_annotations: HashMap::new(),
        }
    }

    #[test]
    fn test_f3_scan_forms_from_compiled_form_extracted() {
        let classes = vec![make_compiled_class_with_forms()];
        let mut container = RuntimeFormContainer::new_empty();
        container.scan_forms_from_compiled(&classes);

        // 验证 Form 提取
        assert_eq!(container.forms.len(), 1, "应提取 1 个 Form");
        let form = container.forms.get("NoteForm").unwrap();
        assert_eq!(form.name, "NoteForm");
        assert_eq!(form.version, "2.0");
        // 元素类型列表当前为空（TODO: 需 VM 调用）
        assert!(form.element_types.is_empty(),
            "当前为骨架实现，元素类型列表应为空");
    }

    #[test]
    fn test_f3_scan_forms_from_compiled_instant_audio() {
        let classes = vec![make_compiled_class_with_forms()];
        let mut container = RuntimeFormContainer::new_empty();
        container.scan_forms_from_compiled(&classes);

        // 验证 InstantAudio 提取
        assert_eq!(container.instant_audio_methods.len(), 1,
            "应提取 1 个 InstantAudio 方法");
        let method_ref = container.instant_audio_methods.get("hit").unwrap();
        assert_eq!(method_ref.class_name, "GorgeFramework.FormDef");
        assert_eq!(method_ref.method_id, 1);
    }

    #[test]
    fn test_f3_scan_forms_from_compiled_empty_classes() {
        let classes: Vec<gorge_core::objective::bytecode::CompiledClass> = vec![];
        let mut container = RuntimeFormContainer::new_empty();
        // 应不 panic
        container.scan_forms_from_compiled(&classes);
        assert!(container.forms.is_empty());
        assert!(container.instant_audio_methods.is_empty());
    }
}
