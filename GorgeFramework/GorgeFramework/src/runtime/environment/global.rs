//! 环境全局注册表（对应 C# `RuntimeStatic` 全局单例）。
//!
//! EnvironmentNative 等 native 类的方法无法直接持有 GorgeSimulationRuntime 引用
//! （native 方法经 VM 调用，参数仅有 `&mut NativeContext`），因此通过本模块的
//! `OnceLock<Mutex<EnvironmentGlobal>>` 全局单例桥接环境数据。
//!
//! # 方案选择
//!
//! C# 通过 `RuntimeStatic.Runtime` 全局单例访问环境数据；
//! Rust 版采用 `OnceLock<Mutex<EnvironmentGlobal>>` 提供等价能力。
//! - 拒绝 `vm.native_payloads` 方案：native 方法执行时 `&mut vm` 已借出，无法再取环境数据
//! - 选择专用全局注册表：与 C# 语义最接近，接口清晰
//!
//! # 使用注意
//!
//! 全局单例在测试间共享。测试应在操作环境数据后清理，避免状态泄漏。
//! 断言不依赖其他测试写入的绝对状态值（优先用相对断言或查询后验证）。

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// ==================== 存活元素快照 ====================

/// 存活元素信息快照（供 `FindAliveLane` 按类型名+字段值查找）。
///
/// ChartManager 在元素创生/销毁时同步本表。
#[derive(Debug, Clone)]
pub struct AliveElementInfo {
    /// 元素对象 ID
    pub element_id: usize,
    /// 元素所属类全名（如 `GorgeFramework.TapNote`）
    pub class_name: String,
    /// 字符串字段 `name` 的值（若无此字段则为空）
    pub name: String,
    /// 整数字段 `id` 的值（若无此字段则为 0）
    pub lane_id: i32,
}

impl AliveElementInfo {
    pub fn new(element_id: usize, class_name: String) -> Self {
        Self { element_id, class_name, name: String::new(), lane_id: 0 }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_lane_id(mut self, lane_id: i32) -> Self {
        self.lane_id = lane_id;
        self
    }
}

// ==================== 环境全局数据 ====================

/// 环境全局数据（对应 C# `RuntimeStatic.Runtime`）。
///
/// 持有 native 类（EnvironmentNative）所需的全部环境引用：
/// - 资产名 → 平台资源句柄（GetAssetByName 在 VM 内包装为资产对象）
/// - 存活元素表（FindAliveLane）
/// - 计分器（Scoring）
/// - 响应音效表（PlayRespondEffect）
pub struct EnvironmentGlobal {
    /// 资产名 → 平台资源句柄
    pub assets: HashMap<String, usize>,
    /// `(VM 地址, 资产名)` → 已包装的 VM 资产对象 ID。
    pub asset_objects: HashMap<(usize, String), usize>,
    /// `(VM 地址, Graph 对象 ID)` → 平台纹理句柄。
    pub graph_handles: HashMap<(usize, usize), usize>,
    /// `(VM 地址, Audio 对象 ID)` → 平台音频句柄。
    pub audio_handles: HashMap<(usize, usize), usize>,
    /// `(VM 地址, Video 对象 ID)` → 平台视频句柄。
    pub video_handles: HashMap<(usize, usize), usize>,
    /// 存活元素信息表（用于 FindAliveLane）
    pub alive_elements: Vec<AliveElementInfo>,
    /// 计分器（由 SceneManager 在 RuntimeInitialize 时设置）
    pub scoring: Option<crate::stage::ScoringV1>,
    /// 响应音效名 → 音效播放器 ID
    pub respond_effects: HashMap<String, usize>,
    /// 视图宽度
    pub viewport_w: f32,
    /// 视图高度
    pub viewport_h: f32,
}

impl EnvironmentGlobal {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            asset_objects: HashMap::new(),
            graph_handles: HashMap::new(),
            audio_handles: HashMap::new(),
            video_handles: HashMap::new(),
            alive_elements: Vec::new(),
            scoring: None,
            respond_effects: HashMap::new(),
            viewport_w: 1920.0,
            viewport_h: 1080.0,
        }
    }
}

impl Default for EnvironmentGlobal {
    fn default() -> Self { Self::new() }
}

// ==================== 全局单例 ====================

/// 全局环境数据单例
static ENV_GLOBAL: OnceLock<Mutex<EnvironmentGlobal>> = OnceLock::new();

/// 初始化环境全局数据（首次调用生效，后续忽略）
pub fn init_env_global() {
    let _ = ENV_GLOBAL.set(Mutex::new(EnvironmentGlobal::new()));
}

/// 以只读方式访问环境全局数据（防锁中毒）
pub fn with_env_global<F, R>(f: F) -> R
where
    F: FnOnce(&EnvironmentGlobal) -> R,
{
    let lock = ENV_GLOBAL.get().expect("EnvironmentGlobal 未初始化，请先调用 init_env_global()");
    let guard = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&guard)
}

