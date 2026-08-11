//! 平台适配模块（对应 C# `Adaptor/` 文件夹）。
//!
//! 定义宿主引擎（Bevy/Godot/Unity 等）需要实现的平台能力接口，
//! 包括精灵渲染、音频播放、资源加载等。
//!
//! # 架构
//!
//! - `ISprite` / `INineSliceSprite` / `ICurveSprite` — 精灵渲染接口族
//! - `IAudioPlayer` — 音频播放器接口
//! - `PlatformBase` — 平台根接口，负责创建精灵/音频等
//! - `HeadlessPlatform` — 无头测试实现，记录调用序列供断言
//!
//! # 安装机制
//!
//! 使用 `std::sync::OnceLock` 全局单例（对齐 C# `Base.Instance`）：
//! - `install_platform()` 在启动时安装平台实现
//! - `platform()` 获取全局平台引用

use std::sync::{Arc, Mutex, OnceLock};

// ==================== 精灵接口族 ====================

/// 场景对象基础接口（C# `ISceneObject`）
pub trait ISprite: Send + Sync {
    /// 设置坐标
    fn set_position(&self, x: f32, y: f32, z: f32);
    /// 设置旋转（欧拉角）
    fn set_rotation(&self, x: f32, y: f32, z: f32);
    /// 设置缩放
    fn set_scale(&self, x: f32, y: f32, z: f32);
    /// 设置精灵图像
    fn set_graph(&self, graph_id: usize);
    /// 设置颜色 (r, g, b, a 各 0-255)
    fn set_color(&self, r: u8, g: u8, b: u8, a: u8);
    /// 销毁
    fn destroy(&self);
}

/// 九宫格精灵接口（C# `INineSliceSprite`）
pub trait INineSliceSprite: Send + Sync {
    /// 设置坐标
    fn set_position(&self, x: f32, y: f32, z: f32);
    /// 设置旋转
    fn set_rotation(&self, x: f32, y: f32, z: f32);
    /// 设置缩放
    fn set_scale(&self, x: f32, y: f32, z: f32);
    /// 设置图像与九宫参数
    fn set_graph(&self, graph_id: usize, base_size_x: f32, base_size_y: f32, left: f32, top: f32, right: f32, bottom: f32);
    /// 设置颜色
    fn set_color(&self, r: u8, g: u8, b: u8, a: u8);
    /// 设置 HSL 色偏
    fn set_hsl(&self, h: f32, s: f32, l: f32);
    /// 销毁
    fn destroy(&self);
}

/// 曲线精灵接口（C# `ICurveSprite`）
pub trait ICurveSprite: Send + Sync {
    /// 设置坐标
    fn set_position(&self, x: f32, y: f32, z: f32);
    /// 设置旋转
    fn set_rotation(&self, x: f32, y: f32, z: f32);
    /// 设置缩放
    fn set_scale(&self, x: f32, y: f32, z: f32);
    /// 设置曲线点坐标（对齐 C# `ICurveSprite.SetLine(ObjectArray)`：
    /// 平台接收完整点数组，而不是只有点数——只有点数时平台无法渲染曲线）
    fn set_points(&self, points: &[(f32, f32)]);
    /// 设置颜色
    fn set_color(&self, r: u8, g: u8, b: u8, a: u8);
    /// 设置线宽
    fn set_width(&self, width: f32);
    /// 销毁
    fn destroy(&self);
}

// ==================== 音频接口 ====================

/// 音频播放器接口（C# `IAudioPlayer`）
pub trait IAudioPlayer: Send + Sync {
    /// 设置音频
    fn set_audio(&self, audio_id: usize);
    /// 播放
    fn play(&self);
    /// 停止
    fn stop(&self);
    /// 音频时长（秒）
    fn audio_length(&self) -> f32;
    /// 是否正在播放
    fn is_playing(&self) -> bool;
    /// 设置播放进度
    fn set_time(&self, time: f32);
    /// 销毁
    fn destruct(&self);
}

/// 音效播放器接口（C# `IAudioEffectPlayer`）
///
/// 用于播放短音效（打击音等），比 IAudioPlayer 更轻量。
pub trait IAudioEffectPlayer: Send + Sync {
    /// 播放音效
    fn play(&self);
    /// 销毁
    fn destruct(&self);
}

