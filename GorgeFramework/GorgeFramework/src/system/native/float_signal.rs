//! `GorgeFramework.FloatSignal` —— 浮点信号值 native 类。
//!
//! 实现 ISignal 标记接口，包装一个 float 值用于信号系统传递。

use gorge_core::objective::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 浮点信号值，包装一个 f32 数值
///
/// 实现 ISignal 接口，供信号过滤器（FloatSignalFilter）使用。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct FloatSignal {
    #[gorge_field]
    pub value: f32,
}

#[gorge_native_impl]
impl FloatSignal {
    /// 构造方法 0：从 value 初始化
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, value: f32) {
        ctx.set_object_float_field(this, Self::FIELD_INDEX_value, value as f64);
    }
}
