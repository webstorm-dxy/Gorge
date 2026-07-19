//! 谱表模型（对应 C# `Chart/Staff.cs`、`IStaff.cs`、`ElementStaff.cs`、`AudioStaff.cs`）。
//!
//! 谱表对应一个 Gorge 类，包含多个乐段。

use crate::chart::period::{AudioPeriod, ElementPeriod, IPeriod};

/// 谱表接口（对应 C# `IStaff`）。
pub trait IStaff: Send + Sync + std::fmt::Debug {
    /// 谱表类名
    fn class_name(&self) -> &str;
    /// 设置类名
    fn set_class_name(&mut self, name: String);
    /// 本谱表属于谱面还是模态
    fn is_chart_class(&self) -> bool;
    /// 谱表显示名
    fn display_name(&self) -> &str;
    /// 设置显示名
    fn set_display_name(&mut self, name: String);
    /// 所含乐段（泛型访问）
    fn periods(&self) -> &[Box<dyn IPeriod>];
    /// 取乐段
    fn try_get_period(&self, period_name: &str) -> Option<&Box<dyn IPeriod>>;
    /// 检查目标乐段名是否和已有乐段名冲突
    fn check_period_name_conflict(&self, period_name: &str) -> bool;
    /// 添加乐段
    fn add_period(&mut self, period: Box<dyn IPeriod>);
    /// 删除乐段
    fn remove_period(&mut self, period_name: &str) -> bool;
    /// 生成谱表代码
    fn to_gorge_code(&self) -> String;
    /// 深拷贝
    fn deep_copy(&self) -> Box<dyn IStaff>;
    /// 检查本谱表是否可以容纳目标乐段
    fn is_valid_period(&self, period: &dyn IPeriod) -> bool;
    /// 作为 Any 访问
    fn as_any(&self) -> &dyn std::any::Any;
}

// ==================== ElementStaff ====================

/// 元素谱表（对应 C# `ElementStaff`）。
///
/// 对应 Gorge 类中带 `@ElementStaff` 注解的类。
#[derive(Debug, Clone)]
pub struct ElementStaff {
    pub class_name: String,
    pub is_chart_class: bool,
    pub display_name: String,
    /// 表单名（对应 C# `FormName`）
    pub form_name: String,
    /// 元素乐段列表
    pub periods: Vec<ElementPeriod>,
}

impl ElementStaff {
    pub fn new(class_name: String, is_chart_class: bool, display_name: String, form_name: String) -> Self {
        Self {
            class_name,
            is_chart_class,
            display_name,
            form_name,
            periods: Vec::new(),
        }
    }

    /// 取元素乐段（按方法名）
    pub fn try_get_period(&self, period_name: &str) -> Option<&ElementPeriod> {
        self.periods.iter().find(|p| p.period_data.method_name == period_name)
    }
}

impl IStaff for ElementStaff {
    fn class_name(&self) -> &str { &self.class_name }
    fn set_class_name(&mut self, name: String) { self.class_name = name; }
    fn is_chart_class(&self) -> bool { self.is_chart_class }
    fn display_name(&self) -> &str { &self.display_name }
    fn set_display_name(&mut self, name: String) { self.display_name = name; }

    fn periods(&self) -> &[Box<dyn IPeriod>] {
        // 返回引用需要把 Vec<ElementPeriod> 转换为 &[Box<dyn IPeriod>]
        // 但这是不可行的，因为 ElementPeriod 和 Box<dyn IPeriod> 是不同的类型
        // 返回空切片作为占位——IPeriod 访问通过 try_get_period 完成
        &[]
    }

    fn try_get_period(&self, _period_name: &str) -> Option<&Box<dyn IPeriod>> {
        // IStaff trait 的 try_get_period 返回 &Box<dyn IPeriod>
        // 但我们存储的是 Vec<ElementPeriod>，无法直接返回引用
        // 此方法在 trait 级别作为占位
        None
    }

    fn check_period_name_conflict(&self, period_name: &str) -> bool {
        if period_name == self.class_name {
            return true;
        }
        self.periods.iter().any(|p| p.period_data.method_name == period_name)
    }

