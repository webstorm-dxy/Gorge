//! `GorgeFramework.CubicHermiteSpline` — 加权三次 Hermite 样条曲线。
//!
//! 字段布局对齐谱面存根与 C# 参考（6 个注入器字段）：
//! `startPoint`/`endPoint` 为 Vector2 对象注入器字段，
//! 其余 4 个 float 字段为端点正切与权重。
//!
//! 与 C# 的差异：C# 求值使用加权 AnimationCurveInterpolant（weight 参与迭代求 t），
//! 本轮仅对齐字段布局，求值仍沿用经典 Hermite 基函数（weight 仅存储、暂不参与求值）。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use super::vector2::Vector2;

/// 加权三次 Hermite 样条曲线
///
/// 注入器字段（声明序对齐 C#）：
/// - `start_point`/`end_point`（Vector2 对象 ID）：曲线两端点；
///   宏对对象字段忽略 `default`，未注入（0）时 evaluate 分别回退 (0,0)/(1,1)
/// - `start_tangent`/`end_tangent`：端点正切，默认 0
/// - `start_weight`/`end_weight`：端点权重，默认 0.33333（当前求值未使用）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct CubicHermiteSpline {
    /// 首点（Vector2 对象 ID，0 时回退 (0,0)）
    #[gorge_field]
    #[inject(name = "startPoint")]
    pub start_point: usize,
    /// 首端正切
    #[gorge_field]
    #[inject(name = "startTangent", default = 0.0)]
    pub start_tangent: f32,
    /// 首端权重
    #[gorge_field]
    #[inject(name = "startWeight", default = 0.33333)]
    pub start_weight: f32,
    /// 尾点（Vector2 对象 ID，0 时回退 (1,1)）
    #[gorge_field]
    #[inject(name = "endPoint")]
    pub end_point: usize,
    /// 尾端正切
    #[gorge_field]
    #[inject(name = "endTangent", default = 0.0)]
    pub end_tangent: f32,
    /// 尾端权重
    #[gorge_field]
    #[inject(name = "endWeight", default = 0.33333)]
    pub end_weight: f32,
}

#[gorge_native_impl]
impl CubicHermiteSpline {
    /// 构造方法 0：无参构造（端点/正切/权重全部由注入器字段默认值提供）
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize) { let _ = (ctx, this); }

    /// 在 x 处求值
    ///
    /// 端点取自 `start_point`/`end_point` 指向的 Vector2 对象；
    /// 对象 ID 为 0 时按 C# 默认值回退 (0,0)/(1,1)。
    #[gorge_method]
    pub fn evaluate(ctx: &mut NativeContext, this: usize, x: f32) -> f32 {
        let start_point = ctx.get_object_object_field(this, Self::FIELD_INDEX_start_point);
        let (ts, vs) = if start_point == 0 {
            (0.0, 0.0)
        } else {
            (
                ctx.get_object_float_field(start_point, Vector2::FIELD_INDEX_x) as f32,
                ctx.get_object_float_field(start_point, Vector2::FIELD_INDEX_y) as f32,
            )
        };
        let end_point = ctx.get_object_object_field(this, Self::FIELD_INDEX_end_point);
        let (te, ve) = if end_point == 0 {
            (1.0, 1.0)
        } else {
            (
                ctx.get_object_float_field(end_point, Vector2::FIELD_INDEX_x) as f32,
                ctx.get_object_float_field(end_point, Vector2::FIELD_INDEX_y) as f32,
            )
        };
        let m0 = ctx.get_object_float_field(this, Self::FIELD_INDEX_start_tangent) as f32;
        let m1 = ctx.get_object_float_field(this, Self::FIELD_INDEX_end_tangent) as f32;
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
