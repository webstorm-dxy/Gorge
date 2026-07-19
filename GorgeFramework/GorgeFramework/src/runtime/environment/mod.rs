//! 运行时环境模块（对应 C# `Runtime/Environment/` 文件夹）。
//!
//! 包含 GorgeSimulationRuntime 及子管理器、SimulationModule trait、环境全局注册表。

pub mod simulation_module;
pub mod global;

use crate::runtime::priority_heap::PriorityHeap;
use crate::signal::channel_split::ChannelSplit;
use crate::signal::edge::Edge;
use crate::signal::fragment::Fragment;
use crate::signal::multichannel_split::MultichannelSplit;
use gorge_core::system::native::injector::Injector;

// ==================== 音频乐段数据 ====================

/// 音频乐段数据（R-5c：供 AudioStaff → AudioManager 数据流用）
///
/// 从 SimulationScore 的谱表提取后传递给 AudioManager.add_period。
#[derive(Debug, Clone)]
pub struct AudioPeriodData {
    /// 乐段对象 ID
    pub period_id: usize,
    /// 音频资产对象 ID
    pub audio_id: usize,
    /// 时间偏移（秒）
    pub time_offset: f32,
}

impl AudioPeriodData {
    pub fn new(period_id: usize, audio_id: usize, time_offset: f32) -> Self {
        Self { period_id, audio_id, time_offset }
    }
}

// ==================== 子管理器骨架 ====================

/// 谱面管理器
///
/// 维护 AliveElements/AliveNotes 表、定时创生/销毁列表、
/// Element 修改器表等。
#[derive(Debug)]
pub struct ChartManager {
    pub begin_chart_time: f32,
    pub terminate_chart_time: f32,
    pub begin_simulate_speed: f32,
    /// 正转定时创生列表：(时间, 元素 injector ID, 构造方法全局 ID)
    pub forward_timed_generate_list: Vec<(f32, usize, usize)>,
    /// 反转定时创生列表
    pub backward_timed_generate_list: Vec<(f32, usize, usize)>,
    /// 正转定时销毁列表：(时间, 元素对象 ID)
    pub forward_timed_destroy_list: Vec<(f32, usize)>,
    /// 反转定时销毁列表
    pub backward_timed_destroy_list: Vec<(f32, usize)>,
    /// 存活元素总表（元素对象 ID 列表）
    pub alive_elements: Vec<usize>,
    /// 存活 Note 表
    pub alive_notes: Vec<usize>,
    /// 存活非派生元素 → 创生时注入器 ID 的映射
    pub alive_injector_map: std::collections::HashMap<usize, usize>,
}

impl ChartManager {
    pub fn new() -> Self {
        Self {
            begin_chart_time: 0.0,
            terminate_chart_time: 0.0,
            begin_simulate_speed: 1.0,
            forward_timed_generate_list: Vec::new(),
            backward_timed_generate_list: Vec::new(),
            forward_timed_destroy_list: Vec::new(),
            backward_timed_destroy_list: Vec::new(),
            alive_elements: Vec::new(),
            alive_notes: Vec::new(),
            alive_injector_map: std::collections::HashMap::new(),
        }
    }

    /// 向定时创生表添加元素（对齐 C# `AddScoreElement` 正转/反转生成部分）
    ///
    /// `injector_id` 为注入器对象 ID，`constructor_id` 为构造方法全局 ID，
    /// `time` 为生成时间，`direction` 为正向/反向。
    pub fn add_timed_generate(&mut self, injector_id: usize, constructor_id: usize, time: f32, direction: crate::runtime::simulation_types::SimulateDirection) {
        match direction {
            crate::runtime::simulation_types::SimulateDirection::Forward => {
                self.forward_timed_generate_list.push((time, injector_id, constructor_id));
            }
            crate::runtime::simulation_types::SimulateDirection::Backward => {
                self.backward_timed_generate_list.push((time, injector_id, constructor_id));
            }
            crate::runtime::simulation_types::SimulateDirection::Infinitesimal => {}
        }
    }

    /// 向定时销毁表添加元素
    pub fn add_timed_destroy(&mut self, element_id: usize, time: f32, direction: crate::runtime::simulation_types::SimulateDirection) {
        match direction {
            crate::runtime::simulation_types::SimulateDirection::Forward => {
                self.forward_timed_destroy_list.push((time, element_id));
            }
            crate::runtime::simulation_types::SimulateDirection::Backward => {
                self.backward_timed_destroy_list.push((time, element_id));
            }
            crate::runtime::simulation_types::SimulateDirection::Infinitesimal => {}
        }
    }

