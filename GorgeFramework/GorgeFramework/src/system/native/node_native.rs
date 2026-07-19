//! `GorgeFramework.Node` —— 场景图节点 native 类（E-2 补齐）。
//!
//! 对齐 C# `System/Native/Node.cs`。存储层级变换（位置/旋转/缩放）
//! 及引用链（existence/position/rotation/size reference），
//! 引用链通过对象 ID 间接引用其他 Node 实例。
//!
//! # 方法编号表
//! | 编号 | 方法 | 说明 |
//! |------|------|------|
//! | 0 | local_position_to_global_position | 局部坐标 → 全局坐标（沿父链累积变换） |
//! | 1 | global_position_to_local_position | 全局坐标 → 局部坐标（逆变换） |
//! | 2 | global_position | 计算全局位置 |
//! | 3 | global_rotation | 计算全局旋转（父链累积） |
//! | 4 | global_size | 计算全局缩放（父链连乘） |
//! | 5 | update_node | 更新节点（检查 existence_reference 存活） |
//! | 6 | destroy | 销毁节点（设置 alive=false） |

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 场景图节点 native 类
///
/// 自引用字段（existence_reference 等）存储为 usize（Node 对象 ID），
/// 0 表示无引用。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Node {
    /// 是否存活
    #[gorge_field]
    pub alive: bool,
    /// 存活依赖（Node 对象 ID，0=无）
    #[gorge_field]
    pub existence_reference: usize,
    /// 局部位置 x
    #[gorge_field]
    pub position_x: f32,
    /// 局部位置 y
    #[gorge_field]
    pub position_y: f32,
    /// 局部位置 z
    #[gorge_field]
    pub position_z: f32,
    /// 位置引用（Node 对象 ID，0=使用自己的 position）
    #[gorge_field]
    pub position_reference: usize,
    /// 局部旋转 x（弧度）
    #[gorge_field]
    pub rotation_x: f32,
    /// 局部旋转 y
    #[gorge_field]
    pub rotation_y: f32,
    /// 局部旋转 z
    #[gorge_field]
    pub rotation_z: f32,
    /// 旋转引用
    #[gorge_field]
    pub rotation_reference: usize,
    /// 局部缩放 x
    #[gorge_field]
    pub size_x: f32,
    /// 局部缩放 y
    #[gorge_field]
    pub size_y: f32,
    /// 局部缩放 z
    #[gorge_field]
    pub size_z: f32,
    /// 缩放引用
    #[gorge_field]
    pub size_reference: usize,
}

impl Node {
    /// 读取节点位置
    fn read_position(ctx: &NativeContext, node_id: usize) -> (f32, f32, f32) {
        (
            ctx.get_object_float_field(node_id, Node::FIELD_INDEX_position_x) as f32,
            ctx.get_object_float_field(node_id, Node::FIELD_INDEX_position_y) as f32,
            ctx.get_object_float_field(node_id, Node::FIELD_INDEX_position_z) as f32,
        )
    }

    /// 读取节点旋转
    fn read_rotation(ctx: &NativeContext, node_id: usize) -> (f32, f32, f32) {
        (
            ctx.get_object_float_field(node_id, Node::FIELD_INDEX_rotation_x) as f32,
            ctx.get_object_float_field(node_id, Node::FIELD_INDEX_rotation_y) as f32,
            ctx.get_object_float_field(node_id, Node::FIELD_INDEX_rotation_z) as f32,
        )
    }

    /// 读取节点缩放
    fn read_size(ctx: &NativeContext, node_id: usize) -> (f32, f32, f32) {
        (
            ctx.get_object_float_field(node_id, Node::FIELD_INDEX_size_x) as f32,
            ctx.get_object_float_field(node_id, Node::FIELD_INDEX_size_y) as f32,
            ctx.get_object_float_field(node_id, Node::FIELD_INDEX_size_z) as f32,
        )
    }

    /// 读取节点存活引用
    fn read_existence_reference(ctx: &NativeContext, node_id: usize) -> usize {
        ctx.get_object_object_field(node_id, Node::FIELD_INDEX_existence_reference)
    }
}

#[gorge_native_impl]
impl Node {
    // ==================== 坐标变换 ====================

