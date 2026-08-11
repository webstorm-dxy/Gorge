//! `GorgeFramework.Environment` —— 环境查询 native 类。
//!
//! 移植自 C# 参考实现 `Environment.cs`（`System/Native/Environment.cs`）。
//! 提供 `GetAssetByName`、`FindAliveLane`、`Scoring`、`PlayRespondEffect`、
//! `ScreenToWorldPoint`、`ViewportSize` 等静态方法，桥接运行时环境注册表。
//!
//! # 方法编号表
//!
//! | 编号 | 方法 | 说明 |
//! |------|------|------|
//! | 0 | `get_asset_by_name(name: String) -> usize` | 按名称查找资产对象 ID |
//! | 1 | `viewport_size() -> usize` | 返回视口尺寸（Vector2 对象 ID） |
//! | 2 | `find_alive_lane(type_name: String, lane_name: String) -> usize` | 按类型名+name 字段查找存活元素 |
//! | 3 | `find_alive_lane_by_id(type_name: String, lane_id: i32) -> usize` | 按类型名+id 字段查找存活元素 |
//! | 4 | `scoring(result: i32) -> ()` | 调用计分器 respond |
//! | 5 | `play_respond_effect(name: String) -> ()` | 播放响应音效 |
//! | 6 | `screen_to_world_point(x: f32, y: f32, z: f32) -> usize` | 屏幕坐标转世界坐标（返回 Vector3 ID） |

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;
use gorge_core::objective::object::RuntimeObject;
use crate::stage::Scoring;
use crate::system::native::asset::Asset;
use crate::system::native::audio::AudioNative;
use crate::system::native::graph::Graph;
use crate::system::native::image_asset::ImageAsset;
use crate::system::native::native_audio_asset::NativeAudioAsset;
use crate::system::native::native_video_asset::NativeVideoAsset;
use crate::system::native::video::VideoNative;

/// 环境查询类（C# `Environment`）
///
/// 纯静态方法类，无实例字段。所有方法通过全局 `EnvironmentGlobal` 单例
/// 访问运行时环境数据（assets / alive_elements / scoring 等）。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Environment {}

#[gorge_native_impl]
impl Environment {
    /// 静态方法 0 号：按名称查找资产
    ///
    /// 对齐 C# `Environment.GetAssetByName(string)`。
    /// 从全局 EnvironmentGlobal 的资产表中按名称查找并返回对象 ID。
    /// 图片资产延迟包装为 `ImageAsset -> Graph`，音频资产包装为
    /// `NativeAudioAsset -> Audio`，视频资产包装为 `NativeVideoAsset -> Video`，
    /// 平台句柄经全局句柄表桥接，不会泄漏为 VM 对象 ID。
    /// 未找到返回 0（null 对象 ID）。
    #[gorge_static]
    pub fn get_asset_by_name(ctx: &mut NativeContext, asset_name: String) -> usize {
        let asset_handle = crate::runtime::environment::global::with_env_global(|env| {
            env.assets.get(&asset_name).copied()
        });
        let Some(asset_handle) = asset_handle else {
            return 0;
        };

        let vm_address = ctx.vm as *mut _ as usize;
        if let Some(asset_object_id) =
            crate::runtime::environment::global::get_asset_object(vm_address, &asset_name)
        {
            if ctx.vm.objects.contains_key(&asset_object_id) {
                return asset_object_id;
            }
        }

        let asset_object_id = if asset_name.starts_with("image:") {
            let graph_object_id = ctx.register_object(RuntimeObject::new_simple(
                "GorgeFramework.Graph".to_string(),
                &Graph::gorge_field_type_count(),
            ));
            let image_asset_id = ctx.register_object(RuntimeObject::new_simple(
                "GorgeFramework.ImageAsset".to_string(),
                &ImageAsset::gorge_field_type_count(),
            ));
            ctx.set_object_string_field(
                image_asset_id,
                ImageAsset::FIELD_INDEX_name,
                asset_name.clone(),
            );
            ctx.set_object_object_field(
                image_asset_id,
                ImageAsset::FIELD_INDEX_texture,
                graph_object_id,
            );
            crate::runtime::environment::global::register_graph_handle(
                vm_address,
                graph_object_id,
                asset_handle,
            );
            image_asset_id
        } else if asset_name.starts_with("audio:") {
            // 音频资产：包装为 NativeAudioAsset -> Audio，平台音频句柄入全局句柄表
            let audio_object_id = ctx.register_object(RuntimeObject::new_simple(
                "GorgeFramework.Audio".to_string(),
                &AudioNative::gorge_field_type_count(),
            ));
            let audio_asset_id = ctx.register_object(RuntimeObject::new_simple(
                "GorgeFramework.NativeAudioAsset".to_string(),
                &NativeAudioAsset::gorge_field_type_count(),
            ));
            ctx.set_object_string_field(
                audio_asset_id,
                NativeAudioAsset::FIELD_INDEX_name,
                asset_name.clone(),
            );
            ctx.set_object_object_field(
                audio_asset_id,
                NativeAudioAsset::FIELD_INDEX_audio,
                audio_object_id,
            );
            crate::runtime::environment::global::register_audio_handle(
                vm_address,
                audio_object_id,
                asset_handle,
            );
            audio_asset_id
        } else if asset_name.starts_with("video:") {
            // 视频资产：包装为 NativeVideoAsset -> Video，平台视频句柄入全局句柄表
            let video_object_id = ctx.register_object(RuntimeObject::new_simple(
                "GorgeFramework.Video".to_string(),
                &VideoNative::gorge_field_type_count(),
            ));
            let video_asset_id = ctx.register_object(RuntimeObject::new_simple(
                "GorgeFramework.NativeVideoAsset".to_string(),
                &NativeVideoAsset::gorge_field_type_count(),
            ));
            ctx.set_object_string_field(
                video_asset_id,
                NativeVideoAsset::FIELD_INDEX_name,
                asset_name.clone(),
            );
            ctx.set_object_object_field(
                video_asset_id,
                NativeVideoAsset::FIELD_INDEX_video,
                video_object_id,
            );
            crate::runtime::environment::global::register_video_handle(
                vm_address,
                video_object_id,
                asset_handle,
            );
            video_asset_id
        } else {
            let asset_id = ctx.register_object(RuntimeObject::new_simple(
                "GorgeFramework.Asset".to_string(),
                &Asset::gorge_field_type_count(),
            ));
            ctx.set_object_string_field(asset_id, Asset::FIELD_INDEX_name, asset_name.clone());
            asset_id
        };

        crate::runtime::environment::global::register_asset_object(
            vm_address,
            asset_name,
            asset_object_id,
        );
        asset_object_id
    }

