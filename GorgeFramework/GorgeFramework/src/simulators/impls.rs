//! 模拟器实现（对应 C# `Simulators/` 下的各具体类）。
//!
//! 包含 TimedElementGenerator/Destroyer（定时创生/销毁）、
//! SongSimulator/GraphicsNodeSimulator（骨架），以及
//! GenerateElement/DestroyElement/DeriveElement 等 GameplayAction 实现。
//! S4c 重构：do_action 签名为 `&mut GorgeSimulationRuntime + &mut MultichannelEdgeQueue + &mut VirtualMachine`。
#![allow(dead_code)]

use crate::runtime::environment::GorgeSimulationRuntime;
use crate::runtime::simulation_types::SimulateDirection;
use crate::simulators::{ISimulator, IGameplayAction};
use crate::signal::multichannel_edge_queue::MultichannelEdgeQueue;
use crate::signal::multichannel_snapshot::MultichannelSnapshot;
use gorge_core::objective::object::GorgeObject;
use gorge_core::system::native::injector::Injector;
use gorge_core::virtual_machine::vm::VirtualMachine;

// ==================== TimedElementGenerator ====================

/// 定时创生器：在预设时间点生成元素
pub struct TimedElementGenerator;

impl ISimulator for TimedElementGenerator {
    fn forward_async_simulation_target(&self, chart_time: f32, runtime: &GorgeSimulationRuntime) -> f32 {
        let list = &runtime.chart.forward_timed_generate_list;
        if list.is_empty() { return f32::MAX; }
        list.iter()
            .map(|(t, _, _)| if *t > chart_time { *t } else { f32::MAX })
            .fold(f32::MAX, f32::min)
    }

    fn backward_async_simulation_target(&self, chart_time: f32, runtime: &GorgeSimulationRuntime) -> f32 {
        let list = &runtime.chart.backward_timed_generate_list;
        if list.is_empty() { return f32::MIN; }
        list.iter()
            .map(|(t, _, _)| if *t < chart_time { *t } else { f32::MIN })
            .fold(f32::MIN, f32::max)
    }

    fn infinitesimal_async_simulation_target(&self, _chart_time: f32, _runtime: &GorgeSimulationRuntime) -> f32 {
        f32::MAX
    }

    fn forward_simulate(
        &self, chart_from: f32, chart_to: f32,
        _snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime,
        _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        let mut actions: Vec<Box<dyn IGameplayAction>> = Vec::new();
        for (time, injector_id, constructor_id) in &runtime.chart.forward_timed_generate_list {
            if *time > chart_from && *time <= chart_to {
                actions.push(Box::new(GenerateElement {
                    injector_id: *injector_id,
                    constructor_id: *constructor_id,
                    is_auto_play: false,
                    is_reverse: false,
                    direction: SimulateDirection::Forward,
                }));
            }
        }
        actions
    }

    fn backward_simulate(
        &self, chart_from: f32, chart_to: f32,
        _snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime,
        _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        let mut actions: Vec<Box<dyn IGameplayAction>> = Vec::new();
        for (time, injector_id, constructor_id) in &runtime.chart.backward_timed_generate_list {
            if *time < chart_from && *time >= chart_to {
                actions.push(Box::new(GenerateElement {
                    injector_id: *injector_id,
                    constructor_id: *constructor_id,
                    is_auto_play: false,
                    is_reverse: true,
                    direction: SimulateDirection::Backward,
                }));
            }
        }
        actions
    }

    fn infinitesimal_simulate(
        &self, _chart_time: f32, _snapshot: &MultichannelSnapshot, _runtime: &GorgeSimulationRuntime,
        _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        Vec::new()
    }

    fn instant_simulate(
        &self, _chart_time: f32, _direction: SimulateDirection,
        _snapshot: &MultichannelSnapshot, _runtime: &GorgeSimulationRuntime,
        _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        Vec::new()
    }
}

// ==================== TimedElementDestroyer ====================

/// 定时销毁器：在预设时间点销毁元素
pub struct TimedElementDestroyer;

impl ISimulator for TimedElementDestroyer {
    fn forward_async_simulation_target(&self, chart_time: f32, runtime: &GorgeSimulationRuntime) -> f32 {
        let list = &runtime.chart.forward_timed_destroy_list;
        if list.is_empty() { return f32::MAX; }
        list.iter()
            .map(|(t, _)| if *t > chart_time { *t } else { f32::MAX })
            .fold(f32::MAX, f32::min)
    }

    fn backward_async_simulation_target(&self, chart_time: f32, runtime: &GorgeSimulationRuntime) -> f32 {
        let list = &runtime.chart.backward_timed_destroy_list;
        if list.is_empty() { return f32::MIN; }
        list.iter()
            .map(|(t, _)| if *t < chart_time { *t } else { f32::MIN })
            .fold(f32::MIN, f32::max)
    }

    fn infinitesimal_async_simulation_target(&self, _chart_time: f32, _runtime: &GorgeSimulationRuntime) -> f32 {
        f32::MAX
    }

    fn forward_simulate(
        &self, chart_from: f32, chart_to: f32,
        _snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime,
        _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        let mut actions: Vec<Box<dyn IGameplayAction>> = Vec::new();
        for (time, element_id) in &runtime.chart.forward_timed_destroy_list {
            if *time > chart_from && *time <= chart_to {
                actions.push(Box::new(DestroyElement { element_id: *element_id }));
            }
        }
        actions
    }

    fn backward_simulate(
        &self, chart_from: f32, chart_to: f32,
        _snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime,
        _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        let mut actions: Vec<Box<dyn IGameplayAction>> = Vec::new();
        for (time, element_id) in &runtime.chart.backward_timed_destroy_list {
            if *time < chart_from && *time >= chart_to {
                actions.push(Box::new(DestroyElement { element_id: *element_id }));
            }
        }
        actions
    }

    fn infinitesimal_simulate(
        &self, _chart_time: f32, _snapshot: &MultichannelSnapshot, _runtime: &GorgeSimulationRuntime,
        _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        Vec::new()
    }

    fn instant_simulate(
        &self, _chart_time: f32, _direction: SimulateDirection,
        _snapshot: &MultichannelSnapshot, _runtime: &GorgeSimulationRuntime,
        _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        Vec::new()
    }
}

// ==================== SongSimulator ====================

/// 音乐播放模拟器：控制音频播放状态与谱面时间同步
///
/// 对齐 C# `SongSimulator`。遍历 AudioManager.period_audio_sources，
/// 按 chart_time 检查音频时段范围，控制播放/停止。
#[derive(Debug)]
pub struct SongSimulator;

