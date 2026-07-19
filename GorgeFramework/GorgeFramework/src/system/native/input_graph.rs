//! `GorgeFramework` — 输入图系统（InputGraph native 类注册）。
//!
//! 移植自 C# 参考实现 `InputGraph.cs`。
//! InputGraph 注册为 native 状态机类。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 输入图
///
/// 对齐 C# `InputGraph`。维护状态列表和当前输入指针，
/// 支持正向状态转移（接受/拒绝/超时）和反向状态还原。
///
/// Gorge 字段：states (ObjectArray ID), input_pointer, accept, stack_respond, export_state。
///
/// 方法编号表：
/// | 编号 | 方法 | 说明 |
/// |------|------|------|
/// | 0 | state_count | 状态总数 |
/// | 1 | state_timeout | 当前状态的超时时间（经 filter.endTime 委托） |
/// | 2 | do_timeout | 超时转移（时间 ≤ endTime 时沿拒绝边或接受边） |
/// | 3 | go_accept_edge | 沿接受边转移，推 InputGraphGoEdge 历史 |
/// | 4 | go_deny_edge | 沿拒绝边转移，推 InputGraphGoEdge 历史 |
/// | 5 | revert_go_edge | 反向还原：恢复 input_pointer/accept/stack_respond/export_state |
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct InputGraph {
    /// 状态列表（ObjectArray 对象 ID）
    #[gorge_field]
    pub states: usize,
    /// 当前输入指针
    #[gorge_field]
    pub input_pointer: i32,
    /// 接收模式
    #[gorge_field]
    pub accept: bool,
    /// 栈响应模式
    #[gorge_field]
    pub stack_respond: bool,
    /// 导出状态
    #[gorge_field]
    pub export_state: String,
}

#[gorge_native_impl]
impl InputGraph {
    #[gorge_ctor]
    pub fn new_ctor(
        ctx: &mut NativeContext,
        this: usize,
        states: usize,
        accept: bool,
        stack_respond: bool,
        input_pointer: i32,
        export_state: String,
    ) {
        ctx.set_object_object_field(this, InputGraph::FIELD_INDEX_states, states);
        ctx.set_object_bool_field(this, InputGraph::FIELD_INDEX_accept, accept);
        ctx.set_object_bool_field(this, InputGraph::FIELD_INDEX_stack_respond, stack_respond);
        ctx.set_object_int_field(this, InputGraph::FIELD_INDEX_input_pointer, input_pointer as i64);
        ctx.set_object_string_field(this, InputGraph::FIELD_INDEX_export_state, export_state);
    }

    /// 状态总数（方法 0）
    #[gorge_method]
    pub fn state_count(ctx: &mut NativeContext, this: usize) -> i32 {
        let states_id = ctx.get_object_object_field(this, InputGraph::FIELD_INDEX_states);
        ctx.object_array_len(states_id) as i32
    }

    /// 状态超时时间（方法 1）
    ///
    /// 对齐 C# `StateTimeout`。读当前状态 → filter 字段（InputGraphState field 0）
    /// → filter.endTime 字段（SignalFilter field 2）→ invoke_delegate → float 返回。
    #[gorge_method]
    pub fn state_timeout(ctx: &mut NativeContext, this: usize) -> f32 {
        let states_id = ctx.get_object_object_field(this, InputGraph::FIELD_INDEX_states);
        let input_pointer = ctx.get_object_int_field(this, InputGraph::FIELD_INDEX_input_pointer) as i32;
        let state_id = ctx.object_array_get(states_id, input_pointer as usize);
        if state_id == 0 {
            return f32::MAX;
        }
        // filter 是 InputGraphState 的 field 0
        let filter_id = ctx.get_object_object_field(state_id, 0);
        if filter_id == 0 {
            return f32::MAX;
        }
        // end_time 是 SignalFilter 的 field 2
        let end_time_delegate_id = ctx.get_object_object_field(filter_id, 2);
        if end_time_delegate_id == 0 {
            return f32::MAX;
        }
        ctx.invoke_delegate(end_time_delegate_id);
        (ctx.get_float_return() as f64) as f32
    }

