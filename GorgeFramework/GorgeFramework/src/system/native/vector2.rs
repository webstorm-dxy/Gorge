//! `GorgeFramework.Vector2` —— 二维向量 native 类。
//!
//! 移植自 C# 参考实现 `System/Native/Vector2.cs`，含字段、构造、实例方法、
//! 静态方法、注入器字段支持，以及向量运算（归一化、角度、插值等）。

use gorge_core::objective::native::NativeContext;
use gorge_core::objective::object::RuntimeObject;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 二维向量
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Vector2 {
    #[gorge_field]
    #[inject(default = 0.0)]
    pub x: f32,
    #[gorge_field]
    #[inject(default = 0.0)]
    pub y: f32,
}

#[gorge_native_impl]
impl Vector2 {
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, x: f32, y: f32) {
        ctx.set_object_float_field(this, Vector2::FIELD_INDEX_x, x as f64);
        ctx.set_object_float_field(this, Vector2::FIELD_INDEX_y, y as f64);
    }

    #[gorge_static]
    pub fn distance(ctx: &mut NativeContext, v1: usize, v2: usize) -> f32 {
        let (x1, y1) = read_xy(ctx, v1);
        let (x2, y2) = read_xy(ctx, v2);
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }

    #[gorge_static]
    pub fn scale(ctx: &mut NativeContext, v1: usize, v2: usize) -> usize {
        let (x1, y1) = read_xy(ctx, v1);
        let (x2, y2) = read_xy(ctx, v2);
        make_vector2(ctx, x1 * x2, y1 * y2)
    }

    #[gorge_method]
    pub fn magnitude(ctx: &mut NativeContext, this: usize) -> f32 {
        let (x, y) = read_xy(ctx, this);
        (x * x + y * y).sqrt()
    }

    #[gorge_method]
    pub fn get_x(ctx: &mut NativeContext, this: usize) -> f32 {
        ctx.get_object_float_field(this, Vector2::FIELD_INDEX_x) as f32
    }

    #[gorge_method]
    pub fn get_y(ctx: &mut NativeContext, this: usize) -> f32 {
        ctx.get_object_float_field(this, Vector2::FIELD_INDEX_y) as f32
    }

    #[gorge_static]
    pub fn lerp(ctx: &mut NativeContext, a: usize, b: usize, t: f32) -> usize {
        let (ax, ay) = read_xy(ctx, a);
        let (bx, by) = read_xy(ctx, b);
        let t = t.clamp(0.0, 1.0);
        make_vector2(ctx, ax + (bx - ax) * t, ay + (by - ay) * t)
    }

    /// 静态方法：归一化，返回单位向量
    #[gorge_static]
    pub fn normalize(ctx: &mut NativeContext, v: usize) -> usize {
        let (x, y) = read_xy(ctx, v);
        let len = (x * x + y * y).sqrt();
        if len < 1e-10 { make_vector2(ctx, 0.0, 0.0) }
        else { make_vector2(ctx, x / len, y / len) }
    }

    /// 实例方法：自身与 x 轴正方向的夹角（弧度，[0, 2π)）
    #[gorge_method]
    pub fn angle(ctx: &mut NativeContext, this: usize) -> f32 {
        let (x, y) = read_xy(ctx, this);
        let a = y.atan2(x);
        if a < 0.0 { a + 2.0 * std::f32::consts::PI } else { a }
    }

    /// 静态方法：两向量夹角（弧度，[0, 2π)）
    #[gorge_static]
    pub fn unsigned_angle(ctx: &mut NativeContext, v1: usize, v2: usize) -> f32 {
        let (x1, y1) = read_xy(ctx, v1);
        let (x2, y2) = read_xy(ctx, v2);
        let dot = x1 * x2 + y1 * y2;
        let det = x1 * y2 - y1 * x2;
        let a = det.atan2(dot);
        if a < 0.0 { a + 2.0 * std::f32::consts::PI } else { a }
    }

    /// 静态方法：从 v1 到 v2 的有符号夹角（弧度，[-π, π)）
    #[gorge_static]
    pub fn signed_angle(ctx: &mut NativeContext, v1: usize, v2: usize) -> f32 {
        let (x1, y1) = read_xy(ctx, v1);
        let (x2, y2) = read_xy(ctx, v2);
        let dot = x1 * x2 + y1 * y2;
        let det = x1 * y2 - y1 * x2;
        det.atan2(dot)
    }

    /// 实例方法：转为三维向量（z = 0）
    #[gorge_method]
    pub fn to_vector3(ctx: &mut NativeContext, this: usize) -> usize {
        let (x, y) = read_xy(ctx, this);
        let obj = RuntimeObject::new_simple(
            "GorgeFramework.Vector3".to_string(),
            &gorge_core::objective::types::TypeCount { float_count: 3, ..Default::default() },
        );
        let id = ctx.register_object(obj);
        ctx.set_object_float_field(id, 0, x as f64);
        ctx.set_object_float_field(id, 1, y as f64);
        ctx.set_object_float_field(id, 2, 0.0);
        id
    }
}

fn read_xy(ctx: &NativeContext, obj_id: usize) -> (f32, f32) {
    let x = ctx.get_object_float_field(obj_id, Vector2::FIELD_INDEX_x) as f32;
    let y = ctx.get_object_float_field(obj_id, Vector2::FIELD_INDEX_y) as f32;
    (x, y)
}

fn make_vector2(ctx: &mut NativeContext, x: f32, y: f32) -> usize {
    let obj = RuntimeObject::new_simple(
        Vector2::GORGE_FULL_NAME.to_string(),
        &Vector2::gorge_field_type_count(),
    );
    let id = ctx.register_object(obj);
    ctx.set_object_float_field(id, Vector2::FIELD_INDEX_x, x as f64);
    ctx.set_object_float_field(id, Vector2::FIELD_INDEX_y, y as f64);
    id
}
