//! `GorgeFramework.CurveWarpTransformer` —— 曲线弯曲变换 native 类。
//!
//! 移植自 C# 参考实现 `System/Native/CurveWarpTransformer.cs`。
//! 沿法线方向按曲线曲率偏移顶点，实现轨道弯曲效果。

use gorge_core::objective::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 曲线弯曲变换器
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct CurveWarpTransformer {
    /// 变换曲线（FunctionCurve 对象 ID）
    #[gorge_field]
    pub curve: usize,
    /// 是否保持比例
    #[gorge_field]
    pub preserve_proportions: bool,
    /// 曲率影响系数
    #[gorge_field]
    pub curvature_influence: f32,
    /// 变换轴：0=X, 1=Y, 2=Z
    #[gorge_field]
    pub transformed_axis: i32,
    /// 曲线值轴：0=X, 1=Y, 2=Z
    #[gorge_field]
    pub curve_value_axis: i32,
}

#[gorge_native_impl]
impl CurveWarpTransformer {
    #[gorge_ctor]
    pub fn new_ctor(
        ctx: &mut NativeContext,
        this: usize,
        curve: usize,
        preserve_proportions: bool,
        curvature_influence: f32,
        transformed_axis: i32,
        curve_value_axis: i32,
    ) {
        ctx.set_object_object_field(this, CurveWarpTransformer::FIELD_INDEX_curve, curve);
        ctx.set_object_bool_field(this, CurveWarpTransformer::FIELD_INDEX_preserve_proportions, preserve_proportions);
        ctx.set_object_float_field(this, CurveWarpTransformer::FIELD_INDEX_curvature_influence, curvature_influence as f64);
        ctx.set_object_int_field(this, CurveWarpTransformer::FIELD_INDEX_transformed_axis, transformed_axis as i64);
        ctx.set_object_int_field(this, CurveWarpTransformer::FIELD_INDEX_curve_value_axis, curve_value_axis as i64);
    }

    /// 对顶点应用曲线弯曲变换，返回新 Vector3 对象 ID
    ///
    /// 对齐 C# `Transform(Vector3 vertex)`：
    /// 1. 按 transformedAxis 取 curveX
    /// 2. 计算切线与法线（数值微分 + 归一化）
    /// 3. 按曲率计算畸变因子，沿法线偏移
    /// 4. 组装结果 Vector3
    #[gorge_method]
    pub fn transform(ctx: &mut NativeContext, this: usize, vertex: usize) -> usize {
        let curve = ctx.get_object_object_field(this, CurveWarpTransformer::FIELD_INDEX_curve);
        let preserve = ctx.get_object_bool_field(this, CurveWarpTransformer::FIELD_INDEX_preserve_proportions);
        let influence = ctx.get_object_float_field(this, CurveWarpTransformer::FIELD_INDEX_curvature_influence) as f32;
        let trans_axis = ctx.get_object_int_field(this, CurveWarpTransformer::FIELD_INDEX_transformed_axis) as i32;
        let cv_axis = ctx.get_object_int_field(this, CurveWarpTransformer::FIELD_INDEX_curve_value_axis) as i32;

        if curve == 0 {
            // 无曲线时原样返回顶点
            let vx = ctx.get_object_float_field(vertex, 0) as f32;
            let vy = ctx.get_object_float_field(vertex, 1) as f32;
            let vz = ctx.get_object_float_field(vertex, 2) as f32;
            return make_vector3(ctx, vx, vy, vz);
        }

        let vx = ctx.get_object_float_field(vertex, 0) as f32;
        let vy = ctx.get_object_float_field(vertex, 1) as f32;
        let vz = ctx.get_object_float_field(vertex, 2) as f32;

        // 按 transformedAxis 取 curveX
        let curve_x = match trans_axis {
            0 => vx,
            1 => vy,
            2 => vz,
            _ => vx,
        };

        let curve_y = ctx.call_native_method_float_f(curve, 0, curve_x as f64) as f32;

        // 计算切线（数值微分 + 归一化）
        let tangent = calculate_tangent(ctx, curve, curve_x);
        // 法线 = 切线逆时针旋转 90°
        let normal = (-tangent.1, tangent.0);

        // 计算曲率
        let curvature = calculate_curvature(ctx, curve, curve_x);
        let distortion = if preserve {
            1.0
        } else {
            1.0 + curvature * influence
        };

        // 按 curveValueAxis 取曲线值
        let curve_value = match cv_axis {
            0 => vx,
            1 => vy,
            2 => vz,
            _ => vy,
        };

        let adjusted_y = curve_value * distortion;
        let warped_x = curve_x + normal.0 * adjusted_y;
        let warped_y = curve_y + normal.1 * adjusted_y;

        // 组装结果
        let mut rx = vx;
        let mut ry = vy;
        let mut rz = vz;

        match trans_axis {
            0 => rx = warped_x,
            1 => ry = warped_x,
            2 => rz = warped_x,
            _ => {}
        }
        match cv_axis {
            0 => rx = warped_y,
            1 => ry = warped_y,
            2 => rz = warped_y,
            _ => {}
        }

        make_vector3(ctx, rx, ry, rz)
    }
}

// ==================== 内部辅助函数（纯 Rust，非 native 方法） ====================

/// 计算曲线在 curveX 处的归一化切线
fn calculate_tangent(ctx: &mut NativeContext, curve_id: usize, curve_x: f32) -> (f32, f32) {
    let epsilon = 0.001f32;
    let x1 = curve_x - epsilon;
    let x2 = curve_x + epsilon;
    let y1 = ctx.call_native_method_float_f(curve_id, 0, x1 as f64) as f32;
    let y2 = ctx.call_native_method_float_f(curve_id, 0, x2 as f64) as f32;
    vec2_normalize(x2 - x1, y2 - y1)
}

/// 计算曲线在 curveX 处的近似曲率（SignedAngle 差分 / 2ε）
fn calculate_curvature(ctx: &mut NativeContext, curve_id: usize, curve_x: f32) -> f32 {
    let epsilon = 0.01f32;
    let t1 = calculate_tangent(ctx, curve_id, curve_x - epsilon);
    let t2 = calculate_tangent(ctx, curve_id, curve_x + epsilon);
    let angle = vec2_signed_angle(t1.0, t1.1, t2.0, t2.1);
    angle / (2.0 * epsilon)
}

/// 二维向量归一化
fn vec2_normalize(x: f32, y: f32) -> (f32, f32) {
    let len = (x * x + y * y).sqrt();
    if len < 1e-10 {
        (0.0, 0.0)
    } else {
        (x / len, y / len)
    }
}

/// 二维向量有符号夹角（弧度，[-π, π)）
fn vec2_signed_angle(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dot = x1 * x2 + y1 * y2;
    let det = x1 * y2 - y1 * x2;
    det.atan2(dot)
}

/// 创建新 Vector3 对象
fn make_vector3(ctx: &mut NativeContext, x: f32, y: f32, z: f32) -> usize {
    use gorge_core::objective::object::RuntimeObject;
    use gorge_core::objective::types::TypeCount;
    let obj = RuntimeObject::new_simple(
        "GorgeFramework.Vector3".to_string(),
        &TypeCount { float_count: 3, ..Default::default() },
    );
    let id = ctx.register_object(obj);
    ctx.set_object_float_field(id, 0, x as f64);
    ctx.set_object_float_field(id, 1, y as f64);
    ctx.set_object_float_field(id, 2, z as f64);
    id
}
