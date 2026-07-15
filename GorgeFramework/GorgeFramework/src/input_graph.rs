//! `GorgeFramework` — 输入图系统（native 类注册）。
//!
//! 移植自 C# 参考实现。InputGraphEdge 注册为 native 数据类；
//! InputGraph/InputGraphState（含 Vec/Option<Box<dyn>>）保留为内部类型。

use crate::signal_filter::SignalFilter;
use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::native::NativeContext;

// ==================== InputGraphEdge（native 注册） ====================

/// 输入图边
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct InputGraphEdge {
    #[gorge_field] pub deny: bool,
    #[gorge_field] pub jump: i32,
    #[gorge_field] pub stack_respond: bool,
    #[gorge_field] pub edge_respond: bool,
    #[gorge_field] pub accept: bool,
    #[gorge_field] pub export_state: String,
}

impl InputGraphEdge {
    pub fn accept_edge() -> Self {
        Self { deny: false, jump: 1, stack_respond: false, edge_respond: true, accept: true, export_state: String::new() }
    }
    pub fn deny_edge() -> Self {
        Self { deny: true, jump: 0, stack_respond: false, edge_respond: false, accept: false, export_state: String::new() }
    }
    pub fn with_jump(jump: i32, stack_respond: bool, edge_respond: bool, export_state: &str) -> Self {
        Self { deny: false, jump, stack_respond, edge_respond, accept: true, export_state: export_state.into() }
    }
}

#[gorge_native_impl]
impl InputGraphEdge {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize,
        deny: bool, jump: i32, stack_respond: bool, edge_respond: bool, accept: bool, export_state: String)
    {
        ctx.set_object_bool_field(this, InputGraphEdge::FIELD_INDEX_deny, deny);
        ctx.set_object_int_field(this, InputGraphEdge::FIELD_INDEX_jump, jump as i64);
        ctx.set_object_bool_field(this, InputGraphEdge::FIELD_INDEX_stack_respond, stack_respond);
        ctx.set_object_bool_field(this, InputGraphEdge::FIELD_INDEX_edge_respond, edge_respond);
        ctx.set_object_bool_field(this, InputGraphEdge::FIELD_INDEX_accept, accept);
        ctx.set_object_string_field(this, InputGraphEdge::FIELD_INDEX_export_state, export_state);
    }
}

impl Default for InputGraphEdge {
    fn default() -> Self { Self::accept_edge() }
}

impl Clone for InputGraphEdge {
    fn clone(&self) -> Self {
        Self {
            deny: self.deny, jump: self.jump, stack_respond: self.stack_respond,
            edge_respond: self.edge_respond, accept: self.accept,
            export_state: self.export_state.clone(),
        }
    }
}

// ==================== 输入图状态 ====================

/// 输入图状态节点
///
/// 对齐 C# `InputGraphState`。代表状态图中的一个状态，
/// 包含信号过滤器和接受/拒绝两条出边。
#[derive(Debug)]
pub struct InputGraphState {
    /// 信号过滤器（决定本状态接受哪些信号）
    pub filter: Option<Box<dyn SignalFilter>>,
    /// 接受出边（检测到信号时沿此边走）
    pub accepted_edge: InputGraphEdge,
    /// 拒绝出边（未检测到信号时沿此边走）
    pub denied_edge: InputGraphEdge,
}

impl InputGraphState {
    pub fn new(filter: Option<Box<dyn SignalFilter>>) -> Self {
        Self { filter, accepted_edge: InputGraphEdge::accept_edge(), denied_edge: InputGraphEdge::deny_edge() }
    }
}

// ==================== 输入图 ====================

/// 输入图（自动机状态图）
///
/// 对齐 C# `InputGraph`。管理状态列表和当前输入指针，
/// 支持正向状态转移（接受/拒绝/超时）和反向状态还原。
#[derive(Debug)]
pub struct InputGraph {
    /// 状态列表
    pub states: Vec<InputGraphState>,
    /// 当前输入指针（指向当前活跃状态的索引）
    pub input_pointer: usize,
    /// 当前是否处于接受状态
    pub accept: bool,
    /// 是否弹栈响应
    pub stack_respond: bool,
    /// 导出状态名
    pub export_state: String,
}

impl InputGraph {
    pub fn new() -> Self {
        Self { states: Vec::new(), input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() }
    }

    /// 添加状态
    pub fn add_state(&mut self, state: InputGraphState) {
        self.states.push(state);
    }

    /// 获取当前状态
    pub fn current_state(&self) -> Option<&InputGraphState> {
        self.states.get(self.input_pointer)
    }

    /// 沿接受边转移（正向一步）
    pub fn go_accept_edge(&mut self) -> Option<&InputGraphEdge> {
        let edge = self.current_state()?.accepted_edge.clone();
        let new_ptr = (self.input_pointer as i32 + edge.jump).max(0) as usize;
        self.input_pointer = new_ptr.min(self.states.len().saturating_sub(1));
        self.accept = edge.accept;
        self.stack_respond = edge.stack_respond;
        if !edge.export_state.is_empty() { self.export_state = edge.export_state.clone(); }
        Some(&self.states[self.input_pointer].accepted_edge)
    }

    /// 沿拒绝边转移
    pub fn go_deny_edge(&mut self) -> Option<&InputGraphEdge> {
        let edge = self.current_state()?.denied_edge.clone();
        let new_ptr = (self.input_pointer as i32 + edge.jump).max(0) as usize;
        self.input_pointer = new_ptr.min(self.states.len().saturating_sub(1));
        self.accept = edge.accept;
        self.stack_respond = edge.stack_respond;
        if !edge.export_state.is_empty() { self.export_state = edge.export_state.clone(); }
        Some(&self.states[self.input_pointer].accepted_edge)
    }

    /// 超时处理：状态未匹配时，沿拒绝边走一步
    pub fn do_timeout(&mut self) {
        if let Some(state) = self.current_state() {
            let edge = &state.denied_edge;
            let new_ptr = (self.input_pointer as i32 + edge.jump).max(0) as usize;
            self.input_pointer = new_ptr.min(self.states.len().saturating_sub(1));
        }
    }
}

impl Default for InputGraph {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal_filter::FloatSignalFilter;

    #[test]
    fn test_input_graph_empty() {
        let g = InputGraph::new();
        assert!(g.current_state().is_none());
    }

    #[test]
    fn test_input_graph_transition() {
        let mut g = InputGraph::new();
        let f1 = FloatSignalFilter::new("tap", 0.0, 1.0);
        let state1 = InputGraphState::new(Some(Box::new(f1)));
        let state2 = InputGraphState::new(None);
        g.add_state(state1);
        g.add_state(state2);
        assert_eq!(g.input_pointer, 0);
        g.go_accept_edge();
        assert_eq!(g.input_pointer, 1);
    }
}
