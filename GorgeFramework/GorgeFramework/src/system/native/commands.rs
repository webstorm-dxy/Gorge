//! `GorgeFramework` — Automaton 指令（native 数据类）。
//!
//! 对齐 C# 参考实现。这些是纯数据命令类，用于自动机状态机的指令传递。

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

/// 派生元素指令
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct DeriveElementCommand {
    #[gorge_field]
    pub element_spec: i32,
}

impl DeriveElementCommand {
    pub fn new(element_spec: i32) -> Self {
        Self { element_spec }
    }
}

#[gorge_native_impl]
impl DeriveElementCommand {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, element_spec: i32) {
        ctx.set_object_int_field(this, DeriveElementCommand::FIELD_INDEX_element_spec, element_spec as i64);
    }
}

/// 销毁元素指令
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct DestroyElementCommand {
    #[gorge_field]
    pub target_type: i32,
}

impl DestroyElementCommand {
    pub fn new(target_type: i32) -> Self {
        Self { target_type }
    }
}

#[gorge_native_impl]
impl DestroyElementCommand {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, target_type: i32) {
        ctx.set_object_int_field(this, DestroyElementCommand::FIELD_INDEX_target_type, target_type as i64);
    }
}
