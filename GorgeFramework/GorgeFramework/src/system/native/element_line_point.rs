//! `GorgeFramework` — 元素连线控制点（native 数据类）。
//!
//! 对齐 C# 参考实现 `ElementLinePoint.cs`。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 元素连线控制点
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct ElementLinePoint {
    /// 时间位置
    #[gorge_field]
    pub time: f32,
    /// 空间位置
    #[gorge_field]
    pub position: f32,
    /// 线宽
    #[gorge_field]
    pub width: f32,
}

impl ElementLinePoint {
    pub fn new(time: f32, position: f32, width: f32) -> Self {
        Self { time, position, width }
    }
}

#[gorge_native_impl]
impl ElementLinePoint {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, time: f32, position: f32, width: f32) {
        ctx.set_object_float_field(this, ElementLinePoint::FIELD_INDEX_time, time as f64);
        ctx.set_object_float_field(this, ElementLinePoint::FIELD_INDEX_position, position as f64);
        ctx.set_object_float_field(this, ElementLinePoint::FIELD_INDEX_width, width as f64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_line_point_fields() {
        let p = ElementLinePoint::new(1.0, 10.0, 2.0);
        assert_eq!(p.time, 1.0);
        assert_eq!(p.position, 10.0);
        assert_eq!(p.width, 2.0);
    }
}
