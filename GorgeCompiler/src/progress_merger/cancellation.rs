#![allow(dead_code)]
//! 取消令牌与编译异步错误类型

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// 协作式取消令牌
///
/// 编译线程在各检查点轮询 `is_cancelled()`，若为 true 则尽快返回
/// `CompileError::Cancelled`。主线程可随时调用 `cancel()` 设置标志。
#[derive(Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// 创建新的取消令牌（初始为未取消状态）
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 设置取消标志
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    /// 检查是否已取消
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

/// 带取消支持的编译结果错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    /// 编译被取消
    Cancelled,
    /// 编译过程中出现诊断错误
    CompilationFailed,
}
