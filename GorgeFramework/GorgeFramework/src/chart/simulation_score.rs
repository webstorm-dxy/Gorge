//! 仿真总谱（对应 C# `Chart/SimulationScore.cs`）。
//!
//! 包含谱面所有元素和资源信息，向上与运行时环境对接，向下与谱面文件互转。

use std::collections::HashMap;
use std::sync::Arc;
use gorge_core::objective::bytecode::{CompiledClass, InjectorConstField};
use gorge_core::virtual_machine::ir::{IntermediateOperator, ValueType};
use crate::chart::package::{AssetFile, Package};
use crate::chart::period::{AudioPeriod, ElementPeriod, IPeriod};
use crate::chart::staff::{AudioStaff, ElementStaff, IStaff};

// ==================== 资产后端抽象 ====================

/// 资产后端抽象（对应 C# `PlatformBase` / `Base.Instance` 的 CreateGraph/CreateAudio/CreateVideo）。
///
/// 真实实现需在 `adaptor/mod.rs` 的 `PlatformBase` trait 上补齐，此处定义 trait
/// 供 `SimulationScore::add_file_asset` 使用。测试用 `MockAssetBackend` 提供空实现。
///
/// **需要 PlatformBase 补的方法清单（F 步）**：
/// - `fn create_graph(&self, path: &str, data: &[u8]) -> Result<usize, String>` — 创建图形资源
/// - `fn create_audio(&self, path: &str, data: &[u8]) -> Result<usize, String>` — 创建音频资源
/// - `fn create_video(&self, path: &str, data: &[u8]) -> Result<usize, String>` — 创建视频资源
pub trait AssetBackend {
    /// 从字节数据创建图形资源，返回句柄
    fn create_graph(&mut self, path: &str, data: &[u8]) -> Result<usize, String>;
    /// 从字节数据创建音频资源，返回句柄
    fn create_audio(&mut self, path: &str, data: &[u8]) -> Result<usize, String>;
    /// 从字节数据创建视频资源，返回句柄
    fn create_video(&mut self, path: &str, data: &[u8]) -> Result<usize, String>;
}

/// 测试用 Mock 资产后端，所有方法返回 0 句柄。
pub struct MockAssetBackend;

impl AssetBackend for MockAssetBackend {
    fn create_graph(&mut self, _path: &str, _data: &[u8]) -> Result<usize, String> {
        Ok(0)
    }
    fn create_audio(&mut self, _path: &str, _data: &[u8]) -> Result<usize, String> {
        Ok(0)
    }
    fn create_video(&mut self, _path: &str, _data: &[u8]) -> Result<usize, String> {
        Ok(0)
    }
}

// ==================== 资源加载器相关类型 ====================

/// 已加载的资产（简化表示）。
///
/// 对应 C# `Asset` 基类。`handle` 是平台资源句柄，不是 VM 对象 ID；
/// `Environment.GetAssetByName` 会在首次查询时将其包装为对应的 VM 资产对象。
#[derive(Debug, Clone)]
pub struct Asset {
    /// 资源名（如 "image:path"、"audio:path"）
    pub name: String,
    /// 资源句柄（对应 native 对象 ID）
    pub handle: usize,
}

/// 资源组（对应 C# `AssetSet`）。
///
/// 对应一个 Gorge 方法，包含一组资源注入器。
#[derive(Debug, Clone)]
pub struct AssetSet {
    /// 方法名
    pub method_name: String,
    /// 资源注入器列表（以 JSON 形式存储）
    pub assets: Vec<serde_json::Value>,
}

impl AssetSet {
    pub fn new(method_name: String) -> Self {
        Self { method_name, assets: Vec::new() }
    }

    pub fn deep_clone(&self) -> Self {
        Self {
            method_name: self.method_name.clone(),
            assets: self.assets.clone(),
        }
    }
}

/// 资源加载器（对应 C# `AssetLoader`）。
///
/// 对应一个 Gorge 类，包含多个资源组。
#[derive(Debug, Clone)]
pub struct AssetLoader {
    /// 资源加载器类名
    pub class_name: String,
    /// 本加载器属于谱面还是模态
    pub is_chart_class: bool,
    /// 资源组列表
    pub asset_sets: Vec<AssetSet>,
}

impl AssetLoader {
    pub fn new(class_name: String, is_chart_class: bool) -> Self {
        Self { class_name, is_chart_class, asset_sets: Vec::new() }
    }

    pub fn deep_clone(&self) -> Self {
        Self {
            class_name: self.class_name.clone(),
            is_chart_class: self.is_chart_class,
            asset_sets: self.asset_sets.iter().map(|s| s.deep_clone()).collect(),
        }
    }
}

// ==================== SimulationScore ====================

/// Gorge 仿真总谱（对应 C# `SimulationScore`）。
///
/// 包含来自所有模态和谱面的元素与资源信息。
#[derive(Debug)]
pub struct SimulationScore {
    /// 仿真起点（秒）
    pub start_time: f32,
    /// 仿真终点（秒）
    pub terminate_time: f32,
    /// 仿真倍速
    pub simulation_speed: f32,
    /// 谱表列表
    pub stave: Vec<Box<dyn IStaff>>,
    /// 谱面资源文件表
    pub chart_asset_files: Vec<AssetFile>,
    /// 资源加载器表
    pub asset_loaders: Vec<AssetLoader>,
    /// 即时播放音效（名称→资产）
    pub instant_audio: HashMap<String, serde_json::Value>,
    /// 已加载的资源（名称→资产）
    pub loaded_assets: HashMap<String, Asset>,
}

impl SimulationScore {
    /// 创建新的仿真总谱
    pub fn new(start_time: f32, terminate_time: f32, simulation_speed: f32) -> Self {
        Self {
            start_time,
            terminate_time,
            simulation_speed,
            stave: Vec::new(),
            chart_asset_files: Vec::new(),
            asset_loaders: Vec::new(),
            instant_audio: HashMap::new(),
            loaded_assets: HashMap::new(),
        }
    }

    /// 从资源包中提取资产（对应 C# `ExtractAssetsFromPackage`）
    pub fn extract_assets_from_package(&mut self, package: &Package) {
        for asset_file in &package.asset_files {
            self.chart_asset_files.push(asset_file.clone());
        }
    }

    /// 从资源文件中自动加载资源到命名资源表中（对应 C# `AddFileAsset`）。
    ///
    /// 遍历 `chart_asset_files`，按扩展名调用 backend 创建图/音频/视频资源。
    /// 扩展名映射：`.png`/`.jpg` → graph，`.wav`/`.mp3`/`.ogg` → audio，`.mp4` → video。
    pub fn add_file_asset(&mut self, backend: &mut dyn AssetBackend) {
        self.asset_loaders.clear();

        let mut asset_loader = AssetLoader::new("AutoLoaded".to_string(), false);
        let mut asset_set = AssetSet::new("AutoLoaded".to_string());

        for asset_file in &self.chart_asset_files {
            let path = &asset_file.path;
            let extension = std::path::Path::new(path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            match extension {
                "png" | "jpg" => {
                    if let Ok(handle) = backend.create_graph(path, &asset_file.data) {
                        let name = format!("image:{}", normalized_asset_stem(path, extension));
                        asset_set.assets.push(serde_json::json!({
                            "name": name,
                            "texture": handle,
                            "type": "image"
                        }));
                    }
                }
                "wav" | "mp3" | "ogg" => {
                    if let Ok(handle) = backend.create_audio(path, &asset_file.data) {
                        let name = format!("audio:{}", normalized_asset_stem(path, extension));
                        asset_set.assets.push(serde_json::json!({
                            "name": name,
                            "audio": handle,
                            "type": "audio"
                        }));
                    }
                }
                "mp4" => {
                    if let Ok(handle) = backend.create_video(path, &asset_file.data) {
                        let name = format!("video:{}", normalized_asset_stem(path, extension));
                        asset_set.assets.push(serde_json::json!({
                            "name": name,
                            "video": handle,
                            "type": "video"
                        }));
                    }
                }
                _ => {}
            }
        }

        asset_loader.asset_sets.push(asset_set);
        self.asset_loaders.push(asset_loader);
    }

    /// 加载资源加载器中的全部资源（对应 C# `LoadAssets`）。
    ///
    /// 骨架实现：遍历 asset_loaders 的所有 asset_sets 中的 assets，
    /// 将每个条目记录到 loaded_assets 中。
    /// 完整实现需要 Gorge 注入器实例化系统（`Injector.Instantiate`）。
    pub fn load_assets(&mut self) {
        self.loaded_assets.clear();

        for asset_loader in &self.asset_loaders {
            for asset_set in &asset_loader.asset_sets {
                for asset_injector in &asset_set.assets {
                    if let Some(name) = asset_injector.get("name").and_then(|v| v.as_str()) {
                        let handle = asset_injector
                            .get("texture")
                            .or_else(|| asset_injector.get("audio"))
                            .or_else(|| asset_injector.get("video"))
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0) as usize;
                        self.loaded_assets.insert(
                            name.to_string(),
                            Asset {
                                name: name.to_string(),
                                handle,
                            },
                        );
                    }
                }
            }
        }
    }

    /// 按名称获取已加载的资产（对应 C# `GetAssetByName`）
    pub fn get_asset_by_name(&self, asset_name: &str) -> Option<&Asset> {
        self.loaded_assets.get(asset_name)
    }

    /// 加载响应音效（对应 C# `LoadInstantAudio`，P0-6 接入 FormContainer）。
    ///
    /// 遍历 `FormContainer.InstantAudioMethods`（音效名 → 静态方法引用），
    /// 通过 VM 调用对应静态方法取得 `AudioAsset` 对象，存入 `instant_audio` 表
    /// （对齐 C# `gorgeClass.InvokeStaticMethod(method, Array.Empty<object>())`）。
    ///
    /// 容器保存类全名，而 Demo/loader 以简单名注册编译类到 VM（与
    /// `ChartManager::resolve_registered_class_name` 相同的双键约定），
    /// 调用前先做名称解析。方法调用失败（类未注册 / 方法缺失 /
    /// 返回值非对象）时跳过该项。
    pub fn load_instant_audio(
        &mut self,
        form_container: &crate::runtime::runtime_form_container::RuntimeFormContainer,
        vm: &mut gorge_core::virtual_machine::vm::VirtualMachine,
    ) {
        self.instant_audio.clear();

        for (name, method_ref) in &form_container.instant_audio_methods {
            // 全名未命中时回退末段短名（loader 注册约定）
            let class_name = if vm.class_table.contains_key(&method_ref.class_name) {
                method_ref.class_name.clone()
            } else {
                let simple = method_ref.class_name
                    .rsplit('.')
                    .next()
                    .unwrap_or(&method_ref.class_name);
                if vm.class_table.contains_key(simple) {
                    simple.to_string()
                } else {
                    continue;
                }
            };

            let Ok(()) = vm.invoke_method_by_id(&class_name, None, method_ref.method_id) else {
                continue;
            };
            // 返回值对象 ID（0 表示 null）→ JSON 化保存，供 AudioManager 延迟物化
            if let Some(obj_id) = vm.return_object.filter(|id| *id != 0) {
                self.instant_audio.insert(
                    name.clone(),
                    serde_json::json!({ "__object_id": obj_id }),
                );
            }
        }
    }

