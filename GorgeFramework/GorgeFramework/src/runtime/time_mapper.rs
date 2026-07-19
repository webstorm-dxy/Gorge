//! 时间映射器（对应 C# `Runtime/ITimeMapper.cs`）。
//!
//! 谱面时间与模拟时间之间的映射。当前仅提供等比缩放实现。

/// 时间映射器 trait
///
/// 谱面时间（Chart Time）→ 模拟时间（Simulate Time）的双向映射。
/// 未来可扩展为非线性映射（如变速谱面）。
pub trait TimeMapper: Send + Sync {
    /// 谱面时间 → 模拟时间
    fn chart_to_simulate(&self, chart_time: f32) -> f32;
    /// 模拟时间 → 谱面时间
    fn simulate_to_chart(&self, simulate_time: f32) -> f32;
}

/// 等比缩放时间映射器
///
/// `simulate_time = chart_time * scale + offset`
#[derive(Debug, Clone)]
pub struct ScalingMapper {
    /// 缩放系数（如 1.0 = 原速，2.0 = 两倍速）
    pub scale: f32,
    /// 偏移量
    pub offset: f32,
}

impl ScalingMapper {
    pub fn new(scale: f32, offset: f32) -> Self {
        Self { scale, offset }
    }
}

impl TimeMapper for ScalingMapper {
    fn chart_to_simulate(&self, chart_time: f32) -> f32 {
        chart_time * self.scale + self.offset
    }

    fn simulate_to_chart(&self, simulate_time: f32) -> f32 {
        if self.scale.abs() < 1e-10 {
            simulate_time
        } else {
            (simulate_time - self.offset) / self.scale
        }
    }
}

/// 恒等映射器（不缩放，simulate_time == chart_time）
#[derive(Debug, Clone)]
pub struct IdentityMapper;

impl TimeMapper for IdentityMapper {
    fn chart_to_simulate(&self, chart_time: f32) -> f32 { chart_time }
    fn simulate_to_chart(&self, simulate_time: f32) -> f32 { simulate_time }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaling_mapper() {
        let m = ScalingMapper::new(2.0, 0.0);
        assert!((m.chart_to_simulate(5.0) - 10.0).abs() < 0.001);
        assert!((m.simulate_to_chart(10.0) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_identity_mapper() {
        let m = IdentityMapper;
        assert_eq!(m.chart_to_simulate(5.0), 5.0);
        assert_eq!(m.simulate_to_chart(5.0), 5.0);
    }
}