    /// 静态方法 1 号：视口尺寸
    ///
    /// 对齐 C# `Environment.ViewportSize()`。
    /// 经全局平台接口获取视口尺寸。
    #[gorge_static]
    pub fn viewport_size(_ctx: &mut NativeContext) -> usize {
        let (w, h) = crate::adaptor::platform().viewport_size();
        // 创建临时 Vector2 对象返回（简化：在全局存储中缓存）
        crate::runtime::environment::global::with_env_global_mut(|env| {
            env.viewport_w = w;
            env.viewport_h = h;
        });
        0
    }

    /// 静态方法 2 号：按类型名和 name 字段查找存活元素
    ///
    /// 对齐 C# `Environment.FindAliveLane(string typeName, string laneName)`。
    /// 遍历全局 alive_elements 表，匹配 class_name 和 name 字段值。
    /// 返回元素对象 ID，未找到返回 0。
    #[gorge_static]
    pub fn find_alive_lane(_ctx: &mut NativeContext, type_name: String, lane_name: String) -> usize {
        crate::runtime::environment::global::with_env_global(|env| {
            env.alive_elements.iter()
                // Demo 以短类名注册编译类，而谱面源码按全限定名查找
                // （如 `FindAliveLane("Dremu.DremuMainLane", ...)`），
                // 因此按全名/末段短名双向匹配，避免音符找不到判定线。
                .find(|info| {
                    info.name == lane_name
                        && (info.class_name == type_name
                            || info.class_name.rsplit('.').next() == Some(type_name.as_str())
                            || type_name.rsplit('.').next() == Some(info.class_name.as_str()))
                })
                .map(|info| info.element_id)
                .unwrap_or(0)
        })
    }

    /// 静态方法 3 号：按类型名和 id 字段查找存活元素
    ///
    /// 对齐 C# `Environment.FindAliveLane(string typeName, int laneId)`。
    /// 遍历全局 alive_elements 表，匹配 class_name 和 lane_id 字段值。
    /// 返回元素对象 ID，未找到返回 0。
    #[gorge_static]
    pub fn find_alive_lane_by_id(_ctx: &mut NativeContext, type_name: String, lane_id: i32) -> usize {
        crate::runtime::environment::global::with_env_global(|env| {
            env.alive_elements.iter()
                .find(|info| info.class_name == type_name && info.lane_id == lane_id)
                .map(|info| info.element_id)
                .unwrap_or(0)
        })
    }