    /// 从 Gorge 语言运行时提取谱表（对应 C# `ExtractStaveFromRuntime`）。
    ///
    /// 遍历已编译类的类级注解（`@AudioStaff`/`@ElementStaff`），
    /// 为匹配的类创建谱表对象。
    ///
    /// `class_table` 为 VM 中已注册的编译类表（类名 → RuntimeClass）。
    /// 静态方法调用（`InvokeStaticMethod`）因 Compile 集成未完成暂为占位，
    /// 当前仅记录找到的类名到谱表空集合。
    ///
    /// **TODO**：接入静态方法调用后，需按 C# 逻辑调用谱表类的隐藏静态方法
    /// 以提取乐段（period）和资产（asset）注入器数据。
    pub fn extract_stave_from_runtime(
        &mut self,
        class_table: &HashMap<String, Arc<gorge_core::objective::class::RuntimeClass>>,
    ) {
        self.stave.clear();

        for (class_full_name, class) in class_table {
            let decl = &class.declaration;
            // 遍历类级注解
            for ann in &decl.annotations {
                match ann.name.as_str() {
                    "AudioStaff" => {
                        let staff = AudioStaff::new(
                            class_full_name.clone(),
                            false, // is_chart_class 需从注解参数推断，暂默认 false
                            class_full_name.clone(), // display_name
                        );
                        self.stave.push(Box::new(staff));
                    }
                    "ElementStaff" => {
                        // 取第一个参数作为 form_name（对齐 C# 注解参数字段）
                        let form_name = ann.arguments.first().cloned().unwrap_or_default();
                        let staff = ElementStaff::new(
                            class_full_name.clone(),
                            false,
                            class_full_name.clone(),
                            form_name,
                        );
                        self.stave.push(Box::new(staff));
                    }
                    _ => {}
                }
            }
        }
    }

