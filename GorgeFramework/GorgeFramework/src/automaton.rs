//! 自动化/状态机信号模块（对应 C# `Automaton/` 文件夹）。

/// 信号自动机接口（对应 C# `ISignalAutomaton.cs`）
///
/// 定义了信号驱动的自动机基础抽象。
pub trait ISignalAutomaton: Send + Sync {
    // 后续 S7 完成时添加方法
}
