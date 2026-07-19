//! 多通道信号边沿队列（对应 C# `Signal/MultichannelEdgeQueue.cs`）。
//!
//! 按频道名组织多个单通道边沿队列。

use std::collections::HashMap;
use crate::input::edge::Edge;
use super::channel_edge_queue::ChannelEdgeQueue;

/// 多通道边沿队列：频道名 → 单通道队列
#[derive(Debug, Clone)]
pub struct MultichannelEdgeQueue {
    channels: HashMap<String, ChannelEdgeQueue>,
}

impl MultichannelEdgeQueue {
    pub fn new() -> Self {
        Self { channels: HashMap::new() }
    }

    /// 总边沿数
    pub fn edge_count(&self) -> usize {
        self.channels.values().map(|c| c.edge_count()).sum()
    }

    /// 入队
    pub fn enqueue(&mut self, channel_name: &str, signal_id: i32, edge: Edge<usize>) {
        self.channels.entry(channel_name.to_string()).or_default().enqueue(signal_id, edge);
    }

    /// 出队，返回 (channelName, signalId, edge)
    pub fn try_dequeue(&mut self) -> Option<(String, i32, Edge<usize>)> {
        for (name, queue) in self.channels.iter_mut() {
            if let Some((sig_id, edge)) = queue.try_dequeue() {
                return Some((name.clone(), sig_id, edge));
            }
        }
        None
    }
}

impl Default for MultichannelEdgeQueue {
    fn default() -> Self { Self::new() }
}
