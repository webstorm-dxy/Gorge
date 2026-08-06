//! 运行时环境模块（对应 C# `Runtime/Environment/` 文件夹）。
//!
//! 包含 GorgeSimulationRuntime 及子管理器、SimulationModule trait、环境全局注册表。

pub mod simulation_module;
pub mod global;
pub mod scene_manager;
pub mod simulation_manager;
pub mod priority_heap;

use crate::runtime::priority_heap::PriorityHeap;
use crate::chart::simulation_score::SimulationScore;
use crate::chart::staff::ElementStaff;
use crate::signal::channel_split::ChannelSplit;
use crate::input::edge::Edge;
use crate::input::fragment::Fragment;
use crate::signal::multichannel_split::MultichannelSplit;
use gorge_core::system::native::injector::Injector;
use gorge_core::system::native::injector::RuntimeInjector;
use gorge_core::objective::object::GorgeObject;
use gorge_core::objective::types::BasicType;
use crate::simulators::IGameplayAction;

// ==================== 音频乐段数据 ====================

/// 音频乐段数据（R-5c：供 AudioStaff → AudioManager 数据流用）
///
/// 从 SimulationScore 的谱表提取后传递给 AudioManager.add_period。
#[derive(Debug, Clone)]
pub struct AudioPeriodData {
    /// 乐段对象 ID
    pub period_id: usize,
    /// 音频资产对象 ID
    pub audio_id: usize,
    /// 时间偏移（秒）
    pub time_offset: f32,
}

impl AudioPeriodData {
    pub fn new(period_id: usize, audio_id: usize, time_offset: f32) -> Self {
        Self { period_id, audio_id, time_offset }
    }
}

// ==================== 子管理器骨架 ====================

/// 谱面管理器
///
/// 维护 AliveElements/AliveNotes 表、定时创生/销毁列表、
/// Element 修改器表等。
#[derive(Debug)]
pub struct ChartManager {
    pub begin_chart_time: f32,
    pub terminate_chart_time: f32,
    pub begin_simulate_speed: f32,
    /// 正转定时创生列表：(时间, 元素 injector ID, 构造方法全局 ID)
    pub forward_timed_generate_list: Vec<(f32, usize, usize)>,
    /// 反转定时创生列表
    pub backward_timed_generate_list: Vec<(f32, usize, usize)>,
    /// 初始化时立即创生的元素：(元素 injector ID, 构造方法全局 ID)
    pub initialize_generate_list: Vec<(usize, usize)>,
    /// 正转定时销毁列表：(时间, 元素对象 ID)
    pub forward_timed_destroy_list: Vec<(f32, usize)>,
    /// 反转定时销毁列表
    pub backward_timed_destroy_list: Vec<(f32, usize)>,
    /// 存活元素总表（元素对象 ID 列表）
    pub alive_elements: Vec<usize>,
    /// 存活 Note 表
    pub alive_notes: Vec<usize>,
    /// 存活非派生元素 → 创生时注入器 ID 的映射
    pub alive_injector_map: HashMap<usize, usize>,
    /// 存活元素 → 其模拟器注册键的映射（P1-4）
    ///
    /// 值为 (主模拟器 reg_key, 尾独立模拟器 reg_key)，未注册的为 None。
    /// GenerateElement 注册模拟器时记录，DestroyElement 据此从优先级堆
    /// 与 SimRegistry 中精确注销（对齐 C# `Simulators.Remove(element.simulator)`）。
    pub element_simulator_keys: HashMap<usize, (Option<usize>, Option<usize>)>,
}

impl ChartManager {
    pub fn new() -> Self {
        Self {
            begin_chart_time: 0.0,
            terminate_chart_time: 0.0,
            begin_simulate_speed: 1.0,
            forward_timed_generate_list: Vec::new(),
            backward_timed_generate_list: Vec::new(),
            initialize_generate_list: Vec::new(),
            forward_timed_destroy_list: Vec::new(),
            backward_timed_destroy_list: Vec::new(),
            alive_elements: Vec::new(),
            alive_notes: Vec::new(),
            alive_injector_map: HashMap::new(),
            element_simulator_keys: HashMap::new(),
        }
    }

    /// 向定时创生表添加元素（对齐 C# `AddScoreElement` 正转/反转生成部分）
    ///
    /// `injector_id` 为注入器对象 ID，`constructor_id` 为构造方法全局 ID，
    /// `time` 为生成时间，`direction` 为正向/反向。
    pub fn add_timed_generate(&mut self, injector_id: usize, constructor_id: usize, time: f32, direction: crate::runtime::simulation_types::SimulateDirection) {
        match direction {
            crate::runtime::simulation_types::SimulateDirection::Forward => {
                self.forward_timed_generate_list.push((time, injector_id, constructor_id));
            }
            crate::runtime::simulation_types::SimulateDirection::Backward => {
                self.backward_timed_generate_list.push((time, injector_id, constructor_id));
            }
            crate::runtime::simulation_types::SimulateDirection::Infinitesimal => {}
        }
    }

    /// 向定时销毁表添加元素
    pub fn add_timed_destroy(&mut self, element_id: usize, time: f32, direction: crate::runtime::simulation_types::SimulateDirection) {
        match direction {
            crate::runtime::simulation_types::SimulateDirection::Forward => {
                self.forward_timed_destroy_list.push((time, element_id));
            }
            crate::runtime::simulation_types::SimulateDirection::Backward => {
                self.backward_timed_destroy_list.push((time, element_id));
            }
            crate::runtime::simulation_types::SimulateDirection::Infinitesimal => {}
        }
    }

    /// 添加谱面元素到生成表（对齐 C# `AddScoreElement`，S4-4）
    ///
    /// 先克隆注入器，再沿继承链调用所有 `@PeriodModifier` 静态方法做 gameplay 修正
    /// （轨道位置、缩放等，对齐 C# `Modify`），最后按修正后的注入器查询
    /// 构造注解（@InitializeGenerate/@ForwardTimedGenerate/@BackwardTimedGenerate），
    /// 填充 ChartManager 的定时创生/初始化创生表。
    ///
    /// 与 C# 的差异：C# 通过 `FormContainer.ElementModifiers` 查找修改器；Rust 侧
    /// 该容器数据已就绪但尚未接线（P0-6），本方法自带自扫描（类 + 父类声明链上的
    /// `@PeriodModifier` 方法），两者效果一致——容器接线后语义也不变。
    ///
    /// `injector_id` 为谱面记载注入器对象 ID，`period_config` 为元素所属乐段的配置，
    /// `vm` 用于注入器操作和修改器方法调用。
    pub fn add_score_element(
        &mut self, injector_id: usize, period_config: &crate::chart::period::PeriodConfig,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        let declared_class_name = match vm.injectors.get(&injector_id) {
            Some(inj) => inj.injection_class_declaration().class_type.full_name(),
            None => return,
        };
        let Some(class_name) = resolve_registered_class_name(vm, &declared_class_name) else {
            return;
        };

        // P0-5：检测类是否继承自 GorgeFramework.Element（对齐 C# ChartManager.cs:57），
        // 防止非元素对象混入生成表。
        if !crate::runtime::runtime_form_container::is_element_subclass(&class_name, vm) {
            return;
        }

        // 1. clone 注入器并应用 @PeriodModifier 静态方法（C# Modify）。
        //    修改版注入器注册回 VM，生成表一律使用它。
        let Some(gameplay_injector_id) = Self::modify_injector(injector_id, period_config, vm)
        else {
            return;
        };

        // 先克隆注解列表以释放不可变借用
        let init_ctors: Vec<(usize, _)> = vm.class_table
            .get(&class_name)
            .map(|cls| cls.declaration.constructors_with_annotation("InitializeGenerate")
                .into_iter().map(|(id, ann)| (id, ann.clone())).collect())
            .unwrap_or_default();
        for (ctor_id, _ann) in init_ctors {
            self.initialize_generate_list.push((gameplay_injector_id, ctor_id));
        }

        let fwd_ctors: Vec<(usize, _)> = vm.class_table
            .get(&class_name)
            .map(|cls| cls.declaration.constructors_with_annotation("ForwardTimedGenerate")
                .into_iter().map(|(id, ann)| (id, ann.clone())).collect())
            .unwrap_or_default();
        for (ctor_id, ann) in fwd_ctors {
            let time = Self::resolve_annotation_time(&ann, &class_name, vm);
            self.forward_timed_generate_list.push((time, gameplay_injector_id, ctor_id));
        }

        let bwd_ctors: Vec<(usize, _)> = vm.class_table
            .get(&class_name)
            .map(|cls| cls.declaration.constructors_with_annotation("BackwardTimedGenerate")
                .into_iter().map(|(id, ann)| (id, ann.clone())).collect())
            .unwrap_or_default();
        for (ctor_id, ann) in bwd_ctors {
            let time = Self::resolve_annotation_time(&ann, &class_name, vm);
            self.backward_timed_generate_list.push((time, gameplay_injector_id, ctor_id));
        }
    }

    /// 克隆注入器并沿继承链应用所有 `@PeriodModifier` 静态方法（对齐 C# `Modify`）。
    ///
    /// 修改器方法签名为 `(元素注入器, PeriodConfig)`（如 `DremuNote.PeriodModifier`），
    /// 调用时参数池 object[0]=修改版注入器、object[1]=物化的 PeriodConfig 对象，
    /// `current_injector` 切到修改版注入器（方法体经 `LoadInjector` 读写注入器字段）。
    ///
    /// 返回修改版注入器对象 ID；克隆失败时返回 None。
    fn modify_injector(
        original_injector_id: usize,
        period_config: &crate::chart::period::PeriodConfig,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) -> Option<usize> {
        if !period_config.active {
            return None;
        }

        // 1. clone 注入器（C# `(Injector) scoreElementInjector.Clone()`：
        //    深拷贝字段值，声明以 Arc 共享）
        let original = vm.injectors.get(&original_injector_id)?;
        let gameplay = original.clone();
        let gameplay_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(gameplay_id, gameplay);

        // 2. 沿类 + 父类声明链收集 @PeriodModifier 静态方法（全局方法 ID）
        //    扫描直接用声明注解表（与 C# 遍历 FormContainer.ElementModifiers 等价），
        //    避免在持有 &mut vm 时借用 class_table。
        //    注解表为 HashMap，按全局 ID 排序保证修改器应用顺序确定（声明顺序）。
        let mut modifiers: Vec<(String, usize)> = Vec::new();
        let mut declared = vm.injectors.get(&gameplay_id)?.injection_class_declaration().clone();
        let mut guard = 0;
        loop {
            let mut annotation_ids: Vec<&usize> = declared.method_annotations.keys().collect();
            annotation_ids.sort();
            for global_id in annotation_ids {
                let annotations = &declared.method_annotations[global_id];
                if annotations.iter().any(|a| a.name == "PeriodModifier") {
                    if let Some(name) = resolve_registered_class_name(vm, &declared.class_type.full_name()) {
                        modifiers.push((name, *global_id));
                    }
                }
            }
            let Some(super_decl) = declared.super_class else { break };
            declared = *super_decl;
            guard += 1;
            if guard > 1000 { break; }
        }
        if modifiers.is_empty() {
            return Some(gameplay_id);
        }

        // 3. 物化 PeriodConfig 对象（修改器方法体以 native 字段读取访问，
        //    如 periodConfig.timeOffset → LoadFloatField(0)）
        let config_obj_id = materialize_period_config_object(period_config, vm);

        // 4. 逐修改器调用：方法体的 LoadObjectParameter(0)/(1) 直接从参数池取参，
        //    LoadInjector 经 current_injector 寻址修改版注入器。
        let saved_injector = vm.current_injector;
        vm.current_injector = Some(gameplay_id);
        for (modifier_class, method_id) in &modifiers {
            vm.param_pool.set_object_param(0, gameplay_id);
            vm.param_pool.set_object_param(1, config_obj_id);
            // @PeriodModifier 是静态方法，须经静态方法表分派（对齐 C# `InvokeStaticMethod`）。
            // 用实例路径 `invoke_method_by_id` 会导致静态方法被误判为实例方法而失败。
            if let Err(e) = vm.invoke_static_method_by_global_id(modifier_class, *method_id) {
                eprintln!("[Gorge] @PeriodModifier 调用失败 {}#{}: {}", modifier_class, method_id, e);
            }
        }
        vm.current_injector = saved_injector;
        Some(gameplay_id)
    }

