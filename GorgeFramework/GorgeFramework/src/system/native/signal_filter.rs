//! `GorgeFramework` — 信号过滤器系统（native 类注册）。
//!
//! 移植自 C# 参考实现。FloatSignalFilter 注册为 native 类；
//! SignalFilter trait 保留为内部 Rust 接口。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

/// 信号过滤器 trait（内部接口，不注册 native）
pub trait SignalFilter: std::fmt::Debug + Send + Sync {
    fn can_detect(&self, channel: &str) -> bool;
    fn detect(&self, value: f32) -> bool;
}

/// 时间模式（枚举作为 i32 存储于 Gorge 对象中）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeMode { CatchBefore = 0, KeepDuring = 1 }

/// 浮点信号过滤器
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct FloatSignalFilter {
    #[gorge_field] pub channel_name: String,
    #[gorge_field] pub min_value: f32,
    #[gorge_field] pub max_value: f32,
    /// 时间模式（存储为 int，0=CatchBefore, 1=KeepDuring）
    #[gorge_field] pub time_mode: i32,
    #[gorge_field] pub accept_consume: bool,
    #[gorge_field] pub deny_consume: bool,
    #[gorge_field] pub end_time: f32,
}

impl FloatSignalFilter {
    pub fn new(channel_name: &str, min: f32, max: f32) -> Self {
        Self {
            channel_name: channel_name.into(), min_value: min, max_value: max,
            time_mode: TimeMode::CatchBefore as i32, accept_consume: true,
            deny_consume: false, end_time: f32::INFINITY,
        }
    }
    /// 从 i32 构造 TimeMode（用于 native 方法内部）
    pub fn time_mode_enum(time_mode: i32) -> TimeMode {
        match time_mode { 0 => TimeMode::CatchBefore, 1 => TimeMode::KeepDuring, _ => TimeMode::CatchBefore }
    }
}

#[gorge_native_impl]
impl FloatSignalFilter {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize,
        channel_name: String, min_value: f32, max_value: f32,
        time_mode: i32, accept_consume: bool, deny_consume: bool, end_time: f32)
    {
        ctx.set_object_string_field(this, FloatSignalFilter::FIELD_INDEX_channel_name, channel_name);
        ctx.set_object_float_field(this, FloatSignalFilter::FIELD_INDEX_min_value, min_value as f64);
        ctx.set_object_float_field(this, FloatSignalFilter::FIELD_INDEX_max_value, max_value as f64);
        ctx.set_object_int_field(this, FloatSignalFilter::FIELD_INDEX_time_mode, time_mode as i64);
        ctx.set_object_bool_field(this, FloatSignalFilter::FIELD_INDEX_accept_consume, accept_consume);
        ctx.set_object_bool_field(this, FloatSignalFilter::FIELD_INDEX_deny_consume, deny_consume);
        ctx.set_object_float_field(this, FloatSignalFilter::FIELD_INDEX_end_time, end_time as f64);
    }

    #[gorge_method]
    pub fn can_detect(ctx: &mut NativeContext, this: usize, channel: String) -> bool {
        let name = ctx.get_object_string_field(this, FloatSignalFilter::FIELD_INDEX_channel_name);
        name == channel
    }

    #[gorge_method]
    pub fn detect(ctx: &mut NativeContext, this: usize, value: f32) -> bool {
        let min = ctx.get_object_float_field(this, FloatSignalFilter::FIELD_INDEX_min_value) as f32;
        let max = ctx.get_object_float_field(this, FloatSignalFilter::FIELD_INDEX_max_value) as f32;
        value >= min && value <= max
    }
}

impl SignalFilter for FloatSignalFilter {
    fn can_detect(&self, channel: &str) -> bool { self.channel_name == channel }
    fn detect(&self, value: f32) -> bool { value >= self.min_value && value <= self.max_value }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_filter_detect() {
        let f = FloatSignalFilter::new("speed", 0.5, 1.0);
        assert!(f.can_detect("speed"));
        assert!(!f.can_detect("position"));
        assert!(f.detect(0.7));
        assert!(!f.detect(0.2));
        assert!(f.detect(1.0));
    }
}