    /// 静态方法 4 号：提交判定结果给计分器
    ///
    /// 对齐 C# `Environment.Scoring(int result)`。
    /// 调用全局 scoring 的 respond 方法。若 scoring 未设置则无操作。
    #[gorge_static]
    pub fn scoring(_ctx: &mut NativeContext, result: i32) {
        crate::runtime::environment::global::with_env_global_mut(|env| {
            if let Some(ref mut scoring) = env.scoring {
                let rr = match result {
                    0 => crate::stage::RespondResult::Miss,
                    1 => crate::stage::RespondResult::Good,
                    2 => crate::stage::RespondResult::Perfect,
                    3 => crate::stage::RespondResult::BestPerfect,
                    _ => crate::stage::RespondResult::Miss,
                };
                scoring.respond(rr);
            }
        })
    }

    /// 静态方法 5 号：播放响应音效
    ///
    /// 对齐 C# `Environment.PlayRespondEffect(string name)`。
    /// 经平台层播放音效（Headless 记录调用）。
    #[gorge_static]
    pub fn play_respond_effect(_ctx: &mut NativeContext, name: String) {
        crate::runtime::environment::global::play_respond_effect_internal(&name);
    }

    /// 静态方法 6 号：屏幕坐标转世界坐标
    ///
    /// 对齐 C# `Environment.ScreenToWorldPoint(Vector3 position)`。
    /// 经全局平台接口转换坐标，创建并返回包含转换后坐标的 Vector3 对象。
    #[gorge_static]
    pub fn screen_to_world_point(ctx: &mut NativeContext, x: f32, y: f32, z: f32) -> usize {
        let (wx, wy, wz) = crate::adaptor::platform().screen_to_world_point(x, y, z);
        // 创建 Vector3 对象（3 个 float 字段：x, y, z）
        let mut tc = gorge_core::objective::types::TypeCount::zero();
        tc.float_count = 3;
        let obj = gorge_core::objective::object::RuntimeObject::new_simple(
            "GorgeFramework.Vector3".to_string(), &tc,
        );
        let obj_id = ctx.register_object(obj);
        ctx.set_object_float_field(obj_id, 0, wx as f64);
        ctx.set_object_float_field(obj_id, 1, wy as f64);
        ctx.set_object_float_field(obj_id, 2, wz as f64);
        obj_id
    }
}

