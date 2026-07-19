//! `GorgeFramework` — 时间项（native 数据类）。
//!
//! 移植自 C# 参考实现 `TimeItem.cs`。
//! TimeItem 的 time 字段为委托对象 ID（`usize`）。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// TimeItem 的 payload 数据（栈内存储用，不在 Gorge 层面暴露）
#[derive(Debug, Clone)]
pub(crate) struct TimeItemData {
    /// 时间委托对象 ID
    pub time_delegate_id: usize,
    /// 是否接收
    pub accept: bool,
    /// 响应模式
    pub respond_mode: String,
}

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
    fn test_time_item_time_is_usize() {
        let ti = TimeItem { time: 999, accept: false, respond_mode: String::new() };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_object_param(0, 777);
        fx.vm.param_pool.set_bool_param(0, false);
        fx.vm.param_pool.set_string_param(0, String::new());
        let id = { let mut ctx = fx.ctx(); ti.do_construct_native(&mut ctx, None, 0) };
        assert_eq!(fx.vm.objects.get(&id).unwrap().get_object_field(0), 777);
    }
}