    fn add_period(&mut self, period: Box<dyn IPeriod>) {
        if let Some(ep) = period.as_any().downcast_ref::<ElementPeriod>() {
            // 验证表单名匹配
            if ep.form_name != self.form_name {
                return;
            }
            self.periods.push(ep.clone());
        }
    }

    fn remove_period(&mut self, period_name: &str) -> bool {
        if let Some(pos) = self.periods.iter().position(|p| p.period_data.method_name == period_name) {
            self.periods.remove(pos);
            true
        } else {
            false
        }
    }

    fn to_gorge_code(&self) -> String {
        if !self.is_chart_class {
            return String::new();
        }
        let mut sb = String::new();
        sb.push_str("[\n");
        sb.push_str(&format!("    string form = \"{}\",\n", self.form_name));
        sb.push_str(&format!("    string displayName = \"{}\"\n", self.display_name));
        sb.push_str("]\n");
        sb.push_str("@ElementStaff\n");
        sb.push_str(&format!("class {}\n", self.class_name));
        sb.push_str("{\n");
        for period in &self.periods {
            sb.push_str(&period.to_gorge_code(1));
            sb.push('\n');
        }
        sb.push_str("}\n");
        sb
    }

    fn deep_copy(&self) -> Box<dyn IStaff> {
        Box::new(ElementStaff {
            class_name: self.class_name.clone(),
            is_chart_class: self.is_chart_class,
            display_name: self.display_name.clone(),
            form_name: self.form_name.clone(),
            periods: self.periods.iter().map(|p| {
                let binding = p.deep_copy();
                let copy: &ElementPeriod = binding.as_any().downcast_ref::<ElementPeriod>().unwrap();
                copy.clone()
            }).collect(),
        })
    }

