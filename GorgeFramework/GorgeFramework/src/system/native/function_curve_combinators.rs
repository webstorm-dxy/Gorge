//! `GorgeFramework` 函数曲线组合器 native 类。
//!
//! 移植自 C# 参考实现 `System/Native/` 下的组合器类。
//! 使用 `usize`（对象 ID）替代 `Box<dyn FunctionCurve>`，
//! 使 Gorge 语言可以通过 VM 创建和调用这些组合器。
//!
//! 每个组合器实现方法 0 = `evaluate(x: f32) -> f32`，
//! 与所有 FunctionCurve 子类保持一致的动态分派协议。

use gorge_core::objective::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

// ==================== AdditionFunctionCurve —— 加法组合 ====================

/// 加法组合曲线：f(x) = first.evaluate(x) + second.evaluate(x)
///
/// 子曲线 ID 为 0 时该项返回 0。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct AdditionFunctionCurve {
    /// 第一个加数曲线（FunctionCurve 对象 ID）
    #[gorge_field]
    #[inject(name = "firstFunctionCurve")]
    pub first: usize,
    /// 第二个加数曲线（FunctionCurve 对象 ID）
    #[gorge_field]
    #[inject(name = "secondFunctionCurve")]
    pub second: usize,
}

#[gorge_native_impl]
impl AdditionFunctionCurve {
    /// 构造方法 0：无参构造（子曲线由注入器字段提供）
    #[gorge_ctor]
    pub fn new_empty(ctx: &mut NativeContext, this: usize) { let _ = (ctx, this); }

    /// 构造方法 1：构造加法组合曲线
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, first: usize, second: usize) {
        ctx.set_object_object_field(this, Self::FIELD_INDEX_first, first);
        ctx.set_object_object_field(this, Self::FIELD_INDEX_second, second);
    }

    /// 计算 f(x) = first.evaluate(x) + second.evaluate(x)
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let first = ctx.get_object_object_field(this, Self::FIELD_INDEX_first);
        let second = ctx.get_object_object_field(this, Self::FIELD_INDEX_second);
        let r1 = if first == 0 {
            0.0
        } else {
            ctx.call_native_method_float_f(first, 0, x as f64) as f32
        };
        let r2 = if second == 0 {
            0.0
        } else {
            ctx.call_native_method_float_f(second, 0, x as f64) as f32
        };
        r1 + r2
    }
}

// ==================== MultiplicationFunctionCurve —— 乘法组合 ====================

/// 乘法组合曲线：f(x) = first.evaluate(x) * second.evaluate(x)
///
/// 子曲线 ID 为 0 时该项返回 0。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct MultiplicationFunctionCurve {
    /// 第一个乘数曲线（FunctionCurve 对象 ID）
    #[gorge_field]
    #[inject(name = "firstFunctionCurve")]
    pub first: usize,
    /// 第二个乘数曲线（FunctionCurve 对象 ID）
    #[gorge_field]
    #[inject(name = "secondFunctionCurve")]
    pub second: usize,
}

#[gorge_native_impl]
impl MultiplicationFunctionCurve {
    /// 构造方法 0：无参构造（子曲线由注入器字段提供）
    #[gorge_ctor]
    pub fn new_empty(ctx: &mut NativeContext, this: usize) { let _ = (ctx, this); }

    /// 构造方法 1：构造乘法组合曲线
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, first: usize, second: usize) {
        ctx.set_object_object_field(this, Self::FIELD_INDEX_first, first);
        ctx.set_object_object_field(this, Self::FIELD_INDEX_second, second);
    }

    /// 计算 f(x) = first.evaluate(x) * second.evaluate(x)
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let first = ctx.get_object_object_field(this, Self::FIELD_INDEX_first);
        let second = ctx.get_object_object_field(this, Self::FIELD_INDEX_second);
        let r1 = if first == 0 {
            0.0
        } else {
            ctx.call_native_method_float_f(first, 0, x as f64) as f32
        };
        let r2 = if second == 0 {
            0.0
        } else {
            ctx.call_native_method_float_f(second, 0, x as f64) as f32
        };
        r1 * r2
    }
}

// ==================== CompositeFunctionCurve —— 函数复合 ====================

/// 函数复合曲线：f(x) = outer.evaluate(inner.evaluate(x))
///
/// 子曲线 ID 为 0 时该项返回 0。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct CompositeFunctionCurve {
    /// 外层函数（FunctionCurve 对象 ID）
    #[gorge_field]
    #[inject(name = "outerFunctionCurve")]
    pub outer: usize,
    /// 内层函数（FunctionCurve 对象 ID）
    #[gorge_field]
    #[inject(name = "innerFunctionCurve")]
    pub inner: usize,
}

#[gorge_native_impl]
impl CompositeFunctionCurve {
    /// 构造方法 0：无参构造（子曲线由注入器字段提供）
    #[gorge_ctor]
    pub fn new_empty(ctx: &mut NativeContext, this: usize) { let _ = (ctx, this); }