impl ISimulator for SongSimulator {
    fn forward_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime) -> f32 { f32::MAX }
    fn backward_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime) -> f32 { f32::MIN }
    fn infinitesimal_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime) -> f32 { f32::MAX }

    fn forward_simulate(
        &self, _chart_from: f32, chart_to: f32,
        _snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime,
        _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        // 遍历已知存活的音频时段（从 period_audio_sources 的键中获取 period_id）
        // 简化实现：由于当前 period_audio_sources 存储 period_id → handle，
        // 检查 chart_to 是否落在各时段时间窗口内
        for (&_period_id, &_handle) in &runtime.audio.period_audio_sources {
            // C# 语义：
            // startChartTime = period.Config.timeOffset + StaticConfig.RespondDelay
            // endChartTime = startChartTime + audioPlayer.AudioLength()
            // if chartTo in [start, end) && !playing → setTime + play
            // else if playing → stop
            let _ = chart_to; // 需要 period 对象来获取 timeOffset 和 audioLength
        }
        Vec::new()
    }

    fn backward_simulate(
        &self, _f: f32, _t: f32, _s: &MultichannelSnapshot,
        _r: &GorgeSimulationRuntime, _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        Vec::new()
    }

    fn infinitesimal_simulate(
        &self, _t: f32, _s: &MultichannelSnapshot,
        _r: &GorgeSimulationRuntime, _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        Vec::new()
    }

    fn instant_simulate(
        &self, _t: f32, _d: SimulateDirection, _s: &MultichannelSnapshot,
        _r: &GorgeSimulationRuntime, _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        Vec::new()
    }
}

// ==================== GraphicsNodeSimulator ====================

/// 图形节点模拟器：驱动所有图形节点更新
///
/// 对齐 C# `GraphicsNodeSimulator`。遍历 GraphicsManager.nodes，
/// 对每个节点调用 UpdateNode 方法（经 NativeContext 分派）。
pub struct GraphicsNodeSimulator;

impl ISimulator for GraphicsNodeSimulator {
    fn forward_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime) -> f32 { f32::MAX }
    fn backward_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime) -> f32 { f32::MIN }
    fn infinitesimal_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime) -> f32 { f32::MAX }

    fn forward_simulate(
        &self, _chart_from: f32, _chart_to: f32,
        _snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        simulate_all_nodes(runtime, vm);
        Vec::new()
    }

    fn backward_simulate(
        &self, _f: f32, _t: f32, _s: &MultichannelSnapshot,
        runtime: &GorgeSimulationRuntime, vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        simulate_all_nodes(runtime, vm);
        Vec::new()
    }

    fn infinitesimal_simulate(
        &self, _t: f32, _s: &MultichannelSnapshot,
        runtime: &GorgeSimulationRuntime, vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        simulate_all_nodes(runtime, vm);
        Vec::new()
    }

    fn instant_simulate(
        &self, _t: f32, _d: SimulateDirection, _s: &MultichannelSnapshot,
        runtime: &GorgeSimulationRuntime, vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        simulate_all_nodes(runtime, vm);
        Vec::new()
    }
}

/// 返回具体图形节点 native 类的更新方法编号。
///
/// `Node.UpdateNode` 在 Node native ABI 中是 5 号方法；三个派生渲染节点
/// 各自在本类 native ABI 中以 0 号方法覆盖更新逻辑。
fn graphics_node_update_method(class_name: &str) -> Option<usize> {
    match class_name.rsplit('.').next().unwrap_or(class_name) {
        "Node" => Some(5),
        "Sprite" | "NineSliceSprite" | "CurveSprite" => Some(0),
        _ => None,
    }
}

/// 遍历所有图形节点并按实际 native 类型调用 UpdateNode。
fn simulate_all_nodes(runtime: &GorgeSimulationRuntime, vm: &mut VirtualMachine) {
    let node_ids: Vec<usize> = runtime.graphics.nodes.clone();
    for node_id in node_ids {
        if node_id == 0 { continue; }
        let class_name = vm.objects
            .get(&node_id)
            .map(|node| node.class_name.clone())
            .unwrap_or_default();
        let Some(update_method) = graphics_node_update_method(&class_name) else {
            continue;
        };
        let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
        if update_method == 0 {
            // 派生更新前先执行 Node 的存在性引用检查。
            ctx.invoke_native_method_on("GorgeFramework.Node", node_id, 5);
        }
        ctx.invoke_native_method_on(&class_name, node_id, update_method);
    }
}

// ==================== Gameplay Actions ====================

/// 追加信号边沿动作（对齐 C# `AppendSignal`）
pub struct AppendSignal {
    channel: String,
    signal_id: i32,
    delay_simulate_time: f32,
    value: usize,
}

impl AppendSignal {
    pub fn new(channel: String, signal_id: i32, value: usize) -> Self {
        Self { channel, signal_id, delay_simulate_time: 0.0, value }
    }

    /// 创建带延迟的追加信号
    pub fn with_delay(channel: String, signal_id: i32, value: usize, delay: f32) -> Self {
        Self { channel, signal_id, delay_simulate_time: delay, value }
    }
}

impl IGameplayAction for AppendSignal {
    fn do_action(
        &self,
        runtime: &mut GorgeSimulationRuntime,
        edge_queue: &mut MultichannelEdgeQueue,
        _vm: &mut VirtualMachine,
    ) {
        let sim_time = runtime.simulate_time;
        let add_success = runtime.automaton.add_signal_edge(
            &self.channel, self.signal_id, sim_time + self.delay_simulate_time, self.value,
        );
        if add_success && self.delay_simulate_time == 0.0 {
            edge_queue.enqueue(&self.channel, self.signal_id, crate::input::edge::Edge::new(sim_time, self.value));
        }
    }

    fn change_signal(&self) -> bool { true }
}

/// 谱面终结动作（对齐 C# `Terminate`）
pub struct Terminate;

impl IGameplayAction for Terminate {
    fn do_action(
        &self, _runtime: &mut GorgeSimulationRuntime,
        _edge_queue: &mut MultichannelEdgeQueue, _vm: &mut VirtualMachine,
    ) {
        // TODO: 调用 runtime.on_terminate 回调
    }
}

// ==================== 元素生命周期动作 ====================

/// 创生元素动作（对齐 C# `GenerateElement`，S4-1/3/6）
///
/// `Element.g` 的 object 字段布局是运行时 ABI：simulator、
/// lateIndependentSimulator、nodes、derivedElements。`Note.automaton`
/// 紧随其后。不要在调用点写入裸编号，以免 native stub 变更后静默错读字段。
const ELEMENT_SIMULATOR_FIELD: usize = 0;
const ELEMENT_LATE_INDEPENDENT_SIMULATOR_FIELD: usize = 1;
const ELEMENT_NODES_FIELD: usize = 2;
const ELEMENT_DERIVED_ELEMENTS_FIELD: usize = 3;
const NOTE_AUTOMATON_FIELD: usize = 4;

