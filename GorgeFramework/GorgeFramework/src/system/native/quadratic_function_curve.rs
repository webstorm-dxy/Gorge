//! `GorgeFramework.QuadraticFunctionCurve` — 二次曲线：f(x) = ax² + bx + c。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use super::function_curve::FunctionCurve;

/// 二次曲线：f(x) = ax² + bx + c
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct QuadraticFunctionCurve {
    #[gorge_field]
    pub a: f32,
    #[gorge_field]
    pub b: f32,
    #[gorge_field]
    pub c: f32,
}

impl QuadraticFunctionCurve {
    pub fn new(a: f32, b: f32, c: f32) -> Self { Self { a, b, c } }
}

#[gorge_native_impl]
impl QuadraticFunctionCurve {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, a: f32, b: f32, c: f32) {
        ctx.set_object_float_field(this, QuadraticFunctionCurve::FIELD_INDEX_a, a as f64);
        ctx.set_object_float_field(this, QuadraticFunctionCurve::FIELD_INDEX_b, b as f64);
        ctx.set_object_float_field(this, QuadraticFunctionCurve::FIELD_INDEX_c, c as f64);
    }

    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let a = ctx.get_object_float_field(this, QuadraticFunctionCurve::FIELD_INDEX_a) as f32;
        let b = ctx.get_object_float_field(this, QuadraticFunctionCurve::FIELD_INDEX_b) as f32;
        let c = ctx.get_object_float_field(this, QuadraticFunctionCurve::FIELD_INDEX_c) as f32;
        a * x * x + b * x + c
    }
}

impl FunctionCurve for QuadraticFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 { self.a * x * x + self.b * x + self.c }
}
