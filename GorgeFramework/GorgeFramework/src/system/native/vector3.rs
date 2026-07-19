//! `GorgeFramework.Vector3` —— 三维向量 native 类。
//!
//! 移植自 C# 参考实现 `System/Native/Vector3.cs`。使用 `glam` crate
//! 替代 C# 的 `System.Numerics`，提供四元数旋转、欧拉角转换等功能。

use gorge_core::objective::native::NativeContext;
use gorge_core::objective::object::RuntimeObject;
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

    /// 静态方法 2：从四元数还原欧拉角（角度制）
    ///
    /// 对齐 C# `FromQuaternion(Quaternion)`。
    /// 读取 Quaternion 对象的 x/y/z/w 浮点字段，
    /// 按 Yaw(绕Y)/Pitch(绕X)/Roll(绕Z) 顺序分解。
    #[gorge_static]
    pub fn from_quaternion(ctx: &mut NativeContext, q_id: usize) -> usize {
        let qx = ctx.get_object_float_field(q_id, 0) as f32;
        let qy = ctx.get_object_float_field(q_id, 1) as f32;
        let qz = ctx.get_object_float_field(q_id, 2) as f32;
        let qw = ctx.get_object_float_field(q_id, 3) as f32;
        use std::f32::consts::PI;
        let rad2deg = 180.0 / PI;

        // pitch（绕 X）
        let sinp = 2.0 * (qw * qx + qy * qz);
        let cosp = 1.0 - 2.0 * (qx * qx + qy * qy);
        let x = sinp.atan2(cosp) * rad2deg;

        // yaw（绕 Y）
        let siny = 2.0 * (qw * qy - qz * qx);
        let y = if siny.abs() >= 1.0 {
            siny.signum() * PI / 2.0
        } else {
            siny.asin()
        };
        let y = y * rad2deg;

        // roll（绕 Z）
        let sinr = 2.0 * (qw * qz + qx * qy);
        let cosr = 1.0 - 2.0 * (qy * qy + qz * qz);
        let z = sinr.atan2(cosr) * rad2deg;

        make_vector3(ctx, x, y, z)
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
        &gorge_core::objective::types::TypeCount { float_count: n, ..Default::default() },
    );
    let id = ctx.register_object(obj);
    for (i, &v) in vals.iter().enumerate() {
        ctx.set_object_float_field(id, i, v as f64);
    }
    id
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
            vm.next_object_id = 100;
            Self { vm }
        }

        fn ctx(&mut self) -> NativeContext<'_> { NativeContext::new(&mut self.vm) }

        fn make_quat_obj(&mut self, x: f32, y: f32, z: f32, w: f32) -> usize {
            make_vec(&mut self.ctx(), Quaternion::GORGE_FULL_NAME, 4, &[x, y, z, w])
        }
    }

    #[test]
    fn test_from_quaternion_identity() {
        let v3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        let mut fx = Fixture::new();
        fx.vm.register_native_class(v3.full_name(), std::sync::Arc::new(Vector3 { x: 0.0, y: 0.0, z: 0.0 }));
        // 单位四元数 (0,0,0,1) 应返回 (0,0,0)
        let q = fx.make_quat_obj(0.0, 0.0, 0.0, 1.0);
        fx.vm.param_pool.set_object_param(0, q);
        { let mut ctx = fx.ctx(); v3.invoke_native_static(&mut ctx, 9); }
        let result_id = fx.vm.param_pool.get_object_return();
        assert!(result_id > 0);
        let (rx, ry, rz) = read_xyz(&fx.ctx(), result_id);
        assert!((rx - 0.0).abs() < 0.1, "identity x 应 ≈0，实际 {rx}");
        assert!((ry - 0.0).abs() < 0.1, "identity y 应 ≈0，实际 {ry}");
        assert!((rz - 0.0).abs() < 0.1, "identity z 应 ≈0，实际 {rz}");
    }

    #[test]
    fn test_from_quaternion_90deg_around_y() {
        // 绕 Y 轴旋转 90° 的四元数：(0, sin45, 0, cos45) = (0, 0.707, 0, 0.707)
        let v3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        let mut fx = Fixture::new();
        fx.vm.register_native_class(v3.full_name(), std::sync::Arc::new(Vector3 { x: 0.0, y: 0.0, z: 0.0 }));
        use std::f32::consts::PI;
        let half = (PI / 4.0).sin(); // sin(45°) = 0.707
        let q = fx.make_quat_obj(0.0, half, 0.0, half); // 绕 Y 轴 90°
        fx.vm.param_pool.set_object_param(0, q);
        { let mut ctx = fx.ctx(); v3.invoke_native_static(&mut ctx, 9); }
        let result_id = fx.vm.param_pool.get_object_return();
        assert!(result_id > 0);
        let (rx, ry, _rz) = read_xyz(&fx.ctx(), result_id);
        // 绕 Y 轴 90° → yaw=90°, pitch=0, roll=0
        assert!(rx.abs() < 0.5, "pitch 应 ≈0，实际 {rx}");
        assert!((ry - 90.0).abs() < 1.0, "yaw 应 ≈90，实际 {ry}");
    }
}
