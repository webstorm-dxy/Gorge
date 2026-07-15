//! `GorgeFramework.BoolSignal` —— 布尔信号值 native 类。
//!
//! 实现 ISignal 标记接口，包装一个 bool 值用于信号系统传递。

use gorge_core::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 布尔信号值，包装一个 bool 值
///
/// 实现 ISignal 接口，常用于开关/触发类信号。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct BoolSignal {
    #[gorge_field]
    pub value: bool,
}

#[gorge_native_impl]
impl BoolSignal {
    /// 构造方法 0：从 value 初始化
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, value: bool) {
        ctx.set_object_bool_field(this, Self::FIELD_INDEX_value, value);
    }
}
