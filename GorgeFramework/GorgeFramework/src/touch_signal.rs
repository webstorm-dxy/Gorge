//! `GorgeFramework.TouchSignal` —— 触摸信号值 native 类。
//!
//! 实现 ISignal 标记接口，包装触摸状态和位置坐标。

use gorge_core::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 触摸信号值，含触摸状态和位置坐标
///
/// 实现 ISignal 接口。`position` 存储 Vector2 对象的运行时 ID（usize），
/// 通过 `ctx.get_object_float_field(position, 0/1)` 访问 x/y 坐标。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct TouchSignal {
    #[gorge_field]
    pub is_touching: bool,
    /// Vector2 对象 ID（运行时引用）
    #[gorge_field]
    pub position: usize,
}

#[gorge_native_impl]
impl TouchSignal {
    /// 构造方法 0：从触摸状态和位置对象 ID 初始化
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, is_touching: bool, position_id: usize) {
        ctx.set_object_bool_field(this, Self::FIELD_INDEX_is_touching, is_touching);
        ctx.set_object_object_field(this, Self::FIELD_INDEX_position, position_id);
    }
}
