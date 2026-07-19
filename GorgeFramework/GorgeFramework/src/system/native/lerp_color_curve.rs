//! `GorgeFramework.LerpColorCurve` —— 补间颜色曲线 native 类。
//!
//! 移植自 C# 参考实现 `System/Native/LerpColorCurve.cs`。
//! 持有 colorPoints（ObjectArray，每个元素为 ColorArgb 对象）
//! 与 progressCurve（FunctionCurve），通过进度曲线映射 x → 进度值，
//! 在 colorPoints 的相邻颜色间线性插值（ColorArgb.Lerp）。

use gorge_core::objective::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 补间颜色曲线
///
/// 字段：
/// - `color_points` (对象 ID)：ObjectArray，内含 ColorArgb 对象
/// - `progress_curve` (对象 ID)：FunctionCurve，将输入 x 映射到 [0, len-1] 进度
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct LerpColorCurve {
    #[gorge_field]
    pub color_points: usize,
    #[gorge_field]
    pub progress_curve: usize,
}

#[gorge_native_impl]
impl LerpColorCurve {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, color_points: usize, progress_curve: usize) {
        ctx.set_object_object_field(this, LerpColorCurve::FIELD_INDEX_color_points, color_points);
        ctx.set_object_object_field(this, LerpColorCurve::FIELD_INDEX_progress_curve, progress_curve);
    }

    /// 在 x 处求值，返回新 ColorArgb 对象 ID
    ///
    /// 对齐 C# `Evaluate(float x)`：
    /// 1. colorPoints 为空 → 返回白色
    /// 2. progressCurve 为空 → 返回第一色
    /// 3. 否则 progress = progressCurve.evaluate(x)，在 point0/point1 之间 Lerp
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> usize {
        let cp = ctx.get_object_object_field(this, LerpColorCurve::FIELD_INDEX_color_points);
        let pc = ctx.get_object_object_field(this, LerpColorCurve::FIELD_INDEX_progress_curve);

        let len = if cp != 0 { ctx.object_array_len(cp) } else { 0 };
        if len == 0 {
            return make_white_color(ctx);
        }

        if pc == 0 {
            let first = ctx.object_array_get(cp, 0);
            return if first != 0 { first } else { make_white_color(ctx) };
        }

        // 进度曲线求值
        let progress = ctx.call_native_method_float_f(pc, 0, x as f64) as f32;
        let point0 = (progress.floor() as i64).clamp(0, len as i64 - 1) as isize;
        let point1 = (progress.ceil() as i64).clamp(0, len as i64 - 1) as isize;

        if point0 == point1 {
            let color = ctx.object_array_get(cp, point0 as usize);
            return if color != 0 { color } else { make_white_color(ctx) };
        }

        let c0_id = ctx.object_array_get(cp, point0 as usize);
        let c1_id = ctx.object_array_get(cp, point1 as usize);
        let t = progress - point0 as f32;

        let c0_id = if c0_id != 0 { c0_id } else { make_white_color(ctx) };
        let c1_id = if c1_id != 0 { c1_id } else { make_white_color(ctx) };

        // 调用 ColorArgb.Lerp 静态方法
        ctx.set_object_param(0, c0_id);
        ctx.set_object_param(1, c1_id);
        ctx.set_float_param(0, t as f64);
        ctx.invoke_native_static_on("GorgeFramework.ColorArgb", 0);
        ctx.get_object_return()
    }
}

/// 创建白色 ColorArgb 对象并返回其 ID
fn make_white_color(ctx: &mut NativeContext) -> usize {
    ctx.set_float_param(0, 1.0);
    ctx.set_float_param(1, 1.0);
    ctx.set_float_param(2, 1.0);
    ctx.set_float_param(3, 1.0);
    let cls = ctx.vm.native_class_table.get("GorgeFramework.ColorArgb").cloned();
    if let Some(cls) = cls {
        cls.do_construct_native(ctx, None, 0)
    } else {
        0
    }
}
