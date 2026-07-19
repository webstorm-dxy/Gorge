//! 乐段模型（对应 C# `Chart/Period.cs`、`ElementPeriod.cs`、`AudioPeriod.cs`）。
//!
//! 谱面数据链中的乐段层，定义乐段的基础抽象与具体类型。
//! 乐段对应 Gorge 类中的一个静态方法（带 `@Chart` 或 `@Song` 注解）。

use serde::{Deserialize, Serialize};

/// 乐段设置（对齐 C# `PeriodConfig` native 类的字段）。
///
/// 与 `system::native::period_config::PeriodConfig`（VM native 类注册版）保持字段一致：
/// - `time_offset`（f32）—— 乐段起点时间
/// - `min_length`（f32，默认 10）—— 最小显示长度
/// - `active`（bool，默认 true）—— 是否激活
///
/// 本版本为 serde 反序列化/JSON 序列化用途，`system::native::period_config::PeriodConfig`
/// 为 Gorge VM 的 native 类注册与运行时构造用途。两者字段语义相同，分工独立。
///
/// 添加 serde 支持以满足谱面 JSON 序列化需求。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeriodConfig {
    /// 乐段起点时间（秒），对应 C# `timeOffset`
    #[serde(default)]
    pub time_offset: f32,
    /// 最小显示长度（秒），注入器默认 10，对应 C# `minLength`
    #[serde(default = "default_min_length")]
    pub min_length: f32,
    /// 是否激活，注入器默认 true，对应 C# `active`
    #[serde(default = "default_active")]
    pub active: bool,
}

fn default_min_length() -> f32 { 10.0 }
fn default_active() -> bool { true }

impl Default for PeriodConfig {
    fn default() -> Self {
        Self {
            time_offset: 0.0,
            min_length: 10.0,
            active: true,
        }
    }
}

impl PeriodConfig {
    /// 从注入器 JSON 数据中解析 PeriodConfig（骨架实现）。
    ///
    /// 完整实现需要 Gorge 注入器实例化系统，此处根据字段名提取。
    /// F 步可通过真实 Injector VM 替换此逻辑。
    pub fn from_injector_json(injector: &serde_json::Value) -> Self {
        let time_offset = injector
            .get("timeOffset")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as f32;
        let min_length = injector
            .get("minLength")
            .and_then(|v| v.as_f64())
            .unwrap_or(10.0) as f32;
        let active = injector
            .get("active")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        Self { time_offset, min_length, active }
    }
}

/// 乐段接口（对应 C# `IPeriod`）。
pub trait IPeriod: Send + Sync + std::fmt::Debug {
    /// 方法名（对应 Gorge 类中带注解的静态方法名）
    fn method_name(&self) -> &str;
    /// 设置方法名
    fn set_method_name(&mut self, name: String);
    /// 乐段设置
    fn config(&self) -> &PeriodConfig;
    /// 设置注入器（以 JSON 形式），返回解析后的配置
    fn config_injector(&self) -> &serde_json::Value;
    /// 更新配置注入器并重新解析 PeriodConfig
    fn update_config(&mut self, injector: serde_json::Value);
    /// 深拷贝
    fn deep_copy(&self) -> Box<dyn IPeriod>;
    /// 生成 Gorge 源码字符串
    fn to_gorge_code(&self, indentation: usize) -> String;
    /// 作为 Any 访问（用于向下转型为具体类型）
    fn as_any(&self) -> &dyn std::any::Any;
}

/// 乐段公共数据（对应 C# `Period` 抽象类的实例字段）。
///
/// ElementPeriod 和 AudioPeriod 均组合此结构。
#[derive(Debug, Clone, PartialEq)]
pub struct PeriodData {
    /// 方法名
    pub method_name: String,
    /// 解析后的乐段设置
    pub config: PeriodConfig,
    /// 设置注入器的 JSON 表示（对应 C# `ConfigInjector`）
    pub config_injector: serde_json::Value,
}

impl PeriodData {
    pub fn new(method_name: String, config_injector: serde_json::Value) -> Self {
        let config = PeriodConfig::from_injector_json(&config_injector);
        Self { method_name, config, config_injector }
    }

