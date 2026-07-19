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
}
