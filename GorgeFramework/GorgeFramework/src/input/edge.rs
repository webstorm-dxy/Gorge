//! 信号边沿（对应 C# `Input/Edge.cs`）。
//!
//! 描述某个信号在某个时刻的突变值。纯数据载体，无算法逻辑。

/// 信号边沿：时间 + 突变值
///
/// `TSignal` 为信号值类型，框架使用 `usize`（GorgeObject ID）。
#[derive(Debug, Clone, PartialEq)]
pub struct Edge<TSignal> {
    /// 边沿发生的模拟时间
    pub time: f32,
    /// 变化后的信号值
    pub value: TSignal,
}

impl<TSignal> Edge<TSignal> {
    /// 创建一个新的信号边沿
    pub fn new(time: f32, value: TSignal) -> Self {
        Self { time, value }
    }
}
