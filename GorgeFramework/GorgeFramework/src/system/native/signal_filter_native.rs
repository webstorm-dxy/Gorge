//! `GorgeFramework.SignalFilter` —— 信号过滤器基类 native 类。
//!
//! 移植自 C# 参考实现 `SignalFilter.cs`。
//! 字段布局对齐 C# 构造顺序：priority、conditionTypes、endTime、timeMode、acceptConsume、denyConsume。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 信号过滤器基类
///
/// 对齐 C# `SignalFilter`。本类在 C# 中为 abstract（CanDetect/Detect 抛异常），
/// Rust 侧 can_detect 返回 false，detect 返回 false 作为默认实现。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct SignalFilter {
    /// 优先级委托（对象 ID）
    #[gorge_field]
    pub priority: usize,
    /// 检测条件类型（IntArray 对象 ID）
    #[gorge_field]
    pub condition_types: usize,
    /// 截止时间委托（对象 ID）
    #[gorge_field]
    pub end_time: usize,
    /// 时间模式：0=CatchBefore（超时拒绝），1=KeepUntil（超时接受）
    #[gorge_field]
    pub time_mode: i32,
    /// 接受时是否消耗信号
    #[gorge_field]
    pub accept_consume: bool,
    /// 拒绝时是否消耗信号
    #[gorge_field]
    pub deny_consume: bool,
}

#[gorge_native_impl]
impl SignalFilter {
    #[gorge_ctor]
    pub fn new_ctor(
        ctx: &mut NativeContext,
        this: usize,
        priority: usize,
        condition_types: usize,
        end_time: usize,
        time_mode: i32,
        accept_consume: bool,
        deny_consume: bool,
    ) {
        ctx.set_object_object_field(this, SignalFilter::FIELD_INDEX_priority, priority);
        ctx.set_object_object_field(this, SignalFilter::FIELD_INDEX_condition_types, condition_types);
        ctx.set_object_object_field(this, SignalFilter::FIELD_INDEX_end_time, end_time);
        ctx.set_object_int_field(this, SignalFilter::FIELD_INDEX_time_mode, time_mode as i64);
        ctx.set_object_bool_field(this, SignalFilter::FIELD_INDEX_accept_consume, accept_consume);
        ctx.set_object_bool_field(this, SignalFilter::FIELD_INDEX_deny_consume, deny_consume);
    }

    /// 检测能否处理指定信道（基类默认实现返回 false，对齐 C# abstract 语义）
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn can_detect(ctx: &mut NativeContext, this: usize, channel_name: String) -> bool {
        false
    }

    /// 检测信号（基类默认实现返回 false，对齐 C# abstract 语义）
    /// 注意：C# 中 detect 参数为 (channelName, signalId, conditionType, signalValue, lastSignalValue)，
    /// 由于宏限制暂以 bare 方法存在，子类通过 invoke_native_method_on 调用时需自行匹配参数。
    #[allow(unused_variables)]
    pub fn detect_base(
        ctx: &mut NativeContext,
        this: usize,
        _signal_id: i32,
        _condition_type: i32,
        _signal_value: usize,
        _last_signal_value: usize,
    ) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::object::GorgeObject;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    #[test]
    fn test_signal_filter_construct() {
        let sf = SignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
        };
        let mut vm = VirtualMachine::new();
        vm.param_pool.set_object_param(0, 10);  // priority
        vm.param_pool.set_object_param(1, 20);  // condition_types
        vm.param_pool.set_object_param(2, 30);  // end_time
        vm.param_pool.set_int_param(0, 1);      // time_mode
        vm.param_pool.set_bool_param(0, true);  // accept_consume
        vm.param_pool.set_bool_param(1, false); // deny_consume
        let id = { let mut ctx = NativeContext::new(&mut vm); sf.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        let obj = vm.objects.get(&id).unwrap();
        assert_eq!(obj.get_object_field(0), 10); // priority
        assert_eq!(obj.get_object_field(1), 20); // condition_types
        assert_eq!(obj.get_object_field(2), 30); // end_time
        assert_eq!(obj.get_int_field(0), 1);
        assert!(obj.get_bool_field(0));
        assert!(!obj.get_bool_field(1));
    }

    #[test]
    fn test_signal_filter_can_detect_returns_false() {
        let sf = SignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
        };
        let mut vm = VirtualMachine::new();
        for i in 0..3 { vm.param_pool.set_object_param(i, 0); }
        vm.param_pool.set_int_param(0, 0);
        vm.param_pool.set_bool_param(0, true);
        vm.param_pool.set_bool_param(1, false);
        let id = { let mut ctx = NativeContext::new(&mut vm); sf.do_construct_native(&mut ctx, None, 0) };
        // can_detect(channel_name) — 基类始终返回 false
        vm.param_pool.set_string_param(0, "speed".to_string());
        { let mut ctx = NativeContext::new(&mut vm); sf.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return());
    }
}