    /// 添加谱面元素到生成表（对齐 C# `AddScoreElement`，S4-4）
    ///
    /// 查询 injector 对应类的构造注解（@InitializeGenerate/@ForwardTimedGenerate/@BackwardTimedGenerate）
    /// 和 @PeriodModifier 静态方法，填充 ChartManager 的定时创生/初始化创生表。
    /// `injector_id` 为注入器对象 ID，`vm` 用于注解扫描和方法调用。
    pub fn add_score_element(
        &mut self, injector_id: usize, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        let class_name = match vm.injectors.get(&injector_id) {
            Some(inj) => inj.injection_class_declaration().class_type.full_name(),
            None => return,
        };

        // 先克隆注解列表以释放不可变借用
        let init_ctors: Vec<(usize, _)> = vm.class_table
            .get(&class_name)
            .map(|cls| cls.declaration.constructors_with_annotation("InitializeGenerate")
                .into_iter().map(|(id, ann)| (id, ann.clone())).collect())
            .unwrap_or_default();
        for (ctor_id, _ann) in init_ctors {
            self.forward_timed_generate_list.push((0.0, injector_id, ctor_id));
        }

        let fwd_ctors: Vec<(usize, _)> = vm.class_table
            .get(&class_name)
            .map(|cls| cls.declaration.constructors_with_annotation("ForwardTimedGenerate")
                .into_iter().map(|(id, ann)| (id, ann.clone())).collect())
            .unwrap_or_default();
        for (ctor_id, ann) in fwd_ctors {
            let time = Self::resolve_annotation_time(&ann, &class_name, vm);
            self.forward_timed_generate_list.push((time, injector_id, ctor_id));
        }

        let bwd_ctors: Vec<(usize, _)> = vm.class_table
            .get(&class_name)
            .map(|cls| cls.declaration.constructors_with_annotation("BackwardTimedGenerate")
                .into_iter().map(|(id, ann)| (id, ann.clone())).collect())
            .unwrap_or_default();
        for (ctor_id, ann) in bwd_ctors {
            let time = Self::resolve_annotation_time(&ann, &class_name, vm);
            self.backward_timed_generate_list.push((time, injector_id, ctor_id));
        }
    }

    /// 从注解参数中解析 time 值（Float 直接取值 / Delegate 经 invoke_method_by_id 求值）
    fn resolve_annotation_time(
        ann: &gorge_core::objective::declaration::MethodAnnotation,
        class_name: &str,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) -> f32 {
        use gorge_core::objective::declaration::AnnotationValue;
        match ann.find_parameter("time") {
            Some(AnnotationValue::Float(f)) => *f as f32,
            Some(AnnotationValue::Delegate(method_id)) => {
                if vm.invoke_method_by_id(class_name, None, *method_id).is_ok() {
                    vm.return_float.unwrap_or(0.0) as f32
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }
}

/// 仿真管理器
///
/// 持有两个优先级堆（Simulators + LateIndependentSimulators）。
/// S4c：SimulationMachine 已移至 RuntimeManager 解决借用冲突。
#[derive(Debug)]
pub struct SimulationManager {
    /// 主模拟器优先级堆（优先级 → 模拟器对象 ID）
    pub simulators: PriorityHeap<i32, usize>,
    /// 尾独立模拟器优先级堆
    pub late_independent_simulators: PriorityHeap<i32, usize>,
}

impl SimulationManager {
    pub fn new() -> Self {
        Self {
            simulators: PriorityHeap::new(),
            late_independent_simulators: PriorityHeap::new(),
        }
    }
}

// ==================== SimulationManager 内部 simulator 注册表 ====================
// 存储已注册的 Box<dyn ISimulator>，优先级 id 为 PriorityHeap 中的 usize 键。
// 因为 Box<dyn ISimulator> 不满足 Hash/Eq/Clone，无法直接放进 PriorityHeap 作为 V。
// 用独立的 Hasher+HashMap 搭配 PriorityHeap 完成管理。

use std::collections::HashMap;
use crate::simulators::ISimulator as SimulatorTrait;

/// 内部 simulator 注册表（非 pub，通过 SimulationManager 方法间接访问）
#[derive(Default)]
pub struct SimRegistry {
    /// 自增 ID → Box<dyn ISimulator>
    map: HashMap<usize, Box<dyn SimulatorTrait>>,
    next_id: usize,
}

impl SimRegistry {
    pub fn new() -> Self { Self { map: HashMap::new(), next_id: 1 } }

    /// 注册一个 simulator，返回分配的内部 ID
    pub fn register(&mut self, sim: Box<dyn SimulatorTrait>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.map.insert(id, sim);
        id
    }

    /// 按 ID 获取 simulator 引用
    pub fn get(&self, id: usize) -> Option<&dyn SimulatorTrait> {
        self.map.get(&id).map(|b| b.as_ref())
    }

    /// 按 ID 删除
    pub fn remove(&mut self, id: usize) {
        self.map.remove(&id);
    }

    /// 遍历全部 simulator
    pub fn iter(&self) -> impl Iterator<Item = (usize, &dyn SimulatorTrait)> {
        self.map.iter().map(|(id, sim)| (*id, sim.as_ref()))
    }
}

impl std::fmt::Debug for SimRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimRegistry").field("count", &self.map.len()).finish()
    }
}

/// 自动机管理器
///
/// 维护信号自动机列表、待决检测条件表、输入信号总表。
#[derive(Debug)]
pub struct AutomatonManager {
    /// 多通道输入信号切片（全部时间范围的信号记录，含边沿）
    pub input_signals: MultichannelSplit,
    /// 已注册的信号自动机对象 ID 列表（S4-3）
    pub automatons: Vec<usize>,
    /// 待决检测条件表：自动机对象 ID → 检测条件列表（S7）
    pub pending_detection_conditions: std::collections::HashMap<usize, Vec<crate::simulators::SignalDetectionCondition>>,
    /// 最多判定数（S7：供 ScoringV1 初始化）
    pub max_combo: i32,
}

impl AutomatonManager {
    pub fn new() -> Self {
        Self {
            input_signals: MultichannelSplit::new(),
            automatons: Vec::new(),
            pending_detection_conditions: std::collections::HashMap::new(),
            max_combo: 0,
        }
    }

