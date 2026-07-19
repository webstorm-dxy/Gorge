//! Optimizer 模块 —— IR 优化器（对应 C# `Optimizer/` 文件夹）。
//!
//! 实现基本块划分、控制流图构建、死代码消除、公共子表达式消除与连跳优化。

pub mod optimizer;
