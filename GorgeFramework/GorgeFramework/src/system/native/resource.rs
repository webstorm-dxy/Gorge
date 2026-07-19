//! `GorgeFramework` — 资源/资产标记类型（native 类注册）。
//!
//! 移植自 C# 参考实现 `System/Native/` 中 Asset 族的 5 个纯数据类。
//! 包含 Asset（基类）、GraphAsset（图形资产基类）、ImageAsset（图片资产）、
//! NativeAudioAsset（原生音频资产）、NativeVideoAsset（原生视频资产）。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

// ==================== Graph ====================

/// 图形/纹理资源（C# `Graph`）
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Graph {
    #[gorge_field]
    pub width: i32,
    #[gorge_field]
    pub height: i32,
}

#[gorge_native_impl]
impl Graph {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, width: i32, height: i32) {
        ctx.set_object_int_field(this, Graph::FIELD_INDEX_width, width as i64);
        ctx.set_object_int_field(this, Graph::FIELD_INDEX_height, height as i64);
    }
}

// ==================== Asset 基类 ====================

/// 资源基类（C# `Asset`）
///
/// 字段 `name` 为资产名称，方法 `LoadAsset` 为虚方法（基类返回 false 表示未实现）。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Asset {
    /// 资产名称
    #[gorge_field]
    pub name: String,
}

#[gorge_native_impl]
impl Asset {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_string_field(this, Asset::FIELD_INDEX_name, String::new());
    }

    /// 0 号方法：加载资产
    ///
    /// 基类默认返回 false（对齐 C# abstract 语义：未实现时抛异常）。
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        false
    }
}

// ==================== GraphAsset ====================

/// 图形资产基类（C# `GraphAsset`，继承自 `Asset`）
///
/// 定义虚方法 `GetAsset`，子类覆盖返回具体的 Graph 对象。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct GraphAsset {
    /// 资产名称（继承自 Asset）
    #[gorge_field]
    pub name: String,
}

#[gorge_native_impl]
impl GraphAsset {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_string_field(this, GraphAsset::FIELD_INDEX_name, String::new());
    }

    /// 0 号方法：加载资产（继承自 Asset）
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        false
    }

    /// 1 号方法：获取图形资产
    ///
    /// 基类默认抛异常（对齐 C# abstract 语义），返回 0 表示无。
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn get_asset(ctx: &mut NativeContext, this: usize) -> usize {
        0
    }
}

// ==================== ImageAsset ====================

/// 图片资产（C# `ImageAsset`，继承自 `GraphAsset`）
///
/// 字段 `texture` 存储 Graph 对象 ID。
/// `LoadAsset` 直接返回 true（值引用无需加载），`GetAsset` 返回 texture。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct ImageAsset {
    /// 资产名称（继承自 Asset）
    #[gorge_field]
    pub name: String,
    /// 纹理 Graph 对象 ID
    #[gorge_field]
    pub texture: usize,
}

#[gorge_native_impl]
impl ImageAsset {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_string_field(this, ImageAsset::FIELD_INDEX_name, String::new());
        ctx.set_object_object_field(this, ImageAsset::FIELD_INDEX_texture, 0);
    }

    /// 0 号方法：加载资产（覆盖，值包装无需加载）
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        true
    }

    /// 1 号方法：获取图形资产（覆盖，返回 texture 字段）
    #[gorge_method]
    pub fn get_asset(ctx: &mut NativeContext, this: usize) -> usize {
        ctx.get_object_object_field(this, ImageAsset::FIELD_INDEX_texture)
    }

    /// 2 号方法：描述显示字符串
    #[gorge_method]
    pub fn descriptor_display_string(ctx: &mut NativeContext, this: usize) -> String {
        ctx.get_object_string_field(this, ImageAsset::FIELD_INDEX_name)
    }
}

// ==================== NativeAudioAsset ====================

/// 原生音频资产（C# `NativeAudioAsset`，继承自 `AudioAsset`）
///
/// 直接持有 Audio 对象引用，`LoadAsset` 返回 true。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct NativeAudioAsset {
    /// 资产名称（继承自 Asset）
    #[gorge_field]
    pub name: String,
    /// 音频对象 ID
    #[gorge_field]
    pub audio: usize,
}

#[gorge_native_impl]
impl NativeAudioAsset {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_string_field(this, NativeAudioAsset::FIELD_INDEX_name, String::new());
        ctx.set_object_object_field(this, NativeAudioAsset::FIELD_INDEX_audio, 0);
    }

    /// 0 号方法：加载资产（覆盖，值包装无需加载）
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        true
    }

    /// 1 号方法：获取音频资产（覆盖，返回 audio 字段）
    #[gorge_method]
    pub fn get_asset(ctx: &mut NativeContext, this: usize) -> usize {
        ctx.get_object_object_field(this, NativeAudioAsset::FIELD_INDEX_audio)
    }
}

// ==================== NativeVideoAsset ====================

/// 原生视频资产（C# `NativeVideoAsset`，继承自 `VideoAsset`）
///
/// 直接持有 Video 对象引用，`LoadAsset` 返回 true。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct NativeVideoAsset {
    /// 资产名称（继承自 Asset）
    #[gorge_field]
    pub name: String,
    /// 视频对象 ID
    #[gorge_field]
    pub video: usize,
}

