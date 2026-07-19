//! `GorgeFramework.Sprite` —— 精灵渲染节点 native 类。
//!
//! 移植自 C# 参考实现 `Sprite.cs`。
//! 继承自 Node（字段包含 Node 的完整字段集 + Sprite 专属字段）。
//! 构造时通过 PlatformBase 创建 ISprite 对象，UpdateNode 时同步位置/颜色/图像。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use crate::system::native::color_argb::read_color_channels;

/// 精灵渲染节点（C# `Sprite`，继承自 `Node`）
///
/// 字段：Node 的 14 个字段（alive/existenceReference/position*/rotation*/size*）
/// + graph(object) + color(object)。
/// 平台 ISprite 对象存入 payload，UpdateNode 时同步调用。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Sprite {
    // ---- Node 继承字段 ----
    /// 是否存活
    #[gorge_field]
    pub alive: bool,
    /// 存活依赖（Node 对象 ID）
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
    /// 位置引用（Node 对象 ID）
    #[gorge_field]
    pub position_reference: usize,
    /// 局部旋转 x
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
    // ---- Sprite 专属字段 ----
    /// 图形对象 ID
    #[gorge_field]
    pub graph: usize,
    /// 颜色对象 ID（ColorArgb）
    #[gorge_field]
    pub color: usize,
}

#[gorge_native_impl]
impl Sprite {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, graph: usize) {
        // Node 字段初始化
        ctx.set_object_bool_field(this, Sprite::FIELD_INDEX_alive, true);
        ctx.set_object_object_field(this, Sprite::FIELD_INDEX_existence_reference, 0);
        ctx.set_object_float_field(this, Sprite::FIELD_INDEX_position_x, 0.0);
        ctx.set_object_float_field(this, Sprite::FIELD_INDEX_position_y, 0.0);
        ctx.set_object_float_field(this, Sprite::FIELD_INDEX_position_z, 0.0);
        ctx.set_object_object_field(this, Sprite::FIELD_INDEX_position_reference, 0);
        ctx.set_object_float_field(this, Sprite::FIELD_INDEX_rotation_x, 0.0);
        ctx.set_object_float_field(this, Sprite::FIELD_INDEX_rotation_y, 0.0);
        ctx.set_object_float_field(this, Sprite::FIELD_INDEX_rotation_z, 0.0);
        ctx.set_object_object_field(this, Sprite::FIELD_INDEX_rotation_reference, 0);
        ctx.set_object_float_field(this, Sprite::FIELD_INDEX_size_x, 1.0);
        ctx.set_object_float_field(this, Sprite::FIELD_INDEX_size_y, 1.0);
        ctx.set_object_float_field(this, Sprite::FIELD_INDEX_size_z, 1.0);
        ctx.set_object_object_field(this, Sprite::FIELD_INDEX_size_reference, 0);
        // Sprite 字段
        ctx.set_object_object_field(this, Sprite::FIELD_INDEX_graph, graph);
        ctx.set_object_object_field(this, Sprite::FIELD_INDEX_color, 0);