// ==================== 平台根接口 ====================

/// 平台根接口（C# `IGorgeFrameworkBase`）
///
/// 宿主引擎需提供本 trait 的一个具体实现，注入框架运行时。
/// 方法签名对齐 C# `Base.Instance` 的工厂方法。
pub trait PlatformBase: Send + Sync {
    /// 创建普通精灵
    fn create_sprite(&self) -> Box<dyn ISprite>;

    /// 创建九宫格精灵
    fn create_nine_slice_sprite(&self) -> Box<dyn INineSliceSprite>;

    /// 创建曲线精灵
    fn create_curve_sprite(&self) -> Box<dyn ICurveSprite>;

    /// 创建音频播放器
    fn create_audio_player(&self) -> Box<dyn IAudioPlayer>;

    /// 创建音效播放器
    fn create_audio_effect_player(&self, audio_id: usize) -> Box<dyn IAudioEffectPlayer>;

    /// 从文件路径创建音频句柄（返回句柄 ID）
    ///
    /// 对齐 C# `CreateAudio(string)` 从路径加载。headless 实现只记录路径不解码。
    /// 返回的 usize 为平台内部句柄，用于后续 play/stop 等操作。
    fn create_audio(&self, path: &str) -> usize;

    /// 从字节数据创建图形资源（对齐 C# `CreateGraph(string, byte[])`）
    ///
    /// 返回资源句柄，失败返回错误信息。
    fn create_graph_from_data(&self, path: &str, data: &[u8]) -> Result<usize, String>;

    /// 从字节数据创建音频资源（对齐 C# `CreateAudio(string, byte[])`）
    ///
    /// 返回资源句柄，失败返回错误信息。
    fn create_audio_from_data(&self, path: &str, data: &[u8]) -> Result<usize, String>;

    /// 从字节数据创建视频资源（对齐 C# `CreateVideo(string, byte[])`）
    ///
    /// 返回资源句柄，失败返回错误信息。
    fn create_video_from_data(&self, path: &str, data: &[u8]) -> Result<usize, String>;

    /// 屏幕坐标转世界坐标（对齐 C# `Base.Instance.ScreenToWorldPoint(Vector3)`）
    fn screen_to_world_point(&self, x: f32, y: f32, z: f32) -> (f32, f32, f32);

    /// 获取视口尺寸
    fn viewport_size(&self) -> (f32, f32);

    /// 日志输出
    fn log(&self, message: &str);
}

// ==================== 全局平台单例 ====================

/// 全局平台实例（对齐 C# `Base.Instance` 静态单例）
static PLATFORM: OnceLock<Box<dyn PlatformBase>> = OnceLock::new();

/// 安装平台实现（应在框架初始化时调用一次）
///
/// 若平台已安装则静默忽略（支持测试中多次调用）。
pub fn install_platform(p: Box<dyn PlatformBase>) {
    let _ = PLATFORM.set(p);
}

/// 获取全局平台引用
pub fn platform() -> &'static dyn PlatformBase {
    PLATFORM.get().expect("平台未安装，请先调用 install_platform()").as_ref()
}

/// 检查平台是否已安装
pub fn platform_installed() -> bool {
    PLATFORM.get().is_some()
}

// ==================== Headless 平台实现 ====================

