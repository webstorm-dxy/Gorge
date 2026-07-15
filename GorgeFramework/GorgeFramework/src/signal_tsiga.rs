//! `GorgeFramework` — 信号时序栈自动机（TSIGA）。
//!
//! 移植自 C# 参考实现 `SignalTsiga.cs`。TSIGA = Time Stack Input Graph Automaton，
//! 是框架核心判定引擎，组合 InputGraph（输入状态图）和 TimeStack（时序栈）
//! 实现基于信号的状态转移、弹栈响应和停机逻辑。

use std::collections::HashMap;
use crate::input_graph::InputGraph;
use crate::time::TimeStack;
use crate::history::HistoryStack;

/// 信号检测条件
///
/// 对齐 C# `SignalDetectionCondition`。每个条件关联一个信号过滤器，
/// 提供检测谓词和接受/拒绝回调。
pub struct SignalDetectionCondition {
    /// 能否检测指定信道
    pub can_detect: Box<dyn Fn(&str) -> bool + Send + Sync>,
    /// 检测信号值，返回 (是否接受, 是否消耗)
    pub detect: Box<dyn Fn(&str, i32, f32) -> (bool, bool) + Send + Sync>,
    /// 接受时的回调
    pub accept: Box<dyn Fn() + Send + Sync>,
    /// 拒绝时的回调
    pub deny: Box<dyn Fn() + Send + Sync>,
}

impl std::fmt::Debug for SignalDetectionCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignalDetectionCondition").finish()
    }
}

/// 信号时序栈自动机
///
/// 对齐 C# `SignalTsiga`。维护信号状态（信道→ID→值）、
/// 输入图和时序栈，实现正向/反向状态转移。
#[derive(Debug)]
pub struct SignalTsiga {
    /// 输入图
    pub input_graph: InputGraph,
    /// 时序栈
    pub time_stack: TimeStack,
    /// 历史栈
    pub history_stack: HistoryStack,
    /// 信号状态：信道名 → (信号ID → 信号值)
    pub signal_state: HashMap<String, HashMap<i32, f32>>,
}

impl SignalTsiga {
    pub fn new(input_graph: InputGraph, time_stack: TimeStack, history_stack: HistoryStack) -> Self {
        Self { input_graph, time_stack, history_stack, signal_state: HashMap::new() }
    }

    /// 获取当前状态名（从输入图导出状态）
    pub fn state(&self) -> &str {
        &self.input_graph.export_state
    }

    /// 正向状态转移时间：输入图和时序栈都处于接受状态时无需转移
    pub fn forward_state_change_time(&self) -> f32 {
        if self.time_stack.accept && self.input_graph.accept {
            return f32::MAX;
        }
        // 返回输入图超时时间和时序栈弹栈时间中的较小值
        // 简化实现：返回时序栈顶时间（若有）
        self.time_stack.peek().map(|t| t.time).unwrap_or(f32::MAX)
    }

    /// 正向状态转移：在给定时间点推进状态
    pub fn forward_state_change(&mut self, chart_time: f32) {
        if self.time_stack.accept && self.input_graph.accept {
            return; // 已停机
        }
        // 先处理弹栈（超时）
        self.pop_until(chart_time);
        // 再处理输入图超时
        self.timeout_until(chart_time);
    }

    /// 反向状态转移：通过历史栈还原到指定时间
    pub fn backward_state_change(&mut self, chart_time: f32) {
        let _ = self.history_stack.pop_until(chart_time);
    }

    /// 弹栈处理：时间超过时序栈顶时弹出
    fn pop_until(&mut self, chart_time: f32) {
        while let Some(top) = self.time_stack.peek() {
            if top.time > chart_time { break; }
            self.time_stack.pop();
        }
    }

    /// 输入图超时处理
    fn timeout_until(&mut self, _chart_time: f32) {
        if self.input_graph.accept { return; }
        self.input_graph.do_timeout();
    }

    /// 获取当前检测条件列表
    pub fn get_detection_conditions(&self) -> Vec<SignalDetectionCondition> {
        let state = match self.input_graph.current_state() {
            Some(s) => s,
            None => return Vec::new(),
        };
        // 已停机则不检测
        if self.input_graph.accept && self.time_stack.accept {
            return Vec::new();
        }
        // 简化：返回一个条件
        // 完整实现需遍历 filter.conditionTypes 并为每个生成条件
        Vec::new()
    }

    /// 处理检测接受
    pub fn detection_accept(&mut self) {
        let stack_respond = self.input_graph.current_state()
            .map(|s| s.accepted_edge.stack_respond).unwrap_or(false);
        let edge_respond = self.input_graph.current_state()
            .map(|s| s.accepted_edge.edge_respond).unwrap_or(false);
        self.input_graph.go_accept_edge();
        if edge_respond { }
        if stack_respond { self.time_stack.pop(); }
    }

    /// 处理检测拒绝
    pub fn detection_deny(&mut self) {
        let stack_respond = self.input_graph.current_state()
            .map(|s| s.denied_edge.stack_respond).unwrap_or(false);
        self.input_graph.go_deny_edge();
        if stack_respond { self.time_stack.pop(); }
    }

    /// 更新信号状态
    pub fn update_signal(&mut self, channel: &str, id: i32, value: f32) {
        self.signal_state
            .entry(channel.to_string())
            .or_default()
            .insert(id, value);
    }

    /// 获取信号值
    pub fn get_signal(&self, channel: &str, id: i32) -> Option<f32> {
        self.signal_state.get(channel).and_then(|m| m.get(&id)).copied()
    }
}

impl Default for SignalTsiga {
    fn default() -> Self {
        Self::new(InputGraph::new(), TimeStack::new(), HistoryStack::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_tsiga_default() {
        let tsiga = SignalTsiga::default();
        assert_eq!(tsiga.state(), "");
        assert!(tsiga.signal_state.is_empty());
    }

    #[test]
    fn test_signal_update_and_get() {
        let mut tsiga = SignalTsiga::default();
        tsiga.update_signal("touch", 1, 0.75);
        assert_eq!(tsiga.get_signal("touch", 1), Some(0.75));
        assert_eq!(tsiga.get_signal("touch", 99), None);
    }

    #[test]
    fn test_forward_state_change_stopped() {
        let mut tsiga = SignalTsiga::default();
        // 都处于接受状态 = 已停机
        tsiga.time_stack.accept = true;
        tsiga.input_graph.accept = true;
        tsiga.forward_state_change(10.0);
        // 不应做任何变化
        assert!(tsiga.time_stack.accept);
    }
}