    /// 更新配置注入器并重新解析 PeriodConfig
    pub fn update_config(&mut self, injector: serde_json::Value) {
        self.config = PeriodConfig::from_injector_json(&injector);
        self.config_injector = injector;
    }
}

// ==================== ElementPeriod ====================

/// 元素乐段（对应 C# `ElementPeriod`）。
///
/// 对应 Gorge 类中带 `@Chart` 注解的静态方法，包含一组元素注入器。
#[derive(Debug, Clone)]
pub struct ElementPeriod {
    pub period_data: PeriodData,
    /// 所属表单名（对应 `ElementStaff.FormName`）
    pub form_name: String,
    /// 元素注入器列表（对应 C# `List<Injector>`，以 JSON 形式存储）
    pub elements: Vec<serde_json::Value>,
}

impl ElementPeriod {
    pub fn new(form_name: String, method_name: String, config_injector: serde_json::Value) -> Self {
        Self {
            period_data: PeriodData::new(method_name, config_injector),
            form_name,
            elements: Vec::new(),
        }
    }
}

impl IPeriod for ElementPeriod {
    fn method_name(&self) -> &str { &self.period_data.method_name }
    fn set_method_name(&mut self, name: String) { self.period_data.method_name = name; }
    fn config(&self) -> &PeriodConfig { &self.period_data.config }
    fn config_injector(&self) -> &serde_json::Value { &self.period_data.config_injector }

    fn update_config(&mut self, injector: serde_json::Value) {
        self.period_data.update_config(injector);
    }

    fn deep_copy(&self) -> Box<dyn IPeriod> {
        Box::new(ElementPeriod {
            period_data: self.period_data.clone(),
            form_name: self.form_name.clone(),
            elements: self.elements.clone(),
        })
    }

    fn to_gorge_code(&self, indentation: usize) -> String {
        let indent = |n: usize| "    ".repeat(n);
        let mut sb = String::new();
        // 注解元数据部分
        sb.push_str(&format!("{}[\n", indent(indentation)));
        sb.push_str(&format!(
            "{}GorgeFramework.PeriodConfig^ config = {}\n",
            indent(indentation + 1),
            injector_to_gorge_literal(&self.period_data.config_injector, indentation + 1)
        ));
        sb.push_str(&format!("{}]\n", indent(indentation)));
        // @Chart 方法
        sb.push_str(&format!("{}@Chart\n", indent(indentation)));
        sb.push_str(&format!(
            "{}static GorgeFramework.Element^[] {}()\n",
            indent(indentation),
            self.period_data.method_name
        ));
        sb.push_str(&format!("{}{{\n", indent(indentation)));
        if self.elements.is_empty() {
            sb.push_str(&format!(
                "{}return new GorgeFramework.Element^[0];\n",
                indent(indentation + 1)
            ));
        } else {
            let elements_str = self
                .elements
                .iter()
                .map(|e| injector_to_gorge_literal(e, indentation + 2))
                .collect::<Vec<_>>()
                .join(",\n");
            sb.push_str(&format!(
                "{}return new GorgeFramework.Element^[{}]{{\n{}\n{}}};",
                indent(indentation + 1),
                self.elements.len(),
                elements_str,
                indent(indentation + 1)
            ));
            sb.push('\n');
        }
        sb.push_str(&format!("{}}}\n", indent(indentation)));
        sb
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}

// ==================== AudioPeriod ====================

/// 音频乐段（对应 C# `AudioPeriod`）。
///
/// 对应 Gorge 类中带 `@Song` 注解的静态方法，包含音频资源注入器。
#[derive(Debug, Clone)]
pub struct AudioPeriod {
    pub period_data: PeriodData,
    /// 音频资源注入器的 JSON 表示（对应 C# `AudioInjector`）
    pub audio_injector: Option<serde_json::Value>,
}

impl AudioPeriod {
    pub fn new(method_name: String, config_injector: serde_json::Value, audio_injector: Option<serde_json::Value>) -> Self {
        Self {
            period_data: PeriodData::new(method_name, config_injector),
            audio_injector,
        }
    }

