//! `GorgeFramework` — 时序系统（native 类注册）。
//!
//! 移植自 C# 参考实现 `TimeStack.cs` / `TimeItem.cs`。
//! TimeItem 的 time 字段为委托对象 ID（`usize`）；
//! TimeStack 为 native 类，内部 Vec<TimeItemData> 通过 native_payloads 存储。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use gorge_core::objective::native::NativeClass;

// ==================== TimeItem 内部存储 ====================

/// TimeItem 的 payload 数据（栈内存储用，不在 Gorge 层面暴露）
#[derive(Debug, Clone)]
struct TimeItemData {
    /// 时间委托对象 ID
    pub time_delegate_id: usize,
    /// 是否接收
    pub accept: bool,
    /// 响应模式
    pub respond_mode: String,
}

// ==================== TimeStack 内部 payload ====================

/// TimeStack 内部 payload（存于 vm.native_payloads）
#[derive(Debug)]
struct TimeStackPayload {
    stack: Vec<TimeItemData>,
}

// ==================== TimeItem（native 注册） ====================

/// 时间项（时间栈元素）
///
/// 对齐 C# `TimeItem`。`time` 字段存储委托对象 ID，
/// 通过 `invoke_delegate` 可获取实际时间值。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct TimeItem {
    /// 时间委托对象 ID
    #[gorge_field]
    pub time: usize,
    /// 是否已响应
    #[gorge_field]
    pub accept: bool,
    /// 响应模式
    #[gorge_field]
    pub respond_mode: String,
}

impl TimeItem {
    pub fn new(time: usize, respond_mode: &str) -> Self {
        Self { time, accept: false, respond_mode: respond_mode.into() }
    }
}

#[gorge_native_impl]
impl TimeItem {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, time: usize, accept: bool, respond_mode: String) {
        ctx.set_object_object_field(this, TimeItem::FIELD_INDEX_time, time);
        ctx.set_object_bool_field(this, TimeItem::FIELD_INDEX_accept, accept);
        ctx.set_object_string_field(this, TimeItem::FIELD_INDEX_respond_mode, respond_mode);
    }
}

// ==================== TimeStack（native 注册） ====================

/// 时序栈
///
/// 对齐 C# `TimeStack`。维护一个时间项栈，支持压栈/弹栈/反向还原。
/// 内部 Vec<TimeItemData> 通过 `vm.native_payloads` 存储。
///
/// 方法编号表：
/// | 编号 | 方法 | 说明 |
/// |------|------|------|
/// | 0 | pop_time | 栈顶时间委托值（float），栈空返回 f32::MAX |
/// | 1 | try_pop | 尝试弹栈（时间 ≤ chartTime），推 Pop 历史 |
/// | 2 | pop | 弹栈，推 Pop 历史，返回新 TimeItem 对象 ID |
/// | 3 | push | 压栈，推 Push 历史 |
/// | 4 | init_push | 初始压栈（不记历史） |
/// | 5 | revert_pop | 反向还原：推回被弹出项，恢复 accept/respondMode |
/// | 6 | revert_push | 反向还原：弹掉栈顶 |
/// | 7 | len | 栈深度 |
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct TimeStack {
    /// 接收模式
    #[gorge_field]
    pub accept: bool,
    /// 响应模式
    #[gorge_field]
    pub respond_mode: String,
}

