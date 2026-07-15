//! `GorgeFramework.PeriodConfig` —— 时间段配置 native 类。
//!
//! 表示一个时间段 [startTime, endTime]，用于 Chart 时间区间定义。

use gorge_core::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 时间段配置，含起止时间（float）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct PeriodConfig {
    #[gorge_field]
    pub start_time: f32,
    #[gorge_field]
    pub end_time: f32,
}

#[gorge_native_impl]
impl PeriodConfig {
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, start: f32, end: f32) {
        ctx.set_object_float_field(this, Self::FIELD_INDEX_start_time, start as f64);
        ctx.set_object_float_field(this, Self::FIELD_INDEX_end_time, end as f64);
    }
}
