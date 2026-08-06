//! 运行时管理器（对应 C# `Runtime/RuntimeManager.cs`）。
//!
//! 顶层编排——管理仿真运行时的完整生命周期（Compile → Extract → Prepare → Init → Load → Simulate）。
//! C# 原版依赖 GorgeCompiler 进行源码编译；Rust 版接收已编译的字节码。
//! S4c：SimulationMachine 存于此（而非 SimulationManager 内）以解决 Rust 借用冲突。

use crate::runtime::environment::global;
use crate::runtime::environment::GorgeSimulationRuntime;
use crate::runtime::runtime_form_container::RuntimeFormContainer;
use crate::runtime::simulation_machine::SimulationMachine;
use crate::chart::simulation_score::SimulationScore;
use gorge_core::objective::bytecode::CompiledClass;

/// 运行时状态（对齐 C# `RuntimeState` 枚举）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeState {
    /// 未初始化
    Uninitialized,
    /// 编译完成
    Compiled,
    /// 仿真资源加载完成
    SimulationResourceLoaded,
    /// 仿真初始化完成
    SimulationInitialized,
    /// 谱面加载完成
    ScoreLoaded,
    /// 仿真中
    Simulating,
}

/// 运行时管理器
///
/// 状态机流程：
/// ```text
/// Uninitialized → Compiled → SimulationResourceLoaded → SimulationInitialized
///   → ScoreLoaded ⇄ Simulating
/// ```
///
/// 每个状态迁移由对应方法执行并检查前置条件。
pub struct RuntimeManager {
    pub state: RuntimeState,
    pub simulation_runtime: Option<GorgeSimulationRuntime>,
    /// 仿真机（与 runtime 同级以避免 &mut self + &mut runtime 借用冲突）
    pub machine: Option<SimulationMachine>,
    /// 仿真总谱（由 extract_simulation_resources 创建，prepare_score 填充）
    pub score: Option<SimulationScore>,
    /// 运行时模态容器（P0-6，对应 C# `RuntimeManager.FormContainer`）。
    ///
    /// 由 `scan_forms` 填充（模态表 / Element 修改器表 / 即时音效方法表），
    /// `prepare_score` / `reload_assets` 经 `load_instant_audio` 读取
    /// 即时音效方法表；`ChartManager` 经容器访问 Element 修改器。
    pub form_container: RuntimeFormContainer,
    /// 编译类引用（供 prepare_score 中提取谱表/模态使用）
    compiled_classes: Vec<CompiledClass>,
}

impl RuntimeManager {
    pub fn new() -> Self {
        global::init_env_global();

        Self {
            state: RuntimeState::Uninitialized,
            simulation_runtime: None,
            machine: None,
            score: None,
            form_container: RuntimeFormContainer::new_empty(),
            compiled_classes: Vec::new(),
        }
    }

    /// 设置编译类上下文（在 extract_simulation_resources 之前调用）
    ///
    /// 将编译后的类信息存储到管理器中，供 prepare_score 提取谱表使用。
    /// 应在状态迁移到 Compiled 后、调用 extract_simulation_resources 前设置。
    pub fn set_compiled_classes(&mut self, classes: Vec<CompiledClass>) {
        self.compiled_classes = classes;
    }

    /// 扫描模态（从编译类中提取 Form 信息）
    ///
    /// 遍历已编译类，提取所有带 `@Form` / `@InstantAudio` 注解的方法，
    /// 将编译类中的模态信息填充到指定的 RuntimeFormContainer 中。
    ///
    /// 通过 VM 调用 `@Form` 静态方法提取元素类型列表。
    ///
    /// P0-6 后本管理器自持 `form_container` 字段；此方法为低层接口
    /// （供 loader 等外部调用方填充自己的容器），内部路径直接操作 `form_container`。
    pub fn scan_forms(
        &self,
        form_container: &mut RuntimeFormContainer,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        form_container.scan_forms_from_compiled(&self.compiled_classes, vm);
    }