/// Headless 调用日志条目
#[derive(Debug, Clone, PartialEq)]
pub enum CallEntry {
    CreateSprite { sprite_id: usize },
    CreateNineSliceSprite { sprite_id: usize },
    CreateCurveSprite { sprite_id: usize },
    CreateAudioPlayer { player_id: usize },
    CreateAudioEffectPlayer { player_id: usize, audio_id: usize },
    CreateAudio { path: String, handle: usize },
    CreateGraphFromData { path: String, data_len: usize, handle: usize },
    CreateAudioFromData { path: String, data_len: usize, handle: usize },
    CreateVideoFromData { path: String, data_len: usize, handle: usize },
    ScreenToWorldPoint { x: f32, y: f32, z: f32, result: (f32, f32, f32) },
    SpriteSetPosition { sprite_id: usize, x: f32, y: f32, z: f32 },
    SpriteSetRotation { sprite_id: usize, x: f32, y: f32, z: f32 },
    SpriteSetScale { sprite_id: usize, x: f32, y: f32, z: f32 },
    SpriteSetGraph { sprite_id: usize, graph_id: usize },
    SpriteSetColor { sprite_id: usize, r: u8, g: u8, b: u8, a: u8 },
    SpriteDestroy { sprite_id: usize },
    NineSliceSetPosition { sprite_id: usize, x: f32, y: f32, z: f32 },
    NineSliceSetRotation { sprite_id: usize, x: f32, y: f32, z: f32 },
    NineSliceSetScale { sprite_id: usize, x: f32, y: f32, z: f32 },
    NineSliceSetGraph { sprite_id: usize, graph_id: usize, base_size_x: f32, base_size_y: f32, left: f32, top: f32, right: f32, bottom: f32 },
    NineSliceSetColor { sprite_id: usize, r: u8, g: u8, b: u8, a: u8 },
    NineSliceSetHsl { sprite_id: usize, h: f32, s: f32, l: f32 },
    NineSliceDestroy { sprite_id: usize },
    CurveSetPosition { sprite_id: usize, x: f32, y: f32, z: f32 },
    CurveSetRotation { sprite_id: usize, x: f32, y: f32, z: f32 },
    CurveSetScale { sprite_id: usize, x: f32, y: f32, z: f32 },
    CurveSetPoints { sprite_id: usize, points: Vec<(f32, f32)> },
    CurveSetColor { sprite_id: usize, r: u8, g: u8, b: u8, a: u8 },
    CurveSetWidth { sprite_id: usize, width: f32 },
    CurveDestroy { sprite_id: usize },
    AudioSetAudio { player_id: usize, audio_id: usize },
    AudioPlay { player_id: usize },
    AudioStop { player_id: usize },
    AudioSetTime { player_id: usize, time: f32 },
    AudioDestruct { player_id: usize },
    AudioEffectPlay { player_id: usize },
    AudioEffectDestruct { player_id: usize },
    Log { message: String },
}

/// Headless 平台实现
///
/// 所有操作记录到内部调用日志（`Vec<CallEntry>`），可通过 `calls()` 查询断言。
/// 精灵/音频句柄使用自增 ID 追踪。
pub struct HeadlessPlatform {
    calls: Arc<Mutex<Vec<CallEntry>>>,
    next_sprite_id: Mutex<usize>,
    next_player_id: Mutex<usize>,
    next_audio_handle: Mutex<usize>,
}

impl HeadlessPlatform {
    /// 创建新的 Headless 平台实例
    pub fn new() -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            next_sprite_id: Mutex::new(1),
            next_player_id: Mutex::new(1),
            next_audio_handle: Mutex::new(1),
        }
    }

    /// 获取调用日志的克隆
    pub fn calls(&self) -> Vec<CallEntry> {
        self.calls.lock().unwrap().clone()
    }

    /// 清空调用日志
    pub fn clear_calls(&self) {
        self.calls.lock().unwrap().clear();
    }

    pub fn call_log(&self) -> Arc<Mutex<Vec<CallEntry>>> {
        self.calls.clone()
    }

    fn record(&self, entry: CallEntry) {
        self.calls.lock().unwrap().push(entry);
    }

    fn alloc_sprite_id(&self) -> usize {
        let mut id = self.next_sprite_id.lock().unwrap();
        let sid = *id;
        *id += 1;
        sid
    }

    fn alloc_player_id(&self) -> usize {
        let mut id = self.next_player_id.lock().unwrap();
        let pid = *id;
        *id += 1;
        pid
    }

    fn alloc_audio_handle(&self) -> usize {
        let mut h = self.next_audio_handle.lock().unwrap();
        let handle = *h;
        *h += 1;
        handle
    }
}

impl PlatformBase for HeadlessPlatform {
    fn create_sprite(&self) -> Box<dyn ISprite> {
        let id = self.alloc_sprite_id();
        self.record(CallEntry::CreateSprite { sprite_id: id });
        Box::new(HeadlessSprite::new(id, self.call_log()))
    }

