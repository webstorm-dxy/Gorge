//! `GorgeFramework.NineSliceSprite` —— 九宫格精灵渲染节点 native 类。
//!
//! 移植自 C# 参考实现 `NineSliceSprite.cs`。
//! 继承自 Node，额外包含 graph/slice/basSize/color/hsl 字段。
//! 构造时通过 PlatformBase 创建 INineSliceSprite 对象。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use crate::system::native::color_argb::read_color_channels;

/// 九宫格精灵渲染节点（C# `NineSliceSprite`，继承自 `Node`）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct NineSliceSprite {
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
    // ---- NineSliceSprite 专属字段 ----
    /// 图形对象 ID
    #[gorge_field]
    pub graph: usize,
    /// 切片左上角 Vector2 对象 ID
    #[gorge_field]
    pub slice_left_top: usize,
    /// 切片右下角 Vector2 对象 ID
    #[gorge_field]
    pub slice_right_bottom: usize,
    /// 基准尺寸 Vector2 对象 ID
    #[gorge_field]
    pub base_size: usize,
    /// 颜色对象 ID（ColorArgb）
    #[gorge_field]
    pub color: usize,
    /// HSL 色偏 Vector3 对象 ID
    #[gorge_field]
    pub hsl: usize,
}

#[gorge_native_impl]
impl NineSliceSprite {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, graph: usize, slice_left_top: usize, slice_right_bottom: usize, base_size: usize) {
        // Node 字段初始化
        ctx.set_object_bool_field(this, NineSliceSprite::FIELD_INDEX_alive, true);
        ctx.set_object_object_field(this, NineSliceSprite::FIELD_INDEX_existence_reference, 0);
        ctx.set_object_float_field(this, NineSliceSprite::FIELD_INDEX_position_x, 0.0);
        ctx.set_object_float_field(this, NineSliceSprite::FIELD_INDEX_position_y, 0.0);
        ctx.set_object_float_field(this, NineSliceSprite::FIELD_INDEX_position_z, 0.0);
        ctx.set_object_object_field(this, NineSliceSprite::FIELD_INDEX_position_reference, 0);
        ctx.set_object_float_field(this, NineSliceSprite::FIELD_INDEX_rotation_x, 0.0);
        ctx.set_object_float_field(this, NineSliceSprite::FIELD_INDEX_rotation_y, 0.0);
        ctx.set_object_float_field(this, NineSliceSprite::FIELD_INDEX_rotation_z, 0.0);
        ctx.set_object_object_field(this, NineSliceSprite::FIELD_INDEX_rotation_reference, 0);
        ctx.set_object_float_field(this, NineSliceSprite::FIELD_INDEX_size_x, 1.0);
        ctx.set_object_float_field(this, NineSliceSprite::FIELD_INDEX_size_y, 1.0);
        ctx.set_object_float_field(this, NineSliceSprite::FIELD_INDEX_size_z, 1.0);
        ctx.set_object_object_field(this, NineSliceSprite::FIELD_INDEX_size_reference, 0);
        // 专属字段
        ctx.set_object_object_field(this, NineSliceSprite::FIELD_INDEX_graph, graph);
        ctx.set_object_object_field(this, NineSliceSprite::FIELD_INDEX_slice_left_top, slice_left_top);
        ctx.set_object_object_field(this, NineSliceSprite::FIELD_INDEX_slice_right_bottom, slice_right_bottom);
        ctx.set_object_object_field(this, NineSliceSprite::FIELD_INDEX_base_size, base_size);
        ctx.set_object_object_field(this, NineSliceSprite::FIELD_INDEX_color, 0);
        ctx.set_object_object_field(this, NineSliceSprite::FIELD_INDEX_hsl, 0);