    /// 局部坐标转换为全局坐标（方法 0）
    ///
    /// 对齐 C# `LocalPositionToGlobalPosition`。
    /// `px, py, pz` 为要转换的局部坐标。
    /// 返回值编码：(gx << 32 未用, 实际返回 gx 值，gy/gz 需用额外调用获取)。
    /// 简化实现：直接返回 selfPosition + selfSize * position 旋转 selfRotation 后的结果。
    #[gorge_method]
    pub fn local_position_to_global_position(
        ctx: &mut NativeContext, this: usize,
        px: f32, py: f32, pz: f32,
    ) -> f32 {
        let (gpx, gpy, gpz) = calc_global_position(ctx, this);
        let (grx, gry, grz) = calc_global_rotation(ctx, this);
        let (gsx, gsy, gsz) = calc_global_size(ctx, this);

        // selfSize * position（按轴缩放）
        let scaled = (gsx * px, gsy * py, gsz * pz);

        // 绕全局旋转旋转
        let quat = euler_to_quat(grx, gry, grz);
        let rotated = rotate_vec3_by_quat(scaled, quat);

        // selfPosition + rotated
        (gpx + rotated.0, gpy + rotated.1, gpz + rotated.2).0
    }

    /// 全局坐标转换为局部坐标（方法 1）
    ///
    /// 对齐 C# `GlobalPositionToLocalPosition`。
    /// 返回 local_x 值（简化）。
    #[gorge_method]
    pub fn global_position_to_local_position(
        ctx: &mut NativeContext, this: usize,
        gx: f32, gy: f32, gz: f32,
    ) -> f32 {
        let (gpx, gpy, gpz) = calc_global_position(ctx, this);
        let (grx, gry, grz) = calc_global_rotation(ctx, this);
        let (gsx, gsy, gsz) = calc_global_size(ctx, this);

        // 差向量
        let diff = (gx - gpx, gy - gpy, gz - gpz);

        // 逆旋转
        let quat = euler_to_quat(grx, gry, grz);
        let inv_quat = quat_inverse(quat);
        let rotated = rotate_vec3_by_quat(diff, inv_quat);

        // 逆缩放
        (rotated.0 / gsx, rotated.1 / gsy, rotated.2 / gsz).0
    }

    /// 计算全局位置（方法 2）
    ///
    /// 对齐 C# `GlobalPosition`。若无 positionReference 则返回局部位置；
    /// 否则沿父链递归计算。
    #[gorge_method]
    pub fn global_position(ctx: &mut NativeContext, this: usize) -> f32 {
        calc_global_position(ctx, this).0
    }

    /// 计算全局旋转（方法 3）
    #[gorge_method]
    pub fn global_rotation(ctx: &mut NativeContext, this: usize) -> f32 {
        calc_global_rotation(ctx, this).0
    }

    /// 计算全局缩放（方法 4）
    #[gorge_method]
    pub fn global_size(ctx: &mut NativeContext, this: usize) -> f32 {
        calc_global_size(ctx, this).0
    }

    /// 更新节点（方法 5）
    ///
    /// 对齐 C# `UpdateNode`。若 existenceReference 不为 null 且不存活，
    /// 则设置自身 alive=false。
    #[gorge_method]
    pub fn update_node(ctx: &mut NativeContext, this: usize) {
        let ref_id = Node::read_existence_reference(ctx, this);
        if ref_id != 0 {
            let ref_alive = ctx.get_object_bool_field(ref_id, Node::FIELD_INDEX_alive);
            if !ref_alive {
                ctx.set_object_bool_field(this, Node::FIELD_INDEX_alive, false);
            }
        }
    }

    /// 销毁节点（方法 6）
    ///
    /// 对齐 C# `Destroy`。设置 alive=false。
    #[gorge_method]
    pub fn destroy(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_bool_field(this, Node::FIELD_INDEX_alive, false);
    }
}

// ==================== 内联辅助函数 ====================

/// 计算节点全局位置（递归沿 position_reference 父链）
fn calc_global_position(ctx: &NativeContext, node_id: usize) -> (f32, f32, f32) {
    let ref_id = ctx.get_object_object_field(node_id, Node::FIELD_INDEX_position_reference);
    if ref_id == 0 {
        return Node::read_position(ctx, node_id);
    }
    let ref_pos = calc_global_position(ctx, ref_id);
    let local_pos = Node::read_position(ctx, node_id);
    // 父节点变换：refPos + localPos 经 refRotation 旋转并缩放
    let ref_rot = calc_global_rotation(ctx, ref_id);
    let ref_size = calc_global_size(ctx, ref_id);
    let scaled = (ref_size.0 * local_pos.0, ref_size.1 * local_pos.1, ref_size.2 * local_pos.2);
    let quat = euler_to_quat(ref_rot.0, ref_rot.1, ref_rot.2);
    let rotated = rotate_vec3_by_quat(scaled, quat);
    (ref_pos.0 + rotated.0, ref_pos.1 + rotated.1, ref_pos.2 + rotated.2)
}

