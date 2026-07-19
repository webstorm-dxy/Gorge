//! `GorgeFramework.Note` —— 音符 native 类（E-2 补齐 DoRespond）。
//!
//! 对齐 C# `System/Native/Note.cs`。Note 继承 Element，
//! 增加自动机（SignalTsiga）字段和 DoRespond 虚方法。
//!
//! # 方法编号表
//! | 编号 | 方法 | 说明 |
//! |------|------|------|
//! | 0 | do_respond | 返回自动机指令 ObjectArray（虚拟方法，子类覆盖） |

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 音符 native 类（继承 Element）
///
/// 继承 Element 的全部字段（通过 Gorge 类型系统），
/// 本结构体仅声明 Note 特有的字段。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Note {
    /// 关联自动机（SignalTsiga 对象 ID）
    #[gorge_field]
    pub automaton: usize,
}

#[gorge_native_impl]
impl Note {
    /// 响应判定（方法 0）
    ///
    /// 对齐 C# `Note.DoRespond`。虚方法，子类应覆盖以返回自动机指令。
    /// 基类默认返回空 ObjectArray（0）。
    /// 参数：respond_mode (String)、respond_chart_time (Float)。
    /// 返回：自动机指令 ObjectArray 的对象 ID（0=空）。
    #[gorge_method]
    pub fn do_respond(ctx: &mut NativeContext, this: usize, _respond_mode: String, _respond_chart_time: f32) -> usize {
        let _ = ctx;
        let _ = this;
        0
    }
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    struct Fixture {
        vm: VirtualMachine,
    }

    impl Fixture {
        fn new() -> Self { Self { vm: VirtualMachine::new() } }
        fn ctx(&mut self) -> NativeContext<'_> { NativeContext::new(&mut self.vm) }

        fn make_note(&mut self) -> usize {
            let note = Note { automaton: 0 };
            self.vm.param_pool.set_object_param(0, 0);
            let id = { let mut ctx = self.ctx(); note.do_construct_native(&mut ctx, None, 0) };
            self.vm.native_class_table.insert(
                "GorgeFramework.Note".to_string(),
                std::sync::Arc::new(Note { automaton: 0 }),
            );
            id
        }
    }

    #[test]
    fn test_note_construct() {
        let note = Note { automaton: 0 };
        let mut fx = Fixture::new();
        let id = {
            let mut ctx = fx.ctx();
            note.do_construct_native(&mut ctx, None, 0)
        };
        assert!(id > 0);
        // 验证对象存在
        assert!(fx.vm.objects.contains_key(&id));
    }

    #[test]
    fn test_note_do_respond_default() {
        let note = Note { automaton: 0 };
        let mut fx = Fixture::new();
        let note_id = fx.make_note();

        // do_respond（方法 0）
        fx.vm.param_pool.set_string_param(0, "tap".to_string());
        fx.vm.param_pool.set_float_param(0, 1.5);
        { let mut ctx = fx.ctx(); note.invoke_native_method(&mut ctx, note_id, 0); }
        // 基类默认返回 0（空 ObjectArray）
        assert_eq!(fx.vm.param_pool.get_object_return(), 0);
    }
}
