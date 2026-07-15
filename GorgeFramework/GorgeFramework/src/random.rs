//! `GorgeFramework.Random` —— 随机数工具 native 类（纯静态方法）。
//!
//! 移植自 C# 参考实现 `System/Native/Random.cs`。

use gorge_core::native::NativeContext;
use gorge_core::object::RuntimeObject;
use gorge_macros::{gorge_native_class, gorge_native_impl};
use std::f32::consts::PI;

/// 随机数工具类（无实例字段，仅提供静态方法）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Random {}

#[gorge_native_impl]
impl Random {
    /// 静态方法 0：返回 [0, 1) 范围随机浮点数
    #[gorge_static]
    pub fn random_float(_ctx: &mut NativeContext) -> f32 {
        rand::random::<f32>()
    }

    /// 静态方法 1：返回 [a, b) 范围随机浮点数
    #[gorge_static]
    pub fn random_range(_ctx: &mut NativeContext, a: f32, b: f32) -> f32 {
        let t: f32 = rand::random();
        a + (b - a) * t
    }

    /// 静态方法 2：返回随机单位圆向量 (Vector2)
    #[gorge_static]
    pub fn random_normalized(ctx: &mut NativeContext) -> usize {
        let angle: f32 = rand::random::<f32>() * 2.0 * PI;
        let obj = RuntimeObject::new_simple(
            "GorgeFramework.Vector2".to_string(),
            &gorge_core::types::TypeCount { float_count: 2, ..Default::default() },
        );
        let id = ctx.register_object(obj);
        ctx.set_object_float_field(id, 0, angle.cos() as f64);
        ctx.set_object_float_field(id, 1, angle.sin() as f64);
        id
    }
}
