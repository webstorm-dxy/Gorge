//! `GorgeFramework.FunctionCurve` — 函数曲线系统（native 类注册）。
//!
//! 移植自 C# 参考实现。简单字段曲线注册为 native 类供 Gorge 调用；
//! 含 trait 对象字段的组合器曲线保留为 Rust 类型。

use std::fmt::Debug;
use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 函数曲线 trait（内部 Rust 接口，不注册 native）
pub trait FunctionCurve: Debug + Send + Sync {
    fn evaluate(&self, x: f32) -> f32;
}

// ==================== FunctionCurve 基类（native 注册，虚方法分派） ====================

/// 函数曲线抽象基类（Rust 名 FunctionCurveNative，Gorge 名 FunctionCurve）
///
/// 对齐 C# `FunctionCurve`。无字段，evaluate 返回 0 作为默认实现，
/// 子类（ConstantFunctionCurve 等）通过重写 evaluate 提供真正计算。
/// Gorge 语言中通过虚方法分派到具体实现。
///
/// 注：Rust 内部已有 `trait FunctionCurve`，故 native struct 用 FunctionCurveNative 避免冲突；
/// 对外 Gorge 类名仍为 `GorgeFramework.FunctionCurve`。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct FunctionCurveNative {
    /// 占位字段（解决宏对零字段类的方法生成问题）
    #[gorge_field]
    pub _placeholder: bool,
}

#[gorge_native_impl]
impl FunctionCurveNative {
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, _placeholder: bool) {
        ctx.set_object_bool_field(this, Self::FIELD_INDEX__placeholder, _placeholder);
    }

    /// 实例方法 0：计算曲线在 x 处的值
    ///
    /// C# `FunctionCurve.Evaluate(float)` 为 `virtual partial` 抽象基类方法，
    /// 抛出异常表明不应直接调用。Rust 返回 0.0 作为占位，子类重写提供真正计算。
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let _ = ctx;
        let _ = this;
        let _ = x;
        0.0
    }
}

// ==================== 注册为 native 的简单曲线 ====================

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

/// 线段曲线：在 (timeStart, valueStart) ~ (timeEnd, valueEnd) 之间线性插值
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct LinearCurve {
    #[gorge_field]
    pub time_start: f32,
    #[gorge_field]
    pub value_start: f32,
    #[gorge_field]
    pub time_end: f32,
    #[gorge_field]
    pub value_end: f32,
}

impl LinearCurve {
    pub fn new(time_start: f32, value_start: f32, time_end: f32, value_end: f32) -> Self {
        Self { time_start, value_start, time_end, value_end }
    }
}

#[gorge_native_impl]
impl LinearCurve {
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

// ==================== CubicHermiteSpline（注册为 native，8 float 字段） ====================

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

// ==================== 保留为 Rust trait 的组合器曲线（含 Box<dyn FunctionCurve> 字段） ====================

// 注：以下结构均含 Box<dyn FunctionCurve> 字段，不可注册为 native 类，
// 仅作为 Rust 内部类型使用。手动实现 Debug（不要求 Clone）。

#[derive(Debug)]
pub struct CompositeFunctionCurve {
    pub outer: Box<dyn FunctionCurve>,
    pub inner: Box<dyn FunctionCurve>,
}
impl CompositeFunctionCurve {
    pub fn new(outer: Box<dyn FunctionCurve>, inner: Box<dyn FunctionCurve>) -> Self { Self { outer, inner } }
}
impl FunctionCurve for CompositeFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 { self.outer.evaluate(self.inner.evaluate(x)) }
}

#[derive(Debug)]
pub struct AdditionFunctionCurve {
    pub first: Box<dyn FunctionCurve>,
    pub second: Box<dyn FunctionCurve>,
}
impl AdditionFunctionCurve {
    pub fn new(f: Box<dyn FunctionCurve>, s: Box<dyn FunctionCurve>) -> Self { Self { first: f, second: s } }
}
impl FunctionCurve for AdditionFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 { self.first.evaluate(x) + self.second.evaluate(x) }
}

#[derive(Debug)]
pub struct MultiplicationFunctionCurve {
    pub first: Box<dyn FunctionCurve>,
    pub second: Box<dyn FunctionCurve>,
}
impl MultiplicationFunctionCurve {
    pub fn new(f: Box<dyn FunctionCurve>, s: Box<dyn FunctionCurve>) -> Self { Self { first: f, second: s } }
}
impl FunctionCurve for MultiplicationFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 { self.first.evaluate(x) * self.second.evaluate(x) }
}

