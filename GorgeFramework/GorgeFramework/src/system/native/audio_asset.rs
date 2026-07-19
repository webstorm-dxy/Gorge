//! `GorgeFramework.AudioAsset` / `WavAudioAsset` —— 音频资产 native 类。
//!
//! 移植自 C# 参考实现 `AudioAsset.cs` / `WavAudioAsset.cs`。
//! AudioAsset 通过 Environment.GetAssetByName 查找资产；
//! WavAudioAsset 通过 PlatformBase.create_audio 创建音频句柄。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

// ==================== AudioAsset ====================

/// 音频资产（C# `AudioAsset`，继承自 `Asset`）
///
/// `LoadAsset` 通过 `Environment.GetAssetByName(name)` 查找 WavAudioAsset，
/// 取其 `GetAsset` 结果作为音频对象。
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
    /// 通过 Environment.GetAssetByName 查找同名资产并获取其音频对象。
    #[gorge_method]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        let _name = ctx.get_object_string_field(this, AudioAsset::FIELD_INDEX_name);
        // 当前为骨架：查找 AssetManager，暂无则返回 false
        // S7 将接入完整的 Runtime.asset 注册表
        false
    }

    /// 1 号方法：获取音频资产
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn get_asset(ctx: &mut NativeContext, this: usize) -> usize {
        0
    }
}

// ==================== WavAudioAsset ====================

/// WAV 音频资产（C# `WavAudioAsset`，继承自 `AudioAsset`）
///
/// `LoadAsset` 调用 `PlatformBase.create_audio(wavFilePath)`，
/// headless 实现只记录路径不解码。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct WavAudioAsset {
    /// 资产名称（继承自 Asset）
    #[gorge_field]
    pub name: String,
    /// WAV 文件路径
    #[gorge_field]
    pub wav_file_path: String,
}

#[gorge_native_impl]
impl WavAudioAsset {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_string_field(this, WavAudioAsset::FIELD_INDEX_name, String::new());
        ctx.set_object_string_field(this, WavAudioAsset::FIELD_INDEX_wav_file_path, String::new());
    }

    /// 0 号方法：加载资产（覆盖 AudioAsset.LoadAsset）
    ///
    /// 调用 PlatformBase.create_audio 创建音频句柄，存入 payload。
    #[gorge_method]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        let path = ctx.get_object_string_field(this, WavAudioAsset::FIELD_INDEX_wav_file_path);
        if path.is_empty() {
            return false;
        }

        use crate::adaptor::{platform, platform_installed};
        if !platform_installed() {
            return false;
        }

        let handle = platform().create_audio(&path);
        // 将句柄存入 payload（后续 GetAsset 返回）
        ctx.insert_payload(this, Box::new(handle));
        true
    }

    /// 1 号方法：获取音频资产（覆盖 AudioAsset.GetAsset）
    #[gorge_method]
    pub fn get_asset(ctx: &mut NativeContext, this: usize) -> usize {
        ctx.get_payload::<usize>(this).copied().unwrap_or(0)
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

    fn register(vm: &mut VirtualMachine, cls: std::sync::Arc<dyn NativeClass>) {
        let name = cls.full_name().to_string();
        vm.register_native_class(&name, cls);
    }

    #[test]
    fn test_audio_asset_construct() {
        let aa = AudioAsset { name: String::new() };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(AudioAsset { name: String::new() }));
        let id = { let mut ctx = NativeContext::new(&mut vm); aa.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        // LoadAsset 骨架目前返回 false（无 AssetManager）
        { let mut ctx = NativeContext::new(&mut vm); aa.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return());
    }

    #[test]
    fn test_wav_audio_asset_load_creates_audio() {
        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));

        let wa = WavAudioAsset { name: String::new(), wav_file_path: String::new() };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(WavAudioAsset { name: String::new(), wav_file_path: String::new() }));
        let id = { let mut ctx = NativeContext::new(&mut vm); wa.do_construct_native(&mut ctx, None, 0) };

        // 设置 wav 文件路径
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_string_field(id, WavAudioAsset::FIELD_INDEX_wav_file_path, "audio/test.wav".to_string());
        }

        // LoadAsset 调用 platform.create_audio("audio/test.wav")
        {
            let mut ctx = NativeContext::new(&mut vm);
            wa.invoke_native_method(&mut ctx, id, 0);
        }
        assert!(vm.param_pool.get_bool_return());

        // 验证 headless 日志 + payload 句柄
        let _p = crate::adaptor::platform();
        {
            let ctx = NativeContext::new(&mut vm);
            let handle: Option<&usize> = ctx.get_payload::<usize>(id);
            assert!(handle.is_some());
            assert!(*handle.unwrap() > 0); // 音频句柄应为正数
        }
    }

    #[test]
    fn test_wav_audio_asset_empty_path_returns_false() {
        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));

        let wa = WavAudioAsset { name: String::new(), wav_file_path: String::new() };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(WavAudioAsset { name: String::new(), wav_file_path: String::new() }));
        let id = { let mut ctx = NativeContext::new(&mut vm); wa.do_construct_native(&mut ctx, None, 0) };

        // 空路径 → LoadAsset 返回 false
        { let mut ctx = NativeContext::new(&mut vm); wa.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return());
    }

    #[test]
    fn test_wav_audio_asset_get_asset_returns_handle() {
        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));

        let wa = WavAudioAsset { name: String::new(), wav_file_path: String::new() };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(WavAudioAsset { name: String::new(), wav_file_path: String::new() }));
        let id = { let mut ctx = NativeContext::new(&mut vm); wa.do_construct_native(&mut ctx, None, 0) };

        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.set_object_string_field(id, WavAudioAsset::FIELD_INDEX_wav_file_path, "test.wav".to_string());
        }
        { let mut ctx = NativeContext::new(&mut vm); wa.invoke_native_method(&mut ctx, id, 0); }
        { let mut ctx = NativeContext::new(&mut vm); wa.invoke_native_method(&mut ctx, id, 1); }
        // 平台是全局共享单例，句柄绝对值取决于测试执行顺序，只断言拿到了非零句柄
        assert!(vm.param_pool.get_object_return() >= 1, "GetAsset 应返回非零音频句柄");
    }
}
