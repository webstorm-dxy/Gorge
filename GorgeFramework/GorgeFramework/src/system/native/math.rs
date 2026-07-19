//! `GorgeFramework.Math` —— 数学工具 native 类（纯静态方法）。
//!
//! 移植自 C# 参考实现 `System/Native/Math.cs`，提供三角函数、角度转换、
//! 插值、钳制等常用数学运算。

use gorge_core::objective::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 数学工具类（无实例字段，仅提供静态方法）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Math {}

/// 圆周率常量
pub const PI: f32 = std::f32::consts::PI;

#[gorge_native_impl]
impl Math {
    /// 静态方法 0：绝对值
    #[gorge_static]
    pub fn abs(_ctx: &mut NativeContext, f: f32) -> f32 { f.abs() }

    /// 静态方法 1：平方根
    #[gorge_static]
    pub fn sqrt(_ctx: &mut NativeContext, f: f32) -> f32 { f.sqrt() }

    /// 静态方法 2：两数最大值
    #[gorge_static]
    pub fn max(_ctx: &mut NativeContext, a: f32, b: f32) -> f32 { a.max(b) }

    /// 静态方法 3：两数最小值
    #[gorge_static]
    pub fn min(_ctx: &mut NativeContext, a: f32, b: f32) -> f32 { a.min(b) }

    /// 静态方法 4：向下取整
    #[gorge_static]
    pub fn floor(_ctx: &mut NativeContext, f: f32) -> i32 { f.floor() as i32 }

    /// 静态方法 5：向上取整
    #[gorge_static]
    pub fn ceil(_ctx: &mut NativeContext, f: f32) -> i32 { f.ceil() as i32 }

    /// 静态方法 6：线性插值，t 钳制到 [0,1]
    #[gorge_static]
    pub fn lerp(_ctx: &mut NativeContext, a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t.clamp(0.0, 1.0)
    }

    /// 静态方法 7：钳制到 [min, max]
    #[gorge_static]
    pub fn clamp(_ctx: &mut NativeContext, value: f32, min: f32, max: f32) -> f32 {
        value.clamp(min, max)
    }

    /// 静态方法 8：整数钳制
    #[gorge_static]
    pub fn clamp_int(_ctx: &mut NativeContext, value: i32, min: i32, max: i32) -> i32 {
        value.clamp(min, max)
    }

    /// 静态方法 9：圆周率常量
    #[gorge_static]
    pub fn pi(_ctx: &mut NativeContext) -> f32 { PI }

    /// 静态方法 10：正弦
    #[gorge_static]
    pub fn sin(_ctx: &mut NativeContext, x: f32) -> f32 { x.sin() }

    /// 静态方法 11：余弦
    #[gorge_static]
    pub fn cos(_ctx: &mut NativeContext, x: f32) -> f32 { x.cos() }

    /// 静态方法 12：反正切
    #[gorge_static]
    pub fn atan(_ctx: &mut NativeContext, x: f32) -> f32 { x.atan() }

    /// 静态方法 13：正弦（度数）
    #[gorge_static]
    pub fn sin_deg(_ctx: &mut NativeContext, x: f32) -> f32 { x.to_radians().sin() }

    /// 静态方法 14：余弦（度数）
    #[gorge_static]
    pub fn cos_deg(_ctx: &mut NativeContext, x: f32) -> f32 { x.to_radians().cos() }

    /// 静态方法 15：角度转弧度
    #[gorge_static]
    pub fn deg2rad(_ctx: &mut NativeContext, deg: f32) -> f32 { deg.to_radians() }

    /// 静态方法 16：弧度转角度
    #[gorge_static]
    pub fn rad2deg(_ctx: &mut NativeContext, rad: f32) -> f32 { rad.to_degrees() }

    /// 静态方法 17：逆线性插值，vp 到 [a,b] 的比例，钳制到 [0,1]
    #[gorge_static]
    pub fn inverse_lerp(_ctx: &mut NativeContext, a: f32, b: f32, v: f32) -> f32 {
        ((v - a) / (b - a)).clamp(0.0, 1.0)
    }

    /// 静态方法 18：逆线性插值（不钳制）
    #[gorge_static]
    pub fn inverse_lerp_unclamp(_ctx: &mut NativeContext, a: f32, b: f32, v: f32) -> f32 {
        (v - a) / (b - a)
    }

    /// 静态方法 19：正无穷大
    #[gorge_static]
    pub fn float_positive_infinity(_ctx: &mut NativeContext) -> f32 { f32::INFINITY }

    /// 静态方法 20：负无穷大
    #[gorge_static]
    pub fn float_negative_infinity(_ctx: &mut NativeContext) -> f32 { f32::NEG_INFINITY }

