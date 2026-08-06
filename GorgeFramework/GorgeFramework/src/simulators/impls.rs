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
    fn forward_async_simulation_target(&self, chart_time: f32, runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 {
        let list = &runtime.chart.forward_timed_generate_list;
        if list.is_empty() { return f32::MAX; }
        list.iter()
            .map(|(t, _, _)| if *t > chart_time { *t } else { f32::MAX })
            .fold(f32::MAX, f32::min)
    }

    fn backward_async_simulation_target(&self, chart_time: f32, runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 {
        let list = &runtime.chart.backward_timed_generate_list;
        if list.is_empty() { return f32::MIN; }
        list.iter()
            .map(|(t, _, _)| if *t < chart_time { *t } else { f32::MIN })
            .fold(f32::MIN, f32::max)
    }

    fn infinitesimal_async_simulation_target(&self, _chart_time: f32, _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 {
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
    fn forward_async_simulation_target(&self, chart_time: f32, runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 {
        let list = &runtime.chart.forward_timed_destroy_list;
        if list.is_empty() { return f32::MAX; }
        list.iter()
            .map(|(t, _)| if *t > chart_time { *t } else { f32::MAX })
            .fold(f32::MAX, f32::min)
    }

    fn backward_async_simulation_target(&self, chart_time: f32, runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 {
        let list = &runtime.chart.backward_timed_destroy_list;
        if list.is_empty() { return f32::MIN; }
        list.iter()
            .map(|(t, _)| if *t < chart_time { *t } else { f32::MIN })
            .fold(f32::MIN, f32::max)
    }

    fn infinitesimal_async_simulation_target(&self, _chart_time: f32, _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 {
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

/// 响应延迟（秒），对齐 C# `StaticConfig.RespondDelay`（当前为 0）。
///
/// 用于将乐段的播放窗口起始时间整体延后，临时播放延迟调整。
const RESPOND_DELAY: f32 = 0.0;

impl ISimulator for SongSimulator {
    fn forward_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 { f32::MAX }
    fn backward_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 { f32::MIN }
    fn infinitesimal_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 { f32::MAX }

    fn forward_simulate(
        &self, _chart_from: f32, chart_to: f32,
        _snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime,
        _vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        // 遍历所有乐段音频源，按播放窗口控制播放/停止（对齐 C# `ForwardSimulate`）。
        // 每段乐段：
        //   startChartTime = period.Config.timeOffset + StaticConfig.RespondDelay
        //   endChartTime   = startChartTime + audioPlayer.AudioLength()
        // 若 chart_to 落在 [start, end) 内且未播放 → SetTime(chart_to - start) + Play；
        // 否则若正在播放 → Stop。
        // 返回空动作列表（音频播放为副效应，不产生 GameplayAction）。
        let period_ids: Vec<usize> = runtime.audio.period_audio_sources.keys().cloned().collect();
        for period_id in period_ids {
            let Some(player) = runtime.audio.period_player(period_id) else {
                continue;
            };
            let start_chart_time = runtime.audio.period_time_offset(period_id) + RESPOND_DELAY;
            let end_chart_time = start_chart_time + player.audio_length();
            if chart_to >= start_chart_time && chart_to < end_chart_time {
                if !player.is_playing() {
                    player.set_time(chart_to - start_chart_time);
                    player.play();
                }
            } else if player.is_playing() {
                player.stop();
            }
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
    fn forward_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 { f32::MAX }
    fn backward_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 { f32::MIN }
    fn infinitesimal_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 { f32::MAX }

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

/// 返回具体图形节点 native 类的销毁方法编号（P1-4）。
///
/// `Node.Destroy` 在 Node native ABI 中是 6 号方法；三个派生渲染节点
/// 各自在本类 native ABI 中以 1 号方法覆盖销毁逻辑（置 alive=false +
/// 销毁平台精灵）。未知类返回 None（无对应 native ABI，跳过销毁调用）。
fn graphics_node_destroy_method(class_name: &str) -> Option<usize> {
    match class_name.rsplit('.').next().unwrap_or(class_name) {
        "Node" => Some(6),
        "Sprite" | "NineSliceSprite" | "CurveSprite" => Some(1),
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
    /// 对齐 C# `Terminate.DoAction`：`Base.Instance.Log("Terminated")` +
    /// `runtime.OnTerminate?.Invoke()`（P1-4 实体化）。
    fn do_action(
        &self, runtime: &mut GorgeSimulationRuntime,
        _edge_queue: &mut MultichannelEdgeQueue, _vm: &mut VirtualMachine,
    ) {
        runtime.logger.debug_log("Terminated", 0);
        if let Some(ref mut callback) = runtime.on_terminate {
            callback();
        }
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
        // P1-4：注册键记录到 element_simulator_keys，供 DestroyElement 精确注销
        let simulator_id = vm.objects.get(&element_id)
            .map(|o| o.get_object_field(ELEMENT_SIMULATOR_FIELD))
            .unwrap_or(0);
        let main_key = if simulator_id != 0 {
            // S7: 通过 ElementSimulatorAdapter 包装原生物件，注册进 SimRegistry
            let adapter = Box::new(ElementSimulatorAdapter::new(simulator_id));
            let reg_key = runtime.sim_registry.register(adapter);
            runtime.simulation.simulators.register(10, reg_key);
            Some(reg_key)
        } else {
            None
        };
        let late_sim_id = vm.objects.get(&element_id)
            .map(|o| o.get_object_field(ELEMENT_LATE_INDEPENDENT_SIMULATOR_FIELD))
            .unwrap_or(0);
        let late_key = if late_sim_id != 0 {
            let adapter = Box::new(ElementSimulatorAdapter::new(late_sim_id));
            let reg_key = runtime.late_sim_registry.register(adapter);
            runtime.simulation.late_independent_simulators.register(10, reg_key);
            Some(reg_key)
        } else {
            None
        };
        if main_key.is_some() || late_key.is_some() {
            runtime.chart.element_simulator_keys.insert(element_id, (main_key, late_key));
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
                fill_pending_detection_conditions(automaton_id, SimulateDirection::Forward, runtime, vm);
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
    /// 对齐 C# `DestroyElement.DoAction`（P1-4 实体化）：
    /// Note 自动机注销 → 模拟器注销 → 统一表注销 → 图形节点销毁。
    ///
    /// 与 C# 一致，节点不建 node→element 反查表，而是直读元素的
    /// `nodes` 字段（ObjectArray）逐个 `Destroy()` 并从节点表移除。
    fn do_action(
        &self,
        runtime: &mut GorgeSimulationRuntime,
        _edge_queue: &mut MultichannelEdgeQueue,
        vm: &mut VirtualMachine,
    ) {
        // 元素对象在销毁时点仍在 VM 对象表中；缺失时跳过字段读取步骤
        let element_class = vm.objects.get(&self.element_id)
            .map(|o| o.class_name.clone())
            .unwrap_or_default();

        // 1. Note 判定与自动机注销（对齐 C# `_element is Note note` 分支）
        if !element_class.is_empty() && is_subclass_of_note(&element_class, vm) {
            let automaton_id = vm.objects.get(&self.element_id)
                .map(|o| o.get_object_field(NOTE_AUTOMATON_FIELD))
                .unwrap_or(0);
            if automaton_id != 0 {
                // C# 移除的是 note.automaton 对象，而非元素 ID
                runtime.automaton.pending_detection_conditions.remove(&automaton_id);
                runtime.automaton.automatons.retain(|&id| id != automaton_id);
            }
            runtime.chart.alive_notes.retain(|&id| id != self.element_id);
        }

        // 2. 注销模拟器（对齐 C# `Simulators.Remove(element.simulator)`）
        // C# 按模拟器对象引用从堆中删除；Rust 堆按注册键删除，
        // 键在 GenerateElement 注册时记录于 element_simulator_keys。
        if let Some((main_key, late_key)) = runtime.chart.element_simulator_keys.remove(&self.element_id) {
            if let Some(key) = main_key {
                runtime.simulation.simulators.remove(&key);
                runtime.sim_registry.remove(key);
            }
            if let Some(key) = late_key {
                runtime.simulation.late_independent_simulators.remove(&key);
                runtime.late_sim_registry.remove(key);
            }
        }

        // 3. 从统一表中注销
        runtime.chart.alive_elements.retain(|&id| id != self.element_id);
        runtime.chart.alive_injector_map.remove(&self.element_id);

        // 4. 注销图形节点（对齐 C#：逐节点 `node.Destroy()` + `Graphics.Nodes.Remove(node)`）
        let nodes_array_id = vm.objects.get(&self.element_id)
            .map(|o| o.get_object_field(ELEMENT_NODES_FIELD))
            .unwrap_or(0);
        if nodes_array_id != 0 {
            let node_ids = vm.native_payloads
                .get(&nodes_array_id)
                .and_then(|p| p.downcast_ref::<gorge_core::system::native::array::ObjectArray>())
                .map(|a| a.items.clone())
                .unwrap_or_default();
            for node_id in node_ids {
                if node_id == 0 { continue; }
                let node_class = vm.objects.get(&node_id)
                    .map(|o| o.class_name.clone())
                    .unwrap_or_default();
                if let Some(destroy_method) = graphics_node_destroy_method(&node_class) {
                    let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
                    ctx.invoke_native_method_on(&node_class, node_id, destroy_method);
                }
                runtime.graphics.nodes.retain(|&id| id != node_id);
            }
        }

        // 5. 注销定时销毁条目（C# 列为 TODO 合并项，Rust 保留此处清理）
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
    direction: SimulateDirection,
    runtime: &mut GorgeSimulationRuntime,
    vm: &mut VirtualMachine,
) {
    let mut ctx = gorge_core::objective::native::NativeContext::new(vm);

    // 调用 SignalTsiga.get_detection_conditions（方法 6，返回 filter_id 或 0）
    let direction_code = match direction {
        SimulateDirection::Forward => 0,
        SimulateDirection::Backward => 1,
        SimulateDirection::Infinitesimal => 2,
    };
    ctx.set_int_param(0, direction_code);
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
            direction,
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
    // 注册模拟器（P1-4：注册键记录到 element_simulator_keys，供 DestroyElement 精确注销）
    let simulator_id = vm.objects.get(&element_id)
        .map(|o| o.get_object_field(ELEMENT_SIMULATOR_FIELD))
        .unwrap_or(0);
    let main_key = if simulator_id != 0 {
        let adapter = Box::new(ElementSimulatorAdapter::new(simulator_id));
        let registry_id = runtime.sim_registry.register(adapter);
        runtime.simulation.simulators.register(10, registry_id);
        Some(registry_id)
    } else {
        None
    };
    let late_sim_id = vm.objects.get(&element_id)
        .map(|o| o.get_object_field(ELEMENT_LATE_INDEPENDENT_SIMULATOR_FIELD))
        .unwrap_or(0);
    let late_key = if late_sim_id != 0 {
        let adapter = Box::new(ElementSimulatorAdapter::new(late_sim_id));
        let registry_id = runtime.late_sim_registry.register(adapter);
        runtime.simulation.late_independent_simulators.register(10, registry_id);
        Some(registry_id)
    } else {
        None
    };
    if main_key.is_some() || late_key.is_some() {
        runtime.chart.element_simulator_keys.insert(element_id, (main_key, late_key));
    }
    // Note 判定 + 自动机注册
    if is_subclass_of_note(&class_name, vm) {
        runtime.chart.alive_notes.push(element_id);
        let automaton_id = vm.objects.get(&element_id)
            .map(|o| o.get_object_field(NOTE_AUTOMATON_FIELD))
            .unwrap_or(0);
        if automaton_id != 0 {
            runtime.automaton.automatons.push(automaton_id);
            fill_pending_detection_conditions(automaton_id, SimulateDirection::Forward, runtime, vm);
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
        &self, runtime: &mut GorgeSimulationRuntime,
        _edge_queue: &mut MultichannelEdgeQueue, vm: &mut VirtualMachine,
    ) {
        // 对齐 C# `UpdatePendingDetectionCondition.DoAction`：
        // 用动作携带的方向重新计算该自动机的检测条件并整体覆盖写回。
        fill_pending_detection_conditions(self.automaton_id, self.direction, runtime, vm);
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
    fn forward_async_simulation_target(&self, _chart_time: f32, runtime: &GorgeSimulationRuntime, vm: &mut VirtualMachine) -> f32 {
        if runtime.automaton.automatons.is_empty() { return f32::MAX; }
        // 计算所有自动机的最早状态转换时间（谱面时间），对齐 C# `ForwardAsyncSimulationTarget`
        // 调用 SignalTsiga native 方法 0（forward_state_change_time），取最小时间
        let automaton_ids: Vec<usize> = runtime.automaton.automatons.clone();
        let mut earliest = f32::MAX;
        for tsiga_id in automaton_ids {
            let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
            ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", tsiga_id, 0);
            let t = ctx.get_float_return() as f32;
            if t < earliest { earliest = t; }
        }
        earliest
    }

    fn backward_async_simulation_target(&self, _chart_time: f32, runtime: &GorgeSimulationRuntime, vm: &mut VirtualMachine) -> f32 {
        if runtime.automaton.automatons.is_empty() { return f32::MIN; }
        // 计算所有自动机的最晚状态转换时间（谱面时间），对齐 C# `BackwardAsyncSimulationTarget`
        // 调用 SignalTsiga native 方法 2（backward_state_change_time），取最大时间
        let automaton_ids: Vec<usize> = runtime.automaton.automatons.clone();
        let mut latest = f32::MIN;
        for tsiga_id in automaton_ids {
            let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
            ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", tsiga_id, 2);
            let t = ctx.get_float_return() as f32;
            if t > latest { latest = t; }
        }
        latest
    }

    fn infinitesimal_async_simulation_target(&self, _chart_time: f32, _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 {
        // 对齐 C# `InfinitesimalAsyncSimulationTarget`：固定返回 f32::MAX，
        // 不基于 pending_detection_conditions 计算目标（该竞争检测在 instant_simulate 中处理）
        f32::MAX
    }

    fn forward_simulate(
        &self, _chart_from: f32, chart_to: f32,
        _snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        // S7: 正向推进——遍历各信号自动机，调用 SignalTsiga 方法 1（forward_state_change），
        // 收集其返回的命令 ObjectArray 转换为 IGameplayAction。
        // 对齐 C# `ForwardSimulate`：遍历 runtime.Automaton.Automatons，
        // 累加每个 automaton.ForwardStateChange(chartTimeTo) 返回的动作；
        // 状态变换后追加 UpdatePendingDetectionCondition 以刷新待决条件、驱动收敛。
        let automaton_ids: Vec<usize> = runtime.automaton.automatons.clone();
        let mut all_actions: Vec<Box<dyn IGameplayAction>> = Vec::new();
        for tsiga_id in automaton_ids {
            let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
            ctx.set_float_param(0, chart_to as f64);
            ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", tsiga_id, 1);
            let arr_id = ctx.get_object_return();
            if arr_id != 0 {
                // 命令数组非空 → 发生了状态转移/边响应，转换命令并追加刷新待决动作
                let commands = convert_actions_from_commands(&mut ctx, arr_id, SimulateDirection::Forward);
                all_actions.extend(commands);
                all_actions.push(Box::new(UpdatePendingDetectionCondition::new(tsiga_id, SimulateDirection::Forward)));
            }
        }
        all_actions
    }

    fn backward_simulate(
        &self, _chart_from: f32, chart_to: f32,
        _snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        // S7: 反向推进——遍历各信号自动机，调用 SignalTsiga 方法 3（backward_state_change），
        // 收集其返回的受影响自动机 ID ObjectArray 转换为 IGameplayAction。
        // 对齐 C# `BackwardSimulate`：遍历 runtime.Automaton.Automatons，
        // 累加每个 automaton.BackwardStateChange(chartTimeTo) 返回的动作。
        // C# 反向弹栈（HistoryStack.PopUntil）仅产生 `UpdatePendingDetectionCondition`
        // 直接动作（非命令动作），故不适用 convert_actions_from_commands；
        // 此处读取方法 3 返回的受影响自动机 ID 数组，非空即为该自动机追加一个
        // UpdatePendingDetectionCondition(Backward) 动作以刷新待决条件、驱动收敛。
        let automaton_ids: Vec<usize> = runtime.automaton.automatons.clone();
        let mut all_actions: Vec<Box<dyn IGameplayAction>> = Vec::new();
        for tsiga_id in automaton_ids {
            let mut ctx = gorge_core::objective::native::NativeContext::new(vm);
            ctx.set_float_param(0, chart_to as f64);
            ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", tsiga_id, 3);
            let arr_id = ctx.get_object_return();
            if arr_id != 0 {
                // 反向弹栈产生了受影响自动机 → 追加刷新待决动作
                let affected = ctx.object_array_items(arr_id);
                if !affected.is_empty() {
                    all_actions.push(Box::new(UpdatePendingDetectionCondition::new(tsiga_id, SimulateDirection::Backward)));
                }
            }
        }
        all_actions
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
        // 收集本轮竞争检测产生的全部动作（对齐 C# `gameActions`）
        let mut game_actions: Vec<Box<dyn IGameplayAction>> = Vec::new();

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
                        let accept_arr_id = {
                            let mut accept_ctx = gorge_core::objective::native::NativeContext::new(vm);
                            accept_ctx.set_float_param(0, chart_time as f64);
                            let dir = match direction {
                                SimulateDirection::Forward => 0,
                                SimulateDirection::Backward => 1,
                                SimulateDirection::Infinitesimal => 2,
                            };
                            accept_ctx.set_int_param(0, dir);
                            accept_ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", cond.tsiga_id, 4);
                            accept_ctx.get_object_return()
                        };
                        // 命令数组非空 → 发生了状态转移，转换命令并追加刷新待决动作
                        if accept_arr_id != 0 {
                            let mut actions_ctx = gorge_core::objective::native::NativeContext::new(vm);
                            let commands = convert_actions_from_commands(&mut actions_ctx, accept_arr_id, direction);
                            game_actions.extend(commands);
                            game_actions.push(Box::new(UpdatePendingDetectionCondition::new(cond.tsiga_id, direction)));
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
                let deny_arr_id = {
                    let mut deny_ctx = gorge_core::objective::native::NativeContext::new(vm);
                    deny_ctx.set_float_param(0, chart_time as f64);
                    deny_ctx.set_int_param(0, 2);
                    deny_ctx.invoke_native_method_on("GorgeFramework.SignalTsiga", *tsiga_id, 5);
                    deny_ctx.get_object_return()
                };
                // 命令数组非空 → 发生了状态转移，转换命令并追加刷新待决动作
                // 对齐 C# `DetectionDeny(chartTimeTo, SimulateDirection.Infinitesimal)`：
                // 拒绝分支固定使用 Infinitesimal 方向（native 方法 5 的 direction 参数
                // 上方已固定传 2，此处命令转换与刷待决动作同向）。
                if deny_arr_id != 0 {
                    let mut actions_ctx = gorge_core::objective::native::NativeContext::new(vm);
                    let commands = convert_actions_from_commands(&mut actions_ctx, deny_arr_id, SimulateDirection::Infinitesimal);
                    game_actions.extend(commands);
                    game_actions.push(Box::new(UpdatePendingDetectionCondition::new(*tsiga_id, SimulateDirection::Infinitesimal)));
                }
            }
        }

        game_actions
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
    fn forward_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 { f32::MAX }
    fn backward_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 { f32::MIN }
    fn infinitesimal_async_simulation_target(&self, _: f32, _runtime: &GorgeSimulationRuntime, _vm: &mut VirtualMachine) -> f32 { f32::MAX }

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

        // P0-5：注册父类链使 is_element_subclass 判定通过（GorgeFramework.Element）
        vm.register_class_super(&class_name, "GorgeFramework.Element");

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
        let default_config = crate::chart::period::PeriodConfig::default();
        runtime.chart.add_score_element(injector_id, &default_config, &mut vm);

        // 验证：生成表中应该有 1 条记录（使用 gameplay 修改版注入器 clone ID）
        assert_eq!(runtime.chart.forward_timed_generate_list.len(), 1);
        let (gen_time, gen_injector, gen_ctor) = runtime.chart.forward_timed_generate_list[0];
        assert!((gen_time - 1.0).abs() < 0.01, "生成时间应为 1.0");
        assert_eq!(gen_injector, injector_id + 1, "生成表应使用 gameplay 修改版注入器");
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
        // P0-5：注册父类链（GorgeFramework.Element）
        vm.register_class_super(&class_name, "GorgeFramework.Element");
        vm.class_table.insert(class_name.clone(), Arc::new(runtime_class));

        let injector = RuntimeInjector::new(Arc::new(
            vm.class_table.get(&class_name).unwrap().declaration.clone(),
        ));
        let injector_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(injector_id, injector);

        let mut runtime = GorgeSimulationRuntime::new();
        let default_config = crate::chart::period::PeriodConfig::default();
        runtime.chart.add_score_element(injector_id, &default_config, &mut vm);

        assert_eq!(runtime.chart.initialize_generate_list.len(), 1);
        assert_eq!(runtime.chart.initialize_generate_list[0], (injector_id + 1, 0));
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
        // P0-5：注册父类链（GorgeFramework.Element）
        vm.register_class_super(&class_name, "GorgeFramework.Element");
        vm.class_table.insert(class_name.clone(), Arc::new(runtime_class));

        let injector = RuntimeInjector::new(Arc::new(
            vm.class_table.get(&class_name).unwrap().declaration.clone(),
        ));
        let injector_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(injector_id, injector);

        let mut runtime = GorgeSimulationRuntime::new();
        let default_config = crate::chart::period::PeriodConfig::default();
        runtime.chart.add_score_element(injector_id, &default_config, &mut vm);

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

    // ==================== P1-4 Element 销毁链测试 ====================

    #[test]
    fn test_p1_4_terminate_invokes_on_terminate() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let mut runtime = GorgeSimulationRuntime::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_in_callback = Arc::clone(&counter);
        runtime.on_terminate = Some(Box::new(move || {
            counter_in_callback.fetch_add(1, Ordering::SeqCst);
        }));

        let mut vm = VirtualMachine::new();
        let mut edge_queue = MultichannelEdgeQueue::new();
        Terminate.do_action(&mut runtime, &mut edge_queue, &mut vm);
        assert_eq!(counter.load(Ordering::SeqCst), 1, "Terminate 应触发 on_terminate 回调");

        // 未注册回调时不应 panic
        let mut runtime_no_callback = GorgeSimulationRuntime::new();
        Terminate.do_action(&mut runtime_no_callback, &mut edge_queue, &mut vm);
    }

    #[test]
    fn test_p1_4_graphics_node_destroy_method_mapping() {
        assert_eq!(graphics_node_destroy_method("GorgeFramework.Node"), Some(6));
        assert_eq!(graphics_node_destroy_method("GorgeFramework.Sprite"), Some(1));
        assert_eq!(graphics_node_destroy_method("NineSliceSprite"), Some(1));
        assert_eq!(graphics_node_destroy_method("GorgeFramework.CurveSprite"), Some(1));
        assert_eq!(graphics_node_destroy_method("Dremu.UnknownNode"), None);
    }

    #[test]
    fn test_p1_4_destroy_element_full_chain() {
        use crate::system::native::node_native::Node;
        use gorge_core::objective::native::NativeClass;

        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;

        // 注册 Node native 类（destroy 分派目标）
        let node_native = Node {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
        };
        vm.register_native_class("GorgeFramework.Node", Arc::new(Node {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
        }));

        // Note 子类（类名含 "Note" 即通过 is_subclass_of_note 判定）
        let note_class_name = "S4TestNote".to_string();
        vm.register_class_super(&note_class_name, "GorgeFramework.Note");

        // 对象 ID 布局
        let element_id = 1;
        let simulator_obj_id = 2;
        let automaton_id = 3;
        let nodes_array_id = 4;
        let node_id = 5;

        // 元素对象：simulator=2, late=0, nodes=4, derived=0, automaton=3
        let mut element = RuntimeObject::new_simple(
            note_class_name,
            &TypeCount { object_count: 5, ..TypeCount::zero() },
        );
        element.set_object_field(ELEMENT_SIMULATOR_FIELD, simulator_obj_id);
        element.set_object_field(ELEMENT_NODES_FIELD, nodes_array_id);
        element.set_object_field(NOTE_AUTOMATON_FIELD, automaton_id);
        vm.objects.insert(element_id, element);

        // 节点对象（alive=true）
        let mut node_obj = RuntimeObject::new_simple(
            "GorgeFramework.Node".to_string(),
            node_native.field_type_count(),
        );
        node_obj.set_bool_field(Node::FIELD_INDEX_alive, true);
        vm.objects.insert(node_id, node_obj);
        vm.native_payloads.insert(nodes_array_id, Box::new(ObjectArray { items: vec![node_id] }));

        // 播种运行期状态（模拟 GenerateElement 已执行）
        let mut runtime = GorgeSimulationRuntime::new();
        runtime.chart.alive_elements.push(element_id);
        runtime.chart.alive_notes.push(element_id);
        runtime.chart.alive_injector_map.insert(element_id, 99);
        runtime.chart.forward_timed_destroy_list.push((2.0, element_id));
        runtime.chart.backward_timed_destroy_list.push((0.5, element_id));
        let main_key = runtime.sim_registry.register(Box::new(ElementSimulatorAdapter::new(simulator_obj_id)));
        runtime.simulation.simulators.register(10, main_key);
        runtime.chart.element_simulator_keys.insert(element_id, (Some(main_key), None));
        runtime.automaton.automatons.push(automaton_id);
        runtime.automaton.pending_detection_conditions.insert(automaton_id, Vec::new());
        runtime.graphics.nodes.push(node_id);

        // 执行销毁
        let mut edge_queue = MultichannelEdgeQueue::new();
        DestroyElement::new(element_id).do_action(&mut runtime, &mut edge_queue, &mut vm);

        // 统一表注销
        assert!(runtime.chart.alive_elements.is_empty(), "存活元素表应移除");
        assert!(runtime.chart.alive_notes.is_empty(), "存活 Note 表应移除");
        assert!(runtime.chart.alive_injector_map.is_empty(), "注入器映射应移除");
        assert!(runtime.chart.element_simulator_keys.is_empty(), "模拟器键映射应移除");

        // 自动机注销（按自动机对象 ID，而非元素 ID）
        assert!(runtime.automaton.automatons.is_empty(), "自动机表应移除 note.automaton");
        assert!(runtime.automaton.pending_detection_conditions.is_empty(), "待决检测条件应移除");

        // 模拟器注销：堆与注册表同步移除（标准模拟器仍在堆中，断言目标键已不存在）
        assert!(runtime.simulation.simulators.iter().all(|(_, key)| *key != main_key),
            "主模拟器堆应移除注册键");
        assert!(runtime.sim_registry.get(main_key).is_none(), "SimRegistry 应移除适配器");

        // 图形节点：destroy 调用（alive=false）+ 节点表移除
        assert!(runtime.graphics.nodes.is_empty(), "图形节点表应移除该元素节点");
        assert!(!vm.objects.get(&node_id).unwrap().get_bool_field(Node::FIELD_INDEX_alive),
            "节点应被 Destroy（alive=false）");

        // 定时销毁条目清理
        assert!(runtime.chart.forward_timed_destroy_list.is_empty());
        assert!(runtime.chart.backward_timed_destroy_list.is_empty());
    }

    #[test]
    fn test_p1_4_destroy_element_after_generate_via_keys() {
        // 端到端：GenerateElement 注册模拟器键 → DestroyElement 按键精确注销
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;
        let (class_name, ctor_id) = register_test_class(&mut vm);

        let decl = vm.class_table.get(&class_name).unwrap().declaration.clone();
        let injector = RuntimeInjector::new(Arc::new(decl));
        let injector_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(injector_id, injector);

        let mut runtime = GorgeSimulationRuntime::new();
        let mut edge_queue = MultichannelEdgeQueue::new();

        // 创生（构造体为空，元素对象全部字段为 0 → 不注册模拟器/节点/自动机，
        // 但销毁链仍需完整清表不 panic）
        GenerateElement {
            injector_id,
            constructor_id: ctor_id,
            is_auto_play: false,
            is_reverse: false,
            direction: SimulateDirection::Infinitesimal,
        }.do_action(&mut runtime, &mut edge_queue, &mut vm);
        assert_eq!(runtime.chart.alive_elements.len(), 1);
        let element_id = runtime.chart.alive_elements[0];
        assert_eq!(runtime.chart.forward_timed_destroy_list.len(), 1);

        DestroyElement::new(element_id).do_action(&mut runtime, &mut edge_queue, &mut vm);
        assert!(runtime.chart.alive_elements.is_empty());
        assert!(runtime.chart.alive_injector_map.is_empty());
        assert!(runtime.chart.forward_timed_destroy_list.is_empty());
        assert!(runtime.chart.element_simulator_keys.is_empty());
    }

    /// 构造一个最小 SignalTsiga 自动机对象（无引用边，方法 6 在 Forward 方向返回有效 filter）。
    /// 返回自动机对象 ID。
    fn make_test_tsiga(vm: &mut VirtualMachine) -> usize {
        use crate::system::native::signal_tsiga::SignalTsiga;
        use gorge_core::objective::native::NativeClass;
        vm.native_class_table.insert(
            "GorgeFramework.SignalTsiga".into(),
            std::sync::Arc::new(SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 }),
        );
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        vm.param_pool.set_object_param(0, 0);
        vm.param_pool.set_object_param(1, 0);
        vm.param_pool.set_object_param(2, 0);
        let id = { let mut c = gorge_core::objective::native::NativeContext::new(vm); st.do_construct_native(&mut c, None, 0) };
        id
    }

    /// 回归测试：`UpdatePendingDetectionCondition::do_action` 用传入方向重新计算
    /// 并整体覆盖写回 `pending_detection_conditions[automaton_id]`（对齐 C#）。
    #[test]
    fn test_update_pending_detection_condition_recomputes_and_overwrites() {
        use crate::runtime::simulation_types::SimulateDirection;
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 200;
        let automaton_id = make_test_tsiga(&mut vm);
        assert!(automaton_id > 0);

        let mut runtime = GorgeSimulationRuntime::new();
        let mut edge_queue = MultichannelEdgeQueue::new();

        // 首次执行：应填充该自动机的待决检测条件
        let action = UpdatePendingDetectionCondition::new(automaton_id, SimulateDirection::Forward);
        action.do_action(&mut runtime, &mut edge_queue, &mut vm);
        let conditions = runtime.automaton.pending_detection_conditions.get(&automaton_id);
        assert!(conditions.is_some(), "do_action 后应按 automaton_id 填充待决检测条件");
        let conditions = conditions.unwrap();
        assert!(!conditions.is_empty(), "Forward 方向应产生至少一条检测条件");
        for c in conditions {
            assert_eq!(c.direction, SimulateDirection::Forward, "存入的条件方向应为动作携带的方向");
            assert_eq!(c.tsiga_id, automaton_id);
            assert!(c.filter_id != 0, "应解析出有效 filter");
        }
        let first_len = conditions.len();

        // 再次执行：应整体覆盖（清空后重建），数量保持一致
        action.do_action(&mut runtime, &mut edge_queue, &mut vm);
        let rebound = runtime.automaton.pending_detection_conditions.get(&automaton_id).unwrap();
        assert_eq!(rebound.len(), first_len);
    }

    /// 回归测试（P2-9）：反向弹栈打通 UpdatePendingDetectionCondition 传播。
    /// HistoryStack.pop_until 返回受影响自动机 ID ObjectArray → SignalTsiga 方法 3
    /// 透传该数组 → backward_simulate 据此追加刷新待决动作。
    #[test]
    fn test_backward_simulate_propagates_update_pending_detection_condition() {
        use crate::signal::multichannel_snapshot::MultichannelSnapshot;
        use crate::system::native::history::HistoryStack;
        use crate::system::native::input_graph::InputGraph;
        use crate::system::native::signal_tsiga::SignalTsiga;
        use crate::system::native::time_stack::TimeStack;
        use gorge_core::objective::native::NativeClass;
        use gorge_core::objective::native::NativeContext;
        use gorge_core::system::native::array::ObjectArrayClass;

        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;

        // 注册各 native 类
        vm.register_native_class("GorgeFramework.HistoryStack".into(),
            Arc::new(HistoryStack { _placeholder: false }));
        vm.register_native_class("GorgeFramework.InputGraph".into(),
            Arc::new(InputGraph { states: 0, input_pointer: 0, accept: false, stack_respond: false, export_state: String::new() }));
        vm.register_native_class("GorgeFramework.TimeStack".into(),
            Arc::new(TimeStack { accept: false, respond_mode: String::new() }));
        vm.register_native_class("GorgeFramework.SignalTsiga".into(),
            Arc::new(SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 }));

        // 空 InputGraph
        let ig = InputGraph { states: 0, input_pointer: 0, accept: false, stack_respond: false, export_state: String::new() };
        let states_arr = { let mut c = NativeContext::new(&mut vm); ObjectArrayClass.do_construct_native(&mut c, None, 0) };
        vm.param_pool.set_object_param(0, states_arr);
        vm.param_pool.set_bool_param(0, false);
        vm.param_pool.set_bool_param(1, false);
        vm.param_pool.set_int_param(0, 0);
        vm.param_pool.set_string_param(0, String::new());
        let ig_id = { let mut c = NativeContext::new(&mut vm); ig.do_construct_native(&mut c, None, 0) };

        // 空 TimeStack
        let ts = TimeStack { accept: false, respond_mode: String::new() };
        vm.param_pool.set_bool_param(0, false);
        vm.param_pool.set_string_param(0, String::new());
        let ts_id = { let mut c = NativeContext::new(&mut vm); ts.do_construct_native(&mut c, None, 0) };

        // HistoryStack，压入一个 TimeStackPop（chart_time=2.0）
        let hs = HistoryStack { _placeholder: false };
        let hs_id = { let mut c = NativeContext::new(&mut vm); hs.do_construct_native(&mut c, None, 0) };
        vm.param_pool.set_float_param(0, 2.0);
        vm.param_pool.set_object_param(0, 100);
        vm.param_pool.set_bool_param(0, false);
        vm.param_pool.set_string_param(0, String::new());
        { let mut c = NativeContext::new(&mut vm); hs.invoke_native_method(&mut c, hs_id, 3); } // push_time_stack_pop

        // SignalTsiga 指向上述三对象
        let st = SignalTsiga { input_graph: 0, time_stack: 0, history_stack: 0 };
        vm.param_pool.set_object_param(0, ig_id);
        vm.param_pool.set_object_param(1, ts_id);
        vm.param_pool.set_object_param(2, hs_id);
        let tsiga_id = { let mut c = NativeContext::new(&mut vm); st.do_construct_native(&mut c, None, 0) };

        // runtime 注册该自动机
        let mut runtime = GorgeSimulationRuntime::new();
        runtime.automaton.automatons.push(tsiga_id);

        // 反向推进到 1.5 < 2.0 → 弹出 TimeStackPop，产生受影响自动机 → 追加刷新待决动作
        let snapshot = MultichannelSnapshot::new();
        let actions = PreciseAutomatonSimulator.backward_simulate(0.0, 1.5, &snapshot, &runtime, &mut vm);
        assert_eq!(actions.len(), 1, "反向弹栈应产生 1 个 UpdatePendingDetectionCondition 动作");

        // 反向推进到 3.0 ≥ 2.0 → 无 TimeStackPop 弹出 → 无动作
        let actions_none = PreciseAutomatonSimulator.backward_simulate(0.0, 3.0, &snapshot, &runtime, &mut vm);
        assert!(actions_none.is_empty(), "无 TimeStackPop 弹出时应无动作");
    }
}

// ==================== P2-8 SongSimulator 播放控制测试 ====================

/// 可控制的音频播放器 mock（P2-8）。
///
/// 与 `HeadlessAudio` 不同，本 mock 允许测试控制 `audio_length`、
/// `is_playing` 的状态，并记录 `set_time`/`play`/`stop` 调用，用于
/// 验证 SongSimulator 的播放窗口判定逻辑。状态经 `Arc<Mutex>` 共享，
/// 使测试可在运行时之外查询/改写播放器状态。
#[cfg(test)]
mod song_simulator_tests {
    use super::*;
    use crate::adaptor::IAudioPlayer;
    use crate::runtime::environment::GorgeSimulationRuntime;
    use crate::signal::multichannel_snapshot::MultichannelSnapshot;
    use gorge_core::virtual_machine::vm::VirtualMachine;
    use std::sync::{Arc, Mutex};

    /// mock 播放器的共享状态（时长、播放开关、调用记录）
    #[derive(Clone)]
    struct MockPlayerState {
        length: f32,
        playing: Arc<Mutex<bool>>,
        calls: Arc<Mutex<Vec<String>>>,
    }

    impl MockPlayerState {
        fn new(length: f32, playing: bool) -> Self {
            Self {
                length,
                playing: Arc::new(Mutex::new(playing)),
                calls: Arc::new(Mutex::new(Vec::new())),
            }
        }

        /// 取调用记录克隆（供断言）
        fn calls(&self) -> Vec<String> {
            self.calls.lock().unwrap().clone()
        }

        /// 清空调用记录
        fn clear_calls(&self) {
            self.calls.lock().unwrap().clear();
        }

        /// 当前播放状态
        fn is_playing(&self) -> bool {
            *self.playing.lock().unwrap()
        }
    }

    /// 平台播放器 mock，实现 `IAudioPlayer`，行为由共享状态驱动
    struct MockAudioPlayer {
        state: MockPlayerState,
    }

    impl IAudioPlayer for MockAudioPlayer {
        fn set_audio(&self, _audio_id: usize) {}
        fn play(&self) {
            *self.state.playing.lock().unwrap() = true;
            self.state.calls.lock().unwrap().push("play".into());
        }
        fn stop(&self) {
            *self.state.playing.lock().unwrap() = false;
            self.state.calls.lock().unwrap().push("stop".into());
        }
        fn audio_length(&self) -> f32 {
            self.state.length
        }
        fn is_playing(&self) -> bool {
            *self.state.playing.lock().unwrap()
        }
        fn set_time(&self, time: f32) {
            self.state.calls.lock().unwrap().push(format!("set_time:{time}"));
        }
        fn destruct(&self) {}
    }

    /// 构造含单个可控乐段音频源的 runtime，窗口为 [start, start+length)。
    ///
    /// - `time_offset`：乐段 Config.timeOffset（对齐 C# `period.Config.timeOffset`）
    /// - `length`：音频时长（对齐 C# `audioPlayer.AudioLength()`）
    /// - `playing`：注入时的初始播放状态
    fn make_runtime(time_offset: f32, length: f32, playing: bool) -> (GorgeSimulationRuntime, MockPlayerState) {
        let mut runtime = GorgeSimulationRuntime::new();
        let state = MockPlayerState::new(length, playing);
        runtime.audio
            .inject_audio_source_for_test(100, time_offset, Box::new(MockAudioPlayer { state: state.clone() }));
        (runtime, state)
    }

    /// (a) 窗口内未播放 → 调用 set_time + play，且返回空动作列表
    #[test]
    fn test_song_simulator_plays_within_window_when_not_playing() {
        // time_offset=2.0，RespondDelay=0 → 窗口 [2.0, 12.0)
        let (runtime, state) = make_runtime(2.0, 10.0, false);
        let snapshot = MultichannelSnapshot::new();
        let mut vm = VirtualMachine::new();

        let actions = SongSimulator.forward_simulate(0.0, 5.0, &snapshot, &runtime, &mut vm);

        assert!(actions.is_empty(), "SongSimulator 不应产生 GameplayAction");
        assert_eq!(state.calls(), vec!["set_time:3", "play"], "窗口内未播放应跳转并播放");
        assert!(state.is_playing(), "play 后应处于播放状态");
    }

    /// (b) 窗口内已播放 → 不重复调用 play/set_time
    #[test]
    fn test_song_simulator_does_not_replay_already_playing_in_window() {
        let (runtime, state) = make_runtime(2.0, 10.0, true);
        let snapshot = MultichannelSnapshot::new();
        let mut vm = VirtualMachine::new();

        SongSimulator.forward_simulate(0.0, 5.0, &snapshot, &runtime, &mut vm);

        assert!(state.calls().is_empty(), "窗口内已播放时不应重复调用 set_time/play");
        assert!(state.is_playing());
    }

    /// (c) 窗口外已播放 → 调用 stop
    #[test]
    fn test_song_simulator_stops_playing_outside_window() {
        // 窗口 [2.0, 12.0)，chart_to=13.0 在窗口外
        let (runtime, state) = make_runtime(2.0, 10.0, true);
        let snapshot = MultichannelSnapshot::new();
        let mut vm = VirtualMachine::new();

        let actions = SongSimulator.forward_simulate(0.0, 13.0, &snapshot, &runtime, &mut vm);

        assert!(actions.is_empty());
        assert_eq!(state.calls(), vec!["stop"], "窗口外正在播放应停止");
        assert!(!state.is_playing(), "stop 后应停止播放");
    }

    /// 边界与窗口外未播放：窗口外未播放不调用任何接口；窗口右端为开区间。
    #[test]
    fn test_song_simulator_window_boundaries() {
        // 窗口 [2.0, 12.0)
        let (runtime, state) = make_runtime(2.0, 10.0, false);
        let snapshot = MultichannelSnapshot::new();
        let mut vm = VirtualMachine::new();

        // 恰在窗口起始点（含）→ 播放
        SongSimulator.forward_simulate(0.0, 2.0, &snapshot, &runtime, &mut vm);
        assert_eq!(state.calls(), vec!["set_time:0", "play"], "窗口左端点含，应播放");
        state.clear_calls();

        // 恰在窗口结束点（开区间右端，不含），且上一步已播放 → 应停止
        SongSimulator.forward_simulate(0.0, 12.0, &snapshot, &runtime, &mut vm);
        assert_eq!(state.calls(), vec!["stop"], "窗口右端点不含且正在播放时应停止");
        assert!(!state.is_playing());
    }

    /// 多乐段：窗口内乐段播放、窗口外乐段停止，互不影响
    #[test]
    fn test_song_simulator_multiple_periods_independent() {
        let mut runtime = GorgeSimulationRuntime::new();
        // 乐段 A：time_offset=0，length=5 → 窗口 [0, 5)
        let state_a = MockPlayerState::new(5.0, false);
        // 乐段 B：time_offset=10，length=5 → 窗口 [10, 15)，初始在播放
        let state_b = MockPlayerState::new(5.0, true);
        runtime.audio
            .inject_audio_source_for_test(100, 0.0, Box::new(MockAudioPlayer { state: state_a.clone() }));
        runtime.audio
            .inject_audio_source_for_test(101, 10.0, Box::new(MockAudioPlayer { state: state_b.clone() }));
        let snapshot = MultichannelSnapshot::new();
        let mut vm = VirtualMachine::new();

        // chart_to=3：A 在窗口内（未播放→播放），B 在窗口外（播放→停止）
        SongSimulator.forward_simulate(0.0, 3.0, &snapshot, &runtime, &mut vm);

        assert_eq!(state_a.calls(), vec!["set_time:3", "play"], "A 窗口内应播放");
        assert!(state_a.is_playing());
        assert_eq!(state_b.calls(), vec!["stop"], "B 窗口外应停止");
        assert!(!state_b.is_playing());
    }

    /// 空音频源表：不 panic，返回空动作列表
    #[test]
    fn test_song_simulator_empty_sources() {
        let runtime = GorgeSimulationRuntime::new();
        let snapshot = MultichannelSnapshot::new();
        let mut vm = VirtualMachine::new();

        let actions = SongSimulator.forward_simulate(0.0, 5.0, &snapshot, &runtime, &mut vm);
        assert!(actions.is_empty());
    }
}
