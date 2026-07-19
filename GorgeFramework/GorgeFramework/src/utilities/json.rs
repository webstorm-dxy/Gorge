//! JSON 序列化/反序列化工具（对应 C# `Utilities/Json/` 文件夹）。
//!
//! 为谱面数据模型提供与 C# 序列化输出严格一致的 serde 结构。

use serde::{Deserialize, Serialize};

/// 二维向量，JSON 格式对齐 C# `GorgeVector2Converter`：`{"x": value, "y": value}`。
///
/// C# 使用 Newtonsoft.Json 的 `JsonConverter<Vector2>` 自定义读写 `{x, y}` 对象。
/// Rust 侧通过 serde derive 即可自然输出相同格式。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GorgeVector2 {
    pub x: f32,
    pub y: f32,
}

impl GorgeVector2 {
    /// 创建新的向量
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Default for GorgeVector2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// 三维向量，JSON 格式 `{"x": value, "y": value, "z": value}`。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GorgeVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl GorgeVector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Default for GorgeVector3 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gorge_vector2_json_roundtrip() {
        let v = GorgeVector2::new(1.052632, -1.6);
        let json = serde_json::to_string(&v).unwrap();
        // C# 格式：{"x":1.052632,"y":-1.6}
        let expected = r#"{"x":1.052632,"y":-1.6}"#;
        assert_eq!(json, expected, "序列化结果应与 C# 格式一致");

        let v2: GorgeVector2 = serde_json::from_str(&json).unwrap();
        assert!((v2.x - v.x).abs() < 0.0001);
        assert!((v2.y - v.y).abs() < 0.0001);
    }

    #[test]
    fn test_gorge_vector2_from_csharp_format() {
        // 手工构造的 C# 格式 JSON fixture
        let json = r#"{"x":3.14,"y":42.0}"#;
        let v: GorgeVector2 = serde_json::from_str(json).unwrap();
        assert!((v.x - 3.14).abs() < 0.0001);
        assert!((v.y - 42.0).abs() < 0.0001);
    }

    #[test]
    fn test_gorge_vector2_default() {
        let v = GorgeVector2::default();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, r#"{"x":0.0,"y":0.0}"#);
    }

    #[test]
    fn test_gorge_vector3_json_roundtrip() {
        let v = GorgeVector3::new(1.0, 2.0, 3.0);
        let json = serde_json::to_string(&v).unwrap();
        let v2: GorgeVector3 = serde_json::from_str(&json).unwrap();
        assert_eq!(v, v2);
    }
}