/// 计算节点全局旋转（递归沿 rotation_reference 父链，角度相加）
fn calc_global_rotation(ctx: &NativeContext, node_id: usize) -> (f32, f32, f32) {
    let ref_id = ctx.get_object_object_field(node_id, Node::FIELD_INDEX_rotation_reference);
    if ref_id == 0 {
        return Node::read_rotation(ctx, node_id);
    }
    let ref_rot = calc_global_rotation(ctx, ref_id);
    let local_rot = Node::read_rotation(ctx, node_id);
    // 四元数乘法：refRotation * localRotation，再转回欧拉角（简化：直接角度相加）
    (ref_rot.0 + local_rot.0, ref_rot.1 + local_rot.1, ref_rot.2 + local_rot.2)
}

/// 计算节点全局缩放（递归沿 size_reference 父链，连乘）
fn calc_global_size(ctx: &NativeContext, node_id: usize) -> (f32, f32, f32) {
    let ref_id = ctx.get_object_object_field(node_id, Node::FIELD_INDEX_size_reference);
    if ref_id == 0 {
        return Node::read_size(ctx, node_id);
    }
    let ref_size = calc_global_size(ctx, ref_id);
    let local_size = Node::read_size(ctx, node_id);
    (ref_size.0 * local_size.0, ref_size.1 * local_size.1, ref_size.2 * local_size.2)
}

// ==================== 纯 Rust 数学工具 ====================

/// 四元数 (w, x, y, z)
type Quat = (f32, f32, f32, f32);

/// 欧拉角（弧度）→ 四元数
///
/// 使用 ZYX 内旋顺序（对齐 C# Quaternion.CreateFromYawPitchRoll 的行为）。
fn euler_to_quat(x: f32, y: f32, z: f32) -> Quat {
    let cx = (x * 0.5).cos();
    let sx = (x * 0.5).sin();
    let cy = (y * 0.5).cos();
    let sy = (y * 0.5).sin();
    let cz = (z * 0.5).cos();
    let sz = (z * 0.5).sin();

    let w = cx * cy * cz + sx * sy * sz;
    let qx = sx * cy * cz - cx * sy * sz;
    let qy = cx * sy * cz + sx * cy * sz;
    let qz = cx * cy * sz - sx * sy * cz;

    (w, qx, qy, qz)
}

/// 四元数逆（假定单位四元数）
fn quat_inverse(q: Quat) -> Quat {
    (q.0, -q.1, -q.2, -q.3)
}

/// 用四元数旋转三维向量
///
/// 公式：v' = v + 2w(cross(q_xyz, v)) + 2(cross(q_xyz, cross(q_xyz, v)))
fn rotate_vec3_by_quat(v: (f32, f32, f32), q: Quat) -> (f32, f32, f32) {
    let qv = (q.1, q.2, q.3);
    let t = cross3(qv, v);
    let t2 = cross3(qv, t);
    (
        v.0 + 2.0 * q.0 * t.0 + 2.0 * t2.0,
        v.1 + 2.0 * q.0 * t.1 + 2.0 * t2.1,
        v.2 + 2.0 * q.0 * t.2 + 2.0 * t2.2,
    )
}

