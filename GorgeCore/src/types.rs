use std::fmt;

/// 基本类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BasicType {
    Int,
    Float,
    Bool,
    Enum,
    String,
    Object,
    Interface,
    Delegate,
    Void,
    /// null 字面量类型，可自动转换为任意 Object/Interface/Delegate
    Null,
}

impl fmt::Display for BasicType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BasicType::Int => write!(f, "int"),
            BasicType::Float => write!(f, "float"),
            BasicType::Bool => write!(f, "bool"),
            BasicType::Enum => write!(f, "enum"),
            BasicType::String => write!(f, "string"),
            BasicType::Object => write!(f, "object"),
            BasicType::Interface => write!(f, "interface"),
            BasicType::Delegate => write!(f, "delegate"),
            BasicType::Void => write!(f, "void"),
            BasicType::Null => write!(f, "null"),
        }
    }
}

/// Gorge 类型（含命名空间、泛型、子类型信息）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GorgeType {
    pub basic_type: BasicType,
    pub class_name: Option<String>,
    pub namespace_name: Option<String>,
    pub is_generics: bool,
    pub sub_types: Vec<GorgeType>,
}

impl GorgeType {
    pub fn new(basic_type: BasicType) -> Self {
        Self {
            basic_type,
            class_name: None,
            namespace_name: None,
            is_generics: false,
            sub_types: Vec::new(),
        }
    }

    pub fn class(name: impl Into<String>, namespace: Option<String>) -> Self {
        Self {
            basic_type: BasicType::Object,
            class_name: Some(name.into()),
            namespace_name: namespace,
            is_generics: false,
            sub_types: Vec::new(),
        }
    }

    pub fn full_name(&self) -> String {
        match self.basic_type {
            BasicType::Int => "int".into(),
            BasicType::Float => "float".into(),
            BasicType::Bool => "bool".into(),
            BasicType::Enum => "".into(),
            BasicType::String => "string".into(),
            BasicType::Void => "void".into(),
            BasicType::Null => "null".into(),
            BasicType::Interface | BasicType::Object | BasicType::Delegate => {
                let name = self.class_name.as_deref().unwrap_or("?");
                match &self.namespace_name {
                    Some(ns) if !ns.is_empty() => format!("{}.{}", ns, name),
                    _ => name.to_string(),
                }
            }
        }
    }

    pub fn is_void(&self) -> bool {
        self.basic_type == BasicType::Void
    }

    /// null 字面量类型
    pub fn is_null(&self) -> bool {
        self.basic_type == BasicType::Null
    }

    /// 构造 null 类型实例
    pub fn null() -> Self {
        Self::new(BasicType::Null)
    }
}

impl Default for GorgeType {
    fn default() -> Self {
        GorgeType::new(BasicType::Void)
    }
}

/// 五类型计数器
///
/// 跟踪 int/float/bool/string/object 各类型的字段数量，
/// 用于计算对象内存布局中各类型字段的索引偏移。
#[derive(Debug, Clone, Default)]
pub struct TypeCount {
    pub int_count: usize,
    pub float_count: usize,
    pub bool_count: usize,
    pub string_count: usize,
    pub object_count: usize,
}

impl TypeCount {
    /// 全部归零
    pub fn zero() -> Self {
        Self::default()
    }

    /// 累加另一个 TypeCount
    pub fn add(&mut self, other: &TypeCount) {
        self.int_count += other.int_count;
        self.float_count += other.float_count;
        self.bool_count += other.bool_count;
        self.string_count += other.string_count;
        self.object_count += other.object_count;
    }

    /// 从当前值中减去另一个 TypeCount
    pub fn minus(&mut self, other: &TypeCount) {
        self.int_count = self.int_count.saturating_sub(other.int_count);
        self.float_count = self.float_count.saturating_sub(other.float_count);
        self.bool_count = self.bool_count.saturating_sub(other.bool_count);
        self.string_count = self.string_count.saturating_sub(other.string_count);
        self.object_count = self.object_count.saturating_sub(other.object_count);
    }

    /// 每种类型追加一个计数
    pub fn add_one(&mut self, basic_type: BasicType) {
        match basic_type {
            BasicType::Int | BasicType::Enum => self.int_count += 1,
            BasicType::Float => self.float_count += 1,
            BasicType::Bool => self.bool_count += 1,
            BasicType::String => self.string_count += 1,
            BasicType::Null => {} // null 不占用字段计数
            _ => self.object_count += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gorge_type_full_name() {
        let t = GorgeType::class("List", Some("Gorge.Collections".into()));
        assert_eq!(t.full_name(), "Gorge.Collections.List");
    }

    #[test]
    fn test_basic_type_display() {
        assert_eq!(format!("{}", BasicType::Int), "int");
        assert_eq!(format!("{}", BasicType::Float), "float");
    }

    #[test]
    fn test_type_count_add() {
        let mut a = TypeCount::zero();
        a.int_count = 2;
        let mut b = TypeCount::zero();
        b.int_count = 3;
        b.float_count = 1;
        a.add(&b);
        assert_eq!(a.int_count, 5);
        assert_eq!(a.float_count, 1);
    }

    #[test]
    fn test_type_count_minus() {
        let mut a = TypeCount { int_count: 5, float_count: 3, ..TypeCount::zero() };
        let b = TypeCount { int_count: 2, float_count: 1, ..TypeCount::zero() };
        a.minus(&b);
        assert_eq!(a.int_count, 3);
        assert_eq!(a.float_count, 2);
    }

    #[test]
    fn test_type_count_add_one() {
        let mut tc = TypeCount::zero();
        tc.add_one(BasicType::Int);
        tc.add_one(BasicType::Int);
        tc.add_one(BasicType::Float);
        tc.add_one(BasicType::Object);
        assert_eq!(tc.int_count, 2);
        assert_eq!(tc.float_count, 1);
        assert_eq!(tc.object_count, 1);
    }
}