    /// 将编译类中的模态信息扫描进自持的 `form_container`（P0-6）
    ///
    /// 对齐 C# `CreateLanguageRuntime` 中 `FormContainer = new RuntimeFormContainer(LanguageRuntime)`
    /// 的时机：编译类就绪后、提取仿真资源前调用一次，此后容器由本管理器持有，
    /// `prepare_score` 读取即时音效方法表、`ChartManager` 读取 Element 修改器表。
    pub fn scan_forms_into_owned(
        &mut self,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        self.form_container.scan_forms_from_compiled(&self.compiled_classes, vm);
    }

    // ==================== R-2a: extract_simulation_resources（C# 104-143） ====================

    /// 提取运行时资源
    ///
    /// 创建 SimulationScore → 提取资产文件 → 调用 prepare_score。
    /// 前置条件：State 至少为 Compiled。
    pub fn extract_simulation_resources(
        &mut self,
        start_time: f32,
        terminate_time: f32,
        speed: f32,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        match self.state {
            RuntimeState::Uninitialized => {
                panic!("尝试在Gorge语言运行时准备完成前提取仿真资源");
            }
            _ => {}
        }

        self.score = Some(SimulationScore::new(start_time, terminate_time, speed));
        self.prepare_score(vm);
        self.state = RuntimeState::SimulationResourceLoaded;
    }

    /// 重新加载资产（不重新提取谱表，对应 C# `ReloadAssets` 148-167）
    pub fn reload_assets(&mut self, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        if let Some(ref mut score) = self.score {
            let mut backend = crate::adaptor::PlatformAssetBackend::new(crate::adaptor::platform());
            score.add_file_asset(&mut backend);
            score.load_assets();
            global::sync_assets_from(
                &score.loaded_assets.iter().map(|(k, v)| (k.clone(), v.handle)).collect()
            );
            // P0-6：即时音效方法表来自自持的 FormContainer
            score.load_instant_audio(&self.form_container, vm);
        }
    }

    // ==================== R-2b: prepare_score（C# 169-215） ====================

    /// 准备谱面
    ///
    /// add_file_asset（需 PlatformBase 已安装）→ LoadAssets →
    /// ExtractStaveFromRuntime → LoadInstantAudio（读取自持 FormContainer 的即时音效方法表）。
    ///
    /// `vm` 用于即时音效静态方法调用（对齐 C# 经 RuntimeStatic 访问运行时的设计）。
    pub fn prepare_score(&mut self, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        let score = match &mut self.score {
            Some(s) => s,
            None => return,
        };

        // 自动将资产文件处理为资产注入器（需平台后端）
        if crate::adaptor::platform_installed() {
            let mut backend = crate::adaptor::PlatformAssetBackend::new(crate::adaptor::platform());
            score.add_file_asset(&mut backend);
        }

        // 加载所有资产（填 loaded_assets 表）
        score.load_assets();

        // 同步资产到全局注册表（供 EnvironmentNative.GetAssetByName 使用）
        global::sync_assets_from(
            &score.loaded_assets.iter().map(|(k, v)| (k.clone(), v.handle)).collect()
        );

        // 提取谱表（从编译类注解中查找 @AudioStaff/@ElementStaff）
        // C#: Score.ExtractStaveFromRuntime(LanguageRuntime)
        score.extract_staves_from_compiled(&self.compiled_classes);

        // 加载即时音频（P0-6：即时音效方法表来自自持的 FormContainer）
        score.load_instant_audio(&self.form_container, vm);
    }

    // ==================== create_simulation_runtime（C# 220-230） ====================