    /// 将总谱中的元素 JSON 物化为 VM 注入器，并填充创生队列。
    pub fn load_score(
        &mut self,
        score: &SimulationScore,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        self.unload_score();
        self.begin_chart_time = score.start_time;
        self.terminate_chart_time = score.terminate_time;
        self.begin_simulate_speed = score.simulation_speed;

        for staff in &score.stave {
            let Some(element_staff) = staff.as_any().downcast_ref::<ElementStaff>() else {
                continue;
            };
            for period in &element_staff.periods {
                if !period.period_data.config.active {
                    continue;
                }
                let period_config = &period.period_data.config;
                for element in &period.elements {
                    if let Some(injector_id) = self.materialize_injector(element, vm) {
                        self.add_score_element(injector_id, period_config, vm);
                    }
                }
            }
        }
    }

    /// 清理由已加载谱面产生的队列和存活索引。
    pub fn unload_score(&mut self) {
        self.begin_chart_time = 0.0;
        self.terminate_chart_time = 0.0;
        self.begin_simulate_speed = 1.0;
        self.initialize_generate_list.clear();
        self.forward_timed_generate_list.clear();
        self.backward_timed_generate_list.clear();
        self.forward_timed_destroy_list.clear();
        self.backward_timed_destroy_list.clear();
        self.alive_elements.clear();
        self.alive_notes.clear();
        self.alive_injector_map.clear();
        self.element_simulator_keys.clear();
    }

    /// 从谱面 JSON 递归创建 RuntimeInjector。
    fn materialize_injector(
        &self,
        value: &serde_json::Value,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) -> Option<usize> {
        let object = value.as_object()?;
        let class_name = object.get("__type")?.as_str()?;
        if matches!(class_name, "int" | "float" | "bool" | "string" | "object") {
            return None;
        }

        let registered_class_name = resolve_registered_class_name(vm, class_name)?;
        // P0-8：注入器字段声明统一取注册键对应的类声明。谱面 JSON 中的元素
        // 与嵌套注入器大量为 native 类（VariableFloat/FunctionCurve 族），
        // native 类以 `NativeClass::declaration()` 提供声明（含 Gorge 字段名，
        // 见 injector_fields_meta），保证按名匹配字段；编译类走 class_table。
        let class_decl = if let Some(class) = vm.class_table.get(&registered_class_name) {
            class.declaration.clone()
        } else if let Some(native) = vm.native_class_table.get(&registered_class_name) {
            native.declaration()
        } else {
            return None;
        };
        // 注入器后续会以声明中的类型名查回 VM。Demo 以短类名注册，
        // 因此这里必须保存实际注册键，而不是 JSON 中的全限定名。
        let mut declaration = class_decl.clone();
        declaration.class_type = gorge_core::objective::types::GorgeType::class(
            registered_class_name.clone(),
            None,
        );
        let mut injector = RuntimeInjector::new(std::sync::Arc::new(declaration));
        let mut int_index = 0;
        let mut float_index = 0;
        let mut bool_index = 0;
        let mut string_index = 0;
        let mut object_index = 0;

        for field in &class_decl.injector_fields {
            let field_value = object.get(&field.name);
            match field.field_type.basic_type {
                BasicType::Int | BasicType::Enum => {
                    if let Some(number) = json_scalar(field_value).and_then(serde_json::Value::as_i64) {
                        injector.set_injector_int(int_index, number);
                    }
                    int_index += 1;
                }
                BasicType::Float => {
                    if let Some(number) = json_scalar(field_value).and_then(serde_json::Value::as_f64) {
                        injector.set_injector_float(float_index, number);
                    }
                    float_index += 1;
                }
                BasicType::Bool => {
                    if let Some(flag) = json_scalar(field_value).and_then(serde_json::Value::as_bool) {
                        injector.set_injector_bool(bool_index, flag);
                    }
                    bool_index += 1;
                }
                BasicType::String => {
                    if let Some(text) = json_scalar(field_value).and_then(serde_json::Value::as_str) {
                        injector.set_injector_string(string_index, text.to_string());
                    }
                    string_index += 1;
                }
                BasicType::Object | BasicType::Interface | BasicType::Delegate => {
                    if let Some(nested) = field_value.and_then(|v| self.materialize_injector(v, vm)) {
                        injector.set_injector_object(object_index, nested);
                    }
                    object_index += 1;
                }
                BasicType::Void | BasicType::Null => {}
            }
        }

        let injector_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(injector_id, injector);
        Some(injector_id)
    }

    /// 从注解参数中解析 time 值（Float 直接取值 / Delegate 经 invoke_method_by_id 求值）
    fn resolve_annotation_time(
        ann: &gorge_core::objective::declaration::MethodAnnotation,
        class_name: &str,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) -> f32 {
        use gorge_core::objective::declaration::AnnotationValue;
        match ann.find_parameter("time") {
            Some(AnnotationValue::Float(f)) => *f as f32,
            Some(AnnotationValue::Delegate(method_id)) => {
                if vm.invoke_method_by_id(class_name, None, *method_id).is_ok() {
                    vm.return_float.unwrap_or(0.0) as f32
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }
}

/// 返回可用于 VM 类表查询的类名。
///
/// 谱面 JSON 保存全限定名，而 Demo 为兼容 native/编译类注册约定使用短类名。
/// 先保持全名精确匹配，再退回末段名称，避免错误覆盖本来已注册的全名。
/// P0-8：native 类注册在 `native_class_table`（`register_native_class` 同时
/// 登记全名与短名），宿主侧物化 native 注入器（VariableFloat/曲线族等）
/// 也按此约定解析。
fn resolve_registered_class_name(
    vm: &gorge_core::virtual_machine::vm::VirtualMachine,
    declared_class_name: &str,
) -> Option<String> {
    if vm.class_table.contains_key(declared_class_name) || vm.native_class_table.contains_key(declared_class_name) {
        return Some(declared_class_name.to_string());
    }

    let simple_name = declared_class_name.rsplit('.').next().unwrap_or(declared_class_name);
    (vm.class_table.contains_key(simple_name) || vm.native_class_table.contains_key(simple_name))
        .then(|| simple_name.to_string())
}

/// 将 `PeriodConfig` 数据物化为 VM 对象表中的一个对象（供 `@PeriodModifier` 方法体读取）。
///
/// 修改器方法体以 native 字段读取访问 periodConfig（如 `periodConfig.timeOffset` →
/// `LoadFloatField(0)`），因此按 `GorgeFramework.PeriodConfig` 的字段布局
/// （float[0]=timeOffset、float[1]=minLength、bool[0]=active）创建 RuntimeObject。
/// 不依赖 native_class_table 注册，避免类注册顺序问题。
fn materialize_period_config_object(
    period_config: &crate::chart::period::PeriodConfig,
    vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
) -> usize {
    use gorge_core::objective::types::TypeCount;
    use gorge_core::objective::object::RuntimeObject;

    let object = RuntimeObject::new_simple(
        "GorgeFramework.PeriodConfig".into(),
        &TypeCount { float_count: 2, bool_count: 1, ..TypeCount::zero() },
    );
    let obj_id = vm.next_object_id;
    vm.next_object_id += 1;
    vm.objects.insert(obj_id, object);
    if let Some(o) = vm.objects.get_mut(&obj_id) {
        o.set_float_field(0, period_config.time_offset as f64);
        o.set_float_field(1, period_config.min_length as f64);
        o.set_bool_field(0, period_config.active);
    }
    obj_id
}

fn json_scalar(value: Option<&serde_json::Value>) -> Option<&serde_json::Value> {
    match value {
        Some(serde_json::Value::Object(object)) => object.get("value").or(value),
        _ => value,
    }
}

/// 仿真管理器
///
/// 持有两个优先级堆（Simulators + LateIndependentSimulators）。
/// S4c：SimulationMachine 已移至 RuntimeManager 解决借用冲突。
#[derive(Debug)]
pub struct SimulationManager {
    /// 主模拟器优先级堆（优先级 → 模拟器对象 ID）
    pub simulators: PriorityHeap<i32, usize>,
    /// 尾独立模拟器优先级堆
    pub late_independent_simulators: PriorityHeap<i32, usize>,
}

impl SimulationManager {
    pub fn new() -> Self {
        Self {
            simulators: PriorityHeap::new(),
            late_independent_simulators: PriorityHeap::new(),
        }
    }
}

// ==================== SimulationManager 内部 simulator 注册表 ====================
// 存储已注册的 Box<dyn ISimulator>，优先级 id 为 PriorityHeap 中的 usize 键。
// 因为 Box<dyn ISimulator> 不满足 Hash/Eq/Clone，无法直接放进 PriorityHeap 作为 V。
// 用独立的 Hasher+HashMap 搭配 PriorityHeap 完成管理。

use std::collections::HashMap;
use crate::simulators::ISimulator as SimulatorTrait;

/// 内部 simulator 注册表（非 pub，通过 SimulationManager 方法间接访问）
#[derive(Default)]
pub struct SimRegistry {
    /// 自增 ID → Box<dyn ISimulator>
    map: HashMap<usize, Box<dyn SimulatorTrait>>,
    next_id: usize,
}

impl SimRegistry {
    pub fn new() -> Self { Self { map: HashMap::new(), next_id: 1 } }

    /// 注册一个 simulator，返回分配的内部 ID
    pub fn register(&mut self, sim: Box<dyn SimulatorTrait>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.map.insert(id, sim);
        id
    }

    /// 按 ID 获取 simulator 引用
    pub fn get(&self, id: usize) -> Option<&dyn SimulatorTrait> {
        self.map.get(&id).map(|b| b.as_ref())
    }

    /// 按 ID 删除
    pub fn remove(&mut self, id: usize) {
        self.map.remove(&id);
    }

    /// 清空全部已注册 simulator（对齐 C# `Simulation.RuntimeInitialize/RuntimeDestruct`
    /// 中重建模拟器表的语义），ID 计数器归零
    pub fn clear(&mut self) {
        self.map.clear();
        self.next_id = 1;
    }

    /// 遍历全部 simulator
    pub fn iter(&self) -> impl Iterator<Item = (usize, &dyn SimulatorTrait)> {
        self.map.iter().map(|(id, sim)| (*id, sim.as_ref()))
    }
}

impl std::fmt::Debug for SimRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimRegistry").field("count", &self.map.len()).finish()
    }
}

/// 自动机管理器
///
/// 维护信号自动机列表、待决检测条件表、输入信号总表。
#[derive(Debug)]
pub struct AutomatonManager {
    /// 多通道输入信号切片（全部时间范围的信号记录，含边沿）
    pub input_signals: MultichannelSplit,
    /// 已注册的信号自动机对象 ID 列表（S4-3）
    pub automatons: Vec<usize>,
    /// 待决检测条件表：自动机对象 ID → 检测条件列表（S7）
    pub pending_detection_conditions: std::collections::HashMap<usize, Vec<crate::simulators::SignalDetectionCondition>>,
    /// 最多判定数（S7：供 ScoringV1 初始化）
    pub max_combo: i32,
    /// 一次性信号编号分配器（对齐 C# `AutomatonManager._nextSignalId`）。
    /// RuntimeInitialize 置 1，RuntimeDestruct 归零。
    pub next_signal_id: i32,
}

impl AutomatonManager {
    pub fn new() -> Self {
        Self {
            input_signals: MultichannelSplit::new(),
            automatons: Vec::new(),
            pending_detection_conditions: std::collections::HashMap::new(),
            max_combo: 0,
            next_signal_id: 0,
        }
    }

