//! `GorgeFramework.CurveSprite` —— 曲线精灵渲染节点 native 类。
//!
//! 移植自 C# 参考实现 `CurveSprite.cs`。
//! 继承自 Node，额外包含 points/color/width 字段。
//! 构造时通过 PlatformBase 创建 ICurveSprite 对象。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use crate::system::native::color_argb::read_color_channels;

/// 从 CurveSprite.points 的 ObjectArray 读取全部曲线点坐标。
///
/// 数组元素为 Vector2/Vector3 对象（float 字段 0/1 为 x/y），
/// 与 C# `SetLine(ObjectArray)` 的上传语义对齐：平台需要完整坐标，
/// 仅传点数会导致渲染侧全部落在 (0,0) 而画不出判定线。
fn read_points(ctx: &NativeContext, points_id: usize) -> Vec<(f32, f32)> {
    if points_id == 0 {
        return Vec::new();
    }
    ctx.object_array_items(points_id)
        .iter()
        .map(|point_id| {
            if *point_id == 0 {
                return (0.0, 0.0);
            }
            let x = ctx.get_object_float_field(*point_id, 0) as f32;
            let y = ctx.get_object_float_field(*point_id, 1) as f32;
            (x, y)
        })
        .collect()
}

/// 曲线精灵渲染节点（C# `CurveSprite`，继承自 `Node`）
///
/// fields: Node 字段 + points(ObjectArray ID) + color(ColorArgb ID) + width(f32)
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct CurveSprite {
    // ---- Node 继承字段 ----
    #[gorge_field]
    pub alive: bool,
    #[gorge_field]
    pub existence_reference: usize,
    /// 局部位置（Vector3 对象 ID）
    #[gorge_field]
    pub position: usize,
    #[gorge_field]
    pub position_reference: usize,
    /// 局部旋转（Vector3 对象 ID）
    #[gorge_field]
    pub rotation: usize,
    #[gorge_field]
    pub rotation_reference: usize,
    /// 局部缩放（Vector3 对象 ID）
    #[gorge_field]
    pub size: usize,
    #[gorge_field]
    pub size_reference: usize,
    // ---- CurveSprite 专属字段 ----
    /// 曲线点 ObjectArray 对象 ID
    #[gorge_field]
    pub points: usize,
    /// 颜色 ColorArgb 对象 ID
    #[gorge_field]
    pub color: usize,
    /// 线宽
    #[gorge_field]
    pub width: f32,
}

#[gorge_native_impl]
impl CurveSprite {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, points: usize) {
        // Node 字段初始化
        ctx.set_object_bool_field(this, CurveSprite::FIELD_INDEX_alive, true);
        ctx.set_object_object_field(this, CurveSprite::FIELD_INDEX_existence_reference, 0);
        ctx.set_object_object_field(this, CurveSprite::FIELD_INDEX_position, 0);
        ctx.set_object_object_field(this, CurveSprite::FIELD_INDEX_position_reference, 0);
        ctx.set_object_object_field(this, CurveSprite::FIELD_INDEX_rotation, 0);
        ctx.set_object_object_field(this, CurveSprite::FIELD_INDEX_rotation_reference, 0);
        ctx.set_object_object_field(this, CurveSprite::FIELD_INDEX_size, 0);
        ctx.set_object_object_field(this, CurveSprite::FIELD_INDEX_size_reference, 0);
        // 专属字段
        ctx.set_object_object_field(this, CurveSprite::FIELD_INDEX_points, points);
        ctx.set_object_object_field(this, CurveSprite::FIELD_INDEX_color, 0);
        ctx.set_object_float_field(this, CurveSprite::FIELD_INDEX_width, 0.1);