    /// 超时转移（方法 2）
    ///
    /// 对齐 C# `DoTimeout`。若 endTime <= targetChartTime：
    /// timeMode==CatchBefore(0) → go_deny_edge；否则 → go_accept_edge。
    /// 返回触发的 InputGraphEdge 对象 ID（无操作则 0）。
    #[gorge_method]
    pub fn do_timeout(ctx: &mut NativeContext, this: usize, target_chart_time: f32, history_stack_id: usize) -> usize {
        let states_id = ctx.get_object_object_field(this, InputGraph::FIELD_INDEX_states);
        let input_pointer = ctx.get_object_int_field(this, InputGraph::FIELD_INDEX_input_pointer) as i32;
        let state_id = ctx.object_array_get(states_id, input_pointer as usize);
        if state_id == 0 {
            return 0;
        }

        let filter_id = ctx.get_object_object_field(state_id, 0);
        if filter_id == 0 {
            return 0;
        }

        let end_time_delegate_id = ctx.get_object_object_field(filter_id, 2);
        if end_time_delegate_id == 0 {
            return 0;
        }

        ctx.invoke_delegate(end_time_delegate_id);
        let end_time = (ctx.get_float_return() as f64) as f32;
        if end_time > target_chart_time {
            return 0;
        }

        let time_mode = ctx.get_object_int_field(filter_id, 0) as i32; // field 0 of SignalFilter = time_mode (int field 0)

        if time_mode == 0 {
            // CatchBefore → go_deny_edge
            ctx.set_float_param(0, target_chart_time as f64);
            ctx.set_object_param(0, history_stack_id);
            ctx.invoke_native_method_on("GorgeFramework.InputGraph", this, 4);
        } else {
            // KeepUntil → go_accept_edge
            ctx.set_float_param(0, target_chart_time as f64);
            ctx.set_object_param(0, history_stack_id);
            ctx.invoke_native_method_on("GorgeFramework.InputGraph", this, 3);
        }
        // 返回触发的 edge（当前状态的对应边）
        // 由于我们调用了 go_accept/deny_edge 后 state 已变，返回原来的边对象 ID
        // 简化：返回 state 的 accepted_edge 或 denied_edge
        if time_mode == 0 {
            ctx.get_object_object_field(state_id, 2) // denied_edge (field 2)
        } else {
            ctx.get_object_object_field(state_id, 1) // accepted_edge (field 1)
        }
    }

    /// 沿接受边转移（方法 3）
    ///
    /// 对齐 C# `GoAcceptEdge`。取当前状态 → 读 acceptedEdge（field 1），
    /// 推 InputGraphGoEdge 历史（HistoryStack method 1），
    /// 按边更新 input_pointer/accept/stack_respond/export_state。
    /// 返回边对象 ID。
    #[gorge_method]
    pub fn go_accept_edge(ctx: &mut NativeContext, this: usize, chart_time: f32, history_stack_id: usize) -> usize {
        let states_id = ctx.get_object_object_field(this, InputGraph::FIELD_INDEX_states);
        let input_pointer = ctx.get_object_int_field(this, InputGraph::FIELD_INDEX_input_pointer) as i32;
        let state_id = ctx.object_array_get(states_id, input_pointer as usize);
        if state_id == 0 {
            return 0;
        }

        let edge_id = ctx.get_object_object_field(state_id, 1); // accepted_edge
        if edge_id == 0 {
            return 0;
        }

        // 推历史：InputGraphGoEdge
        let accept_before = ctx.get_object_bool_field(this, InputGraph::FIELD_INDEX_accept);
        let stack_respond_before = ctx.get_object_bool_field(this, InputGraph::FIELD_INDEX_stack_respond);
        let export_state_before = ctx.get_object_string_field(this, InputGraph::FIELD_INDEX_export_state);

        ctx.set_float_param(0, chart_time as f64);
        ctx.set_int_param(0, input_pointer as i64);
        ctx.set_bool_param(0, accept_before);
        ctx.set_bool_param(1, stack_respond_before);
        ctx.set_string_param(0, export_state_before);
        ctx.invoke_native_method_on("GorgeFramework.HistoryStack", history_stack_id, 1);

        // 按边更新
        // InputGraphEdge bool 字段顺序: deny(0), stack_respond(1), edge_respond(2), accept(3)
        let edge_accept = ctx.get_object_bool_field(edge_id, 3); // accept
        let edge_stack_respond = ctx.get_object_bool_field(edge_id, 1); // stack_respond
        let edge_export_state = ctx.get_object_string_field(edge_id, 0); // export_state (string field 0)
        let edge_deny = ctx.get_object_bool_field(edge_id, 0); // deny
        let edge_jump = ctx.get_object_int_field(edge_id, 0) as i32; // jump (int field 0)

        ctx.set_object_bool_field(this, InputGraph::FIELD_INDEX_accept, edge_accept);
        ctx.set_object_bool_field(this, InputGraph::FIELD_INDEX_stack_respond, edge_stack_respond);
        if !edge_export_state.is_empty() {
            ctx.set_object_string_field(this, InputGraph::FIELD_INDEX_export_state, edge_export_state);
        }

        if !edge_deny {
            let new_ptr = input_pointer + edge_jump;
            ctx.set_object_int_field(this, InputGraph::FIELD_INDEX_input_pointer, new_ptr as i64);
        } else {
            ctx.set_object_string_field(this, InputGraph::FIELD_INDEX_export_state, "Denied".to_string());
            ctx.set_object_int_field(this, InputGraph::FIELD_INDEX_input_pointer, -1);
        }

        edge_id
    }

