//! `GorgeFramework.VideoAsset` —— 视频资产 native 类。
//!
//! 移植自 C# 参考实现 `VideoAsset.cs`。
//! VideoAsset 通过 Environment.GetAssetByName 查找资产。

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
    ///
    /// 对齐 C# `VideoAsset.LoadAsset`：`Environment.GetAssetByName(name)` 查找
    /// 同名资产，再调其 `GetAsset()` 取出 Video 对象并缓存（对应 C# 私有字段
    /// `_video`，以 native 载荷形式存放）。资产未找到或并非视频资产族时返回
    /// false（对齐 C# try/catch 吞异常语义）。
    #[gorge_method]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        let name = ctx.get_object_string_field(this, VideoAsset::FIELD_INDEX_name);
        if name.is_empty() {
            return false;
        }
        // Environment.GetAssetByName(name)：静态方法 0 号，返回资产对象 ID
        ctx.vm.param_pool.set_string_param(0, name);
        ctx.set_object_return(0);
        ctx.invoke_native_static_on("GorgeFramework.Environment", 0);
        let asset_id = ctx.vm.param_pool.get_object_return();
        if asset_id == 0 {
            return false;
        }
        // C# `FromGorgeObject(asset)` 强转 VideoAsset：非视频资产族视为失败。
        // 编译类子类对象经 native_object_id 解析到其 native 基类对象再判定。
        let native_id = ctx.vm.objects.get(&asset_id)
            .and_then(|object| object.native_object_id)
            .unwrap_or(asset_id);
        let is_video_family = ctx.vm.objects.get(&native_id)
            .map(|object| matches!(
                object.class_name.as_str(),
                "GorgeFramework.VideoAsset" | "GorgeFramework.NativeVideoAsset"
            ))
            .unwrap_or(false);
        if !is_video_family {
            return false;
        }
        // 调用资产对象的 GetAsset（方法 1 号）取 Video 对象并缓存（允许为 0，
        // 对齐 C# GetAsset 返回 null 仍视为加载成功）
        ctx.set_object_return(0);
        ctx.invoke_native_method_on("GorgeFramework.VideoAsset", asset_id, 1);
        let video_id = ctx.vm.param_pool.get_object_return();
        ctx.insert_payload(this, Box::new(video_id));
        true
    }

    /// 1 号方法：获取视频资产
    ///
    /// 返回 `load_asset` 缓存的 Video 对象 ID（对应 C# `_video` 字段），
    /// 未加载时返回 0（null）。
    #[gorge_method]
    pub fn get_asset(ctx: &mut NativeContext, this: usize) -> usize {
        ctx.get_payload::<usize>(this).copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;
    use crate::runtime::environment::global;
    use crate::system::native::environment::Environment;
    use crate::system::native::native_video_asset::NativeVideoAsset;

    /// 构造注册桥接链路相关 native 类的 VM，并确保全局环境已初始化
    fn make_asset_vm() -> (VirtualMachine, VideoAsset) {
        global::init_env_global();
        let va = VideoAsset { name: String::new() };
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;
        vm.register_native_class(va.full_name(), std::sync::Arc::new(VideoAsset { name: String::new() }));
        vm.register_native_class("GorgeFramework.Environment", std::sync::Arc::new(Environment {}));
        vm.register_native_class(
            "GorgeFramework.NativeVideoAsset",
            std::sync::Arc::new(NativeVideoAsset { name: String::new(), video: 0 }),
        );
        (vm, va)
    }

    #[test]
    fn test_video_asset_construct() {
        let (mut vm, va) = make_asset_vm();
        let id = { let mut ctx = NativeContext::new(&mut vm); va.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        // name 为空时 LoadAsset 直接返回 false
        { let mut ctx = NativeContext::new(&mut vm); va.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return());
        // 未加载时 GetAsset 返回 0（null）
        { let mut ctx = NativeContext::new(&mut vm); va.invoke_native_method(&mut ctx, id, 1); }
        assert_eq!(vm.param_pool.get_object_return(), 0);
    }

    #[test]
    fn test_video_asset_load_success() {
        let (mut vm, va) = make_asset_vm();
        // 登记平台视频句柄（唯一键避免并行测试冲突）
        global::with_env_global_mut(|env| {
            env.assets.insert("video:test_p1_2_load".to_string(), 888);
        });
        let id = { let mut ctx = NativeContext::new(&mut vm); va.do_construct_native(&mut ctx, None, 0) };
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_string_field(id, VideoAsset::FIELD_INDEX_name, "video:test_p1_2_load".to_string());
        }

        { let mut ctx = NativeContext::new(&mut vm); va.invoke_native_method(&mut ctx, id, 0); }
        assert!(vm.param_pool.get_bool_return(), "已注册的视频资产应加载成功");

        { let mut ctx = NativeContext::new(&mut vm); va.invoke_native_method(&mut ctx, id, 1); }
        let video_id = vm.param_pool.get_object_return();
        assert_ne!(video_id, 0, "GetAsset 应返回 Video 对象 ID");
        assert_eq!(vm.objects[&video_id].class_name, "GorgeFramework.Video");
        let vm_address = &mut vm as *mut VirtualMachine as usize;
        assert_eq!(
            global::resolve_video_handle(vm_address, video_id),
            888,
            "Video 对象应解析回平台视频句柄"
        );
    }

    #[test]
    fn test_video_asset_load_not_found() {
        let (mut vm, va) = make_asset_vm();
        let id = { let mut ctx = NativeContext::new(&mut vm); va.do_construct_native(&mut ctx, None, 0) };
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_string_field(id, VideoAsset::FIELD_INDEX_name, "video:test_p1_2_missing".to_string());
        }
        { let mut ctx = NativeContext::new(&mut vm); va.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return(), "未注册的资产应加载失败");
        { let mut ctx = NativeContext::new(&mut vm); va.invoke_native_method(&mut ctx, id, 1); }
        assert_eq!(vm.param_pool.get_object_return(), 0);
    }

    #[test]
    fn test_video_asset_load_non_video_asset() {
        let (mut vm, va) = make_asset_vm();
        // audio: 前缀资产被包装为 NativeAudioAsset，对齐 C# 强转失败返回 false
        global::with_env_global_mut(|env| {
            env.assets.insert("audio:test_p1_2_wrong_type".to_string(), 556);
        });
        let id = { let mut ctx = NativeContext::new(&mut vm); va.do_construct_native(&mut ctx, None, 0) };
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_string_field(id, VideoAsset::FIELD_INDEX_name, "audio:test_p1_2_wrong_type".to_string());
        }
        { let mut ctx = NativeContext::new(&mut vm); va.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return(), "非视频资产族应加载失败");
    }
}
