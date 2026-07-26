//! `GorgeFramework.Element` —— 游戏元素 native 类。
//!
//! 对齐 C# `System/Native/Element.cs`。游戏元素是谱面的基本单位，
//! 包含关联的场景节点（ObjectArray）、派生元素（ObjectArray）和
//! 模拟器引用。

use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 游戏元素 native 类
///
/// 字段中的 ObjectArray / ISimulator 均以对象 ID（usize）存储。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Element {
    /// 关联模拟器（ISimulator 对象 ID）
    #[gorge_field]
    pub simulator: usize,
    /// 尾独立模拟器（ISimulator 对象 ID）
    #[gorge_field]
    pub late_independent_simulator: usize,
    /// 关联的图形节点（ObjectArray 对象 ID，0=无）
    #[gorge_field]
    pub nodes: usize,
    /// 派生元素列表（ObjectArray 对象 ID）
    #[gorge_field]
    pub derived_elements: usize,
}

#[gorge_native_impl]
impl Element {}
