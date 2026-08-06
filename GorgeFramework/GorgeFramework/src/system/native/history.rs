//! `GorgeFramework` — 历史栈系统（native 类注册）。
//!
//! 移植自 C# 参考实现 `HistoryStack.cs`。
//! HistoryStack 为 native 类，内部 `Vec<HistoryItem>` 通过 native_payloads 存储。
//! HistoryItem 以枚举表达 C# 中多种 IHistoryItem 实现。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::{NativeClass, NativeContext};
use gorge_core::system::native::array::ObjectArrayClass;

// ==================== HistoryItem 枚举 ====================

/// 历史项（对齐 C# IHistoryItem 的多种实现）
///
/// C# 中有 InputGraphGoEdgeHistory、TimeStackPushHistory、TimeStackPopHistory 等
/// 都实现了 IHistoryItem。Rust 侧用枚举统一存储各变体数据。
#[derive(Debug, Clone)]
pub enum HistoryItem {
    /// InputGraphGoEdgeHistory：转移前的输入图状态快照
    InputGraphGoEdge {
        chart_time: f32,
        pointer_before_go: i32,
        accept_before_go: bool,
        stack_respond_before_go: bool,
        export_state_before_go: String,
    },
    /// TimeStackPushHistory：压栈历史
    TimeStackPush {
        chart_time: f32,
    },
    /// TimeStackPopHistory：弹栈历史
    TimeStackPop {
        chart_time: f32,
        /// 被弹出的 TimeItem 对象 ID
        time_item_id: usize,
        accept_before_pop: bool,
        respond_mode_before_pop: String,
    },
}

impl HistoryItem {
    pub fn chart_time(&self) -> f32 {
        match self {
            HistoryItem::InputGraphGoEdge { chart_time, .. } => *chart_time,
            HistoryItem::TimeStackPush { chart_time } => *chart_time,
            HistoryItem::TimeStackPop { chart_time, .. } => *chart_time,
        }
    }
}

// ==================== HistoryStack 内部 payload ====================

/// HistoryStack 内部存储（存于 vm.native_payloads）
#[derive(Debug)]
pub struct HistoryStackPayload {
    /// 历史栈（按时间顺序，最后入栈的在 Vec 末尾）
    pub stack: Vec<HistoryItem>,
}

// ==================== HistoryStack（native 注册） ====================

/// 历史栈
///
/// 对齐 C# `HistoryStack`。维护一个历史记录栈，
/// 支持 push 记录和 pop_until 到指定时间点。
/// 在反向模拟时用于还原自动机状态。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct HistoryStack {
    /// 占位字段（HistoryStack 无 Gorge 可见字段，但宏要求至少一个字段）
    #[gorge_field]
    pub _placeholder: bool,
}