    /// 追加信号边沿（对齐 C# `AutomatonManager.AddSignalEdge`）
    ///
    /// `value` 为信号值对象 ID，0 表示 null（终止信号）。
    /// 返回值表示是否真的追加了边沿。
    pub fn add_signal_edge(&mut self, channel_name: &str, signal_id: i32, time: f32, value: usize) -> bool {
        use std::collections::hash_map::Entry;

        let channel = match self.input_signals.entry(channel_name.to_string()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(ChannelSplit::new()),
        };

        match channel.entry(signal_id) {
            Entry::Vacant(v) => {
                // 分支1：无信号且 value 为空 → 追加失败
                if value == 0 {
                    return false;
                }
                // 分支2：无信号到有信号 → 新建 Fragment
                let fragment = Fragment {
                    signal_id,
                    start_time: time,
                    end_time: f32::INFINITY,
                    start_value: value,
                    edges: Vec::new(),
                };
                v.insert(fragment);
                true
            }
            Entry::Occupied(mut o) => {
                let signal = o.get_mut();
                // 分支3：有信号到无信号 → 终止信号
                if value == 0 {
                    signal.end_time = time;
                    return false;
                }

                // 有信号到有信号：计算当前最新信号值（无边沿取 start_value，否则取最后边沿值）
                let latest_value = signal.edges.last().map(|e| e.value).unwrap_or(signal.start_value);
                if latest_value == value {
                    // 信号值一致
                    // 分支4a：end_time >= time → 无需延续
                    if signal.end_time >= time {
                        return false;
                    }
                    // 分支4b：延续到当前时间
                    signal.end_time = time;
                    return true;
                }

                // 分支4c：信号值不一致 → 追加新边沿
                signal.end_time = f32::INFINITY;
                signal.edges.push(Edge::new(time, value));
                true
            }
        }
    }

    /// 获取输入信号的时间切片（对齐 C# `AutomatonManager.SplitInputSignals`）
    ///
    /// 左开右闭区间 `(from, to]`。
    /// 遍历全部信道的全部信号片段，调用 Fragment::split 切出子片段。
    pub fn split_input_signals(&self, from: f32, to: f32) -> MultichannelSplit {
        let mut result = MultichannelSplit::new();
        for (channel_name, channel_signals) in &self.input_signals {
            let mut split_channel = ChannelSplit::new();
            for (signal_id, fragment) in channel_signals {
                if let Some(split_fragment) = fragment.split(from, to) {
                    split_channel.insert(*signal_id, split_fragment);
                }
            }
            if !split_channel.is_empty() {
                result.insert(channel_name.clone(), split_channel);
            }
        }
        result
    }

