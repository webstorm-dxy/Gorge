use crate::objective::declaration::MethodInfo;

/// Gorge 接口定义
///
/// 对应 C# 的 GorgeInterface，包含接口方法表。
#[derive(Debug, Clone)]
pub struct GorgeInterface {
    pub name: String,
    pub full_name: String,
    pub methods: Vec<MethodInfo>,
    pub super_interfaces: Vec<String>,
}

impl GorgeInterface {
    pub fn new(name: impl Into<String>, full_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            full_name: full_name.into(),
            methods: Vec::new(),
            super_interfaces: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_creation() {
        let iface = GorgeInterface::new("IComparable", "Gorge.IComparable");
        assert_eq!(iface.name, "IComparable");
        assert_eq!(iface.full_name, "Gorge.IComparable");
    }

    #[test]
    fn test_interface_with_methods() {
        let mut iface = GorgeInterface::new("IComparable", "Gorge.IComparable");
        iface.methods.push(MethodInfo {
            name: "compare".into(),
            return_type: crate::objective::types::GorgeType::new(crate::objective::types::BasicType::Int),
            parameters: vec![],
            is_static: false,
            is_native: false,
            is_override: false,
            is_abstract: true,
        });
        assert_eq!(iface.methods.len(), 1);
    }
}
