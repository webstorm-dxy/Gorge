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
    /// 局部位置（Vector3 对象 ID）
    #[gorge_field]
    pub position: usize,
    /// 位置引用（Node 对象 ID）
    #[gorge_field]
    pub position_reference: usize,
    /// 局部旋转（Vector3 对象 ID）
    #[gorge_field]
    pub rotation: usize,
    /// 旋转引用
    #[gorge_field]
    pub rotation_reference: usize,
    /// 局部缩放（Vector3 对象 ID）
    #[gorge_field]
    pub size: usize,
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
        ctx.set_object_object_field(this, Sprite::FIELD_INDEX_position, 0);
        ctx.set_object_object_field(this, Sprite::FIELD_INDEX_position_reference, 0);
        ctx.set_object_object_field(this, Sprite::FIELD_INDEX_rotation, 0);
        ctx.set_object_object_field(this, Sprite::FIELD_INDEX_rotation_reference, 0);
        ctx.set_object_object_field(this, Sprite::FIELD_INDEX_size, 0);
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
        let (px, py, pz) = crate::system::native::node_native::Node::read_vec3_field(
            ctx, this, Sprite::FIELD_INDEX_position, false,
        );
        let (rx, ry, rz) = crate::system::native::node_native::Node::read_vec3_field(
            ctx, this, Sprite::FIELD_INDEX_rotation, false,
        );
        let (sx, sy, sz) = crate::system::native::node_native::Node::read_vec3_field(
            ctx, this, Sprite::FIELD_INDEX_size, true,
        );
        let graph = ctx.get_object_object_field(this, Sprite::FIELD_INDEX_graph);
        let color_id = ctx.get_object_object_field(this, Sprite::FIELD_INDEX_color);
        let (ca, cr, cg, cb) = read_color_channels(ctx, color_id);
        let cr = (cr * 255.0) as u8;
        let cg = (cg * 255.0) as u8;
        let cb = (cb * 255.0) as u8;
        let ca = (ca * 255.0) as u8;
        let vm_address = ctx.vm as *mut _ as usize;
        let graph_handle = crate::runtime::environment::global::resolve_graph_handle(
            vm_address,
            graph,
        );

        if let Some(payload) = ctx.get_payload_mut::<Box<dyn crate::adaptor::ISprite>>(this) {
            payload.set_position(px, py, pz);
            payload.set_rotation(rx, ry, rz);
            payload.set_scale(sx, sy, sz);
            payload.set_color(cr, cg, cb, ca);
            payload.set_graph(graph_handle);
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
            position: 0, position_reference: 0,
            rotation: 0, rotation_reference: 0,
            size: 0, size_reference: 0,
            graph: 0, color: 0,
        };
        let mut vm = make_vm();
        vm.register_native_class(s.full_name(), std::sync::Arc::new(Sprite {
            alive: true, existence_reference: 0,
            position: 0, position_reference: 0,
            rotation: 0, rotation_reference: 0,
            size: 0, size_reference: 0,
            graph: 0, color: 0,
        }));

        // 构造 Sprite(graph=0)
        vm.param_pool.set_object_param(0, 0);
        let id = { let mut ctx = NativeContext::new(&mut vm); s.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);

        // 设置位置
        {
            let vec3_id = vm.next_object_id;
            vm.next_object_id += 1;
            vm.objects.insert(
                vec3_id,
                gorge_core::objective::object::RuntimeObject::new_simple(
                    "GorgeFramework.Vector3".into(),
                    &gorge_core::objective::types::TypeCount {
                        float_count: 3,
                        ..gorge_core::objective::types::TypeCount::zero()
                    },
                ),
            );
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_float_field(vec3_id, 0, 100.0);
            ctx.set_object_float_field(vec3_id, 1, 200.0);
            ctx.set_object_float_field(vec3_id, 2, 0.0);
            ctx.set_object_object_field(id, Sprite::FIELD_INDEX_position, vec3_id);
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
            position: 0, position_reference: 0,
            rotation: 0, rotation_reference: 0,
            size: 0, size_reference: 0,
            graph: 0, color: 0,
        };
        let mut vm = make_vm();
        vm.register_native_class(s.full_name(), std::sync::Arc::new(Sprite {
            alive: true, existence_reference: 0,
            position: 0, position_reference: 0,
            rotation: 0, rotation_reference: 0,
            size: 0, size_reference: 0,
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
            position: 0, position_reference: 0,
            rotation: 0, rotation_reference: 0,
            size: 0, size_reference: 0,
            graph: 0, color: 0,
        };
        let mut vm = make_vm();
        vm.register_native_class(s.full_name(), std::sync::Arc::new(Sprite {
            alive: true, existence_reference: 0,
            position: 0, position_reference: 0,
            rotation: 0, rotation_reference: 0,
            size: 0, size_reference: 0,
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
