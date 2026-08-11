//! Macroquad 平台适配器
//!
//! 实现 `gorge_framework::adaptor` 中定义的全部平台接口，
//! 将 Gorge 框架桥接到 macroquad 2D 渲染引擎。
//!
//! # 架构
//!
//! 所有精灵/音频状态存储在 `Arc<Mutex<InnerState>>` 中。
//! 精灵在创建时持有该 Arc 的克隆，通过 trait 方法直接修改共享状态。
//! `render_all()` 在主循环中统一绘制所有活跃精灵。
//! macroquad 是 2D 引擎，因此 z 轴仅用于绘制排序，旋转仅 z 轴有效。

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use gorge_framework::adaptor::{
    IAudioEffectPlayer, IAudioPlayer, ICurveSprite, INineSliceSprite, ISprite, PlatformBase,
};
use macroquad::prelude::*;
use sasa::backend::cpal::{CpalBackend, CpalSettings};
use sasa::{AudioClip, AudioManager, Music, MusicParams, PlaySfxParams, Sfx};

// ==================== 精灵渲染状态 ====================

/// 普通精灵状态
struct SpriteState {
    position: (f32, f32, f32),
    rotation: (f32, f32, f32),
    scale: (f32, f32, f32),
    graph_id: usize,
    color: (u8, u8, u8, u8),
    alive: bool,
}

impl SpriteState {
    fn new() -> Self {
        Self {
            position: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            graph_id: 0,
            color: (255, 255, 255, 255),
            alive: true,
        }
    }
}

/// 九宫格精灵状态
struct NineSliceState {
    position: (f32, f32, f32),
    rotation: (f32, f32, f32),
    scale: (f32, f32, f32),
    graph_id: usize,
    base_size: (f32, f32),
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    color: (u8, u8, u8, u8),
    hsl: (f32, f32, f32),
    alive: bool,
}

impl NineSliceState {
    fn new() -> Self {
        Self {
            position: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            graph_id: 0,
            base_size: (100.0, 100.0),
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            color: (255, 255, 255, 255),
            hsl: (0.0, 0.0, 0.0),
            alive: true,
        }
    }
}

/// 曲线精灵状态
struct CurveState {
    position: (f32, f32, f32),
    rotation: (f32, f32, f32),
    scale: (f32, f32, f32),
    points: Vec<(f32, f32)>,
    color: (u8, u8, u8, u8),
    width: f32,
    alive: bool,
}

impl CurveState {
    fn new() -> Self {
        Self {
            position: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            points: Vec::new(),
            color: (255, 255, 255, 255),
            width: 2.0,
            alive: true,
        }
    }
}

/// 音频播放器状态
struct AudioPlayerState {
    audio_id: usize,
    alive: bool,
}

impl AudioPlayerState {
    fn new() -> Self {
        Self {
            audio_id: 0,
            alive: true,
        }
    }
}

/// 音效播放器状态
struct AudioEffectState {
    alive: bool,
}

impl AudioEffectState {
    fn new() -> Self {
        Self { alive: true }
    }
}

// ==================== 内部共享状态 ====================

struct InnerState {
    sprites: Vec<SpriteState>,
    nine_slices: Vec<NineSliceState>,
    curves: Vec<CurveState>,
    textures: HashMap<usize, Texture2D>,
    audio_players: Vec<AudioPlayerState>,
    audio_effects: Vec<AudioEffectState>,
    next_texture_id: usize,
    next_audio_id: usize,
    /// 音频资源表：audio_id -> 解码后的音频片段（供时长查询）
    audio_clips: HashMap<usize, AudioClip>,
    /// 音乐播放器表：audio_id -> 播放器（对应 IAudioPlayer 的底层）
    music_players: HashMap<usize, Music>,
    /// 音效播放器表：播放器索引 -> 播放器（对应 IAudioEffectPlayer 的底层；None 表示创建失败）
    sfx_players: HashMap<usize, Option<Sfx>>,
}

impl InnerState {
    fn new() -> Self {
        Self {
            sprites: Vec::new(),
            nine_slices: Vec::new(),
            curves: Vec::new(),
            textures: HashMap::new(),
            audio_players: Vec::new(),
            audio_effects: Vec::new(),
            next_texture_id: 1,
            next_audio_id: 1,
            audio_clips: HashMap::new(),
            music_players: HashMap::new(),
            sfx_players: HashMap::new(),
        }
    }

    fn alloc_texture_id(&mut self) -> usize {
        let id = self.next_texture_id;
        self.next_texture_id += 1;
        id
    }

    fn alloc_audio_id(&mut self) -> usize {
        let id = self.next_audio_id;
        self.next_audio_id += 1;
        id
    }
}

/// 供渲染循环访问的全局状态引用
static RENDER_STATE: OnceLock<Arc<Mutex<InnerState>>> = OnceLock::new();

// sasa 音频管理器单例（thread_local：cpal 设备流不要求 Send/Sync，仅主线程访问）。
// 惰性创建；无音频设备时保持 None，播放相关调用降级为 no-op。
thread_local! {
    static AUDIO_MANAGER: Mutex<Option<AudioManager>> = Mutex::new(None);
}