pub struct GenerateElement {
    /// 元素注入器 ID
    pub injector_id: usize,
    /// 构造方法全局 ID
    pub constructor_id: usize,
    /// 是否自动播放
    pub is_auto_play: bool,
    /// 是否反转
    pub is_reverse: bool,
    /// 模拟方向
    pub direction: SimulateDirection,
}

impl GenerateElement {
    pub fn new(injector_id: usize, constructor_id: usize) -> Self {
        Self { injector_id, constructor_id, is_auto_play: false, is_reverse: false, direction: SimulateDirection::Forward }
    }
}

impl IGameplayAction for GenerateElement {
    fn do_action(
        &self,
        runtime: &mut GorgeSimulationRuntime,
        _edge_queue: &mut MultichannelEdgeQueue,
        vm: &mut VirtualMachine,
    ) {
        let class_name = vm.injectors.get(&self.injector_id)
            .map(|inj| inj.injection_class_declaration().class_type.full_name())
            .unwrap_or_default();
        if class_name.is_empty() { return; }

        // 1. instantiate_with_injector 创建元素
        let element_id = vm.instantiate_with_injector(&class_name, self.constructor_id, self.injector_id)
            .unwrap_or(0);
        if element_id == 0 { return; }

        // 2. 登记到存活元素表
        runtime.chart.alive_elements.push(element_id);
        runtime.chart.alive_injector_map.insert(element_id, self.injector_id);

        // 3. 读取 element.simulator / element.late_independent_simulator 字段
        // Element 类字段顺序：simulator(对象0), late_independent_simulator(对象1)
        let simulator_id = vm.objects.get(&element_id)
            .map(|o| o.get_object_field(ELEMENT_SIMULATOR_FIELD))
            .unwrap_or(0);
        if simulator_id != 0 {
            // S7: 通过 ElementSimulatorAdapter 包装原生物件，注册进 SimRegistry
            let adapter = Box::new(ElementSimulatorAdapter::new(simulator_id));
            let reg_key = runtime.sim_registry.register(adapter);
            runtime.simulation.simulators.register(10, reg_key);
        }
        let late_sim_id = vm.objects.get(&element_id)
            .map(|o| o.get_object_field(ELEMENT_LATE_INDEPENDENT_SIMULATOR_FIELD))
            .unwrap_or(0);
        if late_sim_id != 0 {
            let adapter = Box::new(ElementSimulatorAdapter::new(late_sim_id));
            let reg_key = runtime.late_sim_registry.register(adapter);
            runtime.simulation.late_independent_simulators.register(10, reg_key);
        }

        // 4. Note 判定与自动机注册（S4-3）：沿继承链判定
        let is_note = is_subclass_of_note(&class_name, vm);
        if is_note {
            runtime.chart.alive_notes.push(element_id);
            // Note 的 automaton 字段位于 Element 全部 object 字段之后。
            let automaton_id = vm.objects.get(&element_id)
                .map(|o| o.get_object_field(NOTE_AUTOMATON_FIELD))
                .unwrap_or(0);
            if automaton_id != 0 {
                runtime.automaton.automatons.push(automaton_id);
                // S7: 挂接待决检测条件
                fill_pending_detection_conditions(automaton_id, runtime, vm);
            }
        }

        // 5. 注解扫描 ForwardTimedDestroy / BackwardTimedDestroy（S4-1）
        let fwd_destroy: Vec<(usize, gorge_core::objective::declaration::MethodAnnotation)> =
            vm.class_table.get(&class_name)
                .map(|cls| cls.declaration.methods_with_annotation("ForwardTimedDestroy")
                    .into_iter().map(|(id, ann)| (id, ann.clone())).collect())
                .unwrap_or_default();
        for (_method_id, ann) in fwd_destroy {
            let time = resolve_annotation_time_from_method(&ann, &class_name, Some(element_id), vm);
            runtime.chart.forward_timed_destroy_list.push((time, element_id));
        }

        let bwd_destroy: Vec<(usize, gorge_core::objective::declaration::MethodAnnotation)> =
            vm.class_table.get(&class_name)
                .map(|cls| cls.declaration.methods_with_annotation("BackwardTimedDestroy")
                    .into_iter().map(|(id, ann)| (id, ann.clone())).collect())
                .unwrap_or_default();
        for (_method_id, ann) in bwd_destroy {
            let time = resolve_annotation_time_from_method(&ann, &class_name, Some(element_id), vm);
            runtime.chart.backward_timed_destroy_list.push((time, element_id));
        }

        // 6. Nodes 登记（S4-6）
        // Element 的 nodes 字段（ObjectArray）。
        let nodes_array_id = vm.objects.get(&element_id)
            .map(|o| o.get_object_field(ELEMENT_NODES_FIELD))
            .unwrap_or(0);
        if nodes_array_id != 0 {
            // 经 ObjectArray payload 读取各节点对象 ID
            let node_ids = vm.native_payloads
                .get(&nodes_array_id)
                .and_then(|p| p.downcast_ref::<gorge_core::system::native::array::ObjectArray>())
                .map(|a| a.items.clone())
                .unwrap_or_default();
            for node_id in node_ids {
                runtime.graphics.nodes.push(node_id);
            }
        }

        // 7. derivedElements 处理（S4-2）：读取 derived_elements 字段并逐个派生
        // Element 的 derived_elements 字段（ObjectArray）。
        let derived_array_id = vm.objects.get(&element_id)
            .map(|o| o.get_object_field(ELEMENT_DERIVED_ELEMENTS_FIELD))
            .unwrap_or(0);
        if derived_array_id != 0 {
            let derived_ids = vm.native_payloads
                .get(&derived_array_id)
                .and_then(|p| p.downcast_ref::<gorge_core::system::native::array::ObjectArray>())
                .map(|a| a.items.clone())
                .unwrap_or_default();
            for derived_id in derived_ids {
                let derived_dir = if self.is_reverse { SimulateDirection::Backward } else { SimulateDirection::Forward };
                do_derive_element(runtime, vm, derived_id, derived_dir);
            }
        }
    }

    fn change_automaton(&self) -> bool { true }
}

/// 销毁元素动作（对齐 C# `DestroyElement`，S4-1/3/6）
pub struct DestroyElement {
    pub element_id: usize,
}

impl DestroyElement {
    pub fn new(element_id: usize) -> Self { Self { element_id } }
}

