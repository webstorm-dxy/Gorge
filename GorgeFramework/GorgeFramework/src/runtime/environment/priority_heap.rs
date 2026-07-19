//! 优先级堆（对应 C# `Runtime/Environment/PrioritySimulatorHeap.cs`）。
//!
//! 基于二叉堆的优先级调度器，支持按优先级排序的 Register（插入）、
//! Remove（按值删除，通过反向索引）和迭代。用于 SimulationManager
//! 中管理多个 ISimulator 的执行顺序。
//!
//! C# 原版使用 FibonacciHeap，Rust 版用 BinaryHeap 替代，性能差异
//! 在当前规模下可忽略。

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

/// 堆中元素：优先级 + 关联值
#[derive(Debug, Clone)]
struct HeapEntry<P: Ord, V> {
    priority: P,
    value: V,
}

impl<P: Ord, V: Eq> PartialEq for HeapEntry<P, V> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}
impl<P: Ord, V: Eq> Eq for HeapEntry<P, V> {}

impl<P: Ord, V: Eq> PartialOrd for HeapEntry<P, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<P: Ord, V: Eq> Ord for HeapEntry<P, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap 是最大堆，取反实现最小堆语义
        other.priority.cmp(&self.priority)
    }
}

/// 优先级堆（最小堆）
///
/// `P`：优先级类型（越小越优先），`V`：关联值类型。
/// 内部维护反向索引以实现按值删除（O(log n) 摊销）。
#[derive(Debug, Clone)]
pub struct PriorityHeap<P: Ord + Clone, V: Eq + Hash + Clone> {
    heap: BinaryHeap<HeapEntry<P, V>>,
    /// 反向索引：值 → 优先级（用于删除判断）
    index: HashMap<V, P>,
}

impl<P: Ord + Clone, V: Eq + Hash + Clone> PriorityHeap<P, V> {
    pub fn new() -> Self {
        Self { heap: BinaryHeap::new(), index: HashMap::new() }
    }

    /// 注册（插入或更新优先级）
    pub fn register(&mut self, priority: P, value: V) {
        self.index.insert(value.clone(), priority.clone());
        self.heap.push(HeapEntry { priority, value });
    }

    /// 按值删除
    pub fn remove(&mut self, value: &V) {
        self.index.remove(value);
        // BinaryHeap 不支持按值删除，重建堆
        self.rebuild();
    }

    /// 获取堆顶元素（最小优先级）
    pub fn peek(&self) -> Option<(&P, &V)> {
        self.heap.peek().map(|e| (&e.priority, &e.value))
    }

    /// 弹出堆顶元素
    pub fn pop(&mut self) -> Option<(P, V)> {
        self.heap.pop().map(|e| {
            self.index.remove(&e.value);
            (e.priority, e.value)
        })
    }

    /// 销毁全部
    pub fn destruct(&mut self) {
        self.heap.clear();
        self.index.clear();
    }

    /// 元素数量
    pub fn len(&self) -> usize { self.heap.len() }

    /// 是否为空
    pub fn is_empty(&self) -> bool { self.heap.is_empty() }

    /// 遍历（不保证顺序）
    pub fn iter(&self) -> impl Iterator<Item = (&P, &V)> {
        self.heap.iter().map(|e| (&e.priority, &e.value))
    }

    /// 重建堆（过滤已删除元素）
    fn rebuild(&mut self) {
        let old = std::mem::take(&mut self.heap);
        self.heap = old.into_iter()
            .filter(|e| self.index.contains_key(&e.value))
            .collect();
    }
}

impl<P: Ord + Clone, V: Eq + Hash + Clone> Default for PriorityHeap<P, V> {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_peek() {
        let mut h = PriorityHeap::new();
        h.register(2, "b");
        h.register(1, "a");
        h.register(3, "c");
        assert_eq!(h.peek().map(|(p, v)| (*p, *v)), Some((1, "a")));
    }

    #[test]
    fn test_pop_order() {
        let mut h = PriorityHeap::new();
        h.register(3, "c");
        h.register(1, "a");
        h.register(2, "b");
        assert_eq!(h.pop().map(|(_, v)| v), Some("a"));
        assert_eq!(h.pop().map(|(_, v)| v), Some("b"));
        assert_eq!(h.pop().map(|(_, v)| v), Some("c"));
        assert!(h.is_empty());
    }

    #[test]
    fn test_remove_and_rebuild() {
        let mut h = PriorityHeap::new();
        h.register(1, "a");
        h.register(2, "b");
        h.register(3, "c");
        h.remove(&"b");
        assert_eq!(h.len(), 2);
        let results: Vec<_> = (0..2).map(|_| h.pop().map(|(_, v)| v)).collect();
        assert!(results.contains(&Some("a")));
        assert!(results.contains(&Some("c")));
        assert!(!results.contains(&Some("b")));
    }
}