#[gorge_native_impl]
impl HistoryStack {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.insert_payload(this, Box::new(HistoryStackPayload { stack: Vec::new() }));
    }

    /// 栈顶历史时间（对齐 C# RevertTime）
    ///
    /// 若无栈顶则返回 f32::MIN。
    #[gorge_method]
    pub fn revert_time(ctx: &mut NativeContext, this: usize) -> f32 {
        with_payload(ctx, this, |p| {
            p.stack.last().map(|h| h.chart_time()).unwrap_or(f32::MIN)
        })
    }

    /// 推入 InputGraphGoEdge 历史项
    ///
    /// 对齐 C# `HistoryStack.Push(new InputGraphGoEdgeHistory(...))`。
    #[gorge_method]
    pub fn push_input_graph_go_edge(
        ctx: &mut NativeContext,
        this: usize,
        chart_time: f32,
        pointer_before_go: i32,
        accept_before_go: bool,
        stack_respond_before_go: bool,
        export_state_before_go: String,
    ) {
        with_payload_mut(ctx, this, |p| {
            p.stack.push(HistoryItem::InputGraphGoEdge {
                chart_time,
                pointer_before_go,
                accept_before_go,
                stack_respond_before_go,
                export_state_before_go,
            });
        });
    }

    /// 推入 TimeStackPush 历史项
    #[gorge_method]
    pub fn push_time_stack_push(ctx: &mut NativeContext, this: usize, chart_time: f32) {
        with_payload_mut(ctx, this, |p| {
            p.stack.push(HistoryItem::TimeStackPush { chart_time });
        });
    }

    /// 推入 TimeStackPop 历史项
    #[gorge_method]
    pub fn push_time_stack_pop(
        ctx: &mut NativeContext,
        this: usize,
        chart_time: f32,
        time_item_id: usize,
        accept_before_pop: bool,
        respond_mode_before_pop: String,
    ) {
        with_payload_mut(ctx, this, |p| {
            p.stack.push(HistoryItem::TimeStackPop {
                chart_time,
                time_item_id,
                accept_before_pop,
                respond_mode_before_pop,
            });
        });
    }

    /// 弹栈并执行还原动作，直到目标时间
    ///
    /// 对齐 C# `PopUntil`。从栈顶弹出 chart_time >= target_chart_time 的历史项，
    /// 依次调用对应的 revert 方法（InputGraph.revert_go_edge / TimeStack.revert_push / TimeStack.revert_pop）。
    /// 每当弹出一个 TimeStackPopHistory，就记录受影响的自动机 ID（对齐 C#：弹栈产生
    /// `UpdatePendingDetectionCondition(automaton, direction)` 动作，作用于该 automaton）。
    /// 返回装有所受影响自动机 ID 的 ObjectArray 对象 ID（列表为空返回 0）。
    #[gorge_method]
    pub fn pop_until(
        ctx: &mut NativeContext,
        this: usize,
        target_chart_time: f32,
        automaton_id: usize,
        _direction: i32,
        input_graph_id: usize,
        time_stack_id: usize,
    ) -> usize {
        // 收集需要 revert 的历史项（保留栈内顺序：底→顶）
        let items_to_revert: Vec<HistoryItem> = {
            let payload = ctx.get_payload::<HistoryStackPayload>(this);
            if let Some(payload) = payload {
                // 找到最后一个 chart_time < target 的位置，其后（含栈顶）全部需要还原
                let keep_idx = payload.stack.iter()
                    .rposition(|h| h.chart_time() < target_chart_time)
                    .map(|i| i + 1)
                    .unwrap_or(0);
                payload.stack[keep_idx..].to_vec()
            } else {
                Vec::new()
            }
        };

        // 从栈中弹出这些项
        with_payload_mut(ctx, this, |p| {
            let keep_idx = p.stack.iter()
                .rposition(|h| h.chart_time() < target_chart_time)
                .map(|i| i + 1)
                .unwrap_or(0);
            p.stack.truncate(keep_idx);
        });

        // 对 TimeStackPopHistory 变体，记录受影响的自动机 ID
        // 对齐 C# HistoryStack.PopUntil：弹栈时若为 TimeStackPopHistory 则产生
        // UpdatePendingDetectionCondition(automaton, direction) 动作，作用于该 automaton
        let mut affected_automata: Vec<usize> = Vec::new();
        for item in &items_to_revert {
            if matches!(item, HistoryItem::TimeStackPop { .. }) {
                affected_automata.push(automaton_id);
            }
        }

        // 逆序执行 revert（最后入栈的最先还原）
        for item in items_to_revert.iter().rev() {
            match item {
                HistoryItem::InputGraphGoEdge {
                    pointer_before_go,
                    accept_before_go,
                    stack_respond_before_go,
                    export_state_before_go,
                    ..
                } => {
                    // 经 invoke_native_method_on 调用 InputGraph.revert_go_edge
                    // Instance method 5 = revert_go_edge
                    ctx.set_int_param(0, *pointer_before_go as i64);
                    ctx.set_int_param(1, if *accept_before_go { 1 } else { 0 });
                    ctx.set_int_param(2, if *stack_respond_before_go { 1 } else { 0 });
                    ctx.set_string_param(0, export_state_before_go.clone());
                    ctx.invoke_native_method_on("GorgeFramework.InputGraph", input_graph_id, 5);
                }
                HistoryItem::TimeStackPush { .. } => {
                    // TimeStack.revert_push (method 6)
                    ctx.invoke_native_method_on("GorgeFramework.TimeStack", time_stack_id, 6);
                }
                HistoryItem::TimeStackPop {
                    time_item_id,
                    accept_before_pop,
                    respond_mode_before_pop,
                    ..
                } => {
                    // TimeStack.revert_pop(time_item, accept_before, respond_mode_before) (method 5)
                    ctx.set_object_param(0, *time_item_id);
                    ctx.set_bool_param(0, *accept_before_pop);
                    ctx.set_string_param(0, respond_mode_before_pop.clone());
                    ctx.invoke_native_method_on("GorgeFramework.TimeStack", time_stack_id, 5);
                }
            }
        }

        // 将受影响的自动机 ID 列表封装为 ObjectArray 返回（空列表返回 0）
        if affected_automata.is_empty() {
            0
        } else {
            let arr_id = ObjectArrayClass.do_construct_native(ctx, None, 0);
            if let Some(payload) = ctx.vm.native_payloads.get_mut(&arr_id) {
                use gorge_core::system::native::array::ObjectArray;
                if let Some(arr) = payload.downcast_mut::<ObjectArray>() {
                    arr.items = affected_automata;
                }
            }
            arr_id
        }
    }

    /// 栈深度
    #[gorge_method]
    pub fn len(ctx: &mut NativeContext, this: usize) -> i32 {
        with_payload(ctx, this, |p| p.stack.len() as i32)
    }
}

