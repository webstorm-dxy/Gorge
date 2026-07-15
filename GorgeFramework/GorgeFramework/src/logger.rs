//! `GorgeFramework.Logger` — 日志输出工具（native 纯静态类）。
//!
//! 对齐 C# `Logger` 类，提供调试/日志输出的静态方法。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::native::NativeContext;

/// 日志工具类（纯静态，无字段）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Logger {}

#[gorge_native_impl]
impl Logger {
    /// 输出整数日志（debug 级别）
    #[gorge_static]
    pub fn log_int(_ctx: &mut NativeContext, value: i32) -> i32 {
        eprintln!("[Gorge.Log] {}", value);
        value
    }

    /// 输出浮点日志
    #[gorge_static]
    pub fn log_float(_ctx: &mut NativeContext, value: f32) -> f32 {
        eprintln!("[Gorge.Log] {}", value);
        value
    }

    /// 输出字符串日志
    #[gorge_static]
    pub fn log_string(_ctx: &mut NativeContext, value: String) -> String {
        eprintln!("[Gorge.Log] {}", value);
        value
    }
}