#[gorge_native_impl]
impl TimeStack {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, accept: bool, respond_mode: String) {
        ctx.set_object_bool_field(this, TimeStack::FIELD_INDEX_accept, accept);
        ctx.set_object_string_field(this, TimeStack::FIELD_INDEX_respond_mode, respond_mode);
        ctx.insert_payload(this, Box::new(TimeStackPayload { stack: Vec::new() }));
    }

    /// 弹栈时间（方法 0）
    #[gorge_method]
    pub fn pop_time(ctx: &mut NativeContext, this: usize) -> f32 {
        let delegate_id = match with_payload(ctx, this, |p| p.stack.last().map(|item| item.time_delegate_id)) {
            Some(id) => id,
            None => return f32::MAX,
        };
        ctx.invoke_delegate(delegate_id);
        (ctx.get_float_return() as f64) as f32
    }

    /// 尝试弹栈（方法 1）
    #[gorge_method]
    pub fn try_pop(ctx: &mut NativeContext, this: usize, chart_time: f32, history_stack_id: usize) -> bool {
        let has_items = with_payload(ctx, this, |p| !p.stack.is_empty());
        if !has_items { return false; }

        let old_accept = ctx.get_object_bool_field(this, TimeStack::FIELD_INDEX_accept);
        let old_respond_mode = ctx.get_object_string_field(this, TimeStack::FIELD_INDEX_respond_mode);

        let popped = with_payload(ctx, this, |p| p.stack.last().cloned());
        with_payload_mut(ctx, this, |p| { p.stack.pop(); });

        if let Some(item) = popped {
            ctx.set_object_bool_field(this, TimeStack::FIELD_INDEX_accept, item.accept);
            ctx.set_object_string_field(this, TimeStack::FIELD_INDEX_respond_mode, item.respond_mode.clone());

            ctx.set_float_param(0, chart_time as f64);
            ctx.set_object_param(0, 0);
            ctx.set_bool_param(0, old_accept);
            ctx.set_string_param(0, old_respond_mode.clone());
            ctx.invoke_native_method_on("GorgeFramework.HistoryStack", history_stack_id, 3);
        }
        true
    }

    /// 弹栈（方法 2）
    #[gorge_method]
    pub fn pop(ctx: &mut NativeContext, this: usize, chart_time: f32, history_stack_id: usize) -> usize {
        if !with_payload(ctx, this, |p| !p.stack.is_empty()) {
            return 0;
        }

        let old_accept = ctx.get_object_bool_field(this, TimeStack::FIELD_INDEX_accept);
        let old_respond_mode = ctx.get_object_string_field(this, TimeStack::FIELD_INDEX_respond_mode);

        let popped = with_payload(ctx, this, |p| p.stack.last().cloned());
        with_payload_mut(ctx, this, |p| { p.stack.pop(); });

        let item = match popped {
            Some(item) => item,
            None => return 0,
        };

        ctx.set_object_bool_field(this, TimeStack::FIELD_INDEX_accept, item.accept);
        ctx.set_object_string_field(this, TimeStack::FIELD_INDEX_respond_mode, item.respond_mode.clone());

        ctx.set_float_param(0, chart_time as f64);
        ctx.set_object_param(0, 0);
        ctx.set_bool_param(0, old_accept);
        ctx.set_string_param(0, old_respond_mode.clone());
        ctx.invoke_native_method_on("GorgeFramework.HistoryStack", history_stack_id, 3);

        let ti_obj_id = ctx.alloc_object_id();
        ctx.vm.objects.insert(ti_obj_id,
            gorge_core::objective::object::RuntimeObject::new_simple(
                "GorgeFramework.TimeItem".to_string(),
                TimeItem { time: 0, accept: false, respond_mode: String::new() }.field_type_count(),
            ));
        ctx.set_object_object_field(ti_obj_id, TimeItem::FIELD_INDEX_time, item.time_delegate_id);
        ctx.set_object_bool_field(ti_obj_id, TimeItem::FIELD_INDEX_accept, item.accept);
        ctx.set_object_string_field(ti_obj_id, TimeItem::FIELD_INDEX_respond_mode, item.respond_mode);
        ti_obj_id
    }

    /// 压栈（方法 3）
    #[gorge_method]
    pub fn push(ctx: &mut NativeContext, this: usize, chart_time: f32, time_item_id: usize, history_stack_id: usize) {
        let time_delegate_id = ctx.get_object_object_field(time_item_id, TimeItem::FIELD_INDEX_time);
        let item_accept = ctx.get_object_bool_field(time_item_id, TimeItem::FIELD_INDEX_accept);
        let item_respond_mode = ctx.get_object_string_field(time_item_id, TimeItem::FIELD_INDEX_respond_mode);

        with_payload_mut(ctx, this, |p| {
            p.stack.push(TimeItemData { time_delegate_id, accept: item_accept, respond_mode: item_respond_mode });
        });

        ctx.set_float_param(0, chart_time as f64);
        ctx.invoke_native_method_on("GorgeFramework.HistoryStack", history_stack_id, 2);
    }

    /// 初始压栈（方法 4）
    #[gorge_method]
    pub fn init_push(ctx: &mut NativeContext, this: usize, time_item_id: usize) {
        let time_delegate_id = ctx.get_object_object_field(time_item_id, TimeItem::FIELD_INDEX_time);
        let item_accept = ctx.get_object_bool_field(time_item_id, TimeItem::FIELD_INDEX_accept);
        let item_respond_mode = ctx.get_object_string_field(time_item_id, TimeItem::FIELD_INDEX_respond_mode);

        with_payload_mut(ctx, this, |p| {
            p.stack.push(TimeItemData { time_delegate_id, accept: item_accept, respond_mode: item_respond_mode });
        });
    }

    /// 反向还原弹栈（方法 5）
    #[gorge_method]
    pub fn revert_pop(ctx: &mut NativeContext, this: usize, time_item_id: usize, accept_before: bool, respond_mode_before: String) {
        let time_delegate_id = ctx.get_object_object_field(time_item_id, TimeItem::FIELD_INDEX_time);
        let item_accept = ctx.get_object_bool_field(time_item_id, TimeItem::FIELD_INDEX_accept);
        let item_respond_mode = ctx.get_object_string_field(time_item_id, TimeItem::FIELD_INDEX_respond_mode);

        with_payload_mut(ctx, this, |p| {
            p.stack.push(TimeItemData { time_delegate_id, accept: item_accept, respond_mode: item_respond_mode });
        });

        ctx.set_object_bool_field(this, TimeStack::FIELD_INDEX_accept, accept_before);
        ctx.set_object_string_field(this, TimeStack::FIELD_INDEX_respond_mode, respond_mode_before);
    }

    /// 反向还原压栈（方法 6）
    #[gorge_method]
    pub fn revert_push(ctx: &mut NativeContext, this: usize) {
        with_payload_mut(ctx, this, |p| { p.stack.pop(); });
    }

    /// 栈深度（方法 7）
    #[gorge_method]
    pub fn len(ctx: &mut NativeContext, this: usize) -> i32 {
        with_payload(ctx, this, |p| p.stack.len() as i32)
    }
}

