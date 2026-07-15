//! `GorgeFramework` — 历史栈系统。
//!
//! 移植自 C# 参考实现 `HistoryStack`。用于正向/反向模拟时
//! 记录和还原自动机状态机的历史状态。

use crate::time::{TimeItem, TimeStack};
use crate::input_graph::InputGraph;

/// 历史记录项 trait
pub trait HistoryItem: std::fmt::Debug {
    /// 记录的谱面时间
    fn chart_time(&self) -> f32;
    /// 还原状态到指定输入图和时间栈
    fn revert(&self, graph: &mut InputGraph, time_stack: &mut TimeStack);
}

/// 历史栈
///
/// 对齐 C# `HistoryStack`。维护一个历史记录栈，
/// 支持 push 记录和 pop 到指定时间点。
/// 在反向模拟时用于还原自动机状态。
#[derive(Debug)]
pub struct HistoryStack {
    stack: Vec<Box<dyn HistoryItem>>,
}

impl HistoryStack {
    pub fn new() -> Self { Self { stack: Vec::new() } }

    /// 栈深度
    pub fn len(&self) -> usize { self.stack.len() }
    pub fn is_empty(&self) -> bool { self.stack.is_empty() }

    /// 推入历史记录
    pub fn push(&mut self, item: Box<dyn HistoryItem>) {
        self.stack.push(item);
    }

    /// 弹出直到指定时间之后的所有记录
    pub fn pop_until(&mut self, target_time: f32) -> Vec<Box<dyn HistoryItem>> {
        let split_idx = self.stack.iter()
            .rposition(|item| item.chart_time() < target_time)
            .map(|i| i + 1)
            .unwrap_or(0);
        let removed: Vec<_> = self.stack.drain(split_idx..).collect();
        removed
    }

    /// 查看栈顶
    pub fn peek(&self) -> Option<&Box<dyn HistoryItem>> { self.stack.last() }

    /// 清空
    pub fn clear(&mut self) { self.stack.clear(); }
}

impl Default for HistoryStack {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestItem { time: f32 }
    impl HistoryItem for TestItem {
        fn chart_time(&self) -> f32 { self.time }
        fn revert(&self, _graph: &mut InputGraph, _ts: &mut TimeStack) {}
    }

    #[test]
    fn test_push_and_peek() {
        let mut hs = HistoryStack::new();
        hs.push(Box::new(TestItem { time: 1.0 }));
        hs.push(Box::new(TestItem { time: 2.0 }));
        assert_eq!(hs.len(), 2);
        assert_eq!(hs.peek().unwrap().chart_time(), 2.0);
    }

    #[test]
    fn test_pop_until() {
        let mut hs = HistoryStack::new();
        hs.push(Box::new(TestItem { time: 1.0 }));
        hs.push(Box::new(TestItem { time: 2.0 }));
        hs.push(Box::new(TestItem { time: 3.0 }));
        let removed = hs.pop_until(2.0);
        assert_eq!(removed.len(), 2); // 弹出 time>=2.0 的，即 2.0 和 3.0
        assert_eq!(hs.len(), 1);
    }
}
