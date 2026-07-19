//! 多通道信号快照（对应 C# `Signal/MultichannelSnapshot.cs`）。
//!
//! 某一时刻所有频道的全部信号瞬时值。

use std::collections::HashMap;

/// 多通道信号快照：频道名 → 单通道快照
#[derive(Debug, Clone)]
pub struct MultichannelSnapshot {
    pub channels: HashMap<String, super::channel_snapshot::ChannelSnapshot>,
}

impl MultichannelSnapshot {
    pub fn new() -> Self {
        Self { channels: HashMap::new() }
    }

    /// 设置指定频道下某个信号的值
    pub fn set(&mut self, channel_name: &str, signal_id: i32, value: usize) {
        self.channels
            .entry(channel_name.to_string())
            .or_default()
            .insert(signal_id, value);
    }

    /// 获取指定频道下某个信号的值
    pub fn get(&self, channel_name: &str, signal_id: i32) -> Option<usize> {
        self.channels.get(channel_name).and_then(|c| c.get(&signal_id).copied())
    }
}

impl Default for MultichannelSnapshot {
    fn default() -> Self { Self::new() }
}
