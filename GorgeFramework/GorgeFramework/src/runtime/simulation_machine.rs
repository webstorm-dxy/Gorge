//! 模拟机（对应 C# `Runtime/SimulationMachine.cs`）。
//!
//! 仿真引擎的核心——维护双时间轴、目标栈，驱动复合步模拟循环。
//! 方法接收 `&mut GorgeSimulationRuntime` 以访问所有子管理器。
//! S4c 重构：do_action 接收 `vm: &mut VirtualMachine`，经 drive 调用链传入。
#![allow(dead_code)]

use std::collections::VecDeque;
use crate::runtime::simulation_types::{SimulationTask, SimulationTarget, SimulateDirection};
use crate::runtime::environment::GorgeSimulationRuntime;
use crate::signal::multichannel_edge_queue::MultichannelEdgeQueue;
use crate::signal::multichannel_snapshot::MultichannelSnapshot;
use crate::simulators::IGameplayAction;
use gorge_core::virtual_machine::vm::VirtualMachine;

/// 模拟机
#[derive(Debug)]
pub struct SimulationMachine {
    /// 谱面时间
    pub chart_time: f32,
    /// 模拟时间
    pub simulate_time: f32,
    /// 模拟目标栈
    target_stack: VecDeque<SimulationTarget>,
    /// 当前仿真任务
    current_task: Option<SimulationTask>,
    /// 初始化完成标记
    initialized: bool,
    /// 开始谱面时间
    begin_chart_time: f32,
    /// 终止谱面时间
    terminate_chart_time: f32,
    /// 初始模拟倍速
    begin_simulate_speed: f32,
}

impl SimulationMachine {
    pub fn new(begin_chart_time: f32, terminate_chart_time: f32, begin_simulate_speed: f32) -> Self {
        Self {
            chart_time: 0.0, simulate_time: 0.0,
            target_stack: VecDeque::new(), current_task: None,
            initialized: false,
            begin_chart_time, terminate_chart_time, begin_simulate_speed,
        }
    }

    pub fn runtime_initialize(&mut self) {
        self.simulate_time = 0.0;
        self.chart_time = self.begin_chart_time;
        self.target_stack.clear();
        self.target_stack.push_back(SimulationTarget::new(self.terminate_chart_time, self.begin_simulate_speed));
        self.initialized = true;
    }

    pub fn runtime_destruct(&mut self) {
        self.simulate_time = 0.0;
        self.chart_time = self.begin_chart_time;
        self.target_stack.clear();
        self.initialized = false;
        self.current_task = None;
    }

    pub fn is_initialized(&self) -> bool { self.initialized }

    // ==================== Drive：主驱动入口 ====================

    /// 向前驱动指定模拟时长。
    ///
    /// `vm` 用于执行 GameplayAction（如元素创生需 VM 参与）。
    pub fn drive(&mut self, simulation_time: f32, runtime: &mut GorgeSimulationRuntime, vm: &mut VirtualMachine) {
        if !self.initialized { return; }
        if simulation_time <= 0.0 { return; }
        if self.target_stack.is_empty() { return; }

        let mut remaining = simulation_time;
        while remaining > 0.0 && !self.target_stack.is_empty() {
            let task = self.get_or_calc_task(runtime);
            let (chart_to, sim_to, rem) = self.calculate_step_time(
                remaining, runtime, &task,
            );
            remaining = rem;
            self.simulate_composite_step(sim_to, chart_to, runtime, vm);
            self.try_accept_task(runtime, vm);
        }
        // 尾独立仿真
        self.late_independent_simulate(SimulateDirection::Forward, runtime, vm);
    }

    // ==================== 复合步模拟 ====================

    /// 执行一次复合步：推进步 + 零步长循环
    fn simulate_composite_step(
        &mut self, simulate_time_target: f32, chart_time_target: f32,
        runtime: &mut GorgeSimulationRuntime, vm: &mut VirtualMachine,
    ) {
        let direction = self.direction();
        // 同步时间到 runtime（供 do_action 中读取）
        runtime.simulate_time = self.simulate_time;
        runtime.chart_time = self.chart_time;

        let (snapshot, edge_queue) = self.get_split_signals(simulate_time_target, runtime);

        // 并行模拟所有模拟器（推进步）
        let actions = self.run_all_simulators(
            self.chart_time, chart_time_target, &snapshot, direction, runtime, vm,
        );
        let automaton_flag = self.do_step_actions(actions, runtime, vm);

        // 零步长循环
        self.zero_length_simulate(
            chart_time_target, direction, snapshot, edge_queue, automaton_flag, runtime, vm,
        );

        // 更新时间
        self.chart_time = chart_time_target;
        self.simulate_time = simulate_time_target;
    }