impl IGameplayAction for DestroyElement {
    fn do_action(
        &self,
        runtime: &mut GorgeSimulationRuntime,
        _edge_queue: &mut MultichannelEdgeQueue,
        _vm: &mut VirtualMachine,
    ) {
        // 1. 注销图形节点（S4-6）
        // 移除与该元素关联的全部节点，需维护 element → nodes 映射
        // 当前简化：遍历 GraphicsManager.nodes 移除（后续可优化为索引表）
        runtime.graphics.nodes.retain(|&_nid| {
            // TODO: 检查该节点是否属于此 element（需 node→element 映射）
            true // 暂不移除
        });

        // 2. 注销模拟器
        runtime.simulation.simulators.remove(&self.element_id);
        runtime.simulation.late_independent_simulators.remove(&self.element_id);

        // 3. 从存活表中移除
        runtime.chart.alive_elements.retain(|&id| id != self.element_id);
        runtime.chart.alive_notes.retain(|&id| id != self.element_id);
        runtime.chart.alive_injector_map.remove(&self.element_id);

        // 4. 注销自动机（若为 Note）
        runtime.automaton.automatons.retain(|&id| id != self.element_id);

        // 5. 注销定时销毁条目
        runtime.chart.forward_timed_destroy_list.retain(|(_, id)| *id != self.element_id);
        runtime.chart.backward_timed_destroy_list.retain(|(_, id)| *id != self.element_id);
    }

    fn change_automaton(&self) -> bool { true }
}

/// 派生元素动作（对齐 C# `DeriveElement`，S4-2）
pub struct DeriveElement {
    pub element_id: usize,
    pub direction: SimulateDirection,
}

impl DeriveElement {
    pub fn new(element_id: usize, direction: SimulateDirection) -> Self {
        Self { element_id, direction }
    }
}

impl IGameplayAction for DeriveElement {
    fn do_action(
        &self,
        runtime: &mut GorgeSimulationRuntime,
        _edge_queue: &mut MultichannelEdgeQueue,
        vm: &mut VirtualMachine,
    ) {
        do_derive_element(runtime, vm, self.element_id, self.direction);
    }

    fn change_automaton(&self) -> bool { true }
}

// ==================== 辅助函数 ====================

/// 沿继承链判定元素类是否为 Note 子类（S4-3）
fn is_subclass_of_note(class_name: &str, vm: &VirtualMachine) -> bool {
    let mut current = class_name.to_string();
    loop {
        if current == "GorgeFramework.Note" || current.contains("Note") {
            return true;
        }
        if let Some(cls) = vm.class_table.get(&current) {
            if let Some(ref super_cls) = cls.super_class {
                current = super_cls.declaration.class_type.full_name();
                continue;
            }
        }
        break;
    }
    false
}

/// 为自动机填充待决检测条件（S7）
///
/// 对齐 C# `GetDetectionConditions`。从 SignalTsiga 获取当前 filter，
/// 读取 filter.priority 委托返回的 ObjectArray，展开 Priority 对象并构建条件。
fn fill_pending_detection_conditions(
    tsiga_id: usize,
    runtime: &mut GorgeSimulationRuntime,
    vm: &mut VirtualMachine,
) {
    use crate::runtime::simulation_types::SimulateDirection;
    let mut ctx = gorge_core::objective::native::NativeContext::new(vm);

    // 调用 SignalTsiga.get_detection_conditions（方法 6，返回 filter_id 或 0）
    ctx.set_int_param(0, 0); // direction = Forward
    ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", tsiga_id, 6);
    let filter_id = ctx.get_int_return() as usize;
    if filter_id == 0 { return; }

    // 读取 filter.condition_types（IntArray）
    let condition_types_id = ctx.get_object_object_field(filter_id, 1); // SignalFilter field 1 = conditionTypes
    let condition_type_values = ctx.int_array_items(condition_types_id);

    // 读取 filter.priority 委托
    let priority_delegate_id = ctx.get_object_object_field(filter_id, 0); // SignalFilter field 0 = priority
    let mut priority_items: Vec<usize> = Vec::new();
    if priority_delegate_id != 0 {
        ctx.invoke_delegate(priority_delegate_id);
        let arr_id = ctx.get_object_return();
        if arr_id != 0 {
            priority_items = ctx.object_array_items(arr_id);
        }
    }

    // 若 conditionTypes 为空，添加一条空条件（仅记录 last_value）
    let count = if condition_type_values.is_empty() { 1 } else { condition_type_values.len() };
    let mut conditions: Vec<crate::simulators::SignalDetectionCondition> = Vec::new();
    for _ in 0..count {
        conditions.push(crate::simulators::SignalDetectionCondition {
            tsiga_id,
            filter_id,
            priority_items: priority_items.clone(),
            direction: SimulateDirection::Forward,
        });
    }

    if !conditions.is_empty() {
        runtime.automaton.pending_detection_conditions.insert(tsiga_id, conditions);
    }
}

/// 从方法注解中解析时间值（S4-1）——Float 直接取 / Delegate 经 invoke_method_by_id 求值
fn resolve_annotation_time_from_method(
    ann: &gorge_core::objective::declaration::MethodAnnotation,
    class_name: &str,
    obj_id: Option<usize>,
    vm: &mut VirtualMachine,
) -> f32 {
    use gorge_core::objective::declaration::AnnotationValue;
    if let Some(time_val) = ann.find_parameter("time") {
        match time_val {
            AnnotationValue::Float(f) => return *f as f32,
            AnnotationValue::Delegate(method_id) => {
                if vm.invoke_method_by_id(class_name, None, *method_id).is_ok() {
                    return vm.return_float.unwrap_or(0.0) as f32;
                }
            }
            AnnotationValue::Int(i) => return *i as f32,
            _ => {}
        }
    }
    let _ = obj_id;
    0.0
}

/// 执行派生元素的 @DeriveGenerate 方法 + 登记流程（S4-2）
fn do_derive_element(
    runtime: &mut GorgeSimulationRuntime,
    vm: &mut VirtualMachine,
    element_id: usize,
    _direction: SimulateDirection,
) {
    let class_name = vm.objects.get(&element_id)
        .map(|o| o.class_name.clone())
        .unwrap_or_default();
    if class_name.is_empty() { return; }

    // 扫描 @DeriveGenerate 注解的方法并调用
    let derive_methods: Vec<usize> = vm.class_table.get(&class_name)
        .map(|cls| cls.declaration.methods_with_annotation("DeriveGenerate")
            .into_iter().map(|(id, _)| id).collect())
        .unwrap_or_default();
    for method_id in derive_methods {
        let _ = vm.invoke_method_by_id(&class_name, Some(element_id), method_id);
    }

    // 登记到存活元素表
    runtime.chart.alive_elements.push(element_id);
    // 注册模拟器
    let simulator_id = vm.objects.get(&element_id)
        .map(|o| o.get_object_field(ELEMENT_SIMULATOR_FIELD))
        .unwrap_or(0);
    if simulator_id != 0 {
        let adapter = Box::new(ElementSimulatorAdapter::new(simulator_id));
        let registry_id = runtime.sim_registry.register(adapter);
        runtime.simulation.simulators.register(10, registry_id);
    }
    let late_sim_id = vm.objects.get(&element_id)
        .map(|o| o.get_object_field(ELEMENT_LATE_INDEPENDENT_SIMULATOR_FIELD))
        .unwrap_or(0);
    if late_sim_id != 0 {
        let adapter = Box::new(ElementSimulatorAdapter::new(late_sim_id));
        let registry_id = runtime.late_sim_registry.register(adapter);
        runtime.simulation.late_independent_simulators.register(10, registry_id);
    }
    // Note 判定 + 自动机注册
    if is_subclass_of_note(&class_name, vm) {
        runtime.chart.alive_notes.push(element_id);
        let automaton_id = vm.objects.get(&element_id)
            .map(|o| o.get_object_field(NOTE_AUTOMATON_FIELD))
            .unwrap_or(0);
        if automaton_id != 0 {
            runtime.automaton.automatons.push(automaton_id);
            fill_pending_detection_conditions(automaton_id, runtime, vm);
        }
    }

    // 派生元素与普通创生元素使用相同的图形节点登记规则。
    let nodes_array_id = vm.objects.get(&element_id)
        .map(|o| o.get_object_field(ELEMENT_NODES_FIELD))
        .unwrap_or(0);
    if nodes_array_id != 0 {
        let node_ids = vm.native_payloads
            .get(&nodes_array_id)
            .and_then(|payload| payload.downcast_ref::<gorge_core::system::native::array::ObjectArray>())
            .map(|nodes| nodes.items.clone())
            .unwrap_or_default();
        runtime.graphics.nodes.extend(node_ids.into_iter().filter(|node_id| *node_id != 0));
    }
}

