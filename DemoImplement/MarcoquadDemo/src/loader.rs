//! Gorge 游戏加载流水线
//!
//! 从 gorge_package 目录加载 Native/Modal/Chart 三个 zip 包，
//! 编译全部 .g 源码并注册到虚拟机，加载资产后驱动仿真主循环。

use std::collections::HashMap;
use std::sync::Arc;

use gorge_compiler::compile_sources;
use gorge_compiler::frontend::ast::SourceFile;
use gorge_compiler::frontend::lexer;
use gorge_compiler::frontend::parser::Parser;

use gorge_core::diagnostics::Diagnostics;
use gorge_core::objective::bytecode::{CompiledClass, CompiledModule, InjectorConstField};
use gorge_core::objective::class::RuntimeClass;
use gorge_core::objective::declaration::{ClassDeclaration, InjectorFieldInfo};
use gorge_core::objective::types::{BasicType, GorgeType, TypeCount};
use gorge_core::virtual_machine::ir::{CompiledMethod, ValueType};
use gorge_core::virtual_machine::vm::VirtualMachine;

use gorge_framework::chart::package::Package;
use gorge_framework::runtime::runtime_manager::{RuntimeManager, RuntimeState};

use crate::adaptor;

/// 从完整限定名提取简单类名（最后一段）
fn simple_name(full: &str) -> String {
    full.rsplit('.').next().unwrap_or(full).to_string()
}

fn injector_field_type(value_type: ValueType) -> GorgeType {
    GorgeType::new(match value_type {
        ValueType::Int => BasicType::Int,
        ValueType::Float => BasicType::Float,
        ValueType::Bool => BasicType::Bool,
        ValueType::String => BasicType::String,
        ValueType::Object => BasicType::Object,
    })
}

// ==================== 包加载 ====================

/// 从 gorge_package 目录加载三个 zip 文件
fn load_all_packages() -> Result<(Package, Package, Package), String> {
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("gorge_package");
    let base_str = base.to_string_lossy();
    let native_pkg = Package::load_zip_package(format!("{}/Native.zip", base_str), false)
        .map_err(|e| format!("加载 Native.zip 失败: {}", e))?;
    let modal_pkg = Package::load_zip_package(format!("{}/Dremu.zip", base_str), false)
        .map_err(|e| format!("加载 Dremu.zip 失败: {}", e))?;
    let chart_pkg = Package::load_zip_package(format!("{}/DremuTest.zip", base_str), true)
        .map_err(|e| format!("加载 DremuTest.zip 失败: {}", e))?;
    Ok((native_pkg, modal_pkg, chart_pkg))
}

// ==================== 编译 ====================

/// 诊断源码与 `Span::source_id` 的映射项。
struct DiagnosticSource {
    display_name: String,
    code: String,
}

/// 将诊断渲染为带 zip 包内路径的可读文本。
fn render_diagnostics(diagnostics: &Diagnostics, sources: &[DiagnosticSource]) -> String {
    let named_sources: Vec<(&str, &str)> = sources
        .iter()
        .map(|source| (source.display_name.as_str(), source.code.as_str()))
        .collect();
    diagnostics.render_with_source_names(&named_sources)
}

/// 将所有 Package 中的 .g 源码编译为 CompiledModule
fn compile_packages(
    native_pkg: &Package,
    modal_pkg: &Package,
    chart_pkg: &Package,
) -> Result<CompiledModule, String> {
    let mut source_files: Vec<SourceFile> = Vec::new();
    let mut diagnostic_sources: Vec<DiagnosticSource> = Vec::new();

    // 按 native → modal → chart 顺序收集，确保 native stub 先于引用它的代码
    let all_pkgs = [
        ("Native.zip", native_pkg),
        ("Dremu.zip", modal_pkg),
        ("DremuTest.zip", chart_pkg),
    ];
    let mut total = 0usize;
    for (package_name, pkg) in all_pkgs {
        for src_file in &pkg.source_code_files {
            total += 1;
            // 即使词法或语法失败也必须保留 source_id 槽位，避免后续文件的
            // 语义诊断错误映射到前一个文件。
            let source_id = diagnostic_sources.len();
            diagnostic_sources.push(DiagnosticSource {
                display_name: format!("{}!{}", package_name, src_file.path),
                code: src_file.code.clone(),
            });
            let (tokens, lexer_diags) = lexer::tokenize(&src_file.code, source_id);
            if !lexer_diags.is_empty() {
                let mut diagnostics = Diagnostics::new();
                for diagnostic in lexer_diags {
                    diagnostics.emit(diagnostic);
                }
                eprintln!("词法错误:\n{}", render_diagnostics(&diagnostics, &diagnostic_sources));
                continue;
            }
            let mut parser = Parser::new(tokens);
            match parser.parse_source_file() {
                Ok(source_file) => source_files.push(source_file),
                Err(diags) => {
                    eprintln!("语法错误:\n{}", render_diagnostics(&diags, &diagnostic_sources));
                    continue;
                }
            }
        }
    }

    eprintln!("[Gorge] 解析完成，{}/{} 文件成功，开始编译...", source_files.len(), total);

    if source_files.is_empty() {
        return Err("没有可编译的源文件".to_string());
    }

    compile_sources(&source_files, false).map_err(|diagnostics| {
        format!("编译错误:\n{}", render_diagnostics(&diagnostics, &diagnostic_sources))
    })
}

