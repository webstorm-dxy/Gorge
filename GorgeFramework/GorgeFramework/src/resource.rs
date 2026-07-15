//! `GorgeFramework` — 资源/资产标记类型（native 类注册）。
//!
//! 移植自 C# 参考实现 `System/Native/` 中的 Asset/Graph/Audio/Video 等类型。
//! 这些是纯数据标记类型，在纯 Rust 实现中不绑定具体的渲染/音频引擎，
//! 仅作为数据容器供上层使用。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::native::NativeContext;

// ==================== Graph ====================

/// 图形/纹理资源
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Graph {
    #[gorge_field]
    pub width: i32,
    #[gorge_field]
    pub height: i32,
}

impl Graph {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

#[gorge_native_impl]
impl Graph {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, width: i32, height: i32) {
        ctx.set_object_int_field(this, Graph::FIELD_INDEX_width, width as i64);
        ctx.set_object_int_field(this, Graph::FIELD_INDEX_height, height as i64);
    }
}

// ==================== Audio ====================

/// 音频资源（纯标记类型，暂无 Gorge native 注册）
#[derive(Debug, Clone)]
pub struct Audio {}

impl Audio {
    pub fn new() -> Self { Self {} }
}

// ==================== Video ====================

/// 视频资源（纯标记类型）
#[derive(Debug, Clone)]
pub struct Video {}

impl Video {
    pub fn new() -> Self { Self {} }
}

// ==================== Asset ====================

/// 资源基类（纯标记类型）
#[derive(Debug, Clone)]
pub struct Asset {}

impl Asset {
    pub fn new() -> Self { Self {} }
}
