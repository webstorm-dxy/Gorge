//! `GorgeFramework` — 元素连线（native 数据类）。
//!
//! 对齐 C# 参考实现 `ElementLine.cs`。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 元素连线（字段展开为 r/g/b/a 四个 int 颜色通道，不含 points Vec）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct ElementLine {
    #[gorge_field]
    pub color_r: i32,
    #[gorge_field]
    pub color_g: i32,
    #[gorge_field]
    pub color_b: i32,
    #[gorge_field]
    pub color_a: i32,
}

impl ElementLine {
    pub fn new(r: i32, g: i32, b: i32, a: i32) -> Self {
        Self { color_r: r, color_g: g, color_b: b, color_a: a }
    }
}

#[gorge_native_impl]
impl ElementLine {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, r: i32, g: i32, b: i32, a: i32) {
        ctx.set_object_int_field(this, ElementLine::FIELD_INDEX_color_r, r as i64);
        ctx.set_object_int_field(this, ElementLine::FIELD_INDEX_color_g, g as i64);
        ctx.set_object_int_field(this, ElementLine::FIELD_INDEX_color_b, b as i64);
        ctx.set_object_int_field(this, ElementLine::FIELD_INDEX_color_a, a as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_line_new() {
        let l = ElementLine::new(255, 0, 0, 255);
        assert_eq!(l.color_r, 255);
        assert_eq!(l.color_a, 255);
    }
}