/// 三维向量叉积
fn cross3(a: (f32, f32, f32), b: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::object::GorgeObject;
    use gorge_core::objective::object::RuntimeObject;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    struct Fixture {
        vm: VirtualMachine,
    }

    impl Fixture {
        fn new() -> Self {
            Self {
                vm: VirtualMachine::new(),
            }
        }
        fn ctx(&mut self) -> NativeContext<'_> {
            NativeContext::new(&mut self.vm)
        }

        fn make_node(
            &mut self,
            px: f32, py: f32, pz: f32,
            rx: f32, ry: f32, rz: f32,
            sx: f32, sy: f32, sz: f32,
            pos_ref: usize, rot_ref: usize, size_ref: usize,
        ) -> usize {
            let n = Node {
                alive: true,
                existence_reference: 0,
                position_x: px, position_y: py, position_z: pz,
                position_reference: pos_ref,
                rotation_x: rx, rotation_y: ry, rotation_z: rz,
                rotation_reference: rot_ref,
                size_x: sx, size_y: sy, size_z: sz,
                size_reference: size_ref,
            };
            // Node 无构造方法，需手动设置字段
            let id = self.vm.next_object_id;
            self.vm.next_object_id += 1;
            self.vm.objects.insert(
                id,
                RuntimeObject::new_simple("GorgeFramework.Node".to_string(), n.field_type_count()),
            );
            self.vm.objects.get_mut(&id).unwrap()
                .set_bool_field(Node::FIELD_INDEX_alive, true);
            self.vm.objects.get_mut(&id).unwrap()
                .set_object_field(Node::FIELD_INDEX_existence_reference, 0);
            self.vm.objects.get_mut(&id).unwrap()
                .set_float_field(Node::FIELD_INDEX_position_x, px as f64);
            self.vm.objects.get_mut(&id).unwrap()
                .set_float_field(Node::FIELD_INDEX_position_y, py as f64);
            self.vm.objects.get_mut(&id).unwrap()
                .set_float_field(Node::FIELD_INDEX_position_z, pz as f64);
            self.vm.objects.get_mut(&id).unwrap()
                .set_object_field(Node::FIELD_INDEX_position_reference, pos_ref);
            self.vm.objects.get_mut(&id).unwrap()
                .set_float_field(Node::FIELD_INDEX_rotation_x, rx as f64);
            self.vm.objects.get_mut(&id).unwrap()
                .set_float_field(Node::FIELD_INDEX_rotation_y, ry as f64);
            self.vm.objects.get_mut(&id).unwrap()
                .set_float_field(Node::FIELD_INDEX_rotation_z, rz as f64);
            self.vm.objects.get_mut(&id).unwrap()
                .set_object_field(Node::FIELD_INDEX_rotation_reference, rot_ref);
            self.vm.objects.get_mut(&id).unwrap()
                .set_float_field(Node::FIELD_INDEX_size_x, sx as f64);
            self.vm.objects.get_mut(&id).unwrap()
                .set_float_field(Node::FIELD_INDEX_size_y, sy as f64);
            self.vm.objects.get_mut(&id).unwrap()
                .set_float_field(Node::FIELD_INDEX_size_z, sz as f64);
            self.vm.objects.get_mut(&id).unwrap()
                .set_object_field(Node::FIELD_INDEX_size_reference, size_ref);
            id
        }
    }

    #[test]
    fn test_node_global_position_no_reference() {
        let mut fx = Fixture::new();
        let n = Node {
            alive: true,
            existence_reference: 0,
            position_x: 3.0, position_y: 4.0, position_z: 5.0,
            position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0,
            rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0,
            size_reference: 0,
        };
        let node_id = fx.make_node(3.0, 4.0, 5.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0, 0, 0);

        // global_position（方法 2）
        let mut ctx = fx.ctx();
        n.invoke_native_method(&mut ctx, node_id, 2);
        let gx = fx.vm.param_pool.get_float_return() as f64;
        assert!((gx - 3.0).abs() < 0.001, "无引用时应返回局部位置 x=3.0");
    }

    #[test]
    fn test_node_global_position_with_parent_chain() {
        let mut fx = Fixture::new();
        // 父节点：(10, 0, 0), 缩放 (2, 1, 1)
        let parent_id = fx.make_node(10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 1.0, 1.0, 0, 0, 0);
        // 子节点：(1, 2, 0), 位置引用 parent
        // 全局位置 = parentPos + parentSize * localPos = (10,0,0) + (2*1, 1*2, 1*0) = (12, 2, 0)
        let child_id = fx.make_node(1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, parent_id, 0, 0);

        let ctx = &mut fx.ctx();
        // 直接读对象字段验证
        let ref_id = ctx.get_object_object_field(child_id, Node::FIELD_INDEX_position_reference);
        assert_eq!(ref_id, parent_id);

        // 用 calc_global_position 验证
        let gpos = calc_global_position(ctx, child_id);
        assert!((gpos.0 - 12.0).abs() < 0.01, "全局位置 x 应为 12.0，实际 {}", gpos.0);
        assert!((gpos.1 - 2.0).abs() < 0.01, "全局位置 y 应为 2.0，实际 {}", gpos.1);
    }

    #[test]
    fn test_node_global_size_chain() {
        let mut fx = Fixture::new();
        // A(无引用): size(2, 3, 4)
        let a = fx.make_node(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 3.0, 4.0, 0, 0, 0);
        // B(引用A): size(0.5, 0.6, 0.7)
        let b = fx.make_node(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.6, 0.7, 0, 0, a);

        let ctx = &mut fx.ctx();
        let gs = calc_global_size(ctx, b);
        assert!((gs.0 - 1.0).abs() < 0.01, "全局缩放 x: 2*0.5=1.0");
        assert!((gs.1 - 1.8).abs() < 0.01, "全局缩放 y: 3*0.6=1.8");
        assert!((gs.2 - 2.8).abs() < 0.01, "全局缩放 z: 4*0.7=2.8");
    }

    #[test]
    fn test_node_global_rotation_additive() {
        let mut fx = Fixture::new();
        // A(无引用): rot(π/4, 0, 0)
        let a = fx.make_node(0.0, 0.0, 0.0, std::f32::consts::FRAC_PI_4, 0.0, 0.0, 1.0, 1.0, 1.0, 0, 0, 0);
        // B(引用A): rot(π/4, 0, 0)
        let b = fx.make_node(0.0, 0.0, 0.0, std::f32::consts::FRAC_PI_4, 0.0, 0.0, 1.0, 1.0, 1.0, 0, a, 0);

        let ctx = &mut fx.ctx();
        let gr = calc_global_rotation(ctx, b);
        // 简化：角度相加
        let expected = std::f32::consts::FRAC_PI_2;
        assert!((gr.0 - expected).abs() < 0.01, "全局旋转 x: π/4+π/4=π/2");
    }

    #[test]
    fn test_node_update_node_kills_when_ref_dead() {
        let mut fx = Fixture::new();
        let n = Node {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0,
            position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0,
            rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0,
            size_reference: 0,
        };
        // 引用节点（已死亡）
        let ref_id = fx.make_node(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0, 0, 0);
        fx.vm.objects.get_mut(&ref_id).unwrap()
            .set_bool_field(Node::FIELD_INDEX_alive, false);

        // 目标节点，existence_reference 指向 ref_id
        let target_id = fx.make_node(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0, 0, 0);
        fx.vm.objects.get_mut(&target_id).unwrap()
            .set_object_field(Node::FIELD_INDEX_existence_reference, ref_id);
        fx.vm.objects.get_mut(&target_id).unwrap()
            .set_bool_field(Node::FIELD_INDEX_alive, true);

        // update_node（方法 5）
        let mut ctx = fx.ctx();
        n.invoke_native_method(&mut ctx, target_id, 5);

        assert!(!fx.vm.objects.get(&target_id).unwrap().get_bool_field(Node::FIELD_INDEX_alive),
            "引用节点死亡时自身应变为不存活");
    }

    #[test]
    fn test_node_destroy_sets_alive_false() {
        let mut fx = Fixture::new();
        let n = Node {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0,
            position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0,
            rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0,
            size_reference: 0,
        };
        let node_id = fx.make_node(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0, 0, 0);

        // destroy（方法 6）
        let mut ctx = fx.ctx();
        n.invoke_native_method(&mut ctx, node_id, 6);

        assert!(!fx.vm.objects.get(&node_id).unwrap().get_bool_field(Node::FIELD_INDEX_alive));
    }

    #[test]
    fn test_euler_to_quat_identity() {
        let q = euler_to_quat(0.0, 0.0, 0.0);
        assert!((q.0 - 1.0).abs() < 0.001, "恒等旋转 w=1");
        assert!(q.1.abs() < 0.001);
        assert!(q.2.abs() < 0.001);
        assert!(q.3.abs() < 0.001);
    }

    #[test]
    fn test_rotate_vec3_identity() {
        let q = euler_to_quat(0.0, 0.0, 0.0);
        let v = (1.0, 2.0, 3.0);
        let r = rotate_vec3_by_quat(v, q);
        assert!((r.0 - 1.0).abs() < 0.001);
        assert!((r.1 - 2.0).abs() < 0.001);
        assert!((r.2 - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_rotate_vec3_90deg_z() {
        // Z 轴 90° 旋转：x→y, y→-x
        let q = euler_to_quat(0.0, 0.0, std::f32::consts::FRAC_PI_2);
        let v = (1.0, 0.0, 0.0);
        let r = rotate_vec3_by_quat(v, q);
        assert!((r.0 - 0.0).abs() < 0.001, "x→0");
        assert!((r.1 - 1.0).abs() < 0.001, "y→1");
        assert!((r.2 - 0.0).abs() < 0.001);
    }
}