/// 更新自动机待决检测条件（对齐 C# `UpdatePendingDetectionCondition`）
pub struct UpdatePendingDetectionCondition {
    pub automaton_id: usize,
    pub direction: SimulateDirection,
}

impl UpdatePendingDetectionCondition {
    pub fn new(automaton_id: usize, direction: SimulateDirection) -> Self {
        Self { automaton_id, direction }
    }
}

impl IGameplayAction for UpdatePendingDetectionCondition {
    fn do_action(
        &self, _runtime: &mut GorgeSimulationRuntime,
        _edge_queue: &mut MultichannelEdgeQueue, _vm: &mut VirtualMachine,
    ) {
        // TODO S7: 重新计算检测条件——需接入以下依赖：
        //   - SignalTsiga 的 get_detection_conditions 方法（返回 Vec<SignalDetectionCondition>）
        //   - automaton.pending_detection_conditions 表的更新逻辑
        //   - is_keep_condition 判定是否保留已有条件
    }

    fn change_automaton(&self) -> bool { true }
}

// ==================== PreciseAutomatonSimulator（S7） ====================

/// 精准自动机竞争器（对齐 C# `PreciseAutomatonSimulator`）
///
/// 负责在每次模拟步中检测信号、触发自动机状态转移、产生 GameplayAction。
/// 在前向/反向推进中遍历 automatons 调用状态转移，
/// 在瞬时仿真中基于 pending_detection_conditions 竞争检测。
pub struct PreciseAutomatonSimulator;

impl ISimulator for PreciseAutomatonSimulator {
    fn forward_async_simulation_target(&self, _chart_time: f32, runtime: &GorgeSimulationRuntime) -> f32 {
        if runtime.automaton.automatons.is_empty() { return f32::MAX; }
        // TODO S7: 遍历自动机时间转移列表取最早时间，需接入以下数据来源：
        //   - SignalTsiga（native 类）的状态转移时间表（经 call_native_method_float 查询）
        //   - 需要 &mut VirtualMachine 以调用 NativeContext API
        //   - 当前 ISimulator 签名仅接收 &GorgeSimulationRuntime，无法调用 NativeContext
        // 在 ISimulator trait 签名改为接收 &mut VM 前，维持返回 f32::MAX
        f32::MAX
    }

    fn backward_async_simulation_target(&self, _chart_time: f32, runtime: &GorgeSimulationRuntime) -> f32 {
        if runtime.automaton.automatons.is_empty() { return f32::MIN; }
        // TODO S7: 同 forward_async_simulation_target，需查自动机最晚时间转移
        //   依赖：SignalTsiga 反向状态转移表 + VM 引用
        f32::MIN
    }

    fn infinitesimal_async_simulation_target(&self, _chart_time: f32, _runtime: &GorgeSimulationRuntime) -> f32 {
        // TODO S7: 瞬时仿真目标——自动机零时间转移的竞争检测
        //   依赖：pending_detection_conditions 表 + ISimulator signature 扩展接收 &mut VM
        f32::MAX
    }

    fn forward_simulate(
        &self, _chart_from: f32, chart_to: f32,
        _snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        // TODO S7: 正向推进——遍历各有信号自动机，调用 SignalTsiga 方法 1（forward_simulate）
        //   对每个 automaton 对象调用 ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", id, 1)
        //   收集产生的 GameplayAction（AppendSignal/DeriveElement/DestroyElement）
        let automaton_ids: Vec<usize> = runtime.automaton.automatons.clone();
        for tsiga_id in automaton_ids {
            let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
            ctx.set_float_param(0, chart_to as f64);
            ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", tsiga_id, 1);
        }
        Vec::new()
    }

    fn backward_simulate(
        &self, _chart_from: f32, chart_to: f32,
        _snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        // TODO S7: 反向推进——调用 SignalTsiga 方法 3（backward_simulate）
        let automaton_ids: Vec<usize> = runtime.automaton.automatons.clone();
        for tsiga_id in automaton_ids {
            let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
            ctx.set_float_param(0, chart_to as f64);
            ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", tsiga_id, 3);
        }
        Vec::new()
    }

    fn infinitesimal_simulate(
        &self, _chart_time: f32, _snapshot: &MultichannelSnapshot,
        _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        // TODO S7: 瞬时仿真——基于 pending_detection_conditions 竞争检测
        //   需在 instant_simulate 中结合信号快照匹配检测条件并触发转移
        Vec::new()
    }