    /// 创建并初始化仿真运行时
    ///
    /// 前置条件：State 至少为 SimulationResourceLoaded。
    /// `on_terminate` 为谱面终结回调（对齐 C# `CreateSimulationRuntime(Action? onTerminate = null)`，
    /// P1-4），由 `Terminate` 动作在谱面终结时触发；不需要时传 None。
    pub fn create_simulation_runtime(
        &mut self, begin_chart: f32, terminate_chart: f32, begin_speed: f32,
        on_terminate: Option<Box<dyn FnMut()>>,
    ) {
        match self.state {
            RuntimeState::Uninitialized | RuntimeState::Compiled => {
                panic!("尝试在运行时资源提取完成前准备仿真环境");
            }
            _ => {}
        }

        let mut rt = GorgeSimulationRuntime::new();
        rt.on_terminate = on_terminate;
        let mut machine = SimulationMachine::new(begin_chart, terminate_chart, begin_speed);
        machine.runtime_initialize();
        self.simulation_runtime = Some(rt);
        self.machine = Some(machine);
        self.state = RuntimeState::SimulationInitialized;
    }

    // ==================== R-2c: destruct_simulation_runtime（C# 232-253） ====================

    /// 析构仿真运行时
    ///
    /// 按顺序：Simulating → StopSimulation → ScoreLoaded → UnloadScore → 置 null。
    pub fn destruct_simulation_runtime(&mut self, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        if self.state == RuntimeState::Simulating {
            self.stop_simulation(vm);
        }
        if self.state == RuntimeState::ScoreLoaded {
            self.unload_score(vm);
        }
        if self.state != RuntimeState::SimulationInitialized {
            return;
        }

        self.simulation_runtime = None;
        self.machine = None;
        self.state = RuntimeState::SimulationResourceLoaded;
    }

    // ==================== R-2d: load_score / unload_score（C# 254-279） ====================

    /// 加载谱面
    ///
    /// 前置条件：State 至少为 SimulationInitialized。
    /// 调用 GorgeSimulationRuntime.LoadScore。
    pub fn load_score(&mut self, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        match self.state {
            RuntimeState::Uninitialized | RuntimeState::Compiled | RuntimeState::SimulationResourceLoaded => {
                panic!("尝试在仿真环境准备完成前开始仿真");
            }
            _ => {}
        }

        if let (Some(ref mut rt), Some(ref mut machine), Some(ref score)) =
            (&mut self.simulation_runtime, &mut self.machine, &self.score)
        {
            rt.load_score(score, machine, vm);
            Self::seed_instant_audio(rt, score);
        }
        self.state = RuntimeState::ScoreLoaded;
    }

    /// 从谱面 InstantAudio 播种即时音效缓存到 AudioManager（P1-3）
    ///
    /// 提取 `{"__object_id": N}` 形式的延迟物化条目（音效名 → AudioAsset 对象 ID），
    /// JSON 结构不符的条目跳过。
    fn seed_instant_audio(rt: &mut GorgeSimulationRuntime, score: &SimulationScore) {
        let entries: Vec<(String, usize)> = score.instant_audio.iter()
            .filter_map(|(name, value)| {
                value.get("__object_id")?.as_u64().map(|id| (name.clone(), id as usize))
            })
            .collect();
        rt.audio.cache_instant_audio(entries);
    }

    /// 卸载谱面
    ///
    /// 若正在仿真则先停止。调用 GorgeSimulationRuntime.UnloadScore。
    pub fn unload_score(&mut self, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        if self.state == RuntimeState::Simulating {
            self.stop_simulation(vm);
        }
        if self.state != RuntimeState::ScoreLoaded {
            return;
        }

        if let (Some(ref mut rt), Some(ref mut machine)) = (&mut self.simulation_runtime, &mut self.machine) {
            rt.unload_score(machine, vm);
        }
        self.state = RuntimeState::SimulationInitialized;
    }

    // ==================== start_simulation / stop_simulation（C# 284-307） ====================