/// 以可变方式访问环境全局数据（防锁中毒）
pub fn with_env_global_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut EnvironmentGlobal) -> R,
{
    let lock = ENV_GLOBAL.get().expect("EnvironmentGlobal 未初始化，请先调用 init_env_global()");
    let mut guard = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut guard)
}

/// 重置环境全局数据（测试用：清空所有状态）
///
/// 注意：若全局尚未初始化，先初始化。防锁中毒以支持测试间隔离。
pub fn reset_env_global() {
    if let Some(lock) = ENV_GLOBAL.get() {
        let mut guard = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        *guard = EnvironmentGlobal::new();
    } else {
        let _ = ENV_GLOBAL.set(Mutex::new(EnvironmentGlobal::new()));
    }
}

// ==================== 同步辅助方法 ====================

/// 将 AssetManager 的资产表同步到全局（由 RuntimeManager 在 prepare_score 后调用）
pub fn sync_assets_from(assets: &HashMap<String, usize>) {
    with_env_global_mut(|env| {
        env.assets.clear();
        env.assets.extend(assets.iter().map(|(k, v)| (k.clone(), *v)));
        env.asset_objects.clear();
        env.graph_handles.clear();
        env.audio_handles.clear();
        env.video_handles.clear();
    });
}

/// 查询指定 VM 中已包装的资产对象。
pub fn get_asset_object(vm_address: usize, asset_name: &str) -> Option<usize> {
    with_env_global(|env| {
        env.asset_objects
            .get(&(vm_address, asset_name.to_string()))
            .copied()
    })
}

/// 记录指定 VM 的资产对象。
pub fn register_asset_object(vm_address: usize, asset_name: String, object_id: usize) {
    with_env_global_mut(|env| {
        env.asset_objects.insert((vm_address, asset_name), object_id);
    });
}

/// 记录 Graph VM 对象与平台纹理句柄的对应关系。
pub fn register_graph_handle(
    vm_address: usize,
    graph_object_id: usize,
    texture_handle: usize,
) {
    with_env_global_mut(|env| {
        env.graph_handles.insert((vm_address, graph_object_id), texture_handle);
    });
}

/// 将 Graph VM 对象 ID 解析为平台纹理句柄。
///
/// 非资产创建的 Graph 没有注册映射时，保留原始值以兼容直接传入平台句柄的调用方。
pub fn resolve_graph_handle(vm_address: usize, graph_object_id: usize) -> usize {
    if graph_object_id == 0 {
        return 0;
    }
    with_env_global(|env| {
        env.graph_handles
            .get(&(vm_address, graph_object_id))
            .copied()
            .unwrap_or(graph_object_id)
    })
}

/// 记录 Audio VM 对象与平台音频句柄的对应关系。
pub fn register_audio_handle(
    vm_address: usize,
    audio_object_id: usize,
    audio_handle: usize,
) {
    with_env_global_mut(|env| {
        env.audio_handles.insert((vm_address, audio_object_id), audio_handle);
    });
}

/// 将 Audio VM 对象 ID 解析为平台音频句柄。
///
/// 非资产创建的 Audio 没有注册映射时，保留原始值以兼容直接传入平台句柄的调用方。
pub fn resolve_audio_handle(vm_address: usize, audio_object_id: usize) -> usize {
    if audio_object_id == 0 {
        return 0;
    }
    with_env_global(|env| {
        env.audio_handles
            .get(&(vm_address, audio_object_id))
            .copied()
            .unwrap_or(audio_object_id)
    })
}

/// 记录 Video VM 对象与平台视频句柄的对应关系。
pub fn register_video_handle(
    vm_address: usize,
    video_object_id: usize,
    video_handle: usize,
) {
    with_env_global_mut(|env| {
        env.video_handles.insert((vm_address, video_object_id), video_handle);
    });
}

/// 将 Video VM 对象 ID 解析为平台视频句柄。
///
/// 非资产创建的 Video 没有注册映射时，保留原始值以兼容直接传入平台句柄的调用方。
pub fn resolve_video_handle(vm_address: usize, video_object_id: usize) -> usize {
    if video_object_id == 0 {
        return 0;
    }
    with_env_global(|env| {
        env.video_handles
            .get(&(vm_address, video_object_id))
            .copied()
            .unwrap_or(video_object_id)
    })
}

/// 将 ScoringV1 同步到全局（由 SceneManager.RuntimeInitialize 调用）
pub fn sync_scoring(scoring: crate::stage::ScoringV1) {
    with_env_global_mut(|env| {
        env.scoring = Some(scoring);
    });
}

/// 播放响应音效（经全局音效表 → 平台层）
pub fn play_respond_effect_internal(name: &str) {
    let effect_id = with_env_global(|env| {
        env.respond_effects.get(name).copied()
    });
    if let Some(_id) = effect_id {
        // 音效播放经平台层（Headless 记录调用）
        let effect = crate::adaptor::platform().create_audio_effect_player(0);
        effect.play();
    }
}
