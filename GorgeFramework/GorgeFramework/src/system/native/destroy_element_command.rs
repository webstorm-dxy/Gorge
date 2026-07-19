//! `GorgeFramework` — 销毁元素指令（native 数据类）。
//!
//! 对齐 C# 参考实现 `DestroyElementCommand.cs`。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

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
