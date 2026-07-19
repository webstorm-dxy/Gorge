//! `GorgeFramework.ColorCurve` —— 颜色曲线抽象基类。
//!
//! 对齐 C# `ColorCurve`。无字段的抽象基类，定义 `Evaluate(float) -> ColorArgb`
//! 虚方法接口。子类包括 LerpColorCurve 等。
//!
//! 本类的注册使得其他 native 类可以将 ColorCurve 作为字段类型引用。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 颜色曲线抽象基类
///
/// 字段数为 0，仅作为类型层次中的根基类存在。
/// `evaluate` 返回 ColorArgb 对象 ID，默认返回 0（子类重写）。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct ColorCurve {
    /// 占位字段（解决宏对零字段类的方法生成问题）
    #[gorge_field]
    pub _placeholder: bool,
}

#[gorge_native_impl]
impl ColorCurve {
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, _placeholder: bool) {
        ctx.set_object_bool_field(this, Self::FIELD_INDEX__placeholder, _placeholder);
    }

    /// 实例方法 0：计算颜色曲线在 x 处的颜色值
    ///
    /// C# `ColorCurve.Evaluate(float)` 为 `virtual partial` 抽象基类方法，
    /// 抛出异常表明不应直接调用。返回 ColorArgb 对象 ID，基类占位返回 0。
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> usize {
        let _ = ctx;
        let _ = this;
        let _ = x;
        0
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
    fn test_color_curve_evaluate_placeholder() {
        let cc = ColorCurve { _placeholder: false };
        let mut fx = Fixture::new();

        fx.vm.param_pool.set_bool_param(0, false);
        let obj_id = {
            let mut ctx = fx.ctx();
            cc.do_construct_native(&mut ctx, None, 0)
        };

        // 调 evaluate(x=0.5)，基类应返回 0（对象 ID）
        fx.vm.param_pool.set_float_param(0, 0.5);
        {
            let mut ctx = fx.ctx();
            cc.invoke_native_method(&mut ctx, obj_id, 0);
        }
        assert_eq!(fx.vm.param_pool.get_object_return(), 0);
    }
}
