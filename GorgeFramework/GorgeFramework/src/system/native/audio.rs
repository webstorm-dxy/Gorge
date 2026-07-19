//! `GorgeFramework.Audio` —— 音频句柄 native 类。
//!
//! 移植自 C# 参考实现 `Audio.cs`。
//! 无公开字段，平台音频句柄通过 payload 存储。

use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 音频句柄（C# `Audio`）
///
/// 对应 C# 中由 `Base.Instance.CreateAudio()` 返回的 Audio 对象。
/// 平台播放器句柄通过 payload 存储。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct AudioNative {}

#[gorge_native_impl]
impl AudioNative {}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::native::NativeContext;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    #[test]
    fn test_audio_native_construct() {
        let a = AudioNative {};
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;
        vm.register_native_class(a.full_name(), std::sync::Arc::new(AudioNative {}));
        let id = { let mut ctx = NativeContext::new(&mut vm); a.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
    }
}