#[gorge_native_impl]
impl NativeVideoAsset {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize) {
        ctx.set_object_string_field(this, NativeVideoAsset::FIELD_INDEX_name, String::new());
        ctx.set_object_object_field(this, NativeVideoAsset::FIELD_INDEX_video, 0);
    }

    /// 0 号方法：加载资产（覆盖，值包装无需加载）
    #[gorge_method]
    #[allow(unused_variables)]
    pub fn load_asset(ctx: &mut NativeContext, this: usize) -> bool {
        true
    }

    /// 1 号方法：获取视频资产（覆盖，返回 video 字段）
    #[gorge_method]
    pub fn get_asset(ctx: &mut NativeContext, this: usize) -> usize {
        ctx.get_object_object_field(this, NativeVideoAsset::FIELD_INDEX_video)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    fn make_vm() -> VirtualMachine {
        let mut vm = VirtualMachine::new();
        vm.next_object_id = 1;
        vm
    }

    fn register(vm: &mut VirtualMachine, cls: std::sync::Arc<dyn NativeClass>) {
        let name = cls.full_name().to_string();
        vm.register_native_class(&name, cls);
    }

    #[test]
    fn test_asset_construct_and_load() {
        let a = Asset { name: String::new() };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(Asset { name: String::new() }));
        let id = { let mut ctx = NativeContext::new(&mut vm); a.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        // LoadAsset 基类返回 false
        { let mut ctx = NativeContext::new(&mut vm); a.invoke_native_method(&mut ctx, id, 0); }
        assert!(!vm.param_pool.get_bool_return());
    }

    #[test]
    fn test_image_asset_get_asset_returns_texture() {
        let ia = ImageAsset { name: String::new(), texture: 0 };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(ImageAsset { name: String::new(), texture: 0 }));
        let id = { let mut ctx = NativeContext::new(&mut vm); ia.do_construct_native(&mut ctx, None, 0) };
        // 设置 texture 字段
        { let mut ctx = NativeContext::new(&mut vm); ctx.set_object_object_field(id, ImageAsset::FIELD_INDEX_texture, 42); }
        // GetAsset 返回 texture
        { let mut ctx = NativeContext::new(&mut vm); ia.invoke_native_method(&mut ctx, id, 1); }
        assert_eq!(vm.param_pool.get_object_return(), 42);
        // LoadAsset 返回 true
        { let mut ctx = NativeContext::new(&mut vm); ia.invoke_native_method(&mut ctx, id, 0); }
        assert!(vm.param_pool.get_bool_return());
    }

    #[test]
    fn test_image_asset_descriptor_display_string() {
        let ia = ImageAsset { name: String::new(), texture: 0 };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(ImageAsset { name: String::new(), texture: 0 }));
        let id = { let mut ctx = NativeContext::new(&mut vm); ia.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = NativeContext::new(&mut vm); ctx.set_object_string_field(id, ImageAsset::FIELD_INDEX_name, "test_img".to_string()); }
        { let mut ctx = NativeContext::new(&mut vm); ia.invoke_native_method(&mut ctx, id, 2); }
        assert_eq!(vm.param_pool.get_string_return(), "test_img");
    }

    #[test]
    fn test_native_audio_asset_get_asset() {
        let na = NativeAudioAsset { name: String::new(), audio: 0 };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(NativeAudioAsset { name: String::new(), audio: 0 }));
        let id = { let mut ctx = NativeContext::new(&mut vm); na.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = NativeContext::new(&mut vm); ctx.set_object_object_field(id, NativeAudioAsset::FIELD_INDEX_audio, 99); }
        { let mut ctx = NativeContext::new(&mut vm); na.invoke_native_method(&mut ctx, id, 1); }
        assert_eq!(vm.param_pool.get_object_return(), 99);
        // LoadAsset 返回 true
        { let mut ctx = NativeContext::new(&mut vm); na.invoke_native_method(&mut ctx, id, 0); }
        assert!(vm.param_pool.get_bool_return());
    }

    #[test]
    fn test_native_video_asset_get_asset() {
        let nv = NativeVideoAsset { name: String::new(), video: 0 };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(NativeVideoAsset { name: String::new(), video: 0 }));
        let id = { let mut ctx = NativeContext::new(&mut vm); nv.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = NativeContext::new(&mut vm); ctx.set_object_object_field(id, NativeVideoAsset::FIELD_INDEX_video, 77); }
        { let mut ctx = NativeContext::new(&mut vm); nv.invoke_native_method(&mut ctx, id, 1); }
        assert_eq!(vm.param_pool.get_object_return(), 77);
    }

    #[test]
    fn test_graph_asset_get_asset_base() {
        let ga = GraphAsset { name: String::new() };
        let mut vm = make_vm();
        register(&mut vm, std::sync::Arc::new(GraphAsset { name: String::new() }));
        let id = { let mut ctx = NativeContext::new(&mut vm); ga.do_construct_native(&mut ctx, None, 0) };
        // 基类 GetAsset 返回 0
        { let mut ctx = NativeContext::new(&mut vm); ga.invoke_native_method(&mut ctx, id, 1); }
        assert_eq!(vm.param_pool.get_object_return(), 0);
    }
}