    /// 计算执行时间点后（不包含）的最早边沿时间（对齐 C# `AutomatonManager.GetInputSignalEarliestEdgeTimeAfter`）
    ///
    /// 遍历全部信道全部片段全部边沿，取最小 > `time` 的边沿时间。
    /// 无边沿则返回 `f32::MAX`。
    pub fn get_input_signal_earliest_edge_time_after(&self, time: f32) -> f32 {
        if self.input_signals.is_empty() {
            return f32::MAX;
        }

        let mut earliest = f32::MAX;
        for channel_signals in self.input_signals.values() {
            if channel_signals.is_empty() { continue; }
            for fragment in channel_signals.values() {
                if fragment.edges.is_empty() { continue; }
                for edge in &fragment.edges {
                    if edge.time > time && edge.time < earliest {
                        earliest = edge.time;
                    }
                }
            }
            // 也检查片段起始时间
            for fragment in channel_signals.values() {
                if fragment.start_time > time && fragment.start_time < earliest {
                    earliest = fragment.start_time;
                }
            }
        }
        earliest
    }
}

/// 音频管理器（E-3 实体化）
///
/// 对齐 C# `Runtime/Environment/AudioManager.cs`。
/// 管理运行时音效播放和时段音频源。
pub struct AudioManager {
    /// 时段音频源表：音频时段对象 ID → 平台播放器 ID
    pub period_audio_sources: std::collections::HashMap<usize, usize>,
    /// 响应音效表：音效名 → 音效播放器 ID
    pub respond_effects: std::collections::HashMap<String, usize>,
    /// 时段音频播放器（持有所有权，player_id → Box<dyn IAudioPlayer>）
    period_players: std::collections::HashMap<usize, Box<dyn crate::adaptor::IAudioPlayer>>,
    /// 音效播放器（持有所有权，player_id → Box<dyn IAudioEffectPlayer>）
    effect_players: std::collections::HashMap<usize, Box<dyn crate::adaptor::IAudioEffectPlayer>>,
    /// 播放器自增 ID
    next_player_id: usize,
}

impl std::fmt::Debug for AudioManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioManager")
            .field("period_audio_sources", &self.period_audio_sources)
            .field("respond_effects", &self.respond_effects)
            .field("period_players_count", &self.period_players.len())
            .field("effect_players_count", &self.effect_players.len())
            .finish()
    }
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            period_audio_sources: std::collections::HashMap::new(),
            respond_effects: std::collections::HashMap::new(),
            period_players: std::collections::HashMap::new(),
            effect_players: std::collections::HashMap::new(),
            next_player_id: 1,
        }
    }

    /// 分配播放器 ID
    fn alloc_id(&mut self) -> usize {
        let id = self.next_player_id;
        self.next_player_id += 1;
        id
    }

    /// 播放响应音效（对齐 C# `PlayRespondEffect`）
    pub fn play_respond_effect(&self, name: &str) {
        if let Some(effect_id) = self.respond_effects.get(name) {
            if let Some(effect) = self.effect_players.get(effect_id) {
                effect.play();
            }
        }
    }

    /// 停止所有音乐（对齐 C# `StopAllSong`）
    pub fn stop_all_song(&self) {
        for player in self.period_players.values() {
            player.stop();
        }
    }

    /// 添加音频时段（对齐 C# `AddPeriod`）
    pub fn add_period(&mut self, period_id: usize, _audio_id: usize, _time_offset: f32) {
        let player = crate::adaptor::platform().create_audio_player();
        let handle = self.alloc_id();
        self.period_players.insert(handle, player);
        self.period_audio_sources.insert(period_id, handle);
    }

    /// 移除音频时段（对齐 C# `RemovePeriod`）
    pub fn remove_period(&mut self, period_id: usize) {
        if let Some(handle) = self.period_audio_sources.remove(&period_id) {
            self.period_players.remove(&handle);
        }
    }

    /// 注册响应音效（对齐 C# `StartSimulation` 中的效果注册部分）
    pub fn register_respond_effect(&mut self, name: String, audio_id: usize) {
        let effect = crate::adaptor::platform().create_audio_effect_player(audio_id);
        let handle = self.alloc_id();
        self.effect_players.insert(handle, effect);
        self.respond_effects.insert(name, handle);
    }

    /// 清理所有音效资源（对齐 C# `StopSimulation`）
    pub fn clear_respond_effects(&mut self) {
        self.respond_effects.clear();
        self.effect_players.clear();
    }

    /// 清理所有音频时段（释放播放器所有权）
    pub fn clear_audio_sources(&mut self) {
        self.period_audio_sources.clear();
        self.period_players.clear();
    }
}

/// 图形管理器（S4-6：nodes 表）
#[derive(Debug)]
pub struct GraphicsManager {
    /// 存活图形节点对象 ID 列表
    pub nodes: Vec<usize>,
}

