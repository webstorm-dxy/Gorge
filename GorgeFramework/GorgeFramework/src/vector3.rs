//! `GorgeFramework.Vector3` —— 三维向量 native 类。
//!
//! 移植自 C# 参考实现 `System/Native/Vector3.cs`。使用 `glam` crate
//! 替代 C# 的 `System.Numerics`，提供四元数旋转、欧拉角转换等功能。

use gorge_core::native::NativeContext;
use gorge_core::object::RuntimeObject;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 三维向量
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Vector3 {
    #[gorge_field]
    #[inject(default = 0.0)]
    pub x: f32,
    #[gorge_field]
    #[inject(default = 0.0)]
    pub y: f32,
    #[gorge_field]
    #[inject(default = 0.0)]
    pub z: f32,
}

#[gorge_native_impl]
impl Vector3 {
    #[gorge_ctor]
    pub fn new_empty(ctx: &mut NativeContext, this: usize) { let _ = (ctx, this); }

    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, x: f32, y: f32, z: f32) {
        ctx.set_object_float_field(this, Self::FIELD_INDEX_x, x as f64);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_y, y as f64);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_z, z as f64);
    }

    #[gorge_method]
    pub fn to_vector2(ctx: &mut NativeContext, this: usize) -> usize {
        let (x, y, _) = read_xyz(ctx, this);
        make_vec(ctx, "GorgeFramework.Vector2", 2, &[x, y])
    }

    #[gorge_method]
    pub fn magnitude(ctx: &mut NativeContext, this: usize) -> f32 {
        let (x, y, z) = read_xyz(ctx, this);
        (x * x + y * y + z * z).sqrt()
    }

    #[gorge_method]
    pub fn get_x(ctx: &mut NativeContext, this: usize) -> f32 { ctx.get_object_float_field(this, Self::FIELD_INDEX_x) as f32 }
    #[gorge_method]
    pub fn get_y(ctx: &mut NativeContext, this: usize) -> f32 { ctx.get_object_float_field(this, Self::FIELD_INDEX_y) as f32 }
    #[gorge_method]
    pub fn get_z(ctx: &mut NativeContext, this: usize) -> f32 { ctx.get_object_float_field(this, Self::FIELD_INDEX_z) as f32 }

    #[gorge_static]
    pub fn distance(ctx: &mut NativeContext, v1: usize, v2: usize) -> f32 {
        let (x1, y1, z1) = read_xyz(ctx, v1);
        let (x2, y2, z2) = read_xyz(ctx, v2);
        ((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt()
    }

    #[gorge_static]
    pub fn lerp(ctx: &mut NativeContext, a: usize, b: usize, t: f32) -> usize {
        let (ax, ay, az) = read_xyz(ctx, a);
        let (bx, by, bz) = read_xyz(ctx, b);
        let t = t.clamp(0.0, 1.0);
        make_vector3(ctx, ax + (bx - ax) * t, ay + (by - ay) * t, az + (bz - az) * t)
    }

    /// 实例方法：转为四元数（欧拉角表示，rad）
    #[gorge_method]
    pub fn to_quaternion(ctx: &mut NativeContext, this: usize) -> usize {
        let (x, y, z) = read_xyz(ctx, this);
        let q = glam::Quat::from_euler(glam::EulerRot::YXZ, y, x, z);
        make_quat(ctx, q)
    }

    /// 使用四元数旋转此向量，返回新 Vector3
    #[gorge_method]
    pub fn transform(ctx: &mut NativeContext, this: usize, q_id: usize) -> usize {
        let (x, y, z) = read_xyz(ctx, this);
        // 从对象读取四元数：4 个 float 字段 x, y, z, w
        let qx = ctx.get_object_float_field(q_id, 0) as f32;
        let qy = ctx.get_object_float_field(q_id, 1) as f32;
        let qz = ctx.get_object_float_field(q_id, 2) as f32;
        let qw = ctx.get_object_float_field(q_id, 3) as f32;
        let q = glam::Quat::from_xyzw(qx, qy, qz, qw);
        let v = glam::Vec3::new(x, y, z);
        let r = q * v;
        make_vector3(ctx, r.x, r.y, r.z)
    }
}

// ==================== Quaternion 工具 ====================

/// 四元数 native 类（glam::Quat 的 Gorge 封装）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Quaternion {
    #[gorge_field]
    pub x: f32,
    #[gorge_field]
    pub y: f32,
    #[gorge_field]
    pub z: f32,
    #[gorge_field]
    pub w: f32,
}

#[gorge_native_impl]
impl Quaternion {
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, x: f32, y: f32, z: f32, w: f32) {
        ctx.set_object_float_field(this, Self::FIELD_INDEX_x, x as f64);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_y, y as f64);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_z, z as f64);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_w, w as f64);
    }
}

fn read_xyz(ctx: &NativeContext, obj_id: usize) -> (f32, f32, f32) {
    let x = ctx.get_object_float_field(obj_id, Vector3::FIELD_INDEX_x) as f32;
    let y = ctx.get_object_float_field(obj_id, Vector3::FIELD_INDEX_y) as f32;
    let z = ctx.get_object_float_field(obj_id, Vector3::FIELD_INDEX_z) as f32;
    (x, y, z)
}

fn make_vector3(ctx: &mut NativeContext, x: f32, y: f32, z: f32) -> usize {
    let obj = RuntimeObject::new_simple(
        Vector3::GORGE_FULL_NAME.to_string(),
        &Vector3::gorge_field_type_count(),
    );
    let id = ctx.register_object(obj);
    ctx.set_object_float_field(id, Vector3::FIELD_INDEX_x, x as f64);
    ctx.set_object_float_field(id, Vector3::FIELD_INDEX_y, y as f64);
    ctx.set_object_float_field(id, Vector3::FIELD_INDEX_z, z as f64);
    id
}

fn make_quat(ctx: &mut NativeContext, q: glam::Quat) -> usize {
    make_vec(ctx, Quaternion::GORGE_FULL_NAME, 4, &[q.x, q.y, q.z, q.w])
}

fn make_vec(ctx: &mut NativeContext, name: &str, n: usize, vals: &[f32]) -> usize {
    let obj = RuntimeObject::new_simple(
        name.to_string(),
        &gorge_core::types::TypeCount { float_count: n, ..Default::default() },
    );
    let id = ctx.register_object(obj);
    for (i, &v) in vals.iter().enumerate() {
        ctx.set_object_float_field(id, i, v as f64);
    }
    id
}