/// 惰性创建 sasa 音频管理器
///
/// 首次调用时通过 cpal 初始化默认音频输出设备；失败（如无音频设备）
/// 返回错误信息，调用方应降级处理。
fn ensure_audio_manager() -> Result<(), String> {
    AUDIO_MANAGER.with(|slot| {
        let mut guard = slot.lock().expect("音频管理器锁未被污染");
        if guard.is_none() {
            match AudioManager::new(CpalBackend::new(CpalSettings::default())) {
                Ok(manager) => *guard = Some(manager),
                Err(err) => return Err(format!("音频设备初始化失败: {err}")),
            }
        }
        Ok(())
    })
}

// ==================== 平台 ====================

/// Macroquad 平台实现
///
/// 实现 `PlatformBase` trait，可传入 `gorge_framework::adaptor::install_platform()`。
/// 内部通过 `Arc<Mutex<InnerState>>` 与精灵共享状态。
pub struct MacroquadPlatform {
    state: Arc<Mutex<InnerState>>,
}

impl MacroquadPlatform {
    /// 创建新的平台实例，并注册渲染状态供 `render_all()` 使用
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(InnerState::new()));
        RENDER_STATE.set(state.clone()).ok();
        Self { state }
    }

    /// 获取曲线精灵指定索引处的顶点（用于 native 类设置曲线点）
    pub fn get_curve_point(&self, curve_index: usize, point_index: usize) -> Option<(f32, f32)> {
        let state = self.state.lock().unwrap();
        state.curves.get(curve_index).and_then(|c| c.points.get(point_index).copied())
    }

    /// 设置曲线精灵指定索引处的顶点
    pub fn set_curve_point(&self, curve_index: usize, point_index: usize, x: f32, y: f32) {
        let mut state = self.state.lock().unwrap();
        if let Some(c) = state.curves.get_mut(curve_index) {
            if point_index < c.points.len() {
                c.points[point_index] = (x, y);
            }
        }
    }
}

// ==================== 全局渲染入口 ====================

/// 每帧调用，渲染所有活跃的 Gorge 精灵
///
/// 应在 `clear_background()` 之后、需要叠加 UI 之前调用。
/// 精灵按 z 轴排序确保正确的绘制顺序。
pub fn render_all() {
    let Some(state_lock) = RENDER_STATE.get() else {
        return;
    };
    let state = state_lock.lock().unwrap();

    // 收集所有活跃渲染对象，按 z 轴排序
    enum RenderKind {
        Sprite(usize),
        NineSlice(usize),
        Curve(usize),
    }
    struct RenderItem {
        z: f32,
        kind: RenderKind,
    }

    let mut items = Vec::new();

    for (i, s) in state.sprites.iter().enumerate() {
        if s.alive && s.graph_id != 0 {
            items.push(RenderItem { z: s.position.2, kind: RenderKind::Sprite(i) });
        }
    }
    for (i, ns) in state.nine_slices.iter().enumerate() {
        if ns.alive && ns.graph_id != 0 {
            items.push(RenderItem { z: ns.position.2, kind: RenderKind::NineSlice(i) });
        }
    }
    for (i, c) in state.curves.iter().enumerate() {
        if c.alive {
            items.push(RenderItem { z: c.position.2, kind: RenderKind::Curve(i) });
        }
    }

    items.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap_or(std::cmp::Ordering::Equal));

    for item in &items {
        match item.kind {
            RenderKind::Sprite(i) => render_sprite(&state, i),
            RenderKind::NineSlice(i) => render_nine_slice(&state, i),
            RenderKind::Curve(i) => render_curve(&state, i),
        }
    }
}

/// 返回当前平台中的纹理和存活渲染对象数量。
///
/// 用于启动诊断，帮助区分“谱面未创生”和“已创生但平台未收到图形对象”。
pub fn render_resource_counts() -> (usize, usize, usize, usize) {
    let Some(state_lock) = RENDER_STATE.get() else {
        return (0, 0, 0, 0);
    };
    let state = state_lock.lock().unwrap();
    (
        state.textures.len(),
        state.sprites.iter().filter(|sprite| sprite.alive).count(),
        state
            .nine_slices
            .iter()
            .filter(|sprite| sprite.alive)
            .count(),
        state.curves.iter().filter(|curve| curve.alive).count(),
    )
}

/// 返回当前平台中的音频资源与播放器数量
///
/// 用于启动诊断，验证音频资产是否真实解码注册到平台。
/// 返回 (音频资源数, 音乐播放器数, 音效播放器数)。
pub fn audio_resource_counts() -> (usize, usize, usize) {
    let Some(state_lock) = RENDER_STATE.get() else {
        return (0, 0, 0);
    };
    let state = state_lock.lock().unwrap();
    (
        state.audio_clips.len(),
        state.music_players.len(),
        state.sfx_players.len(),
    )
}