impl GraphicsManager {
    pub fn new() -> Self { Self { nodes: Vec::new() } }
}

/// 资源管理器
///
/// 维护资产名 → 资产对象 ID 的注册表（对齐 C# `Environment.GetAssetByName`）。
#[derive(Debug)]
pub struct AssetManager {
    /// 资产名 → 资产对象 ID
    pub assets: std::collections::HashMap<String, usize>,
}

impl AssetManager {
    pub fn new() -> Self { Self { assets: std::collections::HashMap::new() } }

    /// 注册资产
    pub fn register(&mut self, name: String, object_id: usize) {
        self.assets.insert(name, object_id);
    }

    /// 按名称查找资产对象 ID
    pub fn get_asset_by_name(&self, name: &str) -> Option<usize> {
        self.assets.get(name).copied()
    }
}

/// 场景管理器（骨架）
#[derive(Debug)]
pub struct SceneManager;

impl SceneManager { pub fn new() -> Self { Self } }

/// 仿真日志器（骨架）
#[derive(Debug)]
pub struct SimulationLogger;

impl SimulationLogger {
    pub fn new() -> Self { Self }
    pub fn debug_log(&self, _msg: &str, _indent: usize) {}
}

// ==================== 运行时环境根容器 ====================

/// GorgeSimulationRuntime（对应 C# 同名类）
///
/// 持有全部子 Manager，统一管理 LoadScore / StartSimulation / StopSimulation 生命周期。
pub struct GorgeSimulationRuntime {
    pub chart: ChartManager,
    pub simulation: SimulationManager,
    pub automaton: AutomatonManager,
    pub audio: AudioManager,
    pub graphics: GraphicsManager,
    pub asset: AssetManager,
    pub scene: SceneManager,
    pub logger: SimulationLogger,
    /// 主模拟器注册表（内部 ID → Box<dyn ISimulator>）
    pub sim_registry: SimRegistry,
    /// 尾独立模拟器注册表
    pub late_sim_registry: SimRegistry,
    /// 计分器（S7：ScoringV1，由 add_score_element 初始化）
    pub scoring: crate::stage::ScoringV1,
    /// 当前模拟时间（由 SimulationMachine 在动作执行前同步）
    pub simulate_time: f32,
    /// 当前谱面时间（由 SimulationMachine 在动作执行前同步）
    pub chart_time: f32,
    /// F-3：谱面是否已加载
    pub is_score_loaded: bool,
    /// F-3：是否正在仿真
    pub is_simulating: bool,
}

impl GorgeSimulationRuntime {
    pub fn new() -> Self {
        Self {
            chart: ChartManager::new(),
            simulation: SimulationManager::new(),
            automaton: AutomatonManager::new(),
            audio: AudioManager::new(),
            graphics: GraphicsManager::new(),
            asset: AssetManager::new(),
            scene: SceneManager::new(),
            logger: SimulationLogger::new(),
            sim_registry: SimRegistry::new(),
            late_sim_registry: SimRegistry::new(),
            scoring: crate::stage::ScoringV1::new(1),
            simulate_time: 0.0,
            chart_time: 0.0,
            is_score_loaded: false,
            is_simulating: false,
        }
    }

    // ==================== F-3 生命周期方法 ====================

    /// 加载谱面（对齐 C# `LoadScore` 37-48 行）
    ///
    /// 若已加载则先卸载。调用 Chart.LoadScore 读取谱面数据。
    pub fn load_score(&mut self) {
        if self.is_score_loaded {
            self.unload_score();
        }
        // Chart.LoadScore —— 读取谱面
        self.chart_load_score();
        self.is_score_loaded = true;
    }

    /// 卸载谱面（对齐 C# `UnloadScore` 50-65 行）
    ///
    /// 若未加载则直接返回。若正在仿真则先停止。
    pub fn unload_score(&mut self) {
        if !self.is_score_loaded {
            return;
        }
        if self.is_simulating {
            self.stop_simulation();
        }
        // Chart.UnloadScore
        self.chart_unload_score();
        self.is_score_loaded = false;
    }

    /// 启动仿真（对齐 C# `StartSimulation` 67-89 行）
    ///
    /// 若未加载谱面则 panic。若已在仿真中则先停止再启动。
    pub fn start_simulation(&mut self) {
        if !self.is_score_loaded {
            panic!("尝试在谱面加载前启动仿真");
        }
        if self.is_simulating {
            self.stop_simulation();
        }

        self.logger_start_simulation();
        self.scene_runtime_initialize();
        self.audio_start_simulation();
        self.graphics_start_simulation();
        self.simulation_runtime_initialize();
        self.automaton_runtime_initialize();
        self.chart_start_simulation();
        // Simulation.SimulationMachine.DriveInstantly —— 由 RuntimeManager::drive 负责

        self.is_simulating = true;
    }