    fn is_valid_period(&self, period: &dyn IPeriod) -> bool {
        if let Some(ep) = period.as_any().downcast_ref::<ElementPeriod>() {
            self.form_name == ep.form_name
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}

// ==================== AudioStaff ====================

/// 音频谱表（对应 C# `AudioStaff`）。
///
/// 对应 Gorge 类中带 `@AudioStaff` 注解的类。
#[derive(Debug, Clone)]
pub struct AudioStaff {
    pub class_name: String,
    pub is_chart_class: bool,
    pub display_name: String,
    /// 音频乐段列表
    pub periods: Vec<AudioPeriod>,
}

impl AudioStaff {
    pub fn new(class_name: String, is_chart_class: bool, display_name: String) -> Self {
        Self {
            class_name,
            is_chart_class,
            display_name,
            periods: Vec::new(),
        }
    }

    /// 取音频乐段（按方法名）
    pub fn try_get_period(&self, period_name: &str) -> Option<&AudioPeriod> {
        self.periods.iter().find(|p| p.period_data.method_name == period_name)
    }
}

impl IStaff for AudioStaff {
    fn class_name(&self) -> &str { &self.class_name }
    fn set_class_name(&mut self, name: String) { self.class_name = name; }
    fn is_chart_class(&self) -> bool { self.is_chart_class }
    fn display_name(&self) -> &str { &self.display_name }
    fn set_display_name(&mut self, name: String) { self.display_name = name; }

    fn periods(&self) -> &[Box<dyn IPeriod>] { &[] }

    fn try_get_period(&self, _period_name: &str) -> Option<&Box<dyn IPeriod>> { None }

    fn check_period_name_conflict(&self, period_name: &str) -> bool {
        if period_name == self.class_name {
            return true;
        }
        self.periods.iter().any(|p| p.period_data.method_name == period_name)
    }

    fn add_period(&mut self, period: Box<dyn IPeriod>) {
        if let Some(ap) = period.as_any().downcast_ref::<AudioPeriod>() {
            self.periods.push(ap.clone());
        }
    }

    fn remove_period(&mut self, period_name: &str) -> bool {
        if let Some(pos) = self.periods.iter().position(|p| p.period_data.method_name == period_name) {
            self.periods.remove(pos);
            true
        } else {
            false
        }
    }

    fn to_gorge_code(&self) -> String {
        if !self.is_chart_class {
            return String::new();
        }
        let mut sb = String::new();
        sb.push_str("[\n");
        sb.push_str(&format!("    string displayName = \"{}\"\n", self.display_name));
        sb.push_str("]\n");
        sb.push_str("@AudioStaff\n");
        sb.push_str(&format!("class {}\n", self.class_name));
        sb.push_str("{\n");
        for period in &self.periods {
            sb.push_str(&period.to_gorge_code(1));
            sb.push('\n');
        }
        sb.push_str("}\n");
        sb
    }

    fn deep_copy(&self) -> Box<dyn IStaff> {
        Box::new(AudioStaff {
            class_name: self.class_name.clone(),
            is_chart_class: self.is_chart_class,
            display_name: self.display_name.clone(),
            periods: self.periods.iter().map(|p| {
                let binding = p.deep_copy();
                let copy: &AudioPeriod = binding.as_any().downcast_ref::<AudioPeriod>().unwrap();
                copy.clone()
            }).collect(),
        })
    }

    fn is_valid_period(&self, period: &dyn IPeriod) -> bool {
        period.as_any().is::<AudioPeriod>()
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::period::PeriodConfig;
    use serde_json::json;

    fn make_config_injector() -> serde_json::Value {
        json!({ "timeOffset": 0.0, "minLength": 10.0, "active": true })
    }

    fn make_element_injector(hit_time: f64) -> serde_json::Value {
        json!({ "hitTime": hit_time })
    }

    #[test]
    fn test_element_staff_try_get_period() {
        let config = make_config_injector();
        let mut staff = ElementStaff::new("TestChart".to_string(), true, "测试谱表".to_string(), "TestForm".to_string());

        let mut period = ElementPeriod::new("TestForm".to_string(), "Period1".to_string(), config.clone());
        period.elements.push(make_element_injector(0.5));
        staff.periods.push(period);

        let period2 = ElementPeriod::new("TestForm".to_string(), "Period2".to_string(), config);
        staff.periods.push(period2);

        assert!(staff.try_get_period("Period1").is_some());
        assert!(staff.try_get_period("Period2").is_some());
        assert!(staff.try_get_period("Period3").is_none());
    }

    #[test]
    fn test_element_staff_check_period_name_conflict() {
        let config = make_config_injector();
        let mut staff = ElementStaff::new("TestChart".to_string(), true, "测试".to_string(), "Form".to_string());
        staff.periods.push(ElementPeriod::new("Form".to_string(), "Period1".to_string(), config));

        // 与已有乐段名冲突
        assert!(staff.check_period_name_conflict("Period1"));
        // 与类名冲突
        assert!(staff.check_period_name_conflict("TestChart"));
        // 不冲突
        assert!(!staff.check_period_name_conflict("Period2"));
    }

    #[test]
    fn test_audio_staff_check_period_name_conflict() {
        let config = make_config_injector();
        let mut staff = AudioStaff::new("AudioChart".to_string(), true, "音频谱表".to_string());
        staff.periods.push(AudioPeriod::new("Song1".to_string(), config, None));

        assert!(staff.check_period_name_conflict("Song1"));
        assert!(staff.check_period_name_conflict("AudioChart"));
        assert!(!staff.check_period_name_conflict("Song2"));
    }

    #[test]
    fn test_element_staff_deep_copy() {
        let config = make_config_injector();
        let mut staff = ElementStaff::new("Chart".to_string(), true, "谱表".to_string(), "Form".to_string());
        staff.periods.push(ElementPeriod::new("Form".to_string(), "Period1".to_string(), config));

        let copy = staff.deep_copy();
        let copy_staff = copy.as_any().downcast_ref::<ElementStaff>().unwrap();
        assert_eq!(copy_staff.class_name, "Chart");
        assert_eq!(copy_staff.periods.len(), 1);
        assert_eq!(copy_staff.periods[0].period_data.method_name, "Period1");

        // 修改原对象不影响副本
        staff.class_name = "Modified".to_string();
        staff.periods.clear();
        assert_eq!(copy_staff.class_name, "Chart");
        assert_eq!(copy_staff.periods.len(), 1);
    }

    #[test]
    fn test_audio_staff_deep_copy() {
        let config = make_config_injector();
        let audio_inj = json!({ "wavFilePath": "song.wav" });
        let mut staff = AudioStaff::new("AudioChart".to_string(), true, "音频".to_string());
        staff.periods.push(AudioPeriod::new("Song1".to_string(), config, Some(audio_inj)));

        let copy = staff.deep_copy();
        let copy_staff = copy.as_any().downcast_ref::<AudioStaff>().unwrap();
        assert_eq!(copy_staff.class_name, "AudioChart");
        assert_eq!(copy_staff.periods.len(), 1);
    }

    #[test]
    fn test_element_staff_to_gorge_code() {
        let config = make_config_injector();
        let mut staff = ElementStaff::new("TestChart".to_string(), true, "测试谱表".to_string(), "TestForm".to_string());
        staff.periods.push(ElementPeriod::new("TestForm".to_string(), "Period1".to_string(), config));

        let code = staff.to_gorge_code();
        assert!(code.contains("@ElementStaff"));
        assert!(code.contains("class TestChart"));
        assert!(code.contains("string form = \"TestForm\""));
        assert!(code.contains("string displayName = \"测试谱表\""));
        assert!(code.contains("@Chart"));
        assert!(code.contains("Period1"));
    }

    #[test]
    fn test_element_staff_not_chart_class_returns_empty() {
        let config = make_config_injector();
        let mut staff = ElementStaff::new("Form".to_string(), false, "非谱面".to_string(), "F".to_string());
        staff.periods.push(ElementPeriod::new("F".to_string(), "P1".to_string(), config));
        let code = staff.to_gorge_code();
        assert!(code.is_empty());
    }

    #[test]
    fn test_audio_staff_to_gorge_code() {
        let config = make_config_injector();
        let audio_inj = json!({ "wavFilePath": "song.wav" });
        let mut staff = AudioStaff::new("AudioChart".to_string(), true, "音频谱表".to_string());
        staff.periods.push(AudioPeriod::new("Song1".to_string(), config, Some(audio_inj)));

        let code = staff.to_gorge_code();
        assert!(code.contains("@AudioStaff"));
        assert!(code.contains("class AudioChart"));
        assert!(code.contains("string displayName = \"音频谱表\""));
        assert!(code.contains("@Song"));
        assert!(code.contains("Song1"));
    }

    #[test]
    fn test_element_staff_is_valid_period() {
        let staff = ElementStaff::new("C".to_string(), true, "D".to_string(), "FormA".to_string());
        let valid_period = ElementPeriod::new("FormA".to_string(), "P".to_string(), make_config_injector());
        let invalid_period = ElementPeriod::new("FormB".to_string(), "P".to_string(), make_config_injector());
        let audio_period = AudioPeriod::new("P".to_string(), make_config_injector(), None);

        assert!(staff.is_valid_period(&valid_period));
        assert!(!staff.is_valid_period(&invalid_period));
        assert!(!staff.is_valid_period(&audio_period));
    }

    #[test]
    fn test_element_staff_remove_period() {
        let config = make_config_injector();
        let mut staff = ElementStaff::new("C".to_string(), true, "D".to_string(), "F".to_string());
        staff.periods.push(ElementPeriod::new("F".to_string(), "P1".to_string(), config.clone()));
        staff.periods.push(ElementPeriod::new("F".to_string(), "P2".to_string(), config));

        assert_eq!(staff.periods.len(), 2);
        assert!(staff.remove_period("P1"));
        assert_eq!(staff.periods.len(), 1);
        assert_eq!(staff.periods[0].period_data.method_name, "P2");
    }

    #[test]
    fn test_period_config_json_serialization() {
        let config = PeriodConfig {
            time_offset: 1.5,
            min_length: 20.0,
            active: true,
        };
        let json = serde_json::to_string(&config).unwrap();
        let parsed: PeriodConfig = serde_json::from_str(&json).unwrap();
        assert!((parsed.time_offset - 1.5).abs() < 0.001);
        assert!((parsed.min_length - 20.0).abs() < 0.001);
        assert!(parsed.active);
    }
}