    /// 分配新的信号编号（对齐 C# `GetDisposableSignalId`）
    ///
    /// 一次性使用，只是保证历史不重复。
    pub fn get_disposable_signal_id(&mut self) -> i32 {
        let id = self.next_signal_id;
        self.next_signal_id += 1;
        id
    }

    /// 追加信号边沿（对齐 C# `AutomatonManager.AddSignalEdge`）
    ///
    /// `value` 为信号值对象 ID，0 表示 null（终止信号）。
    /// 返回值表示是否真的追加了边沿。
    pub fn add_signal_edge(&mut self, channel_name: &str, signal_id: i32, time: f32, value: usize) -> bool {
        use std::collections::hash_map::Entry;

        let channel = match self.input_signals.entry(channel_name.to_string()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(ChannelSplit::new()),
        };

        match channel.entry(signal_id) {
            Entry::Vacant(v) => {
                // 分支1：无信号且 value 为空 → 追加失败
                if value == 0 {
                    return false;
                }
                // 分支2：无信号到有信号 → 新建 Fragment
                let fragment = Fragment {
                    signal_id,
                    start_time: time,
                    end_time: f32::INFINITY,
                    start_value: value,
                    edges: Vec::new(),
                };
                v.insert(fragment);
                true
            }
            Entry::Occupied(mut o) => {
                let signal = o.get_mut();
                // 分支3：有信号到无信号 → 终止信号
                if value == 0 {
                    signal.end_time = time;
                    return false;
                }

                // 有信号到有信号：计算当前最新信号值（无边沿取 start_value，否则取最后边沿值）
                let latest_value = signal.edges.last().map(|e| e.value).unwrap_or(signal.start_value);
                if latest_value == value {
                    // 信号值一致
                    // 分支4a：end_time >= time → 无需延续
                    if signal.end_time >= time {
                        return false;
                    }
                    // 分支4b：延续到当前时间
                    signal.end_time = time;
                    return true;
                }

                // 分支4c：信号值不一致 → 追加新边沿
                signal.end_time = f32::INFINITY;
                signal.edges.push(Edge::new(time, value));
                true
            }
        }
    }

    /// 获取输入信号的时间切片（对齐 C# `AutomatonManager.SplitInputSignals`）
    ///
    /// 左开右闭区间 `(from, to]`。
    /// 遍历全部信道的全部信号片段，调用 Fragment::split 切出子片段。
    pub fn split_input_signals(&self, from: f32, to: f32) -> MultichannelSplit {
        let mut result = MultichannelSplit::new();
        for (channel_name, channel_signals) in &self.input_signals {
            let mut split_channel = ChannelSplit::new();
            for (signal_id, fragment) in channel_signals {
                if let Some(split_fragment) = fragment.split(from, to) {
                    split_channel.insert(*signal_id, split_fragment);
                }
            }
            if !split_channel.is_empty() {
                result.insert(channel_name.clone(), split_channel);
            }
        }
        result
    }