// ==================== Native 类注册 ====================

/// 将所有框架 native 类注册到虚拟机
fn register_all_native_classes(vm: &mut VirtualMachine) {
    for cls in gorge_framework::native_classes() {
        vm.register_native_class(&simple_name(cls.full_name()), cls.clone());
    }
}

// ==================== 编译类注册 ====================

/// 将编译模块中的类注册到 VM
///
/// 按继承深度排序 → 注册方法 → 注册字段计数 → 注册字段初始化器（Phase P）→
/// 注册父类 → 构造 RuntimeClass → 注册委托（V5）。native 类同样需要把编译期
/// 声明注册到 class_table，供动态注入器构造读取继承后的构造映射与全局起始 ID。
fn register_module_to_vm(vm: &mut VirtualMachine, module: &CompiledModule) {
    let compiled_map: HashMap<String, &CompiledClass> = module
        .classes
        .iter()
        .map(|c| (simple_name(&c.class_type.full_name()), c))
        .collect();

    let mut ordered: Vec<&CompiledClass> = module.classes.iter().collect();
    ordered.sort_by_key(|c| {
        let mut depth = 0;
        let mut cur = c.super_class_name.clone();
        while let Some(ref n) = cur {
            depth += 1;
            cur = compiled_map
                .get(&simple_name(n))
                .and_then(|x| x.super_class_name.clone());
        }
        depth
    });

    let mut rc_map: HashMap<String, Arc<RuntimeClass>> = HashMap::new();

    for cc in ordered {
        let name = simple_name(&cc.class_type.full_name());

        // 注入器常量池注册（P0-7）：编译器把常量索引按类连续分配
        // （`CompiledClass.injector_constants`），VM 的 LoadInjectorConstant
        // 按全局索引寻址。合并顺序必须与注册顺序一致（按继承深度排序，
        // 基类常量在前），否则索引错位。
        vm.injector_constants.extend(cc.injector_constants.clone());

        // 注册方法实现
        let mut mp: Vec<(CompiledMethod, Vec<ValueType>)> = Vec::new();
        for m in &cc.methods {
            mp.push((m.clone(), vec![]));
        }
        vm.register_class_methods(&name, mp);

        // 注册字段计数（含父类继承字段）
        let tf = TypeCount {
            int_count: cc.field_start_counts[0] + cc.field_counts.int_count,
            float_count: cc.field_start_counts[1] + cc.field_counts.float_count,
            bool_count: cc.field_start_counts[2] + cc.field_counts.bool_count,
            string_count: cc.field_start_counts[3] + cc.field_counts.string_count,
            object_count: cc.field_start_counts[4] + cc.field_counts.object_count,
        };
        vm.register_class_field_counts(&name, tf);

        // Phase P: 注册字段初始化器
        if !cc.field_initializers.is_empty() {
            vm.register_class_field_initializers(&name, cc.field_initializers.clone());
        }

        // 注册父类
        if let Some(ref sn) = cc.super_class_name {
            vm.register_class_super(&name, &simple_name(sn));
        }

        // 查找父类 RuntimeClass
        let sa = cc
            .super_class_name
            .as_ref()
            .and_then(|sn| rc_map.get(&simple_name(sn)).cloned());

        // 构建接口方法实现映射
        let iface_map: HashMap<String, Vec<usize>> = cc
            .interface_method_impl_id
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let mut injector_fields = sa.as_ref()
            .map(|parent| parent.declaration.injector_fields.clone())
            .unwrap_or_default();
        injector_fields.extend(cc.injector_fields.iter().map(|field| InjectorFieldInfo {
                name: field.name.clone(),
                field_type: injector_field_type(field.value_type),
                is_array: field.is_array,
                has_default_value: field.has_default,
                default_value: field.default_value.clone(),
            }));
        let mut injector_field_type_count = sa.as_ref()
            .map(|parent| parent.declaration.injector_field_type_count.clone())
            .unwrap_or_else(TypeCount::zero);
        for field in &cc.injector_fields {
            match field.value_type {
                ValueType::Int => injector_field_type_count.int_count += 1,
                ValueType::Float => injector_field_type_count.float_count += 1,
                ValueType::Bool => injector_field_type_count.bool_count += 1,
                ValueType::String => injector_field_type_count.string_count += 1,
                ValueType::Object => injector_field_type_count.object_count += 1,
            }
        }
        // G4: 统计各类型注入器默认值数量（对齐 vm_main.rs）。
        // 元素构造时 `LoadIntInjectorField` 等会按字段是否声明默认值
        // 读取 `injector_defaults`，容量不足会直接 panic。
        let mut default_type_count = TypeCount::zero();
        for field in &cc.injector_fields {
            if field.default_value.is_some() {
                match field.value_type {
                    ValueType::Int => default_type_count.int_count += 1,
                    ValueType::Float => default_type_count.float_count += 1,
                    ValueType::Bool => default_type_count.bool_count += 1,
                    ValueType::String => default_type_count.string_count += 1,
                    ValueType::Object => default_type_count.object_count += 1,
                }
            }
        }

        let decl = ClassDeclaration {
            class_type: cc.class_type.clone(),
            is_native: cc.is_native,
            annotations: vec![],
            fields: vec![],
            methods: vec![],
            static_methods: vec![],
            constructors: vec![],
            injector_fields,
            super_class: sa.as_ref().map(|a| Box::new(a.declaration.clone())),
            super_interfaces: cc.super_interfaces.clone(),
            field_type_count: cc.field_counts.clone(),
            method_count: cc.methods.len(),
            static_method_count: 0,
            constructor_count: cc.constructors.len(),
            injector_field_type_count,
            injector_field_default_value_type_count: default_type_count,
            method_start_id: cc.method_start_id,
            constructor_start_id: cc.constructor_start_id,
            interface_method_impl_id: iface_map,
            method_override_id: cc.method_override_id.iter().cloned().collect(),
            injector_constructor_impl_id: cc.injector_constructor_impl_id.clone(),
            method_annotations: cc.method_annotations.clone(),
            constructor_annotations: cc.constructor_annotations.clone(),
        };

        let mut rc = RuntimeClass::new(decl, sa);
        // G4: 将注入器字段默认值写入 RuntimeClass.injector_defaults，
        // 使谱面 JSON 未显式提供的字段（如 leadTime 默认 1.5）能取到默认值。
        for field in &cc.injector_fields {
            if let Some(ref dv) = field.default_value {
                // 计算当前字段在同类型默认值中的偏移
                let mut idx = 0;
                for fd in &cc.injector_fields {
                    if fd.name == field.name { break; }
                    if fd.default_value.is_some() && fd.value_type == field.value_type { idx += 1; }
                }
                match (field.value_type, dv) {
                    (ValueType::Int, InjectorConstField::Int(_, v)) => rc.injector_defaults.set_int(idx, *v),
                    (ValueType::Float, InjectorConstField::Float(_, v)) => rc.injector_defaults.set_float(idx, *v),
                    (ValueType::Bool, InjectorConstField::Bool(_, v)) => rc.injector_defaults.set_bool(idx, *v),
                    (ValueType::String, InjectorConstField::String(_, v)) => rc.injector_defaults.set_string(idx, v.clone()),
                    _ => {}
                }
            }
        }
        for (i, m) in cc.methods.iter().enumerate() {
            rc.register_method(i, m.clone());
        }
        for (i, ct) in cc.constructors.iter().enumerate() {
            rc.register_constructor(i, ct.clone());
        }
        let arc = Arc::new(rc);
        rc_map.insert(name.clone(), arc.clone());
        vm.register_runtime_class(&name, arc);

        // V5: 注册委托实现
        let mut cls_delegates: Vec<(
            CompiledMethod,
            Vec<ValueType>,
            ValueType,
            Vec<ValueType>,
        )> = Vec::new();
        for delegate in &cc.delegate_impls {
            cls_delegates.push((
                CompiledMethod {
                    name: "lambda".into(),
                    codes: delegate.body_ir.clone(),
                    local_count: 16,
                },
                delegate.param_types.clone(),
                delegate.return_type,
                delegate.captured_var_types.clone(),
            ));
        }
        vm.register_class_delegates(&name, cls_delegates);
    }
}

