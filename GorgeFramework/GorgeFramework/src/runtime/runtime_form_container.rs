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
    ///    并通过 VM 调用静态方法获取返回的元素类型列表（`String[]`）
    /// 2. **`@InstantAudio` 注解** — 标记即时音效方法，提取 `name` 参数
    ///
    /// # 参数
    /// - `compiled_classes`: 编译后的类列表，须已包含方法注解信息
    /// - `vm`: 虚拟机实例，须已注册目标类；用于调用 `@Form` 静态方法
    pub fn scan_forms_from_compiled(
        &mut self,
        compiled_classes: &[gorge_core::objective::bytecode::CompiledClass],
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
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
                            // 通过 VM 调用该静态方法获取 String[] 返回值
                            let element_types = extract_element_types_from_method(
                                vm,
                                &class_name,
                                *method_id,
                            );

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
            // P0-5：检测类是否继承自 GorgeFramework.Element，防止非元素类混入
            if is_element_subclass(&class_name, vm) {
                self.scan_element_container_from_class(cc, &class_name);
            }
        }
    }

    /// 扫描编译类中与 Element 相关的构造方法注解
    fn scan_element_container_from_class(
        &mut self,
        cc: &gorge_core::objective::bytecode::CompiledClass,
        class_name: &str,
    ) {
        // 检查是否存在任何 Element 相关的构造方法/静态方法注解：
        // 生成方式注解（@InitializeGenerate/@ForwardTimedGenerate/@BackwardTimedGenerate）
        // 位于构造注解表；@PeriodModifier 位于静态方法注解表（C# 中两者
        // 分别来自 Constructors/StaticMethods 的注解，等价判定）。
        let has_element_annotations = cc.constructor_annotations.values()
            .any(|anns| anns.iter().any(|a| matches!(
                a.name.as_str(),
                "InitializeGenerate" | "ForwardTimedGenerate" | "BackwardTimedGenerate"
            )))
            || cc.method_annotations.values()
                .any(|anns| anns.iter().any(|a| a.name == "PeriodModifier"));

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

/// 判定类是否为 `GorgeFramework.Element` 的子类（P0-5，对齐 C# `ClassDeclaration.Is`）。
///
/// C# 的声明链包含 native 父类（`Note : Element`）；Rust 侧 native 类注册表
/// （`native_class_table`）不含继承信息，且 loader 只把编译类父类注册进
/// `class_super_name`（native 父类的父类断链）。因此除沿注册链上溯外，
/// 将框架固定的两个 native 根类 `Element` 与 `Note`（`Note : Element`）
/// 作为硬编码终点判定。
pub fn is_element_subclass(
    class_name: &str,
    vm: &gorge_core::virtual_machine::vm::VirtualMachine,
) -> bool {
    let is_root = |name: &str| {
        let simple = name.rsplit('.').next().unwrap_or(name);
        simple == "Element" || simple == "Note"
    };
    // 自身就是根（防御：native 类本身不会出现在编译类/注入器路径中）
    if is_root(class_name) {
        return true;
    }

    // 沿 class_super_name 链上溯。注册方约定不一：loader 以短类名注册
    // （类 → 父类短名），测试/部分路径可能以全名注册，因此对每一层
    // 都同时尝试「全名 + 简单名」两种形式。
    let mut current = class_name.to_string();
    let mut guard = 0;
    while guard <= 1000 {
        let simple_current = current.rsplit('.').next().unwrap_or(&current).to_string();
        let parent = vm.class_super_name.get(&current)
            .or_else(|| vm.class_super_name.get(&simple_current))
            .cloned();
        let Some(parent) = parent else { return false };

        if is_root(&parent) {
            return true;
        }
        current = parent;
        guard += 1;
    }
    false
}

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

/// 从编译类中提取 @Form 方法返回的元素类型列表
///
/// 通过 VM 调用对应的静态方法，获取 `String[]` 返回值并转换为 `Vec<String>`。
/// 若方法不存在、执行失败或返回值不是字符串数组，则返回空列表。
///
/// # 参数
/// - `vm`: 虚拟机实例，须已注册目标类
/// - `class_name`: 目标类全名
/// - `method_global_id`: 方法全局 ID（`method_annotations` 的键）
///
/// # 返回值
/// 元素类型名字符串列表；失败时返回空 Vec
fn extract_element_types_from_method(
    vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    class_name: &str,
    method_global_id: usize,
) -> Vec<String> {
    // 1. 调用静态方法（无参数、无 this）
    if vm.invoke_method_by_id(class_name, None, method_global_id).is_err() {
        return Vec::new();
    }

    // 2. 读取返回值对象 ID（0 表示 null）
    let array_obj_id = match vm.return_object {
        Some(id) if id != 0 => id,
        _ => return Vec::new(),
    };

    // 3. 从 native 载荷表 downcast 为 StringArray 并克隆元素
    vm.native_payloads
        .get(&array_obj_id)
        .and_then(|p| p.downcast_ref::<gorge_core::system::native::array::StringArray>())
        .map(|arr| arr.items.clone())
        .unwrap_or_default()
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
        // 测试类未注册到 VM，invoke_method_by_id 会失败，element_types 为空
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        container.scan_forms_from_compiled(&classes, &mut vm);

        // 验证 Form 提取
        assert_eq!(container.forms.len(), 1, "应提取 1 个 Form");
        let form = container.forms.get("NoteForm").unwrap();
        assert_eq!(form.name, "NoteForm");
        assert_eq!(form.version, "2.0");
        // 类未注册到 VM，静态方法调用失败，元素类型列表为空
        assert!(form.element_types.is_empty(),
            "未注册 VM 时元素类型列表应为空");
    }

    #[test]
    fn test_f3_scan_forms_from_compiled_instant_audio() {
        let classes = vec![make_compiled_class_with_forms()];
        let mut container = RuntimeFormContainer::new_empty();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        container.scan_forms_from_compiled(&classes, &mut vm);

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
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        // 应不 panic
        container.scan_forms_from_compiled(&classes, &mut vm);
        assert!(container.forms.is_empty());
        assert!(container.instant_audio_methods.is_empty());
    }

    // ==================== P0-5: Element 继承判定测试 ====================

    #[test]
    fn test_p0_5_is_element_subclass_along_registered_chain() {
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        // 短类名注册（loader 约定）：类 → 父类
        vm.register_class_super("DremuNote", "Note");
        vm.register_class_super("DremuLane", "Element");
        // 全名注册（部分测试路径约定）
        vm.register_class_super("Demo.ChildElement", "Demo.BaseElement");
        vm.register_class_super("Demo.BaseElement", "GorgeFramework.Element");

        // 直接/间接继承 Element 的类均判定为元素
        assert!(is_element_subclass("DremuLane", &vm));
        assert!(is_element_subclass("DremuNote", &vm), "Note 是 Element 子类（native 根）");
        assert!(is_element_subclass("Demo.ChildElement", &vm), "沿多级全名链上溯");
        // 非元素类
        assert!(!is_element_subclass("Song", &vm));
        assert!(!is_element_subclass("GorgeFramework.FormDef", &vm));
        // 断链类（父类不在注册表）
        assert!(!is_element_subclass("MysteryClass", &vm));
    }

    #[test]
    fn test_p0_5_scan_element_container_filters_non_element_classes() {
        use gorge_core::objective::bytecode::CompiledClass;
        use gorge_core::objective::declaration::MethodAnnotation;
        use gorge_core::objective::types::{GorgeType, TypeCount};
        use gorge_core::virtual_machine::ir::CompiledMethod;
        use std::collections::HashMap;

        // 元素子类：带 @PeriodModifier 静态方法
        let mut elem_method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        elem_method_annotations.insert(0, vec![
            MethodAnnotation { name: "PeriodModifier".into(), parameters: vec![] },
        ]);
        let elem_class = CompiledClass {
            class_type: GorgeType::class("Dremu.DremuTap", None),
            is_native: false,
            super_class_name: Some("Note".into()),
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![CompiledMethod { name: "PeriodModifier".into(), codes: vec![], local_count: 0 }],
            constructors: vec![],
            injector_fields: vec![],
            delegate_impls: vec![],
            method_start_id: 0,
            method_count_total: 1,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],
            injector_constants: vec![],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![],
            method_annotations: elem_method_annotations,
            constructor_annotations: HashMap::new(),
        };
        // 非元素类：同样带 @PeriodModifier，应被过滤
        let mut non_elem_method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        non_elem_method_annotations.insert(0, vec![
            MethodAnnotation { name: "PeriodModifier".into(), parameters: vec![] },
        ]);
        let non_elem_class = CompiledClass {
            class_type: GorgeType::class("Dremu.Song", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![CompiledMethod { name: "PeriodModifier".into(), codes: vec![], local_count: 0 }],
            constructors: vec![],
            injector_fields: vec![],
            delegate_impls: vec![],
            method_start_id: 0,
            method_count_total: 1,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],
            injector_constants: vec![],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![],
            method_annotations: non_elem_method_annotations,
            constructor_annotations: HashMap::new(),
        };

        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        vm.register_class_super("Dremu.DremuTap", "Note");
        vm.register_class_super("Dremu.Song", "GorgeFramework.AudioAsset");

        let mut container = RuntimeFormContainer::new_empty();
        container.scan_forms_from_compiled(&[elem_class, non_elem_class], &mut vm);

        // 仅元素子类进入 element_modifiers 表
        assert!(container.element_modifiers.contains_key("Dremu.DremuTap"),
            "元素子类应进入修改器表");
        assert!(!container.element_modifiers.contains_key("Dremu.Song"),
            "非元素类不应进入修改器表");
    }
}