    /// 计算执行时间点后（不包含）的最早边沿时间（对齐 C# `AutomatonManager.GetInputSignalEarliestEdgeTimeAfter`）
    ///
    /// 遍历全部信道全部片段全部边沿，取最小 > `time` 的边沿时间。
    /// 无边沿则返回 `f32::MAX`。
    pub fn get_input_signal_earliest_edge_time_after(&self, time: f32) -> f32 {
        if self.input_signals.is_empty() {
            return f32::MAX;
        }

        let mut earliest = f32::MAX;
        for channel_signals in self.input_signals.values() {
            if channel_signals.is_empty() { continue; }
            for fragment in channel_signals.values() {
                if fragment.edges.is_empty() { continue; }
                for edge in &fragment.edges {
                    if edge.time > time && edge.time < earliest {
                        earliest = edge.time;
                    }
                }
            }
            // 也检查片段起始时间
            for fragment in channel_signals.values() {
                if fragment.start_time > time && fragment.start_time < earliest {
                    earliest = fragment.start_time;
                }
            }
        }
        earliest
    }
}

/// 音频管理器（E-3 实体化）
///
/// 对齐 C# `Runtime/Environment/AudioManager.cs`。
/// 管理运行时音效播放和时段音频源。
pub struct AudioManager {
    /// 时段音频源表：音频时段对象 ID → 平台播放器 ID
    pub period_audio_sources: std::collections::HashMap<usize, usize>,
    /// 响应音效表：音效名 → 音效播放器 ID
    pub respond_effects: std::collections::HashMap<String, usize>,
    /// 时段音频播放器（持有所有权，player_id → Box<dyn IAudioPlayer>）
    period_players: std::collections::HashMap<usize, Box<dyn crate::adaptor::IAudioPlayer>>,
    /// 音效播放器（持有所有权，player_id → Box<dyn IAudioEffectPlayer>）
    effect_players: std::collections::HashMap<usize, Box<dyn crate::adaptor::IAudioEffectPlayer>>,
    /// 播放器自增 ID
    next_player_id: usize,
    /// 乐段数据缓存（P1-3：StopSimulation 销毁播放器后，StartSimulation 按缓存重建）
    cached_periods: Vec<AudioPeriodData>,
    /// 即时音效缓存（P1-3：音效名 → AudioAsset 对象 ID，
    /// 由 RuntimeManager 在 load_score / start_simulation 时从 Score.InstantAudio 播种）
    cached_instant_audio: Vec<(String, usize)>,
}

impl std::fmt::Debug for AudioManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioManager")
            .field("period_audio_sources", &self.period_audio_sources)
            .field("respond_effects", &self.respond_effects)
            .field("period_players_count", &self.period_players.len())
            .field("effect_players_count", &self.effect_players.len())
            .field("cached_periods_count", &self.cached_periods.len())
            .field("cached_instant_audio_count", &self.cached_instant_audio.len())
            .finish()
    }
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            period_audio_sources: std::collections::HashMap::new(),
            respond_effects: std::collections::HashMap::new(),
            period_players: std::collections::HashMap::new(),
            effect_players: std::collections::HashMap::new(),
            next_player_id: 1,
            cached_periods: Vec::new(),
            cached_instant_audio: Vec::new(),
        }
    }

    /// 分配播放器 ID
    fn alloc_id(&mut self) -> usize {
        let id = self.next_player_id;
        self.next_player_id += 1;
        id
    }

    /// 播放响应音效（对齐 C# `PlayRespondEffect`）
    pub fn play_respond_effect(&self, name: &str) {
        if let Some(effect_id) = self.respond_effects.get(name) {
            if let Some(effect) = self.effect_players.get(effect_id) {
                effect.play();
            }
        }
    }

    /// 停止所有音乐（对齐 C# `StopAllSong`）
    pub fn stop_all_song(&self) {
        for player in self.period_players.values() {
            player.stop();
        }
    }

    /// 添加音频时段（对齐 C# `AddPeriod`）
    ///
    /// 乐段数据会缓存到 `cached_periods`，供 StopSimulation 销毁播放器后
    /// StartSimulation 重建（对齐 C# StartSimulation 遍历 AudioStaff 重建播放器表）。
    pub fn add_period(&mut self, period_id: usize, audio_id: usize, time_offset: f32) {
        // 同 period_id 已存在则替换缓存条目，避免重复
        self.cached_periods.retain(|p| p.period_id != period_id);
        self.cached_periods.push(AudioPeriodData::new(period_id, audio_id, time_offset));
        self.create_period_player(period_id);
    }

    /// 创建乐段播放器并登记两表（`add_period` 与 `start_simulation` 复用）
    fn create_period_player(&mut self, period_id: usize) {
        let player = crate::adaptor::platform().create_audio_player();
        let handle = self.alloc_id();
        self.period_players.insert(handle, player);
        self.period_audio_sources.insert(period_id, handle);
    }

    /// 移除音频时段（对齐 C# `RemovePeriod`）
    pub fn remove_period(&mut self, period_id: usize) {
        self.cached_periods.retain(|p| p.period_id != period_id);
        if let Some(handle) = self.period_audio_sources.remove(&period_id) {
            if let Some(player) = self.period_players.remove(&handle) {
                player.destruct();
            }
        }
    }

    /// 按乐段 ID 获取对应的音频播放器引用（对齐 C# `PeriodAudioSources[period]`）。
    ///
    /// 经 `period_audio_sources`（period_id → 播放器句柄）解析句柄后，
    /// 从 `period_players`（句柄 → 播放器）取得播放器；乐段不存在时返回 None。
    /// 供 SongSimulator 控制播放/停止使用（只读，不改动播放器所有权）。
    pub fn period_player(&self, period_id: usize) -> Option<&dyn crate::adaptor::IAudioPlayer> {
        let handle = *self.period_audio_sources.get(&period_id)?;
        self.period_players.get(&handle).map(|p| p.as_ref())
    }

    /// 获取乐段的时间偏移（秒），对齐 C# `period.Config.timeOffset`。
    ///
    /// 从缓存乐段数据（`cached_periods`）读取提取时记录的 time_offset；
    /// 乐段未缓存时回退为 0。
    pub fn period_time_offset(&self, period_id: usize) -> f32 {
        self.cached_periods.iter()
            .find(|p| p.period_id == period_id)
            .map(|p| p.time_offset)
            .unwrap_or(0.0)
    }

    /// 测试辅助：直接注入一个乐段音频源（绕过平台工厂创建）。
    ///
    /// 仅在 `#[cfg(test)]` 下可用，供 SongSimulator 单测注入可控制的
    /// 播放器 mock 并登记 period_id → handle → player 与 time_offset 缓存。
    #[cfg(test)]
    pub fn inject_audio_source_for_test(
        &mut self,
        period_id: usize,
        time_offset: f32,
        player: Box<dyn crate::adaptor::IAudioPlayer>,
    ) {
        let handle = self.alloc_id();
        self.period_players.insert(handle, player);
        self.period_audio_sources.insert(period_id, handle);
        self.cached_periods.retain(|p| p.period_id != period_id);
        self.cached_periods.push(AudioPeriodData::new(period_id, 0, time_offset));
    }

    /// 缓存即时音效表（P1-3）
    ///
    /// 由 RuntimeManager 在 load_score / start_simulation 时从
    /// `Score.InstantAudio` 提取（音效名 → AudioAsset 对象 ID）后播种，
    /// `start_simulation` 据此创建音效播放器。
    pub fn cache_instant_audio(&mut self, entries: Vec<(String, usize)>) {
        self.cached_instant_audio = entries;
    }

    /// 启动仿真（对齐 C# `AudioManager.StartSimulation`，P1-3 实体化）
    ///
    /// 1. 重建响应音效表：遍历缓存的即时音效，对 AudioAsset 对象依次调用
    ///    `LoadAsset`（0 号方法）/ `GetAsset`（1 号方法）取得 Audio 对象，
    ///    解析平台音频句柄后经平台创建音效播放器；
    /// 2. 重建乐段播放器表：按缓存的乐段数据补齐缺失的播放器（只创建不播放，
    ///    播放由 SongSimulator 负责）。
    ///
    /// 平台未安装时直接返回（无平台的测试环境不创建任何播放器）。
    pub fn start_simulation(&mut self, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        if !crate::adaptor::platform_installed() {
            return;
        }

        // 1. 重建响应音效表
        self.clear_respond_effects();
        let instant_audio = self.cached_instant_audio.clone();
        // AudioAsset native 类未注册时无法走 LoadAsset 链路（裸 VM 测试环境），整体跳过
        let audio_asset_registered = vm.native_class_table.contains_key("GorgeFramework.AudioAsset");
        for (name, asset_object_id) in instant_audio {
            if asset_object_id == 0 || !audio_asset_registered {
                continue;
            }
            // LoadAsset：资产未找到 / 非音频资产族时返回 false（对齐 C# try/catch 语义）
            let loaded = {
                let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
                ctx.set_bool_return(false);
                ctx.invoke_native_method_on("GorgeFramework.AudioAsset", asset_object_id, 0);
                ctx.vm.param_pool.get_bool_return()
            };
            if !loaded {
                continue;
            }
            // GetAsset：取缓存的 Audio 对象 ID（允许为 0，为 0 则跳过播放器创建）
            let audio_object_id = {
                let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
                ctx.set_object_return(0);
                ctx.invoke_native_method_on("GorgeFramework.AudioAsset", asset_object_id, 1);
                ctx.vm.param_pool.get_object_return()
            };
            if audio_object_id == 0 {
                continue;
            }
            let vm_address = &*vm as *const gorge_core::virtual_machine::vm::VirtualMachine as usize;
            let audio_handle = crate::runtime::environment::global::resolve_audio_handle(vm_address, audio_object_id);
            self.register_respond_effect(name, audio_handle);
        }

        // 2. 重建乐段播放器表（补齐 StopSimulation 销毁后缺失的播放器）
        let missing_period_ids: Vec<usize> = self.cached_periods.iter()
            .map(|p| p.period_id)
            .filter(|id| !self.period_audio_sources.contains_key(id))
            .collect();
        for period_id in missing_period_ids {
            self.create_period_player(period_id);
        }
    }

    /// 停止仿真（对齐 C# `AudioManager.StopSimulation`，P1-3 实体化）
    ///
    /// 先停止所有音乐，再销毁全部音效播放器与乐段播放器并清空两表。
    /// 乐段/音效缓存保留，供下次 StartSimulation 重建。
    pub fn stop_simulation(&mut self) {
        self.stop_all_song();
        self.clear_respond_effects();
        self.clear_audio_sources();
    }

    /// 注册响应音效（对齐 C# `StartSimulation` 中的效果注册部分）
    pub fn register_respond_effect(&mut self, name: String, audio_id: usize) {
        let effect = crate::adaptor::platform().create_audio_effect_player(audio_id);
        let handle = self.alloc_id();
        self.effect_players.insert(handle, effect);
        self.respond_effects.insert(name, handle);
    }

    /// 清理所有音效资源（对齐 C# `StopSimulation`：逐个 Destruct 后清空）
    pub fn clear_respond_effects(&mut self) {
        for effect in self.effect_players.values() {
            effect.destruct();
        }
        self.respond_effects.clear();
        self.effect_players.clear();
    }

    /// 清理所有音频时段（对齐 C# `StopSimulation`：逐个 Destruct 后清空）
    pub fn clear_audio_sources(&mut self) {
        for player in self.period_players.values() {
            player.destruct();
        }
        self.period_audio_sources.clear();
        self.period_players.clear();
    }
}

/// 图形管理器（S4-6：nodes 表）
#[derive(Debug)]
pub struct GraphicsManager {
    /// 存活图形节点对象 ID 列表
    pub nodes: Vec<usize>,
}

impl GraphicsManager {
    pub fn new() -> Self { Self { nodes: Vec::new() } }
}

/// 资源管理器
///
/// 维护资产名 → 资产对象 ID 的注册表（对齐 C# `Environment.GetAssetByName`）。
#[derive(Debug)]
pub struct AssetManager {
    /// 资产名 → 资产对象 ID
    pub assets: std::collections::HashMap<String, usize>,
}

impl AssetManager {
    pub fn new() -> Self { Self { assets: std::collections::HashMap::new() } }

    /// 注册资产
    pub fn register(&mut self, name: String, object_id: usize) {
        self.assets.insert(name, object_id);
    }

    /// 按名称查找资产对象 ID
    pub fn get_asset_by_name(&self, name: &str) -> Option<usize> {
        self.assets.get(name).copied()
    }
}

/// 场景管理器（骨架）
#[derive(Debug)]
pub struct SceneManager;

impl SceneManager { pub fn new() -> Self { Self } }

/// 仿真日志器（骨架）
#[derive(Debug)]
pub struct SimulationLogger;

impl SimulationLogger {
    pub fn new() -> Self { Self }
    pub fn debug_log(&self, _msg: &str, _indent: usize) {}
}

// ==================== 运行时环境根容器 ====================

/// GorgeSimulationRuntime（对应 C# 同名类）
///
/// 持有全部子 Manager，统一管理 LoadScore / StartSimulation / StopSimulation 生命周期。
pub struct GorgeSimulationRuntime {
    pub chart: ChartManager,
    pub simulation: SimulationManager,
    pub automaton: AutomatonManager,
    pub audio: AudioManager,
    pub graphics: GraphicsManager,
    pub asset: AssetManager,
    pub scene: SceneManager,
    pub logger: SimulationLogger,
    /// 主模拟器注册表（内部 ID → Box<dyn ISimulator>）
    pub sim_registry: SimRegistry,
    /// 尾独立模拟器注册表
    pub late_sim_registry: SimRegistry,
    /// 计分器（S7：ScoringV1，由 add_score_element 初始化）
    pub scoring: crate::stage::ScoringV1,
    /// 当前模拟时间（由 SimulationMachine 在动作执行前同步）
    pub simulate_time: f32,
    /// 当前谱面时间（由 SimulationMachine 在动作执行前同步）
    pub chart_time: f32,
    /// F-3：谱面是否已加载
    pub is_score_loaded: bool,
    /// F-3：是否正在仿真
    pub is_simulating: bool,
    /// 谱面终结回调（对齐 C# `GorgeSimulationRuntime.OnTerminate`，P1-4）
    ///
    /// 由 `Terminate` 动作触发。创建运行时后由调用方按需赋值；
    /// Rust 无 `Action?` 默认值语义，None 表示未注册回调。
    pub on_terminate: Option<Box<dyn FnMut()>>,
}

impl GorgeSimulationRuntime {
    pub fn new() -> Self {
        let mut runtime = Self {
            chart: ChartManager::new(),
            simulation: SimulationManager::new(),
            automaton: AutomatonManager::new(),
            audio: AudioManager::new(),
            graphics: GraphicsManager::new(),
            asset: AssetManager::new(),
            scene: SceneManager::new(),
            logger: SimulationLogger::new(),
            sim_registry: SimRegistry::new(),
            late_sim_registry: SimRegistry::new(),
            scoring: crate::stage::ScoringV1::new(1),
            simulate_time: 0.0,
            chart_time: 0.0,
            is_score_loaded: false,
            is_simulating: false,
            on_terminate: None,
        };
        runtime.register_standard_simulators();
        runtime
    }

    // ==================== F-3 生命周期方法 ====================

    /// 加载谱面（对齐 C# `LoadScore` 37-48 行）
    ///
    /// 若已加载则先卸载。调用 Chart.LoadScore 读取谱面数据。
    /// `machine` 为外部持有的 SimulationMachine（S4c 拆分后与本运行时分离），
    /// 仅在需要先停止旧仿真时使用。
    pub fn load_score(
        &mut self,
        score: &SimulationScore,
        machine: &mut crate::runtime::simulation_machine::SimulationMachine,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        if self.is_score_loaded {
            self.unload_score(machine, vm);
        }
        // Chart.LoadScore —— 读取谱面
        self.chart_load_score(score, vm);
        self.is_score_loaded = true;
    }

