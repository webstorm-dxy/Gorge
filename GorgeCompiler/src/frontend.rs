//! Frontend 模块 —— 编译器前端（对应 C# `AntlrGen/` 的手写替代实现）。
//!
//! 包含词法分析器、语法分析器与抽象语法树定义。

pub mod ast;
pub mod lexer;
pub mod parser;