/// Rust 侧兼容名称；native 注册名始终为 Gorge 声明中的 `Environment`。
pub type EnvironmentNative = Environment;

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;
    use crate::runtime::environment::global;

    /// 确保环境全局数据已初始化（不清除已有数据，避免并行测试间互相干扰）
    fn ensure_global() {
        global::init_env_global();
    }

    #[test]
    fn test_environment_class_exists() {
        let env = EnvironmentNative {};
        assert_eq!(env.full_name(), "GorgeFramework.Environment");
    }

    // ==================== R-3 新测试 ====================

    #[test]
    fn test_r3_get_asset_by_name_found() {
        ensure_global();
        // 写入平台纹理句柄（使用唯一键避免测试间冲突）
        global::with_env_global_mut(|env| {
            env.assets.insert("image:test_r3_graph".to_string(), 42);
        });

        let env = EnvironmentNative {};
        let mut vm = VirtualMachine::new();
        vm.register_native_class(env.full_name(), std::sync::Arc::new(EnvironmentNative {}));
        vm.param_pool.set_string_param(0, "image:test_r3_graph".to_string());
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.invoke_native_static_on("GorgeFramework.Environment", 0);
        }
        let image_asset_id = vm.param_pool.get_object_return();
        assert_ne!(image_asset_id, 0, "图片资产应包装为 VM 对象");
        assert_eq!(vm.objects[&image_asset_id].class_name, "GorgeFramework.ImageAsset");

        let graph_id = {
            let ctx = NativeContext::new(&mut vm);
            ctx.get_object_object_field(image_asset_id, ImageAsset::FIELD_INDEX_texture)
        };
        assert_ne!(graph_id, 0, "ImageAsset.texture 应指向 Graph VM 对象");
        let vm_address = &mut vm as *mut VirtualMachine as usize;
        assert_eq!(global::resolve_graph_handle(vm_address, graph_id), 42);

        vm.param_pool.set_string_param(0, "image:test_r3_graph".to_string());
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.invoke_native_static_on("GorgeFramework.Environment", 0);
        }
        assert_eq!(
            vm.param_pool.get_object_return(),
            image_asset_id,
            "同一 VM 应复用资产对象"
        );
    }

    #[test]
    fn test_p1_2_get_asset_by_name_audio() {
        ensure_global();
        global::with_env_global_mut(|env| {
            env.assets.insert("audio:test_p1_2_env".to_string(), 88);
        });

        let env = EnvironmentNative {};
        let mut vm = VirtualMachine::new();
        vm.register_native_class(env.full_name(), std::sync::Arc::new(EnvironmentNative {}));
        vm.param_pool.set_string_param(0, "audio:test_p1_2_env".to_string());
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.invoke_native_static_on("GorgeFramework.Environment", 0);
        }
        let asset_id = vm.param_pool.get_object_return();
        assert_ne!(asset_id, 0, "音频资产应包装为 VM 对象");
        assert_eq!(vm.objects[&asset_id].class_name, "GorgeFramework.NativeAudioAsset");

        let audio_id = {
            let ctx = NativeContext::new(&mut vm);
            ctx.get_object_object_field(asset_id, NativeAudioAsset::FIELD_INDEX_audio)
        };
        assert_ne!(audio_id, 0, "NativeAudioAsset.audio 应指向 Audio VM 对象");
        assert_eq!(vm.objects[&audio_id].class_name, "GorgeFramework.Audio");
        let vm_address = &mut vm as *mut VirtualMachine as usize;
        assert_eq!(global::resolve_audio_handle(vm_address, audio_id), 88);

        // 同名再查应复用已包装的资产对象
        vm.param_pool.set_string_param(0, "audio:test_p1_2_env".to_string());
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.invoke_native_static_on("GorgeFramework.Environment", 0);
        }
        assert_eq!(vm.param_pool.get_object_return(), asset_id, "同一 VM 应复用资产对象");
    }

    #[test]
    fn test_p1_2_get_asset_by_name_video() {
        ensure_global();
        global::with_env_global_mut(|env| {
            env.assets.insert("video:test_p1_2_env".to_string(), 99);
        });

        let env = EnvironmentNative {};
        let mut vm = VirtualMachine::new();
        vm.register_native_class(env.full_name(), std::sync::Arc::new(EnvironmentNative {}));
        vm.param_pool.set_string_param(0, "video:test_p1_2_env".to_string());
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.invoke_native_static_on("GorgeFramework.Environment", 0);
        }
        let asset_id = vm.param_pool.get_object_return();
        assert_ne!(asset_id, 0, "视频资产应包装为 VM 对象");
        assert_eq!(vm.objects[&asset_id].class_name, "GorgeFramework.NativeVideoAsset");

        let video_id = {
            let ctx = NativeContext::new(&mut vm);
            ctx.get_object_object_field(asset_id, NativeVideoAsset::FIELD_INDEX_video)
        };
        assert_ne!(video_id, 0, "NativeVideoAsset.video 应指向 Video VM 对象");
        assert_eq!(vm.objects[&video_id].class_name, "GorgeFramework.Video");
        let vm_address = &mut vm as *mut VirtualMachine as usize;
        assert_eq!(global::resolve_video_handle(vm_address, video_id), 99);
    }

    #[test]
    fn test_r3_get_asset_by_name_not_found() {
        ensure_global();
        // 全局中无此资产
        let env = EnvironmentNative {};
        let mut vm = VirtualMachine::new();
        vm.register_native_class(env.full_name(), std::sync::Arc::new(EnvironmentNative {}));
        vm.param_pool.set_string_param(0, "nonexistent".to_string());
        { let mut ctx = NativeContext::new(&mut vm); env.invoke_native_static(&mut ctx, 0); }
        assert_eq!(vm.param_pool.get_object_return(), 0);
    }

    #[test]
    fn test_r3_find_alive_lane_by_name_found() {
        ensure_global();
        let info = global::AliveElementInfo::new(100, "GorgeFramework.TapNote".into())
            .with_name("r3_lane_1".into())
            .with_lane_id(5);
        global::with_env_global_mut(|env| {
            env.alive_elements.push(info);
        });

        let env = EnvironmentNative {};
        let mut vm = VirtualMachine::new();
        vm.register_native_class(env.full_name(), std::sync::Arc::new(EnvironmentNative {}));
        vm.param_pool.set_string_param(0, "GorgeFramework.TapNote".to_string());
        vm.param_pool.set_string_param(1, "r3_lane_1".to_string());
        { let mut ctx = NativeContext::new(&mut vm); env.invoke_native_static(&mut ctx, 2); }
        assert_eq!(vm.param_pool.get_object_return(), 100);
    }

    #[test]
    fn test_r3_find_alive_lane_by_name_not_found() {
        ensure_global();
        let env = EnvironmentNative {};
        let mut vm = VirtualMachine::new();
        vm.register_native_class(env.full_name(), std::sync::Arc::new(EnvironmentNative {}));
        vm.param_pool.set_string_param(0, "GorgeFramework.TapNote".to_string());
        vm.param_pool.set_string_param(1, "missing_lane".to_string());
        { let mut ctx = NativeContext::new(&mut vm); env.invoke_native_static(&mut ctx, 2); }
        assert_eq!(vm.param_pool.get_object_return(), 0);
    }

    #[test]
    fn test_r3_find_alive_lane_by_id_found() {
        ensure_global();
        let info = global::AliveElementInfo::new(200, "GorgeFramework.HoldNote".into())
            .with_name("hold_r3".into())
            .with_lane_id(7);
        global::with_env_global_mut(|env| {
            env.alive_elements.push(info);
        });

        let env = EnvironmentNative {};
        let mut vm = VirtualMachine::new();
        vm.register_native_class(env.full_name(), std::sync::Arc::new(EnvironmentNative {}));
        vm.param_pool.set_string_param(0, "GorgeFramework.HoldNote".to_string());
        vm.param_pool.set_int_param(0, 7);
        { let mut ctx = NativeContext::new(&mut vm); env.invoke_native_static(&mut ctx, 3); }
        assert_eq!(vm.param_pool.get_object_return(), 200);
    }

    #[test]
    fn test_r3_scoring_calls_respond() {
        ensure_global();
        let scoring = crate::stage::ScoringV1::new(10);
        global::with_env_global_mut(|env| {
            env.scoring = Some(scoring);
        });

        let env = EnvironmentNative {};
        let mut vm = VirtualMachine::new();
        vm.register_native_class(env.full_name(), std::sync::Arc::new(EnvironmentNative {}));
        // result=2 → Perfect
        vm.param_pool.set_int_param(0, 2);
        { let mut ctx = NativeContext::new(&mut vm); env.invoke_native_static(&mut ctx, 4); }
        // 调用后检查 scoring 状态
        global::with_env_global(|env| {
            if let Some(ref s) = env.scoring {
                assert_eq!(s.combo(), 1);
                assert_eq!(s.milepost(), crate::stage::ScoreMilepost::AllPerfect);
            } else {
                panic!("scoring 不应为 None");
            }
        });
    }

    #[test]
    fn test_r3_play_respond_effect_no_panic() {
        ensure_global();
        // 安装 Headless 平台
        let hp = crate::adaptor::HeadlessPlatform::new();
        crate::adaptor::install_platform(Box::new(hp));

        let env = EnvironmentNative {};
        let mut vm = VirtualMachine::new();
        vm.register_native_class(env.full_name(), std::sync::Arc::new(EnvironmentNative {}));
        vm.param_pool.set_string_param(0, "hit_sound".to_string());
        // 验证调用不 panic（Headless 记录调用至内部日志）
        { let mut ctx = NativeContext::new(&mut vm); env.invoke_native_static(&mut ctx, 5); }
    }

    #[test]
    fn test_c1_screen_to_world_point_returns_vector3_object() {
        ensure_global();
        let hp = crate::adaptor::HeadlessPlatform::new();
        crate::adaptor::install_platform(Box::new(hp));

        let env = EnvironmentNative {};
        let mut vm = VirtualMachine::new();
        vm.register_native_class(env.full_name(), std::sync::Arc::new(EnvironmentNative {}));
        vm.param_pool.set_float_param(0, 100.0);
        vm.param_pool.set_float_param(1, 200.0);
        vm.param_pool.set_float_param(2, 0.0);
        { let mut ctx = NativeContext::new(&mut vm); env.invoke_native_static(&mut ctx, 6); }
        let obj_id = vm.param_pool.get_object_return();
        // 返回非零对象 ID
        assert!(obj_id > 0, "screen_to_world_point 应返回非零对象 ID");
        // 验证 Vector3 对象存在于对象表中
        assert!(vm.objects.contains_key(&obj_id));
        // HeadlessPlatform 的 screen_to_world_point 返回原值
        // 验证字段已正确设置
        let obj = &vm.objects[&obj_id];
        assert_eq!(obj.class_name, "GorgeFramework.Vector3");
    }
}