    fn create_nine_slice_sprite(&self) -> Box<dyn INineSliceSprite> {
        let id = self.alloc_sprite_id();
        self.record(CallEntry::CreateNineSliceSprite { sprite_id: id });
        Box::new(HeadlessNineSlice::new(id, self.call_log()))
    }

    fn create_curve_sprite(&self) -> Box<dyn ICurveSprite> {
        let id = self.alloc_sprite_id();
        self.record(CallEntry::CreateCurveSprite { sprite_id: id });
        Box::new(HeadlessCurve::new(id, self.call_log()))
    }

    fn create_audio_player(&self) -> Box<dyn IAudioPlayer> {
        let id = self.alloc_player_id();
        self.record(CallEntry::CreateAudioPlayer { player_id: id });
        Box::new(HeadlessAudio::new(id, self.call_log()))
    }

    fn create_audio_effect_player(&self, audio_id: usize) -> Box<dyn IAudioEffectPlayer> {
        let id = self.alloc_player_id();
        self.record(CallEntry::CreateAudioEffectPlayer { player_id: id, audio_id });
        Box::new(HeadlessAudioEffect::new(id, self.call_log()))
    }

    fn create_audio(&self, path: &str) -> usize {
        let handle = self.alloc_audio_handle();
        self.record(CallEntry::CreateAudio { path: path.to_string(), handle });
        handle
    }

    fn create_graph_from_data(&self, path: &str, data: &[u8]) -> Result<usize, String> {
        let handle = self.alloc_audio_handle();
        self.record(CallEntry::CreateGraphFromData { path: path.to_string(), data_len: data.len(), handle });
        Ok(handle)
    }

    fn create_audio_from_data(&self, path: &str, data: &[u8]) -> Result<usize, String> {
        let handle = self.alloc_audio_handle();
        self.record(CallEntry::CreateAudioFromData { path: path.to_string(), data_len: data.len(), handle });
        Ok(handle)
    }

    fn create_video_from_data(&self, path: &str, data: &[u8]) -> Result<usize, String> {
        let handle = self.alloc_audio_handle();
        self.record(CallEntry::CreateVideoFromData { path: path.to_string(), data_len: data.len(), handle });
        Ok(handle)
    }

    fn screen_to_world_point(&self, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        // Headless 无相机，直接返回原坐标
        let result = (x, y, z);
        self.record(CallEntry::ScreenToWorldPoint { x, y, z, result });
        result
    }

    fn viewport_size(&self) -> (f32, f32) {
        (1920.0, 1080.0)
    }

    fn log(&self, message: &str) {
        self.record(CallEntry::Log { message: message.to_string() });
    }
}

// ==================== Headless 精灵/音频具体实现 ====================

struct HeadlessSprite {
    id: usize,
    calls: Arc<Mutex<Vec<CallEntry>>>,
}

impl HeadlessSprite {
    fn new(id: usize, calls: Arc<Mutex<Vec<CallEntry>>>) -> Self {
        Self { id, calls }
    }
}

impl ISprite for HeadlessSprite {
    fn set_position(&self, x: f32, y: f32, z: f32) {
        self.calls.lock().unwrap().push(CallEntry::SpriteSetPosition { sprite_id: self.id, x, y, z });
    }
    fn set_rotation(&self, x: f32, y: f32, z: f32) {
        self.calls.lock().unwrap().push(CallEntry::SpriteSetRotation { sprite_id: self.id, x, y, z });
    }
    fn set_scale(&self, x: f32, y: f32, z: f32) {
        self.calls.lock().unwrap().push(CallEntry::SpriteSetScale { sprite_id: self.id, x, y, z });
    }
    fn set_graph(&self, graph_id: usize) {
        self.calls.lock().unwrap().push(CallEntry::SpriteSetGraph { sprite_id: self.id, graph_id });
    }
    fn set_color(&self, r: u8, g: u8, b: u8, a: u8) {
        self.calls.lock().unwrap().push(CallEntry::SpriteSetColor { sprite_id: self.id, r, g, b, a });
    }
    fn destroy(&self) {
        self.calls.lock().unwrap().push(CallEntry::SpriteDestroy { sprite_id: self.id });
    }
}

struct HeadlessNineSlice {
    id: usize,
    calls: Arc<Mutex<Vec<CallEntry>>>,
}

