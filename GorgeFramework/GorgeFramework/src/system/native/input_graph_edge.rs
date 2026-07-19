//! `GorgeFramework` — 输入图边（native 数据类）。
//!
//! 移植自 C# 参考实现 `InputGraphEdge.cs`。
//! InputGraphEdge 注册为 native 数据类（S2 前已有）。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 输入图边
///
/// 对齐 C# `InputGraphEdge`。描述状态之间的一条转移边。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct InputGraphEdge {
    /// 是否为拒绝边
    #[gorge_field] pub deny: bool,
    /// 跳转步数
    #[gorge_field] pub jump: i32,
    /// 弹栈响应标志
    #[gorge_field] pub stack_respond: bool,
    /// 边响应标志
    #[gorge_field] pub edge_respond: bool,
    /// 接收标志
    #[gorge_field] pub accept: bool,
    /// 导出状态名
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

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::object::GorgeObject;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    struct Fixture {
        vm: VirtualMachine,
    }

    impl Fixture {
        fn new() -> Self {
            let mut vm = VirtualMachine::new();
            vm.next_object_id = 100;
            Self { vm }
        }
        fn ctx(&mut self) -> NativeContext<'_> { NativeContext::new(&mut self.vm) }
    }

    #[test]
    fn test_input_graph_edge_construct() {
        let e = InputGraphEdge { deny: false, jump: 0, stack_respond: false, edge_respond: false, accept: false, export_state: String::new() };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_bool_param(0, false);
        fx.vm.param_pool.set_int_param(0, 1);
        fx.vm.param_pool.set_bool_param(1, true);
        fx.vm.param_pool.set_bool_param(2, true);
        fx.vm.param_pool.set_bool_param(3, true);
        fx.vm.param_pool.set_string_param(0, "Active".to_string());
        let id = { let mut ctx = fx.ctx(); e.do_construct_native(&mut ctx, None, 0) };
        let obj = fx.vm.objects.get(&id).unwrap();
        assert!(!obj.get_bool_field(0)); // deny=false
        assert_eq!(obj.get_int_field(0), 1); // jump=1
        assert!(obj.get_bool_field(1)); // stack_respond
        assert!(obj.get_bool_field(2)); // edge_respond
        assert!(obj.get_bool_field(3)); // accept
    }
}
