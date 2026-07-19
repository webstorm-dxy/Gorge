//! `GorgeFramework` — 原生音频资产（C# `NativeAudioAsset`，继承自 `AudioAsset`）。
//!
//! 移植自 C# 参考实现 `System/Native/NativeAudioAsset`。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 原生音频资产（C# `NativeAudioAsset`，继承自 `AudioAsset`）
///
/// 直接持有 Audio 对象引用，`LoadAsset` 返回 true。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct NativeAudioAsset {
    /// 资产名称（继承自 Asset）
    #[gorge_field]
    pub name: String,
    /// 音频对象 ID
    #[gorge_field]
    pub audio: usize,
}

#[gorge_native_impl]
impl NativeAudioAsset {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_string_field(this, NativeAudioAsset::FIELD_INDEX_name, String::new());
        ctx.set_object_object_field(this, NativeAudioAsset::FIELD_INDEX_audio, 0);
    }

    /// 0 号方法：加载资产（覆盖，值包装无需加载）
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        true
    }

    /// 1 号方法：获取音频资产（覆盖，返回 audio 字段）
    #[gorge_method]
    pub fn get_asset(ctx: &mut NativeContext, this: usize) -> usize {
        ctx.get_object_object_field(this, NativeAudioAsset::FIELD_INDEX_audio)
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
    fn test_native_audio_asset_get_asset() {
        let na = NativeAudioAsset { name: String::new(), audio: 0 };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(NativeAudioAsset { name: String::new(), audio: 0 }));
        let id = { let mut ctx = NativeContext::new(&mut vm); na.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = NativeContext::new(&mut vm); ctx.set_object_object_field(id, NativeAudioAsset::FIELD_INDEX_audio, 99); }
        { let mut ctx = NativeContext::new(&mut vm); na.invoke_native_method(&mut ctx, id, 1); }
        assert_eq!(vm.param_pool.get_object_return(), 99);
        // LoadAsset 返回 true
        { let mut ctx = NativeContext::new(&mut vm); na.invoke_native_method(&mut ctx, id, 0); }
        assert!(vm.param_pool.get_bool_return());
    }
}
