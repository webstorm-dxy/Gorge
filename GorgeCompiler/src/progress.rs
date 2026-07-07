#![allow(dead_code)]

/// 编译进度信息
#[derive(Debug, Clone)]
pub struct CompileProgress {
    pub current_step: usize,
    pub total_steps: usize,
    pub description: String,
}

/// 进度报告器 trait
pub trait ProgressReporter: Send {
    fn report(&self, progress: &CompileProgress);
}

/// 控制台进度报告器（输出到 stderr）
pub struct ConsoleReporter;

impl ProgressReporter for ConsoleReporter {
    fn report(&self, progress: &CompileProgress) {
        eprintln!("[{}/{}] {}", progress.current_step, progress.total_steps, progress.description);
    }
}

/// 静默进度报告器（不做任何输出）
pub struct SilentReporter;

impl ProgressReporter for SilentReporter {
    fn report(&self, _progress: &CompileProgress) {}
}