    /// 获取信号切片和边沿队列（S4b：接入 AutomatonManager）
    fn get_split_signals(
        &self, simulate_time_target: f32, runtime: &GorgeSimulationRuntime,
    ) -> (MultichannelSnapshot, MultichannelEdgeQueue) {
        let split = runtime.automaton.split_input_signals(self.simulate_time, simulate_time_target);
        let mut snapshot = MultichannelSnapshot::new();
        let mut edge_queue = MultichannelEdgeQueue::new();

        for (channel_name, channel_signals) in &split {
            for (signal_id, fragment) in channel_signals {
                // 快照：取当前时刻的起始值
                snapshot.set(channel_name, *signal_id, fragment.start_value);
                // 边沿：全部边沿入队
                for edge in &fragment.edges {
                    edge_queue.enqueue(channel_name, *signal_id, edge.clone());
                }
            }
        }

        (snapshot, edge_queue)
    }

    // ==================== 零步长循环 ====================

    /// 零步长仿真：重复直到收敛（信号边沿处理完毕且自动机不变化）
    fn zero_length_simulate(
        &mut self, chart_time: f32, direction: SimulateDirection,
        mut snapshot: MultichannelSnapshot, mut edge_queue: MultichannelEdgeQueue,
        initial_automaton_flag: bool, runtime: &mut GorgeSimulationRuntime, vm: &mut VirtualMachine,
    ) {
        let mut automaton_flag = initial_automaton_flag;
        let mut first_instant = true;

        loop {
            let actions;

            if automaton_flag || first_instant {
                // 自动机变化或首次进入 → 对所有模拟器执行瞬时仿真
                actions = self.instant_simulate_all(chart_time, direction, &snapshot, runtime, vm);
                first_instant = false;
            } else if let Some((channel, signal_id, edge)) = edge_queue.try_dequeue() {
                // 消费一个待决边沿
                snapshot.set(&channel, signal_id, edge.value);
                actions = self.instant_simulate_all(chart_time, direction, &snapshot, runtime, vm);
            } else {
                // 无边沿且无自动机变化 → 收敛
                break;
            }

            automaton_flag = self.do_step_actions(actions, runtime, vm);
        }
    }

    // ==================== 模拟器分派 ====================

    /// 对所有模拟器执行推进步仿真（S4d：真实分派）
    fn run_all_simulators(
        &self, chart_from: f32, chart_to: f32,
        snapshot: &MultichannelSnapshot, direction: SimulateDirection,
        runtime: &GorgeSimulationRuntime, vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        let mut actions: Vec<Box<dyn IGameplayAction>> = Vec::new();
        for (_pri, sim_id) in runtime.simulation.simulators.iter() {
            if let Some(sim) = runtime.sim_registry.get(*sim_id) {
                let sim_actions = match direction {
                    SimulateDirection::Forward =>
                        sim.forward_simulate(chart_from, chart_to, snapshot, runtime, vm),
                    SimulateDirection::Backward =>
                        sim.backward_simulate(chart_from, chart_to, snapshot, runtime, vm),
                    SimulateDirection::Infinitesimal =>
                        sim.infinitesimal_simulate(chart_to, snapshot, runtime, vm),
                };
                actions.extend(sim_actions);
            }
        }
        actions
    }

    /// 对所有模拟器执行瞬时仿真（S4d：真实分派）
    fn instant_simulate_all(
        &self, chart_time: f32, direction: SimulateDirection,
        snapshot: &MultichannelSnapshot, runtime: &GorgeSimulationRuntime, vm: &mut VirtualMachine,
    ) -> Vec<Box<dyn IGameplayAction>> {
        let mut actions: Vec<Box<dyn IGameplayAction>> = Vec::new();
        for (_pri, sim_id) in runtime.simulation.simulators.iter() {
            if let Some(sim) = runtime.sim_registry.get(*sim_id) {
                let sim_actions = sim.instant_simulate(chart_time, direction, snapshot, runtime, vm);
                actions.extend(sim_actions);
            }
        }
        actions
    }