    /// 沿拒绝边转移（方法 4）
    #[gorge_method]
    pub fn go_deny_edge(ctx: &mut NativeContext, this: usize, chart_time: f32, history_stack_id: usize) -> usize {
        let states_id = ctx.get_object_object_field(this, InputGraph::FIELD_INDEX_states);
        let input_pointer = ctx.get_object_int_field(this, InputGraph::FIELD_INDEX_input_pointer) as i32;
        let state_id = ctx.object_array_get(states_id, input_pointer as usize);
        if state_id == 0 {
            return 0;
        }

        let edge_id = ctx.get_object_object_field(state_id, 2); // denied_edge
        if edge_id == 0 {
            return 0;
        }

        let accept_before = ctx.get_object_bool_field(this, InputGraph::FIELD_INDEX_accept);
        let stack_respond_before = ctx.get_object_bool_field(this, InputGraph::FIELD_INDEX_stack_respond);
        let export_state_before = ctx.get_object_string_field(this, InputGraph::FIELD_INDEX_export_state);

        ctx.set_float_param(0, chart_time as f64);
        ctx.set_int_param(0, input_pointer as i64);
        ctx.set_bool_param(0, accept_before);
        ctx.set_bool_param(1, stack_respond_before);
        ctx.set_string_param(0, export_state_before);
        ctx.invoke_native_method_on("GorgeFramework.HistoryStack", history_stack_id, 1);

        // InputGraphEdge bool 字段顺序: deny(0), stack_respond(1), edge_respond(2), accept(3)
        let edge_accept = ctx.get_object_bool_field(edge_id, 3);
        let edge_stack_respond = ctx.get_object_bool_field(edge_id, 1);
        let edge_export_state = ctx.get_object_string_field(edge_id, 0);
        let edge_deny = ctx.get_object_bool_field(edge_id, 0);
        let edge_jump = ctx.get_object_int_field(edge_id, 0) as i32;

        ctx.set_object_bool_field(this, InputGraph::FIELD_INDEX_accept, edge_accept);
        ctx.set_object_bool_field(this, InputGraph::FIELD_INDEX_stack_respond, edge_stack_respond);
        if !edge_export_state.is_empty() {
            ctx.set_object_string_field(this, InputGraph::FIELD_INDEX_export_state, edge_export_state);
        }

        if !edge_deny {
            let new_ptr = input_pointer + edge_jump;
            ctx.set_object_int_field(this, InputGraph::FIELD_INDEX_input_pointer, new_ptr as i64);
        } else {
            ctx.set_object_string_field(this, InputGraph::FIELD_INDEX_export_state, "Denied".to_string());
            ctx.set_object_int_field(this, InputGraph::FIELD_INDEX_input_pointer, -1);
        }

        edge_id
    }

    /// 反向还原转移（方法 5）
    #[gorge_method]
    pub fn revert_go_edge(
        ctx: &mut NativeContext,
        this: usize,
        pointer_before: i32,
        accept_before: bool,
        stack_respond_before: bool,
        export_state_before: String,
    ) {
        ctx.set_object_bool_field(this, InputGraph::FIELD_INDEX_accept, accept_before);
        ctx.set_object_bool_field(this, InputGraph::FIELD_INDEX_stack_respond, stack_respond_before);
        ctx.set_object_int_field(this, InputGraph::FIELD_INDEX_input_pointer, pointer_before as i64);
        ctx.set_object_string_field(this, InputGraph::FIELD_INDEX_export_state, export_state_before);
    }
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::native::input_graph_edge::InputGraphEdge;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::object::GorgeObject;
    use gorge_core::virtual_machine::vm::VirtualMachine;
    use crate::system::native::input_graph_state::InputGraphState;
    use crate::system::native::history::{HistoryStack, HistoryStackPayload};
    use crate::system::native::signal_filter_native::SignalFilter;

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

        fn make_history_stack(&mut self) -> usize {
            let hs = HistoryStack { _placeholder: false };
            let id = { let mut ctx = self.ctx(); hs.do_construct_native(&mut ctx, None, 0) };
            self.vm.native_class_table.insert(
                "GorgeFramework.HistoryStack".to_string(),
                std::sync::Arc::new(HistoryStack { _placeholder: false }),
            );
            id
        }