    /// 静态方法 21：Deg2Rad 常量（PI/180，无参）
    ///
    /// 对齐 C# `Deg2Rad()`，返回角度转弧度的乘法因子，
    /// 与 `deg2rad(deg)` 的参数化版本互补。
    #[gorge_static]
    pub fn deg2rad_constant(_ctx: &mut NativeContext) -> f32 { std::f32::consts::PI / 180.0 }

    /// 静态方法 22：Rad2Deg 常量（180/PI，无参）
    ///
    /// 对齐 C# `Rad2Deg()`，返回弧度转角度的乘法因子。
    #[gorge_static]
    pub fn rad2deg_constant(_ctx: &mut NativeContext) -> f32 { 180.0 / std::f32::consts::PI }

    /// 静态方法 23：四参数最大值
    ///
    /// 对齐 C# `Max(f1,f2,f3,f4)`。
    #[gorge_static]
    pub fn max4(_ctx: &mut NativeContext, f1: f32, f2: f32, f3: f32, f4: f32) -> f32 {
        f1.max(f2).max(f3).max(f4)
    }

    /// 静态方法 24：四参数最小值
    ///
    /// 对齐 C# `Min(f1,f2)` 扩展（Gorge 无 params）。
    #[gorge_static]
    pub fn min4(_ctx: &mut NativeContext, f1: f32, f2: f32, f3: f32, f4: f32) -> f32 {
        f1.min(f2).min(f3).min(f4)
    }

    /// 静态方法 25：FloatArray 最大值
    ///
    /// 对齐 C# `Max(params float[])`，接收 FloatArray 对象 ID。
    #[gorge_static]
    pub fn max_array(ctx: &mut NativeContext, array_id: usize) -> f32 {
        let items = ctx.float_array_items(array_id);
        if items.is_empty() { return 0.0; }
        items.into_iter().fold(f32::NEG_INFINITY, |a, b| a.max(b as f32))
    }

    /// 静态方法 26：FloatArray 最小值
    ///
    /// 对齐 C# `Min(params float[])`，接收 FloatArray 对象 ID。
    #[gorge_static]
    pub fn min_array(ctx: &mut NativeContext, array_id: usize) -> f32 {
        let items = ctx.float_array_items(array_id);
        if items.is_empty() { return 0.0; }
        items.into_iter().fold(f32::INFINITY, |a, b| a.min(b as f32))
    }
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
    fn test_math_deg2rad_constant() {
        let m = Math {};
        let mut fx = Fixture::new();
        { let mut ctx = fx.ctx(); m.invoke_native_static(&mut ctx, 21); }
        let v = (fx.vm.param_pool.get_float_return() as f64) as f32;
        let expected = std::f32::consts::PI / 180.0;
        assert!((v - expected).abs() < 1e-6);
    }

    #[test]
    fn test_math_rad2deg_constant() {
        let m = Math {};
        let mut fx = Fixture::new();
        { let mut ctx = fx.ctx(); m.invoke_native_static(&mut ctx, 22); }
        let v = (fx.vm.param_pool.get_float_return() as f64) as f32;
        let expected = 180.0 / std::f32::consts::PI;
        assert!((v - expected).abs() < 1e-6);
    }

    #[test]
    fn test_math_max4() {
        let m = Math {};
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_float_param(0, 3.0);
        fx.vm.param_pool.set_float_param(1, 7.0);
        fx.vm.param_pool.set_float_param(2, 2.0);
        fx.vm.param_pool.set_float_param(3, 5.0);
        { let mut ctx = fx.ctx(); m.invoke_native_static(&mut ctx, 23); }
        let v = (fx.vm.param_pool.get_float_return() as f64) as f32;
        assert!((v - 7.0).abs() < 0.01);
    }

    #[test]
    fn test_math_min4() {
        let m = Math {};
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_float_param(0, 3.0);
        fx.vm.param_pool.set_float_param(1, 7.0);
        fx.vm.param_pool.set_float_param(2, -2.0);
        fx.vm.param_pool.set_float_param(3, 5.0);
        { let mut ctx = fx.ctx(); m.invoke_native_static(&mut ctx, 24); }
        let v = (fx.vm.param_pool.get_float_return() as f64) as f32;
        assert!((v - (-2.0)).abs() < 0.01);
    }

    #[test]
    fn test_math_max_array_empty() {
        let m = Math {};
        let mut fx = Fixture::new();
        // 创建空 FloatArray
        use gorge_core::system::native::array::FloatArrayClass;
        let cls = FloatArrayClass;
        let arr_id = { let mut ctx = fx.ctx(); cls.do_construct_native(&mut ctx, None, 0) };
        fx.vm.param_pool.set_object_param(0, arr_id);
        { let mut ctx = fx.ctx(); m.invoke_native_static(&mut ctx, 25); }
        let v = (fx.vm.param_pool.get_float_return() as f64) as f32;
        assert!((v - 0.0).abs() < 0.01, "空数组应返回 0.0");
    }
}
