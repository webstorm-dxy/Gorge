//! `GorgeFramework.AudioAsset` —— 音频资产 native 类。
//!
//! 移植自 C# 参考实现 `AudioAsset.cs`。
//! AudioAsset 通过 Environment.GetAssetByName 查找资产。

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
}
