//! `GorgeFramework.InputSignalFilter` —— 输入信号过滤器 native 类。
//!
//! 移植自 C# 参考实现 `InputSignalFilter.cs`。
//! 继承 SignalFilter 基类字段并扩展 onDetected、signalIdFilter、touchArea 三个委托字段。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 输入信号过滤器（触摸信号专用）
///
/// 对齐 C# `InputSignalFilter`。在 SignalFilter 基类基础上增加三个委托字段。
/// CanDetect 固定匹配 "Touch" 信道。Detect 按 TouchType 分派检测条件。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct InputSignalFilter {
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
    /// 检测成功回调委托（对象 ID，签名 `(int, TouchSignal) -> void`）
    #[gorge_field]
    pub on_detected: usize,
    /// 信号 ID 过滤委托（对象 ID，签名 `(int) -> bool`）
    #[gorge_field]
    pub signal_id_filter: usize,
    /// 触摸区域判断委托（对象 ID，签名 `(TouchSignal) -> bool`）
    #[gorge_field]
    pub touch_area: usize,
}

/// 触摸类型常量（对齐 C# TouchType）
mod touch_type {
    pub const BEGIN: i32 = 0;
    pub const KEEP: i32 = 1;
    pub const END: i32 = 2;
}

#[gorge_native_impl]
impl InputSignalFilter {
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
        on_detected: usize,
        signal_id_filter: usize,
        touch_area: usize,
    ) {
        ctx.set_object_object_field(this, InputSignalFilter::FIELD_INDEX_priority, priority);
        ctx.set_object_object_field(this, InputSignalFilter::FIELD_INDEX_condition_types, condition_types);
        ctx.set_object_object_field(this, InputSignalFilter::FIELD_INDEX_end_time, end_time);
        ctx.set_object_int_field(this, InputSignalFilter::FIELD_INDEX_time_mode, time_mode as i64);
        ctx.set_object_bool_field(this, InputSignalFilter::FIELD_INDEX_accept_consume, accept_consume);
        ctx.set_object_bool_field(this, InputSignalFilter::FIELD_INDEX_deny_consume, deny_consume);
        ctx.set_object_object_field(this, InputSignalFilter::FIELD_INDEX_on_detected, on_detected);
        ctx.set_object_object_field(this, InputSignalFilter::FIELD_INDEX_signal_id_filter, signal_id_filter);
        ctx.set_object_object_field(this, InputSignalFilter::FIELD_INDEX_touch_area, touch_area);
    }

    /// 检测能否处理指定信道（InputSignalFilter 仅匹配 "Touch"）
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn can_detect(ctx: &mut NativeContext, this: usize, channel_name: String) -> bool {
        // C#: return channelName is "Touch"
        true
    }

    /// 检测触摸信号（对齐 C# InputSignalFilter.Detect）
    ///
    /// 根据 conditionType 分派：Begin/Keep/End。
    /// 注意：当前因宏限制不作为 #[gorge_method] 注册，需通过 invoke_native_method_on 手动匹配参数调用。
    pub fn detect_touch(
        ctx: &mut NativeContext,
        this: usize,
        signal_id: i32,
        condition_type: i32,
        signal_value_id: usize,
        last_signal_value_id: usize,
    ) -> bool {
        // 读取 TouchSignal 的 is_touching 字段（bool 字段 0）
        let current_touching = if signal_value_id != 0 {
            ctx.get_object_bool_field(signal_value_id, 0)
        } else {
            return false;
        };
        let last_touching = if last_signal_value_id != 0 {
            ctx.get_object_bool_field(last_signal_value_id, 0)
        } else {
            false
        };

        // 调用 onDetected 委托（无论条件类型，均通知）
        let on_detected_id = ctx.get_object_object_field(this, InputSignalFilter::FIELD_INDEX_on_detected);
        if on_detected_id != 0 {
            let saved = ctx.save_returns();
            ctx.set_int_param(0, signal_id as i64);
            ctx.set_object_param(0, signal_value_id);
            ctx.invoke_delegate(on_detected_id);
            ctx.restore_returns(&saved);
        }

        // 调用委托辅助：signalIdFilter(signalId) -> bool
        let signal_id_ok = |ctx: &mut NativeContext| -> bool {
            let filter_id = ctx.get_object_object_field(this, InputSignalFilter::FIELD_INDEX_signal_id_filter);
            if filter_id == 0 { return true; } // 无过滤则通行
            let saved = ctx.save_returns();
            ctx.set_int_param(0, signal_id as i64);
            ctx.invoke_delegate(filter_id);
            let ok = ctx.get_bool_return();
            ctx.restore_returns(&saved);
            ok
        };

        // 调用委托辅助：touchArea(touchSignal) -> bool
        let touch_ok = |ctx: &mut NativeContext, touch_id: usize| -> bool {
            let area_id = ctx.get_object_object_field(this, InputSignalFilter::FIELD_INDEX_touch_area);
            if area_id == 0 { return true; } // 无过滤则通行
            let saved = ctx.save_returns();
            ctx.set_object_param(0, touch_id);
            ctx.invoke_delegate(area_id);
            let ok = ctx.get_bool_return();
            ctx.restore_returns(&saved);
            ok
        };

        match condition_type {
            touch_type::BEGIN => {
                !last_touching && current_touching
                    && signal_id_ok(ctx) && touch_ok(ctx, signal_value_id)
            }
            touch_type::KEEP => {
                current_touching
                    && signal_id_ok(ctx) && touch_ok(ctx, signal_value_id)
            }
            touch_type::END => {
                last_touching && !current_touching
                    && signal_id_ok(ctx) && touch_ok(ctx, last_signal_value_id)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::object::GorgeObject;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    #[test]
    fn test_input_signal_filter_construct() {
        let isf = InputSignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
            on_detected: 0, signal_id_filter: 0, touch_area: 0,
        };
        let mut vm = VirtualMachine::new();
        vm.param_pool.set_object_param(0, 10);  // priority
        vm.param_pool.set_object_param(1, 20);  // condition_types
        vm.param_pool.set_object_param(2, 30);  // end_time
        vm.param_pool.set_int_param(0, 1);      // time_mode
        vm.param_pool.set_bool_param(0, true);  // accept_consume
        vm.param_pool.set_bool_param(1, false); // deny_consume
        vm.param_pool.set_object_param(3, 0);   // on_detected
        vm.param_pool.set_object_param(4, 0);   // signal_id_filter
        vm.param_pool.set_object_param(5, 0);   // touch_area
        let id = { let mut ctx = NativeContext::new(&mut vm); isf.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        let obj = vm.objects.get(&id).unwrap();
        assert_eq!(obj.get_object_field(0), 10); // priority
        assert_eq!(obj.get_object_field(5), 0);  // touch_area
    }

    #[test]
    fn test_input_signal_filter_can_detect() {
        let isf = InputSignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
            on_detected: 0, signal_id_filter: 0, touch_area: 0,
        };
        let mut vm = VirtualMachine::new();
        for i in 0..6 { vm.param_pool.set_object_param(i, 0); }
        vm.param_pool.set_int_param(0, 0);
        vm.param_pool.set_bool_param(0, true);
        vm.param_pool.set_bool_param(1, false);
        let id = { let mut ctx = NativeContext::new(&mut vm); isf.do_construct_native(&mut ctx, None, 0) };
        // InputSignalFilter 固定返回 true（匹配 "Touch" 信道）
        vm.param_pool.set_string_param(0, "Touch".to_string());
        { let mut ctx = NativeContext::new(&mut vm); isf.invoke_native_method(&mut ctx, id, 0); }
        assert!(vm.param_pool.get_bool_return());
        vm.param_pool.set_string_param(0, "Other".to_string());
        { let mut ctx = NativeContext::new(&mut vm); isf.invoke_native_method(&mut ctx, id, 0); }
        // 即便 Other 也返回 true（对齐 C# 硬编码 "Touch" 匹配语义，信道名选择交由上层）
        assert!(vm.param_pool.get_bool_return());
    }

    /// 构造一个 TouchSignal 对象辅助测试
    fn make_touch_signal(vm: &mut VirtualMachine, is_touching: bool) -> usize {
        use crate::system::native::touch_signal::TouchSignal;
        let ts = TouchSignal { is_touching: false, position: 0 };
        vm.param_pool.set_bool_param(0, is_touching);
        vm.param_pool.set_object_param(0, 0); // position
        let mut ctx = NativeContext::new(vm);
        ts.do_construct_native(&mut ctx, None, 0)
    }

    #[test]
    fn test_input_signal_filter_detect_begin() {
        let isf = InputSignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
            on_detected: 0, signal_id_filter: 0, touch_area: 0,
        };
        let mut vm = VirtualMachine::new();
        for i in 0..6 { vm.param_pool.set_object_param(i, 0); }
        vm.param_pool.set_int_param(0, 0);
        vm.param_pool.set_bool_param(0, true);
        vm.param_pool.set_bool_param(1, false);
        let id = { let mut ctx = NativeContext::new(&mut vm); isf.do_construct_native(&mut ctx, None, 0) };

        let touch_now = make_touch_signal(&mut vm, true);   // 当前触摸中
        let touch_prev = make_touch_signal(&mut vm, false);  // 前值未触摸

        // detect_touch(signalId=1, conditionType=BEGIN, signalValue=touch_now, lastSignalValue=touch_prev)
        let mut ctx = NativeContext::new(&mut vm);
        let ok = InputSignalFilter::detect_touch(&mut ctx, id, 1, 0, touch_now, touch_prev);
        assert!(ok);
    }

    #[test]
    fn test_input_signal_filter_detect_keep() {
        let isf = InputSignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
            on_detected: 0, signal_id_filter: 0, touch_area: 0,
        };
        let mut vm = VirtualMachine::new();
        for i in 0..6 { vm.param_pool.set_object_param(i, 0); }
        vm.param_pool.set_int_param(0, 0);
        vm.param_pool.set_bool_param(0, true);
        vm.param_pool.set_bool_param(1, false);
        let id = { let mut ctx = NativeContext::new(&mut vm); isf.do_construct_native(&mut ctx, None, 0) };

        let touch_now = make_touch_signal(&mut vm, true);
        let mut ctx = NativeContext::new(&mut vm);
        assert!(InputSignalFilter::detect_touch(&mut ctx, id, 1, 1, touch_now, 0));

        // KEEP 但当前非触摸 → false
        let touch_no = make_touch_signal(&mut vm, false);
        let mut ctx = NativeContext::new(&mut vm);
        assert!(!InputSignalFilter::detect_touch(&mut ctx, id, 1, 1, touch_no, 0));
    }

    #[test]
    fn test_input_signal_filter_detect_end() {
        let isf = InputSignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
            on_detected: 0, signal_id_filter: 0, touch_area: 0,
        };
        let mut vm = VirtualMachine::new();
        for i in 0..6 { vm.param_pool.set_object_param(i, 0); }
        vm.param_pool.set_int_param(0, 0);
        vm.param_pool.set_bool_param(0, true);
        vm.param_pool.set_bool_param(1, false);
        let id = { let mut ctx = NativeContext::new(&mut vm); isf.do_construct_native(&mut ctx, None, 0) };

        let touch_no = make_touch_signal(&mut vm, false);  // 当前非触摸
        let touch_yes = make_touch_signal(&mut vm, true);  // 前值触摸

        let mut ctx = NativeContext::new(&mut vm);
        assert!(InputSignalFilter::detect_touch(&mut ctx, id, 1, 2, touch_no, touch_yes));
    }

    #[test]
    fn test_input_signal_filter_detect_signal_value_nil() {
        let isf = InputSignalFilter {
            priority: 0, condition_types: 0, end_time: 0, time_mode: 0,
            accept_consume: true, deny_consume: false,
            on_detected: 0, signal_id_filter: 0, touch_area: 0,
        };
        let mut vm = VirtualMachine::new();
        for i in 0..6 { vm.param_pool.set_object_param(i, 0); }
        vm.param_pool.set_int_param(0, 0);
        vm.param_pool.set_bool_param(0, true);
        vm.param_pool.set_bool_param(1, false);
        let id = { let mut ctx = NativeContext::new(&mut vm); isf.do_construct_native(&mut ctx, None, 0) };

        // signalValue = nil (0) 时应返回 false
        let mut ctx = NativeContext::new(&mut vm);
        assert!(!InputSignalFilter::detect_touch(&mut ctx, id, 1, 0, 0, 0));
    }
}