impl HeadlessNineSlice {
    fn new(id: usize, calls: Arc<Mutex<Vec<CallEntry>>>) -> Self {
        Self { id, calls }
    }
}

impl INineSliceSprite for HeadlessNineSlice {
    fn set_position(&self, x: f32, y: f32, z: f32) {
        self.calls.lock().unwrap().push(CallEntry::NineSliceSetPosition { sprite_id: self.id, x, y, z });
    }
    fn set_rotation(&self, x: f32, y: f32, z: f32) {
        self.calls.lock().unwrap().push(CallEntry::NineSliceSetRotation { sprite_id: self.id, x, y, z });
    }
    fn set_scale(&self, x: f32, y: f32, z: f32) {
        self.calls.lock().unwrap().push(CallEntry::NineSliceSetScale { sprite_id: self.id, x, y, z });
    }
    fn set_graph(&self, graph_id: usize, base_size_x: f32, base_size_y: f32, left: f32, top: f32, right: f32, bottom: f32) {
        self.calls.lock().unwrap().push(CallEntry::NineSliceSetGraph { sprite_id: self.id, graph_id, base_size_x, base_size_y, left, top, right, bottom });
    }
    fn set_color(&self, r: u8, g: u8, b: u8, a: u8) {
        self.calls.lock().unwrap().push(CallEntry::NineSliceSetColor { sprite_id: self.id, r, g, b, a });
    }
    fn set_hsl(&self, h: f32, s: f32, l: f32) {
        self.calls.lock().unwrap().push(CallEntry::NineSliceSetHsl { sprite_id: self.id, h, s, l });
    }
    fn destroy(&self) {
        self.calls.lock().unwrap().push(CallEntry::NineSliceDestroy { sprite_id: self.id });
    }
}

struct HeadlessCurve {
    id: usize,
    calls: Arc<Mutex<Vec<CallEntry>>>,
}

impl HeadlessCurve {
    fn new(id: usize, calls: Arc<Mutex<Vec<CallEntry>>>) -> Self {
        Self { id, calls }
    }
}

impl ICurveSprite for HeadlessCurve {
    fn set_position(&self, x: f32, y: f32, z: f32) {
        self.calls.lock().unwrap().push(CallEntry::CurveSetPosition { sprite_id: self.id, x, y, z });
    }
    fn set_rotation(&self, x: f32, y: f32, z: f32) {
        self.calls.lock().unwrap().push(CallEntry::CurveSetRotation { sprite_id: self.id, x, y, z });
    }
    fn set_scale(&self, x: f32, y: f32, z: f32) {
        self.calls.lock().unwrap().push(CallEntry::CurveSetScale { sprite_id: self.id, x, y, z });
    }
    fn set_points(&self, points: &[(f32, f32)]) {
        self.calls.lock().unwrap().push(CallEntry::CurveSetPoints {
            sprite_id: self.id,
            points: points.to_vec(),
        });
    }
    fn set_color(&self, r: u8, g: u8, b: u8, a: u8) {
        self.calls.lock().unwrap().push(CallEntry::CurveSetColor { sprite_id: self.id, r, g, b, a });
    }
    fn set_width(&self, width: f32) {
        self.calls.lock().unwrap().push(CallEntry::CurveSetWidth { sprite_id: self.id, width });
    }
    fn destroy(&self) {
        self.calls.lock().unwrap().push(CallEntry::CurveDestroy { sprite_id: self.id });
    }
}

struct HeadlessAudio {
    id: usize,
    calls: Arc<Mutex<Vec<CallEntry>>>,
}

impl HeadlessAudio {
    fn new(id: usize, calls: Arc<Mutex<Vec<CallEntry>>>) -> Self {
        Self { id, calls }
    }
}

impl IAudioPlayer for HeadlessAudio {
    fn set_audio(&self, audio_id: usize) {
        self.calls.lock().unwrap().push(CallEntry::AudioSetAudio { player_id: self.id, audio_id });
    }
    fn play(&self) {
        self.calls.lock().unwrap().push(CallEntry::AudioPlay { player_id: self.id });
    }
    fn stop(&self) {
        self.calls.lock().unwrap().push(CallEntry::AudioStop { player_id: self.id });
    }
    fn audio_length(&self) -> f32 { 0.0 }
    fn is_playing(&self) -> bool { false }
    fn set_time(&self, time: f32) {
        self.calls.lock().unwrap().push(CallEntry::AudioSetTime { player_id: self.id, time });
    }
    fn destruct(&self) {
        self.calls.lock().unwrap().push(CallEntry::AudioDestruct { player_id: self.id });
    }
}

