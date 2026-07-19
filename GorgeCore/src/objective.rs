//! Objective 模块 —— 对象模型与运行时元数据（对应 C# `Objective/` 文件夹）。
//!
//! 包含类/对象/接口/委托的运行时表示、类型系统、类声明元数据、
//! 字节码序列化与 native 互操作接口。

pub mod bytecode;
pub mod class;
pub mod declaration;
pub mod delegate;
pub mod interface;
pub mod native;
pub mod object;
pub mod runtime;
pub mod types;
pub mod value_pool;