/// 返回音频播放器运行状态摘要（开发诊断用）
///
/// 逐音乐播放器报告暂停状态与播放位置，用于验证 SongSimulator
/// 是否真实触发播放。格式：`music[id]=paused:true|false,pos:0.00`。
pub fn audio_playback_diagnostics() -> String {
    let Some(state_lock) = RENDER_STATE.get() else {
        return "无平台状态".to_string();
    };
    let mut state = state_lock.lock().unwrap();
    let mut parts: Vec<String> = Vec::new();
    let mut ids: Vec<usize> = state.music_players.keys().copied().collect();
    ids.sort_unstable();
    for id in ids {
        if let Some(music) = state.music_players.get_mut(&id) {
            parts.push(format!("music[{id}]=paused:{},pos:{:.3}", music.paused(), music.position()));
        }
    }
    if parts.is_empty() {
        return "无音乐播放器".to_string();
    }
    parts.join(" ")
}

// ==================== 渲染函数 ====================

fn to_macroquad_color(c: (u8, u8, u8, u8)) -> Color {
    Color::from_rgba(c.0, c.1, c.2, c.3)
}

fn render_sprite(state: &InnerState, index: usize) {
    let s = &state.sprites[index];
    let tex = match state.textures.get(&s.graph_id) {
        Some(t) => t,
        None => return,
    };
    draw_texture_ex(
        tex,
        s.position.0,
        s.position.1,
        to_macroquad_color(s.color),
        DrawTextureParams {
            dest_size: Some(Vec2::new(tex.width() * s.scale.0, tex.height() * s.scale.1)),
            rotation: s.rotation.2,
            ..Default::default()
        },
    );
}

fn render_nine_slice(state: &InnerState, index: usize) {
    let ns = &state.nine_slices[index];
    let tex = match state.textures.get(&ns.graph_id) {
        Some(t) => t,
        None => return,
    };

    let color = to_macroquad_color(ns.color);
    let target_w = ns.base_size.0 * ns.scale.0;
    let target_h = ns.base_size.1 * ns.scale.1;
    let px = ns.position.0;
    let py = ns.position.1;
    let rot = ns.rotation.2;

    // 九个源区域（纹理空间）
    #[rustfmt::skip]
    let src = [
        (0.0,                                0.0,                                  ns.left,  ns.top),
        (ns.left,                            0.0,                                  ns.base_size.0 - ns.left - ns.right, ns.top),
        (ns.base_size.0 - ns.right,          0.0,                                  ns.right, ns.top),
        (0.0,                                ns.top,                               ns.left,  ns.base_size.1 - ns.top - ns.bottom),
        (ns.left,                            ns.top,                               ns.base_size.0 - ns.left - ns.right, ns.base_size.1 - ns.top - ns.bottom),
        (ns.base_size.0 - ns.right,          ns.top,                               ns.right, ns.base_size.1 - ns.top - ns.bottom),
        (0.0,                                ns.base_size.1 - ns.bottom,           ns.left,  ns.bottom),
        (ns.left,                            ns.base_size.1 - ns.bottom,           ns.base_size.0 - ns.left - ns.right, ns.bottom),
        (ns.base_size.0 - ns.right,          ns.base_size.1 - ns.bottom,           ns.right, ns.bottom),
    ];

    // 目标尺寸
    let left_w = ns.left * ns.scale.0;
    let right_w = ns.right * ns.scale.0;
    let top_h = ns.top * ns.scale.1;
    let bottom_h = ns.bottom * ns.scale.1;
    let ctr_w = (target_w - left_w - right_w).max(0.0);
    let ctr_h = (target_h - top_h - bottom_h).max(0.0);

    let positions = [
        (0.0, 0.0), (left_w, 0.0), (left_w + ctr_w, 0.0),
        (0.0, top_h), (left_w, top_h), (left_w + ctr_w, top_h),
        (0.0, top_h + ctr_h), (left_w, top_h + ctr_h), (left_w + ctr_w, top_h + ctr_h),
    ];
    let sizes = [
        (left_w, top_h), (ctr_w, top_h), (right_w, top_h),
        (left_w, ctr_h), (ctr_w, ctr_h), (right_w, ctr_h),
        (left_w, bottom_h), (ctr_w, bottom_h), (right_w, bottom_h),
    ];

    // 旋转中心
    let cx = px + target_w / 2.0;
    let cy = py + target_h / 2.0;
    let (sin_r, cos_r) = rot.sin_cos();

    for i in 0..9 {
        let (sx, sy, sw, sh) = src[i];
        let (dx, dy) = positions[i];
        let (dw, dh) = sizes[i];
        if dw <= 0.0 || dh <= 0.0 || sw <= 0.0 || sh <= 0.0 {
            continue;
        }

        // 绕整体中心旋转每块的位置
        let piece_cx = px + dx + dw / 2.0 - cx;
        let piece_cy = py + dy + dh / 2.0 - cy;
        let rx = piece_cx * cos_r - piece_cy * sin_r + cx - dw / 2.0;
        let ry = piece_cx * sin_r + piece_cy * cos_r + cy - dh / 2.0;

        draw_texture_ex(
            tex,
            rx,
            ry,
            color,
            DrawTextureParams {
                source: Some(Rect::new(sx, sy, sw, sh)),
                dest_size: Some(Vec2::new(dw, dh)),
                ..Default::default()
            },
        );
    }
}