struct HeadlessAudioEffect {
    id: usize,
    calls: Arc<Mutex<Vec<CallEntry>>>,
}

impl HeadlessAudioEffect {
    fn new(id: usize, calls: Arc<Mutex<Vec<CallEntry>>>) -> Self {
        Self { id, calls }
    }
}

impl IAudioEffectPlayer for HeadlessAudioEffect {
    fn play(&self) {
        self.calls.lock().unwrap().push(CallEntry::AudioEffectPlay { player_id: self.id });
    }
    fn destruct(&self) {
        self.calls.lock().unwrap().push(CallEntry::AudioEffectDestruct { player_id: self.id });
    }
}

// ==================== AssetBackend 适配器 ====================

/// `PlatformBase` 的 `AssetBackend` 适配器（F-1）。
///
/// 将 `SimulationScore::add_file_asset` 所需的 `AssetBackend` trait
/// 桥接到 `PlatformBase` 的 `create_graph_from_data` / `create_audio_from_data` / `create_video_from_data`。
pub struct PlatformAssetBackend<'a> {
    platform: &'a dyn PlatformBase,
}

impl<'a> PlatformAssetBackend<'a> {
    /// 从平台引用创建适配器
    pub fn new(platform: &'a dyn PlatformBase) -> Self {
        Self { platform }
    }
}

impl<'a> crate::chart::simulation_score::AssetBackend for PlatformAssetBackend<'a> {
    fn create_graph(&mut self, path: &str, data: &[u8]) -> Result<usize, String> {
        self.platform.create_graph_from_data(path, data)
    }

    fn create_audio(&mut self, path: &str, data: &[u8]) -> Result<usize, String> {
        self.platform.create_audio_from_data(path, data)
    }

    fn create_video(&mut self, path: &str, data: &[u8]) -> Result<usize, String> {
        self.platform.create_video_from_data(path, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // F-1 测试：PlatformBase 资源创建经 Headless 断言
    #[test]
    fn test_f1_headless_create_graph_from_data() {
        let hp = HeadlessPlatform::new();
        let data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header
        let result = hp.create_graph_from_data("test.png", &data);
        assert!(result.is_ok());
        let handle = result.unwrap();
        assert_eq!(handle, 1);
        let calls = hp.calls();
        assert!(matches!(&calls[0], CallEntry::CreateGraphFromData { path, data_len: 4, handle: 1 } if path == "test.png"));
    }

    #[test]
    fn test_f1_headless_create_audio_from_data() {
        let hp = HeadlessPlatform::new();
        let data = vec![0u8; 100];
        let result = hp.create_audio_from_data("bgm.wav", &data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        let calls = hp.calls();
        assert!(matches!(&calls[0], CallEntry::CreateAudioFromData { path, data_len: 100, handle: 1 } if path == "bgm.wav"));
    }

    #[test]
    fn test_f1_platform_asset_backend_adapter() {
        use crate::chart::simulation_score::AssetBackend;
        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));
        let mut backend = PlatformAssetBackend::new(platform());
        let png = vec![1, 2, 3];
        // 平台是全局共享单例，句柄绝对值取决于测试执行顺序，只断言拿到了非零句柄
        let handle = backend.create_graph("img.png", &png).unwrap();
        assert!(handle >= 1, "create_graph 应返回非零句柄");
    }

    #[test]
    fn test_f1_simulation_score_with_platform_backend() {
        use crate::chart::simulation_score::SimulationScore;
        use crate::chart::package::AssetFile;

        let mut score = SimulationScore::default();
        score.chart_asset_files.push(AssetFile::new("test.png".into(), vec![1, 2, 3], true));
        score.chart_asset_files.push(AssetFile::new("song.wav".into(), vec![4, 5, 6], true));

        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));
        let mut backend = PlatformAssetBackend::new(platform());
        score.add_file_asset(&mut backend);

