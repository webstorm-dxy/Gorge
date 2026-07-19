//! `GorgeFramework.NoteLinkage` —— 音符联动数据 native 类。
//!
//! 对齐 C# `NoteLinkage`。单字段数据类，通过 JSON 字符串描述音符间的
//! 联动关系（如 hold→click 的依赖链）。当前仅注册字段布局，
//! JSON 解析逻辑由宿主侧处理。

use gorge_core::objective::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 音符联动数据
///
/// 字段 `json` 保存序列化的联动关系描述（JSON 格式）。
/// 完整解析（`JsonConvert.DeserializeObject<Linkage.NoteLinkage>`）
/// 属宿主引擎能力，native 桥接层仅保留原始字符串。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct NoteLinkage {
    #[gorge_field]
    pub json: String,
}

#[gorge_native_impl]
impl NoteLinkage {
    /// 构造方法 0：从 json 初始化
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, json: String) {
        ctx.set_object_string_field(this, Self::FIELD_INDEX_json, json);
    }
}