        // 创建平台 ISprite 对象
        use crate::adaptor::{platform, platform_installed};
        if platform_installed() {
            let sprite_obj = platform().create_sprite();
            ctx.insert_payload(this, Box::new(sprite_obj));
        }
    }

    /// 0 号方法：更新节点（C# `UpdateNode`）
    ///
    /// 同步位置/旋转/缩放/颜色/图像到平台 ISprite。
    #[gorge_method]
    pub fn update_node(ctx: &mut NativeContext, this: usize) {
        let alive = ctx.get_object_bool_field(this, Sprite::FIELD_INDEX_alive);
        if !alive {
            return;
        }

        // 先读取所有字段值（避免与 get_payload_mut 的借用冲突）
        let px = ctx.get_object_float_field(this, Sprite::FIELD_INDEX_position_x) as f32;
        let py = ctx.get_object_float_field(this, Sprite::FIELD_INDEX_position_y) as f32;
        let pz = ctx.get_object_float_field(this, Sprite::FIELD_INDEX_position_z) as f32;
        let rx = ctx.get_object_float_field(this, Sprite::FIELD_INDEX_rotation_x) as f32;
        let ry = ctx.get_object_float_field(this, Sprite::FIELD_INDEX_rotation_y) as f32;
        let rz = ctx.get_object_float_field(this, Sprite::FIELD_INDEX_rotation_z) as f32;
        let sx = ctx.get_object_float_field(this, Sprite::FIELD_INDEX_size_x) as f32;
        let sy = ctx.get_object_float_field(this, Sprite::FIELD_INDEX_size_y) as f32;
        let sz = ctx.get_object_float_field(this, Sprite::FIELD_INDEX_size_z) as f32;
        let graph = ctx.get_object_object_field(this, Sprite::FIELD_INDEX_graph);
        let color_id = ctx.get_object_object_field(this, Sprite::FIELD_INDEX_color);
        let (ca, cr, cg, cb) = read_color_channels(ctx, color_id);
        let cr = (cr * 255.0) as u8;
        let cg = (cg * 255.0) as u8;
        let cb = (cb * 255.0) as u8;
        let ca = (ca * 255.0) as u8;

        if let Some(payload) = ctx.get_payload_mut::<Box<dyn crate::adaptor::ISprite>>(this) {
            payload.set_position(px, py, pz);
            payload.set_rotation(rx, ry, rz);
            payload.set_scale(sx, sy, sz);
            payload.set_color(cr, cg, cb, ca);
            payload.set_graph(graph);
        }
    }

    /// 1 号方法：销毁（C# `Destroy`）
    #[gorge_method]
    pub fn destroy(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_bool_field(this, Sprite::FIELD_INDEX_alive, false);
        if let Some(payload) = ctx.get_payload_mut::<Box<dyn crate::adaptor::ISprite>>(this) {
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

    #[test]
    fn test_sprite_construct_and_update_node() {
        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));

        let s = Sprite {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
            graph: 0, color: 0,
        };
        let mut vm = make_vm();
        vm.register_native_class(s.full_name(), std::sync::Arc::new(Sprite {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
            graph: 0, color: 0,
        }));

        // 构造 Sprite(graph=0)
        vm.param_pool.set_object_param(0, 0);
        let id = { let mut ctx = NativeContext::new(&mut vm); s.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);

        // 设置位置
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_float_field(id, Sprite::FIELD_INDEX_position_x, 100.0);
            ctx.set_object_float_field(id, Sprite::FIELD_INDEX_position_y, 200.0);
            ctx.set_object_float_field(id, Sprite::FIELD_INDEX_position_z, 0.0);
        }

        // UpdateNode
        { let mut ctx = NativeContext::new(&mut vm); s.invoke_native_method(&mut ctx, id, 0); }

        // 验证已安装
        let _p = crate::adaptor::platform();
        assert!(_p.viewport_size() == (1920.0, 1080.0));
    }

    #[test]
    fn test_sprite_destroy() {
        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));

        let s = Sprite {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
            graph: 0, color: 0,
        };
        let mut vm = make_vm();
        vm.register_native_class(s.full_name(), std::sync::Arc::new(Sprite {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
            graph: 0, color: 0,
        }));

        vm.param_pool.set_object_param(0, 0);
        let id = { let mut ctx = NativeContext::new(&mut vm); s.do_construct_native(&mut ctx, None, 0) };
        // Destroy
        { let mut ctx = NativeContext::new(&mut vm); s.invoke_native_method(&mut ctx, id, 1); }
        let alive = { let ctx = NativeContext::new(&mut vm); ctx.get_object_bool_field(id, Sprite::FIELD_INDEX_alive) };
        assert!(!alive);
    }

    #[test]
    fn test_sprite_update_node_not_alive_skips() {
        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));

        let s = Sprite {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
            graph: 0, color: 0,
        };
        let mut vm = make_vm();
        vm.register_native_class(s.full_name(), std::sync::Arc::new(Sprite {
            alive: true, existence_reference: 0,
            position_x: 0.0, position_y: 0.0, position_z: 0.0, position_reference: 0,
            rotation_x: 0.0, rotation_y: 0.0, rotation_z: 0.0, rotation_reference: 0,
            size_x: 1.0, size_y: 1.0, size_z: 1.0, size_reference: 0,
            graph: 0, color: 0,
        }));

        vm.param_pool.set_object_param(0, 0);
        let id = { let mut ctx = NativeContext::new(&mut vm); s.do_construct_native(&mut ctx, None, 0) };
        // 设为不存活
        { let mut ctx = NativeContext::new(&mut vm); ctx.set_object_bool_field(id, Sprite::FIELD_INDEX_alive, false); }
        // UpdateNode 应跳过（不 panic）
        { let mut ctx = NativeContext::new(&mut vm); s.invoke_native_method(&mut ctx, id, 0); }
    }
}