        assert_eq!(score.asset_loaders.len(), 1);
        assert_eq!(score.asset_loaders[0].asset_sets[0].assets.len(), 2);
    }

    #[test]
    fn test_f1_headless_screen_to_world_point() {
        let hp = HeadlessPlatform::new();
        let result = hp.screen_to_world_point(100.0, 200.0, 0.0);
        assert_eq!(result, (100.0, 200.0, 0.0));
        let calls = hp.calls();
        assert!(matches!(&calls[0], CallEntry::ScreenToWorldPoint { x: 100.0, y: 200.0, z: 0.0, result: (100.0, 200.0, 0.0) }));
    }

    // ==================== 原有 Headless 测试 ====================
    #[test]
    fn test_headless_sprite_calls() {
        let hp = HeadlessPlatform::new();
        let sprite = hp.create_sprite();
        sprite.set_position(1.0, 2.0, 0.0);
        sprite.set_color(255, 0, 0, 255);
        sprite.set_graph(42);
        sprite.destroy();

        let calls = hp.calls();
        assert_eq!(calls.len(), 5);
        assert!(matches!(calls[0], CallEntry::CreateSprite { sprite_id: 1 }));
        assert!(matches!(calls[1], CallEntry::SpriteSetPosition { sprite_id: 1, x: 1.0, y: 2.0, z: 0.0 }));
        assert!(matches!(calls[2], CallEntry::SpriteSetColor { sprite_id: 1, r: 255, g: 0, b: 0, a: 255 }));
        assert!(matches!(calls[3], CallEntry::SpriteSetGraph { sprite_id: 1, graph_id: 42 }));
        assert!(matches!(calls[4], CallEntry::SpriteDestroy { sprite_id: 1 }));
    }

    #[test]
    fn test_headless_nine_slice_calls() {
        let hp = HeadlessPlatform::new();
        let ns = hp.create_nine_slice_sprite();
        ns.set_position(0.0, 0.0, 0.0);
        ns.set_hsl(0.5, 0.3, 0.2);
        ns.destroy();

        let calls = hp.calls();
        assert_eq!(calls.len(), 4);
        assert!(matches!(calls[0], CallEntry::CreateNineSliceSprite { sprite_id: 1 }));
        assert!(matches!(calls[3], CallEntry::NineSliceDestroy { sprite_id: 1 }));
    }

    #[test]
    fn test_headless_curve_calls() {
        let hp = HeadlessPlatform::new();
        let cs = hp.create_curve_sprite();
        cs.set_points(&[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]);
        cs.set_width(0.5);
        cs.set_color(100, 200, 50, 255);
        cs.destroy();

        let calls = hp.calls();
        assert_eq!(calls.len(), 5);
        assert!(matches!(calls[0], CallEntry::CreateCurveSprite { sprite_id: 1 }));
        assert!(matches!(&calls[1], CallEntry::CurveSetPoints { sprite_id: 1, points }
            if points == &vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]));
        assert!(matches!(calls[2], CallEntry::CurveSetWidth { sprite_id: 1, width: 0.5 }));
        assert!(matches!(calls[3], CallEntry::CurveSetColor { .. }));
        assert!(matches!(calls[4], CallEntry::CurveDestroy { sprite_id: 1 }));
    }

    #[test]
    fn test_headless_audio_calls() {
        let hp = HeadlessPlatform::new();
        let handle = hp.create_audio("test.wav");
        assert_eq!(handle, 1);

        let player = hp.create_audio_player();
        player.set_audio(handle);
        player.play();
        player.set_time(1.5);
        player.stop();
        player.destruct();

        let calls = hp.calls();
        assert_eq!(calls.len(), 7);
        assert!(matches!(&calls[0], CallEntry::CreateAudio { path, handle: 1 } if path == "test.wav"));
        assert!(matches!(calls[1], CallEntry::CreateAudioPlayer { player_id: 1 }));
        assert!(matches!(calls[6], CallEntry::AudioDestruct { player_id: 1 }));
    }

    #[test]
    fn test_platform_install_and_get() {
        let hp = HeadlessPlatform::new();
        install_platform(Box::new(hp));
        assert!(platform_installed());
        assert_eq!(platform().viewport_size(), (1920.0, 1080.0));
    }
}