        fn make_edge(&mut self, deny: bool, jump: i32, stack_respond: bool, edge_respond: bool, accept: bool, export_state: &str) -> usize {
            let e = InputGraphEdge { deny: false, jump: 0, stack_respond: false, edge_respond: false, accept: false, export_state: String::new() };
            self.vm.param_pool.set_bool_param(0, deny);
            self.vm.param_pool.set_int_param(0, jump as i64);
            self.vm.param_pool.set_bool_param(1, stack_respond);
            self.vm.param_pool.set_bool_param(2, edge_respond);
            self.vm.param_pool.set_bool_param(3, accept);
            self.vm.param_pool.set_string_param(0, export_state.to_string());
            let id = { let mut ctx = self.ctx(); e.do_construct_native(&mut ctx, None, 0) };
            id
        }

        fn make_state(&mut self, filter_id: usize, accepted_edge_id: usize, denied_edge_id: usize) -> usize {
            let s = InputGraphState { filter: 0, accepted_edge: 0, denied_edge: 0 };
            self.vm.param_pool.set_object_param(0, filter_id);
            self.vm.param_pool.set_object_param(1, accepted_edge_id);
            self.vm.param_pool.set_object_param(2, denied_edge_id);
            let id = { let mut ctx = self.ctx(); s.do_construct_native(&mut ctx, None, 0) };
            id
        }