    /// 构造方法 1：构造复合曲线
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, outer: usize, inner: usize) {
        ctx.set_object_object_field(this, Self::FIELD_INDEX_outer, outer);
        ctx.set_object_object_field(this, Self::FIELD_INDEX_inner, inner);
    }

    /// 计算 f(x) = outer.evaluate(inner.evaluate(x))
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let outer = ctx.get_object_object_field(this, Self::FIELD_INDEX_outer);
        let inner = ctx.get_object_object_field(this, Self::FIELD_INDEX_inner);
        if outer == 0 {
            return 0.0;
        }
        let inner_val = if inner == 0 {
            0.0
        } else {
            ctx.call_native_method_float_f(inner, 0, x as f64) as f32
        };
        ctx.call_native_method_float_f(outer, 0, inner_val as f64) as f32
    }
}

// ==================== PeriodicFunctionCurve —— 周期映射 ====================

/// 周期映射曲线：将 x 折叠到 [start_x, end_x] 周期区间后再求值
///
/// `left_closed` 为 true 时区间左包含（缺省，对齐 C# 默认），
/// 否则右包含。curve_id 为 0 时返回 0。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct PeriodicFunctionCurve {
    /// 被重复的曲线（FunctionCurve 对象 ID）
    #[gorge_field]
    #[inject(name = "functionCurve")]
    pub curve: usize,
    /// 周期左边界
    #[gorge_field]
    #[inject(name = "startX", default = 0.0)]
    pub start_x: f32,
    /// 周期右边界
    #[gorge_field]
    #[inject(name = "endX", default = 1.0)]
    pub end_x: f32,
    /// 左包含（否则为右包含）
    #[gorge_field]
    #[inject(name = "leftClosed", default = true)]
    pub left_closed: bool,
}

#[gorge_native_impl]
impl PeriodicFunctionCurve {
    /// 构造方法 0：无参构造（字段由注入器提供）
    #[gorge_ctor]
    pub fn new_empty(ctx: &mut NativeContext, this: usize) { let _ = (ctx, this); }

    /// 构造方法 1：构造周期映射曲线（left_closed 取注入器默认值 true）
    #[gorge_ctor]
    pub fn new(
        ctx: &mut NativeContext,
        this: usize,
        curve: usize,
        start_x: f32,
        end_x: f32,
    ) {
        ctx.set_object_object_field(this, Self::FIELD_INDEX_curve, curve);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_start_x, start_x as f64);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_end_x, end_x as f64);
    }

    /// 将 x 折叠到周期区间后求值（对齐 C#：leftClosed 决定左包含/右包含）
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let curve_id = ctx.get_object_object_field(this, Self::FIELD_INDEX_curve);
        if curve_id == 0 {
            return 0.0;
        }
        let start_x = ctx.get_object_float_field(this, Self::FIELD_INDEX_start_x) as f32;
        let end_x = ctx.get_object_float_field(this, Self::FIELD_INDEX_end_x) as f32;
        let left_closed = ctx.get_object_bool_field(this, Self::FIELD_INDEX_left_closed);
        let range = end_x - start_x;
        // f32 对 0 取余得到 NaN，周期退化为 0 时直接在左边界求值（C# 未处理该边界）
        if range.abs() < 1e-10 {
            return ctx.call_native_method_float_f(curve_id, 0, start_x as f64) as f32;
        }
        let mut real_x = (x - start_x) % range + start_x;
        if left_closed {
            if real_x < start_x {
                real_x += range;
            }
        } else if real_x <= start_x {
            real_x += range;
        }
        ctx.call_native_method_float_f(curve_id, 0, real_x as f64) as f32
    }
}

// ==================== AxialSymmetricFunctionCurve —— 轴对称 ====================

/// 轴对称曲线（对齐 C# `AxialSymmetricFunctionCurve`）
///
/// `keep_left` 为 true 时保留 axis 左侧、镜像到右侧，反之保留右侧镜像到左侧：
/// 对称侧取 `curve.evaluate(2 * axis - x)`。curve_id 为 0 时返回 0。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct AxialSymmetricFunctionCurve {
    /// 原始曲线（FunctionCurve 对象 ID）
    #[gorge_field]
    #[inject(name = "functionCurve")]
    pub curve: usize,
    /// 对称轴
    #[gorge_field]
    #[inject(name = "axis", default = 0.0)]
    pub axis: f32,
    /// 是否保留左侧而对称到右侧
    #[gorge_field]
    #[inject(name = "keepLeft", default = true)]
    pub keep_left: bool,
}

#[gorge_native_impl]
impl AxialSymmetricFunctionCurve {
    /// 构造方法 0：无参构造（字段由注入器提供）
    #[gorge_ctor]
    pub fn new_empty(ctx: &mut NativeContext, this: usize) { let _ = (ctx, this); }