    /// 停止仿真（对齐 C# `StopSimulation` 91-107 行）
    ///
    /// 若未在仿真中则直接返回。
    pub fn stop_simulation(&mut self) {
        if !self.is_simulating {
            return;
        }

        self.chart_stop_simulation();
        self.automaton_runtime_destruct();
        self.simulation_runtime_destruct();
        self.graphics_stop_simulation();
        self.audio_stop_simulation();
        self.scene_runtime_destruct();
        self.logger_stop_simulation();

        self.is_simulating = false;
    }

    /// 复位后重新跳转到当前位置（对齐 C# `RePlay` 112-120 行）
    ///
    /// 记录当前 chartTime → StopSimulation → StartSimulation →
    /// SimulationMachine.DriveToChartTime(nowChartTime) → StopAllSong。
    ///
    /// `machine` 为外部持有的 SimulationMachine（S4c 拆分后与本运行时分离）。
    /// `vm` 用于 GameplayAction 执行中的 VM 操作。
    pub fn replay(
        &mut self,
        machine: &mut crate::runtime::simulation_machine::SimulationMachine,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        let now_chart_time = self.chart_time;

        self.stop_simulation();
        self.start_simulation();
        machine.drive_to_chart_time(now_chart_time, self, vm);
        self.audio.stop_all_song();
    }

    // ==================== R-5c 音频乐段注册 ====================

    /// 从谱面数据注册音频乐段到 AudioManager（R-5c）
    ///
    /// 将提取的 AudioPeriod 数据传递给 AudioManager，
    /// AudioManager 经平台创建对应音频播放器并关联时段。
    pub fn register_audio_periods(&mut self, periods: &[AudioPeriodData]) {
        for p in periods {
            self.audio.add_period(p.period_id, p.audio_id, p.time_offset);
        }
    }

    // ==================== 子管理器生命周期钩子（内部） ====================

    fn chart_load_score(&mut self) {}
    fn chart_unload_score(&mut self) {}
    fn chart_start_simulation(&mut self) {}
    fn chart_stop_simulation(&mut self) {}

    fn audio_start_simulation(&mut self) {}
    fn audio_stop_simulation(&mut self) {}

    fn graphics_start_simulation(&mut self) {}
    fn graphics_stop_simulation(&mut self) {}

    fn automaton_runtime_initialize(&mut self) {}
    fn automaton_runtime_destruct(&mut self) {}

    fn simulation_runtime_initialize(&mut self) {}
    fn simulation_runtime_destruct(&mut self) {}

    fn scene_runtime_initialize(&mut self) {
        // 对齐 C# SceneManager.RuntimeInitialize → new ScoringV1(1395)
        self.scoring = crate::stage::ScoringV1::new(1395);
    }

    fn scene_runtime_destruct(&mut self) {}

    fn logger_start_simulation(&mut self) {
        self.logger.debug_log("仿真开始", 0);
    }

