//! 信号片段（对应 C# `Input/Fragment.cs`）。
//!
//! 描述某条信号在一段时间区间内（左闭右闭）的变化过程，
//! 包含起始值和一个按时间升序的边沿列表。
//!
//! 支持 AppendEdges（追加边沿）、Split（时间切片）和
//! Sample / SampleBeforeEdge（时点采样）等操作。

use super::edge::Edge;

/// 信号片段：时间区间 [start_time, end_time] 内的信号变化
///
/// `TSignal` 为信号值类型，框架使用 `usize`。
#[derive(Debug, Clone, PartialEq)]
pub struct Fragment<TSignal: Clone + PartialEq> {
    /// 信号编号
    pub signal_id: i32,
    /// 片段起始时刻（模拟时间）
    pub start_time: f32,
    /// 片段结束时刻（模拟时间）
    pub end_time: f32,
    /// 起始信号值
    pub start_value: TSignal,
    /// 片段内的信号边沿列表（按时间升序）
    pub edges: Vec<Edge<TSignal>>,
}

impl<TSignal: Clone + PartialEq> Fragment<TSignal> {
    /// 创建空片段
    pub fn new(signal_id: i32, start_time: f32, end_time: f32, start_value: TSignal) -> Self {
        Self { signal_id, start_time, end_time, start_value, edges: Vec::new() }
    }

    /// 返回最新信号值（无边沿时返回起始值）
    pub fn latest_value(&self) -> TSignal {
        self.edges.last().map(|e| e.value.clone()).unwrap_or_else(|| self.start_value.clone())
    }

    /// 追加边沿列表（合并去重，更新 end_time）
    ///
    /// 若首边沿值与当前最新值相同，跳过首边沿（去重）。
    pub fn append_edges(&mut self, edges: Vec<Edge<TSignal>>) {
        if edges.is_empty() { return; }
        let mut edges = edges;
        let current_latest = self.latest_value();
        if !edges.is_empty() && edges[0].value == current_latest {
            edges.remove(0);
        }
        if edges.is_empty() { return; }
        self.end_time = edges.last().unwrap().time;
        self.edges.extend(edges);
    }

    /// 边沿后值采样：取时间 ≤ sample_time 的最后边沿值
    pub fn sample(&self, sample_time: f32) -> TSignal {
        self.edges.iter()
            .rev()
            .find(|e| e.time <= sample_time)
            .map(|e| e.value.clone())
            .unwrap_or_else(|| self.start_value.clone())
    }

    /// 边沿前值采样：取时间 < sample_time 的最后边沿值
    pub fn sample_before_edge(&self, sample_time: f32) -> TSignal {
        self.edges.iter()
            .rev()
            .find(|e| e.time < sample_time)
            .map(|e| e.value.clone())
            .unwrap_or_else(|| self.start_value.clone())
    }

    /// 时间切片：左开右闭 (from_time, to_time]
    ///
    /// 三种情况：
    /// - start_time > from_time：直接继承 start_value
    /// - from_time == to_time：精确点采样（samle_before_edge）
    /// - start_time <= from_time：用 sample_after_edge 重算起点
    pub fn split(&self, from_time: f32, to_time: f32) -> Option<Fragment<TSignal>> {
        // 不满足切分条件
        if from_time >= self.end_time || to_time < self.start_time {
            return None;
        }
        // 仅一点
        if (from_time - self.end_time).abs() < f32::EPSILON && (from_time - to_time).abs() < f32::EPSILON {
            return None;
        }

        let new_start_value;
        let new_edges;

        if self.start_time > from_time {
            // 情况 1：左端点在切分区间右
            new_start_value = self.start_value.clone();
            new_edges = self.edges.iter()
                .filter(|e| e.time > from_time && e.time <= to_time)
                .cloned()
                .collect();
        } else if (from_time - to_time).abs() < f32::EPSILON {
            // 情况 2：精确点
            new_start_value = self.sample_before_edge(from_time);
            new_edges = Vec::new();
        } else {
            // 情况 3：重新计算起点
            let new_start = self.sample(from_time);
            new_start_value = new_start;
            new_edges = self.edges.iter()
                .filter(|e| e.time > from_time && e.time <= to_time)
                .cloned()
                .collect();
        }

        Some(Fragment {
            signal_id: self.signal_id,
            start_time: from_time,
            end_time: to_time,
            start_value: new_start_value,
            edges: new_edges,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(t: f32, v: i32) -> Edge<i32> { Edge::new(t, v) }

    #[test]
    fn test_fragment_sample() {
        let f = Fragment {
            signal_id: 1, start_time: 0.0, end_time: 10.0,
            start_value: 0, edges: vec![e(3.0, 5), e(7.0, 9)],
        };
        assert_eq!(f.sample(2.0), 0);
        assert_eq!(f.sample(3.0), 5);
        assert_eq!(f.sample(5.0), 5);
        assert_eq!(f.sample(7.0), 9);
    }

    #[test]
    fn test_fragment_split_exact_point() {
        let f = Fragment {
            signal_id: 1, start_time: 0.0, end_time: 10.0,
            start_value: 0, edges: vec![e(5.0, 8)],
        };
        let s = f.split(3.0, 3.0).unwrap();
        assert_eq!(s.start_value, 0);
        assert!(s.edges.is_empty());
    }

    #[test]
    fn test_fragment_split_range() {
        let f = Fragment {
            signal_id: 1, start_time: 0.0, end_time: 10.0,
            start_value: 0, edges: vec![e(3.0, 5), e(7.0, 9)],
        };
        let s = f.split(4.0, 8.0).unwrap();
        assert_eq!(s.start_value, 5);
        assert_eq!(s.edges.len(), 1);
        assert_eq!(s.edges[0].time, 7.0);
        assert_eq!(s.edges[0].value, 9);
    }
}
