//! `GorgeFramework.AnnulusMeshTransformer` —— 圆环网格变换 native 类。
//!
//! 移植自 C# 参考实现 `System/Native/AnnulusMeshTransformer.cs`。
//! 将顶点按极坐标变换：x 角经 xAngle 曲线映射，y 半径经 yRadius 曲线映射，
//! 极坐标 (radius, angle) → 直角坐标 (radius*cos(angle), radius*sin(angle), z 保持不变)。

use gorge_core::objective::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 圆环网格变换器
///
/// 字段：
/// - `x_angle` (对象 ID)：FunctionCurve，将顶点 x 映射为角度
/// - `y_radius` (对象 ID)：FunctionCurve，将顶点 y 映射为半径
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct AnnulusMeshTransformer {
    #[gorge_field]
    pub x_angle: usize,
    #[gorge_field]
    pub y_radius: usize,
}

#[gorge_native_impl]
impl AnnulusMeshTransformer {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, x_angle: usize, y_radius: usize) {
        ctx.set_object_object_field(this, AnnulusMeshTransformer::FIELD_INDEX_x_angle, x_angle);
        ctx.set_object_object_field(this, AnnulusMeshTransformer::FIELD_INDEX_y_radius, y_radius);
    }

    /// 对顶点应用圆环变换，返回新 Vector3 对象 ID
    ///
    /// 对齐 C# `Transform(Vector3 vertex)`：
    /// angle = xAngle?.Evaluate(vertex.x) ?? vertex.x
    /// radius = yRadius?.Evaluate(vertex.y) ?? vertex.y
    /// → (radius*cos(angle), radius*sin(angle), vertex.z)
    #[gorge_method]
    pub fn transform(ctx: &mut NativeContext, this: usize, vertex: usize) -> usize {
        let xa = ctx.get_object_object_field(this, AnnulusMeshTransformer::FIELD_INDEX_x_angle);
        let yr = ctx.get_object_object_field(this, AnnulusMeshTransformer::FIELD_INDEX_y_radius);

        let vx = ctx.get_object_float_field(vertex, 0) as f32;
        let vy = ctx.get_object_float_field(vertex, 1) as f32;
        let vz = ctx.get_object_float_field(vertex, 2) as f32;

        let angle = if xa != 0 {
            ctx.call_native_method_float_f(xa, 0, vx as f64) as f32
        } else {
            vx
        };
        let radius = if yr != 0 {
            ctx.call_native_method_float_f(yr, 0, vy as f64) as f32
        } else {
            vy
        };

        make_vector3(ctx, radius * angle.cos(), radius * angle.sin(), vz)
    }
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
