//! `GorgeFramework` — 追加信号指令（native 数据类）。
//!
//! 对齐 C# 参考实现 `AppendSignalCommand.cs`。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 追加信号指令
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct AppendSignalCommand {
    #[gorge_field]
    pub signal_id: i32,
    #[gorge_field]
    pub priority: i32,
}

impl AppendSignalCommand {
    pub fn new(signal_id: i32, priority: i32) -> Self {
        Self { signal_id, priority }
    }
}

#[gorge_native_impl]
impl AppendSignalCommand {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, signal_id: i32, priority: i32) {
        ctx.set_object_int_field(this, AppendSignalCommand::FIELD_INDEX_signal_id, signal_id as i64);
        ctx.set_object_int_field(this, AppendSignalCommand::FIELD_INDEX_priority, priority as i64);
    }
}