fn render_curve(state: &InnerState, index: usize) {
    let c = &state.curves[index];
    let color = to_macroquad_color(c.color);
    // 一次性诊断：第一条非零曲线的 y 范围（区分直线与真实曲线）
    use std::sync::atomic::{AtomicBool, Ordering};
    static SHAPE_LOGGED: AtomicBool = AtomicBool::new(false);
    if !SHAPE_LOGGED.load(Ordering::Relaxed) {
        let (mut min_y, mut max_y, mut min_x, mut max_x) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
        for (x, y) in &c.points {
            min_y = min_y.min(*y); max_y = max_y.max(*y);
            min_x = min_x.min(*x); max_x = max_x.max(*x);
        }
        if max_y > min_y || max_x > min_x {
            SHAPE_LOGGED.store(true, Ordering::Relaxed);
            eprintln!(
                "[Gorge] 曲线形状诊断 idx={} x=[{:.2},{:.2}] y=[{:.2},{:.2}] 点数={}",
                index, min_x, max_x, min_y, max_y, c.points.len(),
            );
        }
    }
    // 线宽与坐标同尺度缩放（谱面 width 单位，如 0.1），并保底 1px，
    // 否则默认 0.1px 的判定线在屏幕上不可见。
    let line_width = (c.width * 64.0).max(1.0);
    // 谱面坐标 → 屏幕坐标（平台适配）：
    // 判定线谱面 x 约 -10~10（横贯）、y 为曲线偏移（-25~163），
    // 以 1280x720 视口中心为原点：x 每单位 64px、y 每单位 4px（y 向下）。
    let center_x = 640.0;
    let center_y = 360.0;
    let scale_x = 64.0;
    let scale_y = 4.0;
    for j in 1..c.points.len() {
        let (x0, y0) = c.points[j - 1];
        let (x1, y1) = c.points[j];
        draw_line(
            center_x + (c.position.0 + x0) * scale_x,
            center_y + (c.position.1 + y0) * scale_y,
            center_x + (c.position.0 + x1) * scale_x,
            center_y + (c.position.1 + y1) * scale_y,
            line_width,
            color,
        );
    }
}

// ==================== 精灵实现 ====================

struct MacroquadSprite {
    state: Arc<Mutex<InnerState>>,
    index: usize,
}

impl ISprite for MacroquadSprite {
    fn set_position(&self, x: f32, y: f32, z: f32) {
        if let Some(s) = self.state.lock().unwrap().sprites.get_mut(self.index) {
            s.position = (x, y, z);
        }
    }

    fn set_rotation(&self, x: f32, y: f32, z: f32) {
        if let Some(s) = self.state.lock().unwrap().sprites.get_mut(self.index) {
            s.rotation = (x, y, z);
        }
    }

    fn set_scale(&self, x: f32, y: f32, z: f32) {
        if let Some(s) = self.state.lock().unwrap().sprites.get_mut(self.index) {
            s.scale = (x, y, z);
        }
    }

    fn set_graph(&self, graph_id: usize) {
        if let Some(s) = self.state.lock().unwrap().sprites.get_mut(self.index) {
            s.graph_id = graph_id;
        }
    }

    fn set_color(&self, r: u8, g: u8, b: u8, a: u8) {
        if let Some(s) = self.state.lock().unwrap().sprites.get_mut(self.index) {
            s.color = (r, g, b, a);
        }
    }

    fn destroy(&self) {
        if let Some(s) = self.state.lock().unwrap().sprites.get_mut(self.index) {
            s.alive = false;
        }
    }
}

struct MacroquadNineSlice {
    state: Arc<Mutex<InnerState>>,
    index: usize,
}

impl INineSliceSprite for MacroquadNineSlice {
    fn set_position(&self, x: f32, y: f32, z: f32) {
        if let Some(ns) = self.state.lock().unwrap().nine_slices.get_mut(self.index) {
            ns.position = (x, y, z);
        }
    }

    fn set_rotation(&self, x: f32, y: f32, z: f32) {
        if let Some(ns) = self.state.lock().unwrap().nine_slices.get_mut(self.index) {
            ns.rotation = (x, y, z);
        }
    }

    fn set_scale(&self, x: f32, y: f32, z: f32) {
        if let Some(ns) = self.state.lock().unwrap().nine_slices.get_mut(self.index) {
            ns.scale = (x, y, z);
        }
    }

    fn set_graph(
        &self,
        graph_id: usize,
        base_size_x: f32,
        base_size_y: f32,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    ) {
        if let Some(ns) = self.state.lock().unwrap().nine_slices.get_mut(self.index) {
            ns.graph_id = graph_id;
            ns.base_size = (base_size_x, base_size_y);
            ns.left = left;
            ns.top = top;
            ns.right = right;
            ns.bottom = bottom;
        }
    }

    fn set_color(&self, r: u8, g: u8, b: u8, a: u8) {
        if let Some(ns) = self.state.lock().unwrap().nine_slices.get_mut(self.index) {
            ns.color = (r, g, b, a);
        }
    }

    fn set_hsl(&self, h: f32, s: f32, l: f32) {
        if let Some(ns) = self.state.lock().unwrap().nine_slices.get_mut(self.index) {
            ns.hsl = (h, s, l);
        }
    }

