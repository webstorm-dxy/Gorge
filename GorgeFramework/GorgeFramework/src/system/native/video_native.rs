//! `GorgeFramework.Video` —— 视频句柄 native 类。
//!
//! 移植自 C# 参考实现 `Video.cs`。
//! 无公开字段，平台视频句柄通过 payload 存储。

use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 视频句柄（C# `Video`）
///
/// 对应 C# 中由 `Base.Instance.CreateVideo()` 返回的 Video 对象。
/// 平台句柄通过 payload 存储。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct VideoNative {}

#[gorge_native_impl]
impl VideoNative {}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::native::NativeContext;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    #[test]
    fn test_video_native_construct() {
        let v = VideoNative {};
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;
        vm.register_native_class(v.full_name(), std::sync::Arc::new(VideoNative {}));
        let id = { let mut ctx = NativeContext::new(&mut vm); v.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
    }
}