// ==================== GameLoader ====================

/// 游戏加载器
///
/// 封装从 zip 包加载、编译、注册 VM、资产加载到仿真驱动的完整流程。
pub struct GameLoader {
    /// Gorge 虚拟机
    pub vm: VirtualMachine,
    /// 运行时管理器
    pub runtime_manager: RuntimeManager,
    /// 当前仿真时间（秒）
    simulation_time: f32,
    /// 上次音频状态诊断时间（秒）
    last_audio_diagnostic: f32,
}

impl GameLoader {
    /// 创建空的游戏加载器
    pub fn new() -> Self {
        Self {
            vm: VirtualMachine::new(),
            runtime_manager: RuntimeManager::new(),
            simulation_time: 0.0,
            last_audio_diagnostic: 0.0,
        }
    }

    /// 执行完整加载流程
    ///
    /// 1. 注册所有 native 类
    /// 2. 加载三个 zip 包
    /// 3. 编译全部 .g 源码
    /// 4. 注册编译类到 VM
    /// 5. 提取仿真资源 → 加载资产 → 创建运行时 → 加载谱面 → 启动仿真
    pub fn load_all(&mut self) -> Result<(), String> {
        // 1. 注册 native 类
        eprintln!("[Gorge] 1/7 注册 native 类...");
        register_all_native_classes(&mut self.vm);

        // 2. 加载 zip 包
        eprintln!("[Gorge] 2/7 加载 zip 包...");
        let (native_pkg, modal_pkg, chart_pkg) = load_all_packages()?;
        eprintln!(
            "[Gorge] 源码文件: native={} modal={} chart={}",
            native_pkg.source_code_files.len(),
            modal_pkg.source_code_files.len(),
            chart_pkg.source_code_files.len()
        );

        // 3. 编译所有 .g 源码
        eprintln!("[Gorge] 3/7 编译源码...");
        let module = compile_packages(&native_pkg, &modal_pkg, &chart_pkg)?;
        eprintln!("[Gorge] 编译完成，共 {} 个类", module.classes.len());

        // 4. 注册编译类到 VM
        eprintln!("[Gorge] 4/7 注册编译类...");
        register_module_to_vm(&mut self.vm, &module);
        self.runtime_manager.set_compiled_classes(module.classes.clone());
        eprintln!("[Gorge] 注册完成");

        // 5. 走到 Compiled 状态，扫描模态进自持 FormContainer（P0-6：
        //    模态表 / Element 修改器表 / 即时音效方法表，供 prepare_score
        //    与 ChartManager 读取）
        self.runtime_manager.state = RuntimeState::Compiled;
        eprintln!("[Gorge] 5/7 提取仿真资源...");
        self.runtime_manager.scan_forms_into_owned(&mut self.vm);

        // 6. 提取仿真资源
        self.runtime_manager
            // 起始谱面时间对齐 C# 参考实现（RuntimeManager.cs: `new SimulationScore(-1, ...)`）：
            // 从 -1s 起步，保证 generateTime=0 的轨道（严格 `time > chart_from` 判定）
            // 在仿真开始时被正常生成，音符 FindAliveLane 才能找到判定线。
            .extract_simulation_resources(-1.0, 100.0, 1.0, &mut self.vm);

        // 7. 资产加载
        eprintln!("[Gorge] 6/7 加载资产...");
        if let Some(ref mut score) = self.runtime_manager.score {
            for pkg in [&native_pkg, &modal_pkg, &chart_pkg] {
                score.extract_assets_from_package(pkg);
            }
        }
        self.runtime_manager.reload_assets(&mut self.vm);

        // 8. 初始化并启动仿真
        eprintln!("[Gorge] 7/7 启动仿真...");
        self.runtime_manager
            // 仿真机起始谱面时间与谱面资源一致（C# 参考为 -1s）：
            // 若 SimulationMachine 仍从 0 起步，generateTime=0 的轨道会因
            // 严格 `time > chart_from` 判定被跳过，音符 FindAliveLane 找不到判定线。
            .create_simulation_runtime(-1.0, 100.0, 1.0, None);
        self.runtime_manager.load_score(&mut self.vm);
        self.runtime_manager.start_simulation(&mut self.vm);
        let score_element_count = self
            .runtime_manager
            .score
            .as_ref()
            .map(|score| {
                score
                    .stave
                    .iter()
                    .filter_map(|staff| {
                        staff
                            .as_any()
                            .downcast_ref::<gorge_framework::chart::staff::ElementStaff>()
                    })
                    .flat_map(|staff| staff.periods.iter())
                    .map(|period| period.elements.len())
                    .sum::<usize>()
            })
            .unwrap_or(0);
        if let Some(runtime) = self.runtime_manager.simulation_runtime.as_ref() {
            eprintln!(
                "[Gorge] 运行时计数: score_elements={} initialize={} forward={} backward={} alive={} nodes={}",
                score_element_count,
                runtime.chart.initialize_generate_list.len(),
                runtime.chart.forward_timed_generate_list.len(),
                runtime.chart.backward_timed_generate_list.len(),
                runtime.chart.alive_elements.len(),
                runtime.graphics.nodes.len(),
            );
        }
        let (audio_clips, music_players, sfx_players) = adaptor::audio_resource_counts();
        let audio_period_count = self
            .runtime_manager
            .simulation_runtime
            .as_ref()
            .map(|rt| rt.audio.period_audio_sources.len())
            .unwrap_or(0);
        eprintln!(
            "[Gorge] 音频资源: clips={} music={} sfx={} period_players={}",
            audio_clips, music_players, sfx_players, audio_period_count
        );
        eprintln!("[Gorge] 加载完成!");

        Ok(())
    }

