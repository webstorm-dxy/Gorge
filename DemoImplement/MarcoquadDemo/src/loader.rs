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
use gorge_core::objective::bytecode::{CompiledClass, CompiledModule};
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
/// 逻辑与 GorgeRunner 完全一致：按继承深度排序 → 注册方法 → 注册字段计数 →
/// 注册字段初始化器（Phase P）→ 注册父类 → 构造 RuntimeClass → 注册委托（V5）。
fn register_module_to_vm(vm: &mut VirtualMachine, module: &CompiledModule) {
    let compiled_map: HashMap<String, &CompiledClass> = module
        .classes
        .iter()
        .filter(|c| !c.is_native)
        .map(|c| (simple_name(&c.class_type.full_name()), c))
        .collect();

    let mut ordered: Vec<&CompiledClass> =
        module.classes.iter().filter(|c| !c.is_native).collect();
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
                has_default_value: field.has_default,
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

        let decl = ClassDeclaration {
            class_type: cc.class_type.clone(),
            is_native: false,
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
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: cc.method_start_id,
            constructor_start_id: cc.constructor_start_id,
            interface_method_impl_id: iface_map,
            method_override_id: cc.method_override_id.iter().cloned().collect(),
            injector_constructor_impl_id: cc.injector_constructor_impl_id.clone(),
            method_annotations: cc.method_annotations.clone(),
            constructor_annotations: cc.constructor_annotations.clone(),
        };

        let mut rc = RuntimeClass::new(decl, sa);
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
            .extract_simulation_resources(0.0, 100.0, 1.0, &mut self.vm);

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
            .create_simulation_runtime(0.0, 100.0, 1.0, None);
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
            eprintln!(
                "[Gorge] t={:.2} 音频: {}",
                self.simulation_time,
                adaptor::audio_playback_diagnostics()
            );
        }
    }

    /// 返回当前仿真时间（秒）
    pub fn simulation_time(&self) -> f32 {
        self.simulation_time
    }
}
