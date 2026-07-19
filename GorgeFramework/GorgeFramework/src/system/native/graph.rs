//! `GorgeFramework` — 图形/纹理资源（C# `Graph`）。
//!
//! 移植自 C# 参考实现 `System/Native/Graph`。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 图形/纹理资源（C# `Graph`）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Graph {
    #[gorge_field]
    pub width: i32,
    #[gorge_field]
    pub height: i32,
}

#[gorge_native_impl]
impl Graph {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, width: i32, height: i32) {
        ctx.set_object_int_field(this, Graph::FIELD_INDEX_width, width as i64);
        ctx.set_object_int_field(this, Graph::FIELD_INDEX_height, height as i64);
    }
}
