//! `GorgeFramework.VideoAsset` / `NativeVideoAsset` —— 视频资产 native 类。
//!
//! 移植自 C# 参考实现 `VideoAsset.cs`。
//! VideoAsset 通过 Environment.GetAssetByName 查找资产；
//! NativeVideoAsset 直接持有 Video 对象引用。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

// ==================== VideoAsset ====================

/// 视频资产（C# `VideoAsset`，继承自 `Asset`）
///
/// `LoadAsset` 通过 `Environment.GetAssetByName(name)` 查找视频资产。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct VideoAsset {
    /// 资产名称
    #[gorge_field]
    pub name: String,
}

#[gorge_native_impl]
impl VideoAsset {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_string_field(this, VideoAsset::FIELD_INDEX_name, String::new());
    }

    /// 0 号方法：加载资产（覆盖 Asset.LoadAsset）
    #[gorge_method]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        let _name = ctx.get_object_string_field(this, VideoAsset::FIELD_INDEX_name);
        // 骨架：暂无 AssetManager，留待 S7
        false
    }

    /// 1 号方法：获取视频资产
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn get_asset(ctx: &mut NativeContext, this: usize) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    #[test]
    fn test_video_asset_construct() {
        let va = VideoAsset { name: String::new() };
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;
        vm.register_native_class(va.full_name(), std::sync::Arc::new(VideoAsset { name: String::new() }));
        let id = { let mut ctx = NativeContext::new(&mut vm); va.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        // LoadAsset 骨架返回 false
        { let mut ctx = NativeContext::new(&mut vm); va.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return());
    }
}
