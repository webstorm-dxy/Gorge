//! `GorgeFramework` — 派生元素指令（native 数据类）。
//!
//! 对齐 C# 参考实现 `DeriveElementCommand.cs`。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

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
