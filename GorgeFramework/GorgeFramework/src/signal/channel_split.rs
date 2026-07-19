//! 单通道信号切片（对应 C# `Signal/ChannelSplit.cs`）。
//!
//! 一段时间内某通道的全部信号片段（signalId → Fragment）。

use std::collections::HashMap;
use crate::input::fragment::Fragment;

/// 单通道信号切片
pub type ChannelSplit = HashMap<i32, Fragment<usize>>;
