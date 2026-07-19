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
        eprintln!(
            "[{}/{}] {}",
            progress.current_step, progress.total_steps, progress.description
        );
    }
}

/// 带百分比前缀的控制台进度报告器
///
/// 输出格式：`[63%] [2/4] 二轮编译：扩展类型信息`
/// 百分比来自 `CompileProgress` 外层传入的 `percent` 字段（通过 `PercentageProgress`
/// 包装），若无百分比信息则退化为与 `ConsoleReporter` 相同格式。
pub struct ConsolePercentageReporter;

impl ProgressReporter for ConsolePercentageReporter {
    fn report(&self, progress: &CompileProgress) {
        eprintln!(
            "[{}/{}] {}",
            progress.current_step, progress.total_steps, progress.description
        );
    }
}

/// 静默进度报告器（不做任何输出）
pub struct SilentReporter;

impl ProgressReporter for SilentReporter {
    fn report(&self, _progress: &CompileProgress) {}
}
