//! `GorgeFramework.Random` —— 随机数工具 native 类（纯静态方法）。
//!
//! 对齐 C# 参考实现 `System/Native/Random.cs`。
//! 方法顺序对齐 C# 声明序：RandomNormalized(0)、RandomFloat(a,b)(1)。

use gorge_core::objective::native::NativeContext;
use gorge_core::objective::object::RuntimeObject;
use gorge_macros::{gorge_native_class, gorge_native_impl};
use std::f32::consts::PI;

/// 随机数工具类（无实例字段，仅提供静态方法）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Random {}

#[gorge_native_impl]
impl Random {
    /// 静态方法 0：返回随机单位圆向量 (Vector2)
    ///
    /// 对齐 C# `RandomNormalized`。
    #[gorge_static]
    pub fn random_normalized(ctx: &mut NativeContext) -> usize {
        let angle: f32 = rand::random::<f32>() * 2.0 * PI;
        let obj = RuntimeObject::new_simple(
            "GorgeFramework.Vector2".to_string(),
            &gorge_core::objective::types::TypeCount { float_count: 2, ..Default::default() },
        );
        let id = ctx.register_object(obj);
        ctx.set_object_float_field(id, 0, angle.cos() as f64);
        ctx.set_object_float_field(id, 1, angle.sin() as f64);
        id
    }

    /// 静态方法 1：返回 [a, b) 范围随机浮点数
    ///
    /// 对齐 C# `RandomFloat(a, b)`。
    #[gorge_static]
    pub fn random_float(_ctx: &mut NativeContext, a: f32, b: f32) -> f32 {
        let t: f32 = rand::random();
        a + (b - a) * t
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
    fn test_random_normalized_returns_vector2() {
        let r = Random {};
        let mut fx = Fixture::new();
        { let mut ctx = fx.ctx(); r.invoke_native_static(&mut ctx, 0); }
        let id = fx.vm.param_pool.get_object_return();
        assert!(id > 0, "应返回 Vector2 对象 ID");
    }

    #[test]
    fn test_random_float_in_range() {
        let r = Random {};
        let mut fx = Fixture::new();
        // 测试多次，确保始终落在 [a, b) 区间
        for _ in 0..20 {
            fx.vm.param_pool.set_float_param(0, 10.0);
            fx.vm.param_pool.set_float_param(1, 20.0);
            { let mut ctx = fx.ctx(); r.invoke_native_static(&mut ctx, 1); }
            let v = (fx.vm.param_pool.get_float_return() as f64) as f32;
            assert!(v >= 10.0 && v < 20.0, "random_float(10,20) 应在 [10,20)，实际 {}", v);
        }
    }

    #[test]
    fn test_random_float_same_bounds_returns_bound() {
        let r = Random {};
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_float_param(0, 5.0);
        fx.vm.param_pool.set_float_param(1, 5.0);
        { let mut ctx = fx.ctx(); r.invoke_native_static(&mut ctx, 1); }
        let v = (fx.vm.param_pool.get_float_return() as f64) as f32;
        assert!((v - 5.0).abs() < 0.01, "random_float(5,5) 应返回 5.0，实际 {}", v);
    }
}