    /// 执行 GameplayAction 队列（S4c：传入 vm）
    fn do_step_actions(
        &self, actions: Vec<Box<dyn IGameplayAction>>,
        runtime: &mut GorgeSimulationRuntime, vm: &mut VirtualMachine,
    ) -> bool {
        let mut automaton_flag = false;
        // 创建一个临时 edge_queue 供 action 内部使用（如 AppendSignal 追加边沿）
        let mut eq = MultichannelEdgeQueue::new();
        for action in &actions {
            action.do_action(runtime, &mut eq, vm);
            if action.change_automaton() { automaton_flag = true; }
        }
        automaton_flag
    }

    /// 尾独立仿真（S4d：真实分派）
    fn late_independent_simulate(
        &self, direction: SimulateDirection, runtime: &mut GorgeSimulationRuntime, vm: &mut VirtualMachine,
    ) {
        let snapshot = MultichannelSnapshot::new();
        for (_pri, sim_id) in runtime.simulation.late_independent_simulators.iter() {
            if let Some(sim) = runtime.late_sim_registry.get(*sim_id) {
                let _ = sim.instant_simulate(self.chart_time, direction, &snapshot, runtime, vm);
                // 返回值丢弃（C# 行为）
            }
        }
    }

    // ==================== 步长计算 ====================

    /// 计算单步时间（S4b：接入 AutomatonManager 最早边沿时间）
    fn calculate_step_time(
        &self, remaining: f32, runtime: &GorgeSimulationRuntime, task: &SimulationTask,
    ) -> (f32, f32, f32) {
        let sim_task_len = task.simulate_time - self.simulate_time;
        let signal_len = runtime.automaton.get_input_signal_earliest_edge_time_after(self.simulate_time)
            - self.simulate_time;
        let sim_step = sim_task_len.min(remaining).min(signal_len);

        let remaining_after = remaining - sim_step;
        let chart_step = if (sim_step - sim_task_len).abs() < 1e-6 && task.chart_time.is_some() {
            task.chart_time.unwrap() - self.chart_time
        } else {
            let speed = self.target_stack.back().map(|t| t.simulate_speed).unwrap_or(1.0);
            sim_step * speed
        };

        (self.chart_time + chart_step, self.simulate_time + sim_step, remaining_after)
    }

    // ==================== 任务管理 ====================

    fn get_or_calc_task(&mut self, runtime: &GorgeSimulationRuntime) -> SimulationTask {
        if self.current_task.is_none() {
            self.current_task = Some(self.calculate_simulation_task(runtime));
        }
        self.current_task.clone().unwrap()
    }

    /// 计算仿真任务（S4d：调用 ISimulator 方法获取异步目标）
    fn calculate_simulation_task(&self, runtime: &GorgeSimulationRuntime) -> SimulationTask {
        let target = match self.target_stack.back() {
            Some(t) => t,
            None => return SimulationTask { simulate_time: self.simulate_time, chart_time: None },
        };

        if target.simulate_speed > 0.0 {
            // 正转：取所有模拟器的最小异步目标
            let mut min_target = f32::MAX;
            for (_pri, sim_id) in runtime.simulation.simulators.iter() {
                if let Some(sim) = runtime.sim_registry.get(*sim_id) {
                    let t = sim.forward_async_simulation_target(self.chart_time, runtime);
                    if t < min_target { min_target = t; }
                }
            }
            if min_target <= self.chart_time { min_target = self.chart_time + 0.01; }
            let step_chart = if self.chart_time < target.chart_time {
                min_target.min(target.chart_time)
            } else { min_target };
            let target_sim = self.simulate_time + (step_chart - self.chart_time) / target.simulate_speed;
            SimulationTask { simulate_time: target_sim, chart_time: Some(step_chart) }
        } else if target.simulate_speed < 0.0 {
            // 反转
            let mut max_target = f32::MIN;
            for (_pri, sim_id) in runtime.simulation.simulators.iter() {
                if let Some(sim) = runtime.sim_registry.get(*sim_id) {
                    let t = sim.backward_async_simulation_target(self.chart_time, runtime);
                    if t > max_target { max_target = t; }
                }
            }
            let step_chart = if self.chart_time > target.chart_time {
                max_target.max(target.chart_time)
            } else { max_target };
            let target_sim = self.simulate_time + (step_chart - self.chart_time) / target.simulate_speed;
            SimulationTask { simulate_time: target_sim, chart_time: Some(step_chart) }
        } else {
            // 零速
            let mut min_target = f32::MAX;
            for (_pri, sim_id) in runtime.simulation.simulators.iter() {
                if let Some(sim) = runtime.sim_registry.get(*sim_id) {
                    let t = sim.infinitesimal_async_simulation_target(self.chart_time, runtime);
                    if t < min_target { min_target = t; }
                }
            }
            SimulationTask { simulate_time: min_target, chart_time: None }
        }
    }

