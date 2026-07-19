//! 多通道信号切片（对应 C# `Signal/MultichannelSplit.cs`）。
//!
//! 一段时间内所有频道的全部信号片段。

use std::collections::HashMap;
use super::channel_split::ChannelSplit;

/// 多通道信号切片：频道名 → 单通道切片
pub type MultichannelSplit = HashMap<String, ChannelSplit>;