    fn instant_simulate(
        &self, chart_time: f32, direction: SimulateDirection,
        signal_snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        if direction == SimulateDirection::Backward {
            return Vec::new();
        }

        let detection_conditions = runtime.automaton.pending_detection_conditions.clone();
        let mut remaining_conditions: std::collections::HashMap<usize, Vec<crate::simulators::SignalDetectionCondition>> =
            detection_conditions.clone();

        // 遍历信号快照
        for (channel_name, channel_snapshot) in signal_snapshot.channels.iter() {
            for (signal_id, value_obj_id) in channel_snapshot.iter() {
                if *value_obj_id == 0 { continue; }

                // 收集能检测该信道的条件（can_detect 借用 vm）
                let mut all_conditions: Vec<crate::simulators::SignalDetectionCondition> = Vec::new();
                for (_, conditions) in &detection_conditions {
                    for cond in conditions {
                        let can_detect = {
                            let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
                            ctx.set_string_param(0, channel_name.clone());
                            ctx.invoke_native_method_on("GorgeFramework.SignalFilter", cond.filter_id, 0);
                            ctx.get_bool_return()
                        };
                        if can_detect {
                            all_conditions.push(cond.clone());
                        }
                    }
                }

                // 按优先级排序
                all_conditions.sort_by(|a, b| {
                    let max_len = a.priority_items.len().max(b.priority_items.len());
                    for i in 0..max_len {
                        let a_pri = if let Some(&pid) = a.priority_items.get(i) {
                            if pid == 0 { 0.0 }
                            else {
                                let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
                                ctx.invoke_native_method_on("GorgeFramework.Priority", pid, 0);
                                (ctx.get_float_return() as f64) as f32
                            }
                        } else { 0.0 };
                        let b_pri = if let Some(&pid) = b.priority_items.get(i) {
                            if pid == 0 { 0.0 }
                            else {
                                let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
                                ctx.invoke_native_method_on("GorgeFramework.Priority", pid, 0);
                                (ctx.get_float_return() as f64) as f32
                            }
                        } else { 0.0 };
                        if a_pri < b_pri { return std::cmp::Ordering::Less; }
                        if a_pri > b_pri { return std::cmp::Ordering::Greater; }
                    }
                    std::cmp::Ordering::Equal
                });

                let mut consume_flag = false;
                let mut accepted_tsigas: Vec<usize> = Vec::new();

                for cond in &all_conditions {
                    if accepted_tsigas.contains(&cond.tsiga_id) { continue; }

                    // 更新信号记录（update_signal_record）
                    {
                        let mut update_ctx = gorge_core::objective::native::NativeContext::new(vm);
                        update_ctx.set_string_param(0, channel_name.clone());
                        update_ctx.set_int_param(0, *signal_id as i64);
                        update_ctx.set_object_param(0, if consume_flag { 0 } else { *value_obj_id });
                        update_ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", cond.tsiga_id, 10);
                    }

                    // 获取 last_value
                    let last_value = {
                        let mut last_ctx = gorge_core::objective::native::NativeContext::new(vm);
                        last_ctx.set_string_param(0, channel_name.clone());
                        last_ctx.set_int_param(0, *signal_id as i64);
                        last_ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", cond.tsiga_id, 9);
                        last_ctx.get_int_return() as usize
                    };

                    // 检测
                    let detect_ok = {
                        let mut detect_ctx = gorge_core::objective::native::NativeContext::new(vm);
                        let class_name = detect_ctx.vm.objects.get(&cond.filter_id)
                            .map(|o| o.class_name.clone()).unwrap_or_default();
                        if class_name.contains("InputSignalFilter") {
                            crate::system::native::input_signal_filter::InputSignalFilter::detect_touch(
                                &mut detect_ctx, cond.filter_id, *signal_id, 0,
                                if consume_flag { 0 } else { *value_obj_id }, last_value,
                            )
                        } else {
                            false
                        }
                    };

                    if detect_ok {
                        // 检测接受
                        {
                            let mut accept_ctx = gorge_core::objective::native::NativeContext::new(vm);
                            accept_ctx.set_float_param(0, chart_time as f64);
                            let dir = match direction {
                                SimulateDirection::Forward => 0,
                                SimulateDirection::Backward => 1,
                                SimulateDirection::Infinitesimal => 2,
                            };
                            accept_ctx.set_int_param(0, dir);
                            accept_ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", cond.tsiga_id, 4);
                        }

                        // 检查 accept_consume
                        let accept_consume = {
                            let ac = gorge_core::objective::native::NativeContext::new(vm);
                            ac.get_object_bool_field(cond.filter_id, 3)
                        };
                        if accept_consume { consume_flag = true; }

                        accepted_tsigas.push(cond.tsiga_id);
                        remaining_conditions.remove(&cond.tsiga_id);
                    }
                }
            }
        }

        // 对所有仍然待决的自动机响应拒绝
        for (tsiga_id, conditions) in remaining_conditions.iter() {
            if !conditions.is_empty() {
                let mut deny_ctx = gorge_core::objective::native::NativeContext::new(vm);
                deny_ctx.set_float_param(0, chart_time as f64);
                deny_ctx.set_int_param(0, 2);
                deny_ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", *tsiga_id, 5);
            }
        }

        Vec::new()
    }
}

// ==================== ElementSimulatorAdapter（S7） ====================

/// ElementSimulator 的 ISimulator 适配器
///
/// 每个 ElementSimulator 原生对象通过此适配器注册进 SimRegistry，
/// 在模拟时调用 transformers payload 中的各 ITransformer.Transform 方法。
pub struct ElementSimulatorAdapter {
    pub object_id: usize,
}

impl ElementSimulatorAdapter {
    pub fn new(object_id: usize) -> Self { Self { object_id } }
}

impl ISimulator for ElementSimulatorAdapter {
    fn forward_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime) -> f32 { f32::MAX }
    fn backward_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime) -> f32 { f32::MIN }
    fn infinitesimal_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime) -> f32 { f32::MAX }

    fn forward_simulate(
        &self, _chart_from: f32, chart_to: f32,
        _snapshot: &MultichannelSnapshot, _runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        element_simulate(self.object_id, chart_to, SimulateDirection::Forward, vm)
    }

    fn backward_simulate(
        &self, _chart_from: f32, chart_to: f32,
        _snapshot: &MultichannelSnapshot, _runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        element_simulate(self.object_id, chart_to, SimulateDirection::Backward, vm)
    }

    fn infinitesimal_simulate(
        &self, chart_time: f32, _snapshot: &MultichannelSnapshot,
        _runtime: &GorgeSimulationRuntime, vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        element_simulate(self.object_id, chart_time, SimulateDirection::Infinitesimal, vm)
    }

    fn instant_simulate(
        &self, chart_time: f32, direction: SimulateDirection,
        _snapshot: &MultichannelSnapshot, _runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        element_simulate(self.object_id, chart_time, direction, vm)
    }
}

/// 执行 ElementSimulator 的模拟：遍历 transformers 并调用 ITransformer.Transform
fn element_simulate(
    object_id: usize,
    chart_time: f32,
    direction: SimulateDirection,
    vm: &mut VirtualMachine,
) -> Vec<Box<dyn IGameplayAction>> {
    let ctx = gorge_core::objective::native::NativeContext::new(vm);
    let transformers: Vec<usize> = {
        let default = crate::system::native::element_simulator::ElementSimulatorPayload { transformers: Vec::new() };
        ctx.get_payload::<crate::system::native::element_simulator::ElementSimulatorPayload>(object_id)
            .unwrap_or(&default)
            .transformers
            .clone()
    };

    let mut all_actions: Vec<Box<dyn IGameplayAction>> = Vec::new();
    for transformer_id in transformers {
        if transformer_id == 0 { continue; }
        let mut tctx = gorge_core::objective::native::NativeContext::new(vm);
        // 调用 transform 方法 (method 0) 并用 float 参数
        let cls_name = tctx.vm.objects.get(&transformer_id)
            .map(|o| o.class_name.clone()).unwrap_or_default();
        tctx.set_float_param(0, chart_time as f64);
        tctx.invoke_native_method_on(&cls_name, transformer_id, 0);
        let result_arr_id = tctx.get_object_return();
        if result_arr_id != 0 {
            let conv = convert_actions_from_commands(&mut tctx, result_arr_id, direction);
            all_actions.extend(conv);
        }
    }
    all_actions
}

