//! `StackExtension` —— 栈安全访问扩展。
//!
//! 移植自 C# 参考实现 `Utilities/StackExtension.cs`。
//!
//! C# 的 `Top<T>(this Stack<T>)` 用 `stack.Count > 0 ? stack.Peek() : default` 提供安全栈顶访问。
//! Rust 的 `Vec::last()` 已天然返回 `Option<&T>`，无需额外封装。
//! 此处仅提供文档与单元测试，供对照参考。

/// 获取栈顶元素的安全包装（对应 C# `StackExtension.Top<T>`）
///
/// Rust 的 `Vec::last()` 天然返回 `Option<&T>`，已在类型层面保证安全。
/// 本函数仅为完整对照 C# 语义提供：无栈顶时返回 `None`（对应 C# `default`）。
pub fn top<T>(stack: &[T]) -> Option<&T> {
    stack.last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_non_empty() {
        let s = vec![1, 2, 3];
        assert_eq!(top(&s), Some(&3));
    }

    #[test]
    fn test_top_empty() {
        let s: Vec<i32> = vec![];
        assert_eq!(top(&s), None);
    }
}