    /// 每帧驱动仿真推进
    pub fn drive(&mut self, delta_time: f32) {
        self.simulation_time += delta_time;
        self.runtime_manager
            .drive(delta_time, &mut self.vm);
        // 每 2 秒打印一次音频播放状态（开发诊断：验证 SongSimulator 播放触发）
        if self.simulation_time - self.last_audio_diagnostic >= 2.0 {
            self.last_audio_diagnostic = self.simulation_time;
            let (alive, nodes) = self.runtime_manager.simulation_runtime.as_ref()
                .map(|runtime| (runtime.chart.alive_elements.len(), runtime.graphics.nodes.len()))
                .unwrap_or((0, 0));
            let chart_time = self.runtime_manager.machine.as_ref()
                .map(|machine| machine.chart_time)
                .unwrap_or(0.0);
            let next_generate_time = self.runtime_manager.simulation_runtime.as_ref()
                .and_then(|runtime| runtime.chart.forward_timed_generate_list.iter()
                    .map(|(time, _, _)| *time)
                    .filter(|time| *time > chart_time)
                    .min_by(|left, right| left.total_cmp(right)));
            let generate_time_range = self.runtime_manager.simulation_runtime.as_ref()
                .and_then(|runtime| {
                    let minimum = runtime.chart.forward_timed_generate_list.iter()
                        .map(|(time, _, _)| *time)
                        .min_by(|left, right| left.total_cmp(right))?;
                    let maximum = runtime.chart.forward_timed_generate_list.iter()
                        .map(|(time, _, _)| *time)
                        .max_by(|left, right| left.total_cmp(right))?;
                    Some((minimum, maximum))
                });
            let (_, sprites, nine_slices, curves) = adaptor::render_resource_counts();
            eprintln!(
                "[Gorge] t={:.2} chart_time={:.2} generate_range={:?} next_generate={:?} alive={} nodes={} sprites={} nine_slices={} curves={} 音频: {}",
                self.simulation_time,
                chart_time,
                generate_time_range,
                next_generate_time,
                alive,
                nodes,
                sprites,
                nine_slices,
                curves,
                adaptor::audio_playback_diagnostics()
            );
        }
    }

