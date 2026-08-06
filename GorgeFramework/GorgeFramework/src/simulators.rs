//! 仿真器模块（对应 C# `Simulators/` 文件夹）。
//!
//! 定义 ISimulator 和 IGameplayAction 核心 trait。
//! ISimulator 方法接收 `&GorgeSimulationRuntime` + `&mut VirtualMachine`，
//! 以支持原生类模拟器（如 ElementSimulator）通过 VM 访问对象方法。
//! IGameplayAction.do_action 接收 `&mut GorgeSimulationRuntime` + `&mut VirtualMachine`
//! 以实现元素创生、注解扫描等需要 VM 的操作。

use crate::runtime::simulation_types::SimulateDirection;
use crate::runtime::environment::GorgeSimulationRuntime;
use crate::signal::multichannel_edge_queue::MultichannelEdgeQueue;
use crate::signal::multichannel_snapshot::MultichannelSnapshot;
use gorge_core::virtual_machine::vm::VirtualMachine;

pub mod impls;

/// 信号检测条件（S7 数据化结构体）
///
/// 对齐 C# `SignalDetectionCondition`。存储过滤器/自动机对象 ID 和模拟方向，
/// 供 PreciseAutomatonSimulator 调用点解释执行。
#[derive(Debug, Clone)]
pub struct SignalDetectionCondition {
    /// 优先级对象 ID 列表（filter.priority 委托 invoke 返回 ObjectArray 展开）
    pub priority_items: Vec<usize>,
    /// 所属自动机（SignalTsiga）对象 ID
    pub tsiga_id: usize,
    /// 信号过滤器对象 ID
    pub filter_id: usize,
    /// 模拟方向
    pub direction: SimulateDirection,
}

/// 可模拟对象（对应 C# `ISimulator`）
///
/// 定义了仿真器在四种模拟方向下的异步目标计算方法
/// 和四种模拟执行方法。各方法接收运行时与信号快照。
/// S7 扩展：增加 `vm: &mut VirtualMachine` 参数以支持 ElementSimulator 等原生类模拟器。
pub trait ISimulator: Send + Sync {
    /// 计算前向异步模拟的目标谱面时间
    fn forward_async_simulation_target(&self, chart_time: f32, runtime: &GorgeSimulationRuntime, vm: &mut VirtualMachine) -> f32;

    /// 计算后向异步模拟的目标谱面时间
    fn backward_async_simulation_target(&self, chart_time: f32, runtime: &GorgeSimulationRuntime, vm: &mut VirtualMachine) -> f32;

    /// 计算零速异步模拟的目标模拟时间
    fn infinitesimal_async_simulation_target(&self, chart_time: f32, runtime: &GorgeSimulationRuntime, vm: &mut VirtualMachine) -> f32;

    /// 前向模拟：从 chart_time_from 到 chart_time_to
    fn forward_simulate(
        &self,
        chart_time_from: f32,
        chart_time_to: f32,
        signal_snapshot: &MultichannelSnapshot,
        runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>>;

    /// 后向模拟：从 chart_time_from 到 chart_time_to
    fn backward_simulate(
        &self,
        chart_time_from: f32,
        chart_time_to: f32,
        signal_snapshot: &MultichannelSnapshot,
        runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>>;

    /// 零速模拟（谱面时间不变，仅模拟时间推进）
    fn infinitesimal_simulate(
        &self,
        chart_time: f32,
        signal_snapshot: &MultichannelSnapshot,
        runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>>;

    /// 零步长模拟（模拟时间和谱面时间均不变，仅非时间状态变化）
    fn instant_simulate(
        &self,
        chart_time: f32,
        direction: SimulateDirection,
        signal_snapshot: &MultichannelSnapshot,
        runtime: &GorgeSimulationRuntime,
        vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>>;
}

/// Gameplay 控制动作（对应 C# `IGameplayAction`）
///
/// 仿真器在每次模拟步中产出的动作序列。每个动作在 `do_action` 中
/// 修改运行时状态（创建/销毁元素、追加信号边沿、更新自动机等）。
/// S4c 重构：增加 `runtime: &mut GorgeSimulationRuntime` + `vm: &mut VirtualMachine` 参数。
pub trait IGameplayAction: Send + Sync {
    /// 执行动作
    ///
    /// `vm` 用于元素创生（instantiate_with_injector）、注解扫描（class_methods_with_annotation）
    /// 和方法调用（invoke_method_by_id）等需要 VM 的操作。
    fn do_action(
        &self,
        runtime: &mut GorgeSimulationRuntime,
        edge_queue: &mut MultichannelEdgeQueue,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    );

    /// 该动作是否会触发自动机状态变换
    fn change_automaton(&self) -> bool { false }

    /// 该动作是否触发信号变换
    fn change_signal(&self) -> bool { false }
}
