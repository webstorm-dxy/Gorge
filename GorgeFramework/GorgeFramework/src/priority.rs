//! `GorgeFramework.Priority` —— 优先级值 native 类。
//!
//! 简单的 int 包装类型，用于调度优先级排序。

use gorge_core::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 优先级值，包装一个整数值
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Priority {
    #[gorge_field]
    pub value: i32,
}

#[gorge_native_impl]
impl Priority {
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, value: i32) {
        ctx.set_object_int_field(this, Self::FIELD_INDEX_value, value as i64);
    }
}