#[derive(Debug)]
pub struct PeriodicFunctionCurve {
    pub curve: Box<dyn FunctionCurve>,
    pub start_x: f32,
    pub end_x: f32,
    pub left_closed: bool,
}
impl PeriodicFunctionCurve {
    pub fn new(curve: Box<dyn FunctionCurve>, start_x: f32, end_x: f32) -> Self {
        Self { curve, start_x, end_x, left_closed: true }
    }
}
impl FunctionCurve for PeriodicFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 {
        let period = self.end_x - self.start_x;
        if period.abs() < 1e-10 { return self.curve.evaluate(self.start_x); }
        let mut t = (x - self.start_x) % period;
        if t < 0.0 { t += period; }
        self.curve.evaluate(self.start_x + t)
    }
}

// ==================== 轴对称 + 分段 + 三次样条等（保留为 Rust 类型，未注册 native） ====================

#[derive(Debug)]
pub struct AxialSymmetricFunctionCurve {
    pub curve: Box<dyn FunctionCurve>,
    pub axis_center: f32,
    pub axis_amplitude: f32,
}
impl FunctionCurve for AxialSymmetricFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 { self.curve.evaluate(self.axis_center + self.axis_amplitude - x) }
}

#[derive(Debug)]
pub struct FunctionPiece { pub curve: Box<dyn FunctionCurve>, pub start_x: f32, pub end_x: f32 }
impl FunctionCurve for FunctionPiece {
    fn evaluate(&self, x: f32) -> f32 {
        if x < self.start_x || x > self.end_x { 0.0 } else { self.curve.evaluate(x) }
    }
}

#[derive(Debug)]
pub struct PiecewiseFunctionCurve { pub pieces: Vec<FunctionPiece> }
impl FunctionCurve for PiecewiseFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 {
        self.pieces.iter().find(|p| x >= p.start_x && x <= p.end_x).map(|p| p.curve.evaluate(x)).unwrap_or(0.0)
    }
}

/// 可变浮点数（含局部曲线 + 全局曲线）
#[derive(Debug)]
pub struct VariableFloat {
    pub base_value: f32,
    pub variation_curve: Box<dyn FunctionCurve>,
    pub global_curve: Box<dyn FunctionCurve>,
}
impl VariableFloat {
    pub fn evaluate_add(&self, x: f32) -> f32 { self.base_value + self.variation_curve.evaluate(x) }
    pub fn evaluate_multiply(&self, x: f32) -> f32 { self.base_value * self.variation_curve.evaluate(x) }
    pub fn evaluate_global(&self, x: f32) -> f32 { self.global_curve.evaluate(x) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    struct Fixture {
        vm: VirtualMachine,
    }

    impl Fixture {
        fn new() -> Self {
            let mut vm = VirtualMachine::new();
            vm.next_object_id = 1;
            Self { vm }
        }
        fn ctx(&mut self) -> NativeContext<'_> { NativeContext::new(&mut self.vm) }
    }

    #[test]
    fn test_function_curve_native_evaluate_placeholder() {
        let fc = FunctionCurveNative { _placeholder: false };
        let mut fx = Fixture::new();

        fx.vm.param_pool.set_bool_param(0, false);
        let obj_id = {
            let mut ctx = fx.ctx();
            fc.do_construct_native(&mut ctx, None, 0)
        };

        // 调 evaluate(x=1.0)，基类应返回 0.0
        fx.vm.param_pool.set_float_param(0, 1.0);
        {
            let mut ctx = fx.ctx();
            fc.invoke_native_method(&mut ctx, obj_id, 0);
        }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 0.0);
    }

    #[test]
    fn test_function_curve_native_evaluate_any_x_returns_zero() {
        let fc = FunctionCurveNative { _placeholder: false };
        let mut fx = Fixture::new();

        fx.vm.param_pool.set_bool_param(0, false);
        let obj_id = {
            let mut ctx = fx.ctx();
            fc.do_construct_native(&mut ctx, None, 0)
        };

        // 任意 x 均返回 0.0
        for x_val in [0.0_f32, -5.0, 3.5, 100.0] {
            fx.vm.param_pool.set_float_param(0, x_val as f64);
            {
                let mut ctx = fx.ctx();
                fc.invoke_native_method(&mut ctx, obj_id, 0);
            }
            assert_eq!(
                fx.vm.param_pool.get_float_return() as f32,
                0.0,
                "evaluate({}) should return 0.0", x_val
            );
        }
    }
}