    /// 构造方法 1：构造轴对称曲线
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, curve: usize, axis: f32, keep_left: bool) {
        ctx.set_object_object_field(this, Self::FIELD_INDEX_curve, curve);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_axis, axis as f64);
        ctx.set_object_bool_field(this, Self::FIELD_INDEX_keep_left, keep_left);
    }

    /// 计算轴对称后的曲线值（对齐 C#：keepLeft 决定保留侧）
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let curve_id = ctx.get_object_object_field(this, Self::FIELD_INDEX_curve);
        if curve_id == 0 {
            return 0.0;
        }
        let axis = ctx.get_object_float_field(this, Self::FIELD_INDEX_axis) as f32;
        let keep_left = ctx.get_object_bool_field(this, Self::FIELD_INDEX_keep_left);
        let on_kept_side = if keep_left { x <= axis } else { x >= axis };
        let mapped = if on_kept_side { x } else { axis - x + axis };
        ctx.call_native_method_float_f(curve_id, 0, mapped as f64) as f32
    }
}

// ==================== FunctionPiece —— 分段 ====================

/// 函数分段：仅在区间内求值，否则返回 0
///
/// 区间包含性由 `left_closed`/`right_closed` 决定
/// （默认左包含、右不包含，对齐 C# 注入器默认值）。
/// curve_id 为 0 时返回 0。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct FunctionPiece {
    /// 分段内的曲线（FunctionCurve 对象 ID）
    #[gorge_field]
    #[inject(name = "functionCurve")]
    pub curve: usize,
    /// 分段左边界
    #[gorge_field]
    #[inject(name = "startX", default = 0.0)]
    pub start_x: f32,
    /// 分段右边界
    #[gorge_field]
    #[inject(name = "endX", default = 1.0)]
    pub end_x: f32,
    /// 左边界包含
    #[gorge_field]
    #[inject(name = "leftClosed", default = true)]
    pub left_closed: bool,
    /// 右边界包含
    #[gorge_field]
    #[inject(name = "rightClosed", default = false)]
    pub right_closed: bool,
}

#[gorge_native_impl]
impl FunctionPiece {
    /// 构造方法 0：无参构造（字段由注入器提供）
    #[gorge_ctor]
    pub fn new_empty(ctx: &mut NativeContext, this: usize) { let _ = (ctx, this); }

    /// 构造方法 1：构造函数分段（left_closed/right_closed 取注入器默认值）
    #[gorge_ctor]
    pub fn new(
        ctx: &mut NativeContext,
        this: usize,
        curve: usize,
        start_x: f32,
        end_x: f32,
    ) {
        ctx.set_object_object_field(this, Self::FIELD_INDEX_curve, curve);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_start_x, start_x as f64);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_end_x, end_x as f64);
    }

    /// 判断 x 是否落在分段区间内（按 left_closed/right_closed 决定边界包含性）
    fn contains_x(ctx: &mut NativeContext, piece_id: usize, x: f32) -> bool {
        let start_x = ctx.get_object_float_field(piece_id, Self::FIELD_INDEX_start_x) as f32;
        let end_x = ctx.get_object_float_field(piece_id, Self::FIELD_INDEX_end_x) as f32;
        let left_closed = ctx.get_object_bool_field(piece_id, Self::FIELD_INDEX_left_closed);
        let right_closed = ctx.get_object_bool_field(piece_id, Self::FIELD_INDEX_right_closed);
        let after_left = if left_closed { x >= start_x } else { x > start_x };
        let before_right = if right_closed { x <= end_x } else { x < end_x };
        after_left && before_right
    }

    /// 若 x 在分段区间内则返回曲线值，否则返回 0
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        if !Self::contains_x(ctx, this, x) {
            return 0.0;
        }
        let curve_id = ctx.get_object_object_field(this, Self::FIELD_INDEX_curve);
        if curve_id == 0 {
            return 0.0;
        }
        ctx.call_native_method_float_f(curve_id, 0, x as f64) as f32
    }
}

// ==================== PiecewiseFunctionCurve —— 分段函数 ====================

/// 分段函数曲线：由多个 FunctionPiece 组成，按区间匹配求值
///
/// pieces 为 ObjectArray 对象 ID，存储 FunctionPiece 对象 ID 列表。
/// 遍历所有分段，在第一个匹配的区间内求值并返回，无匹配则返回 0；
/// 遇到空分段（对象 ID 为 0）时直接返回 0（对齐 C#）。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct PiecewiseFunctionCurve {
    /// 分段列表（ObjectArray 对象 ID）
    #[gorge_field]
    #[inject(name = "functionPieces")]
    pub pieces: usize,
}

#[gorge_native_impl]
impl PiecewiseFunctionCurve {
    /// 构造方法 0：无参构造（分段列表由注入器字段提供）
    #[gorge_ctor]
    pub fn new_empty(ctx: &mut NativeContext, this: usize) { let _ = (ctx, this); }