    /// 启动仿真
    ///
    /// 前置条件：State 为 ScoreLoaded。
    /// 调用 GorgeSimulationRuntime.StartSimulation。
    pub fn start_simulation(&mut self, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        match self.state {
            RuntimeState::Uninitialized | RuntimeState::Compiled
            | RuntimeState::SimulationResourceLoaded | RuntimeState::SimulationInitialized => {
                panic!("尝试在谱面加载完成前开始仿真");
            }
            _ => {}
        }

        // P1-3：启动前刷新即时音效缓存（reload_assets 后 restart 能拿到新音效）
        if let (Some(ref mut rt), Some(ref score)) = (&mut self.simulation_runtime, &self.score) {
            Self::seed_instant_audio(rt, score);
        }

        if let (Some(ref mut rt), Some(ref mut machine)) = (&mut self.simulation_runtime, &mut self.machine) {
            rt.start_simulation(machine, vm);
        }

        // 启动完成后同步全局：scoring（Scene.RuntimeInitialize 已重建）与
        // 音效表（Audio.StartSimulation 已创建播放器，EnvironmentNative 需要）
        if let Some(ref rt) = self.simulation_runtime {
            global::sync_scoring(rt.scoring.clone());
            global::with_env_global_mut(|env| {
                env.respond_effects.clear();
                env.respond_effects.extend(
                    rt.audio.respond_effects.iter().map(|(k, v)| (k.clone(), *v))
                );
            });
        }
        self.state = RuntimeState::Simulating;
    }

    /// 停止仿真
    ///
    /// 前置条件：State 为 Simulating。
    pub fn stop_simulation(&mut self, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        if self.state != RuntimeState::Simulating {
            return;
        }

        if let (Some(ref mut rt), Some(ref mut machine)) = (&mut self.simulation_runtime, &mut self.machine) {
            rt.stop_simulation(machine, vm);
        }
        // P1-3：播放器已随 Audio.StopSimulation 销毁，同步清空全局音效表
        global::with_env_global_mut(|env| env.respond_effects.clear());
        self.state = RuntimeState::SimulationInitialized;
    }

    /// 驱动仿真推进（vm 需要传入以支持 GameplayAction 中的 VM 操作）
    pub fn drive(&mut self, simulation_time: f32, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        if self.state != RuntimeState::Simulating { return; }
        if let (Some(ref mut rt), Some(ref mut machine)) = (&mut self.simulation_runtime, &mut self.machine) {
            machine.drive(simulation_time, rt, vm);
        }
    }

    /// 卸载全部资源
    pub fn unload(&mut self, vm: &mut gorge_core::virtual_machine::vm::VirtualMachine) {
        if self.state == RuntimeState::Simulating {
            self.stop_simulation(vm);
        }
        if self.state == RuntimeState::ScoreLoaded {
            self.unload_score(vm);
        }
        self.simulation_runtime = None;
        self.machine = None;
        self.score = None;
        self.state = RuntimeState::Compiled;
    }
}

