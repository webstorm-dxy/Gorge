use std::fmt;

/// 诊断信息的严重级别。
///
/// 用于区分编译/分析过程中产生的不同严重程度的消息，
/// 决定诊断信息在输出时的格式与着色。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    /// 错误级别：表示代码中存在无法继续处理的问题。
    Error,
    /// 警告级别：表示代码可能有潜在问题，但不阻止继续处理。
    Warning,
    /// 提示级别：用于提供补充说明或建议，不表示代码有问题。
    Note,
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticLevel::Error => write!(f, "error"),
            DiagnosticLevel::Warning => write!(f, "warning"),
            DiagnosticLevel::Note => write!(f, "note"),
        }
    }
}

/// 源代码中的位置区间。
///
/// 描述一段源代码在输入中的精确位置，包括字节偏移量和行列信息。
/// `source_id` 用于支持多文件场景下区分不同源文件。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// 区间起始位置的字节偏移量。
    pub start: usize,
    /// 区间结束位置的字节偏移量（不包含）。
    pub end: usize,
    /// 区间起始位置所在的行号（从 1 开始）。
    pub line: usize,
    /// 区间起始位置所在的列号（从 1 开始）。
    pub column: usize,
    /// 源文件标识符，用于在多文件场景中定位具体文件。
    pub source_id: usize,
}

impl Span {
    /// 创建一个新的位置区间。
    ///
    /// # 参数
    ///
    /// * `start` - 字节起始偏移量
    /// * `end` - 字节结束偏移量
    /// * `line` - 起始行号（1-based）
    /// * `column` - 起始列号（1-based）
    /// * `source_id` - 源文件标识符
    pub fn new(start: usize, end: usize, line: usize, column: usize, source_id: usize) -> Self {
        Self { start, end, line, column, source_id }
    }

    /// 创建一个占位的空区间，所有字段均为 0。
    ///
    /// 用于在不需要精确位置信息时（例如测试或语法树的虚拟节点）作为默认值。
    pub fn dummy() -> Self {
        Self { start: 0, end: 0, line: 0, column: 0, source_id: 0 }
    }
}

/// 单条诊断信息。
///
/// 包含一条编译/分析消息的完整信息：严重级别、消息文本、
/// 在源代码中的位置区间，以及可选的修复提示。
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// 诊断信息的严重级别。
    pub level: DiagnosticLevel,
    /// 诊断信息的主文本描述。
    pub message: String,
    /// 诊断信息对应的源代码位置区间。
    pub span: Span,
    /// 可选的修复/补充提示，提供解决问题的建议。
    pub hint: Option<String>,
}

impl Diagnostic {
    /// 创建一条错误级别的诊断信息。
    ///
    /// `message` 接受任何可转换为 `String` 的类型，方便调用方直接传入 `&str` 或 `String`。
    pub fn error(span: Span, message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message: message.into(),
            span,
            hint: None,
        }
    }

    /// 创建一条警告级别的诊断信息。
    pub fn warning(span: Span, message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            message: message.into(),
            span,
            hint: None,
        }
    }

    /// 创建一条提示级别的诊断信息。
    pub fn note(span: Span, message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Note,
            message: message.into(),
            span,
            hint: None,
        }
    }

    /// 为此诊断信息附加一条修复提示。
    ///
    /// 采用构建器模式，返回 `Self` 以支持链式调用。
    /// 提示通常用于向用户建议如何修复该问题。
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

/// 诊断信息集合。
///
/// 管理编译/分析过程中产生的所有诊断信息，提供统计、遍历和渲染功能。
/// 使用 `Default` trait 即可获得空集合，也可用 `new()` 显式创建。
#[derive(Debug, Default, Clone)]
pub struct Diagnostics {
    entries: Vec<Diagnostic>,
}

impl Diagnostics {
    /// 创建一个空的诊断信息集合。
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// 产生一条错误级别的诊断信息并加入集合。
    ///
    /// 这是 `emit(Diagnostic::error(...))` 的便捷方法，
    /// 避免调用方手动构造 `Diagnostic` 实例。
    pub fn emit_error(&mut self, span: Span, message: impl Into<String>) {
        self.entries.push(Diagnostic::error(span, message));
    }

    /// 产生一条警告级别的诊断信息并加入集合。
    pub fn emit_warning(&mut self, span: Span, message: impl Into<String>) {
        self.entries.push(Diagnostic::warning(span, message));
    }

    /// 产生一条提示级别的诊断信息并加入集合。
    pub fn emit_note(&mut self, span: Span, message: impl Into<String>) {
        self.entries.push(Diagnostic::note(span, message));
    }

    /// 将已构造的 `Diagnostic` 实例直接加入集合。
    ///
    /// 用于需要手动指定完整诊断信息（如带有 hint）的场景。
    pub fn emit(&mut self, diagnostic: Diagnostic) {
        self.entries.push(diagnostic);
    }

