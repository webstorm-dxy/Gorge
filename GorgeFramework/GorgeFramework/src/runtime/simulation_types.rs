//! 仿真数据类型（对应 C# `Runtime/SimulateDirection.cs` + `SimulationTask.cs` + `SimulationTarget.cs`）。

/// 模拟方向（对应 C# `SimulateDirection`）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulateDirection {
    /// 正向模拟
    Forward,
    /// 反向模拟
    Backward,
    /// 零速模拟（静止帧）
    Infinitesimal,
}

impl SimulateDirection {
    /// 从速度值推导方向
    pub fn from_speed(speed: f32) -> Self {
        if speed > 0.0 { SimulateDirection::Forward }
        else if speed < 0.0 { SimulateDirection::Backward }
        else { SimulateDirection::Infinitesimal }
    }
}

/// 模拟目标（对应 C# `SimulationTarget`）
///
/// 驱动 SimulationMachine 推进到指定谱面时间的指令。
#[derive(Debug, Clone)]
pub struct SimulationTarget {
    /// 目标谱面时间
    pub chart_time: f32,
    /// 模拟倍速（正值=正转/负值=反转/零=零速）
    pub simulate_speed: f32,
}

impl SimulationTarget {
    pub fn new(chart_time: f32, simulate_speed: f32) -> Self {
        Self { chart_time, simulate_speed }
    }

    /// 从速度推导方向
    pub fn direction(&self) -> SimulateDirection {
        SimulateDirection::from_speed(self.simulate_speed)
    }
}

/// 仿真任务（对应 C# `SimulationTask`）
///
/// 在 SimulationMachine 中，一个仿真任务代表"推进到某一模拟时间，
/// 同时若指定了 chart_time 则对齐谱面时间"。
/// `pending_actions` 在完成任务后顺次执行。
#[derive(Debug, Clone)]
pub struct SimulationTask {
    /// 目标模拟时间
    pub simulate_time: f32,
    /// 对应谱面时间（可选），用于依赖谱面时间的模拟器
    pub chart_time: Option<f32>,
}