    /// 卸载谱面（对齐 C# `UnloadScore` 50-65 行）
    ///
    /// 若未加载则直接返回。若正在仿真则先停止。
    pub fn unload_score(
        &mut self,
        machine: &mut crate::runtime::simulation_machine::SimulationMachine,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        if !self.is_score_loaded {
            return;
        }
        if self.is_simulating {
            self.stop_simulation(machine, vm);
        }
        // Chart.UnloadScore
        self.chart_unload_score();
        self.is_score_loaded = false;
    }

    /// 启动仿真（对齐 C# `StartSimulation` 67-89 行）
    ///
    /// 若未加载谱面则 panic。若已在仿真中则先停止再启动。
    /// `machine` 在 Simulation.RuntimeInitialize 阶段复位（对齐 C#
    /// `SimulationMachine.RuntimeInitialize`）。
    pub fn start_simulation(
        &mut self,
        machine: &mut crate::runtime::simulation_machine::SimulationMachine,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        if !self.is_score_loaded {
            panic!("尝试在谱面加载前启动仿真");
        }
        if self.is_simulating {
            self.stop_simulation(machine, vm);
        }

        self.logger_start_simulation();
        self.scene_runtime_initialize();
        self.audio_start_simulation(vm);
        self.graphics_start_simulation();
        self.simulation_runtime_initialize(machine);
        self.automaton_runtime_initialize();
        self.chart_start_simulation(vm);
        // Simulation.SimulationMachine.DriveInstantly —— 由 RuntimeManager::drive 负责

        self.is_simulating = true;
    }

    /// 停止仿真（对齐 C# `StopSimulation` 91-107 行）
    ///
    /// 若未在仿真中则直接返回。严格按启动的逆序析构各子系统。
    pub fn stop_simulation(
        &mut self,
        machine: &mut crate::runtime::simulation_machine::SimulationMachine,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        if !self.is_simulating {
            return;
        }

        self.chart_stop_simulation(vm);
        self.automaton_runtime_destruct();
        self.simulation_runtime_destruct(machine);
        self.graphics_stop_simulation();
        self.audio_stop_simulation();
        self.scene_runtime_destruct();
        self.logger_stop_simulation();

        self.is_simulating = false;
    }

    /// 复位后重新跳转到当前位置（对齐 C# `RePlay` 112-120 行）
    ///
    /// 记录当前 chartTime → StopSimulation → StartSimulation →
    /// SimulationMachine.DriveToChartTime(nowChartTime) → StopAllSong。
    ///
    /// `machine` 为外部持有的 SimulationMachine（S4c 拆分后与本运行时分离）。
    /// `vm` 用于 GameplayAction 执行中的 VM 操作。
    pub fn replay(
        &mut self,
        machine: &mut crate::runtime::simulation_machine::SimulationMachine,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        let now_chart_time = self.chart_time;

        self.stop_simulation(machine, vm);
        self.start_simulation(machine, vm);
        machine.drive_to_chart_time(now_chart_time, self, vm);
        self.audio.stop_all_song();
    }

    // ==================== R-5c 音频乐段注册 ====================

    /// 从谱面数据注册音频乐段到 AudioManager（R-5c）
    ///
    /// 将提取的 AudioPeriod 数据传递给 AudioManager，
    /// AudioManager 经平台创建对应音频播放器并关联时段。
    pub fn register_audio_periods(&mut self, periods: &[AudioPeriodData]) {
        for p in periods {
            self.audio.add_period(p.period_id, p.audio_id, p.time_offset);
        }
    }

    // ==================== 子管理器生命周期钩子（内部） ====================

    fn register_standard_simulators(&mut self) {
        let generator = self.sim_registry.register(Box::new(crate::simulators::impls::TimedElementGenerator));
        self.simulation.simulators.register(-1, generator);
        let destroyer = self.sim_registry.register(Box::new(crate::simulators::impls::TimedElementDestroyer));
        self.simulation.simulators.register(-1, destroyer);
        let automaton = self.sim_registry.register(Box::new(crate::simulators::impls::PreciseAutomatonSimulator));
        self.simulation.simulators.register(0, automaton);
        let song = self.sim_registry.register(Box::new(crate::simulators::impls::SongSimulator));
        self.simulation.simulators.register(10_000, song);
        let graphics = self.late_sim_registry.register(Box::new(crate::simulators::impls::GraphicsNodeSimulator));
        self.simulation.late_independent_simulators.register(100_000, graphics);
    }

    fn chart_load_score(
        &mut self,
        score: &SimulationScore,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        self.chart.load_score(score, vm);
    }

    fn chart_unload_score(&mut self) { self.chart.unload_score(); }

    fn chart_start_simulation(&mut self, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        let initial_elements = self.chart.initialize_generate_list.clone();
        let mut edge_queue = crate::signal::multichannel_edge_queue::MultichannelEdgeQueue::new();
        for (injector_id, constructor_id) in initial_elements {
            crate::simulators::impls::GenerateElement {
                injector_id,
                constructor_id,
                is_auto_play: false,
                is_reverse: false,
                direction: crate::runtime::simulation_types::SimulateDirection::Infinitesimal,
            }.do_action(self, &mut edge_queue, vm);
        }
    }
    /// 停止谱面子系统（对齐 C# `ChartManager.StopSimulation` 239-255 行，P1-3 实体化）
    ///
    /// 对全部存活元素逐个执行 `DestroyElement`，然后清空五张运行期表。
    /// 加载期的生成表（initialize_generate_list / 正反向定时创生表）保留，
    /// 供 RePlay / 再次 StartSimulation 重新创生。
    fn chart_stop_simulation(&mut self, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        let alive_elements = self.chart.alive_elements.clone();
        let mut edge_queue = crate::signal::multichannel_edge_queue::MultichannelEdgeQueue::new();
        for element_id in alive_elements {
            crate::simulators::impls::DestroyElement::new(element_id).do_action(self, &mut edge_queue, vm);
        }
        // C# 将五张运行期表置 null；Rust 无 null 语义，显式清空
        // （DestroyElement 已逐元素移除条目，此处兜底）
        self.chart.alive_elements.clear();
        self.chart.alive_notes.clear();
        self.chart.alive_injector_map.clear();
        self.chart.element_simulator_keys.clear();
        self.chart.forward_timed_destroy_list.clear();
        self.chart.backward_timed_destroy_list.clear();
    }

    /// 启动音频子系统（对齐 C# `AudioManager.StartSimulation`，P1-3 实体化）
    fn audio_start_simulation(&mut self, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        self.audio.start_simulation(vm);
    }

    /// 停止音频子系统（对齐 C# `AudioManager.StopSimulation`，P1-3 实体化）
    fn audio_stop_simulation(&mut self) {
        self.audio.stop_simulation();
    }

    /// 启动图形子系统（对齐 C# `GraphicsManager.StartSimulation`：`Nodes = new List`）
    ///
    /// C# 不销毁精灵（精灵句柄在 native 节点载荷中），Rust 同样只清空节点表。
    fn graphics_start_simulation(&mut self) {
        self.graphics.nodes.clear();
    }

    /// 停止图形子系统（对齐 C# `GraphicsManager.StopSimulation`：`Nodes = null`）
    fn graphics_stop_simulation(&mut self) {
        self.graphics.nodes.clear();
    }

    /// 初始化自动机子系统（对齐 C# `AutomatonManager.RuntimeInitialize` 17-23 行，P1-3 实体化）
    fn automaton_runtime_initialize(&mut self) {
        self.automaton.automatons.clear();
        self.automaton.pending_detection_conditions.clear();
        self.automaton.input_signals.clear();
        // C# `_nextSignalId = 1`：信号编号从 1 开始分配
        self.automaton.next_signal_id = 1;
    }

    /// 析构自动机子系统（对齐 C# `AutomatonManager.RuntimeDestruct` 25-31 行，P1-3 实体化）
    fn automaton_runtime_destruct(&mut self) {
        self.automaton.automatons.clear();
        self.automaton.pending_detection_conditions.clear();
        self.automaton.input_signals.clear();
        // C# `_nextSignalId = default`（即 0）
        self.automaton.next_signal_id = 0;
    }

    /// 初始化仿真子系统（对齐 C# `SimulationManager.RuntimeInitialize` 67-89 行，P1-3 实体化）
    ///
    /// 重建两张优先级堆与模拟器注册表，重新注册标准模拟器，
    /// 并复位 SimulationMachine（对齐 C# `SimulationMachine.RuntimeInitialize`）。
    fn simulation_runtime_initialize(&mut self, machine: &mut crate::runtime::simulation_machine::SimulationMachine) {
        self.simulation.simulators.destruct();
        self.simulation.late_independent_simulators.destruct();
        self.sim_registry.clear();
        self.late_sim_registry.clear();
        self.register_standard_simulators();
        machine.runtime_initialize();
    }

    /// 析构仿真子系统（对齐 C# `SimulationManager.RuntimeDestruct` 91-102 行，P1-3 实体化）
    fn simulation_runtime_destruct(&mut self, machine: &mut crate::runtime::simulation_machine::SimulationMachine) {
        self.simulation.simulators.destruct();
        self.simulation.late_independent_simulators.destruct();
        self.sim_registry.clear();
        self.late_sim_registry.clear();
        machine.runtime_destruct();
    }

    fn scene_runtime_initialize(&mut self) {
        // 对齐 C# SceneManager.RuntimeInitialize → new ScoringV1(1395)
        self.scoring = crate::stage::ScoringV1::new(1395);
    }

    /// 析构场景子系统（对齐 C# `SceneManager.RuntimeDestruct`，C# 本为空实现）
    fn scene_runtime_destruct(&mut self) {}

    fn logger_start_simulation(&mut self) {
        self.logger.debug_log("仿真开始", 0);
    }

