//! `GorgeFramework.InputGraphState` —— 输入图状态节点 native 类。
//!
//! 移植自 C# 参考实现 `InputGraphState.cs`。
//! 每个状态包含一个信号过滤器和两条出边（接受/拒绝）。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 输入图状态节点
///
/// 对齐 C# `InputGraphState`。字段均为对象 ID（SignalFilter / InputGraphEdge）。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct InputGraphState {
    /// 信号过滤器（SignalFilter 对象 ID）
    #[gorge_field]
    pub filter: usize,
    /// 接受出边（InputGraphEdge 对象 ID）
    #[gorge_field]
    pub accepted_edge: usize,
    /// 拒绝出边（InputGraphEdge 对象 ID）
    #[gorge_field]
    pub denied_edge: usize,
}

#[gorge_native_impl]
impl InputGraphState {
    #[gorge_ctor]
    pub fn new_ctor(
        ctx: &mut NativeContext,
        this: usize,
        filter: usize,
        accepted_edge: usize,
        denied_edge: usize,
    ) {
        ctx.set_object_object_field(this, InputGraphState::FIELD_INDEX_filter, filter);
        ctx.set_object_object_field(this, InputGraphState::FIELD_INDEX_accepted_edge, accepted_edge);
        ctx.set_object_object_field(this, InputGraphState::FIELD_INDEX_denied_edge, denied_edge);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::object::GorgeObject;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    #[test]
    fn test_input_graph_state_construct() {
        let igs = InputGraphState { filter: 0, accepted_edge: 0, denied_edge: 0 };
        let mut vm = VirtualMachine::new();
        vm.param_pool.set_object_param(0, 10); // filter
        vm.param_pool.set_object_param(1, 20); // accepted_edge
        vm.param_pool.set_object_param(2, 30); // denied_edge
        let id = { let mut ctx = NativeContext::new(&mut vm); igs.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        let obj = vm.objects.get(&id).unwrap();
        assert_eq!(obj.get_object_field(0), 10);
        assert_eq!(obj.get_object_field(1), 20);
        assert_eq!(obj.get_object_field(2), 30);
    }
}