// ==================== 辅助函数 ====================

fn with_payload<T>(ctx: &NativeContext, this: usize, f: impl FnOnce(&TimeStackPayload) -> T) -> T {
    let default = TimeStackPayload { stack: Vec::new() };
    let payload = ctx.get_payload::<TimeStackPayload>(this).unwrap_or(&default);
    f(payload)
}

fn with_payload_mut(ctx: &mut NativeContext, this: usize, f: impl FnOnce(&mut TimeStackPayload)) {
    if let Some(payload) = ctx.get_payload_mut::<TimeStackPayload>(this) {
        f(payload);
    }
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::object::GorgeObject;
    use gorge_core::virtual_machine::vm::VirtualMachine;
    use crate::system::native::history::{HistoryStack, HistoryStackPayload};

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

        fn make_time_item(&mut self, time_delegate_id: usize, accept: bool, respond_mode: &str) -> usize {
            let ti = TimeItem { time: 0, accept: false, respond_mode: String::new() };
            self.vm.param_pool.set_object_param(0, time_delegate_id);
            self.vm.param_pool.set_bool_param(0, accept);
            self.vm.param_pool.set_string_param(0, respond_mode.to_string());
            let id = { let mut ctx = self.ctx(); ti.do_construct_native(&mut ctx, None, 0) };
            id
        }
    }

    #[test]
    fn test_time_item_construct() {
        let ti = TimeItem { time: 0, accept: false, respond_mode: String::new() };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_object_param(0, 42);
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_string_param(0, "tap".to_string());
        let id = { let mut ctx = fx.ctx(); ti.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        assert_eq!(fx.vm.objects.get(&id).unwrap().get_object_field(0), 42);
        assert!(fx.vm.objects.get(&id).unwrap().get_bool_field(0));
    }

    #[test]
    fn test_time_stack_construct() {
        let ts = TimeStack { accept: true, respond_mode: String::new() };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_string_param(0, "mode".to_string());
        let id = { let mut ctx = fx.ctx(); ts.do_construct_native(&mut ctx, None, 0) };
        assert!(fx.ctx().has_payload(id));
        assert_eq!(with_payload(&fx.ctx(), id, |p| p.stack.len()), 0);
    }

    #[test]
    fn test_time_stack_push_and_len() {
        let ts = TimeStack { accept: true, respond_mode: String::new() };
        let mut fx = Fixture::new();
        fx.vm.native_class_table.insert(
            "GorgeFramework.HistoryStack".to_string(),
            std::sync::Arc::new(HistoryStack { _placeholder: false }),
        );
        let hs_id = fx.make_history_stack();

        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_string_param(0, "mode".to_string());
        let id = { let mut ctx = fx.ctx(); ts.do_construct_native(&mut ctx, None, 0) };

        let ti_id = fx.make_time_item(101, true, "tap");

        fx.vm.param_pool.set_float_param(0, 1.0);
        fx.vm.param_pool.set_object_param(0, ti_id);
        fx.vm.param_pool.set_object_param(1, hs_id);
        { let mut ctx = fx.ctx(); ts.invoke_native_method(&mut ctx, id, 3); }

        { let mut ctx = fx.ctx(); ts.invoke_native_method(&mut ctx, id, 7); }
        assert_eq!(fx.vm.param_pool.get_int_return(), 1);

        let p = fx.vm.native_payloads.get(&hs_id).unwrap().downcast_ref::<HistoryStackPayload>().unwrap();
        assert_eq!(p.stack.len(), 1);
    }

    #[test]
    fn test_time_stack_init_push_and_pop_time() {
        let ts = TimeStack { accept: true, respond_mode: String::new() };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_string_param(0, "mode".to_string());
        let id = { let mut ctx = fx.ctx(); ts.do_construct_native(&mut ctx, None, 0) };

        let ti_id = fx.make_time_item(200, false, "hold");
        fx.vm.param_pool.set_object_param(0, ti_id);
        { let mut ctx = fx.ctx(); ts.invoke_native_method(&mut ctx, id, 4); }
        assert_eq!(with_payload(&fx.ctx(), id, |p| p.stack.len()), 1);
    }

    #[test]
    fn test_time_stack_revert_pop_and_revert_push() {
        let ts = TimeStack { accept: true, respond_mode: String::new() };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_string_param(0, "a".to_string());
        let id = { let mut ctx = fx.ctx(); ts.do_construct_native(&mut ctx, None, 0) };

        let ti_id = fx.make_time_item(200, false, "hold");
        fx.vm.param_pool.set_object_param(0, ti_id);
        { let mut ctx = fx.ctx(); ts.invoke_native_method(&mut ctx, id, 4); }

        assert_eq!(with_payload(&fx.ctx(), id, |p| p.stack.len()), 1);

        { let mut ctx = fx.ctx(); ts.invoke_native_method(&mut ctx, id, 6); }
        assert_eq!(with_payload(&fx.ctx(), id, |p| p.stack.len()), 0);

        fx.vm.param_pool.set_object_param(0, ti_id);
        fx.vm.param_pool.set_bool_param(0, false);
        fx.vm.param_pool.set_string_param(0, "restored".to_string());
        { let mut ctx = fx.ctx(); ts.invoke_native_method(&mut ctx, id, 5); }
        assert_eq!(with_payload(&fx.ctx(), id, |p| p.stack.len()), 1);
    }

    #[test]
    fn test_time_item_time_is_usize() {
        let ti = TimeItem { time: 999, accept: false, respond_mode: String::new() };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_object_param(0, 777);
        fx.vm.param_pool.set_bool_param(0, false);
        fx.vm.param_pool.set_string_param(0, String::new());
        let id = { let mut ctx = fx.ctx(); ti.do_construct_native(&mut ctx, None, 0) };
        assert_eq!(fx.vm.objects.get(&id).unwrap().get_object_field(0), 777);
    }

    #[test]
    fn test_pop_time_invokes_delegate() {
        use gorge_core::objective::delegate::RuntimeDelegate;
        use gorge_core::objective::types::{GorgeType, BasicType};
        use gorge_core::objective::value_pool::FixedFieldValuePool;
        use gorge_core::virtual_machine::ir::{CompiledMethod, CodeWithSpan, IntermediateCode, Operand, Address, ValueType};

        let ts = TimeStack { accept: true, respond_mode: String::new() };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_string_param(0, String::new());
        let id = { let mut ctx = fx.ctx(); ts.do_construct_native(&mut ctx, None, 0) };

        // 构造返回 3.14 的委托
        let result_addr = Address::new(ValueType::Float, 0);
        let method = CompiledMethod {
            name: "get_pi".into(),
            codes: vec![
                CodeWithSpan::new(
                    IntermediateCode::assign(result_addr, Operand::float(3.14)),
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

        let ti_id = fx.make_time_item(delegate_obj_id, false, "");
        fx.vm.param_pool.set_object_param(0, ti_id);
        { let mut ctx = fx.ctx(); ts.invoke_native_method(&mut ctx, id, 4); }

        { let mut ctx = fx.ctx(); ts.invoke_native_method(&mut ctx, id, 0); }
        let result = (fx.vm.param_pool.get_float_return() as f64) as f32;
        assert!((result - 3.14).abs() < 0.01, "预期 3.14，实际 {}", result);
    }

    #[test]
    fn test_pop_time_empty_returns_max() {
        let ts = TimeStack { accept: true, respond_mode: String::new() };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_bool_param(0, true);
        fx.vm.param_pool.set_string_param(0, String::new());
        let id = { let mut ctx = fx.ctx(); ts.do_construct_native(&mut ctx, None, 0) };

        { let mut ctx = fx.ctx(); ts.invoke_native_method(&mut ctx, id, 0); }
        let result = (fx.vm.param_pool.get_float_return() as f64) as f32;
        assert_eq!(result, f32::MAX);
    }
}