    fn logger_stop_simulation(&mut self) {
        self.logger.debug_log("仿真停止", 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_score() -> SimulationScore {
        SimulationScore::new(0.0, 100.0, 1.0)
    }

    /// 构造测试用 SimulationMachine（与 empty_score 的时间范围一致）
    fn test_machine() -> crate::runtime::simulation_machine::SimulationMachine {
        crate::runtime::simulation_machine::SimulationMachine::new(0.0, 100.0, 1.0)
    }

    // ==================== AutomatonManager 测试 ====================

    #[test]
    fn test_add_signal_edge_new_channel_new_signal() {
        // 分支2：无信道 + 有值 → 新建信号，返回 true
        let mut mgr = AutomatonManager::new();
        assert!(mgr.add_signal_edge("Touch", 1, 0.0, 100));
        assert!(mgr.input_signals.contains_key("Touch"));
        let ch = &mgr.input_signals["Touch"];
        assert!(ch.contains_key(&1));
        let frag = &ch[&1];
        assert_eq!(frag.signal_id, 1);
        assert_eq!(frag.start_time, 0.0);
        assert_eq!(frag.start_value, 100);
        assert!(frag.end_time.is_infinite());
        assert!(frag.edges.is_empty());
    }

    #[test]
    fn test_add_signal_edge_null_value_no_signal() {
        // 分支1：无信号且 value=0 → 返回 false
        let mut mgr = AutomatonManager::new();
        assert!(!mgr.add_signal_edge("Touch", 1, 0.0, 0));
        // 信道被创建但无信号
        assert!(mgr.input_signals.contains_key("Touch"));
        assert!(!mgr.input_signals["Touch"].contains_key(&1));
    }

    #[test]
    fn test_add_signal_edge_terminate_signal() {
        // 分支3：有信号到无信号 → 终止信号，返回 false
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("Touch", 1, 0.0, 100);
        assert!(!mgr.add_signal_edge("Touch", 1, 5.0, 0));
        assert_eq!(mgr.input_signals["Touch"][&1].end_time, 5.0);
    }

    #[test]
    fn test_add_signal_edge_same_value_not_expired() {
        // 分支4a：信号值一致 + end_time >= time → 不追加，返回 false
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("Touch", 1, 0.0, 100);
        // end_time 是 INF，>= 3.0
        assert!(!mgr.add_signal_edge("Touch", 1, 3.0, 100));
        // 信号未被修改
        assert!(mgr.input_signals["Touch"][&1].end_time.is_infinite());
    }

    #[test]
    fn test_add_signal_edge_same_value_expired() {
        // 分支4b：信号值一致 + end_time < time → 延续到当前时间
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("Touch", 1, 0.0, 100);
        // 先终止
        mgr.add_signal_edge("Touch", 1, 3.0, 0);
        assert_eq!(mgr.input_signals["Touch"][&1].end_time, 3.0);
        // 再以同值 "复活"，end_time=3.0 < 5.0 → 延续
        assert!(mgr.add_signal_edge("Touch", 1, 5.0, 100));
        assert_eq!(mgr.input_signals["Touch"][&1].end_time, 5.0);
    }

    #[test]
    fn test_add_signal_edge_different_value() {
        // 分支4c：信号值不一致 → 追加新边沿
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("Touch", 1, 0.0, 100);
        assert!(mgr.add_signal_edge("Touch", 1, 2.0, 200));
        let frag = &mgr.input_signals["Touch"][&1];
        assert!(frag.end_time.is_infinite());
        assert_eq!(frag.edges.len(), 1);
        assert_eq!(frag.edges[0].time, 2.0);
        assert_eq!(frag.edges[0].value, 200);
    }

    #[test]
    fn test_add_signal_edge_multiple_channels() {
        // 多信道独立
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("Touch", 1, 0.0, 100);
        mgr.add_signal_edge("Keyboard", 2, 1.0, 42);
        assert_eq!(mgr.input_signals.len(), 2);
        assert_eq!(mgr.input_signals["Touch"][&1].start_value, 100);
        assert_eq!(mgr.input_signals["Keyboard"][&2].start_value, 42);
    }

    #[test]
    fn test_split_input_signals_empty() {
        let mgr = AutomatonManager::new();
        let split = mgr.split_input_signals(0.0, 10.0);
        assert!(split.is_empty());
    }

    #[test]
    fn test_split_input_signals_with_signal() {
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("Touch", 1, 0.0, 100);
        mgr.add_signal_edge("Touch", 1, 3.0, 200);
        mgr.add_signal_edge("Touch", 1, 5.0, 0); // 终止

        let split = mgr.split_input_signals(1.0, 4.0);
        assert!(split.contains_key("Touch"));
        let ch = &split["Touch"];
        assert!(ch.contains_key(&1));
        let frag = &ch[&1];
        // start_time=1.0, start_value 应该是 sample(1.0) = 100（原始信号末边沿≤1.0=无→start_value=100）
        assert_eq!(frag.start_value, 100);
        // edges: 只有 3.0 的边沿在 (1.0,4.0] 区间，5.0 的不在
        assert_eq!(frag.edges.len(), 1);
        assert_eq!(frag.edges[0].time, 3.0);
        assert_eq!(frag.edges[0].value, 200);
    }

    #[test]
    fn test_get_input_signal_earliest_edge_time_empty() {
        let mgr = AutomatonManager::new();
        assert_eq!(mgr.get_input_signal_earliest_edge_time_after(0.0), f32::MAX);
    }

    #[test]
    fn test_get_input_signal_earliest_edge_time_with_edges() {
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("A", 1, 0.0, 100);
        mgr.add_signal_edge("A", 1, 1.5, 200); // edge at 1.5
        mgr.add_signal_edge("A", 1, 3.0, 300); // edge at 3.0

        // >1.0 的最早边沿是 1.5
        assert!((mgr.get_input_signal_earliest_edge_time_after(1.0) - 1.5).abs() < 0.001);
        // >2.0 的最早边沿是 3.0
        assert!((mgr.get_input_signal_earliest_edge_time_after(2.0) - 3.0).abs() < 0.001);
        // >5.0 无边沿
        assert_eq!(mgr.get_input_signal_earliest_edge_time_after(5.0), f32::MAX);
    }

    #[test]
    fn test_get_input_signal_earliest_edge_time_multi_channel() {
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("A", 1, 0.0, 100);
        mgr.add_signal_edge("A", 1, 2.0, 200);
        mgr.add_signal_edge("B", 2, 0.0, 50);
        mgr.add_signal_edge("B", 2, 1.0, 150);

        // >0.5 的最早边沿应该是 1.0（B 信道）
        assert!((mgr.get_input_signal_earliest_edge_time_after(0.5) - 1.0).abs() < 0.001);
    }

    // ==================== F-3 GorgeSimulationRuntime 生命周期测试 ====================

    #[test]
    fn test_f3_lifecycle_full_sequence() {
        let mut rt = GorgeSimulationRuntime::new();
        let score = empty_score();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        let mut machine = test_machine();
        // 初始状态
        assert!(!rt.is_score_loaded);
        assert!(!rt.is_simulating);

        rt.load_score(&score, &mut machine, &mut vm);
        assert!(rt.is_score_loaded);
        assert!(!rt.is_simulating);

        rt.start_simulation(&mut machine, &mut vm);
        assert!(rt.is_score_loaded);
        assert!(rt.is_simulating);

        rt.stop_simulation(&mut machine, &mut vm);
        assert!(rt.is_score_loaded);
        assert!(!rt.is_simulating);

        rt.unload_score(&mut machine, &mut vm);
        assert!(!rt.is_score_loaded);
        assert!(!rt.is_simulating);
    }

    #[test]
    fn test_f3_double_load_rejects_previous() {
        let mut rt = GorgeSimulationRuntime::new();
        let score = empty_score();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        let mut machine = test_machine();
        rt.load_score(&score, &mut machine, &mut vm);
        assert!(rt.is_score_loaded);

        // 第二次 load 应先 unload 再 load
        rt.load_score(&score, &mut machine, &mut vm);
        assert!(rt.is_score_loaded);
    }

    #[test]
    #[should_panic(expected = "尝试在谱面加载前启动仿真")]
    fn test_f3_start_before_load_panics() {
        let mut rt = GorgeSimulationRuntime::new();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        let mut machine = test_machine();
        rt.start_simulation(&mut machine, &mut vm);
    }

    #[test]
    fn test_f3_unload_while_simulating_stops_first() {
        let mut rt = GorgeSimulationRuntime::new();
        let score = empty_score();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        let mut machine = test_machine();
        rt.load_score(&score, &mut machine, &mut vm);
        rt.start_simulation(&mut machine, &mut vm);
        assert!(rt.is_simulating);

        rt.unload_score(&mut machine, &mut vm);
        assert!(!rt.is_simulating);
        assert!(!rt.is_score_loaded);
    }

    #[test]
    fn test_chart_score_load_and_initialize_generate() {
        use gorge_core::diagnostics::Span;
        use gorge_core::objective::class::RuntimeClass;
        use gorge_core::objective::declaration::{InjectorFieldInfo, MethodAnnotation};
        use gorge_core::objective::types::{GorgeType, TypeCount};
        use gorge_core::virtual_machine::ir::{CodeWithSpan, CompiledMethod, IntermediateCode, IntermediateOperator, Operand};
        use std::collections::HashMap;
        use std::sync::Arc;

        let mut declaration = gorge_core::objective::declaration::ClassDeclaration::dummy("Demo.ScoreElement".into());
        declaration.injector_fields.push(InjectorFieldInfo {
            name: "value".into(),
            field_type: GorgeType::new(BasicType::Int),
            has_default_value: false,
        });
        declaration.injector_field_type_count = TypeCount { int_count: 1, ..TypeCount::zero() };
        declaration.field_type_count = TypeCount { object_count: 5, ..TypeCount::zero() };
        declaration.constructor_count = 1;
        declaration.constructor_annotations = HashMap::from([(
            0,
            vec![MethodAnnotation { name: "InitializeGenerate".into(), parameters: vec![] }],
        )]);

        let mut runtime_class = RuntimeClass::new(declaration, None);
        runtime_class.register_constructor(0, CompiledMethod {
            name: "ctor".into(),
            codes: vec![CodeWithSpan::new(IntermediateCode {
                result: None,
                operator: IntermediateOperator::ReturnInt,
                left: Operand::int(0),
                right: None,
            }, Span::dummy())],
            local_count: 0,
        });

        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        // P0-5：注册父类链（GorgeFramework.Element）
        vm.register_class_super("ScoreElement", "GorgeFramework.Element");
        vm.class_table.insert("ScoreElement".into(), Arc::new(runtime_class));
        let score = SimulationScore::load_score_from_element_list(
            "TestForm",
            vec![serde_json::json!({ "__type": "Demo.ScoreElement", "value": 7 })],
            vec![],
            1.0,
            10.0,
            1.5,
        );
        let mut runtime = GorgeSimulationRuntime::new();
        let mut machine = test_machine();

        runtime.load_score(&score, &mut machine, &mut vm);
        assert_eq!(runtime.chart.begin_chart_time, 1.0);
        assert_eq!(runtime.chart.initialize_generate_list.len(), 1);
        // 注入器 = 谱面记载 1 个 + gameplay 修改版 clone 1 个
        assert_eq!(vm.injectors.len(), 2);

        runtime.start_simulation(&mut machine, &mut vm);
        assert_eq!(runtime.chart.alive_elements.len(), 1);

        // P1-3：stop 后五张运行期表清空，加载期生成表保留
        runtime.stop_simulation(&mut machine, &mut vm);
        assert!(runtime.chart.alive_elements.is_empty());
        assert!(runtime.chart.alive_notes.is_empty());
        assert!(runtime.chart.alive_injector_map.is_empty());
        assert!(runtime.chart.forward_timed_destroy_list.is_empty());
        assert!(runtime.chart.backward_timed_destroy_list.is_empty());
        assert_eq!(runtime.chart.initialize_generate_list.len(), 1);
    }

    #[test]
    fn test_p0_4_period_modifier_applied_along_inheritance_chain() {
        use gorge_core::diagnostics::Span;
        use gorge_core::objective::class::RuntimeClass;
        use gorge_core::objective::declaration::{InjectorFieldInfo, MethodAnnotation};
        use gorge_core::objective::types::{GorgeType, TypeCount};
        use gorge_core::virtual_machine::ir::{Address, CodeWithSpan, CompiledMethod, IntermediateCode, IntermediateOperator, Operand, ValueType};
        use std::collections::HashMap;
        use std::sync::Arc;

        // 基类：float 注入器字段 fvalue + @PeriodModifier 静态方法
        // 方法体：periodConfig.timeOffset（object 参数 1 的 float 字段 0）加到注入器 fvalue 上
        let mut base_decl = gorge_core::objective::declaration::ClassDeclaration::dummy("Demo.BaseElement".into());
        base_decl.injector_fields.push(InjectorFieldInfo {
            name: "fvalue".into(),
            field_type: GorgeType::new(BasicType::Float),
            has_default_value: false,
        });
        base_decl.injector_field_type_count = TypeCount { float_count: 1, ..TypeCount::zero() };
        base_decl.method_count = 1;
        base_decl.method_start_id = 0;
        base_decl.method_annotations = HashMap::from([(
            0, // 全局 ID = method_start_id + 0
            vec![MethodAnnotation { name: "PeriodModifier".into(), parameters: vec![] }],
        )]);

        let modifier_body = CompiledMethod {
            name: "PeriodModifier".into(),
            codes: vec![
                // periodConfig = LoadObjectParameter(1) → object[0]
                CodeWithSpan::new(IntermediateCode {
                    result: Some(Address::new(ValueType::Object, 0)),
                    operator: IntermediateOperator::LoadObjectParameter,
                    left: Operand::int(1), right: None,
                }, Span::dummy()),
                // offset = periodConfig.timeOffset（float 字段 0）→ float[0]
                CodeWithSpan::new(IntermediateCode {
                    result: Some(Address::new(ValueType::Float, 0)),
                    operator: IntermediateOperator::LoadFloatField(0),
                    left: Operand::Address(Address::new(ValueType::Object, 0)), right: None,
                }, Span::dummy()),
                // inj = LoadInjector（current_injector = gameplay 修改版）→ object[1]
                CodeWithSpan::new(IntermediateCode {
                    result: Some(Address::new(ValueType::Object, 1)),
                    operator: IntermediateOperator::LoadInjector,
                    left: Operand::int(0), right: None,
                }, Span::dummy()),
                // cur = inj.^fvalue → float[1]
                CodeWithSpan::new(IntermediateCode {
                    result: Some(Address::new(ValueType::Float, 1)),
                    operator: IntermediateOperator::LoadFloatInjectorField(0),
                    left: Operand::Address(Address::new(ValueType::Object, 1)), right: None,
                }, Span::dummy()),
                // sum = cur + offset → float[2]
                CodeWithSpan::new(IntermediateCode {
                    result: Some(Address::new(ValueType::Float, 2)),
                    operator: IntermediateOperator::FloatAdd,
                    left: Operand::Address(Address::new(ValueType::Float, 1)),
                    right: Some(Operand::Address(Address::new(ValueType::Float, 0))),
                }, Span::dummy()),
                // inj.^fvalue = sum（SetFloatInjectorField: left=值, right=注入器）
                CodeWithSpan::new(IntermediateCode {
                    result: None,
                    operator: IntermediateOperator::SetFloatInjectorField(0),
                    left: Operand::Address(Address::new(ValueType::Float, 2)),
                    right: Some(Operand::Address(Address::new(ValueType::Object, 1))),
                }, Span::dummy()),
                CodeWithSpan::new(IntermediateCode {
                    result: None,
                    operator: IntermediateOperator::ReturnVoid,
                    left: Operand::int(0), right: None,
                }, Span::dummy()),
            ],
            local_count: 4,
        };
        let mut base_class = RuntimeClass::new(base_decl, None);
        base_class.register_method(0, modifier_body.clone());

        // 子类：继承基类（含注入器字段合并 + 声明链），带 int 注入器字段 value 和 @InitializeGenerate 构造
        let mut child_decl = gorge_core::objective::declaration::ClassDeclaration::dummy("Demo.ChildElement".into());
        child_decl.super_class = Some(Box::new(base_class.declaration.clone()));
        child_decl.injector_fields.push(InjectorFieldInfo {
            name: "value".into(),
            field_type: GorgeType::new(BasicType::Int),
            has_default_value: false,
        });
        child_decl.injector_field_type_count = TypeCount {
            int_count: 1, float_count: 1, ..TypeCount::zero()
        };
        child_decl.constructor_count = 1;
        child_decl.constructor_annotations = HashMap::from([(
            0,
            vec![MethodAnnotation { name: "InitializeGenerate".into(), parameters: vec![] }],
        )]);
        let mut child_class = RuntimeClass::new(child_decl, Some(Arc::new(base_class.clone())));
        child_class.register_constructor(0, CompiledMethod {
            name: "ctor".into(),
            codes: vec![CodeWithSpan::new(IntermediateCode {
                result: None,
                operator: IntermediateOperator::ReturnInt,
                left: Operand::int(0),
                right: None,
            }, Span::dummy())],
            local_count: 0,
        });

        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        let child_class = Arc::new(child_class);
        let base_class = Arc::new(base_class);
        // P0-5：注册父类链（Demo.ChildElement → Demo.BaseElement → GorgeFramework.Element）
        vm.register_class_super("Demo.ChildElement", "Demo.BaseElement");
        vm.register_class_super("Demo.BaseElement", "GorgeFramework.Element");
        vm.class_table.insert("Demo.ChildElement".into(), child_class.clone());
        vm.class_table.insert("Demo.BaseElement".into(), base_class.clone());
        // @PeriodModifier 是静态方法，须注册进 class_static_methods（静态方法表）
        // 供 modify_injector 经 invoke_static_method_by_global_id 分派。
        vm.register_class_methods("Demo.BaseElement", vec![(modifier_body.clone(), vec![])]);

        // 谱面记载注入器：fvalue=1.0、value=7
        let original_id = vm.next_object_id;
        vm.next_object_id += 1;
        let mut injector = RuntimeInjector::new(Arc::new(child_class.declaration.clone()));
        injector.set_injector_float(0, 1.0);
        injector.set_injector_int(0, 7);
        vm.injectors.insert(original_id, injector);

        let mut runtime = GorgeSimulationRuntime::new();
        let period_config = crate::chart::period::PeriodConfig {
            time_offset: 2.5,
            min_length: 10.0,
            active: true,
        };
        runtime.chart.add_score_element(original_id, &period_config, &mut vm);

        // 1. 生成表使用 gameplay 修改版注入器（clone 的下一 ID）
        assert_eq!(runtime.chart.initialize_generate_list.len(), 1);
        let (gameplay_id, _ctor) = runtime.chart.initialize_generate_list[0];
        assert_eq!(gameplay_id, original_id + 1);

        // 2. 修改版注入器的 fvalue 已被基类 @PeriodModifier 修正：1.0 + timeOffset(2.5)
        let gameplay = vm.injectors.get(&gameplay_id).unwrap();
        assert!((gameplay.get_injector_float(0) - 3.5).abs() < 0.001,
            "fvalue 应被修改为 3.5，实际 {}", gameplay.get_injector_float(0));
        // 未声明修改器的字段保持原值
        assert_eq!(gameplay.get_injector_int(0), 7);

        // 3. 原注入器不受修改影响（clone 语义）
        let original = vm.injectors.get(&original_id).unwrap();
        assert!((original.get_injector_float(0) - 1.0).abs() < 0.001,
            "原注入器不应被修改");

        // 4. PeriodConfig 已物化为对象供方法体读取（timeOffset 写入 float 字段 0）
        assert!(vm.objects.values().any(|o| {
            o.class_name == "GorgeFramework.PeriodConfig"
                && (o.get_float_field(0) - 2.5).abs() < 0.001
        }), "PeriodConfig 对象应已物化并写入 timeOffset");
    }

    #[test]
    fn test_p0_5_add_score_element_rejects_non_element_class() {
        use gorge_core::diagnostics::Span;
        use gorge_core::objective::class::RuntimeClass;
        use gorge_core::objective::declaration::MethodAnnotation;
        use gorge_core::virtual_machine::ir::{CodeWithSpan, CompiledMethod, IntermediateCode, IntermediateOperator, Operand};
        use std::collections::HashMap;
        use std::sync::Arc;

        // 非元素类（无父类链）：带 @InitializeGenerate 构造注解，不应进入生成表
        let mut decl = gorge_core::objective::declaration::ClassDeclaration::dummy("Dremu.Song".into());
        decl.constructor_count = 1;
        decl.constructor_annotations = HashMap::from([(
            0,
            vec![MethodAnnotation { name: "InitializeGenerate".into(), parameters: vec![] }],
        )]);
        let mut runtime_class = RuntimeClass::new(decl, None);
        runtime_class.register_constructor(0, CompiledMethod {
            name: "ctor".into(),
            codes: vec![CodeWithSpan::new(IntermediateCode {
                result: None,
                operator: IntermediateOperator::ReturnInt,
                left: Operand::int(0),
                right: None,
            }, Span::dummy())],
            local_count: 0,
        });

        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        vm.class_table.insert("Dremu.Song".into(), Arc::new(runtime_class));

        let injector_id = vm.next_object_id;
        vm.next_object_id += 1;
        let injector = RuntimeInjector::new(Arc::new(
            vm.class_table.get("Dremu.Song").unwrap().declaration.clone(),
        ));
        vm.injectors.insert(injector_id, injector);

        let mut runtime = GorgeSimulationRuntime::new();
        let default_config = crate::chart::period::PeriodConfig::default();
        runtime.chart.add_score_element(injector_id, &default_config, &mut vm);

        assert!(runtime.chart.initialize_generate_list.is_empty(),
            "非元素类不应进入初始化创生表");
        assert!(runtime.chart.forward_timed_generate_list.is_empty());
        assert!(runtime.chart.backward_timed_generate_list.is_empty());
    }

    #[test]
    fn test_standard_simulators_match_reference_priorities() {
        let runtime = GorgeSimulationRuntime::new();
        let mut main_priorities: Vec<i32> = runtime.simulation.simulators.iter()
            .map(|(priority, _)| *priority)
            .collect();
        main_priorities.sort_unstable();
        assert_eq!(main_priorities, vec![-1, -1, 0, 10_000]);

        let late_priorities: Vec<i32> = runtime.simulation.late_independent_simulators.iter()
            .map(|(priority, _)| *priority)
            .collect();
        assert_eq!(late_priorities, vec![100_000]);
    }

    #[test]
    fn test_f3_stop_not_simulating_noop() {
        let mut rt = GorgeSimulationRuntime::new();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        let mut machine = test_machine();
        rt.stop_simulation(&mut machine, &mut vm); // 不应 panic
        assert!(!rt.is_simulating);
    }

    // ==================== R-5c AudioStaff→AudioManager 数据流测试 ====================

    #[test]
    fn test_r5c_register_audio_periods_into_audio_manager() {
        crate::runtime::environment::global::init_env_global();
        crate::adaptor::install_platform(Box::new(crate::adaptor::HeadlessPlatform::new()));

        let mut rt = GorgeSimulationRuntime::new();
        let periods = vec![
            AudioPeriodData::new(100, 10, 0.5),
            AudioPeriodData::new(101, 11, 1.5),
        ];

        // 注册前 audio 为空
        assert!(rt.audio.period_audio_sources.is_empty());

        rt.register_audio_periods(&periods);

        // 注册后应有 2 个时段
        assert_eq!(rt.audio.period_audio_sources.len(), 2);
        assert!(rt.audio.period_audio_sources.contains_key(&100));
        assert!(rt.audio.period_audio_sources.contains_key(&101));
    }

    #[test]
    fn test_r5c_remove_audio_period_cleans_up() {
        crate::runtime::environment::global::init_env_global();
        crate::adaptor::install_platform(Box::new(crate::adaptor::HeadlessPlatform::new()));

        let mut rt = GorgeSimulationRuntime::new();
        rt.register_audio_periods(&[AudioPeriodData::new(1, 10, 0.0)]);
        assert_eq!(rt.audio.period_audio_sources.len(), 1);

        rt.audio.remove_period(1);
        assert!(rt.audio.period_audio_sources.is_empty());
    }

    // ==================== P1-3 Manager 生命周期钩子测试 ====================

    #[test]
    fn test_p1_3_automaton_runtime_initialize_and_destruct() {
        let mut rt = GorgeSimulationRuntime::new();
        let score = empty_score();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        let mut machine = test_machine();

        // 播种自动机状态（模拟上一轮仿真残留）
        rt.automaton.add_signal_edge("Touch", 1, 0.0, 100);
        rt.automaton.automatons.push(7);
        rt.automaton.pending_detection_conditions.insert(7, Vec::new());
        rt.automaton.next_signal_id = 9;

        rt.load_score(&score, &mut machine, &mut vm);
        rt.start_simulation(&mut machine, &mut vm);

        // RuntimeInitialize：三表清空，信号编号从 1 开始（对齐 C# `_nextSignalId = 1`）
        assert!(rt.automaton.input_signals.is_empty());
        assert!(rt.automaton.automatons.is_empty());
        assert!(rt.automaton.pending_detection_conditions.is_empty());
        assert_eq!(rt.automaton.next_signal_id, 1);
        assert_eq!(rt.automaton.get_disposable_signal_id(), 1);
        assert_eq!(rt.automaton.get_disposable_signal_id(), 2);

        // 再播种后 stop：RuntimeDestruct 清空且编号归零（对齐 C# `_nextSignalId = default`）
        rt.automaton.add_signal_edge("Touch", 1, 0.0, 100);
        rt.automaton.automatons.push(8);
        rt.stop_simulation(&mut machine, &mut vm);
        assert!(rt.automaton.input_signals.is_empty());
        assert!(rt.automaton.automatons.is_empty());
        assert!(rt.automaton.pending_detection_conditions.is_empty());
        assert_eq!(rt.automaton.next_signal_id, 0);
    }

    #[test]
    fn test_p1_3_graphics_start_stop_clears_nodes() {
        let mut rt = GorgeSimulationRuntime::new();
        let score = empty_score();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        let mut machine = test_machine();

        rt.graphics.nodes.push(42);
        rt.load_score(&score, &mut machine, &mut vm);
        rt.start_simulation(&mut machine, &mut vm);
        assert!(rt.graphics.nodes.is_empty(), "StartSimulation 应重建节点表");

        rt.graphics.nodes.push(43);
        rt.stop_simulation(&mut machine, &mut vm);
        assert!(rt.graphics.nodes.is_empty(), "StopSimulation 应清空节点表");
    }

    #[test]
    fn test_p1_3_simulation_machine_and_simulators_reset() {
        let mut rt = GorgeSimulationRuntime::new();
        let score = empty_score();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        let mut machine = test_machine();

        rt.load_score(&score, &mut machine, &mut vm);
        rt.start_simulation(&mut machine, &mut vm);
        assert!(machine.is_initialized(), "StartSimulation 应复位 SimulationMachine");

        rt.stop_simulation(&mut machine, &mut vm);
        assert!(!machine.is_initialized(), "StopSimulation 应析构 SimulationMachine");
        assert!(rt.simulation.simulators.is_empty());
        assert!(rt.simulation.late_independent_simulators.is_empty());
        assert_eq!(rt.sim_registry.iter().count(), 0, "注册表应随 RuntimeDestruct 清空");

        // 再次启动：标准模拟器按参考优先级重新注册
        rt.start_simulation(&mut machine, &mut vm);
        assert!(machine.is_initialized());
        let mut main_priorities: Vec<i32> = rt.simulation.simulators.iter()
            .map(|(priority, _)| *priority)
            .collect();
        main_priorities.sort_unstable();
        assert_eq!(main_priorities, vec![-1, -1, 0, 10_000]);
        let late_priorities: Vec<i32> = rt.simulation.late_independent_simulators.iter()
            .map(|(priority, _)| *priority)
            .collect();
        assert_eq!(late_priorities, vec![100_000]);
    }

    #[test]
    fn test_p1_3_audio_lifecycle_rebuilds_players() {
        crate::runtime::environment::global::init_env_global();
        crate::adaptor::install_platform(Box::new(crate::adaptor::HeadlessPlatform::new()));

        let mut rt = GorgeSimulationRuntime::new();
        let score = empty_score();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        let mut machine = test_machine();

        // load 期注册乐段（R-5c 数据流）
        rt.register_audio_periods(&[
            AudioPeriodData::new(100, 10, 0.5),
            AudioPeriodData::new(101, 11, 1.5),
        ]);
        assert_eq!(rt.audio.period_audio_sources.len(), 2);

        rt.load_score(&score, &mut machine, &mut vm);
        rt.start_simulation(&mut machine, &mut vm);
        assert_eq!(rt.audio.period_audio_sources.len(), 2, "已创建的播放器不应重复创建");

        // StopSimulation：全部播放器 Destruct，两表清空
        rt.stop_simulation(&mut machine, &mut vm);
        assert!(rt.audio.period_audio_sources.is_empty());
        assert!(rt.audio.respond_effects.is_empty());

        // 再次 StartSimulation：按缓存重建乐段播放器（restart 有音乐）
        rt.start_simulation(&mut machine, &mut vm);
        assert_eq!(rt.audio.period_audio_sources.len(), 2, "应按缓存重建乐段播放器");
        assert!(rt.audio.period_audio_sources.contains_key(&100));
        assert!(rt.audio.period_audio_sources.contains_key(&101));
    }

    #[test]
    fn test_p1_3_audio_start_creates_instant_audio_effects() {
        use gorge_core::objective::native::{NativeClass, NativeContext};
        use crate::system::native::audio_asset::AudioAsset;
        use crate::system::native::environment::Environment;
        use crate::system::native::native_audio_asset::NativeAudioAsset;

        crate::runtime::environment::global::init_env_global();
        crate::adaptor::install_platform(Box::new(crate::adaptor::HeadlessPlatform::new()));

        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        vm.next_object_id = 1;
        // 注册资产桥接链路相关 native 类（仿 audio_asset.rs 测试）
        vm.register_native_class("GorgeFramework.AudioAsset", std::sync::Arc::new(AudioAsset { name: String::new() }));
        vm.register_native_class("GorgeFramework.Environment", std::sync::Arc::new(Environment {}));
        vm.register_native_class("GorgeFramework.NativeAudioAsset", std::sync::Arc::new(NativeAudioAsset { name: String::new(), audio: 0 }));
        // 登记平台音频句柄（键名唯一，避免并行测试冲突）
        crate::runtime::environment::global::with_env_global_mut(|env| {
            env.assets.insert("audio:test_p1_3_effect".to_string(), 888);
        });

        // 构造指向该资产的 AudioAsset 对象（模拟 Score.InstantAudio 的延迟物化条目）
        let audio_asset = AudioAsset { name: String::new() };
        let asset_object_id = {
            let mut ctx = NativeContext::new(&mut vm);
            audio_asset.do_construct_native(&mut ctx, None, 0)
        };
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_string_field(asset_object_id, AudioAsset::FIELD_INDEX_name, "audio:test_p1_3_effect".to_string());
        }

        let mut rt = GorgeSimulationRuntime::new();
        let score = empty_score();
        let mut machine = test_machine();
        rt.audio.cache_instant_audio(vec![("hit".to_string(), asset_object_id)]);

        rt.load_score(&score, &mut machine, &mut vm);
        rt.start_simulation(&mut machine, &mut vm);
        assert!(rt.audio.respond_effects.contains_key("hit"), "StartSimulation 应创建即时音效播放器");

        rt.stop_simulation(&mut machine, &mut vm);
        assert!(rt.audio.respond_effects.is_empty(), "StopSimulation 应销毁音效播放器");

        // 缓存保留，restart 时重建
        rt.start_simulation(&mut machine, &mut vm);
        assert!(rt.audio.respond_effects.contains_key("hit"), "restart 应按缓存重建音效播放器");
    }

    // ==================== R-1 / C-3 RePlay 测试 ====================

    #[test]
    fn test_r1_replay_preserves_state() {
        use crate::runtime::simulation_machine::SimulationMachine;
        use gorge_core::virtual_machine::vm::VirtualMachine;

        let mut rt = GorgeSimulationRuntime::new();
        let score = empty_score();
        let mut vm = VirtualMachine::new();
        let mut machine = SimulationMachine::new(0.0, 100.0, 1.0);
        machine.runtime_initialize();
        rt.load_score(&score, &mut machine, &mut vm);
        rt.start_simulation(&mut machine, &mut vm);
        rt.chart_time = 42.0;

        // RePlay: stop → start → drive_to_chart_time，chart_time 被恢复
        rt.replay(&mut machine, &mut vm);

        assert!(rt.is_simulating);
        // drive_to_chart_time 将 chart_time 恢复到 replay 前的值
        assert!(machine.chart_time >= 42.0);
    }
}
