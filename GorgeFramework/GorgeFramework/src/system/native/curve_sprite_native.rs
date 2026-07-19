//! `GorgeFramework.CurveSprite` —— 曲线精灵渲染节点 native 类。
//!
//! 移植自 C# 参考实现 `CurveSprite.cs`。
//! 继承自 Node，额外包含 points/color/width 字段。
//! 构造时通过 PlatformBase 创建 ICurveSprite 对象。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use crate::system::native::color_argb::read_color_channels;

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
    #[gorge_field]
    pub position_x: f32,
    #[gorge_field]
    pub position_y: f32,
    #[gorge_field]
    pub position_z: f32,
    #[gorge_field]
    pub position_reference: usize,
    #[gorge_field]
    pub rotation_x: f32,
    #[gorge_field]
    pub rotation_y: f32,
    #[gorge_field]
    pub rotation_z: f32,
    #[gorge_field]
    pub rotation_reference: usize,
    #[gorge_field]
    pub size_x: f32,
    #[gorge_field]
    pub size_y: f32,
    #[gorge_field]
    pub size_z: f32,
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
        ctx.set_object_float_field(this, CurveSprite::FIELD_INDEX_position_x, 0.0);
        ctx.set_object_float_field(this, CurveSprite::FIELD_INDEX_position_y, 0.0);
        ctx.set_object_float_field(this, CurveSprite::FIELD_INDEX_position_z, 0.0);
        ctx.set_object_object_field(this, CurveSprite::FIELD_INDEX_position_reference, 0);
        ctx.set_object_float_field(this, CurveSprite::FIELD_INDEX_rotation_x, 0.0);
        ctx.set_object_float_field(this, CurveSprite::FIELD_INDEX_rotation_y, 0.0);
        ctx.set_object_float_field(this, CurveSprite::FIELD_INDEX_rotation_z, 0.0);
        ctx.set_object_object_field(this, CurveSprite::FIELD_INDEX_rotation_reference, 0);
        ctx.set_object_float_field(this, CurveSprite::FIELD_INDEX_size_x, 1.0);
        ctx.set_object_float_field(this, CurveSprite::FIELD_INDEX_size_y, 1.0);
        ctx.set_object_float_field(this, CurveSprite::FIELD_INDEX_size_z, 1.0);
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
                let pt_count = ctx.object_array_len(points);
                sprite_obj.set_line(pt_count);
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

        let px = ctx.get_object_float_field(this, CurveSprite::FIELD_INDEX_position_x) as f32;
        let py = ctx.get_object_float_field(this, CurveSprite::FIELD_INDEX_position_y) as f32;
        let pz = ctx.get_object_float_field(this, CurveSprite::FIELD_INDEX_position_z) as f32;
        let rx = ctx.get_object_float_field(this, CurveSprite::FIELD_INDEX_rotation_x) as f32;
        let ry = ctx.get_object_float_field(this, CurveSprite::FIELD_INDEX_rotation_y) as f32;
        let rz = ctx.get_object_float_field(this, CurveSprite::FIELD_INDEX_rotation_z) as f32;
        let sx = ctx.get_object_float_field(this, CurveSprite::FIELD_INDEX_size_x) as f32;
        let sy = ctx.get_object_float_field(this, CurveSprite::FIELD_INDEX_size_y) as f32;
        let sz = ctx.get_object_float_field(this, CurveSprite::FIELD_INDEX_size_z) as f32;
        let color_id = ctx.get_object_object_field(this, CurveSprite::FIELD_INDEX_color);
        let points_id = ctx.get_object_object_field(this, CurveSprite::FIELD_INDEX_points);
        let w = ctx.get_object_float_field(this, CurveSprite::FIELD_INDEX_width) as f32;
        let pt_count = if points_id != 0 { ctx.object_array_len(points_id) } else { 0 };
        let (ca, cr, cg, cb) = read_color_channels(ctx, color_id);
        let cr = (cr * 255.0) as u8;
        let cg = (cg * 255.0) as u8;
        let cb = (cb * 255.0) as u8;
        let ca = (ca * 255.0) as u8;

        if let Some(payload) = ctx.get_payload_mut::<Box<dyn crate::adaptor::ICurveSprite>>(this) {
            payload.set_position(px, py, pz);
            payload.set_rotation(rx, ry, rz);
            payload.set_scale(sx, sy, sz);
            payload.set_color(cr, cg, cb, ca);
            if points_id != 0 { payload.set_line(pt_count); }
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
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
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
