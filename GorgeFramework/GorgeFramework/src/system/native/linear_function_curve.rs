//! `GorgeFramework.LinearFunctionCurve` — 线性曲线：f(x) = kx + b。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use super::function_curve::FunctionCurve;

/// 线性曲线：f(x) = kx + b
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct LinearFunctionCurve {
    #[gorge_field]
    pub k: f32,
    #[gorge_field]
    pub b: f32,
}

impl LinearFunctionCurve {
    pub fn new(k: f32, b: f32) -> Self { Self { k, b } }
}

#[gorge_native_impl]
impl LinearFunctionCurve {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, k: f32, b: f32) {
        ctx.set_object_float_field(this, LinearFunctionCurve::FIELD_INDEX_k, k as f64);
        ctx.set_object_float_field(this, LinearFunctionCurve::FIELD_INDEX_b, b as f64);
    }

    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let k = ctx.get_object_float_field(this, LinearFunctionCurve::FIELD_INDEX_k) as f32;
        let b = ctx.get_object_float_field(this, LinearFunctionCurve::FIELD_INDEX_b) as f32;
        k * x + b
    }
}

impl FunctionCurve for LinearFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 { self.k * x + self.b }
}
