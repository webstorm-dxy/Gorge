//! `GorgeFramework.CubicHermiteSpline` — 三次 Hermite 样条曲线（8 个 float 字段）。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use super::function_curve::FunctionCurve;

#[gorge_native_class(namespace = "GorgeFramework")]
pub struct CubicHermiteSpline {
    #[gorge_field] pub time_start: f32,
    #[gorge_field] pub value_start: f32,
    #[gorge_field] pub m0: f32,
    pub w0: f32,
    #[gorge_field] pub time_end: f32,
    #[gorge_field] pub value_end: f32,
    #[gorge_field] pub m1: f32,
    pub w1: f32,
}

impl CubicHermiteSpline {
    pub fn new(ts: f32, vs: f32, m0: f32, _w0: f32, te: f32, ve: f32, m1: f32, _w1: f32) -> Self {
        Self { time_start: ts, value_start: vs, m0, w0: _w0, time_end: te, value_end: ve, m1, w1: _w1 }
    }
}

#[gorge_native_impl]
impl CubicHermiteSpline {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize,
        ts: f32, vs: f32, m0: f32, _w0: f32, te: f32, ve: f32, m1: f32, _w1: f32)
    {
        ctx.set_object_float_field(this, CubicHermiteSpline::FIELD_INDEX_time_start, ts as f64);
        ctx.set_object_float_field(this, CubicHermiteSpline::FIELD_INDEX_value_start, vs as f64);
        ctx.set_object_float_field(this, CubicHermiteSpline::FIELD_INDEX_m0, m0 as f64);
        ctx.set_object_float_field(this, CubicHermiteSpline::FIELD_INDEX_time_end, te as f64);
        ctx.set_object_float_field(this, CubicHermiteSpline::FIELD_INDEX_value_end, ve as f64);
        ctx.set_object_float_field(this, CubicHermiteSpline::FIELD_INDEX_m1, m1 as f64);
    }

    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let ts = ctx.get_object_float_field(this, CubicHermiteSpline::FIELD_INDEX_time_start) as f32;
        let vs = ctx.get_object_float_field(this, CubicHermiteSpline::FIELD_INDEX_value_start) as f32;
        let m0 = ctx.get_object_float_field(this, CubicHermiteSpline::FIELD_INDEX_m0) as f32;
        let te = ctx.get_object_float_field(this, CubicHermiteSpline::FIELD_INDEX_time_end) as f32;
        let ve = ctx.get_object_float_field(this, CubicHermiteSpline::FIELD_INDEX_value_end) as f32;
        let m1 = ctx.get_object_float_field(this, CubicHermiteSpline::FIELD_INDEX_m1) as f32;
        let denom = te - ts;
        if denom.abs() < 1e-10 { return vs; }
        let t = ((x - ts) / denom).clamp(0.0, 1.0);
        let t2 = t * t; let t3 = t2 * t;
        let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
        let h10 = t3 - 2.0 * t2 + t;
        let h01 = -2.0 * t3 + 3.0 * t2;
        let h11 = t3 - t2;
        h00 * vs + h10 * m0 * denom + h01 * ve + h11 * m1 * denom
    }
}

impl FunctionCurve for CubicHermiteSpline {
    fn evaluate(&self, x: f32) -> f32 {
        let denom = self.time_end - self.time_start;
        if denom.abs() < 1e-10 { return self.value_start; }
        let t = ((x - self.time_start) / denom).clamp(0.0, 1.0);
        let t2 = t * t; let t3 = t2 * t;
        let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
        let h10 = t3 - 2.0 * t2 + t;
        let h01 = -2.0 * t3 + 3.0 * t2;
        let h11 = t3 - t2;
        h00 * self.value_start + h10 * self.m0 * denom + h01 * self.value_end + h11 * self.m1 * denom
    }
}
