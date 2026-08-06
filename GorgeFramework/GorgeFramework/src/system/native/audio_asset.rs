//! `GorgeFramework.AudioAsset` —— 音频资产 native 类。
//!
//! 移植自 C# 参考实现 `AudioAsset.cs`。
//! AudioAsset 通过 Environment.GetAssetByName 查找资产。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

// ==================== AudioAsset ====================

/// 音频资产（C# `AudioAsset`，继承自 `Asset`）
///
/// `LoadAsset` 通过 `Environment.GetAssetByName(name)` 查找同名音频资产
/// （如 `NativeAudioAsset`），取其 `GetAsset` 结果作为音频对象。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct AudioAsset {
    /// 资产名称
    #[gorge_field]
    pub name: String,
}

#[gorge_native_impl]
impl AudioAsset {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_string_field(this, AudioAsset::FIELD_INDEX_name, String::new());
    }

    /// 0 号方法：加载资产（覆盖 Asset.LoadAsset）
    ///
    /// 对齐 C# `AudioAsset.LoadAsset`：`Environment.GetAssetByName(name)` 查找
    /// 同名资产，再调其 `GetAsset()` 取出 Audio 对象并缓存（对应 C# 私有字段
    /// `_audio`，以 native 载荷形式存放）。资产未找到或并非音频资产族时返回
    /// false（对齐 C# try/catch 吞异常语义）。
    #[gorge_method]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        let name = ctx.get_object_string_field(this, AudioAsset::FIELD_INDEX_name);
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
        // C# `FromGorgeObject(asset)` 强转 AudioAsset：非音频资产族视为失败。
        // 编译类子类对象经 native_object_id 解析到其 native 基类对象再判定。
        let native_id = ctx.vm.objects.get(&asset_id)
            .and_then(|object| object.native_object_id)
            .unwrap_or(asset_id);
        let is_audio_family = ctx.vm.objects.get(&native_id)
            .map(|object| matches!(
                object.class_name.as_str(),
                "GorgeFramework.AudioAsset"
                    | "GorgeFramework.NativeAudioAsset"
                    | "GorgeFramework.WavAudioAsset"
            ))
            .unwrap_or(false);
        if !is_audio_family {
            return false;
        }
        // 调用资产对象的 GetAsset（方法 1 号）取 Audio 对象并缓存（允许为 0，
        // 对齐 C# GetAsset 返回 null 仍视为加载成功）
        ctx.set_object_return(0);
        ctx.invoke_native_method_on("GorgeFramework.AudioAsset", asset_id, 1);
        let audio_id = ctx.vm.param_pool.get_object_return();
        ctx.insert_payload(this, Box::new(audio_id));
        true
    }

    /// 1 号方法：获取音频资产
    ///
    /// 返回 `load_asset` 缓存的 Audio 对象 ID（对应 C# `_audio` 字段），
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
    use crate::system::native::native_audio_asset::NativeAudioAsset;

    fn make_vm() -> VirtualMachine {
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;
        vm
    }

    fn register(vm: &mut VirtualMachine, cls: std::sync::Arc<dyn NativeClass>) {
        let name = cls.full_name().to_string();
        vm.register_native_class(&name, cls);
    }

    /// 构造注册桥接链路相关 native 类的 VM，并确保全局环境已初始化
    fn make_asset_vm() -> (VirtualMachine, AudioAsset) {
        global::init_env_global();
        let aa = AudioAsset { name: String::new() };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(AudioAsset { name: String::new() }));
        register(&mut vm, std::sync::Arc::new(Environment {}));
        register(&mut vm, std::sync::Arc::new(NativeAudioAsset { name: String::new(), audio: 0 }));
        (vm, aa)
    }

    #[test]
    fn test_audio_asset_construct() {
        let (mut vm, aa) = make_asset_vm();
        let id = { let mut ctx = NativeContext::new(&mut vm); aa.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        // name 为空时 LoadAsset 直接返回 false
        { let mut ctx = NativeContext::new(&mut vm); aa.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return());
        // 未加载时 GetAsset 返回 0（null）
        { let mut ctx = NativeContext::new(&mut vm); aa.invoke_native_method(&mut ctx, id, 1); }
        assert_eq!(vm.param_pool.get_object_return(), 0);
    }

    #[test]
    fn test_audio_asset_load_success() {
        let (mut vm, aa) = make_asset_vm();
        // 登记平台音频句柄（唯一键避免并行测试冲突）
        global::with_env_global_mut(|env| {
            env.assets.insert("audio:test_p1_2_load".to_string(), 777);
        });
        let id = { let mut ctx = NativeContext::new(&mut vm); aa.do_construct_native(&mut ctx, None, 0) };
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_string_field(id, AudioAsset::FIELD_INDEX_name, "audio:test_p1_2_load".to_string());
        }

        { let mut ctx = NativeContext::new(&mut vm); aa.invoke_native_method(&mut ctx, id, 0); }
        assert!(vm.param_pool.get_bool_return(), "已注册的音频资产应加载成功");

        { let mut ctx = NativeContext::new(&mut vm); aa.invoke_native_method(&mut ctx, id, 1); }
        let audio_id = vm.param_pool.get_object_return();
        assert_ne!(audio_id, 0, "GetAsset 应返回 Audio 对象 ID");
        assert_eq!(vm.objects[&audio_id].class_name, "GorgeFramework.Audio");
        let vm_address = &mut vm as *mut VirtualMachine as usize;
        assert_eq!(
            global::resolve_audio_handle(vm_address, audio_id),
            777,
            "Audio 对象应解析回平台音频句柄"
        );
    }

    #[test]
    fn test_audio_asset_load_not_found() {
        let (mut vm, aa) = make_asset_vm();
        let id = { let mut ctx = NativeContext::new(&mut vm); aa.do_construct_native(&mut ctx, None, 0) };
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_string_field(id, AudioAsset::FIELD_INDEX_name, "audio:test_p1_2_missing".to_string());
        }
        { let mut ctx = NativeContext::new(&mut vm); aa.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return(), "未注册的资产应加载失败");
        { let mut ctx = NativeContext::new(&mut vm); aa.invoke_native_method(&mut ctx, id, 1); }
        assert_eq!(vm.param_pool.get_object_return(), 0);
    }

    #[test]
    fn test_audio_asset_load_non_audio_asset() {
        let (mut vm, aa) = make_asset_vm();
        // 无前缀资产被包装为裸 Asset，对齐 C# FromGorgeObject 强转失败返回 false
        global::with_env_global_mut(|env| {
            env.assets.insert("misc:test_p1_2_wrong_type".to_string(), 555);
        });
        let id = { let mut ctx = NativeContext::new(&mut vm); aa.do_construct_native(&mut ctx, None, 0) };
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_string_field(id, AudioAsset::FIELD_INDEX_name, "misc:test_p1_2_wrong_type".to_string());
        }
        { let mut ctx = NativeContext::new(&mut vm); aa.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return(), "非音频资产族应加载失败");
    }
}