    /// 更新音频注入器
    pub fn update_audio(&mut self, audio_injector: Option<serde_json::Value>) {
        self.audio_injector = audio_injector;
    }
}

impl IPeriod for AudioPeriod {
    fn method_name(&self) -> &str { &self.period_data.method_name }
    fn set_method_name(&mut self, name: String) { self.period_data.method_name = name; }
    fn config(&self) -> &PeriodConfig { &self.period_data.config }
    fn config_injector(&self) -> &serde_json::Value { &self.period_data.config_injector }

    fn update_config(&mut self, injector: serde_json::Value) {
        self.period_data.update_config(injector);
    }

    fn deep_copy(&self) -> Box<dyn IPeriod> {
        Box::new(AudioPeriod {
            period_data: self.period_data.clone(),
            audio_injector: self.audio_injector.clone(),
        })
    }

    fn to_gorge_code(&self, indentation: usize) -> String {
        let indent = |n: usize| "    ".repeat(n);
        let mut sb = String::new();
        sb.push_str(&format!("{}[\n", indent(indentation)));
        sb.push_str(&format!(
            "{}GorgeFramework.PeriodConfig^ config = {}\n",
            indent(indentation + 1),
            injector_to_gorge_literal(&self.period_data.config_injector, indentation + 1)
        ));
        sb.push_str(&format!("{}]\n", indent(indentation)));
        sb.push_str(&format!("{}@Song\n", indent(indentation)));
        sb.push_str(&format!(
            "{}static GorgeFramework.AudioAsset^ {}()\n",
            indent(indentation),
            self.period_data.method_name
        ));
        sb.push_str(&format!("{}{{\n", indent(indentation)));
        let audio_literal = self
            .audio_injector
            .as_ref()
            .map(|v| injector_to_gorge_literal(v, indentation + 1))
            .unwrap_or_else(|| "null".to_string());
        sb.push_str(&format!(
            "{}return {};\n",
            indent(indentation + 1),
            audio_literal
        ));
        sb.push_str(&format!("{}}}\n", indent(indentation)));
        sb
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}

// ==================== ToGorgeCode 辅助函数 ====================

/// 将 JSON 值转换为 Gorge 注入器字面量的近似表示。
///
/// 完整实现需 `InjectorHardcodeGenerator`（C# 侧），此处生成简化格式。
fn injector_to_gorge_literal(value: &serde_json::Value, _indentation: usize) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                f.to_string()
            } else {
                n.to_string()
            }
        }
        serde_json::Value::String(s) => format!("\"{}\"", s),
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(|v| injector_to_gorge_literal(v, _indentation)).collect();
            format!("[{}]", items.join(", "))
        }
        serde_json::Value::Object(obj) => {
            let fields: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("{}: {}", k, injector_to_gorge_literal(v, _indentation)))
                .collect();
            if fields.is_empty() {
                "{ : }".to_string()
            } else {
                format!("{{ {} }}", fields.join(", "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_config_injector_json() -> serde_json::Value {
        serde_json::json!({
            "timeOffset": 1.5,
            "minLength": 20.0,
            "active": true
        })
    }

    fn make_element_injector_json(hit_time: f64) -> serde_json::Value {
        serde_json::json!({
            "hitTime": hit_time,
            "position": { "x": 100.0, "y": 200.0 }
        })
    }

    #[test]
    fn test_period_config_from_injector() {
        let json = make_config_injector_json();
        let config = PeriodConfig::from_injector_json(&json);
        assert!((config.time_offset - 1.5).abs() < 0.001);
        assert!((config.min_length - 20.0).abs() < 0.001);
        assert!(config.active);
    }

    #[test]
    fn test_period_config_default() {
        let config = PeriodConfig::default();
        assert_eq!(config.time_offset, 0.0);
        assert_eq!(config.min_length, 10.0);
        assert!(config.active);
    }

    #[test]
    fn test_period_config_from_empty_json() {
        let json = serde_json::json!({});
        let config = PeriodConfig::from_injector_json(&json);
        assert_eq!(config.time_offset, 0.0);
        assert_eq!(config.min_length, 10.0);
        assert!(config.active);
    }

    #[test]
    fn test_element_period_creation() {
        let config_injector = make_config_injector_json();
        let mut period = ElementPeriod::new("TestForm".to_string(), "TestChart".to_string(), config_injector);
        assert_eq!(period.method_name(), "TestChart");
        assert_eq!(period.form_name, "TestForm");
        assert_eq!(period.elements.len(), 0);

        period.elements.push(make_element_injector_json(0.5));
        period.elements.push(make_element_injector_json(1.0));
        assert_eq!(period.elements.len(), 2);
    }

    #[test]
    fn test_element_period_update_config() {
        let config_injector = make_config_injector_json();
        let mut period = ElementPeriod::new("Form".to_string(), "Chart".to_string(), config_injector);
        assert!((period.config().time_offset - 1.5).abs() < 0.001);

        let new_config = serde_json::json!({ "timeOffset": 3.0, "minLength": 30.0, "active": false });
        period.update_config(new_config);
        assert!((period.config().time_offset - 3.0).abs() < 0.001);
        assert!((period.config().min_length - 30.0).abs() < 0.001);
        assert!(!period.config().active);
    }

    #[test]
    fn test_audio_period_creation() {
        let config_injector = make_config_injector_json();
        let audio_injector = serde_json::json!({
            "wavFilePath": "song.wav",
            "offset": 0
        });
        let period = AudioPeriod::new("TestSong".to_string(), config_injector, Some(audio_injector));
        assert_eq!(period.method_name(), "TestSong");
        assert!(period.audio_injector.is_some());
        assert!((period.config().min_length - 20.0).abs() < 0.001);
    }

    #[test]
    fn test_audio_period_no_audio() {
        let config_injector = make_config_injector_json();
        let period = AudioPeriod::new("SilentSong".to_string(), config_injector, None);
        assert_eq!(period.method_name(), "SilentSong");
        assert!(period.audio_injector.is_none());
    }

    #[test]
    fn test_period_deep_copy() {
        let config_injector = make_config_injector_json();
        let mut original = ElementPeriod::new("Form".to_string(), "Chart".to_string(), config_injector);
        original.elements.push(make_element_injector_json(0.5));

        let copy = original.deep_copy();
        assert_eq!(copy.method_name(), "Chart");

        // 深拷贝后修改原对象不应影响副本
        original.set_method_name("Modified".to_string());
        original.elements.clear();
        assert_eq!(copy.method_name(), "Chart");
        let copy_ep = copy.as_any().downcast_ref::<ElementPeriod>().unwrap();
        assert_eq!(copy_ep.elements.len(), 1);
    }

    #[test]
    fn test_to_gorge_code_element_period_no_elements() {
        let config_injector = make_config_injector_json();
        let period = ElementPeriod::new("Form".to_string(), "TestChart".to_string(), config_injector);
        let code = period.to_gorge_code(0);
        assert!(code.contains("@Chart"));
        assert!(code.contains("TestChart"));
        assert!(code.contains("GorgeFramework.PeriodConfig^"));
        assert!(code.contains("GorgeFramework.Element^[0]"));
    }

    #[test]
    fn test_to_gorge_code_element_period_with_elements() {
        let config_injector = make_config_injector_json();
        let mut period = ElementPeriod::new("Form".to_string(), "TestChart".to_string(), config_injector);
        period.elements.push(make_element_injector_json(0.5));
        let code = period.to_gorge_code(1);
        assert!(code.contains("@Chart"));
        assert!(code.contains("GorgeFramework.Element^[1]"));
    }

    #[test]
    fn test_to_gorge_code_audio_period() {
        let config_injector = make_config_injector_json();
        let audio_injector = serde_json::json!({ "wavFilePath": "song.wav" });
        let period = AudioPeriod::new("TestSong".to_string(), config_injector, Some(audio_injector));
        let code = period.to_gorge_code(0);
        assert!(code.contains("@Song"));
        assert!(code.contains("TestSong"));
        assert!(code.contains("GorgeFramework.AudioAsset^"));
    }
}
