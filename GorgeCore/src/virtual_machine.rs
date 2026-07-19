//! VirtualMachine 模块 —— 虚拟机执行核心（对应 C# `VirtualMachine/` 文件夹）。
//!
//! 包含中间代码定义、类型分离栈虚拟机与调用参数池。

pub mod ir;
pub mod param_pool;
pub mod vm;
