//! `GorgeFramework.Math` —— 数学工具 native 类（纯静态方法）。
//!
//! 移植自 C# 参考实现 `System/Native/Math.cs` 的代表性子集，
//! 用于验证 `#[gorge_native_class]` / `#[gorge_native_impl]` 在纯静态、
//! 无字段场景下的桥接。

use gorge_core::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 数学工具类（无实例字段，仅提供静态方法）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Math {}

#[gorge_native_impl]
impl Math {
    /// 静态方法 0：绝对值
    #[gorge_static]
    pub fn abs(_ctx: &mut NativeContext, f: f32) -> f32 {
        f.abs()
    }

    /// 静态方法 1：平方根
    #[gorge_static]
    pub fn sqrt(_ctx: &mut NativeContext, f: f32) -> f32 {
        f.sqrt()
    }

    /// 静态方法 2：两数最大值
    #[gorge_static]
    pub fn max(_ctx: &mut NativeContext, a: f32, b: f32) -> f32 {
        a.max(b)
    }

    /// 静态方法 3：两数最小值
    #[gorge_static]
    pub fn min(_ctx: &mut NativeContext, a: f32, b: f32) -> f32 {
        a.min(b)
    }

    /// 静态方法 4：向下取整（返回 int）
    #[gorge_static]
    pub fn floor(_ctx: &mut NativeContext, f: f32) -> i32 {
        f.floor() as i32
    }

    /// 静态方法 5：向上取整（返回 int）
    #[gorge_static]
    pub fn ceil(_ctx: &mut NativeContext, f: f32) -> i32 {
        f.ceil() as i32
    }

    /// 静态方法 6：线性插值，t 被钳制到 [0,1]
    #[gorge_static]
    pub fn lerp(_ctx: &mut NativeContext, a: f32, b: f32, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        a + (b - a) * t
    }

    /// 静态方法 7：把值钳制到 [min, max]
    #[gorge_static]
    pub fn clamp(_ctx: &mut NativeContext, value: f32, min: f32, max: f32) -> f32 {
        value.clamp(min, max)
    }
}