    /// 检查集合中是否包含任意错误级别的诊断信息。
    ///
    /// 通常用于编译流程中决定是否应该中止后续处理。
    pub fn has_errors(&self) -> bool {
        self.entries.iter().any(|d| d.level == DiagnosticLevel::Error)
    }

    /// 返回错误级别诊断信息的数量。
    pub fn error_count(&self) -> usize {
        self.entries.iter().filter(|d| d.level == DiagnosticLevel::Error).count()
    }

    /// 返回警告级别诊断信息的数量。
    pub fn warning_count(&self) -> usize {
        self.entries.iter().filter(|d| d.level == DiagnosticLevel::Warning).count()
    }

    /// 以不可变引用的方式遍历所有诊断信息。
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.entries.iter()
    }

    /// 以所有权转移的方式遍历所有诊断信息，消费集合自身。
    pub fn into_iter(self) -> impl Iterator<Item = Diagnostic> {
        self.entries.into_iter()
    }

    /// 将所有诊断信息渲染为人类可读的文本格式。
    ///
    /// # 参数
    ///
    /// * `sources` - 源文件内容的切片数组，索引对应 `Span.source_id`。
    ///   传入空数组或缺少对应索引时，相关诊断信息会标记为 `<unknown>`。
    ///
    /// # 返回
    ///
    /// 返回包含所有诊断信息的格式化字符串，每条诊断信息之间用空行分隔。
    /// 格式仿照 Rust 编译器的错误输出风格，包含级别、消息、位置标记和下划线高亮。
    pub fn render(&self, sources: &[&str]) -> String {
        let mut output = String::new();
        for diagnostic in &self.entries {
            // 在非首条诊断信息前插入空行，提高可读性
            if !output.is_empty() {
                output.push('\n');
            }
            output.push_str(&Self::render_diagnostic(diagnostic, sources));
        }
        output
    }

    /// 渲染单条诊断信息。
    ///
    /// 输出格式示例：
    /// ```text
    /// error: unexpected token
    ///   --> <source>:3:15
    ///    |
    ///  3 |     let x = ;
    ///    |               ^
    ///    |               error: expected expression
    /// ```
    fn render_diagnostic(diagnostic: &Diagnostic, sources: &[&str]) -> String {
        use std::fmt::Write;

        let mut out = String::new();
        let span = &diagnostic.span;

        // 根据 source_id 获取对应的源代码文本，不存在时回退为 "<unknown>"
        let source = sources.get(span.source_id).copied().unwrap_or("<unknown>");

        // 输出第一行：级别和消息文本
        write!(
            out,
            "{}: {}\n",
            diagnostic.level, diagnostic.message
        )
        .unwrap();

        // 输出第二行：位置指示符
        write!(
            out,
            "  --> <source>:{}:{}\n",
            span.line, span.column
        )
        .unwrap();

        out.push_str("   |\n");

        // 获取对应行的内容，行号从 1 开始所以需要减 1
        let line_content = source.lines().nth(span.line.saturating_sub(1)).unwrap_or("");
        write!(out, "{:>2} | {}\n", span.line, line_content).unwrap();

        // 对齐到错误列，在正确位置输出下划线标记
        write!(out, "   | {:>width$}", "", width = span.column.saturating_sub(1)).unwrap();

        // 下划线长度至少为 1，即使是单字符错误也能看到标记
        let underline_len = (span.end - span.start).max(1);
        for _ in 0..underline_len {
            out.push('^');
        }
        out.push('\n');

        // 如果有提示信息，在下一行输出
        if let Some(ref hint) = diagnostic.hint {
            write!(
                out,
                "   | {:>width$} {}: {}",
                "",
                diagnostic.level,
                hint,
                width = span.column.saturating_sub(1)
            )
            .unwrap();
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostics_basic() {
        let mut diags = Diagnostics::new();
        assert!(!diags.has_errors());

        diags.emit_error(Span::dummy(), "unexpected token");
        assert!(diags.has_errors());
        assert_eq!(diags.error_count(), 1);
        assert_eq!(diags.warning_count(), 0);
    }

    #[test]
    fn test_diagnostic_with_hint() {
        let d = Diagnostic::error(Span::dummy(), "type mismatch")
            .with_hint("expected int, found string");
        assert!(d.hint.is_some());
    }

    #[test]
    fn test_render() {
        let mut diags = Diagnostics::new();
        diags.emit_error(
            Span::new(10, 11, 3, 15, 0),
            "unexpected token",
        );

        let source = "fn main() {\n    let x = ;\n}\n";
        let rendered = diags.render(&[source]);
        println!("{}", rendered);
        assert!(rendered.contains("error: unexpected token"));
        assert!(rendered.contains("-->"));
        assert!(rendered.contains("^"));
    }
}