    fn destroy(&self) {
        if let Some(ns) = self.state.lock().unwrap().nine_slices.get_mut(self.index) {
            ns.alive = false;
        }
    }
}

struct MacroquadCurve {
    state: Arc<Mutex<InnerState>>,
    index: usize,
}

impl ICurveSprite for MacroquadCurve {
    fn set_position(&self, x: f32, y: f32, z: f32) {
        if let Some(c) = self.state.lock().unwrap().curves.get_mut(self.index) {
            c.position = (x, y, z);
        }
    }

    fn set_rotation(&self, x: f32, y: f32, z: f32) {
        if let Some(c) = self.state.lock().unwrap().curves.get_mut(self.index) {
            c.rotation = (x, y, z);
        }
    }

    fn set_scale(&self, x: f32, y: f32, z: f32) {
        if let Some(c) = self.state.lock().unwrap().curves.get_mut(self.index) {
            c.scale = (x, y, z);
        }
    }

    fn set_points(&self, points: &[(f32, f32)]) {
        if let Some(c) = self.state.lock().unwrap().curves.get_mut(self.index) {
            c.points = points.to_vec();
        }
    }

    fn set_color(&self, r: u8, g: u8, b: u8, a: u8) {
        if let Some(c) = self.state.lock().unwrap().curves.get_mut(self.index) {
            c.color = (r, g, b, a);
        }
    }

    fn set_width(&self, width: f32) {
        if let Some(c) = self.state.lock().unwrap().curves.get_mut(self.index) {
            c.width = width;
        }
    }

    fn destroy(&self) {
        if let Some(c) = self.state.lock().unwrap().curves.get_mut(self.index) {
            c.alive = false;
        }
    }
}

// ==================== 音频实现 ====================

struct MacroquadAudio {
    state: Arc<Mutex<InnerState>>,
    index: usize,
}

impl IAudioPlayer for MacroquadAudio {
    fn set_audio(&self, audio_id: usize) {
        if let Some(ap) = self.state.lock().unwrap().audio_players.get_mut(self.index) {
            ap.audio_id = audio_id;
        }
    }

    fn play(&self) {
        let mut state = self.state.lock().unwrap();
        let audio_id = state.audio_players.get(self.index).map(|ap| ap.audio_id).unwrap_or(0);
        if audio_id != 0 {
            if let Some(music) = state.music_players.get_mut(&audio_id) {
                // sasa 播放：从当前（或 set_time 设置的）位置继续
                if let Err(err) = music.play() {
                    eprintln!("[Gorge] 音频播放失败: {err}");
                }
            }
        }
    }

    fn stop(&self) {
        let mut state = self.state.lock().unwrap();
        let audio_id = state.audio_players.get(self.index).map(|ap| ap.audio_id).unwrap_or(0);
        if audio_id != 0 {
            if let Some(music) = state.music_players.get_mut(&audio_id) {
                if let Err(err) = music.pause() {
                    eprintln!("[Gorge] 音频暂停失败: {err}");
                }
            }
        }
    }

    /// 音频时长（秒）：直接取自解码后的音频数据
    fn audio_length(&self) -> f32 {
        let state = self.state.lock().unwrap();
        let audio_id = state.audio_players.get(self.index).map(|ap| ap.audio_id).unwrap_or(0);
        state.audio_clips.get(&audio_id).map(|clip| clip.length()).unwrap_or(0.0)
    }

    /// 是否正在播放：查询 sasa 播放器的真实暂停状态
    fn is_playing(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        let audio_id = state.audio_players.get(self.index).map(|ap| ap.audio_id).unwrap_or(0);
        if audio_id != 0 {
            if let Some(music) = state.music_players.get_mut(&audio_id) {
                return !music.paused();
            }
        }
        false
    }

    /// 设置播放进度：sasa 支持真实 seek
    fn set_time(&self, time: f32) {
        let mut state = self.state.lock().unwrap();
        let audio_id = state.audio_players.get(self.index).map(|ap| ap.audio_id).unwrap_or(0);
        if audio_id != 0 {
            if let Some(music) = state.music_players.get_mut(&audio_id) {
                if let Err(err) = music.seek_to(time) {
                    eprintln!("[Gorge] 音频 seek 失败: {err}");
                }
            }
        }
    }

    fn destruct(&self) {
        if let Some(ap) = self.state.lock().unwrap().audio_players.get_mut(self.index) {
            ap.alive = false;
        }
    }
}

struct MacroquadAudioEffect {
    state: Arc<Mutex<InnerState>>,
    index: usize,
}

impl IAudioEffectPlayer for MacroquadAudioEffect {
    fn play(&self) {
        let mut state = self.state.lock().unwrap();
        if let Some(Some(sfx)) = state.sfx_players.get_mut(&self.index) {
            // 音效每次从 0 位置播放（可叠加）
            if let Err(err) = sfx.play(PlaySfxParams::default()) {
                eprintln!("[Gorge] 音效播放失败: {err}");
            }
        }
    }

