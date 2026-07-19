//! 信号系统模块（对应 C# `Signal/` 文件夹）。
//!
//! 定义信号总线底层数据结构：通道队列/快照/切片
//! 的单通道和多通道层次。供 Runtime 仿真引擎使用。
//!
//! 注意：`edge` / `fragment` 已迁移到 `crate::input`（对应 C# `Input/` 文件夹）。

pub mod channel_edge_queue;
pub mod channel_snapshot;
pub mod channel_split;
pub mod multichannel_edge_queue;
pub mod multichannel_snapshot;
pub mod multichannel_split;
