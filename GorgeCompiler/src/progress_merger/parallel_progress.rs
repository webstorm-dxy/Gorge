//! 加权并行进度合并器（对齐 C# `ParallelProgressMerger`）
//!
//! 多个子任务各有权重，子进度按权重合成总进度：
//! `总进度 = Σ(子进度 × 权重) / Σ权重`
//!
//! 通过 `Arc<Mutex<...>>` 保证线程安全。

use std::sync::{Arc, Mutex};

/// 加权并行进度合并器
///
/// 注册多个子进度后，任一子进度更新会触发总进度回调。
pub struct WeightedProgressMerger {
    inner: Arc<Mutex<MergerState>>,
}

struct MergerState {
    children: Vec<ChildState>,
    on_progress: Option<Box<dyn FnMut(f32) + Send>>,
}

struct ChildState {
    weight: f32,
    progress: f32,
}

impl WeightedProgressMerger {
    /// 创建新的进度合并器
    ///
    /// `on_progress` 接收 0.0~1.0 的总进度回调。
    pub fn new(on_progress: Box<dyn FnMut(f32) + Send>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(MergerState {
                children: Vec::new(),
                on_progress: Some(on_progress),
            })),
        }
    }

    /// 注册一个子进度并返回其句柄
    ///
    /// `weight` 必须大于 0，表示该子任务在总进度中的相对权重。
    pub fn register(&self, weight: f32) -> ChildProgress {
        assert!(weight > 0.0, "权重必须大于 0");
        let mut state = self.inner.lock().unwrap();
        let index = state.children.len();
        state.children.push(ChildState {
            weight,
            progress: 0.0,
        });
        ChildProgress {
            inner: Arc::clone(&self.inner),
            index,
        }
    }

    /// 内部方法：子进度更新时调用，重算总进度并回调
    fn recompute_total(state: &mut MergerState) {
        let total_weight: f32 = state.children.iter().map(|c| c.weight).sum();
        if total_weight <= 0.0 {
            return;
        }
        let weighted_sum: f32 = state
            .children
            .iter()
            .map(|c| c.progress * c.weight)
            .sum();
        let overall = (weighted_sum / total_weight).clamp(0.0, 1.0);
        if let Some(ref mut callback) = state.on_progress {
            callback(overall);
        }
    }
}

/// 子进度句柄
///
/// 调用 `report(0.0~1.0)` 更新该子进度的完成比例，
/// 触发合并器重新计算总进度。
pub struct ChildProgress {
    inner: Arc<Mutex<MergerState>>,
    index: usize,
}

impl ChildProgress {
    /// 上报当前子进度的完成比例（0.0~1.0）
    pub fn report(&self, progress: f32) {
        let mut state = self.inner.lock().unwrap();
        if let Some(child) = state.children.get_mut(self.index) {
            child.progress = progress.clamp(0.0, 1.0);
        }
        WeightedProgressMerger::recompute_total(&mut state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_merger_basic() {
        let last = Arc::new(Mutex::new(0.0f32));
        let last_clone = Arc::clone(&last);
        let merger = WeightedProgressMerger::new(Box::new(move |p| {
            *last_clone.lock().unwrap() = p;
        }));

        let c1 = merger.register(1.0);
        let c2 = merger.register(2.0);
        let c3 = merger.register(3.0);

        // 子进度 2 完成一半：(0.5*2) / 6 = 1/6 ≈ 0.1667
        c2.report(0.5);
        let v = *last.lock().unwrap();
        assert!((v - 1.0 / 6.0).abs() < 0.01, "期望 ~0.1667, 实际 {}", v);

        // 子进度 1 完成：(1.0*1 + 0.5*2) / 6 = 2/6 = 0.333
        c1.report(1.0);
        let v = *last.lock().unwrap();
        assert!((v - 2.0 / 6.0).abs() < 0.01, "期望 ~0.333, 实际 {}", v);

        // 全部完成
        c2.report(1.0);
        c3.report(1.0);
        let v = *last.lock().unwrap();
        assert!((v - 1.0).abs() < 0.01, "期望 1.0, 实际 {}", v);
    }

    #[test]
    fn test_weighted_merger_non_normalized_weights() {
        let last = Arc::new(Mutex::new(0.0f32));
        let last_clone = Arc::clone(&last);
        let merger = WeightedProgressMerger::new(Box::new(move |p| {
            *last_clone.lock().unwrap() = p;
        }));

        // 与 C# 相同的权重分配：5 个子任务各 0.1，总权重 0.5
        let c1 = merger.register(0.1);
        let c2 = merger.register(0.1);
        let c3 = merger.register(0.1);
        let c4 = merger.register(0.1);
        let c5 = merger.register(0.1);

        // 前 2 个完成：0.2 / 0.5 = 0.4
        c1.report(1.0);
        c2.report(1.0);
        let v = *last.lock().unwrap();
        assert!((v - 0.4).abs() < 0.01, "期望 0.4, 实际 {}", v);

        // 全部完成
        c3.report(1.0);
        c4.report(1.0);
        c5.report(1.0);
        let v = *last.lock().unwrap();
        assert!((v - 1.0).abs() < 0.01, "期望 1.0, 实际 {}", v);
    }

    #[test]
    fn test_weighted_merger_empty_no_panic() {
        // 空注册时不应 panic，只是不触发回调
        let merger = WeightedProgressMerger::new(Box::new(|_p| {}));
        // 不做任何 register，验证结构本身可正常创建和 drop
        drop(merger);
    }

    #[test]
    fn test_progress_monotonic() {
        // 收集所有进度回调值，验证非递减且终值为 1.0
        let history = Arc::new(Mutex::new(Vec::<f32>::new()));
        let history_clone = Arc::clone(&history);
        let merger = WeightedProgressMerger::new(Box::new(move |p| {
            history_clone.lock().unwrap().push(p);
        }));

        let c1 = merger.register(1.0);
        let c2 = merger.register(1.0);

        c1.report(0.3);
        c1.report(0.7);
        c2.report(0.5);
        c1.report(1.0);
        c2.report(1.0);

        let values = history.lock().unwrap().clone();
        assert!(!values.is_empty(), "应有进度回调");
        // 验证非递减
        for w in values.windows(2) {
            assert!(w[0] <= w[1] + 0.001, "进度应非递减: {} -> {}", w[0], w[1]);
        }
        // 验证终值 ≈ 1.0
        let last = values.last().copied().unwrap();
        assert!((last - 1.0).abs() < 0.01, "终值应为 1.0, 实际 {}", last);
    }
}