// ==================== 辅助函数 ====================

fn with_payload<T>(ctx: &NativeContext, this: usize, f: impl FnOnce(&HistoryStackPayload) -> T) -> T {
    let default = HistoryStackPayload { stack: Vec::new() };
    let payload = ctx.get_payload::<HistoryStackPayload>(this).unwrap_or(&default);
    f(payload)
}

fn with_payload_mut(ctx: &mut NativeContext, this: usize, f: impl FnOnce(&mut HistoryStackPayload)) {
    if let Some(payload) = ctx.get_payload_mut::<HistoryStackPayload>(this) {
        f(payload);
    }
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    struct Fixture {
        vm: VirtualMachine,
    }

    impl Fixture {
        fn new() -> Self { Self { vm: VirtualMachine::new() } }
        fn ctx(&mut self) -> NativeContext<'_> { NativeContext::new(&mut self.vm) }
    }

    #[test]
    fn test_history_stack_construct() {
        let hs = HistoryStack { _placeholder: false };
        let mut fx = Fixture::new();
        let id = { let mut ctx = fx.ctx(); hs.do_construct_native(&mut ctx, None, 0) };
        assert!(fx.ctx().has_payload(id));
        assert_eq!(with_payload(&fx.ctx(), id, |p| p.stack.len()), 0);
    }

    #[test]
    fn test_history_stack_push_and_revert_time() {
        let hs = HistoryStack { _placeholder: false };
        let mut fx = Fixture::new();
        let id = { let mut ctx = fx.ctx(); hs.do_construct_native(&mut ctx, None, 0) };
        // push_input_graph_go_edge(chart_time=1.0, pointer=0, accept=true, stack_respond=false, export_state="")
        fx.vm.param_pool.set_float_param(0, 1.0);
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_string_param(0, String::new());
        { let mut ctx = fx.ctx(); hs.invoke_native_method(&mut ctx, id, 1); }
        // revert_time 应返回 1.0
        { let mut ctx = fx.ctx(); hs.invoke_native_method(&mut ctx, id, 0); }
        assert!((fx.vm.param_pool.get_float_return() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_history_stack_push_types() {
        let hs = HistoryStack { _placeholder: false };
        let mut fx = Fixture::new();
        let id = { let mut ctx = fx.ctx(); hs.do_construct_native(&mut ctx, None, 0) };
        // push_input_graph_go_edge(chart_time=1.0, ...)
        fx.vm.param_pool.set_float_param(0, 1.0);
        fx.vm.param_pool.set_int_param(0, 0);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_bool_param(1, false);
        fx.vm.param_pool.set_string_param(0, "Waiting".to_string());
        { let mut ctx = fx.ctx(); hs.invoke_native_method(&mut ctx, id, 1); }
        // push_time_stack_push(chart_time=2.0)
        fx.vm.param_pool.set_float_param(0, 2.0);
        { let mut ctx = fx.ctx(); hs.invoke_native_method(&mut ctx, id, 2); }
        // push_time_stack_pop(chart_time=3.0, time_item=100, ...)
        fx.vm.param_pool.set_float_param(0, 3.0);
        fx.vm.param_pool.set_object_param(0, 100);
        fx.vm.param_pool.set_bool_param(0, false);
        fx.vm.param_pool.set_string_param(0, "mode".to_string());
        { let mut ctx = fx.ctx(); hs.invoke_native_method(&mut ctx, id, 3); }
        // 验证栈深 = 3
        { let mut ctx = fx.ctx(); hs.invoke_native_method(&mut ctx, id, 5); }
        assert_eq!(fx.vm.param_pool.get_int_return(), 3);
    }

    #[test]
    fn test_history_stack_pop_until() {
        let hs = HistoryStack { _placeholder: false };
        let mut fx = Fixture::new();
        let id = { let mut ctx = fx.ctx(); hs.do_construct_native(&mut ctx, None, 0) };
        // push 3 items at times 1.0, 2.0, 3.0
        for t in [1.0f32, 2.0, 3.0] {
            fx.vm.param_pool.set_float_param(0, t as f64);
            { let mut ctx = fx.ctx(); hs.invoke_native_method(&mut ctx, id, 2); } // push_time_stack_push
        }
        // pop_until(2.0) 应弹出 time >= 2.0 的项，剩余 time=1.0 的项
        fx.vm.param_pool.set_float_param(0, 2.0);
        fx.vm.param_pool.set_object_param(0, 0); // automaton_id
        fx.vm.param_pool.set_int_param(0, 0); // direction
        fx.vm.param_pool.set_object_param(1, 0); // input_graph_id
        fx.vm.param_pool.set_object_param(2, 0); // time_stack_id
        { let mut ctx = fx.ctx(); hs.invoke_native_method(&mut ctx, id, 4); }
        // 弹出的 3 项均为 TimeStackPush（非 TimeStackPop），受影响自动机列表应为空 → 返回 0
        assert_eq!(fx.vm.param_pool.get_object_return(), 0);
        { let mut ctx = fx.ctx(); hs.invoke_native_method(&mut ctx, id, 5); }
        assert_eq!(fx.vm.param_pool.get_int_return(), 1); // 剩余 1 项

        // revert_time = 1.0
        { let mut ctx = fx.ctx(); hs.invoke_native_method(&mut ctx, id, 0); }
    }

    #[test]
    fn test_history_stack_pop_until_affected_time_stack_pop() {
        let hs = HistoryStack { _placeholder: false };
        let mut fx = Fixture::new();
        let id = { let mut ctx = fx.ctx(); hs.do_construct_native(&mut ctx, None, 0) };
        // push 2 个 TimeStackPop 历史项（弹栈历史），时间为 1.0、2.0
        for (t, item) in [(1.0f32, 100usize), (2.0, 200)] {
            fx.vm.param_pool.set_float_param(0, t as f64);
            fx.vm.param_pool.set_object_param(0, item);
            fx.vm.param_pool.set_bool_param(0, false);
            fx.vm.param_pool.set_string_param(0, "mode".to_string());
            { let mut ctx = fx.ctx(); hs.invoke_native_method(&mut ctx, id, 3); } // push_time_stack_pop
        }
        // pop_until(1.5) 弹出 time >= 1.5 的项（仅 time=2.0 的 TimeStackPop），
        // 每个 TimeStackPop 记录一次受影响自动机 → 列表含 1 个 automaton_id=77
        fx.vm.param_pool.set_float_param(0, 1.5);
        fx.vm.param_pool.set_object_param(0, 77); // automaton_id
        fx.vm.param_pool.set_int_param(0, 0); // direction
        fx.vm.param_pool.set_object_param(1, 0); // input_graph_id
        fx.vm.param_pool.set_object_param(2, 0); // time_stack_id
        { let mut ctx = fx.ctx(); hs.invoke_native_method(&mut ctx, id, 4); }
        let arr_id = fx.vm.param_pool.get_object_return();
        assert_ne!(arr_id, 0, "含 TimeStackPop 时 pop_until 应返回非空受影响数组 ID");
        let items = fx.ctx().object_array_items(arr_id);
        assert_eq!(items, vec![77], "受影响自动机列表应包含 1 个 automaton_id=77（纯 ID，无标志位）");
    }
}
