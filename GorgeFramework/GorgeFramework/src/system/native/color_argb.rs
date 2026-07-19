//! `GorgeFramework.ColorArgb` —— ARGB 四通道颜色 native 类。
//!
//! 对齐 C# 参考实现 `System/Native/ColorArgb.cs`。
//! 字段 a/r/g/b 均为 float（0~1 范围），提供 Lerp 静态方法用于颜色插值。

use gorge_core::objective::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 四通道颜色（float 0~1）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct ColorArgb {
    /// 不透明度（0~1），注入器默认 1
    #[gorge_field]
    #[inject(default = 1.0)]
    pub a: f32,
    /// 红（0~1），注入器默认 1
    #[gorge_field]
    #[inject(default = 1.0)]
    pub r: f32,
    /// 绿（0~1），注入器默认 1
    #[gorge_field]
    #[inject(default = 1.0)]
    pub g: f32,
    /// 蓝（0~1），注入器默认 1
    #[gorge_field]
    #[inject(default = 1.0)]
    pub b: f32,
}

/// 预定义白色常量
pub const COLOR_WHITE: ColorArgb = ColorArgb { a: 1.0, r: 1.0, g: 1.0, b: 1.0 };

/// 从 ColorArgb 对象读取全部四个 float 通道
///
/// 供 sprite 族 UpdateNode 等外部使用方复用，
/// 消除硬编码字段索引。
pub fn read_color_channels(ctx: &NativeContext, color_id: usize) -> (f32, f32, f32, f32) {
    if color_id == 0 {
        return (1.0, 1.0, 1.0, 1.0);
    }
    let a = ctx.get_object_float_field(color_id, ColorArgb::FIELD_INDEX_a) as f32;
    let r = ctx.get_object_float_field(color_id, ColorArgb::FIELD_INDEX_r) as f32;
    let g = ctx.get_object_float_field(color_id, ColorArgb::FIELD_INDEX_g) as f32;
    let b = ctx.get_object_float_field(color_id, ColorArgb::FIELD_INDEX_b) as f32;
    (a, r, g, b)
}

#[gorge_native_impl]
impl ColorArgb {
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, a: f32, r: f32, g: f32, b: f32) {
        ctx.set_object_float_field(this, Self::FIELD_INDEX_a, a as f64);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_r, r as f64);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_g, g as f64);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_b, b as f64);
    }

    /// 静态方法 0：线性插值，t 钳制到 [0,1]
    ///
    /// 读取两个源颜色的 float 通道，按 t 插值后返回新 ColorArgb 对象 ID。
    #[gorge_static]
    pub fn lerp(ctx: &mut NativeContext, c1: usize, c2: usize, t: f32) -> usize {
        let a1 = ctx.get_object_float_field(c1, Self::FIELD_INDEX_a) as f32;
        let r1 = ctx.get_object_float_field(c1, Self::FIELD_INDEX_r) as f32;
        let g1 = ctx.get_object_float_field(c1, Self::FIELD_INDEX_g) as f32;
        let b1 = ctx.get_object_float_field(c1, Self::FIELD_INDEX_b) as f32;
        let a2 = ctx.get_object_float_field(c2, Self::FIELD_INDEX_a) as f32;
        let r2 = ctx.get_object_float_field(c2, Self::FIELD_INDEX_r) as f32;
        let g2 = ctx.get_object_float_field(c2, Self::FIELD_INDEX_g) as f32;
        let b2 = ctx.get_object_float_field(c2, Self::FIELD_INDEX_b) as f32;
        let t = t.clamp(0.0, 1.0);
        let obj = gorge_core::objective::object::RuntimeObject::new_simple(
            Self::GORGE_FULL_NAME.to_string(),
            &Self::gorge_field_type_count(),
        );
        let id = ctx.register_object(obj);
        ctx.set_object_float_field(id, Self::FIELD_INDEX_a, (a1 + (a2 - a1) * t) as f64);
        ctx.set_object_float_field(id, Self::FIELD_INDEX_r, (r1 + (r2 - r1) * t) as f64);
        ctx.set_object_float_field(id, Self::FIELD_INDEX_g, (g1 + (g2 - g1) * t) as f64);
        ctx.set_object_float_field(id, Self::FIELD_INDEX_b, (b1 + (b2 - b1) * t) as f64);
        id
    }
}
