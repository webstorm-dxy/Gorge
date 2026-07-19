//! 单通道信号边沿队列（对应 C# `Signal/ChannelEdgeQueue.cs`）。
//!
//! 每个信号 ID 对应一个 FIFO 边沿队列，支持入队和出队操作。

use std::collections::{HashMap, VecDeque};
use super::edge::Edge;

/// 单通道边沿队列：signalId → FIFO 边沿队列
#[derive(Debug, Clone)]
pub struct ChannelEdgeQueue {
    queues: HashMap<i32, VecDeque<Edge<usize>>>,
}

impl ChannelEdgeQueue {
    /// 创建空队列
    pub fn new() -> Self {
        Self { queues: HashMap::new() }
    }

    /// 总边沿数
    pub fn edge_count(&self) -> usize {
        self.queues.values().map(|q| q.len()).sum()
    }

    /// 入队一个边沿
    pub fn enqueue(&mut self, signal_id: i32, edge: Edge<usize>) {
        self.queues.entry(signal_id).or_default().push_back(edge);
    }

    /// 出队一个边沿，返回 (signalId, edge)
    pub fn try_dequeue(&mut self) -> Option<(i32, Edge<usize>)> {
        if self.edge_count() == 0 { return None; }
        for (&sig_id, queue) in self.queues.iter_mut() {
            if let Some(edge) = queue.pop_front() {
                return Some((sig_id, edge));
            }
        }
        None
    }
}

impl Default for ChannelEdgeQueue {
    fn default() -> Self { Self::new() }
}
