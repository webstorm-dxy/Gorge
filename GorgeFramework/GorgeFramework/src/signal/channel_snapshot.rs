//! 单通道信号快照（对应 C# `Signal/ChannelSnapshot.cs`）。
//!
//! 某一时刻所有信号的瞬时值映射（signalId → 信号值）。

use std::collections::HashMap;

/// 单通道信号快照
pub type ChannelSnapshot = HashMap<i32, usize>;