/// 将自动机指令 ObjectArray 转换为 IGameplayAction（内联版本，避免循环依赖）
fn convert_actions_from_commands(
    ctx: &mut gorge_core::objective::native::NativeContext,
    commands_array_id: usize,
    direction: SimulateDirection,
) -> Vec<Box<dyn IGameplayAction>> {
    if commands_array_id == 0 { return Vec::new(); }
    let items = ctx.object_array_items(commands_array_id);
    let mut actions: Vec<Box<dyn IGameplayAction>> = Vec::new();
    for cmd_id in items {
        if cmd_id == 0 { continue; }
        let cls = ctx.vm.objects.get(&cmd_id).map(|o| o.class_name.clone()).unwrap_or_default();
        if cls.contains("DeriveElementCommand") {
            let elem = ctx.get_object_int_field(cmd_id, 0) as usize;
            actions.push(Box::new(DeriveElement { element_id: elem, direction }));
        } else if cls.contains("AppendSignalCommand") {
            let sid = ctx.get_object_int_field(cmd_id, 0) as i32;
            let val = ctx.get_object_int_field(cmd_id, 1) as usize;
            actions.push(Box::new(AppendSignal::new(String::new(), sid, val)));
        } else if cls.contains("DestroyElementCommand") {
            let elem = ctx.get_object_int_field(cmd_id, 0) as usize;
            actions.push(Box::new(DestroyElement { element_id: elem }));
        }
    }
    actions
}

#[cfg(test)]
mod s4_integration_tests {
    use super::*;
    use crate::runtime::simulation_machine::SimulationMachine;
    use gorge_core::diagnostics::Span;
    use gorge_core::objective::class::RuntimeClass;
    use gorge_core::objective::declaration::*;
    use gorge_core::objective::object::RuntimeObject;
    use gorge_core::objective::types::*;
    use gorge_core::system::native::array::ObjectArray;
    use gorge_core::system::native::injector::RuntimeInjector;
    use gorge_core::virtual_machine::ir::{
        CodeWithSpan, CompiledMethod, IntermediateCode, IntermediateOperator, Operand,
    };
    use gorge_core::virtual_machine::vm::VirtualMachine;
    use std::collections::HashMap;
    use std::sync::Arc;

    /// 构造一个最小元素类的 ClassDeclaration，含：
    /// - @ForwardTimedGenerate(time=1.0) 构造注解
    /// - @ForwardTimedDestroy（方法注解，time 参数为 Float 2.0）
    fn make_test_element_decl() -> ClassDeclaration {
        let class_type = GorgeType::class("S4TestElement", Some("GorgeFramework".into()));
        let mut constructor_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        constructor_annotations.insert(0, vec![
            MethodAnnotation {
                name: "ForwardTimedGenerate".into(),
                parameters: vec![("time".into(), AnnotationValue::Float(1.0))],
            },
        ]);

        let mut method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        method_annotations.insert(0, vec![
            MethodAnnotation {
                name: "ForwardTimedDestroy".into(),
                parameters: vec![("time".into(), AnnotationValue::Float(2.0))],
            },
        ]);

        ClassDeclaration {
            class_type,
            is_native: false,
            annotations: vec![],
            fields: vec![
                FieldInfo { name: "simulator".into(), field_type: GorgeType::new(BasicType::Object), is_static: false, is_native: false },
                FieldInfo { name: "late_independent_simulator".into(), field_type: GorgeType::new(BasicType::Object), is_static: false, is_native: false },
                FieldInfo { name: "nodes".into(), field_type: GorgeType::new(BasicType::Object), is_static: false, is_native: false },
                FieldInfo { name: "derived_elements".into(), field_type: GorgeType::new(BasicType::Object), is_static: false, is_native: false },
                FieldInfo { name: "automaton".into(), field_type: GorgeType::new(BasicType::Object), is_static: false, is_native: false },
            ],
            methods: vec![MethodInfo {
                name: "__annotation_ForwardTimedDestroy_time".into(),
                return_type: GorgeType::new(BasicType::Float),
                parameters: vec![],
                is_static: false, is_native: false, is_override: false, is_abstract: false,
            }],
            static_methods: vec![],
            constructors: vec![ConstructorInfo { parameters: vec![], is_native: false, is_injector: false }],
            injector_fields: vec![],
            super_class: None,
            super_interfaces: vec![],
            field_type_count: TypeCount { int_count: 0, float_count: 0, bool_count: 0, string_count: 0, object_count: 5 },
            method_count: 1,
            static_method_count: 0,
            constructor_count: 1,
            injector_field_type_count: TypeCount::zero(),
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0,
            constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
            injector_constructor_impl_id: vec![],
            method_annotations,
            constructor_annotations,
        }
    }

    /// 构造一个空的构造方法体（仅含 ReturnInt，不执行任何操作）
    fn make_empty_ctor_method() -> CompiledMethod {
        let code = IntermediateCode {
            result: None,
            operator: IntermediateOperator::ReturnInt,
            left: Operand::int(0),
            right: None,
        };
        CompiledMethod {
            name: "ctor".into(),
            codes: vec![CodeWithSpan::new(code, Span::dummy())],
            local_count: 0,
        }
    }

    /// 向 VM 注册最小测试类，返回类全名和构造方法 ID
    fn register_test_class(vm: &mut VirtualMachine) -> (String, usize) {
        let decl = make_test_element_decl();
        let class_name = decl.class_type.full_name();
        let mut runtime_class = RuntimeClass::new(decl, None);

        // 注册构造方法实现（全局ID=0，对应 constructor_start_id=0）
        runtime_class.register_constructor(0, make_empty_ctor_method());

        vm.class_table.insert(class_name.clone(), Arc::new(runtime_class));
        (class_name, 0)
    }