        // 创建平台 INineSliceSprite 对象
        use crate::adaptor::{platform, platform_installed};
        if platform_installed() {
            let sprite_obj = platform().create_nine_slice_sprite();
            // 读取 Vector2 值
            let lt_x = ctx.get_object_float_field(slice_left_top, 0);
            let lt_y = ctx.get_object_float_field(slice_left_top, 1);
            let rb_x = ctx.get_object_float_field(slice_right_bottom, 0);
            let rb_y = ctx.get_object_float_field(slice_right_bottom, 1);
            let bs_x = ctx.get_object_float_field(base_size, 0);
            let bs_y = ctx.get_object_float_field(base_size, 1);
            let graph_handle = crate::runtime::environment::global::resolve_graph_handle(
                ctx.vm as *mut _ as usize,
                graph,
            );
            sprite_obj.set_graph(
                graph_handle,
                bs_x as f32,
                bs_y as f32,
                lt_x as f32,
                lt_y as f32,
                rb_x as f32,
                rb_y as f32,
            );
            ctx.insert_payload(this, Box::new(sprite_obj));
        }
    }

    /// 0 号方法：更新节点
    #[gorge_method]
    pub fn update_node(ctx: &mut NativeContext, this: usize) {
        let alive = ctx.get_object_bool_field(this, NineSliceSprite::FIELD_INDEX_alive);
        if !alive {
            return;
        }

        let px = ctx.get_object_float_field(this, NineSliceSprite::FIELD_INDEX_position_x) as f32;
        let py = ctx.get_object_float_field(this, NineSliceSprite::FIELD_INDEX_position_y) as f32;
        let pz = ctx.get_object_float_field(this, NineSliceSprite::FIELD_INDEX_position_z) as f32;
        let rx = ctx.get_object_float_field(this, NineSliceSprite::FIELD_INDEX_rotation_x) as f32;
        let ry = ctx.get_object_float_field(this, NineSliceSprite::FIELD_INDEX_rotation_y) as f32;
        let rz = ctx.get_object_float_field(this, NineSliceSprite::FIELD_INDEX_rotation_z) as f32;
        let sx = ctx.get_object_float_field(this, NineSliceSprite::FIELD_INDEX_size_x) as f32;
        let sy = ctx.get_object_float_field(this, NineSliceSprite::FIELD_INDEX_size_y) as f32;
        let sz = ctx.get_object_float_field(this, NineSliceSprite::FIELD_INDEX_size_z) as f32;
        let color_id = ctx.get_object_object_field(this, NineSliceSprite::FIELD_INDEX_color);
        let hsl_id = ctx.get_object_object_field(this, NineSliceSprite::FIELD_INDEX_hsl);
        let (ca, cr, cg, cb) = read_color_channels(ctx, color_id);
        let cr = (cr * 255.0) as u8;
        let cg = (cg * 255.0) as u8;
        let cb = (cb * 255.0) as u8;
        let ca = (ca * 255.0) as u8;
        let h = if hsl_id != 0 { ctx.get_object_float_field(hsl_id, 0) as f32 } else { 0.0 };
        let s = if hsl_id != 0 { ctx.get_object_float_field(hsl_id, 1) as f32 } else { 0.0 };
        let l = if hsl_id != 0 { ctx.get_object_float_field(hsl_id, 2) as f32 } else { 0.0 };

        if let Some(payload) = ctx.get_payload_mut::<Box<dyn crate::adaptor::INineSliceSprite>>(this) {
            payload.set_position(px, py, pz);
            payload.set_rotation(rx, ry, rz);
            payload.set_scale(sx, sy, sz);
            payload.set_color(cr, cg, cb, ca);
            payload.set_hsl(h, s, l);
        }
    }

    /// 1 号方法：销毁
    #[gorge_method]
    pub fn destroy(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_bool_field(this, NineSliceSprite::FIELD_INDEX_alive, false);
        if let Some(payload) = ctx.get_payload_mut::<Box<dyn crate::adaptor::INineSliceSprite>>(this) {
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

    fn make_nss() -> NineSliceSprite {
        NineSliceSprite {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
            graph: 0, slice_left_top: 0, slice_right_bottom: 0, base_size: 0, color: 0, hsl: 0,
        }
    }

    #[test]
    fn test_nine_slice_construct_and_update() {
        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));

        let ns = make_nss();
        let mut vm = make_vm();
        vm.register_native_class(ns.full_name(), std::sync::Arc::new(make_nss()));

        vm.param_pool.set_object_param(0, 0); // graph
        vm.param_pool.set_object_param(1, 0); // sliceLeftTop
        vm.param_pool.set_object_param(2, 0); // sliceRightBottom
        vm.param_pool.set_object_param(3, 0); // baseSize
        let id = { let mut ctx = NativeContext::new(&mut vm); ns.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);

        { let mut ctx = NativeContext::new(&mut vm); ns.invoke_native_method(&mut ctx, id, 0); }
    }

    #[test]
    fn test_nine_slice_destroy() {
        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));

        let ns = make_nss();
        let mut vm = make_vm();
        vm.register_native_class(ns.full_name(), std::sync::Arc::new(make_nss()));

        for i in 0..4 { vm.param_pool.set_object_param(i, 0); }
        let id = { let mut ctx = NativeContext::new(&mut vm); ns.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = NativeContext::new(&mut vm); ns.invoke_native_method(&mut ctx, id, 1); }
        let alive = { let ctx = NativeContext::new(&mut vm); ctx.get_object_bool_field(id, NineSliceSprite::FIELD_INDEX_alive) };
        assert!(!alive);
    }
}