        // 创建平台 ICurveSprite 对象
        use crate::adaptor::{platform, platform_installed};
        if platform_installed() {
            let sprite_obj = platform().create_curve_sprite();
            if points != 0 {
                sprite_obj.set_points(&read_points(ctx, points));
            }
            ctx.insert_payload(this, Box::new(sprite_obj));
        }
    }

    /// 0 号方法：更新节点
    #[gorge_method]
    pub fn update_node(ctx: &mut NativeContext, this: usize) {
        let alive = ctx.get_object_bool_field(this, CurveSprite::FIELD_INDEX_alive);
        if !alive {
            return;
        }

        // 位置/旋转/缩放按 C# 语义使用 Global*（沿 *Reference 父链结算）：
        // 判定线的 positionReference 指向轨道的 positionNode，本地位置为 0，
        // 若不结算 reference，曲线永远画在原点。
        let (px, py, pz) = crate::system::native::node_native::calc_global_position(ctx, this);
        let (rx, ry, rz) = crate::system::native::node_native::calc_global_rotation(ctx, this);
        let (sx, sy, sz) = crate::system::native::node_native::calc_global_size(ctx, this);
        let color_id = ctx.get_object_object_field(this, CurveSprite::FIELD_INDEX_color);
        let points_id = ctx.get_object_object_field(this, CurveSprite::FIELD_INDEX_points);
        let w = ctx.get_object_float_field(this, CurveSprite::FIELD_INDEX_width) as f32;
        let (ca, cr, cg, cb) = read_color_channels(ctx, color_id);
        let cr = (cr * 255.0) as u8;
        let cg = (cg * 255.0) as u8;
        let cb = (cb * 255.0) as u8;
        let ca = (ca * 255.0) as u8;
        // 先读取坐标（不可变借用），再取 payload（可变借用），避免借用冲突
        let points = read_points(ctx, points_id);

        if let Some(payload) = ctx.get_payload_mut::<Box<dyn crate::adaptor::ICurveSprite>>(this) {
            payload.set_position(px, py, pz);
            payload.set_rotation(rx, ry, rz);
            payload.set_scale(sx, sy, sz);
            payload.set_color(cr, cg, cb, ca);
            payload.set_points(&points);
            payload.set_width(w);
        }
    }

    /// 1 号方法：销毁
    #[gorge_method]
    pub fn destroy(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_bool_field(this, CurveSprite::FIELD_INDEX_alive, false);
        if let Some(payload) = ctx.get_payload_mut::<Box<dyn crate::adaptor::ICurveSprite>>(this) {
            payload.destroy();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptor::{install_platform, HeadlessPlatform};
    use gorge_core::objective::native::NativeClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    fn make_vm() -> VirtualMachine {
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;
        vm
    }

    fn make_cs() -> CurveSprite {
        CurveSprite {
            alive: true, existence_reference: 0,
            position: 0, position_reference: 0,
            rotation: 0, rotation_reference: 0,
            size: 0, size_reference: 0,
            points: 0, color: 0, width: 0.1,
        }
    }

    #[test]
    fn test_curve_sprite_construct_and_update() {
        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));

        let cs = make_cs();
        let mut vm = make_vm();
        vm.register_native_class(cs.full_name(), std::sync::Arc::new(make_cs()));

        vm.param_pool.set_object_param(0, 0); // points
        let id = { let mut ctx = NativeContext::new(&mut vm); cs.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);

        // 设置宽度
        { let mut ctx = NativeContext::new(&mut vm); ctx.set_object_float_field(id, CurveSprite::FIELD_INDEX_width, 0.5); }
        { let mut ctx = NativeContext::new(&mut vm); cs.invoke_native_method(&mut ctx, id, 0); }
    }

    /// 记录型 ICurveSprite，用于不依赖全局平台单例验证 update_node 上传坐标
    struct RecordingCurve {
        uploaded: std::sync::Arc<std::sync::Mutex<Vec<(f32, f32)>>>,
    }

    impl crate::adaptor::ICurveSprite for RecordingCurve {
        fn set_position(&self, _x: f32, _y: f32, _z: f32) {}
        fn set_rotation(&self, _x: f32, _y: f32, _z: f32) {}
        fn set_scale(&self, _x: f32, _y: f32, _z: f32) {}
        fn set_points(&self, points: &[(f32, f32)]) {
            self.uploaded.lock().unwrap().extend_from_slice(points);
        }
        fn set_color(&self, _r: u8, _g: u8, _b: u8, _a: u8) {}
        fn set_width(&self, _width: f32) {}
        fn destroy(&self) {}
    }

    /// 回归：CurveSprite 必须把 points 数组的每个 Vector2 坐标上传到平台
    /// （对齐 C# `SetLine(ObjectArray)`）。此前只传点数，平台 points 全为
    /// (0,0)，导致判定线渲染成零长度线段、画面看不见。
    #[test]
    fn test_curve_sprite_uploads_point_coordinates() {
        use gorge_core::objective::object::GorgeObject;
        use gorge_core::objective::object::RuntimeObject;
        use gorge_core::objective::types::TypeCount;
        use gorge_core::system::native::array::ObjectArray;

        let cs = make_cs();
        let mut vm = make_vm();
        vm.register_native_class(cs.full_name(), std::sync::Arc::new(make_cs()));

        // 两个 Vector2 坐标点对象（float 字段 0/1 = x/y）
        let make_point = |vm: &mut VirtualMachine, x: f64, y: f64| -> usize {
            let id = vm.next_object_id;
            vm.next_object_id += 1;
            let mut object = RuntimeObject::new_simple(
                "GorgeFramework.Vector2".into(),
                &TypeCount { float_count: 2, ..TypeCount::zero() },
            );
            object.set_float_field(0, x);
            object.set_float_field(1, y);
            vm.objects.insert(id, object);
            id
        };
        let p1 = make_point(&mut vm, 10.0, 20.0);
        let p2 = make_point(&mut vm, 30.0, 40.0);

        // ObjectArray 载荷（CurveSprite.points）
        let array_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.native_payloads.insert(array_id, Box::new(ObjectArray { items: vec![p1, p2] }));

        // 构造 CurveSprite（points 参数 = array_id）。构造会尝试访问全局
        // 平台单例，但随后用 RecordingCurve 覆盖 payload，与平台无关。
        vm.param_pool.set_object_param(0, array_id);
        let id = { let mut ctx = NativeContext::new(&mut vm); cs.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);

        // 覆盖 payload 为记录型曲线，验证 update_node 上传真实坐标
        let uploaded = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let recording: Box<dyn crate::adaptor::ICurveSprite> =
            Box::new(RecordingCurve { uploaded: uploaded.clone() });
        vm.native_payloads.insert(id, Box::new(recording));

        { let mut ctx = NativeContext::new(&mut vm); cs.invoke_native_method(&mut ctx, id, 0); }

        let got = uploaded.lock().unwrap();
        assert_eq!(*got, vec![(10.0, 20.0), (30.0, 40.0)]);
    }

    #[test]
    fn test_curve_sprite_destroy() {
        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));

        let cs = make_cs();
        let mut vm = make_vm();
        vm.register_native_class(cs.full_name(), std::sync::Arc::new(make_cs()));

        vm.param_pool.set_object_param(0, 0);
        let id = { let mut ctx = NativeContext::new(&mut vm); cs.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = NativeContext::new(&mut vm); cs.invoke_native_method(&mut ctx, id, 1); }
        let alive = { let ctx = NativeContext::new(&mut vm); ctx.get_object_bool_field(id, CurveSprite::FIELD_INDEX_alive) };
        assert!(!alive);
    }
}