    /// 从编译类列表中提取谱表（对应 C# `ExtractStaveFromRuntime`，Rust 简化版）。
    ///
    /// 扫描带 `@ElementStaff` / `@AudioStaff` 注解的类，
    /// 进一步扫描类中带 `@Chart` / `@Song` 注解的静态方法，
    /// 提取乐段（period）配置并生成 ElementStaff / AudioStaff 对象。
    ///
    /// # 参数
    /// - `compiled_classes`: 编译后的类列表，须已包含注解信息（Phase Q3 + S3）
    ///
    /// # 提取逻辑
    /// 1. 遍历所有编译类，检查类级注解
    /// 2. 对于匹配的类，查找静态方法注解（`@Chart` / `@Song`）
    /// 3. 从方法注解的 `config` 参数提取 `PeriodConfig`
    /// 4. 定位方法字节码中的 `LoadInjectorConstant`，将该方法返回的注入器常量
    ///    转换为 JSON 填入 `ElementPeriod.elements` / `AudioPeriod.audio_injector`
    pub fn extract_staves_from_compiled(
        &mut self,
        compiled_classes: &[gorge_core::objective::bytecode::CompiledClass],
    ) {
        self.stave.clear();

        // 构建统一的注入器字段元数据视图（编译类 + native 类，含继承合并），
        // 供嵌套常量字段按位置对齐恢复字段名
        let meta = InjectorFieldMetaProvider::build(compiled_classes);

        for cc in compiled_classes {
            let class_name = cc.class_type.full_name();

            // 扫描类级注解
            for ann in &cc.annotations {
                match ann.name.as_str() {
                    "AudioStaff" => {
                        let display_name = ann.find_argument("displayName")
                            .cloned()
                            .unwrap_or_else(|| class_name.to_string());

                        let mut staff = AudioStaff::new(
                            class_name.to_string(),
                            true, // is_chart_class
                            display_name,
                        );

                        // 扫描带 @Song 注解的静态方法，提取音频乐段
                        Self::extract_audio_periods_from_class(cc, &mut staff, &meta);

                        self.stave.push(Box::new(staff));
                        break; // 一个类只对应一个谱表类型
                    }
                    "ElementStaff" => {
                        let form_name = ann.find_argument("form")
                            .or_else(|| ann.first_positional_argument())
                            .cloned()
                            .unwrap_or_default();

                        let display_name = ann.find_argument("displayName")
                            .cloned()
                            .unwrap_or_else(|| class_name.to_string());

                        let mut staff = ElementStaff::new(
                            class_name.to_string(),
                            true, // is_chart_class
                            display_name,
                            form_name.clone(),
                        );

                        // 扫描带 @Chart 注解的静态方法，提取元素乐段
                        Self::extract_element_periods_from_class(cc, &mut staff, &form_name, &meta);

                        self.stave.push(Box::new(staff));
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    /// 从编译类中提取 `@Song` 方法的音频乐段
    fn extract_audio_periods_from_class(
        cc: &gorge_core::objective::bytecode::CompiledClass,
        staff: &mut AudioStaff,
        meta: &InjectorFieldMetaProvider,
    ) {
        // 方法注解表为 HashMap，按方法全局 ID 排序保证乐段顺序确定（声明顺序）
        let mut method_ids: Vec<&usize> = cc.method_annotations.keys().collect();
        method_ids.sort();
        for method_id in method_ids {
            for ann in &cc.method_annotations[method_id] {
                if ann.name != "Song" {
                    continue;
                }
                // 从方法注解的 config 参数提取 PeriodConfig 注入器（走完整注入器实例化路径）
                let config_injector = extract_period_config_injector(cc, ann, meta);
                // @Song 方法返回单个音频资产注入器：定位方法返回的注入器常量并转 JSON
                let audio_injector = method_injector_constant_index(cc, *method_id)
                    .and_then(|idx| cc.injector_constants.get(idx))
                    .filter(|c| c.class_name != "Array")
                    .map(|c| const_object_to_json(&c.class_name, &c.fields, meta));

                let period = AudioPeriod::new(
                    method_name_by_id(cc, *method_id).to_string(),
                    config_injector,
                    audio_injector,
                );
                staff.periods.push(period);
            }
        }
    }

    /// 从编译类中提取 `@Chart` 方法的元素乐段
    fn extract_element_periods_from_class(
        cc: &gorge_core::objective::bytecode::CompiledClass,
        staff: &mut ElementStaff,
        form_name: &str,
        meta: &InjectorFieldMetaProvider,
    ) {
        // 方法注解表为 HashMap，按方法全局 ID 排序保证乐段顺序确定（声明顺序）
        let mut method_ids: Vec<&usize> = cc.method_annotations.keys().collect();
        method_ids.sort();
        for method_id in method_ids {
            for ann in &cc.method_annotations[method_id] {
                if ann.name != "Chart" {
                    continue;
                }
                let config_injector = extract_period_config_injector(cc, ann, meta);
                let mut period = ElementPeriod::new(
                    form_name.to_string(),
                    method_name_by_id(cc, *method_id).to_string(),
                    config_injector,
                );
                // 定位方法返回的注入器数组常量，逐元素转 JSON 填入 elements
                fill_element_period_from_method(cc, *method_id, &mut period, meta);

                staff.periods.push(period);
            }
        }
    }

    /// 导出谱面包（对应 C# `ExportChartPackage`）
    pub fn export_chart_package(&self) -> Package {
        let mut package = Package::new();

        for staff in &self.stave {
            if staff.is_chart_class() {
                let class_name = staff.class_name().to_string();
                package.source_code_files.push(
                    crate::chart::package::SourceCodeFile::new(
                        format!("{}.g", class_name),
                        staff.to_gorge_code(),
                        true,
                    ),
                );
            }
        }

        for loader in &self.asset_loaders {
            if loader.is_chart_class {
                package.source_code_files.push(
                    crate::chart::package::SourceCodeFile::new(
                        format!("{}.g", loader.class_name),
                        String::new(), // AssetLoader.ToGorgeCode 需注入器系统，骨架留空
                        true,
                    ),
                );
            }
        }

        for asset_file in &self.chart_asset_files {
            if asset_file.is_chart_asset {
                package.asset_files.push(asset_file.clone());
            }
        }

        package
    }

    /// 按类名查找谱表（对应 C# `TryGetStaff`）
    pub fn try_get_staff(&self, staff_name: &str) -> Option<&Box<dyn IStaff>> {
        self.stave.iter().find(|s| s.class_name() == staff_name)
    }

    /// 按谱表名和乐段名查找乐段（对应 C# `TryGetPeriod`）
    pub fn try_get_period(&self, staff_name: &str, period_name: &str) -> Option<&dyn IPeriod> {
        let staff = self.try_get_staff(staff_name)?;
        // 通过具体类型尝试访问
        if let Some(es) = staff.as_any().downcast_ref::<ElementStaff>() {
            es.try_get_period(period_name).map(|p| p as &dyn IPeriod)
        } else if let Some(as_) = staff.as_any().downcast_ref::<AudioStaff>() {
            as_.try_get_period(period_name).map(|p| p as &dyn IPeriod)
        } else {
            None
        }
    }

    /// 检查目标谱表名是否和已有谱表名冲突（对应 C# `CheckStaffNameConflict`）
    pub fn check_staff_name_conflict(&self, staff_name: &str) -> bool {
        self.stave.iter().any(|s| s.class_name() == staff_name)
    }

    /// 从元素注入器列表构造总谱（对应 C# `LoadScoreFromElementList`）。
    ///
    /// 用于从制谱器直接生成总谱，所有元素放在一个 ElementPeriod 中。
    pub fn load_score_from_element_list(
        form_name: &str,
        element_injectors: Vec<serde_json::Value>,
        asset_injectors: Vec<serde_json::Value>,
        start_time: f32,
        terminate_time: f32,
        simulation_speed: f32,
    ) -> Self {
        let mut score = Self::new(start_time, terminate_time, simulation_speed);

        let config_injector = serde_json::json!({
            "timeOffset": 0.0,
            "minLength": 10.0,
            "active": true
        });

        let mut period = ElementPeriod::new(
            form_name.to_string(),
            "Period".to_string(),
            config_injector,
        );
        period.elements = element_injectors;

        let mut staff = ElementStaff::new(
            "Chart".to_string(),
            true,
            "Chart".to_string(),
            form_name.to_string(),
        );
        staff.periods.push(period);
        score.stave.push(Box::new(staff));

        if !asset_injectors.is_empty() {
            let mut asset_loader = AssetLoader::new("Asset".to_string(), true);
            let mut asset_set = AssetSet::new("AssetSet".to_string());
            for asset in asset_injectors {
                asset_set.assets.push(asset);
            }
            asset_loader.asset_sets.push(asset_set);
            score.asset_loaders.push(asset_loader);
        }

        score
    }
}

/// 生成供 Gorge 代码查询的资源路径。
///
/// 部分 zip 包会以包名再包一层目录，例如
/// `Dremu/Dremu/FormAsset/Tap.png`。源码使用的是逻辑路径
/// `Dremu/FormAsset/Tap`，因此仅折叠相邻且相同的首级目录；其它层级保持不变。
fn normalized_asset_stem(path: &str, extension: &str) -> String {
    let suffix = format!(".{}", extension);
    let stem = path.strip_suffix(&suffix).unwrap_or(path);
    let mut segments: Vec<&str> = stem
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.len() >= 2 && segments[0] == segments[1] {
        segments.remove(0);
    }
    segments.join("/")
}

impl Default for SimulationScore {
    fn default() -> Self {
        Self::new(0.0, 1.0, 1.0)
    }
}

// ==================== 谱表提取辅助函数 ====================

/// 从方法注解中提取 PeriodConfig 注入器的 JSON 表示（完整注入器实例化路径）。
///
/// 真实谱面的 `config` 参数是 `GorgeFramework.PeriodConfig^` 注入器字面量，
/// 编译器常量折叠无法处理注入器对象，会为它生成一个隐藏静态方法
///（`__annotation_<Anno>_config`），注解参数记录为 `AnnotationValue::Delegate(全局方法 ID)`。
/// 本函数沿 Delegate 找到隐藏方法字节码中的 `LoadInjectorConstant`，从类常量池取出
/// 该 `PeriodConfig` 注入器常量并转 JSON（与音频/元素注入器同一条完整实例化路径）。
///
/// 防御回退：`config` 缺失或不是 Delegate 时返回默认配置，并继续收集注解上的
/// 直接标量参数（`timeOffset`/`minLength`/`active`）。
fn extract_period_config_injector(
    cc: &gorge_core::objective::bytecode::CompiledClass,
    ann: &gorge_core::objective::declaration::MethodAnnotation,
    meta: &InjectorFieldMetaProvider,
) -> serde_json::Value {
    let mut config = serde_json::json!({
        "timeOffset": 0.0,
        "minLength": 10.0,
        "active": true
    });

    for (key, value) in &ann.parameters {
        if key == "config" {
            // config 参数为注入器常量引用：Delegate 指向返回该注入器的隐藏方法
            if let gorge_core::objective::declaration::AnnotationValue::Delegate(hidden_id) = value {
                if let Some(cfg_json) = resolve_delegate_injector_json(cc, *hidden_id, meta) {
                    return cfg_json;
                }
            }
            // 解析失败（如注入器非常量）则回退默认配置，继续尝试直接参数
        } else if key == "timeOffset" {
            if let gorge_core::objective::declaration::AnnotationValue::Float(v) = value {
                config["timeOffset"] = serde_json::json!(*v as f32);
            }
        } else if key == "minLength" {
            if let gorge_core::objective::declaration::AnnotationValue::Float(v) = value {
                config["minLength"] = serde_json::json!(*v as f32);
            }
        } else if key == "active" {
            if let gorge_core::objective::declaration::AnnotationValue::Bool(v) = value {
                config["active"] = serde_json::json!(*v);
            }
        }
    }
    config
}

/// 从编译类中查找方法的名称
fn method_name_by_id(cc: &gorge_core::objective::bytecode::CompiledClass, method_id: usize) -> String {
    let local_id = if method_id >= cc.method_start_id {
        method_id - cc.method_start_id
    } else {
        method_id
    };
    cc.methods.get(local_id)
        .map(|m| m.name.clone())
        .unwrap_or_else(|| format!("Method_{}", method_id))
}

// ==================== 注入器常量提取与 JSON 转换 ====================

/// 注入器字段元数据提供者。
///
/// 统一编译类（`CompiledClass.injector_fields`）与 native 类
///（`NativeClass::injector_fields_meta`）的注入器字段声明视图，
/// 供注入器常量转 JSON 时将嵌套字段与声明按位置对齐以恢复字段名。
struct InjectorFieldMetaProvider {
    /// 类名 → 注入器字段（名, 值类型）有序表（含继承合并，父类字段在前）
    fields_by_class: HashMap<String, Vec<(String, ValueType)>>,
}

impl InjectorFieldMetaProvider {
    /// 从编译类列表构建元数据视图（native 类元数据来自 `crate::native_classes()`）
    fn build(compiled_classes: &[CompiledClass]) -> Self {
        let mut fields_by_class: HashMap<String, Vec<(String, ValueType)>> = HashMap::new();

        // native 类元数据（宏生成的字段表）
        for cls in crate::native_classes() {
            let meta: Vec<(String, ValueType)> = cls
                .injector_fields_meta()
                .iter()
                .map(|(name, vt)| (name.to_string(), *vt))
                .collect();
            fields_by_class.insert(cls.full_name().to_string(), meta);
        }

        // 编译类元数据（含继承链合并，父类字段在前）
        let compiled_map: HashMap<String, &CompiledClass> = compiled_classes
            .iter()
            .map(|c| (c.class_type.full_name(), c))
            .collect();
        // 克隆 native 表供继承解析使用，避免与后续插入产生借用冲突
        let native_snapshot = fields_by_class.clone();
        for cc in compiled_classes {
            let name = cc.class_type.full_name();
            let fields = Self::collect_compiled_fields(cc, &compiled_map, &native_snapshot, 0);
            fields_by_class.insert(name, fields);
        }

        Self { fields_by_class }
    }

    /// 递归收集编译类的注入器字段（父类字段在前，本类字段在后）。
    ///
    /// 父类为编译类时递归合并；父类为 native 类时查 native 元数据表。
    /// `depth` 防止继承环导致无限递归。
    fn collect_compiled_fields(
        cc: &CompiledClass,
        compiled_map: &HashMap<String, &CompiledClass>,
        native_map: &HashMap<String, Vec<(String, ValueType)>>,
        depth: usize,
    ) -> Vec<(String, ValueType)> {
        const MAX_INHERIT_DEPTH: usize = 32;
        let mut fields = Vec::new();
        if depth < MAX_INHERIT_DEPTH {
            if let Some(super_name) = &cc.super_class_name {
                if let Some(parent) = compiled_map.get(super_name) {
                    fields = Self::collect_compiled_fields(parent, compiled_map, native_map, depth + 1);
                } else if let Some(native_fields) = native_map.get(super_name) {
                    fields = native_fields.clone();
                }
            }
        }
        fields.extend(
            cc.injector_fields
                .iter()
                .map(|f| (f.name.clone(), f.value_type)),
        );
        fields
    }

    /// 按类名查找注入器字段声明（全名未命中时回退末段短名）
    fn lookup(&self, class_name: &str) -> Option<&Vec<(String, ValueType)>> {
        if let Some(fields) = self.fields_by_class.get(class_name) {
            return Some(fields);
        }
        let simple = class_name.rsplit('.').next().unwrap_or(class_name);
        self.fields_by_class.get(simple)
    }
}

/// 解析注解参数中的隐藏方法引用（`AnnotationValue::Delegate`），取得其返回的
/// 注入器常量的 JSON 表示（完整注入器实例化路径）。
///
/// 注解参数为注入器字面量时，编译器生成返回该注入器的隐藏静态方法
///（S3b，如 `__annotation_Song_config`），参数值为该方法的全局方法 ID。
/// 本函数沿字节码定位隐藏方法体内的 `LoadInjectorConstant` 指令，从类常量池
/// 取出常量并转换；`config` 参数等注入器引用均走此路径。
///
/// 解析失败（方法不存在 / 无注入器常量 / 常量缺失）时返回 None。
fn resolve_delegate_injector_json(
    cc: &CompiledClass,
    global_id: usize,
    meta: &InjectorFieldMetaProvider,
) -> Option<serde_json::Value> {
    let idx = method_injector_constant_index(cc, global_id)?;
    let constant = cc.injector_constants.get(idx)?;
    Some(const_object_to_json(&constant.class_name, &constant.fields, meta))
}

/// 在方法字节码中定位返回值对应的注入器常量索引。
///
/// `@Chart`/`@Song` 方法体为 `return <注入器字面量>;`，编译后由一条
/// `LoadInjectorConstant` 指令加载常量池条目。若方法中有多条（如局部注入器
/// 变量），取最后一条（最接近 return 的赋值）。方法无注入器常量时返回 None。
fn method_injector_constant_index(cc: &CompiledClass, method_global_id: usize) -> Option<usize> {
    let local_id = if method_global_id >= cc.method_start_id {
        method_global_id - cc.method_start_id
    } else {
        method_global_id
    };
    let method = cc.methods.get(local_id)?;
    method.codes.iter().rev().find_map(|code| match code.code.operator {
        IntermediateOperator::LoadInjectorConstant(idx) => Some(idx),
        _ => None,
    })
}

/// 从 `@Chart` 方法返回的注入器数组常量填充元素乐段的元素列表。
///
/// 方法字节码中的 `LoadInjectorConstant` 指向类常量池中该方法返回的常量：
/// - `class_name == "Array"`：注入器数组，每个字段是一个元素注入器对象常量
/// - 其他：单个注入器对象（防御路径，视作单元素数组）
fn fill_element_period_from_method(
    cc: &CompiledClass,
    method_global_id: usize,
    period: &mut ElementPeriod,
    meta: &InjectorFieldMetaProvider,
) {
    let Some(idx) = method_injector_constant_index(cc, method_global_id) else {
        return;
    };
    let Some(constant) = cc.injector_constants.get(idx) else {
        return;
    };
    if constant.class_name == "Array" {
        for element in &constant.fields {
            if let InjectorConstField::InjectObject(class_name, _, fields) = element {
                period.elements.push(const_object_to_json(class_name, fields, meta));
            }
        }
    } else {
        period.elements.push(const_object_to_json(&constant.class_name, &constant.fields, meta));
    }
}

/// 将注入器对象常量递归转换为 JSON。
///
/// 输出格式与 `ChartManager::materialize_injector` 的输入约定一致：
/// `{ "__type": 类名, 字段名: 值, ... }`。
///
/// 嵌套 `InjectObject`/`Array` 常量不含字段名（编译期常量表示中对象字段的
/// 槽位用于保留类名），此处通过 `meta` 中父类的注入器字段声明按位置对齐恢复：
/// 命名标量字段钉住游标，未命名的对象/数组字段依次取下一个 Object 类型声明字段。
/// 该对齐假设源文件中字段按声明顺序给出（制谱器生成的谱面满足此约定）；
/// 声明缺失的类（未注册/未知类）退化为 `__unnamed_N` 键保留数据。
fn const_object_to_json(
    class_name: &str,
    fields: &[InjectorConstField],
    meta: &InjectorFieldMetaProvider,
) -> serde_json::Value {
    // 接口/抽象类型的注入器字面量 `Interface^ : {Concrete : {...}}`：
    // 常量外层类名是接口（如 FunctionCurve），唯一子对象字段名与内层
    // 具体类名一致（如 AxialSymmetricFunctionCurve），此时应直接输出
    // 内层具体类，否则物化端按接口名查不到类、元素整体丢失。
    if fields.len() == 1 {
        if let InjectorConstField::InjectObject(nested_class, nested_field_name, nested_fields) = &fields[0] {
            let nested_simple = nested_class.rsplit('.').next().unwrap_or(nested_class);
            if *nested_field_name == nested_simple {
                return const_object_to_json(nested_class, nested_fields, meta);
            }
        }
    }
    let mut obj = serde_json::Map::new();
    obj.insert("__type".into(), class_name.into());

    let declared = meta.lookup(class_name);
    let mut cursor = 0usize;
    let mut unnamed_count = 0usize;

    for field in fields {
        match field {
            InjectorConstField::Int(name, v) => {
                cursor = advance_past(declared, cursor, name);
                obj.insert(name.clone(), (*v).into());
            }
            InjectorConstField::Float(name, v) => {
                cursor = advance_past(declared, cursor, name);
                obj.insert(name.clone(), json_float(*v));
            }
            InjectorConstField::Bool(name, v) => {
                cursor = advance_past(declared, cursor, name);
                obj.insert(name.clone(), (*v).into());
            }
            InjectorConstField::String(name, v) => {
                cursor = advance_past(declared, cursor, name);
                obj.insert(name.clone(), v.clone().into());
            }
            InjectorConstField::Object(name, id) => {
                cursor = advance_past(declared, cursor, name);
                obj.insert(name.clone(), (*id as i64).into());
            }
            InjectorConstField::InjectObject(nested_class, field_name, nested_fields) => {
                let field_name = if field_name.is_empty() {
                    // 旧常量/顶层对象无字段名：按声明位置对齐回退
                    next_object_field_name(declared, &mut cursor)
                        .unwrap_or_else(|| {
                            unnamed_count += 1;
                            format!("__unnamed_{}", unnamed_count)
                        })
                } else {
                    cursor = advance_past(declared, cursor, field_name);
                    field_name.clone()
                };
                obj.insert(
                    field_name,
                    const_object_to_json(nested_class, nested_fields, meta),
                );
            }
            InjectorConstField::Array(field_name, elements) => {
                let field_name = if field_name.is_empty() {
                    next_object_field_name(declared, &mut cursor)
                        .unwrap_or_else(|| {
                            unnamed_count += 1;
                            format!("__unnamed_{}", unnamed_count)
                        })
                } else {
                    cursor = advance_past(declared, cursor, field_name);
                    field_name.clone()
                };
                let arr: Vec<serde_json::Value> = elements
                    .iter()
                    .map(|e| const_element_to_json(e, meta))
                    .collect();
                obj.insert(field_name, serde_json::Value::Array(arr));
            }
        }
    }
    serde_json::Value::Object(obj)
}

/// 转换数组元素（元素类名在常量中保留，无需恢复字段名）
fn const_element_to_json(
    field: &InjectorConstField,
    meta: &InjectorFieldMetaProvider,
) -> serde_json::Value {
    match field {
        InjectorConstField::InjectObject(class_name, _, fields) => {
            const_object_to_json(class_name, fields, meta)
        }
        InjectorConstField::Int(_, v) => (*v).into(),
        InjectorConstField::Float(_, v) => json_float(*v),
        InjectorConstField::Bool(_, v) => (*v).into(),
        InjectorConstField::String(_, v) => v.clone().into(),
        InjectorConstField::Object(_, id) => (*id as i64).into(),
        InjectorConstField::Array(_, elements) => serde_json::Value::Array(
            elements.iter().map(|e| const_element_to_json(e, meta)).collect(),
        ),
    }
}

/// 将浮点值转换为谱面 JSON 值。
///
/// serde_json 不支持 ±Infinity/NaN，而真实谱面注入器用
/// `(-1.0/0.0)` 硬编码负无穷（C# `InjectorHardcodeGenerator.FloatToString`）。
/// 此处约定非有限浮点编码为字符串 `"Infinity"` / `"-Infinity"` / `"NaN"`，
/// 由 `materialize_injector` 一侧还原为 f64。
fn json_float(value: f64) -> serde_json::Value {
    if value.is_nan() {
        serde_json::Value::String("NaN".to_string())
    } else if value == f64::INFINITY {
        serde_json::Value::String("Infinity".to_string())
    } else if value == f64::NEG_INFINITY {
        serde_json::Value::String("-Infinity".to_string())
    } else {
        serde_json::Value::from(value)
    }
}

/// 游标推进到越过名为 `name` 的声明字段（命名字段钉住位置）。
///
/// 返回推进后的游标；声明缺失或未找到时游标不变。
fn advance_past(
    declared: Option<&Vec<(String, ValueType)>>,
    cursor: usize,
    name: &str,
) -> usize {
    if let Some(fields) = declared {
        for (i, (field_name, _)) in fields.iter().enumerate().skip(cursor) {
            if field_name == name {
                return i + 1;
            }
        }
    }
    cursor
}

/// 取游标后第一个 Object 类型的声明字段名并推进游标。
///
/// 用于为未命名的嵌套对象/数组常量恢复字段名；声明缺失或没有剩余
/// Object 字段时返回 None。
fn next_object_field_name(
    declared: Option<&Vec<(String, ValueType)>>,
    cursor: &mut usize,
) -> Option<String> {
    let fields = declared?;
    for (i, (field_name, vt)) in fields.iter().enumerate().skip(*cursor) {
        if *vt == ValueType::Object {
            *cursor = i + 1;
            return Some(field_name.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::package::AssetFile as PkgAssetFile;

    #[test]
    fn test_simulation_score_new() {
        let score = SimulationScore::new(0.0, 10.0, 2.0);
        assert!((score.start_time - 0.0).abs() < 0.01);
        assert!((score.terminate_time - 10.0).abs() < 0.01);
        assert!((score.simulation_speed - 2.0).abs() < 0.01);
        assert!(score.stave.is_empty());
        assert!(score.loaded_assets.is_empty());
    }

    #[test]
    fn test_extract_assets_from_package() {
        let mut package = Package::new();
        package.asset_files.push(PkgAssetFile::new(
            "test.png".to_string(), vec![1, 2, 3], true,
        ));
        package.asset_files.push(PkgAssetFile::new(
            "song.wav".to_string(), vec![4, 5, 6], true,
        ));

        let mut score = SimulationScore::default();
        score.extract_assets_from_package(&package);
        assert_eq!(score.chart_asset_files.len(), 2);
        assert_eq!(score.chart_asset_files[0].path, "test.png");
        assert_eq!(score.chart_asset_files[1].path, "song.wav");
    }

    #[test]
    fn test_add_file_asset_with_mock_backend() {
        let mut score = SimulationScore::default();
        score.chart_asset_files.push(PkgAssetFile::new(
            "sprite.png".to_string(), vec![1, 2, 3], true,
        ));
        score.chart_asset_files.push(PkgAssetFile::new(
            "bgm.wav".to_string(), vec![4, 5, 6], true,
        ));
        score.chart_asset_files.push(PkgAssetFile::new(
            "unknown.bin".to_string(), vec![7, 8], true,
        ));

        let mut backend = MockAssetBackend;
        score.add_file_asset(&mut backend);

        assert_eq!(score.asset_loaders.len(), 1);
        let loader = &score.asset_loaders[0];
        assert_eq!(loader.class_name, "AutoLoaded");
        let set = &loader.asset_sets[0];
        // png → image asset, wav → audio asset, bin → 忽略
        assert_eq!(set.assets.len(), 2);
    }

    #[test]
    fn test_normalized_asset_stem_collapses_duplicate_package_root() {
        assert_eq!(
            normalized_asset_stem("Dremu/Dremu/FormAsset/Tap.png", "png"),
            "Dremu/FormAsset/Tap"
        );
        assert_eq!(
            normalized_asset_stem("Background1.png", "png"),
            "Background1"
        );
        assert_eq!(
            normalized_asset_stem("Theme/Background1.png", "png"),
            "Theme/Background1"
        );
    }

    #[test]
    fn test_load_assets() {
        let mut score = SimulationScore::default();
        let mut loader = AssetLoader::new("TestLoader".to_string(), true);
        let mut set = AssetSet::new("TestSet".to_string());
        set.assets.push(serde_json::json!({
            "name": "image:sprite",
            "texture": 42,
            "type": "image"
        }));
        set.assets.push(serde_json::json!({
            "name": "audio:bgm",
            "audio": 43,
            "type": "audio"
        }));
        loader.asset_sets.push(set);
        score.asset_loaders.push(loader);

        score.load_assets();

        assert_eq!(score.loaded_assets.len(), 2);
        let img = score.get_asset_by_name("image:sprite").unwrap();
        assert_eq!(img.handle, 42);
        let aud = score.get_asset_by_name("audio:bgm").unwrap();
        assert_eq!(aud.handle, 43);
    }

    #[test]
    fn test_get_asset_by_name_not_found() {
        let score = SimulationScore::default();
        assert!(score.get_asset_by_name("nonexistent").is_none());
    }

    #[test]
    fn test_try_get_staff_and_period() {
        let mut score = SimulationScore::default();

        let mut staff = ElementStaff::new(
            "TestChart".to_string(), true, "测试谱表".to_string(), "TestForm".to_string(),
        );
        let config = serde_json::json!({ "timeOffset": 0.0, "minLength": 10.0, "active": true });
        staff.periods.push(ElementPeriod::new(
            "TestForm".to_string(), "PeriodA".to_string(), config,
        ));
        score.stave.push(Box::new(staff));

        assert!(score.try_get_staff("TestChart").is_some());
        assert!(score.try_get_staff("Unknown").is_none());

        let period = score.try_get_period("TestChart", "PeriodA");
        assert!(period.is_some());
        assert_eq!(period.unwrap().method_name(), "PeriodA");

        assert!(score.try_get_period("TestChart", "PeriodB").is_none());
    }

    #[test]
    fn test_check_staff_name_conflict() {
        let mut score = SimulationScore::default();
        let staff = ElementStaff::new(
            "Existing".to_string(), true, "已存在".to_string(), "F".to_string(),
        );
        score.stave.push(Box::new(staff));

        assert!(score.check_staff_name_conflict("Existing"));
        assert!(!score.check_staff_name_conflict("NewOne"));
    }

    #[test]
    fn test_export_chart_package() {
        let mut score = SimulationScore::default();
        let mut staff = ElementStaff::new(
            "ChartClass".to_string(), true, "谱表".to_string(), "Form".to_string(),
        );
        let config = serde_json::json!({ "timeOffset": 0.0, "minLength": 10.0, "active": true });
        staff.periods.push(ElementPeriod::new(
            "Form".to_string(), "Period1".to_string(), config,
        ));
        score.stave.push(Box::new(staff));

        // 添加非谱面 staff 不应被导出
        let modal_staff = ElementStaff::new(
            "ModalClass".to_string(), false, "模态".to_string(), "F".to_string(),
        );
        score.stave.push(Box::new(modal_staff));

        // 添加资产文件
        score.chart_asset_files.push(PkgAssetFile::new(
            "image.png".to_string(), vec![1, 2], true,
        ));

        let package = score.export_chart_package();

        // 仅 is_chart_class=true 的 staff 应导出
        assert_eq!(package.source_code_files.len(), 1);
        assert_eq!(package.source_code_files[0].path, "ChartClass.g");
        // 仅 is_chart_asset=true 的 asset 应导出
        assert_eq!(package.asset_files.len(), 1);
        assert_eq!(package.asset_files[0].path, "image.png");
    }

    #[test]
    fn test_load_score_from_element_list() {
        let elements = vec![
            serde_json::json!({ "hitTime": 0.5, "position": { "x": 100, "y": 200 } }),
            serde_json::json!({ "hitTime": 1.0, "position": { "x": 200, "y": 300 } }),
        ];
        let assets = vec![
            serde_json::json!({ "name": "image:bg", "type": "image" }),
        ];

        let score = SimulationScore::load_score_from_element_list(
            "NoteForm", elements, assets, 0.0, 5.0, 1.0,
        );

        assert_eq!(score.stave.len(), 1);
        let staff = score.stave[0].as_any().downcast_ref::<ElementStaff>().unwrap();
        assert_eq!(staff.class_name, "Chart");
        assert_eq!(staff.periods.len(), 1);
        assert_eq!(staff.periods[0].elements.len(), 2);
        assert_eq!(score.asset_loaders.len(), 1);
        assert_eq!(score.asset_loaders[0].asset_sets[0].assets.len(), 1);
    }

    #[test]
    fn test_load_instant_audio_clears() {
        let mut score = SimulationScore::default();
        score.instant_audio.insert("hit".to_string(), serde_json::json!({}));
        // 空容器 + 空 VM：表被清空且无条目写入
        let container = crate::runtime::runtime_form_container::RuntimeFormContainer::new_empty();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        score.load_instant_audio(&container, &mut vm);
        assert!(score.instant_audio.is_empty());
    }

    // ==================== P0-6: load_instant_audio 读取 FormContainer ====================

    /// 构造带 `@InstantAudio` 注解的编译类：方法 0 返回注入器常量对象。
    fn make_instant_audio_class() -> CompiledClass {
        use gorge_core::objective::bytecode::{
            CompiledClass, InjectorConstField, InjectorConstantDef,
        };
        use gorge_core::objective::declaration::{AnnotationValue, MethodAnnotation};
        use gorge_core::objective::types::{GorgeType, TypeCount};
        use gorge_core::virtual_machine::ir::{
            Address, CodeWithSpan, CompiledMethod, IntermediateCode, IntermediateOperator, Operand,
        };
        use std::collections::HashMap;

        let load_constant = CodeWithSpan::new(
            IntermediateCode {
                result: Some(Address::new(ValueType::Object, 0)),
                operator: IntermediateOperator::LoadInjectorConstant(0),
                left: Operand::int(0),
                right: None,
            },
            Span::dummy(),
        );
        let return_object = CodeWithSpan::new(
            IntermediateCode {
                result: None,
                operator: IntermediateOperator::ReturnObject,
                left: Operand::addr(Address::new(ValueType::Object, 0)),
                right: None,
            },
            Span::dummy(),
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
    fn test_p0_6_load_instant_audio_invokes_methods_from_container() {
        // 容器扫描出方法表 → VM 调用静态方法 → 注入器对象 ID 入库
        let classes = vec![make_instant_audio_class()];
        let mut container = crate::runtime::runtime_form_container::RuntimeFormContainer::new_empty();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        container.scan_forms_from_compiled(&classes, &mut vm);
        assert_eq!(container.instant_audio_methods.len(), 1);

        // 按 loader 约定以简单名注册编译类到 VM
        use gorge_core::objective::class::RuntimeClass;
        use gorge_core::objective::declaration::ClassDeclaration;
        for cc in &classes {
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

        let mut score = SimulationScore::default();
        score.load_instant_audio(&container, &mut vm);

        assert_eq!(score.instant_audio.len(), 1);
        let entry = score.instant_audio.get("RespondA").expect("RespondA 应存在");
        let obj_id = entry["__object_id"].as_u64().unwrap() as usize;
        assert!(obj_id > 0);
        assert!(vm.injectors.contains_key(&obj_id));
    }

    // ==================== P0-7: VM 执行路径嵌套常量物化 ====================

    /// `@InstantAudio` 方法返回的常量含嵌套注入器 + 数组：经 VM 执行路径
    /// 完整物化（嵌套注入器递归 + 数组 native 载荷）。
    ///
    /// 回归：常量池未注册或嵌套字段只占槽填 0 时，返回对象的 object 字段
    /// 为 0、数组无数据。
    #[test]
    fn test_p0_7_instant_audio_nested_constant_materialized() {
        use gorge_core::objective::bytecode::{
            CompiledClass, InjectorConstField, InjectorConstantDef,
        };
        use gorge_core::objective::class::RuntimeClass;
        use gorge_core::objective::declaration::{AnnotationValue, ClassDeclaration, MethodAnnotation};
        use gorge_core::objective::types::{GorgeType, TypeCount};
        use gorge_core::virtual_machine::ir::{
            Address, CodeWithSpan, CompiledMethod, IntermediateCode, IntermediateOperator, Operand,
        };
        use gorge_core::system::native::injector::Injector;
        use std::collections::HashMap;

        // AudioAsset 类声明：string(0) + object(0) 两个注入器字段
        let asset_decl = ClassDeclaration {
            class_type: GorgeType::class("GorgeFramework.AudioAsset", None),
            injector_field_type_count: TypeCount {
                string_count: 1, object_count: 1, ..TypeCount::zero()
            },
            method_count: 1,
            ..ClassDeclaration::dummy("GorgeFramework.AudioAsset".into())
        };

        // 常量：name + 嵌套注入器对象（数组字段）
        let load_constant = CodeWithSpan::new(
            IntermediateCode {
                result: Some(Address::new(ValueType::Object, 0)),
                operator: IntermediateOperator::LoadInjectorConstant(0),
                left: Operand::int(0),
                right: None,
            },
            Span::dummy(),
        );
        let return_object = CodeWithSpan::new(
            IntermediateCode {
                result: None,
                operator: IntermediateOperator::ReturnObject,
                left: Operand::addr(Address::new(ValueType::Object, 0)),
                right: None,
            },
            Span::dummy(),
        );

        let mut method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        method_annotations.insert(0, vec![
            MethodAnnotation {
                name: "InstantAudio".into(),
                parameters: vec![
                    ("name".into(), AnnotationValue::String("Nested".into())),
                ],
            },
        ]);

        let cc = CompiledClass {
            class_type: GorgeType::class("Dremu.DremuNativeResources", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![
                CompiledMethod {
                    name: "GetNested".into(),
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
            injector_constants: vec![InjectorConstantDef {
                class_name: "GorgeFramework.AudioAsset".into(),
                fields: vec![
                    InjectorConstField::String("name".into(), "audio:Hit".into()),
                    // 嵌套注入器（含数组字段）：AudioAsset 声明的 object 槽位
                    InjectorConstField::InjectObject(
                        "GorgeFramework.Inner".into(),
                        String::new(),
                        vec![InjectorConstField::Array(String::new(), vec![
                            InjectorConstField::Int("".into(), 11),
                            InjectorConstField::Int("".into(), 22),
                        ])],
                    ),
                ],
            }],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![],
            method_annotations,
            constructor_annotations: HashMap::new(),
        };

        let mut container = crate::runtime::runtime_form_container::RuntimeFormContainer::new_empty();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        container.scan_forms_from_compiled(&[cc.clone()], &mut vm);

        // 按 loader 约定注册类（注入器声明用含注入器字段的 AudioAsset）
        let name = cc.class_type.full_name().rsplit('.').next().unwrap().to_string();
        let mut rc = RuntimeClass::new(asset_decl, None);
        for (i, m) in cc.methods.iter().enumerate() {
            rc.register_method(i, m.clone());
        }
        vm.register_runtime_class(&name, std::sync::Arc::new(rc));
        // 常量池注册（P0-7：loader 合并顺序与类注册顺序一致）
        vm.injector_constants = cc.injector_constants.clone();

        let mut score = SimulationScore::default();
        score.load_instant_audio(&container, &mut vm);

        let entry = score.instant_audio.get("Nested").expect("Nested 应存在");
        let obj_id = entry["__object_id"].as_u64().unwrap() as usize;

        // 外层注入器：string 字段 + object 字段均非默认且有效
        let inj = vm.injectors.get(&obj_id).expect("应为注入器");
        assert_eq!(inj.get_injector_string(0), "audio:Hit");
        assert!(!inj.get_injector_string_default_value(0));
        let nested_id = inj.get_injector_object(0);
        assert!(nested_id > 0, "嵌套注入器应被递归物化（不再填 0）");
        assert!(!inj.get_injector_object_default_value(0));

        // 嵌套注入器的数组字段：IntArray 载荷元素完整
        let nested = vm.injectors.get(&nested_id).expect("嵌套应为注入器");
        let wrapper_id = nested.get_injector_object(0);
        assert!(wrapper_id > 0, "数组字段应被物化");
        let wrapper = vm.objects.get(&wrapper_id).expect("应为编译层包装对象");
        use gorge_core::objective::object::GorgeObject;
        assert_eq!(wrapper.get_int_field(0), 2, "包装对象 length 字段应为 2");
        let native_id = wrapper.native_object_id.expect("应链接 native 载荷");
        use gorge_core::system::native::array::IntArray;
        let payload = vm.native_payloads.get(&native_id)
            .and_then(|p| p.downcast_ref::<IntArray>())
            .expect("标量 int 元素数组应为 IntArray");
        assert_eq!(payload.items, vec![11, 22], "数组元素值应完整写入");
    }

    #[test]
    fn test_p0_6_load_instant_audio_skips_unregistered_class() {
        // 方法表指向未注册到 VM 的类：invoke 失败，条目被跳过（不 panic）
        use crate::runtime::runtime_form_container::{RuntimeFormContainer, StaticMethodRef};

        let mut container = RuntimeFormContainer::new_empty();
        container.instant_audio_methods.insert(
            "Missing".into(),
            StaticMethodRef::new("No.Such.Class".into(), 0),
        );

        let mut score = SimulationScore::default();
        let mut vm = gorge_core::virtual_machine::vm::VirtualMachine::new();
        score.load_instant_audio(&container, &mut vm);
        assert!(score.instant_audio.is_empty());
    }

    #[test]
    fn test_c2_extract_stave_from_runtime_with_annotations() {
        use gorge_core::objective::class::RuntimeClass;
        use gorge_core::objective::declaration::{ClassDeclaration, Annotation};

        // 构造带 @AudioStaff 注解的类声明
        let audio_decl = ClassDeclaration {
            annotations: vec![
                Annotation {
                    name: "AudioStaff".into(),
                    generic_type: None,
                    arguments: vec![],
                },
            ],
            ..ClassDeclaration::dummy("GorgeFramework.BgmStaff".into())
        };
        let audio_class = Arc::new(RuntimeClass::new(audio_decl, None));

        // 构造带 @ElementStaff 注解的类声明
        let elem_decl = ClassDeclaration {
            annotations: vec![
                Annotation {
                    name: "ElementStaff".into(),
                    generic_type: None,
                    arguments: vec!["NoteForm".into()],
                },
            ],
            ..ClassDeclaration::dummy("GorgeFramework.ChartStaff".into())
        };
        let elem_class = Arc::new(RuntimeClass::new(elem_decl, None));

        // 构造无注解的普通类（应被忽略）
        let normal_decl = ClassDeclaration::dummy("GorgeFramework.SomeUtil".into());
        let normal_class = Arc::new(RuntimeClass::new(normal_decl, None));

        let mut class_table: HashMap<String, Arc<RuntimeClass>> = HashMap::new();
        class_table.insert("GorgeFramework.BgmStaff".into(), audio_class);
        class_table.insert("GorgeFramework.ChartStaff".into(), elem_class);
        class_table.insert("GorgeFramework.SomeUtil".into(), normal_class);

        let mut score = SimulationScore::default();
        score.extract_stave_from_runtime(&class_table);

        // 应提取 2 个谱表（AudioStaff + ElementStaff），普通类被忽略
        assert_eq!(score.stave.len(), 2, "应提取 2 个谱表");

        // 验证存在 AudioStaff
        let audio_count = score.stave.iter()
            .filter(|s| s.class_name() == "GorgeFramework.BgmStaff"
                && s.as_any().downcast_ref::<AudioStaff>().is_some())
            .count();
        assert_eq!(audio_count, 1, "应存在 1 个 AudioStaff");

        // 验证存在 ElementStaff
        let elem_count = score.stave.iter()
            .filter(|s| s.class_name() == "GorgeFramework.ChartStaff"
                && s.as_any().downcast_ref::<ElementStaff>().is_some())
            .count();
        assert_eq!(elem_count, 1, "应存在 1 个 ElementStaff");

        // 验证 ElementStaff 的 form_name
        let elem_staff = score.stave.iter()
            .find(|s| s.class_name() == "GorgeFramework.ChartStaff")
            .and_then(|s| s.as_any().downcast_ref::<ElementStaff>())
            .unwrap();
        assert_eq!(elem_staff.form_name, "NoteForm");
    }

    #[test]
    fn test_c2_extract_stave_from_runtime_empty_table() {
        use gorge_core::objective::class::RuntimeClass;

        let class_table: HashMap<String, Arc<RuntimeClass>> = HashMap::new();
        let mut score = SimulationScore::default();
        // 应不 panic，stave 保持空
        score.extract_stave_from_runtime(&class_table);
        assert!(score.stave.is_empty());
    }

    // ==================== C-3: extract_staves_from_compiled 测试 ====================

    /// 构造一个带 `@ElementStaff` 注解的编译类
    fn make_compiled_class_with_element_staff() -> gorge_core::objective::bytecode::CompiledClass {
        use gorge_core::objective::bytecode::{CompiledAnnotation, CompiledClass, InjectorConstField, InjectorConstantDef};
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
            class_type: GorgeType::class("GorgeFramework.ChartStaff", None),
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
            injector_constants: vec![
                InjectorConstantDef {
                    class_name: "TapNote".into(),
                    fields: vec![
                        InjectorConstField::Float("hitTime".into(), 0.5),
                        InjectorConstField::Float("position".into(), 100.0),
                    ],
                },
            ],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![
                CompiledAnnotation {
                    name: "ElementStaff".into(),
                    generic_type: None,
                    arguments: vec![("form".into(), "NoteForm".into())],
                },
            ],
            method_annotations,
            constructor_annotations: HashMap::new(),
        }
    }

    /// 非有限浮点（±Infinity/NaN）必须经字符串约定往返：
    /// `const_object_to_json` 输出 `"-Infinity"`，不能 panic（serde_json
    /// 不支持非有限数），且真实谱面 `(-1.0/0.0)` 折叠出的负无穷不丢失。
    #[test]
    fn test_const_object_to_json_encodes_non_finite_float_as_string() {
        use gorge_core::objective::bytecode::InjectorConstField;

        let meta = InjectorFieldMetaProvider::build(&[]);
        let json = const_object_to_json(
            "GorgeFramework.FunctionPiece",
            &[
                InjectorConstField::Float("startX".into(), f64::NEG_INFINITY),
                InjectorConstField::Float("endX".into(), f64::INFINITY),
                InjectorConstField::Float("weight".into(), f64::NAN),
            ],
            &meta,
        );
        let obj = json.as_object().expect("应输出 JSON 对象");
        assert_eq!(obj.get("startX").and_then(|v| v.as_str()), Some("-Infinity"));
        assert_eq!(obj.get("endX").and_then(|v| v.as_str()), Some("Infinity"));
        assert_eq!(obj.get("weight").and_then(|v| v.as_str()), Some("NaN"));

        // 有限浮点保持 JSON 数字
        let finite = const_object_to_json(
            "GorgeFramework.Sample",
            &[InjectorConstField::Float("value".into(), 1.25)],
            &meta,
        );
        assert_eq!(
            finite.get("value").and_then(|v| v.as_f64()),
            Some(1.25),
        );
    }

    /// 构造一个带 `@AudioStaff` 注解的编译类
    fn make_compiled_class_with_audio_staff() -> gorge_core::objective::bytecode::CompiledClass {
        use gorge_core::objective::bytecode::{CompiledAnnotation, CompiledClass};
        use gorge_core::objective::declaration::{MethodAnnotation, AnnotationValue};
        use gorge_core::objective::types::{GorgeType, TypeCount};
        use gorge_core::virtual_machine::ir::CompiledMethod;
        use std::collections::HashMap;

        let mut method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        method_annotations.insert(0, vec![
            MethodAnnotation {
                name: "Song".into(),
                parameters: vec![
                    ("timeOffset".into(), AnnotationValue::Float(0.0)),
                ],
            },
        ]);

        CompiledClass {
            class_type: GorgeType::class("GorgeFramework.BgmStaff", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![
                CompiledMethod { name: "Bgm".into(), codes: vec![], local_count: 0 },
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
                    name: "AudioStaff".into(),
                    generic_type: None,
                    arguments: vec![("displayName".into(), "背景音乐".into())],
                },
            ],
            method_annotations,
            constructor_annotations: HashMap::new(),
        }
    }

    #[test]
    fn test_c3_extract_staves_from_compiled_element_staff() {
        let classes = vec![make_compiled_class_with_element_staff()];
        let mut score = SimulationScore::default();
        score.extract_staves_from_compiled(&classes);

        assert_eq!(score.stave.len(), 1, "应提取 1 个谱表");

        let staff = score.stave[0].as_any().downcast_ref::<ElementStaff>().unwrap();
        assert_eq!(staff.class_name, "GorgeFramework.ChartStaff");
        assert_eq!(staff.form_name, "NoteForm");
        // 应有 1 个乐段（Period1）
        assert_eq!(staff.periods.len(), 1);
        assert_eq!(staff.periods[0].period_data.method_name, "Period1");
        // 配置应从注解参数提取
        assert!((staff.periods[0].period_data.config.time_offset - 1.5).abs() < 0.001);
        assert!((staff.periods[0].period_data.config.min_length - 20.0).abs() < 0.001);
        assert!(staff.periods[0].period_data.config.active);
    }

    #[test]
    fn test_c3_extract_staves_from_compiled_audio_staff() {
        let classes = vec![make_compiled_class_with_audio_staff()];
        let mut score = SimulationScore::default();
        score.extract_staves_from_compiled(&classes);

        assert_eq!(score.stave.len(), 1, "应提取 1 个谱表");

        let staff = score.stave[0].as_any().downcast_ref::<AudioStaff>().unwrap();
        assert_eq!(staff.class_name, "GorgeFramework.BgmStaff");
        assert_eq!(staff.display_name, "背景音乐");
        assert_eq!(staff.periods.len(), 1);
        assert_eq!(staff.periods[0].period_data.method_name, "Bgm");
    }

    #[test]
    fn test_c3_extract_staves_from_compiled_empty() {
        let classes: Vec<gorge_core::objective::bytecode::CompiledClass> = vec![];
        let mut score = SimulationScore::default();
        // 应不 panic，stave 保持空
        score.extract_staves_from_compiled(&classes);
        assert!(score.stave.is_empty());
    }

    #[test]
    fn test_c3_extract_staves_from_compiled_no_staff_class() {
        // 不含谱表注解的普通类应被忽略
        use gorge_core::objective::bytecode::{CompiledClass};
        use gorge_core::objective::types::{GorgeType, TypeCount};
        use std::collections::HashMap;

        let normal = CompiledClass {
            class_type: GorgeType::class("SomeUtil", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![],
            constructors: vec![],
            injector_fields: vec![],
            delegate_impls: vec![],
            method_start_id: 0,
            method_count_total: 0,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],
            injector_constants: vec![],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![],
            method_annotations: HashMap::new(),
            constructor_annotations: HashMap::new(),
        };

        let classes = vec![normal];
        let mut score = SimulationScore::default();
        score.extract_staves_from_compiled(&classes);
        assert!(score.stave.is_empty());
    }

    #[test]
    fn test_c3_extract_staves_from_compiled_multiple_staves() {
        let classes = vec![
            make_compiled_class_with_element_staff(),
            make_compiled_class_with_audio_staff(),
        ];
        let mut score = SimulationScore::default();
        score.extract_staves_from_compiled(&classes);

        assert_eq!(score.stave.len(), 2, "应提取 2 个谱表");

        let has_element = score.stave.iter()
            .any(|s| s.as_any().downcast_ref::<ElementStaff>().is_some());
        let has_audio = score.stave.iter()
            .any(|s| s.as_any().downcast_ref::<AudioStaff>().is_some());
        assert!(has_element, "应包含 ElementStaff");
        assert!(has_audio, "应包含 AudioStaff");
    }

    // ==================== P0-2: 按方法提取注入器常量测试 ====================

    use gorge_core::diagnostics::Span;
    use gorge_core::objective::bytecode::{CompiledClass, InjectorConstField, InjectorConstantDef};
    use gorge_core::objective::declaration::MethodAnnotation;
    use gorge_core::objective::types::{GorgeType, TypeCount};
    use gorge_core::virtual_machine::ir::{
        CodeWithSpan, CompiledMethod, IntermediateCode, IntermediateOperator, Operand,
    };

    /// 构造 `LoadInjectorConstant(idx)` 指令
    fn load_injector_constant_code(idx: usize) -> CodeWithSpan {
        CodeWithSpan::new(
            IntermediateCode {
                result: None,
                operator: IntermediateOperator::LoadInjectorConstant(idx),
                left: Operand::int(0),
                right: None,
            },
            Span::dummy(),
        )
    }

    /// 构造带 @Chart 方法（返回注入器数组常量）的元素谱表编译类
    fn make_chart_staff_with_method_constant() -> CompiledClass {
        let mut method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        method_annotations.insert(0, vec![
            MethodAnnotation { name: "Chart".into(), parameters: vec![] },
        ]);

        CompiledClass {
            class_type: GorgeType::class("Test.ChartStaff", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![
                CompiledMethod {
                    name: "Period".into(),
                    codes: vec![load_injector_constant_code(0)],
                    local_count: 0,
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
                // @Chart 方法返回的注入器数组常量
                InjectorConstantDef {
                    class_name: "Array".into(),
                    fields: vec![
                        InjectorConstField::InjectObject(
                            "Test.TapNote".into(),
                            String::new(),
                            vec![
                                InjectorConstField::Float("hitTime".into(), 0.5),
                                InjectorConstField::Float("keepTime".into(), 1.0),
                            ],
                        ),
                        InjectorConstField::InjectObject(
                            "Test.TapNote".into(),
                            String::new(),
                            vec![
                                InjectorConstField::Float("hitTime".into(), 1.5),
                                InjectorConstField::Float("keepTime".into(), 2.0),
                            ],
                        ),
                    ],
                },
                // 干扰项：其他方法的常量不应被提取到本乐段
                InjectorConstantDef {
                    class_name: "Test.Unrelated".into(),
                    fields: vec![InjectorConstField::Int("x".into(), 99)],
                },
            ],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![
                gorge_core::objective::bytecode::CompiledAnnotation {
                    name: "ElementStaff".into(),
                    generic_type: None,
                    arguments: vec![("form".into(), "NoteForm".into())],
                },
            ],
            method_annotations,
            constructor_annotations: HashMap::new(),
        }
    }

    #[test]
    fn test_p02_chart_method_elements_extracted_from_method_constant() {
        let classes = vec![make_chart_staff_with_method_constant()];
        let mut score = SimulationScore::default();
        score.extract_staves_from_compiled(&classes);

        let staff = score.stave[0].as_any().downcast_ref::<ElementStaff>().unwrap();
        assert_eq!(staff.periods.len(), 1);
        let period = &staff.periods[0];

        // 应提取 2 个元素（而非旧实现的全类常量混填）
        assert_eq!(period.elements.len(), 2, "应按方法常量提取 2 个元素");
        assert_eq!(period.elements[0]["__type"], "Test.TapNote");
        assert!((period.elements[0]["hitTime"].as_f64().unwrap() - 0.5).abs() < 1e-9);
        assert!((period.elements[0]["keepTime"].as_f64().unwrap() - 1.0).abs() < 1e-9);
        assert!((period.elements[1]["hitTime"].as_f64().unwrap() - 1.5).abs() < 1e-9);
    }

    #[test]
    fn test_p02_song_method_audio_injector_extracted() {
        let mut method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        method_annotations.insert(0, vec![
            MethodAnnotation { name: "Song".into(), parameters: vec![] },
        ]);

        let cc = CompiledClass {
            class_type: GorgeType::class("Test.SongStaff", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![
                CompiledMethod {
                    name: "GetSong".into(),
                    codes: vec![load_injector_constant_code(0)],
                    local_count: 0,
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
                    fields: vec![InjectorConstField::String("name".into(), "audio:Song".into())],
                },
            ],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![
                gorge_core::objective::bytecode::CompiledAnnotation {
                    name: "AudioStaff".into(),
                    generic_type: None,
                    arguments: vec![],
                },
            ],
            method_annotations,
            constructor_annotations: HashMap::new(),
        };

        let classes = vec![cc];
        let mut score = SimulationScore::default();
        score.extract_staves_from_compiled(&classes);

        let staff = score.stave[0].as_any().downcast_ref::<AudioStaff>().unwrap();
        assert_eq!(staff.periods.len(), 1);
        let audio = staff.periods[0].audio_injector.as_ref().expect("应提取音频注入器");
        assert_eq!(audio["__type"], "GorgeFramework.AudioAsset");
        assert_eq!(audio["name"], "audio:Song");
    }

    #[test]
    fn test_p02_nested_inject_object_field_name_recovered_by_position() {
        // 父类声明：name(String) + drawStartX(Object) + drawEndX(Object)
        let lane_cc = CompiledClass {
            class_type: GorgeType::class("Test.Lane", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![],
            constructors: vec![],
            injector_fields: vec![
                gorge_core::objective::bytecode::InjectorFieldDef {
                    name: "name".into(),
                    value_type: ValueType::String,
                    is_array: false,
                    has_default: false,
                    default_value: None,
                },
                gorge_core::objective::bytecode::InjectorFieldDef {
                    name: "drawStartX".into(),
                    value_type: ValueType::Object,
                    is_array: false,
                    has_default: false,
                    default_value: None,
                },
                gorge_core::objective::bytecode::InjectorFieldDef {
                    name: "drawEndX".into(),
                    value_type: ValueType::Object,
                    is_array: false,
                    has_default: false,
                    default_value: None,
                },
            ],
            delegate_impls: vec![],
            method_start_id: 0,
            method_count_total: 0,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],
            injector_constants: vec![],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![],
            method_annotations: HashMap::new(),
            constructor_annotations: HashMap::new(),
        };

        let mut method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        method_annotations.insert(0, vec![
            MethodAnnotation { name: "Chart".into(), parameters: vec![] },
        ]);
        let staff_cc = CompiledClass {
            class_type: GorgeType::class("Test.LaneStaff", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![
                CompiledMethod {
                    name: "Period".into(),
                    codes: vec![load_injector_constant_code(0)],
                    local_count: 0,
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
                    class_name: "Array".into(),
                    fields: vec![
                        InjectorConstField::InjectObject(
                            "Test.Lane".into(),
                            String::new(),
                            vec![
                                InjectorConstField::String("name".into(), "L1".into()),
                                InjectorConstField::InjectObject(
                                    "GorgeFramework.VariableFloat".into(),
                                    String::new(),
                                    vec![InjectorConstField::Float("baseValue".into(), 1.0)],
                                ),
                                InjectorConstField::InjectObject(
                                    "GorgeFramework.VariableFloat".into(),
                                    String::new(),
                                    vec![InjectorConstField::Float("baseValue".into(), 2.0)],
                                ),
                            ],
                        ),
                    ],
                },
            ],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![
                gorge_core::objective::bytecode::CompiledAnnotation {
                    name: "ElementStaff".into(),
                    generic_type: None,
                    arguments: vec![("form".into(), "F".into())],
                },
            ],
            method_annotations,
            constructor_annotations: HashMap::new(),
        };

        let classes = vec![lane_cc, staff_cc];
        let mut score = SimulationScore::default();
        score.extract_staves_from_compiled(&classes);

        let staff = score.stave[0].as_any().downcast_ref::<ElementStaff>().unwrap();
        let lane = &staff.periods[0].elements[0];
        assert_eq!(lane["__type"], "Test.Lane");
        assert_eq!(lane["name"], "L1");
        // 嵌套注入器按声明位置恢复字段名：drawStartX / drawEndX
        assert_eq!(lane["drawStartX"]["__type"], "GorgeFramework.VariableFloat");
        assert!((lane["drawStartX"]["baseValue"].as_f64().unwrap() - 1.0).abs() < 1e-9);
        assert_eq!(lane["drawEndX"]["__type"], "GorgeFramework.VariableFloat");
        assert!((lane["drawEndX"]["baseValue"].as_f64().unwrap() - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_p02_native_injector_fields_meta_available() {
        // 宏生成的 native 注入器字段元数据应可用于声明查询
        let provider = InjectorFieldMetaProvider::build(&[]);
        // P0-8：注入器字段名对齐谱面存根的 Gorge 名（baseValue/variationCurve）
        let variable_float = provider
            .lookup("GorgeFramework.VariableFloat")
            .expect("VariableFloat 应有元数据");
        let vf_names: Vec<&str> = variable_float.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(vf_names, ["baseValue", "variationCurve"]);
        assert!(variable_float.iter().any(|(name, vt)|
            name == "baseValue" && *vt == ValueType::Float));
        assert!(variable_float.iter().any(|(name, vt)|
            name == "variationCurve" && *vt == ValueType::Object));
        let vector2 = provider
            .lookup("GorgeFramework.Vector2")
            .expect("Vector2 应有元数据");
        assert!(vector2.iter().any(|(name, _)| name == "x"));
        assert!(vector2.iter().any(|(name, _)| name == "y"));
    }

    #[test]
    fn test_p08_curve_family_injector_fields_meta() {
        // P0-8：曲线/向量族 native 类的注入器字段元数据对齐谱面存根
        let provider = InjectorFieldMetaProvider::build(&[]);

        // CubicHermiteSpline：6 个注入器字段，声明序与 Gorge 名对齐存根
        let spline = provider
            .lookup("GorgeFramework.CubicHermiteSpline")
            .expect("CubicHermiteSpline 应有元数据");
        let spline_names: Vec<&str> = spline.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(
            spline_names,
            ["startPoint", "startTangent", "startWeight", "endPoint", "endTangent", "endWeight"]
        );
        assert_eq!(spline[0].1, ValueType::Object);
        assert_eq!(spline[3].1, ValueType::Object);
        assert_eq!(spline[1].1, ValueType::Float);

        // LerpColorCurve：两个对象注入器字段
        let lerp = provider
            .lookup("GorgeFramework.LerpColorCurve")
            .expect("LerpColorCurve 应有元数据");
        let lerp_names: Vec<&str> = lerp.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(lerp_names, ["colorPoints", "progressCurve"]);
        assert!(lerp.iter().all(|(_, vt)| *vt == ValueType::Object));

        // PeriodicFunctionCurve：leftClosed 默认值 true（直接查宏生成的默认值方法）
        let periodic = provider
            .lookup("GorgeFramework.PeriodicFunctionCurve")
            .expect("PeriodicFunctionCurve 应有元数据");
        let periodic_names: Vec<&str> = periodic.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(periodic_names, ["functionCurve", "startX", "endX", "leftClosed"]);
        assert!(
            crate::system::native::function_curve_combinators::PeriodicFunctionCurve::gorge_injector_default_left_closed(),
            "PeriodicFunctionCurve.leftClosed 默认值应为 true"
        );
    }

    #[test]
    fn test_p02_method_without_injector_constant_yields_empty_elements() {
        // 方法字节码中无 LoadInjectorConstant（如纯计算动态生成）时，元素列表为空
        let mut method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        method_annotations.insert(0, vec![
            MethodAnnotation { name: "Chart".into(), parameters: vec![] },
        ]);
        let cc = CompiledClass {
            class_type: GorgeType::class("Test.DynamicStaff", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![
                CompiledMethod { name: "Period".into(), codes: vec![], local_count: 0 },
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
                gorge_core::objective::bytecode::CompiledAnnotation {
                    name: "ElementStaff".into(),
                    generic_type: None,
                    arguments: vec![],
                },
            ],
            method_annotations,
            constructor_annotations: HashMap::new(),
        };

        let classes = vec![cc];
        let mut score = SimulationScore::default();
        score.extract_staves_from_compiled(&classes);

        let staff = score.stave[0].as_any().downcast_ref::<ElementStaff>().unwrap();
        assert_eq!(staff.periods.len(), 1);
        assert!(staff.periods[0].elements.is_empty(),
            "无注入器常量的方法应得到空元素列表");
    }

    // ==================== P0-3: config 注入器完整实例化测试 ====================

    /// 真实谱面的 `@Song` 注解形态：`config` 参数是 `PeriodConfig^` 注入器字面量，
    /// 编译器将其生成为隐藏方法（`__annotation_Song_config`，全局 ID = method_start_id + 1），
    /// 注解参数记录为 `Delegate(hidden_id)`。音频乐段的 config 必须沿该路径完整实例化，
    /// 而不是回退默认配置或近似推导。
    #[test]
    fn test_p03_song_config_resolved_from_hidden_method_delegate() {
        use gorge_core::objective::declaration::AnnotationValue;

        let mut method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        method_annotations.insert(0, vec![
            // @Song 的 config 参数指向隐藏方法 __annotation_Song_config（全局 ID 1）
            MethodAnnotation {
                name: "Song".into(),
                parameters: vec![("config".into(), AnnotationValue::Delegate(1))],
            },
        ]);

        let cc = CompiledClass {
            class_type: GorgeType::class("Test.SongStaff", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            // methods[0]=GetSong；methods[1]=隐藏方法 __annotation_Song_config（S3b 插入）
            methods: vec![
                CompiledMethod {
                    name: "GetSong".into(),
                    codes: vec![load_injector_constant_code(0)],
                    local_count: 0,
                },
                CompiledMethod {
                    name: "__annotation_Song_config".into(),
                    codes: vec![load_injector_constant_code(1)],
                    local_count: 0,
                },
            ],
            constructors: vec![],
            injector_fields: vec![],
            delegate_impls: vec![],
            method_start_id: 0,
            method_count_total: 2,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],
            injector_constants: vec![
                // constants[0]：@Song 方法返回的音频资产注入器
                InjectorConstantDef {
                    class_name: "GorgeFramework.AudioAsset".into(),
                    fields: vec![InjectorConstField::String("name".into(), "audio:Song".into())],
                },
                // constants[1]：隐藏方法返回的 PeriodConfig 注入器
                InjectorConstantDef {
                    class_name: "GorgeFramework.PeriodConfig".into(),
                    fields: vec![
                        InjectorConstField::Float("timeOffset".into(), 0.373),
                        InjectorConstField::Float("minLength".into(), 30.0),
                        InjectorConstField::Bool("active".into(), false),
                    ],
                },
            ],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![
                gorge_core::objective::bytecode::CompiledAnnotation {
                    name: "AudioStaff".into(),
                    generic_type: None,
                    arguments: vec![],
                },
            ],
            method_annotations,
            constructor_annotations: HashMap::new(),
        };

        let classes = vec![cc];
        let mut score = SimulationScore::default();
        score.extract_staves_from_compiled(&classes);

        let staff = score.stave[0].as_any().downcast_ref::<AudioStaff>().unwrap();
        assert_eq!(staff.periods.len(), 1);
        let period = &staff.periods[0];

        // config 应来自隐藏方法返回的 PeriodConfig 注入器常量（完整实例化路径）
        assert!((period.period_data.config.time_offset - 0.373).abs() < 1e-6,
            "config 应从注入器常量解析 timeOffset，实际 {:?}", period.period_data.config);
        assert!((period.period_data.config.min_length - 30.0).abs() < 1e-6);
        assert!(!period.period_data.config.active);
        assert_eq!(period.period_data.config_injector["__type"], "GorgeFramework.PeriodConfig");
        // config_injector 保留完整注入器 JSON（含嵌套结构可扩展性）
        assert_eq!(period.period_data.config_injector["timeOffset"], 0.373);

        // 音频注入器不受影响
        let audio = period.audio_injector.as_ref().expect("应提取音频注入器");
        assert_eq!(audio["__type"], "GorgeFramework.AudioAsset");
        assert_eq!(audio["name"], "audio:Song");
    }

    /// 防御回退：`config` 参数不是 Delegate（无法解析）时，回退默认配置并保留
    /// 注解上的直接标量参数（旧行为）。
    #[test]
    fn test_p03_config_delegate_unresolvable_falls_back_to_direct_params() {
        use gorge_core::objective::declaration::AnnotationValue;

        let mut method_annotations: HashMap<usize, Vec<MethodAnnotation>> = HashMap::new();
        method_annotations.insert(0, vec![
            MethodAnnotation {
                name: "Song".into(),
                parameters: vec![
                    // 指向不存在的隐藏方法（方法表仅 1 个，ID 99 越界）
                    ("config".into(), AnnotationValue::Delegate(99)),
                    // 直接标量参数（旧路径）
                    ("timeOffset".into(), AnnotationValue::Float(2.0)),
                ],
            },
        ]);

        let cc = CompiledClass {
            class_type: GorgeType::class("Test.SongStaff", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods: vec![
                CompiledMethod {
                    name: "GetSong".into(),
                    codes: vec![load_injector_constant_code(0)],
                    local_count: 0,
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
                    fields: vec![InjectorConstField::String("name".into(), "audio:Song".into())],
                },
            ],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![
                gorge_core::objective::bytecode::CompiledAnnotation {
                    name: "AudioStaff".into(),
                    generic_type: None,
                    arguments: vec![],
                },
            ],
            method_annotations,
            constructor_annotations: HashMap::new(),
        };

        let classes = vec![cc];
        let mut score = SimulationScore::default();
        score.extract_staves_from_compiled(&classes);

        let staff = score.stave[0].as_any().downcast_ref::<AudioStaff>().unwrap();
        let period = &staff.periods[0];
        // 回退路径：直接标量参数生效，其余为默认值
        assert!((period.period_data.config.time_offset - 2.0).abs() < 1e-6);
        assert!((period.period_data.config.min_length - 10.0).abs() < 1e-6);
        assert!(period.period_data.config.active);
        // 音频注入器仍应提取
        assert!(period.audio_injector.is_some());
    }
}