impl Default for RuntimeManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptor::HeadlessPlatform;

    fn setup() -> RuntimeManager {
        crate::adaptor::install_platform(Box::new(HeadlessPlatform::new()));
        RuntimeManager::new()
    }

    #[test]
    fn test_runtime_manager_new_initializes_environment_global() {
        let _manager = RuntimeManager::new();
        global::with_env_global(|_| ());
    }

    // ==================== R-2 生命周期测试 ====================

    #[test]
    fn test_r2_full_lifecycle_chain() {
        let mut mgr = setup();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        assert_eq!(mgr.state, RuntimeState::Uninitialized);

        // 跳过编译阶段（Compiled），直接进入资源提取
        mgr.state = RuntimeState::Compiled;

        // extract → prepare → init
        mgr.extract_simulation_resources(0.0, 100.0, 1.0, &mut vm);
        assert_eq!(mgr.state, RuntimeState::SimulationResourceLoaded);
        assert!(mgr.score.is_some());

        mgr.create_simulation_runtime(0.0, 100.0, 1.0, None);
        assert_eq!(mgr.state, RuntimeState::SimulationInitialized);
        assert!(mgr.simulation_runtime.is_some());
        assert!(mgr.machine.is_some());

        // load → start → stop → unload → destruct
        mgr.load_score(&mut vm);
        assert_eq!(mgr.state, RuntimeState::ScoreLoaded);

        mgr.start_simulation(&mut vm);
        assert_eq!(mgr.state, RuntimeState::Simulating);

        mgr.stop_simulation(&mut vm);
        assert_eq!(mgr.state, RuntimeState::SimulationInitialized);

        mgr.unload_score(&mut vm);
        assert_eq!(mgr.state, RuntimeState::SimulationInitialized);

        mgr.destruct_simulation_runtime(&mut vm);
        assert_eq!(mgr.state, RuntimeState::SimulationResourceLoaded);
        assert!(mgr.simulation_runtime.is_none());
    }

    #[test]
    #[should_panic(expected = "尝试在Gorge语言运行时准备完成前提取仿真资源")]
    fn test_r2_extract_before_compile_panics() {
        let mut mgr = setup();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        // State = Uninitialized
        mgr.extract_simulation_resources(0.0, 100.0, 1.0, &mut vm);
    }

    #[test]
    fn test_r2_start_before_load_panics() {
        let mut mgr = setup();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        mgr.state = RuntimeState::Compiled;
        mgr.extract_simulation_resources(0.0, 100.0, 1.0, &mut vm);
        mgr.create_simulation_runtime(0.0, 100.0, 1.0, None);
        // 跳过 load_score，直接 start 应 panic
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            mgr.start_simulation(&mut vm);
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_r2_load_score_twice_does_not_panic() {
        let mut mgr = setup();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        mgr.state = RuntimeState::Compiled;
        mgr.extract_simulation_resources(0.0, 100.0, 1.0, &mut vm);
        mgr.create_simulation_runtime(0.0, 100.0, 1.0, None);
        mgr.load_score(&mut vm);
        // 第二次 load 应正常（内部 unload→reload）
        mgr.load_score(&mut vm);
        assert_eq!(mgr.state, RuntimeState::ScoreLoaded);
    }

    #[test]
    fn test_r2_destruct_while_simulating_auto_stops() {
        let mut mgr = setup();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        mgr.state = RuntimeState::Compiled;
        mgr.extract_simulation_resources(0.0, 100.0, 1.0, &mut vm);
        mgr.create_simulation_runtime(0.0, 100.0, 1.0, None);
        mgr.load_score(&mut vm);
        mgr.start_simulation(&mut vm);
        assert_eq!(mgr.state, RuntimeState::Simulating);

        // destruct 时应自动 stop → unload → null
        mgr.destruct_simulation_runtime(&mut vm);
        assert_eq!(mgr.state, RuntimeState::SimulationResourceLoaded);
        assert!(mgr.simulation_runtime.is_none());
    }

    #[test]
    fn test_r2_unload_while_simulating_auto_stops() {
        let mut mgr = setup();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        mgr.state = RuntimeState::Compiled;
        mgr.extract_simulation_resources(0.0, 100.0, 1.0, &mut vm);
        mgr.create_simulation_runtime(0.0, 100.0, 1.0, None);
        mgr.load_score(&mut vm);
        mgr.start_simulation(&mut vm);
        assert_eq!(mgr.state, RuntimeState::Simulating);

        // unload 时应自动 stop
        mgr.unload_score(&mut vm);
        assert_eq!(mgr.state, RuntimeState::SimulationInitialized);
    }

    // ==================== R-3: set_compiled_classes / scan_forms 测试 ====================

    /// 构造一个带 @ElementStaff 注解的测试编译类
    fn make_test_compiled_class() -> CompiledClass {
        use gorge_core::objective::bytecode::CompiledAnnotation;
        use gorge_core::objective::declaration::{MethodAnnotation, AnnotationValue};
        use gorge_core::objective::types::{GorgeType, TypeCount};
        use gorge_core::virtual_machine::ir::CompiledMethod;
        use std::collections::HashMap;

        let mut method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        method_annotations.insert(0, vec![
            MethodAnnotation {
                name: "Chart".into(),
                parameters: vec![
                    ("timeOffset".into(), AnnotationValue::Float(1.5)),
                    ("minLength".into(), AnnotationValue::Float(20.0)),
                    ("active".into(), AnnotationValue::Bool(true)),
                ],
            },
        ]);

        CompiledClass {
            class_type: GorgeType::class("Test.ChartStaff", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![
                CompiledMethod { name: "Period1".into(), codes: vec![], local_count: 0 },
            ],
            constructors: vec![],
            injector_fields: vec![],
            delegate_impls: vec![],
            method_start_id: 0,
            method_count_total: 1,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],
            injector_constants: vec![],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![
                CompiledAnnotation {
                    name: "ElementStaff".into(),
                    generic_type: None,
                    arguments: vec![("form".into(), "TestForm".into())],
                },
            ],
            method_annotations,
            constructor_annotations: HashMap::new(),
        }
    }

    #[test]
    fn test_r3_set_compiled_classes_and_extract() {
        let mut mgr = setup();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        mgr.state = RuntimeState::Compiled;

        // 设置编译类
        mgr.set_compiled_classes(vec![make_test_compiled_class()]);

        // extract_simulation_resources 内部调用 extract_staves_from_compiled
        mgr.extract_simulation_resources(0.0, 100.0, 1.0, &mut vm);
        assert_eq!(mgr.state, RuntimeState::SimulationResourceLoaded);

        // 验证谱表被正确提取
        let score = mgr.score.as_ref().unwrap();
        assert_eq!(score.stave.len(), 1, "应提取 1 个谱表");

        use crate::chart::staff::ElementStaff;
        let staff = score.stave[0].as_any().downcast_ref::<ElementStaff>().unwrap();
        assert_eq!(staff.class_name, "Test.ChartStaff");
        assert_eq!(staff.form_name, "TestForm");
        assert_eq!(staff.periods.len(), 1);
        assert_eq!(staff.periods[0].period_data.method_name, "Period1");
    }

    #[test]
    fn test_r3_scan_forms_with_compiled_classes() {
        let mgr = setup();

        use crate::runtime::runtime_form_container::RuntimeFormContainer;
        let mut form_container = RuntimeFormContainer::new_empty();

        // 空编译类列表时扫描不 panic
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        mgr.scan_forms(&mut form_container, &mut vm);
        assert!(form_container.forms.is_empty());
    }

    // ==================== P0-6: 自持 FormContainer 测试 ====================

    /// 构造一个带 `@InstantAudio` 注解与可执行静态方法体的编译类：
    /// `LoadInjectorConstant(0)` → `ReturnObject`，返回常量池中的注入器对象。
    /// 注册到 VM 后，`invoke_method_by_id` 可真实执行并返回注入器对象 ID，
    /// 供 `load_instant_audio` 读取（等价于真实谱面
    /// `return (AudioAsset) Environment.GetAssetByName(...)` 的返回对象路径）。
    fn make_instant_audio_compiled_class() -> CompiledClass {
        use gorge_core::objective::bytecode::{InjectorConstField, InjectorConstantDef};
        use gorge_core::objective::declaration::{AnnotationValue, MethodAnnotation};
        use gorge_core::objective::types::{GorgeType, TypeCount};
        use gorge_core::virtual_machine::ir::{
            Address, CodeWithSpan, CompiledMethod, IntermediateCode, IntermediateOperator, Operand,
            ValueType,
        };
        use std::collections::HashMap;

        // 注入器常量 → 局部 object[0] → ReturnObject
        let load_constant = CodeWithSpan::new(
            IntermediateCode {
                result: Some(Address::new(ValueType::Object, 0)),
                operator: IntermediateOperator::LoadInjectorConstant(0),
                left: Operand::int(0),
                right: None,
            },
            gorge_core::diagnostics::Span::dummy(),
        );
        let return_object = CodeWithSpan::new(
            IntermediateCode {
                result: None,
                operator: IntermediateOperator::ReturnObject,
                left: Operand::addr(Address::new(ValueType::Object, 0)),
                right: None,
            },
            gorge_core::diagnostics::Span::dummy(),
        );

        let mut method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        method_annotations.insert(0, vec![
            MethodAnnotation {
                name: "InstantAudio".into(),
                parameters: vec![
                    ("name".into(), AnnotationValue::String("RespondA".into())),
                ],
            },
        ]);

        CompiledClass {
            class_type: GorgeType::class("Dremu.DremuNativeResources", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![
                CompiledMethod {
                    name: "GetRespondA".into(),
                    codes: vec![load_constant, return_object],
                    local_count: 1,
                },
            ],
            constructors: vec![],
            injector_fields: vec![],
            delegate_impls: vec![],
            method_start_id: 0,
            method_count_total: 1,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],
            injector_constants: vec![
                InjectorConstantDef {
                    class_name: "GorgeFramework.AudioAsset".into(),
                    fields: vec![InjectorConstField::String("name".into(), "audio:Hit".into())],
                },
            ],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![],
            method_annotations,
            constructor_annotations: HashMap::new(),
        }
    }

    #[test]
    fn test_p0_6_owned_form_container_populated_by_scan() {
        let mut mgr = setup();
        mgr.set_compiled_classes(vec![make_instant_audio_compiled_class()]);
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();

        mgr.scan_forms_into_owned(&mut vm);

        // 容器自持：即时音效方法表已填充（模态表无 @Form 时为 0）
        assert_eq!(mgr.form_container.instant_audio_methods.len(), 1);
        let method_ref = mgr.form_container.instant_audio_methods.get("RespondA").unwrap();
        assert_eq!(method_ref.class_name, "Dremu.DremuNativeResources");
        assert_eq!(method_ref.method_id, 0);
        assert!(mgr.form_container.forms.is_empty());
    }

    #[test]
    fn test_p0_6_prepare_score_loads_instant_audio_from_owned_container() {
        let mut mgr = setup();
        mgr.set_compiled_classes(vec![make_instant_audio_compiled_class()]);
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();

        // 按 loader 约定以简单名注册编译类到 VM（容器保存全名，load 时回退短名）
        use gorge_core::objective::class::RuntimeClass;
        use gorge_core::objective::declaration::ClassDeclaration;
        for cc in &mgr.compiled_classes {
            let name = cc.class_type.full_name().rsplit('.').next().unwrap().to_string();
            let decl = ClassDeclaration {
                class_type: cc.class_type.clone(),
                method_start_id: cc.method_start_id,
                method_count: cc.methods.len(),
                method_annotations: cc.method_annotations.clone(),
                ..ClassDeclaration::dummy(cc.class_type.full_name())
            };
            let mut rc = RuntimeClass::new(decl, None);
            for (i, m) in cc.methods.iter().enumerate() {
                rc.register_method(i, m.clone());
            }
            vm.register_runtime_class(&name, std::sync::Arc::new(rc));
            // 注入器常量池注册（P0-7 前 VM 执行路径的测试侧填充）
            vm.injector_constants = cc.injector_constants.clone();
        }

        mgr.scan_forms_into_owned(&mut vm);
        mgr.state = RuntimeState::Compiled;
        mgr.extract_simulation_resources(0.0, 100.0, 1.0, &mut vm);

        // 即时音效表从自持容器的方法表装载：方法可执行，返回注入器对象 ID
        let score = mgr.score.as_ref().unwrap();
        assert_eq!(score.instant_audio.len(), 1, "应装载 1 个即时音效");
        let entry = score.instant_audio.get("RespondA").expect("RespondA 应存在");
        let obj_id = entry["__object_id"].as_u64().unwrap() as usize;
        assert!(obj_id > 0, "静态方法应返回有效对象 ID");
        use gorge_core::system::native::injector::Injector;
        assert!(vm.injectors.contains_key(&obj_id), "返回对象应为注入器");
        assert_eq!(
            vm.injectors[&obj_id].injection_class_declaration().class_type.full_name(),
            "GorgeFramework.AudioAsset"
        );
    }
}
