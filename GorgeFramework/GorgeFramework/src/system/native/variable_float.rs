//! `GorgeFramework.VariableFloat` —— 可变浮点数 native 类。
//!
//! 对齐 C# `VariableFloat`。组合"基值 + 变化曲线"，支持
//! `EvaluateAdd`（基值 + 曲线求值）和 `EvaluateDoubleLerp`（双向插值）
//! 两种求值模式。

use gorge_core::objective::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 可变浮点数：基值 + 变化曲线
///
/// `variation_curve` 为 FunctionCurve 对象 ID（0 表示无曲线，退化为常值）。
/// 常用于谱面参数的动态控制（如宽度、速度随时间变化）。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct VariableFloat {
    /// 基值（C#/存根未声明默认值，未注入时回退 Rust 默认 0.0）
    #[gorge_field]
    #[inject(name = "baseValue")]
    pub base_value: f32,

    /// 变化曲线（FunctionCurve 对象 ID，0 表示无曲线）
    #[gorge_field]
    #[inject(name = "variationCurve")]
    pub variation_curve: usize,
}

#[gorge_native_impl]
impl VariableFloat {
    /// 构造方法 0：无参构造（字段由注入器提供）
    #[gorge_ctor]
    pub fn new_empty(ctx: &mut NativeContext, this: usize) { let _ = (ctx, this); }

    /// 构造方法 1：基值 + 可选曲线
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, base_value: f32, variation_curve: usize) {
        ctx.set_object_float_field(this, Self::FIELD_INDEX_base_value, base_value as f64);
        ctx.set_object_object_field(this, Self::FIELD_INDEX_variation_curve, variation_curve);
    }

    /// 加值曲线求值：`baseValue + curve.Evaluate(curveTime)`
    ///
    /// 若 variation_curve 为 0（无曲线），直接返回基值。
    #[gorge_method]
    pub fn evaluate_add(ctx: &mut NativeContext, this: usize, curve_time: f32) -> f32 {
        let base = ctx.get_object_float_field(this, Self::FIELD_INDEX_base_value) as f32;
        let curve_id = ctx.get_object_object_field(this, Self::FIELD_INDEX_variation_curve);
        if curve_id == 0 {
            return base;
        }
        let v = ctx.call_native_method_float_f(curve_id, 0, curve_time as f64) as f32;
        base + v
    }

    /// 双向插值曲线求值
    ///
    /// - `curve.Evaluate(curveTime) = 0` → 返回基值
    /// - `curve.Evaluate(curveTime) > 0` → `Math.Lerp(baseValue, max, value)`
    /// - `curve.Evaluate(curveTime) < 0` → `Math.Lerp(min, baseValue, value + 1)`
    #[gorge_method]
    pub fn evaluate_double_lerp(
        ctx: &mut NativeContext,
        this: usize,
        curve_time: f32,
        min: f32,
        max: f32,
    ) -> f32 {
        let base = ctx.get_object_float_field(this, Self::FIELD_INDEX_base_value) as f32;
        let curve_id = ctx.get_object_object_field(this, Self::FIELD_INDEX_variation_curve);
        if curve_id == 0 {
            return base;
        }
        let v = ctx.call_native_method_float_f(curve_id, 0, curve_time as f64) as f32;
        if v > 0.0 {
            // lerp(base, max, v): base + (max - base) * v
            base + (max - base) * v
        } else if v < 0.0 {
            // lerp(min, base, v + 1): min + (base - min) * (v + 1)
            min + (base - min) * (v + 1.0)
        } else {
            base
        }
    }
}
