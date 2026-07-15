//! `GorgeFramework.ColorArgb` —— ARGB 四通道颜色 native 类。
//!
//! 移植自 C# 参考实现。注意：C# 使用 float(0~1) 字段，此处沿用 i32(0~255)
//! 以保持现有 Gorge 代码兼容，提供 Lerp 方法用于颜色插值。

use gorge_core::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 四通道颜色
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct ColorArgb {
    #[gorge_field]
    pub a: i32,
    #[gorge_field]
    pub r: i32,
    #[gorge_field]
    pub g: i32,
    #[gorge_field]
    pub b: i32,
}

/// 预定义白色常量
pub const COLOR_WHITE: ColorArgb = ColorArgb { a: 255, r: 255, g: 255, b: 255 };

#[gorge_native_impl]
impl ColorArgb {
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, a: i32, r: i32, g: i32, b: i32) {
        ctx.set_object_int_field(this, Self::FIELD_INDEX_a, a as i64);
        ctx.set_object_int_field(this, Self::FIELD_INDEX_r, r as i64);
        ctx.set_object_int_field(this, Self::FIELD_INDEX_g, g as i64);
        ctx.set_object_int_field(this, Self::FIELD_INDEX_b, b as i64);
    }

    /// 静态方法：线性插值，t 钳制到 [0,1]
    #[gorge_static]
    pub fn lerp(ctx: &mut NativeContext, c1: usize, c2: usize, t: f32) -> usize {
        let a1 = ctx.get_object_int_field(c1, Self::FIELD_INDEX_a) as i32;
        let r1 = ctx.get_object_int_field(c1, Self::FIELD_INDEX_r) as i32;
        let g1 = ctx.get_object_int_field(c1, Self::FIELD_INDEX_g) as i32;
        let b1 = ctx.get_object_int_field(c1, Self::FIELD_INDEX_b) as i32;
        let a2 = ctx.get_object_int_field(c2, Self::FIELD_INDEX_a) as i32;
        let r2 = ctx.get_object_int_field(c2, Self::FIELD_INDEX_r) as i32;
        let g2 = ctx.get_object_int_field(c2, Self::FIELD_INDEX_g) as i32;
        let b2 = ctx.get_object_int_field(c2, Self::FIELD_INDEX_b) as i32;
        let t = t.clamp(0.0, 1.0);
        let lerp_i = |v1: i32, v2: i32| -> i32 {
            (v1 as f32 + (v2 as f32 - v1 as f32) * t).round() as i32
        };
        let obj = gorge_core::object::RuntimeObject::new_simple(
            Self::GORGE_FULL_NAME.to_string(),
            &Self::gorge_field_type_count(),
        );
        let id = ctx.register_object(obj);
        ctx.set_object_int_field(id, Self::FIELD_INDEX_a, lerp_i(a1, a2) as i64);
        ctx.set_object_int_field(id, Self::FIELD_INDEX_r, lerp_i(r1, r2) as i64);
        ctx.set_object_int_field(id, Self::FIELD_INDEX_g, lerp_i(g1, g2) as i64);
        ctx.set_object_int_field(id, Self::FIELD_INDEX_b, lerp_i(b1, b2) as i64);
        id
    }
}
