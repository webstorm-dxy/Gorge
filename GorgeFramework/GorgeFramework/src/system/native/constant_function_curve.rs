//! `GorgeFramework.ConstantFunctionCurve` — 常量曲线：永远返回固定值。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use super::function_curve::FunctionCurve;

/// 常量曲线：永远返回固定值
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct ConstantFunctionCurve {
    #[gorge_field]
    pub value: f32,
}

impl ConstantFunctionCurve {
    pub fn new(value: f32) -> Self { Self { value } }
}

#[gorge_native_impl]
impl ConstantFunctionCurve {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, value: f32) {
        ctx.set_object_float_field(this, ConstantFunctionCurve::FIELD_INDEX_value, value as f64);
    }

    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, _x: f32) -> f32 {
        let v = ctx.get_object_float_field(this, ConstantFunctionCurve::FIELD_INDEX_value) as f32;
        v
    }
}

impl FunctionCurve for ConstantFunctionCurve {
    fn evaluate(&self, _x: f32) -> f32 { self.value }
}