        fn make_filter(&mut self, end_time_delegate_id: usize, time_mode: i32) -> usize {
            let sf = SignalFilter {
                priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
                accept_consume: true, deny_consume: false,
            };
            self.vm.param_pool.set_object_param(0, 0);  // priority
            self.vm.param_pool.set_object_param(1, 0);  // condition_types
            self.vm.param_pool.set_object_param(2, end_time_delegate_id); // end_time
            self.vm.param_pool.set_int_param(0, time_mode as i64);
            self.vm.param_pool.set_bool_param(0, true);
            self.vm.param_pool.set_bool_param(1, false);
            let id = { let mut ctx = self.ctx(); sf.do_construct_native(&mut ctx, None, 0) };
            id
        }
    }

    #[test]
    fn test_input_graph_construct() {
        let ig = InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_object_param(0, 0);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_string_param(0, "Waiting".to_string());
        let id = { let mut ctx = fx.ctx(); ig.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        let obj = fx.vm.objects.get(&id).unwrap();
        assert!(obj.get_bool_field(0)); // accept
        assert!(!obj.get_bool_field(1)); // stack_respond
    }

    #[test]
    fn test_input_graph_go_accept_edge() {
        let ig = InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() };
        let mut fx = Fixture::new();
        let hs_id = fx.make_history_stack();
        // 注册 InputGraph 到 native_class_table（供 do_timeout 等自身调用）
        fx.vm.native_class_table.insert(
            "GorgeFramework.InputGraph".to_string(),
            std::sync::Arc::new(InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() }),
        );

        // 创建两个 edge 和两个 state
        let acc_edge = fx.make_edge(false, 1, false, true, true, "Active");
        let den_edge = fx.make_edge(true, 0, false, false, false, "");
        let filter = fx.make_filter(0, 0);
        let state0 = fx.make_state(filter, acc_edge, den_edge);
        let state1 = fx.make_state(filter, acc_edge, den_edge);

        // 创建 ObjectArray 装载 states
        use gorge_core::system::native::array::ObjectArrayClass;
        let cls = ObjectArrayClass;
        let arr_id = { let mut ctx = fx.ctx(); cls.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = fx.ctx(); ctx.object_array_add(arr_id, state0); }
        { let mut ctx = fx.ctx(); ctx.object_array_add(arr_id, state1); }

        fx.vm.param_pool.set_object_param(0, arr_id);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_string_param(0, "Waiting".to_string());
        let id = { let mut ctx = fx.ctx(); ig.do_construct_native(&mut ctx, None, 0) };

        // go_accept_edge(chart_time=1.0, history_stack)
        fx.vm.param_pool.set_float_param(0, 1.0);
        fx.vm.param_pool.set_object_param(0, hs_id);
        { let mut ctx = fx.ctx(); ig.invoke_native_method(&mut ctx, id, 3); }

        // 验证 input_pointer 已变为 1
        let obj = fx.vm.objects.get(&id).unwrap();
        assert_eq!(obj.get_int_field(0), 1);

        // HistoryStack 应有 1 条 InputGraphGoEdge 记录
        let p = fx.vm.native_payloads.get(&hs_id).unwrap().downcast_ref::<HistoryStackPayload>().unwrap();
        assert_eq!(p.stack.len(), 1);
    }

    #[test]
    fn test_input_graph_revert_go_edge() {
        let ig = InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_object_param(0, 0);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_int_param(0, 5);
        fx.vm.param_pool.set_string_param(0, "Active".to_string());
        let id = { let mut ctx = fx.ctx(); ig.do_construct_native(&mut ctx, None, 0) };

        // revert_go_edge(pointer_before=0, accept_before=true, stack_respond_before=true, export_state_before="Waiting")
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, true);
        fx.vm.param_pool.set_string_param(0, "Waiting".to_string());
        { let mut ctx = fx.ctx(); ig.invoke_native_method(&mut ctx, id, 5); }

        let obj = fx.vm.objects.get(&id).unwrap();
        assert_eq!(obj.get_int_field(0), 0);
        assert!(obj.get_bool_field(0));  // accept=true
        assert!(obj.get_bool_field(1));  // stack_respond=true
        assert_eq!(obj.get_string_field(0), "Waiting");
    }

    #[test]
    fn test_input_graph_state_count() {
        let ig = InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() };
        let mut fx = Fixture::new();

        use gorge_core::system::native::array::ObjectArrayClass;
        let cls = ObjectArrayClass;
        let arr_id = { let mut ctx = fx.ctx(); cls.do_construct_native(&mut ctx, None, 0) };
        // 添加 3 个元素
        for _ in 0..3 { let mut ctx = fx.ctx(); ctx.object_array_add(arr_id, 100); }

        fx.vm.param_pool.set_object_param(0, arr_id);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_string_param(0, String::new());
        let id = { let mut ctx = fx.ctx(); ig.do_construct_native(&mut ctx, None, 0) };

        { let mut ctx = fx.ctx(); ig.invoke_native_method(&mut ctx, id, 0); }
        assert_eq!(fx.vm.param_pool.get_int_return(), 3);
    }

    #[test]
    fn test_input_graph_state_timeout_with_delegate() {
        use gorge_core::objective::delegate::RuntimeDelegate;
        use gorge_core::objective::types::{GorgeType, BasicType};
        use gorge_core::objective::value_pool::FixedFieldValuePool;
        use gorge_core::virtual_machine::ir::{CompiledMethod, CodeWithSpan, IntermediateCode, Operand, Address, ValueType};

        let ig = InputGraph { states: 0, input_pointer: 0, accept: true, stack_respond: false, export_state: String::new() };
        let mut fx = Fixture::new();

        // 创建返回 5.0 的委托
        let result_addr = Address::new(ValueType::Float, 0);
        let method = CompiledMethod {
            name: "get5".into(),
            codes: vec![
                CodeWithSpan::new(
                    IntermediateCode::assign(result_addr, Operand::float(5.0)),
                    gorge_core::diagnostics::Span::dummy(),
                ),
                CodeWithSpan::new(
                    IntermediateCode::return_value(ValueType::Float),
                    gorge_core::diagnostics::Span::dummy(),
                ),
            ],
            local_count: 1,
        };
        let delegate = RuntimeDelegate {
            delegate_type: GorgeType::new(BasicType::Delegate),
            method_impl: method,
            captured_values: FixedFieldValuePool::default(),
            param_types: vec![],
            captured_var_types: vec![],
            creator_this: None,
        };
        let delegate_obj_id = fx.vm.next_object_id;
        fx.vm.next_object_id += 1;
        fx.vm.runtime_delegates.insert(delegate_obj_id, delegate);

        let filter = fx.make_filter(delegate_obj_id, 0);
        let acc_edge = fx.make_edge(false, 1, false, true, true, "");
        let den_edge = fx.make_edge(true, 0, false, false, false, "");
        let state0 = fx.make_state(filter, acc_edge, den_edge);

        use gorge_core::system::native::array::ObjectArrayClass;
        let cls = ObjectArrayClass;
        let arr_id = { let mut ctx = fx.ctx(); cls.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = fx.ctx(); ctx.object_array_add(arr_id, state0); }

        fx.vm.param_pool.set_object_param(0, arr_id);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_string_param(0, String::new());
        let id = { let mut ctx = fx.ctx(); ig.do_construct_native(&mut ctx, None, 0) };

        // state_timeout → 应经 delegate 返回 5.0
        { let mut ctx = fx.ctx(); ig.invoke_native_method(&mut ctx, id, 1); }
        let result = (fx.vm.param_pool.get_float_return() as f64) as f32;
        assert!((result - 5.0).abs() < 0.01, "预期 5.0，实际 {}", result);
    }
}