    fn logger_stop_simulation(&mut self) {
        self.logger.debug_log("仿真停止", 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== AutomatonManager 测试 ====================

    #[test]
    fn test_add_signal_edge_new_channel_new_signal() {
        // 分支2：无信道 + 有值 → 新建信号，返回 true
        let mut mgr = AutomatonManager::new();
        assert!(mgr.add_signal_edge("Touch", 1, 0.0, 100));
        assert!(mgr.input_signals.contains_key("Touch"));
        let ch = &mgr.input_signals["Touch"];
        assert!(ch.contains_key(&1));
        let frag = &ch[&1];
        assert_eq!(frag.signal_id, 1);
        assert_eq!(frag.start_time, 0.0);
        assert_eq!(frag.start_value, 100);
        assert!(frag.end_time.is_infinite());
        assert!(frag.edges.is_empty());
    }

    #[test]
    fn test_add_signal_edge_null_value_no_signal() {
        // 分支1：无信号且 value=0 → 返回 false
        let mut mgr = AutomatonManager::new();
        assert!(!mgr.add_signal_edge("Touch", 1, 0.0, 0));
        // 信道被创建但无信号
        assert!(mgr.input_signals.contains_key("Touch"));
        assert!(!mgr.input_signals["Touch"].contains_key(&1));
    }

    #[test]
    fn test_add_signal_edge_terminate_signal() {
        // 分支3：有信号到无信号 → 终止信号，返回 false
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("Touch", 1, 0.0, 100);
        assert!(!mgr.add_signal_edge("Touch", 1, 5.0, 0));
        assert_eq!(mgr.input_signals["Touch"][&1].end_time, 5.0);
    }

    #[test]
    fn test_add_signal_edge_same_value_not_expired() {
        // 分支4a：信号值一致 + end_time >= time → 不追加，返回 false
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("Touch", 1, 0.0, 100);
        // end_time 是 INF，>= 3.0
        assert!(!mgr.add_signal_edge("Touch", 1, 3.0, 100));
        // 信号未被修改
        assert!(mgr.input_signals["Touch"][&1].end_time.is_infinite());
    }

    #[test]
    fn test_add_signal_edge_same_value_expired() {
        // 分支4b：信号值一致 + end_time < time → 延续到当前时间
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("Touch", 1, 0.0, 100);
        // 先终止
        mgr.add_signal_edge("Touch", 1, 3.0, 0);
        assert_eq!(mgr.input_signals["Touch"][&1].end_time, 3.0);
        // 再以同值 "复活"，end_time=3.0 < 5.0 → 延续
        assert!(mgr.add_signal_edge("Touch", 1, 5.0, 100));
        assert_eq!(mgr.input_signals["Touch"][&1].end_time, 5.0);
    }

    #[test]
    fn test_add_signal_edge_different_value() {
        // 分支4c：信号值不一致 → 追加新边沿
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("Touch", 1, 0.0, 100);
        assert!(mgr.add_signal_edge("Touch", 1, 2.0, 200));
        let frag = &mgr.input_signals["Touch"][&1];
        assert!(frag.end_time.is_infinite());
        assert_eq!(frag.edges.len(), 1);
        assert_eq!(frag.edges[0].time, 2.0);
        assert_eq!(frag.edges[0].value, 200);
    }

    #[test]
    fn test_add_signal_edge_multiple_channels() {
        // 多信道独立
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("Touch", 1, 0.0, 100);
        mgr.add_signal_edge("Keyboard", 2, 1.0, 42);
        assert_eq!(mgr.input_signals.len(), 2);
        assert_eq!(mgr.input_signals["Touch"][&1].start_value, 100);
        assert_eq!(mgr.input_signals["Keyboard"][&2].start_value, 42);
    }

    #[test]
    fn test_split_input_signals_empty() {
        let mgr = AutomatonManager::new();
        let split = mgr.split_input_signals(0.0, 10.0);
        assert!(split.is_empty());
    }

    #[test]
    fn test_split_input_signals_with_signal() {
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("Touch", 1, 0.0, 100);
        mgr.add_signal_edge("Touch", 1, 3.0, 200);
        mgr.add_signal_edge("Touch", 1, 5.0, 0); // 终止

        let split = mgr.split_input_signals(1.0, 4.0);
        assert!(split.contains_key("Touch"));
        let ch = &split["Touch"];
        assert!(ch.contains_key(&1));
        let frag = &ch[&1];
        // start_time=1.0, start_value 应该是 sample(1.0) = 100（原始信号末边沿≤1.0=无→start_value=100）
        assert_eq!(frag.start_value, 100);
        // edges: 只有 3.0 的边沿在 (1.0,4.0] 区间，5.0 的不在
        assert_eq!(frag.edges.len(), 1);
        assert_eq!(frag.edges[0].time, 3.0);
        assert_eq!(frag.edges[0].value, 200);
    }

    #[test]
    fn test_get_input_signal_earliest_edge_time_empty() {
        let mgr = AutomatonManager::new();
        assert_eq!(mgr.get_input_signal_earliest_edge_time_after(0.0), f32::MAX);
    }

    #[test]
    fn test_get_input_signal_earliest_edge_time_with_edges() {
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("A", 1, 0.0, 100);
        mgr.add_signal_edge("A", 1, 1.5, 200); // edge at 1.5
        mgr.add_signal_edge("A", 1, 3.0, 300); // edge at 3.0

        // >1.0 的最早边沿是 1.5
        assert!((mgr.get_input_signal_earliest_edge_time_after(1.0) - 1.5).abs() < 0.001);
        // >2.0 的最早边沿是 3.0
        assert!((mgr.get_input_signal_earliest_edge_time_after(2.0) - 3.0).abs() < 0.001);
        // >5.0 无边沿
        assert_eq!(mgr.get_input_signal_earliest_edge_time_after(5.0), f32::MAX);
    }

    #[test]
    fn test_get_input_signal_earliest_edge_time_multi_channel() {
        let mut mgr = AutomatonManager::new();
        mgr.add_signal_edge("A", 1, 0.0, 100);
        mgr.add_signal_edge("A", 1, 2.0, 200);
        mgr.add_signal_edge("B", 2, 0.0, 50);
        mgr.add_signal_edge("B", 2, 1.0, 150);

        // >0.5 的最早边沿应该是 1.0（B 信道）
        assert!((mgr.get_input_signal_earliest_edge_time_after(0.5) - 1.0).abs() < 0.001);
    }

    // ==================== F-3 GorgeSimulationRuntime 生命周期测试 ====================

    #[test]
    fn test_f3_lifecycle_full_sequence() {
        let mut rt = GorgeSimulationRuntime::new();
        // 初始状态
        assert!(!rt.is_score_loaded);
        assert!(!rt.is_simulating);

        rt.load_score();
        assert!(rt.is_score_loaded);
        assert!(!rt.is_simulating);

        rt.start_simulation();
        assert!(rt.is_score_loaded);
        assert!(rt.is_simulating);

        rt.stop_simulation();
        assert!(rt.is_score_loaded);
        assert!(!rt.is_simulating);

        rt.unload_score();
        assert!(!rt.is_score_loaded);
        assert!(!rt.is_simulating);
    }

    #[test]
    fn test_f3_double_load_rejects_previous() {
        let mut rt = GorgeSimulationRuntime::new();
        rt.load_score();
        assert!(rt.is_score_loaded);

        // 第二次 load 应先 unload 再 load
        rt.load_score();
        assert!(rt.is_score_loaded);
    }

    #[test]
    #[should_panic(expected = "尝试在谱面加载前启动仿真")]
    fn test_f3_start_before_load_panics() {
        let mut rt = GorgeSimulationRuntime::new();
        rt.start_simulation();
    }

    #[test]
    fn test_f3_unload_while_simulating_stops_first() {
        let mut rt = GorgeSimulationRuntime::new();
        rt.load_score();
        rt.start_simulation();
        assert!(rt.is_simulating);

        rt.unload_score();
        assert!(!rt.is_simulating);
        assert!(!rt.is_score_loaded);
    }

    #[test]
    fn test_f3_stop_not_simulating_noop() {
        let mut rt = GorgeSimulationRuntime::new();
        rt.stop_simulation(); // 不应 panic
        assert!(!rt.is_simulating);
    }

    // ==================== R-5c AudioStaff→AudioManager 数据流测试 ====================

    #[test]
    fn test_r5c_register_audio_periods_into_audio_manager() {
        crate::runtime::environment::global::init_env_global();
        crate::adaptor::install_platform(Box::new(crate::adaptor::HeadlessPlatform::new()));

        let mut rt = GorgeSimulationRuntime::new();
        let periods = vec![
            AudioPeriodData::new(100, 10, 0.5),
            AudioPeriodData::new(101, 11, 1.5),
        ];

        // 注册前 audio 为空
        assert!(rt.audio.period_audio_sources.is_empty());

        rt.register_audio_periods(&periods);

        // 注册后应有 2 个时段
        assert_eq!(rt.audio.period_audio_sources.len(), 2);
        assert!(rt.audio.period_audio_sources.contains_key(&100));
        assert!(rt.audio.period_audio_sources.contains_key(&101));
    }

    #[test]
    fn test_r5c_remove_audio_period_cleans_up() {
        crate::runtime::environment::global::init_env_global();
        crate::adaptor::install_platform(Box::new(crate::adaptor::HeadlessPlatform::new()));

        let mut rt = GorgeSimulationRuntime::new();
        rt.register_audio_periods(&[AudioPeriodData::new(1, 10, 0.0)]);
        assert_eq!(rt.audio.period_audio_sources.len(), 1);

        rt.audio.remove_period(1);
        assert!(rt.audio.period_audio_sources.is_empty());
    }

    // ==================== R-1 / C-3 RePlay 测试 ====================

    #[test]
    fn test_r1_replay_preserves_state() {
        use crate::runtime::simulation_machine::SimulationMachine;
        use gorge_core::virtual_machine::vm::VirtualMachine;

        let mut rt = GorgeSimulationRuntime::new();
        rt.load_score();
        rt.start_simulation();
        rt.chart_time = 42.0;

        // RePlay: stop → start → drive_to_chart_time，chart_time 被恢复
        let mut machine = SimulationMachine::new(0.0, 100.0, 1.0);
        machine.runtime_initialize();
        let mut vm = VirtualMachine::new();
        rt.replay(&mut machine, &mut vm);

        assert!(rt.is_simulating);
        // drive_to_chart_time 将 chart_time 恢复到 replay 前的值
        assert!(machine.chart_time >= 42.0);
    }
}