    /// 集成测试：注解驱动的元素创生 → 模拟推进 → 定时销毁
    #[test]
    fn test_annotation_driven_generate_and_destroy() {
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;

        let (class_name, ctor_id) = register_test_class(&mut vm);

        // 创建注入器对象
        let decl = vm.class_table.get(&class_name).unwrap().declaration.clone();
        let injector = RuntimeInjector::new(Arc::new(decl));
        let injector_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(injector_id, injector);

        // add_score_element → 填充定时创生表
        let mut runtime = GorgeSimulationRuntime::new();
        runtime.chart.add_score_element(injector_id, &mut vm);

        // 验证：生成表中应该有 1 条记录
        assert_eq!(runtime.chart.forward_timed_generate_list.len(), 1);
        let (gen_time, gen_injector, gen_ctor) = runtime.chart.forward_timed_generate_list[0];
        assert!((gen_time - 1.0).abs() < 0.01, "生成时间应为 1.0");
        assert_eq!(gen_injector, injector_id);
        assert_eq!(gen_ctor, ctor_id);

        // 创建模拟机并初始化
        let mut machine = SimulationMachine::new(0.0, 100.0, 1.0);
        machine.runtime_initialize();

        // 推进到 t=1.5：应在此时点之后生成元素（time=1.0 在 (0, 1.5] 内）
        machine.drive(1.5, &mut runtime, &mut vm);

        // 断言：AliveElements 有元素
        assert_eq!(runtime.chart.alive_elements.len(), 1, "t=1.5 时应已生成元素");
        let element_id = runtime.chart.alive_elements[0];

        // 定时销毁表应从注解扫描填入
        assert_eq!(runtime.chart.forward_timed_destroy_list.len(), 1, "注解扫描应产生定时销毁条目");
        let (destroy_time, destroy_elem) = runtime.chart.forward_timed_destroy_list[0];
        assert!((destroy_time - 2.0).abs() < 0.01, "销毁时间应为 2.0");
        assert_eq!(destroy_elem, element_id);

        // 继续推进到 t=2.5：超时销毁
        machine.drive(1.0, &mut runtime, &mut vm);
        assert!(runtime.chart.alive_elements.is_empty(), "t=2.5 时元素应已被销毁");
    }

    /// 测试：add_score_element 处理 @InitializeGenerate 构造注解
    #[test]
    fn test_add_score_element_initialize_generate() {
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;

        let mut decl = make_test_element_decl();
        decl.constructor_annotations.clear();
        decl.constructor_annotations.insert(0, vec![
            MethodAnnotation { name: "InitializeGenerate".into(), parameters: vec![] },
        ]);

        let class_name = decl.class_type.full_name();
        let mut runtime_class = RuntimeClass::new(decl, None);
        runtime_class.register_constructor(0, make_empty_ctor_method());
        vm.class_table.insert(class_name.clone(), Arc::new(runtime_class));

        let injector = RuntimeInjector::new(Arc::new(
            vm.class_table.get(&class_name).unwrap().declaration.clone(),
        ));
        let injector_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(injector_id, injector);

        let mut runtime = GorgeSimulationRuntime::new();
        runtime.chart.add_score_element(injector_id, &mut vm);

        assert_eq!(runtime.chart.initialize_generate_list.len(), 1);
        assert_eq!(runtime.chart.initialize_generate_list[0], (injector_id, 0));
    }

    /// 测试：add_score_element 处理 Delegate 注解参数
    #[test]
    fn test_add_score_element_delegate_time() {
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;

        let mut decl = make_test_element_decl();
        decl.constructor_annotations.clear();
        decl.constructor_annotations.insert(0, vec![
            MethodAnnotation {
                name: "ForwardTimedGenerate".into(),
                parameters: vec![("time".into(), AnnotationValue::Delegate(0))],
            },
        ]);

        let class_name = decl.class_type.full_name();
        let mut runtime_class = RuntimeClass::new(decl, None);
        runtime_class.register_constructor(0, make_empty_ctor_method());
        // 注册隐藏方法（全局ID=0），返回 float 通过 vm.return_float
        let return_code = IntermediateCode {
            result: None,
            operator: IntermediateOperator::ReturnFloat,
            left: Operand::float(3.0),
            right: None,
        };
        let hidden_method = CompiledMethod {
            name: "__annotation_ForwardTimedGenerate_time".into(),
            codes: vec![CodeWithSpan::new(return_code, Span::dummy())],
            local_count: 0,
        };
        runtime_class.register_method(0, hidden_method);
        vm.class_table.insert(class_name.clone(), Arc::new(runtime_class));

        let injector = RuntimeInjector::new(Arc::new(
            vm.class_table.get(&class_name).unwrap().declaration.clone(),
        ));
        let injector_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(injector_id, injector);

        let mut runtime = GorgeSimulationRuntime::new();
        runtime.chart.add_score_element(injector_id, &mut vm);

        assert_eq!(runtime.chart.forward_timed_generate_list.len(), 1);
        // Delegate 方法体返回 float 3.0
        assert!((runtime.chart.forward_timed_generate_list[0].0 - 3.0).abs() < 0.01,
            "Delegate 方法返回 3.0，time 应为 3.0");
    }

    #[test]
    fn test_graphics_node_update_method_uses_derived_native_abi() {
        assert_eq!(ELEMENT_SIMULATOR_FIELD, 0);
        assert_eq!(ELEMENT_LATE_INDEPENDENT_SIMULATOR_FIELD, 1);
        assert_eq!(ELEMENT_NODES_FIELD, 2);
        assert_eq!(ELEMENT_DERIVED_ELEMENTS_FIELD, 3);
        assert_eq!(NOTE_AUTOMATON_FIELD, 4);
        assert_eq!(graphics_node_update_method("GorgeFramework.Node"), Some(5));
        assert_eq!(graphics_node_update_method("GorgeFramework.Sprite"), Some(0));
        assert_eq!(graphics_node_update_method("NineSliceSprite"), Some(0));
        assert_eq!(graphics_node_update_method("GorgeFramework.CurveSprite"), Some(0));
        assert_eq!(graphics_node_update_method("Dremu.UnknownNode"), None);
    }

    #[test]
    fn test_derive_element_registers_nodes_from_element_nodes_field() {
        let mut vm = VirtualMachine::new();
        let (class_name, _) = register_test_class(&mut vm);
        let element_id = 1;
        let nodes_array_id = 2;
        let node_id = 3;

        let mut element = RuntimeObject::new_simple(
            class_name,
            &TypeCount { object_count: 5, ..TypeCount::zero() },
        );
        element.set_object_field(ELEMENT_NODES_FIELD, nodes_array_id);
        vm.objects.insert(element_id, element);
        vm.native_payloads.insert(
            nodes_array_id,
            Box::new(ObjectArray { items: vec![node_id] }),
        );

        let mut runtime = GorgeSimulationRuntime::new();
        do_derive_element(
            &mut runtime,
            &mut vm,
            element_id,
            SimulateDirection::Forward,
        );

        assert_eq!(runtime.chart.alive_elements, vec![element_id]);
        assert_eq!(runtime.graphics.nodes, vec![node_id]);
        assert!(runtime.automaton.automatons.is_empty());
    }
}