    fn destruct(&self) {
        if let Some(ae) = self.state.lock().unwrap().audio_effects.get_mut(self.index) {
            ae.alive = false;
        }
    }
}

// ==================== PlatformBase 实现 ====================

impl PlatformBase for MacroquadPlatform {
    fn create_sprite(&self) -> Box<dyn ISprite> {
        let mut state = self.state.lock().unwrap();
        let index = state.sprites.len();
        state.sprites.push(SpriteState::new());
        Box::new(MacroquadSprite { state: self.state.clone(), index })
    }

    fn create_nine_slice_sprite(&self) -> Box<dyn INineSliceSprite> {
        let mut state = self.state.lock().unwrap();
        let index = state.nine_slices.len();
        state.nine_slices.push(NineSliceState::new());
        Box::new(MacroquadNineSlice { state: self.state.clone(), index })
    }

    fn create_curve_sprite(&self) -> Box<dyn ICurveSprite> {
        let mut state = self.state.lock().unwrap();
        let index = state.curves.len();
        state.curves.push(CurveState::new());
        Box::new(MacroquadCurve { state: self.state.clone(), index })
    }

    fn create_audio_player(&self) -> Box<dyn IAudioPlayer> {
        let mut state = self.state.lock().unwrap();
        let index = state.audio_players.len();
        state.audio_players.push(AudioPlayerState::new());
        Box::new(MacroquadAudio { state: self.state.clone(), index })
    }

    fn create_audio_effect_player(&self, audio_id: usize) -> Box<dyn IAudioEffectPlayer> {
        let mut state = self.state.lock().unwrap();
        let index = state.audio_effects.len();
        // 音频资源已就绪且设备可用时创建 sasa 音效播放器；否则保持 None（播放为 no-op）
        let clip = state.audio_clips.get(&audio_id).cloned();
        let sfx = clip.and_then(|c| {
            ensure_audio_manager().ok()?;
            AUDIO_MANAGER
                .with(|slot| slot.lock().expect("音频管理器锁未被污染").as_mut()?.create_sfx(c, None).ok())
        });
        state.sfx_players.insert(index, sfx);
        state.audio_effects.push(AudioEffectState::new());
        Box::new(MacroquadAudioEffect { state: self.state.clone(), index })
    }

    fn create_audio(&self, path: &str) -> usize {
        // 从磁盘读取音频文件字节后复用字节加载路径（资源包解压到磁盘时可用）
        match std::fs::read(path) {
            Ok(data) => self.create_audio_from_data(path, &data).unwrap_or(0),
            Err(err) => {
                eprintln!("[Gorge] 音频文件读取失败 {path}: {err}");
                0
            }
        }
    }

    fn create_graph_from_data(&self, _path: &str, data: &[u8]) -> Result<usize, String> {
        let tex = Texture2D::from_file_with_format(data, None);
        let mut state = self.state.lock().unwrap();
        let id = state.alloc_texture_id();
        state.textures.insert(id, tex);
        Ok(id)
    }

    fn create_audio_from_data(&self, _path: &str, data: &[u8]) -> Result<usize, String> {
        // sasa 用 symphonia 同步解码（支持 WAV/MP3/FLAC/OGG 等），无需异步上下文
        let clip = AudioClip::new(data.to_vec()).map_err(|err| format!("音频解码失败: {err}"))?;
        ensure_audio_manager()?;
        let music = AUDIO_MANAGER.with(|slot| {
            slot.lock()
                .expect("音频管理器锁未被污染")
                .as_mut()
                .expect("音频管理器已初始化")
                .create_music(clip.clone(), MusicParams::default())
                .map_err(|err| format!("音频播放器创建失败: {err}"))
        })?;
        let mut state = self.state.lock().unwrap();
        let id = state.alloc_audio_id();
        state.audio_clips.insert(id, clip);
        state.music_players.insert(id, music);
        Ok(id)
    }

    fn create_video_from_data(&self, path: &str, data: &[u8]) -> Result<usize, String> {
        // macroquad 无视频解码能力：识别常见视频容器魔数并返回明确错误
        if is_video_data(data) {
            return Err(format!("macroquad 无视频解码能力，视频资源不受支持: {path}"));
        }
        self.create_graph_from_data(path, data)
    }

    fn screen_to_world_point(&self, x: f32, y: f32, _z: f32) -> (f32, f32, f32) {
        // 2D 引擎无相机变换，屏幕坐标即世界坐标
        (x, y, 0.0)
    }

    fn viewport_size(&self) -> (f32, f32) {
        (screen_width(), screen_height())
    }

    fn log(&self, message: &str) {
        println!("[Gorge] {}", message);
    }
}

// ==================== 视频格式识别 ====================

