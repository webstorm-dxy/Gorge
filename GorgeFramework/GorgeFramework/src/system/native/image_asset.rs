//! `GorgeFramework` — 图片资产（C# `ImageAsset`，继承自 `GraphAsset`）。
//!
//! 移植自 C# 参考实现 `System/Native/ImageAsset`。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 图片资产（C# `ImageAsset`，继承自 `GraphAsset`）
///
/// 字段 `texture` 存储 Graph 对象 ID。
/// `LoadAsset` 直接返回 true（值引用无需加载），`GetAsset` 返回 texture。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct ImageAsset {
    /// 资产名称（继承自 Asset）
    #[gorge_field]
    pub name: String,
    /// 纹理 Graph 对象 ID
    #[gorge_field]
    pub texture: usize,
}

#[gorge_native_impl]
impl ImageAsset {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_string_field(this, ImageAsset::FIELD_INDEX_name, String::new());
        ctx.set_object_object_field(this, ImageAsset::FIELD_INDEX_texture, 0);
    }

    /// 0 号方法：加载资产（覆盖，值包装无需加载）
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        true
    }

    /// 1 号方法：获取图形资产（覆盖，返回 texture 字段）
    #[gorge_method]
    pub fn get_asset(ctx: &mut NativeContext, this: usize) -> usize {
        ctx.get_object_object_field(this, ImageAsset::FIELD_INDEX_texture)
    }

    /// 2 号方法：描述显示字符串
    #[gorge_method]
    pub fn descriptor_display_string(ctx: &mut NativeContext, this: usize) -> String {
        ctx.get_object_string_field(this, ImageAsset::FIELD_INDEX_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    fn make_vm() -> VirtualMachine {
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;
        vm
    }

    fn register(vm: &mut VirtualMachine, cls: std::sync::Arc<dyn NativeClass>) {
        let name = cls.full_name().to_string();
        vm.register_native_class(&name, cls);
    }

    #[test]
    fn test_image_asset_get_asset_returns_texture() {
        let ia = ImageAsset { name: String::new(), texture: 0 };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(ImageAsset { name: String::new(), texture: 0 }));
        let id = { let mut ctx = NativeContext::new(&mut vm); ia.do_construct_native(&mut ctx, None, 0) };
        // 设置 texture 字段
        { let mut ctx = NativeContext::new(&mut vm); ctx.set_object_object_field(id, ImageAsset::FIELD_INDEX_texture, 42); }
        // GetAsset 返回 texture
        { let mut ctx = NativeContext::new(&mut vm); ia.invoke_native_method(&mut ctx, id, 1); }
        assert_eq!(vm.param_pool.get_object_return(), 42);
        // LoadAsset 返回 true
        { let mut ctx = NativeContext::new(&mut vm); ia.invoke_native_method(&mut ctx, id, 0); }
        assert!(vm.param_pool.get_bool_return());
    }

    #[test]
    fn test_image_asset_descriptor_display_string() {
        let ia = ImageAsset { name: String::new(), texture: 0 };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(ImageAsset { name: String::new(), texture: 0 }));
        let id = { let mut ctx = NativeContext::new(&mut vm); ia.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = NativeContext::new(&mut vm); ctx.set_object_string_field(id, ImageAsset::FIELD_INDEX_name, "test_img".to_string()); }
        { let mut ctx = NativeContext::new(&mut vm); ia.invoke_native_method(&mut ctx, id, 2); }
        assert_eq!(vm.param_pool.get_string_return(), "test_img");
    }
}
