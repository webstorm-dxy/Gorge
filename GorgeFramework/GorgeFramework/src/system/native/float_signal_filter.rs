//! `GorgeFramework.FloatSignalFilter` —— 浮点信号过滤器 native 类。
//!
//! 移植自 C# 参考实现 `FloatSignalFilter.cs`。
//! 继承 SignalFilter 基类字段并扩展 channelName、filterRange。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 浮点信号过滤器
///
/// 对齐 C# `FloatSignalFilter`。在 SignalFilter 基类基础上增加信道名和浮点过滤范围委托。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct FloatSignalFilter {
    /// 优先级委托（对象 ID）
    #[gorge_field]
    pub priority: usize,
    /// 检测条件类型（IntArray 对象 ID）
    #[gorge_field]
    pub condition_types: usize,
    /// 截止时间委托（对象 ID）
    #[gorge_field]
    pub end_time: usize,
    /// 时间模式：0=CatchBefore，1=KeepUntil
    #[gorge_field]
    pub time_mode: i32,
    /// 接受时是否消耗信号
    #[gorge_field]
    pub accept_consume: bool,
    /// 拒绝时是否消耗信号
    #[gorge_field]
    pub deny_consume: bool,
    /// 监听的信道名
    #[gorge_field]
    pub channel_name: String,
    /// 范围过滤委托（对象 ID，签名 `(FloatSignal) -> bool`）
    #[gorge_field]
    pub filter_range: usize,
}

#[gorge_native_impl]
impl FloatSignalFilter {
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
        channel_name: String,
        filter_range: usize,
    ) {
        ctx.set_object_object_field(this, FloatSignalFilter::FIELD_INDEX_priority, priority);
        ctx.set_object_object_field(this, FloatSignalFilter::FIELD_INDEX_condition_types, condition_types);
        ctx.set_object_object_field(this, FloatSignalFilter::FIELD_INDEX_end_time, end_time);
        ctx.set_object_int_field(this, FloatSignalFilter::FIELD_INDEX_time_mode, time_mode as i64);
        ctx.set_object_bool_field(this, FloatSignalFilter::FIELD_INDEX_accept_consume, accept_consume);
        ctx.set_object_bool_field(this, FloatSignalFilter::FIELD_INDEX_deny_consume, deny_consume);
        ctx.set_object_string_field(this, FloatSignalFilter::FIELD_INDEX_channel_name, channel_name);
        ctx.set_object_object_field(this, FloatSignalFilter::FIELD_INDEX_filter_range, filter_range);
    }

    /// 检测能否处理指定信道（按 channelName 字段匹配）
    #[gorge_method]
    pub fn can_detect(ctx: &mut NativeContext, this: usize, channel_name: String) -> bool {
        let name = ctx.get_object_string_field(this, FloatSignalFilter::FIELD_INDEX_channel_name);
        name == channel_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::object::GorgeObject;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    #[test]
    fn test_float_signal_filter_construct() {
        let fsf = FloatSignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
            channel_name: String::new(), filter_range: 0,
        };
        let mut vm = VirtualMachine::new();
        vm.param_pool.set_object_param(0, 10);  // priority
        vm.param_pool.set_object_param(1, 20);  // condition_types
        vm.param_pool.set_object_param(2, 30);  // end_time
        vm.param_pool.set_int_param(0, 1);      // time_mode
        vm.param_pool.set_bool_param(0, true);  // accept_consume
        vm.param_pool.set_bool_param(1, false); // deny_consume
        vm.param_pool.set_string_param(0, "speed".to_string()); // channel_name
        vm.param_pool.set_object_param(3, 40);  // filter_range
        let id = { let mut ctx = NativeContext::new(&mut vm); fsf.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        let obj = vm.objects.get(&id).unwrap();
        assert_eq!(obj.get_object_field(0), 10);
        assert_eq!(obj.get_object_field(2), 30);
        assert_eq!(obj.get_string_field(0), "speed");
        assert_eq!(obj.get_object_field(3), 40);
    }

    #[test]
    fn test_float_signal_filter_can_detect() {
        let fsf = FloatSignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
            channel_name: String::new(), filter_range: 0,
        };
        let mut vm = VirtualMachine::new();
        for i in 0..4 { vm.param_pool.set_object_param(i, 0); }
        vm.param_pool.set_int_param(0, 0);
        vm.param_pool.set_bool_param(0, true);
        vm.param_pool.set_bool_param(1, false);
        vm.param_pool.set_string_param(0, "speed".to_string());
        let id = { let mut ctx = NativeContext::new(&mut vm); fsf.do_construct_native(&mut ctx, None, 0) };
        // can_detect("speed") → true
        vm.param_pool.set_string_param(0, "speed".to_string());
        { let mut ctx = NativeContext::new(&mut vm); fsf.invoke_native_method(&mut ctx, id, 0); }
        assert!(vm.param_pool.get_bool_return());
        // can_detect("other") → false
        vm.param_pool.set_string_param(0, "other".to_string());
        { let mut ctx = NativeContext::new(&mut vm); fsf.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return());
    }
}