/// 识别常见视频容器格式魔数（MP4/MOV/WebM/MKV/AVI/FLV/WMV）
///
/// macroquad 无视频解码能力，`create_video_from_data` 据此提前识别
/// 视频资源并返回明确错误，避免把视频字节当作图片纹理加载。
fn is_video_data(data: &[u8]) -> bool {
    // MP4 / MOV / M4A：偏移 4 处为 `ftyp` 品牌标识
    if data.len() >= 8 && &data[4..8] == b"ftyp" {
        return true;
    }
    // WebM / Matroska（MKV）：EBML 头
    if data.len() >= 4 && data[0..4] == [0x1A, 0x45, 0xDF, 0xA3] {
        return true;
    }
    // AVI：RIFF 容器 + AVI 类型标识
    if data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"AVI " {
        return true;
    }
    // FLV
    if data.len() >= 3 && &data[0..3] == b"FLV" {
        return true;
    }
    // WMV / ASF：ASF GUID 头
    if data.len() >= 16
        && data[0..16]
            == [0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C]
    {
        return true;
    }
    false
}

// ==================== 安装辅助 ====================

/// 创建并安装 Macroquad 平台到 Gorge 框架
///
/// 同时将渲染状态注册到全局，使 `render_all()` 可用。
pub fn install_macroquad_platform() {
    let platform = MacroquadPlatform::new();
    // 确保 RENDER_STATE 已设置（new() 中已设，这里是双保险）
    if RENDER_STATE.get().is_none() {
        RENDER_STATE.set(platform.state.clone()).ok();
    }
    gorge_framework::adaptor::install_platform(Box::new(platform));
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sprite_and_set_properties() {
        let platform = MacroquadPlatform::new();

        let sprite = platform.create_sprite();
        sprite.set_position(100.0, 200.0, 5.0);
        sprite.set_color(255, 0, 0, 128);
        sprite.set_rotation(0.0, 0.0, 1.5);
        sprite.set_scale(2.0, 3.0, 1.0);
        sprite.set_graph(42);

        let state = platform.state.lock().unwrap();
        let s = &state.sprites[0];
        assert_eq!(s.position, (100.0, 200.0, 5.0));
        assert_eq!(s.color, (255, 0, 0, 128));
        assert_eq!(s.rotation, (0.0, 0.0, 1.5));
        assert_eq!(s.scale, (2.0, 3.0, 1.0));
        assert_eq!(s.graph_id, 42);
        assert!(s.alive);
    }

    #[test]
    fn test_destroy_sprite() {
        let platform = MacroquadPlatform::new();
        let sprite = platform.create_sprite();
        assert!(platform.state.lock().unwrap().sprites[0].alive);

        sprite.destroy();
        assert!(!platform.state.lock().unwrap().sprites[0].alive);
    }

    #[test]
    fn test_create_nine_slice_sprite() {
        let platform = MacroquadPlatform::new();

        let ns = platform.create_nine_slice_sprite();
        ns.set_position(10.0, 20.0, 0.0);
        ns.set_graph(1, 200.0, 100.0, 8.0, 8.0, 8.0, 8.0);
        ns.set_color(100, 200, 50, 255);
        ns.set_hsl(0.5, 0.3, 0.2);

        let state = platform.state.lock().unwrap();
        let n = &state.nine_slices[0];
        assert_eq!(n.position, (10.0, 20.0, 0.0));
        assert_eq!(n.graph_id, 1);
        assert_eq!(n.base_size, (200.0, 100.0));
        assert_eq!(n.left, 8.0);
        assert_eq!(n.right, 8.0);
        assert_eq!(n.color, (100, 200, 50, 255));
        assert_eq!(n.hsl, (0.5, 0.3, 0.2));
    }

    #[test]
    fn test_create_curve_sprite() {
        let platform = MacroquadPlatform::new();

        let cs = platform.create_curve_sprite();
        cs.set_points(&[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]);
        cs.set_width(5.0);
        cs.set_color(50, 100, 200, 255);
        cs.set_position(0.0, 0.0, 1.0);

        {
            let state = platform.state.lock().unwrap();
            let c = &state.curves[0];
            assert_eq!(c.points, vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]);
            assert_eq!(c.width, 5.0);
            assert_eq!(c.color, (50, 100, 200, 255));
            assert_eq!(c.position, (0.0, 0.0, 1.0));
            assert!(c.alive);
        }

        // 设置曲线点（需在锁释放后调用，避免死锁）
        platform.set_curve_point(0, 0, 10.0, 20.0);
        platform.set_curve_point(0, 1, 30.0, 40.0);
        platform.set_curve_point(0, 2, 50.0, 60.0);
        let points = platform.get_curve_point(0, 1);
        assert_eq!(points, Some((30.0, 40.0)));
    }

    #[test]
    fn test_platform_base_trait_creation() {
        let platform = MacroquadPlatform::new();

        let _sprite: Box<dyn ISprite> = platform.create_sprite();
        let _nine: Box<dyn INineSliceSprite> = platform.create_nine_slice_sprite();
        let _curve: Box<dyn ICurveSprite> = platform.create_curve_sprite();
        let _audio: Box<dyn IAudioPlayer> = platform.create_audio_player();
        let _effect: Box<dyn IAudioEffectPlayer> = platform.create_audio_effect_player(1);
    }

    #[test]
    fn test_screen_to_world_point() {
        let platform = MacroquadPlatform::new();
        let result = platform.screen_to_world_point(100.0, 200.0, 10.0);
        assert_eq!(result, (100.0, 200.0, 0.0));
    }

    #[test]
    fn test_log() {
        let platform = MacroquadPlatform::new();
        platform.log("test message");
    }

    #[test]
    fn test_multiple_sprites() {
        let platform = MacroquadPlatform::new();

        let s1 = platform.create_sprite();
        let s2 = platform.create_sprite();
        s1.set_position(0.0, 0.0, 0.0);
        s2.set_position(100.0, 100.0, 1.0);

        let state = platform.state.lock().unwrap();
        assert_eq!(state.sprites.len(), 2);
        assert_eq!(state.sprites[0].position, (0.0, 0.0, 0.0));
        assert_eq!(state.sprites[1].position, (100.0, 100.0, 1.0));
    }

    #[test]
    fn test_install_macroquad_platform_sets_render_state() {
        // RENDER_STATE 是全局单例，可能已被其他测试设置
        // 仅验证 new() 会尝试设置
        let _platform = MacroquadPlatform::new();
        assert!(RENDER_STATE.get().is_some());
    }

    /// 构造最小标准 WAV 文件字节（44 字节头 + 纯 PCM 采样）
    ///
    /// 参数：采样数、采样率、声道数、位深（8/16）、音频格式（1=PCM）
    fn make_wav_bytes(sample_count: u32, sample_rate: u32, channels: u16, bits: u16) -> Vec<u8> {
        let bytes_per_sample = (bits / 8) as u32;
        let block_align: u16 = (channels as u32 * bytes_per_sample) as u16;
        let byte_rate = sample_rate * block_align as u32;
        let data_size = sample_count * block_align as u32;
        let mut bytes = Vec::with_capacity(44 + data_size as usize);
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&(36 + data_size).to_le_bytes());
        bytes.extend_from_slice(b"WAVE");
        bytes.extend_from_slice(b"fmt ");
        bytes.extend_from_slice(&16u32.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes()); // PCM
        bytes.extend_from_slice(&channels.to_le_bytes());
        bytes.extend_from_slice(&sample_rate.to_le_bytes());
        bytes.extend_from_slice(&byte_rate.to_le_bytes());
        bytes.extend_from_slice(&block_align.to_le_bytes());
        bytes.extend_from_slice(&bits.to_le_bytes());
        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&data_size.to_le_bytes());
        // 填充静音采样（8 位无符号 / 16 位有符号的小端字节）
        let sample_bytes: [u8; 2] = if bits == 8 { [0x00, 0x00] } else { 0i16.to_le_bytes() };
        for _ in 0..data_size / bytes_per_sample {
            bytes.extend_from_slice(&sample_bytes);
        }
        bytes
    }

    #[test]
    fn test_is_video_data_recognizes_video_magic_numbers() {
        // MP4：偏移 4 处为 ftyp
        let mp4 = [0x00, 0x00, 0x00, 0x18, b'f', b't', b'y', b'p', b'i', b's', b'o', b'm'];
        assert!(is_video_data(&mp4));
        // WebM / MKV：EBML 头
        assert!(is_video_data(&[0x1A, 0x45, 0xDF, 0xA3, 0x9F, 0x42, 0x86]));
        // AVI
        let mut avi = Vec::new();
        avi.extend_from_slice(b"RIFF");
        avi.extend_from_slice(&0u32.to_le_bytes());
        avi.extend_from_slice(b"AVI ");
        assert!(is_video_data(&avi));
        // FLV
        assert!(is_video_data(b"FLV\x01"));
        // WMV / ASF GUID
        let wmv = [0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C];
        assert!(is_video_data(&wmv));
    }

    #[test]
    fn test_is_video_data_rejects_non_video_data() {
        // WAV 音频
        assert!(!is_video_data(&make_wav_bytes(100, 8000, 1, 16)));
        // PNG 魔数
        assert!(!is_video_data(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]));
        // 空数据 / 过短数据
        assert!(!is_video_data(&[]));
        assert!(!is_video_data(&[0x1A, 0x45]));
        // 无头 RIFF（无 AVI 标识）
        assert!(!is_video_data(b"RIFF\x00\x00\x00\x00WAVE"));
    }

    #[test]
    fn test_audio_clip_decodes_wav_length() {
        // 8000Hz 单声道 16 位，1000 采样 → 0.125 秒
        let wav = make_wav_bytes(1000, 8000, 1, 16);
        let clip = AudioClip::new(wav).expect("最小 WAV 应可被 symphonia 解码");
        assert_eq!(clip.sample_rate(), 8000);
        assert_eq!(clip.frame_count(), 1000);
        assert!((clip.length() - 0.125).abs() < 1e-4, "时长应为 0.125s，实际 {}", clip.length());
    }

    #[test]
    fn test_audio_clip_decodes_stereo_wav() {
        // 44100Hz 双声道 16 位，4410 采样 → 0.1 秒
        let wav = make_wav_bytes(4410, 44100, 2, 16);
        let clip = AudioClip::new(wav).expect("立体声 WAV 应可被解码");
        assert_eq!(clip.sample_rate(), 44100);
        assert_eq!(clip.frame_count(), 4410);
        assert!((clip.length() - 0.1).abs() < 1e-4, "时长应为 0.1s，实际 {}", clip.length());
    }
}
