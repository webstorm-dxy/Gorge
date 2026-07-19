//! `GorgeFramework.PeriodConfig` —— 乐段设置 native 类。
//!
//! 对齐 C# 参考实现 `System/Native/PeriodConfig.cs`。
//! 字段：timeOffset（乐段起点时间）、minLength（最小显示长度，默认 10）、
//! active（是否激活，默认 true）。

use gorge_core::objective::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 乐段设置
///
/// 字段按 C# 声明顺序：timeOffset、minLength、active。
/// 注入器默认值对齐 C# 的 InjectorFieldDefaultValue_*。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct PeriodConfig {
    /// 乐段起点时间（float，秒）
    #[gorge_field]
    #[inject(default = 0.0)]
    pub time_offset: f32,
    /// 最小显示长度（float，秒），注入器默认 10
    #[gorge_field]
    #[inject(default = 10.0)]
    pub min_length: f32,
    /// 是否激活（bool），注入器默认 true
    #[gorge_field]
    #[inject(default = true)]
    pub active: bool,
}

#[gorge_native_impl]
impl PeriodConfig {
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, time_offset: f32, min_length: f32, active: bool) {
        ctx.set_object_float_field(this, Self::FIELD_INDEX_time_offset, time_offset as f64);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_min_length, min_length as f64);
        ctx.set_object_bool_field(this, Self::FIELD_INDEX_active, active);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    struct Fixture {
        vm: VirtualMachine,
    }

    impl Fixture {
        fn new() -> Self {
            let mut vm = VirtualMachine::new();
            vm.next_object_id = 1;
            Self { vm }
        }
        fn ctx(&mut self) -> NativeContext<'_> { NativeContext::new(&mut self.vm) }
    }

    #[test]
    fn test_period_config_construct_with_defaults() {
        let pc = PeriodConfig { time_offset: 0.0, min_length: 0.0, active: false };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_float_param(0, 2.5);
        fx.vm.param_pool.set_float_param(1, 15.0);
        fx.vm.param_pool.set_bool_param(0, true);
        let id = { let mut ctx = fx.ctx(); pc.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        let to = { let ctx = fx.ctx(); ctx.get_object_float_field(id, PeriodConfig::FIELD_INDEX_time_offset) as f32 };
        let ml = { let ctx = fx.ctx(); ctx.get_object_float_field(id, PeriodConfig::FIELD_INDEX_min_length) as f32 };
        let act = { let ctx = fx.ctx(); ctx.get_object_bool_field(id, PeriodConfig::FIELD_INDEX_active) };
        assert!((to - 2.5).abs() < 0.01);
        assert!((ml - 15.0).abs() < 0.01);
        assert!(act);
    }

    #[test]
    fn test_period_config_inactive() {
        let pc = PeriodConfig { time_offset: 0.0, min_length: 0.0, active: false };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_float_param(0, 0.0);
        fx.vm.param_pool.set_float_param(1, 10.0);
        fx.vm.param_pool.set_bool_param(0, false);
        let id = { let mut ctx = fx.ctx(); pc.do_construct_native(&mut ctx, None, 0) };
        let act = { let ctx = fx.ctx(); ctx.get_object_bool_field(id, PeriodConfig::FIELD_INDEX_active) };
        assert!(!act);
    }

    #[test]
    fn test_period_config_field_count() {
        let tc = PeriodConfig { time_offset: 0.0, min_length: 0.0, active: false }.field_type_count();
        assert_eq!(tc.float_count, 2); // time_offset, min_length
        assert_eq!(tc.bool_count, 1);  // active
        assert_eq!(tc.int_count, 0);
        assert_eq!(tc.string_count, 0);
        assert_eq!(tc.object_count, 0);
    }
}