    /// 构造方法 1：构造分段函数曲线
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, pieces: usize) {
        ctx.set_object_object_field(this, Self::FIELD_INDEX_pieces, pieces);
    }

    /// 遍历分段，返回第一个匹配区间的曲线值
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let pieces_id = ctx.get_object_object_field(this, Self::FIELD_INDEX_pieces);
        if pieces_id == 0 {
            return 0.0;
        }
        let items = ctx.object_array_items(pieces_id);
        for piece_id in &items {
            // 对齐 C#：空分段直接返回 0
            if *piece_id == 0 {
                return 0.0;
            }
            if FunctionPiece::contains_x(ctx, *piece_id, x) {
                let curve_id =
                    ctx.get_object_object_field(*piece_id, FunctionPiece::FIELD_INDEX_curve);
                if curve_id == 0 {
                    return 0.0;
                }
                return ctx.call_native_method_float_f(curve_id, 0, x as f64) as f32;
            }
        }
        0.0
    }
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::object::RuntimeObject;
    use gorge_core::system::native::array::ObjectArrayClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;
    use crate::system::native::constant_function_curve::ConstantFunctionCurve;
    use crate::system::native::linear_function_curve::LinearFunctionCurve;

    struct Fixture {
        vm: VirtualMachine,
    }

    impl Fixture {
        fn new() -> Self {
            let mut vm = VirtualMachine::new();
            vm.next_object_id = 1;
            Self { vm }
        }

        fn ctx(&mut self) -> NativeContext<'_> {
            NativeContext::new(&mut self.vm)
        }

        fn register_class(&mut self, cls: std::sync::Arc<dyn NativeClass>) {
            let name = cls.full_name().to_string();
            self.vm.register_native_class(&name, cls);
        }
    }

    /// 创建一个 ConstantFunctionCurve 对象并返回其 ID（手动方式，不经过构造方法）
    fn make_constant_curve(fx: &mut Fixture, value: f32) -> usize {
        let obj = RuntimeObject::new_simple(
            ConstantFunctionCurve::GORGE_FULL_NAME.to_string(),
            &ConstantFunctionCurve::gorge_field_type_count(),
        );
        let id = { let mut ctx = fx.ctx(); ctx.register_object(obj) };
        {
            let mut ctx = fx.ctx();
            ctx.set_object_float_field(
                id,
                ConstantFunctionCurve::FIELD_INDEX_value,
                value as f64,
            );
        }
        id
    }

    /// 创建一个 LinearFunctionCurve(k, 0) 对象并返回其 ID（手动方式）
    fn make_linear_curve(fx: &mut Fixture, k: f32) -> usize {
        let obj = RuntimeObject::new_simple(
            LinearFunctionCurve::GORGE_FULL_NAME.to_string(),
            &LinearFunctionCurve::gorge_field_type_count(),
        );
        let id = { let mut ctx = fx.ctx(); ctx.register_object(obj) };
        {
            let mut ctx = fx.ctx();
            ctx.set_object_float_field(id, LinearFunctionCurve::FIELD_INDEX_k, k as f64);
            ctx.set_object_float_field(id, LinearFunctionCurve::FIELD_INDEX_b, 0.0);
        }
        id
    }

    /// 创建一个 FunctionPiece 对象并返回其 ID（手动方式，区间双闭）
    fn make_piece(fx: &mut Fixture, curve_id: usize, start_x: f32, end_x: f32) -> usize {
        let obj = RuntimeObject::new_simple(
            FunctionPiece::GORGE_FULL_NAME.to_string(),
            &FunctionPiece::gorge_field_type_count(),
        );
        let id = { let mut ctx = fx.ctx(); ctx.register_object(obj) };
        {
            let mut ctx = fx.ctx();
            ctx.set_object_object_field(id, FunctionPiece::FIELD_INDEX_curve, curve_id);
            ctx.set_object_float_field(
                id,
                FunctionPiece::FIELD_INDEX_start_x,
                start_x as f64,
            );
            ctx.set_object_float_field(id, FunctionPiece::FIELD_INDEX_end_x, end_x as f64);
            ctx.set_object_bool_field(id, FunctionPiece::FIELD_INDEX_left_closed, true);
            ctx.set_object_bool_field(id, FunctionPiece::FIELD_INDEX_right_closed, true);
        }
        id
    }

    // ==================== 注入器字段元数据与默认值测试 ====================

    #[test]
    fn test_periodic_injector_fields_meta_and_defaults() {
        // 注入器字段名与声明序对齐谱面存根
        let meta = PeriodicFunctionCurve::gorge_injector_fields_meta();
        let names: Vec<&str> = meta.iter().map(|(name, _)| *name).collect();
        assert_eq!(names, ["functionCurve", "startX", "endX", "leftClosed"]);
        // 默认值对齐 C#：startX=0、endX=1、leftClosed=true
        assert_eq!(PeriodicFunctionCurve::gorge_injector_default_start_x(), 0.0);
        assert_eq!(PeriodicFunctionCurve::gorge_injector_default_end_x(), 1.0);
        assert!(PeriodicFunctionCurve::gorge_injector_default_left_closed());
    }

    #[test]
    fn test_function_piece_injector_fields_meta_and_defaults() {
        let meta = FunctionPiece::gorge_injector_fields_meta();
        let names: Vec<&str> = meta.iter().map(|(name, _)| *name).collect();
        assert_eq!(
            names,
            ["functionCurve", "startX", "endX", "leftClosed", "rightClosed"]
        );
        // 默认值对齐 C#：左包含、右不包含
        assert!(FunctionPiece::gorge_injector_default_left_closed());
        assert!(!FunctionPiece::gorge_injector_default_right_closed());
    }

    #[test]
    fn test_axial_symmetric_injector_fields_meta() {
        let meta = AxialSymmetricFunctionCurve::gorge_injector_fields_meta();
        let names: Vec<&str> = meta.iter().map(|(name, _)| *name).collect();
        assert_eq!(names, ["functionCurve", "axis", "keepLeft"]);
        assert!(AxialSymmetricFunctionCurve::gorge_injector_default_keep_left());
    }

    // ==================== AdditionFunctionCurve 测试 ====================

    #[test]
    fn test_addition_both_zero_returns_zero() {
        let add = AdditionFunctionCurve {
            first: 0,
            second: 0,
        };
        let mut fx = Fixture::new();

        fx.vm.param_pool.set_object_param(0, 0);
        fx.vm.param_pool.set_object_param(1, 0);
        let obj_id = { let mut ctx = fx.ctx(); add.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, 5.0_f64);
        { let mut ctx = fx.ctx(); add.invoke_native_method(&mut ctx, obj_id, 0); }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 0.0);
    }

    #[test]
    fn test_addition_with_real_curves() {
        let add = AdditionFunctionCurve {
            first: 0,
            second: 0,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));
        fx.register_class(std::sync::Arc::new(AdditionFunctionCurve {
            first: 0,
            second: 0,
        }));

        let c1 = make_constant_curve(&mut fx, 3.0);
        let c2 = make_constant_curve(&mut fx, 7.0);

        fx.vm.param_pool.set_object_param(0, c1);
        fx.vm.param_pool.set_object_param(1, c2);
        let obj_id = { let mut ctx = fx.ctx(); add.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, 0.0_f64);
        { let mut ctx = fx.ctx(); add.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_addition_first_zero() {
        let add = AdditionFunctionCurve {
            first: 0,
            second: 0,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));
        fx.register_class(std::sync::Arc::new(AdditionFunctionCurve {
            first: 0,
            second: 0,
        }));

        let c2 = make_constant_curve(&mut fx, 5.0);

        fx.vm.param_pool.set_object_param(0, 0);
        fx.vm.param_pool.set_object_param(1, c2);
        let obj_id = { let mut ctx = fx.ctx(); add.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, 1.0_f64);
        { let mut ctx = fx.ctx(); add.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 5.0).abs() < 0.001);
    }

    // ==================== MultiplicationFunctionCurve 测试 ====================

    #[test]
    fn test_multiplication_both_zero_returns_zero() {
        let mul = MultiplicationFunctionCurve {
            first: 0,
            second: 0,
        };
        let mut fx = Fixture::new();

        fx.vm.param_pool.set_object_param(0, 0);
        fx.vm.param_pool.set_object_param(1, 0);
        let obj_id = { let mut ctx = fx.ctx(); mul.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, 5.0_f64);
        { let mut ctx = fx.ctx(); mul.invoke_native_method(&mut ctx, obj_id, 0); }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 0.0);
    }

    #[test]
    fn test_multiplication_with_real_curves() {
        let mul = MultiplicationFunctionCurve {
            first: 0,
            second: 0,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));
        fx.register_class(std::sync::Arc::new(MultiplicationFunctionCurve {
            first: 0,
            second: 0,
        }));

        let c1 = make_constant_curve(&mut fx, 4.0);
        let c2 = make_constant_curve(&mut fx, 0.5);

        fx.vm.param_pool.set_object_param(0, c1);
        fx.vm.param_pool.set_object_param(1, c2);
        let obj_id = { let mut ctx = fx.ctx(); mul.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, 0.0_f64);
        { let mut ctx = fx.ctx(); mul.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 2.0).abs() < 0.001);
    }

    // ==================== CompositeFunctionCurve 测试 ====================

    #[test]
    fn test_composite_outer_zero_returns_zero() {
        let comp = CompositeFunctionCurve {
            outer: 0,
            inner: 0,
        };
        let mut fx = Fixture::new();

        fx.vm.param_pool.set_object_param(0, 0);
        fx.vm.param_pool.set_object_param(1, 0);
        let obj_id = { let mut ctx = fx.ctx(); comp.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, 5.0_f64);
        { let mut ctx = fx.ctx(); comp.invoke_native_method(&mut ctx, obj_id, 0); }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 0.0);
    }

    #[test]
    fn test_composite_with_real_curves() {
        let comp = CompositeFunctionCurve {
            outer: 0,
            inner: 0,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));
        fx.register_class(std::sync::Arc::new(CompositeFunctionCurve {
            outer: 0,
            inner: 0,
        }));

        // outer=Constant(9.0), inner=Constant(2.0) → outer(inner(x)) = outer(2.0) = 9.0
        let outer_id = make_constant_curve(&mut fx, 9.0);
        let inner_id = make_constant_curve(&mut fx, 2.0);

        fx.vm.param_pool.set_object_param(0, outer_id);
        fx.vm.param_pool.set_object_param(1, inner_id);
        let obj_id = { let mut ctx = fx.ctx(); comp.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, 0.0_f64);
        { let mut ctx = fx.ctx(); comp.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 9.0).abs() < 0.001);
    }

    #[test]
    fn test_composite_inner_zero_outer_gets_zero() {
        let comp = CompositeFunctionCurve {
            outer: 0,
            inner: 0,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));
        fx.register_class(std::sync::Arc::new(CompositeFunctionCurve {
            outer: 0,
            inner: 0,
        }));

        let outer_id = make_constant_curve(&mut fx, 7.0);

        fx.vm.param_pool.set_object_param(0, outer_id);
        fx.vm.param_pool.set_object_param(1, 0);
        let obj_id = { let mut ctx = fx.ctx(); comp.do_construct_native(&mut ctx, None, 1) };

        // inner=0 → inner_val=0 → outer(0) = 7.0
        fx.vm.param_pool.set_float_param(0, 3.0_f64);
        { let mut ctx = fx.ctx(); comp.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 7.0).abs() < 0.001);
    }

    // ==================== PeriodicFunctionCurve 测试 ====================

    #[test]
    fn test_periodic_curve_zero_returns_zero() {
        let p = PeriodicFunctionCurve {
            curve: 0,
            start_x: 0.0,
            end_x: 10.0,
            left_closed: true,
        };
        let mut fx = Fixture::new();

        fx.vm.param_pool.set_object_param(0, 0);
        fx.vm.param_pool.set_float_param(0, 0.0);
        fx.vm.param_pool.set_float_param(1, 10.0);
        let obj_id = { let mut ctx = fx.ctx(); p.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, 5.0_f64);
        { let mut ctx = fx.ctx(); p.invoke_native_method(&mut ctx, obj_id, 0); }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 0.0);
    }

    #[test]
    fn test_periodic_folds_x_into_interval() {
        let p = PeriodicFunctionCurve {
            curve: 0,
            start_x: 0.0,
            end_x: 0.0,
            left_closed: true,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));
        fx.register_class(std::sync::Arc::new(PeriodicFunctionCurve {
            curve: 0,
            start_x: 0.0,
            end_x: 0.0,
            left_closed: true,
        }));

        // curve = Constant(42.0)，无论 x 映射后是哪，都返回 42.0
        let cid = make_constant_curve(&mut fx, 42.0);

        fx.vm.param_pool.set_object_param(0, cid);
        fx.vm.param_pool.set_float_param(0, 0.0);
        fx.vm.param_pool.set_float_param(1, 10.0);
        let obj_id = { let mut ctx = fx.ctx(); p.do_construct_native(&mut ctx, None, 1) };

        // x=3 在区间内
        fx.vm.param_pool.set_float_param(0, 3.0_f64);
        { let mut ctx = fx.ctx(); p.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 42.0).abs() < 0.001);
    }

    #[test]
    fn test_periodic_left_closed_folds_negative_x() {
        // left_closed=true（默认）：x=-1 折叠到周期区间末尾一侧
        let p = PeriodicFunctionCurve {
            curve: 0,
            start_x: 0.0,
            end_x: 10.0,
            left_closed: true,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(LinearFunctionCurve { k: 0.0, b: 0.0 }));
        fx.register_class(std::sync::Arc::new(PeriodicFunctionCurve {
            curve: 0,
            start_x: 0.0,
            end_x: 0.0,
            left_closed: true,
        }));

        // curve = f(t) = t；x=-1 → realX = -1 % 10 + 0 = -1 < 0 → +10 → 9
        let cid = make_linear_curve(&mut fx, 1.0);

        fx.vm.param_pool.set_object_param(0, cid);
        fx.vm.param_pool.set_float_param(0, 0.0);
        fx.vm.param_pool.set_float_param(1, 10.0);
        let obj_id = { let mut ctx = fx.ctx(); p.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, -1.0_f64);
        { let mut ctx = fx.ctx(); p.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 9.0).abs() < 0.001);
    }

    #[test]
    fn test_periodic_zero_period_returns_curve_at_start() {
        let p = PeriodicFunctionCurve {
            curve: 0,
            start_x: 5.0,
            end_x: 5.0,
            left_closed: true,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));
        fx.register_class(std::sync::Arc::new(PeriodicFunctionCurve {
            curve: 0,
            start_x: 0.0,
            end_x: 0.0,
            left_closed: true,
        }));

        let cid = make_constant_curve(&mut fx, 99.0);

        fx.vm.param_pool.set_object_param(0, cid);
        fx.vm.param_pool.set_float_param(0, 5.0);
        fx.vm.param_pool.set_float_param(1, 5.0);
        let obj_id = { let mut ctx = fx.ctx(); p.do_construct_native(&mut ctx, None, 1) };

        // period = 0 → curve.evaluate(start_x) = curve.evaluate(5) = 99.0
        fx.vm.param_pool.set_float_param(0, 7.0_f64);
        { let mut ctx = fx.ctx(); p.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 99.0).abs() < 0.001);
    }

    // ==================== AxialSymmetricFunctionCurve 测试 ====================

    #[test]
    fn test_axial_symmetric_curve_zero_returns_zero() {
        let a = AxialSymmetricFunctionCurve {
            curve: 0,
            axis: 0.0,
            keep_left: true,
        };
        let mut fx = Fixture::new();

        fx.vm.param_pool.set_object_param(0, 0);
        fx.vm.param_pool.set_float_param(0, 0.0);
        fx.vm.param_pool.set_bool_param(0, true);
        let obj_id = { let mut ctx = fx.ctx(); a.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, 1.0_f64);
        { let mut ctx = fx.ctx(); a.invoke_native_method(&mut ctx, obj_id, 0); }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 0.0);
    }

    #[test]
    fn test_axial_symmetric_with_constant_curve() {
        let a = AxialSymmetricFunctionCurve {
            curve: 0,
            axis: 0.0,
            keep_left: true,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));
        fx.register_class(std::sync::Arc::new(AxialSymmetricFunctionCurve {
            curve: 0,
            axis: 0.0,
            keep_left: true,
        }));

        // curve=Constant(10.0), axis=2, keep_left=true
        // x=100 > 2 → curve.evaluate(2*2-100)=curve(-96)=10.0（常值曲线忽略参数）
        let cid = make_constant_curve(&mut fx, 10.0);

        fx.vm.param_pool.set_object_param(0, cid);
        fx.vm.param_pool.set_float_param(0, 2.0);
        fx.vm.param_pool.set_bool_param(0, true);
        let obj_id = { let mut ctx = fx.ctx(); a.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, 100.0_f64);
        { let mut ctx = fx.ctx(); a.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_axial_symmetric_keep_right_mirrors_left_side() {
        // keep_left=false：保留 axis 右侧，左侧镜像到右侧
        let a = AxialSymmetricFunctionCurve {
            curve: 0,
            axis: 0.0,
            keep_left: false,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(LinearFunctionCurve { k: 0.0, b: 0.0 }));
        fx.register_class(std::sync::Arc::new(AxialSymmetricFunctionCurve {
            curve: 0,
            axis: 0.0,
            keep_left: true,
        }));

        // curve = f(t) = t；axis=0、keep_left=false，x=-3 → curve(0-(-3)+0)=curve(3)=3
        let cid = make_linear_curve(&mut fx, 1.0);

        fx.vm.param_pool.set_object_param(0, cid);
        fx.vm.param_pool.set_float_param(0, 0.0);
        fx.vm.param_pool.set_bool_param(0, false);
        let obj_id = { let mut ctx = fx.ctx(); a.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, -3.0_f64);
        { let mut ctx = fx.ctx(); a.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 3.0).abs() < 0.001);
    }

    // ==================== FunctionPiece 测试 ====================

    #[test]
    fn test_function_piece_in_range() {
        let piece = FunctionPiece {
            curve: 0,
            start_x: 0.0,
            end_x: 0.0,
            left_closed: true,
            right_closed: false,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));
        fx.register_class(std::sync::Arc::new(FunctionPiece {
            curve: 0,
            start_x: 0.0,
            end_x: 0.0,
            left_closed: true,
            right_closed: false,
        }));

        let cid = make_constant_curve(&mut fx, 25.0);

        fx.vm.param_pool.set_object_param(0, cid);
        fx.vm.param_pool.set_float_param(0, 0.0);
        fx.vm.param_pool.set_float_param(1, 10.0);
        let obj_id = { let mut ctx = fx.ctx(); piece.do_construct_native(&mut ctx, None, 1) };

        // x=5 在 [0,10) 内 → 25.0
        fx.vm.param_pool.set_float_param(0, 5.0_f64);
        { let mut ctx = fx.ctx(); piece.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_function_piece_out_of_range() {
        let piece = FunctionPiece {
            curve: 0,
            start_x: 0.0,
            end_x: 0.0,
            left_closed: true,
            right_closed: false,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));
        fx.register_class(std::sync::Arc::new(FunctionPiece {
            curve: 0,
            start_x: 0.0,
            end_x: 0.0,
            left_closed: true,
            right_closed: false,
        }));

        let cid = make_constant_curve(&mut fx, 25.0);

        fx.vm.param_pool.set_object_param(0, cid);
        fx.vm.param_pool.set_float_param(0, 0.0);
        fx.vm.param_pool.set_float_param(1, 10.0);
        let obj_id = { let mut ctx = fx.ctx(); piece.do_construct_native(&mut ctx, None, 1) };

        // x=15 不在 [0,10) 内 → 0.0
        fx.vm.param_pool.set_float_param(0, 15.0_f64);
        { let mut ctx = fx.ctx(); piece.invoke_native_method(&mut ctx, obj_id, 0); }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 0.0);
    }

    #[test]
    fn test_function_piece_curve_zero_returns_zero() {
        let piece = FunctionPiece {
            curve: 0,
            start_x: 0.0,
            end_x: 0.0,
            left_closed: true,
            right_closed: false,
        };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(FunctionPiece {
            curve: 0,
            start_x: 0.0,
            end_x: 0.0,
            left_closed: true,
            right_closed: false,
        }));

        fx.vm.param_pool.set_object_param(0, 0);
        fx.vm.param_pool.set_float_param(0, 0.0);
        fx.vm.param_pool.set_float_param(1, 10.0);
        let obj_id = { let mut ctx = fx.ctx(); piece.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, 5.0_f64);
        { let mut ctx = fx.ctx(); piece.invoke_native_method(&mut ctx, obj_id, 0); }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 0.0);
    }

    // ==================== PiecewiseFunctionCurve 测试 ====================

    #[test]
    fn test_piecewise_empty_pieces_returns_zero() {
        let pfc = PiecewiseFunctionCurve { pieces: 0 };
        let mut fx = Fixture::new();

        fx.vm.param_pool.set_object_param(0, 0);
        let obj_id = { let mut ctx = fx.ctx(); pfc.do_construct_native(&mut ctx, None, 1) };

        fx.vm.param_pool.set_float_param(0, 5.0_f64);
        { let mut ctx = fx.ctx(); pfc.invoke_native_method(&mut ctx, obj_id, 0); }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 0.0);
    }

    #[test]
    fn test_piecewise_matches_correct_piece() {
        let pfc = PiecewiseFunctionCurve { pieces: 0 };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));
        fx.register_class(std::sync::Arc::new(FunctionPiece {
            curve: 0,
            start_x: 0.0,
            end_x: 0.0,
            left_closed: true,
            right_closed: false,
        }));
        fx.register_class(std::sync::Arc::new(PiecewiseFunctionCurve { pieces: 0 }));

        // 创建两条曲线和两个分段
        let c1 = make_constant_curve(&mut fx, 10.0);
        let c2 = make_constant_curve(&mut fx, 20.0);
        let p1 = make_piece(&mut fx, c1, 0.0, 5.0); // [0,5] → 10.0
        let p2 = make_piece(&mut fx, c2, 6.0, 10.0); // [6,10] → 20.0

        // 创建 ObjectArray 并添加分段
        let cls = ObjectArrayClass;
        let arr_id = { let mut ctx = fx.ctx(); cls.do_construct_native(&mut ctx, None, 0) };
        {
            let mut ctx = fx.ctx();
            ctx.object_array_add(arr_id, p1);
            ctx.object_array_add(arr_id, p2);
        }

        fx.vm.param_pool.set_object_param(0, arr_id);
        let obj_id = { let mut ctx = fx.ctx(); pfc.do_construct_native(&mut ctx, None, 1) };

        // x=3 在 [0,5] 内 → 第一条曲线 10.0
        fx.vm.param_pool.set_float_param(0, 3.0_f64);
        { let mut ctx = fx.ctx(); pfc.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 10.0).abs() < 0.001);

        // x=7 在 [6,10] 内 → 第二条曲线 20.0
        fx.vm.param_pool.set_float_param(0, 7.0_f64);
        { let mut ctx = fx.ctx(); pfc.invoke_native_method(&mut ctx, obj_id, 0); }
        assert!((fx.vm.param_pool.get_float_return() as f32 - 20.0).abs() < 0.001);
    }

    #[test]
    fn test_piecewise_no_match_returns_zero() {
        let pfc = PiecewiseFunctionCurve { pieces: 0 };
        let mut fx = Fixture::new();
        fx.register_class(std::sync::Arc::new(ConstantFunctionCurve { value: 0.0 }));
        fx.register_class(std::sync::Arc::new(FunctionPiece {
            curve: 0,
            start_x: 0.0,
            end_x: 0.0,
            left_closed: true,
            right_closed: false,
        }));
        fx.register_class(std::sync::Arc::new(PiecewiseFunctionCurve { pieces: 0 }));

        let c1 = make_constant_curve(&mut fx, 10.0);
        let p1 = make_piece(&mut fx, c1, 0.0, 5.0);

        let cls = ObjectArrayClass;
        let arr_id = { let mut ctx = fx.ctx(); cls.do_construct_native(&mut ctx, None, 0) };
        {
            let mut ctx = fx.ctx();
            ctx.object_array_add(arr_id, p1);
        }

        fx.vm.param_pool.set_object_param(0, arr_id);
        let obj_id = { let mut ctx = fx.ctx(); pfc.do_construct_native(&mut ctx, None, 1) };

        // x=99 不在任何分段内 → 0.0
        fx.vm.param_pool.set_float_param(0, 99.0_f64);
        { let mut ctx = fx.ctx(); pfc.invoke_native_method(&mut ctx, obj_id, 0); }
        assert_eq!(fx.vm.param_pool.get_float_return() as f32, 0.0);
    }
}
