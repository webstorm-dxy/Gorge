//! `GorgeFramework.LinearCurve` — 线段曲线：在 (timeStart, valueStart) ~ (timeEnd, valueEnd) 之间线性插值。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use super::function_curve::FunctionCurve;

/// 线段曲线：在 (timeStart, valueStart) ~ (timeEnd, valueEnd) 之间线性插值
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct LinearCurve {
    #[gorge_field]
    #[inject(name = "timeStart", default = 0.0)]
    pub time_start: f32,
    #[gorge_field]
    #[inject(name = "valueStart", default = 0.0)]
    pub value_start: f32,
    #[gorge_field]
    #[inject(name = "timeEnd", default = 1.0)]
    pub time_end: f32,
    #[gorge_field]
    #[inject(name = "valueEnd", default = 1.0)]
    pub value_end: f32,
}

impl LinearCurve {
    pub fn new(time_start: f32, value_start: f32, time_end: f32, value_end: f32) -> Self {
        Self { time_start, value_start, time_end, value_end }
    }
}

#[gorge_native_impl]
impl LinearCurve {
    /// 构造方法 0：无参构造（字段取注入器默认值）
    #[gorge_ctor]
    pub fn new_empty(ctx: &mut NativeContext, this: usize) { let _ = (ctx, this); }

    /// 构造方法 1：以显式参数构造
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, ts: f32, vs: f32, te: f32, ve: f32) {
        ctx.set_object_float_field(this, LinearCurve::FIELD_INDEX_time_start, ts as f64);
        ctx.set_object_float_field(this, LinearCurve::FIELD_INDEX_value_start, vs as f64);
        ctx.set_object_float_field(this, LinearCurve::FIELD_INDEX_time_end, te as f64);
        ctx.set_object_float_field(this, LinearCurve::FIELD_INDEX_value_end, ve as f64);
    }

    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let ts = ctx.get_object_float_field(this, LinearCurve::FIELD_INDEX_time_start) as f32;
        let vs = ctx.get_object_float_field(this, LinearCurve::FIELD_INDEX_value_start) as f32;
        let te = ctx.get_object_float_field(this, LinearCurve::FIELD_INDEX_time_end) as f32;
        let ve = ctx.get_object_float_field(this, LinearCurve::FIELD_INDEX_value_end) as f32;
        if (te - ts).abs() < 1e-10 { return vs; }
        let t = ((x - ts) / (te - ts)).clamp(0.0, 1.0);
        vs + (ve - vs) * t
    }
}

impl FunctionCurve for LinearCurve {
    fn evaluate(&self, x: f32) -> f32 {
        if (self.time_end - self.time_start).abs() < 1e-10 { return self.value_start; }
        let t = ((x - self.time_start) / (self.time_end - self.time_start)).clamp(0.0, 1.0);
        self.value_start + (self.value_end - self.value_start) * t
    }
}