    /// 返回当前仿真时间（秒）
    pub fn simulation_time(&self) -> f32 {
        self.simulation_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 加载真实 Dremu 包并用无头平台验证元素、节点和渲染对象确实被创建。
    #[test]
    #[ignore = "读取真实谱面资源，作为发布前端到端验收单独运行"]
    fn dremu_load_creates_visible_elements_with_headless_platform() {
        use gorge_framework::adaptor::{install_platform, CallEntry, HeadlessPlatform};
        use gorge_core::system::native::injector::Injector;

        let platform = HeadlessPlatform::new();
        let call_log = platform.call_log();
        install_platform(Box::new(platform));

        let mut loader = GameLoader::new();
        loader.load_all().expect("真实 Dremu 包应能完整加载");

        let runtime = loader.runtime_manager.simulation_runtime.as_ref()
            .expect("应创建仿真运行时");

        // 1. 生成时间必须非零（此前 Lambda 隐藏方法返回委托对象导致全 0）
        assert!(
            !runtime.chart.forward_timed_generate_list.is_empty(),
            "应存在待生成元素",
        );
        assert!(
            runtime.chart.forward_timed_generate_list.iter().any(|(t, _, _)| *t != 0.0),
            "生成时间不得全为 0",
        );

        // 2. 手动实例化首元素（DremuMainLane）与引导轨道，验证完整构造链
        let (_, injector_id, constructor_id) = runtime.chart.forward_timed_generate_list[0];
        let class_name = loader.vm.injectors.get(&injector_id)
            .map(|inj| inj.injection_class_declaration().class_type.full_name())
            .expect("元素注入器应存在");
        loader.vm.instantiate_with_injector(&class_name, constructor_id, injector_id)
            .unwrap_or_else(|error| panic!("首个真实元素构造失败: {}", error));

        if let Some((_t, guide_injector_id, guide_ctor_id)) = runtime.chart.forward_timed_generate_list.iter()
            .find(|(_, inj_id, _)| loader.vm.injectors.get(inj_id)
                .map(|inj| inj.injection_class_declaration().class_type.name().contains("GuideLane"))
                .unwrap_or(false))
            .copied()
        {
            let guide_class = loader.vm.injectors.get(&guide_injector_id)
                .map(|inj| inj.injection_class_declaration().class_type.full_name())
                .unwrap_or_default();
            loader.vm.instantiate_with_injector(&guide_class, guide_ctor_id, guide_injector_id)
                .unwrap_or_else(|error| panic!("引导轨道构造失败: {}", error));
        }

        // 3. 驱动仿真 30 秒：真实谱面唯一的长寿命轨道（ArtLine6.5）只在
        //    7.37s~7.58s 存活，因此用峰值跟踪而不是终点状态验收
        let mut peak_alive = 0usize;
        let mut peak_nodes = 0usize;
        for _ in 0..300 {
            loader.drive(0.1);
            if let Some(runtime) = loader.runtime_manager.simulation_runtime.as_ref() {
                peak_alive = peak_alive.max(runtime.chart.alive_elements.len());
                peak_nodes = peak_nodes.max(runtime.graphics.nodes.len());
            }
        }
        assert!(peak_alive > 0, "仿真过程中应至少生成过一个元素");
        assert!(peak_nodes > 0, "仿真过程中应登记过图形节点");

        // 4. 渲染调用日志是累计记录：轨道生命周期内创建的精灵必须出现
        let calls = call_log.lock().unwrap();
        assert!(calls.iter().any(|entry| matches!(
            entry,
            CallEntry::CreateSprite { .. }
                | CallEntry::CreateNineSliceSprite { .. }
                | CallEntry::CreateCurveSprite { .. }
        )), "应创建至少一种可渲染精灵");

        // 5. 判定线（CurveSprite）必须把真实曲线点坐标上传到平台：
        //    只传点数会导致平台 points 全为 (0,0)，画面画不出判定线。
        assert!(calls.iter().any(|entry| matches!(
            entry,
            CallEntry::CurveSetPoints { points, .. } if points.iter().any(|(x, y)| *x != 0.0 || *y != 0.0)
        )), "应上传至少一个非零曲线点坐标（判定线可见性验收）");
    }

    #[test]
    fn register_module_to_vm_keeps_native_injector_constructor_metadata() {
        let (tokens, lexer_diagnostics) = lexer::tokenize(r#"
native class BaseCurve
{
    injector BaseCurve();
}
native class ConcreteCurve : BaseCurve
{
    ConcreteCurve();
}
"#, 0);
        assert!(lexer_diagnostics.is_empty());
        let source = Parser::new(tokens)
            .parse_source_file()
            .expect("native 注入器构造测试源码应能解析");
        let module = compile_sources(&[source], false)
            .expect("native 子类应能继承注入器构造契约");

        let mut vm = VirtualMachine::new();
        register_module_to_vm(&mut vm, &module);

        let concrete = vm.class_table.get("ConcreteCurve")
            .expect("native 编译声明应注册到 class_table");
        assert!(concrete.declaration.is_native);
        assert_eq!(concrete.declaration.constructor_start_id, 1);
        assert_eq!(concrete.declaration.injector_constructor_impl_id, vec![1]);
    }
}
