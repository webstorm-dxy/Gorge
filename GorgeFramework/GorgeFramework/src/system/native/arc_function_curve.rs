//! `GorgeFramework.ArcFunctionCurve` — 弧形曲线：给定弦 (chordStart, chordEnd) 和圆心角 angle (rad)。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use super::function_curve::FunctionCurve;

/// 弧形曲线：给定弦 (chordStart, chordEnd) 和圆心角 angle (rad)
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct ArcFunctionCurve {
    #[gorge_field]
    pub chord_start: f32,
    #[gorge_field]
    pub chord_end: f32,
    #[gorge_field]
    pub angle: f32,
}

impl ArcFunctionCurve {
    pub fn new(chord_start: f32, chord_end: f32, angle: f32) -> Self {
        Self { chord_start, chord_end, angle }
    }
}

#[gorge_native_impl]
impl ArcFunctionCurve {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, cs: f32, ce: f32, angle: f32) {
        ctx.set_object_float_field(this, ArcFunctionCurve::FIELD_INDEX_chord_start, cs as f64);
        ctx.set_object_float_field(this, ArcFunctionCurve::FIELD_INDEX_chord_end, ce as f64);
        ctx.set_object_float_field(this, ArcFunctionCurve::FIELD_INDEX_angle, angle as f64);
    }

    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let cs = ctx.get_object_float_field(this, ArcFunctionCurve::FIELD_INDEX_chord_start) as f32;
        let ce = ctx.get_object_float_field(this, ArcFunctionCurve::FIELD_INDEX_chord_end) as f32;
        let angle = ctx.get_object_float_field(this, ArcFunctionCurve::FIELD_INDEX_angle) as f32;
        let chord_len = ce - cs;
        if chord_len.abs() < 1e-10 { return 0.0; }
        let half = angle / 2.0;
        let radius = chord_len / (2.0 * half.sin());
        if radius.abs() < 1e-10 { return 0.0; }
        let mid = (cs + ce) / 2.0;
        let dx = (x - mid).clamp(-radius, radius);
        let afc = (dx / radius).asin();
        radius * (half.cos() - (half - afc).cos())
    }
}

impl FunctionCurve for ArcFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 {
        let chord_len = self.chord_end - self.chord_start;
        if chord_len.abs() < 1e-10 { return 0.0; }
        let half = self.angle / 2.0;
        let radius = chord_len / (2.0 * half.sin());
        if radius.abs() < 1e-10 { return 0.0; }
        let mid = (self.chord_start + self.chord_end) / 2.0;
        let dx = (x - mid).clamp(-radius, radius);
        let afc = (dx / radius).asin();
        radius * (half.cos() - (half - afc).cos())
    }
}
