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

// ==================== 保留为 Rust trait 的组合器曲线（含 Box<dyn FunctionCurve> 字段） ====================

// 注：以下结构均含 Box<dyn FunctionCurve> 字段，不可注册为 native 类，
// 仅作为 Rust 内部类型使用。手动实现 Debug（不要求 Clone）。

#[derive(Debug)]
pub struct RustCompositeFunctionCurve {
    pub outer: Box<dyn FunctionCurve>,
    pub inner: Box<dyn FunctionCurve>,
}
impl RustCompositeFunctionCurve {
    pub fn new(outer: Box<dyn FunctionCurve>, inner: Box<dyn FunctionCurve>) -> Self { Self { outer, inner } }
}
impl FunctionCurve for RustCompositeFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 { self.outer.evaluate(self.inner.evaluate(x)) }
}

#[derive(Debug)]
pub struct RustAdditionFunctionCurve {
    pub first: Box<dyn FunctionCurve>,
    pub second: Box<dyn FunctionCurve>,
}
impl RustAdditionFunctionCurve {
    pub fn new(f: Box<dyn FunctionCurve>, s: Box<dyn FunctionCurve>) -> Self { Self { first: f, second: s } }
}
impl FunctionCurve for RustAdditionFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 { self.first.evaluate(x) + self.second.evaluate(x) }
}

#[derive(Debug)]
pub struct RustMultiplicationFunctionCurve {
    pub first: Box<dyn FunctionCurve>,
    pub second: Box<dyn FunctionCurve>,
}
impl RustMultiplicationFunctionCurve {
    pub fn new(f: Box<dyn FunctionCurve>, s: Box<dyn FunctionCurve>) -> Self { Self { first: f, second: s } }
}
impl FunctionCurve for RustMultiplicationFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 { self.first.evaluate(x) * self.second.evaluate(x) }
}

#[derive(Debug)]
pub struct RustPeriodicFunctionCurve {
    pub curve: Box<dyn FunctionCurve>,
    pub start_x: f32,
    pub end_x: f32,
    pub left_closed: bool,
}
impl RustPeriodicFunctionCurve {
    pub fn new(curve: Box<dyn FunctionCurve>, start_x: f32, end_x: f32) -> Self {
        Self { curve, start_x, end_x, left_closed: true }
    }
}
impl FunctionCurve for RustPeriodicFunctionCurve {
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
pub struct RustAxialSymmetricFunctionCurve {
    pub curve: Box<dyn FunctionCurve>,
    pub axis_center: f32,
    pub axis_amplitude: f32,
}
impl FunctionCurve for RustAxialSymmetricFunctionCurve {
    fn evaluate(&self, x: f32) -> f32 { self.curve.evaluate(self.axis_center + self.axis_amplitude - x) }
}

#[derive(Debug)]
pub struct RustFunctionPiece { pub curve: Box<dyn FunctionCurve>, pub start_x: f32, pub end_x: f32 }
impl FunctionCurve for RustFunctionPiece {
    fn evaluate(&self, x: f32) -> f32 {
        if x < self.start_x || x > self.end_x { 0.0 } else { self.curve.evaluate(x) }
    }
}

#[derive(Debug)]
pub struct RustPiecewiseFunctionCurve { pub pieces: Vec<RustFunctionPiece> }
impl FunctionCurve for RustPiecewiseFunctionCurve {
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
