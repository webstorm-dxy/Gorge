//! 仿真总谱（对应 C# `Chart/SimulationScore.cs`）。
//!
//! 包含谱面所有元素和资源信息，向上与运行时环境对接，向下与谱面文件互转。

use std::collections::HashMap;
use std::sync::Arc;
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

    /// 加载响应音效（对应 C# `LoadInstantAudio`）。
    ///
    /// 骨架实现：需要 `RuntimeStatic.Runtime.FormContainer.InstantAudioMethods`
    /// 运行时数据，当前仅清空表。F 步接入真实运行时后补齐。
    pub fn load_instant_audio(&mut self) {
        self.instant_audio.clear();
        // 骨架：实际需要从 RuntimeStatic 读取 InstantAudioMethods
        // F 步需接入 GorgeLanguageRuntime 的 FormContainer
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
    /// 4. 从 `injector_constants` 提取元素/音频注入器数据
    pub fn extract_staves_from_compiled(
        &mut self,
        compiled_classes: &[gorge_core::objective::bytecode::CompiledClass],
    ) {
        self.stave.clear();

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
                        self.extract_audio_periods_from_class(cc, &mut staff);

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
                        self.extract_element_periods_from_class(cc, &mut staff, &form_name);

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
        &mut self,
        cc: &gorge_core::objective::bytecode::CompiledClass,
        staff: &mut AudioStaff,
    ) {
        for (method_id, annotations) in &cc.method_annotations {
            for ann in annotations {
                if ann.name != "Song" {
                    continue;
                }
                // 从方法注解的 config 参数提取 PeriodConfig 注入器
                let config_injector = extract_period_config_injector(ann);
                // 尝试从 injector_constants 查找对应方法的音频常量
                let audio_injector = cc.injector_constants.iter()
                    .find(|c| c.class_name == "AudioAsset" || c.class_name.contains("Audio"))
                    .map(|c| injector_const_to_json(c));

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
        &mut self,
        cc: &gorge_core::objective::bytecode::CompiledClass,
        staff: &mut ElementStaff,
        form_name: &str,
    ) {
        for (method_id, annotations) in &cc.method_annotations {
            for ann in annotations {
                if ann.name != "Chart" {
                    continue;
                }
                let config_injector = extract_period_config_injector(ann);
                let mut period = ElementPeriod::new(
                    form_name.to_string(),
                    method_name_by_id(cc, *method_id).to_string(),
                    config_injector,
                );
                // 从 injector_constants 提取元素数据
                // TODO: 完整实现需要注入器实例化系统，当前从常量定义推导
                self.fill_element_period_from_constants(cc, &mut period);

                staff.periods.push(period);
            }
        }
    }

    /// 从注入器常量中填充元素乐段的元素列表（骨架实现）
    fn fill_element_period_from_constants(
        &mut self,
        cc: &gorge_core::objective::bytecode::CompiledClass,
        period: &mut ElementPeriod,
    ) {
        // 遍历注入器常量，将数组或注入器对象转换为 JSON 元素
        for constant in &cc.injector_constants {
            for field in &constant.fields {
                match field {
                    gorge_core::objective::bytecode::InjectorConstField::Array(elements) => {
                        for elem in elements {
                            if let Some(json) = injector_const_field_to_json(elem) {
                                period.elements.push(json);
                            }
                        }
                    }
                    // 单个注入器对象也作为一个元素
                    _ => {
                        if let Some(json) = injector_const_field_to_json(field) {
                            period.elements.push(json);
                        }
                    }
                }
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

/// 从方法注解中提取 PeriodConfig 注入器的 JSON 表示
///
/// 查找 `config` 参数，若不存在则返回默认配置。
fn extract_period_config_injector(ann: &gorge_core::objective::declaration::MethodAnnotation) -> serde_json::Value {
    let mut config = serde_json::json!({
        "timeOffset": 0.0,
        "minLength": 10.0,
        "active": true
    });

    for (key, value) in &ann.parameters {
        if key == "config" {
            // cfg 参数为注入器常量引用（InjectObject），此处简化处理
            // TODO: 完整实现需通过注入器实例化系统解析
            match value {
                gorge_core::objective::declaration::AnnotationValue::Float(f) => {
                    config["timeOffset"] = serde_json::json!(*f as f32);
                }
                _ => {}
            }
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

/// 将注入器常量定义转换为 JSON 值（递归处理嵌套对象）
fn injector_const_to_json(
    constant: &gorge_core::objective::bytecode::InjectorConstantDef,
) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    obj.insert("__type".into(), serde_json::Value::String(constant.class_name.clone()));
    for field in &constant.fields {
        let (name, value) = injector_const_field_entry(field);
        obj.insert(name, value);
    }
    serde_json::Value::Object(obj)
}

/// 将注入器常量字段转换为 JSON 值
fn injector_const_field_to_json(
    field: &gorge_core::objective::bytecode::InjectorConstField,
) -> Option<serde_json::Value> {
    match field {
        gorge_core::objective::bytecode::InjectorConstField::Int(name, v) => {
            let mut obj = serde_json::Map::new();
            if !name.is_empty() { obj.insert("__field".into(), name.clone().into()); }
            obj.insert("__type".into(), "int".into());
            obj.insert("value".into(), (*v).into());
            Some(serde_json::Value::Object(obj))
        }
        gorge_core::objective::bytecode::InjectorConstField::Float(name, v) => {
            let mut obj = serde_json::Map::new();
            if !name.is_empty() { obj.insert("__field".into(), name.clone().into()); }
            obj.insert("__type".into(), "float".into());
            obj.insert("value".into(), (*v).into());
            Some(serde_json::Value::Object(obj))
        }
        gorge_core::objective::bytecode::InjectorConstField::Bool(name, v) => {
            let mut obj = serde_json::Map::new();
            if !name.is_empty() { obj.insert("__field".into(), name.clone().into()); }
            obj.insert("__type".into(), "bool".into());
            obj.insert("value".into(), (*v).into());
            Some(serde_json::Value::Object(obj))
        }
        gorge_core::objective::bytecode::InjectorConstField::String(name, v) => {
            let mut obj = serde_json::Map::new();
            if !name.is_empty() { obj.insert("__field".into(), name.clone().into()); }
            obj.insert("__type".into(), "string".into());
            obj.insert("value".into(), v.clone().into());
            Some(serde_json::Value::Object(obj))
        }
        gorge_core::objective::bytecode::InjectorConstField::InjectObject(class_name, fields) => {
            let mut obj = serde_json::Map::new();
            obj.insert("__type".into(), class_name.clone().into());
            for f in fields {
                let (n, v) = injector_const_field_entry(f);
                obj.insert(n, v);
            }
            Some(serde_json::Value::Object(obj))
        }
        gorge_core::objective::bytecode::InjectorConstField::Array(elements) => {
            let arr: Vec<serde_json::Value> = elements.iter()
                .filter_map(injector_const_field_to_json)
                .collect();
            Some(serde_json::Value::Array(arr))
        }
        gorge_core::objective::bytecode::InjectorConstField::Object(name, id) => {
            let mut obj = serde_json::Map::new();
            obj.insert("__type".into(), "object".into());
            obj.insert("__field".into(), name.clone().into());
            obj.insert("id".into(), (*id as i64).into());
            Some(serde_json::Value::Object(obj))
        }
    }
}

/// 提取注入器常量字段的 (字段名, JSON值) 对
fn injector_const_field_entry(
    field: &gorge_core::objective::bytecode::InjectorConstField,
) -> (String, serde_json::Value) {
    match field {
        gorge_core::objective::bytecode::InjectorConstField::Int(name, v) => {
            (name.clone(), (*v).into())
        }
        gorge_core::objective::bytecode::InjectorConstField::Float(name, v) => {
            (name.clone(), (*v).into())
        }
        gorge_core::objective::bytecode::InjectorConstField::Bool(name, v) => {
            (name.clone(), (*v).into())
        }
        gorge_core::objective::bytecode::InjectorConstField::String(name, v) => {
            (name.clone(), v.clone().into())
        }
        gorge_core::objective::bytecode::InjectorConstField::Object(name, id) => {
            (name.clone(), (*id as i64).into())
        }
        gorge_core::objective::bytecode::InjectorConstField::InjectObject(class_name, fields) => {
            let mut obj = serde_json::Map::new();
            obj.insert("__type".into(), class_name.clone().into());
            for f in fields {
                let (n, v) = injector_const_field_entry(f);
                obj.insert(n, v);
            }
            (String::new(), serde_json::Value::Object(obj))
        }
        gorge_core::objective::bytecode::InjectorConstField::Array(elements) => {
            let arr: Vec<serde_json::Value> = elements.iter()
                .map(|e| injector_const_field_to_json(e).unwrap_or(serde_json::Value::Null))
                .collect();
            (String::new(), serde_json::Value::Array(arr))
        }
    }
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
        score.load_instant_audio();
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
}
