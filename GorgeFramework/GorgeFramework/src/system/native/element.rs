//! `GorgeFramework` — 游戏元素系统（native 类注册）。
//!
//! 移植自 C# 参考实现。包含 ElementLinePoint（连线控制点）和
//! ElementLine（元素连线）的 native 类注册。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

// ==================== ElementLinePoint ====================

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

// ==================== ElementLine ====================

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

// ==================== 内部 Rust 类型（不注册 native） ====================

/// 游戏元素基类（含 Node 抽象层级，不适合 native 类注册）
#[derive(Debug)]
pub struct Element {
    pub nodes: Vec<crate::system::native::node::Node>,
    pub derived_elements: Vec<Element>,
    pub simulator: Option<Box<dyn ElementSimulator>>,
    pub late_independent_simulator: Option<Box<dyn ElementSimulator>>,
}

impl Element {
    pub fn new() -> Self {
        Self { nodes: Vec::new(), derived_elements: Vec::new(), simulator: None, late_independent_simulator: None }
    }
    pub fn alive_nodes(&self) -> impl Iterator<Item = &crate::system::native::node::Node> {
        self.nodes.iter().filter(|n| n.alive)
    }
}
impl Default for Element { fn default() -> Self { Self::new() } }

/// 元素模拟器 trait（对齐 C# ISimulator 接口）
pub trait ElementSimulator: std::fmt::Debug + Send + Sync {
    fn update(&mut self, time: f32);
}

/// 音符元素（含 Element 嵌套，不适合 native 类注册）
#[derive(Debug)]
pub struct Note {
    pub element: Element,
    pub automaton_enabled: bool,
}
impl Note {
    pub fn new() -> Self { Self { element: Element::new(), automaton_enabled: false } }
    pub fn do_respond(&self, _respond_mode: &str, _chart_time: f32) -> Vec<String> { Vec::new() }
}
impl Default for Note { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_default() {
        let e = Element::new();
        assert!(e.nodes.is_empty());
        assert!(e.derived_elements.is_empty());
    }

    #[test]
    fn test_note_default() {
        let n = Note::new();
        assert!(!n.automaton_enabled);
    }

    #[test]
    fn test_element_line_point_fields() {
        let p = ElementLinePoint::new(1.0, 10.0, 2.0);
        assert_eq!(p.time, 1.0);
        assert_eq!(p.position, 10.0);
        assert_eq!(p.width, 2.0);
    }

    #[test]
    fn test_element_line_new() {
        let l = ElementLine::new(255, 0, 0, 255);
        assert_eq!(l.color_r, 255);
        assert_eq!(l.color_a, 255);
    }
}