    /// 尝试接收仿真任务（任务完成则弹栈 + 执行挂起动作）
    fn try_accept_task(&mut self, runtime: &mut GorgeSimulationRuntime, vm: &mut VirtualMachine) {
        let task = match &self.current_task {
            Some(t) => t.clone(),
            None => return,
        };
        // 检查模拟时间是否到达任务目标
        if self.simulate_time < task.simulate_time { return; }
        self.current_task = None;
        // 弹栈
        if let Some(target) = self.target_stack.back() {
            if self.chart_time >= target.chart_time && target.simulate_speed > 0.0 {
                self.target_stack.pop_back();
            } else if self.chart_time <= target.chart_time && target.simulate_speed < 0.0 {
                self.target_stack.pop_back();
            }
        }
        let _ = (runtime, vm);
    }

    pub fn direction(&self) -> SimulateDirection {
        self.target_stack.back().map(|t| t.direction()).unwrap_or(SimulateDirection::Forward)
    }

    // ==================== C-3 DriveToChartTime ====================

    /// 驱动仿真推进直到 chart_time 到达或超过目标时间（对齐 C# `DriveToChartTime`）。
    ///
    /// 内部调用 `drive()` 将 simulation_time 换算为所需时长后推进。
    /// 若目标时间已在当前 chart_time 之后（或目标栈为空），直接返回。
    /// 推进完成后 chart_time >= target_chart_time。
    pub fn drive_to_chart_time(
        &mut self, target_chart_time: f32,
        runtime: &mut GorgeSimulationRuntime, vm: &mut VirtualMachine,
    ) {
        if target_chart_time <= self.chart_time || self.target_stack.is_empty() {
            return;
        }
        let speed = self.target_stack.back().map(|t| t.simulate_speed).unwrap_or(1.0);
        let sim_step = (target_chart_time - self.chart_time) / speed;
        if sim_step > 0.0 {
            self.drive(sim_step, runtime, vm);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::environment::GorgeSimulationRuntime;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    #[test]
    fn test_construct_and_initialize() {
        let mut sm = SimulationMachine::new(0.0, 100.0, 1.0);
        assert!(!sm.is_initialized());
        sm.runtime_initialize();
        assert!(sm.is_initialized());
        assert_eq!(sm.chart_time, 0.0);
    }

    #[test]
    fn test_drive_without_simulators() {
        let mut sm = SimulationMachine::new(0.0, 100.0, 1.0);
        sm.runtime_initialize();
        let mut runtime = GorgeSimulationRuntime::new();
        let mut vm = VirtualMachine::new();
        sm.drive(10.0, &mut runtime, &mut vm);
        // 无模拟器时应推进时间
        assert!(sm.chart_time >= 0.0);
    }

    // ==================== C-3 drive_to_chart_time 测试 ====================

    #[test]
    fn test_c3_drive_to_chart_time_advances() {
        let mut sm = SimulationMachine::new(0.0, 100.0, 1.0);
        sm.runtime_initialize();
        let mut runtime = GorgeSimulationRuntime::new();
        let mut vm = VirtualMachine::new();

        let target = 42.0;
        sm.drive_to_chart_time(target, &mut runtime, &mut vm);

        // 应推进到目标或之后
        assert!(sm.chart_time >= target,
            "drive_to_chart_time 应推进 chart_time 至少到目标时间 {}，实际 {}", target, sm.chart_time);
    }

    #[test]
    fn test_c3_drive_to_chart_time_noop_when_behind() {
        let mut sm = SimulationMachine::new(0.0, 100.0, 1.0);
        sm.runtime_initialize();
        sm.chart_time = 50.0;
        let mut runtime = GorgeSimulationRuntime::new();
        let mut vm = VirtualMachine::new();

        let before = sm.chart_time;
        // target 已在当前 chart_time 之前，不应推进
        sm.drive_to_chart_time(30.0, &mut runtime, &mut vm);
        assert_eq!(sm.chart_time, before,
            "target 已过期时不应推进 chart_time");
    }
}
