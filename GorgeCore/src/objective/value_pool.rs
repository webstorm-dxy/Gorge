use crate::objective::types::TypeCount;

/// 固定大小字段值池
///
/// 编译时确定每种类型的字段数量，运行时按类型索引高效访问。
/// 对应 C# 的 FixedFieldValuePool。
#[derive(Debug, Clone, Default)]
pub struct FixedFieldValuePool {
    pub ints: Vec<i64>,
    pub floats: Vec<f64>,
    pub bools: Vec<bool>,
    pub strings: Vec<String>,
    pub objects: Vec<usize>,
}

impl FixedFieldValuePool {
    /// 按指定大小分配所有数组
    pub fn new(counts: &TypeCount) -> Self {
        Self {
            ints: vec![0; counts.int_count],
            floats: vec![0.0; counts.float_count],
            bools: vec![false; counts.bool_count],
            strings: vec![String::new(); counts.string_count],
            objects: vec![0; counts.object_count],
        }
    }

    /// 获取整数
    pub fn get_int(&self, index: usize) -> i64 {
        self.ints[index]
    }

    /// 设置整数
    pub fn set_int(&mut self, index: usize, value: i64) {
        self.ints[index] = value;
    }

    /// 获取浮点数
    pub fn get_float(&self, index: usize) -> f64 {
        self.floats[index]
    }

    /// 设置浮点数
    pub fn set_float(&mut self, index: usize, value: f64) {
        self.floats[index] = value;
    }

    /// 获取布尔
    pub fn get_bool(&self, index: usize) -> bool {
        self.bools[index]
    }

    /// 设置布尔
    pub fn set_bool(&mut self, index: usize, value: bool) {
        self.bools[index] = value;
    }

    /// 获取字符串引用
    pub fn get_string(&self, index: usize) -> &str {
        &self.strings[index]
    }

    /// 设置字符串
    pub fn set_string(&mut self, index: usize, value: String) {
        self.strings[index] = value;
    }

    /// 获取对象 ID
    pub fn get_object(&self, index: usize) -> usize {
        self.objects[index]
    }

    /// 设置对象 ID
    pub fn set_object(&mut self, index: usize, id: usize) {
        self.objects[index] = id;
    }

    /// 序列比较两个值池是否相等（对齐 C# FixedFieldValuePool.Equals）
    ///
    /// 逐元素比较所有类型字段，object 字段按 ID 比较（引用相等语义）。
    pub fn equals(&self, other: &FixedFieldValuePool) -> bool {
        if self.ints.len() != other.ints.len()
            || self.floats.len() != other.floats.len()
            || self.bools.len() != other.bools.len()
            || self.strings.len() != other.strings.len()
            || self.objects.len() != other.objects.len()
        {
            return false;
        }
        for i in 0..self.ints.len() {
            if self.ints[i] != other.ints[i] { return false; }
        }
        for i in 0..self.floats.len() {
            if self.floats[i] != other.floats[i] { return false; }
        }
        for i in 0..self.bools.len() {
            if self.bools[i] != other.bools[i] { return false; }
        }
        for i in 0..self.strings.len() {
            if self.strings[i] != other.strings[i] { return false; }
        }
        for i in 0..self.objects.len() {
            if self.objects[i] != other.objects[i] { return false; }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_pool_allocate_and_access() {
        let counts = TypeCount {
            int_count: 3,
            float_count: 2,
            bool_count: 1,
            string_count: 2,
            object_count: 0,
        };
        let mut pool = FixedFieldValuePool::new(&counts);

        pool.set_int(0, 42);
        pool.set_int(1, 100);
        assert_eq!(pool.get_int(0), 42);
        assert_eq!(pool.get_int(1), 100);

        pool.set_string(0, "hello".into());
        assert_eq!(pool.get_string(0), "hello");
    }

    #[test]
    fn test_value_pool_defaults() {
        let counts = TypeCount::zero();
        let pool = FixedFieldValuePool::new(&counts);
        assert!(pool.ints.is_empty());
        assert!(pool.floats.is_empty());
    }

    // ==================== A-4 equals 测试 ====================

    #[test]
    fn test_a4_value_pool_equals() {
        let counts = TypeCount { int_count: 2, float_count: 1, bool_count: 1, string_count: 1, object_count: 0 };
        let mut a = FixedFieldValuePool::new(&counts);
        let mut b = FixedFieldValuePool::new(&counts);

        // 相同默认值应相等
        assert!(a.equals(&b));

        // int 字段不同
        a.set_int(0, 42);
        assert!(!a.equals(&b));
        b.set_int(0, 42);
        assert!(a.equals(&b));

        // float 字段不同
        a.set_float(0, 3.14);
        assert!(!a.equals(&b));
        b.set_float(0, 3.14);
        assert!(a.equals(&b));

        // bool 字段不同
        a.set_bool(0, true);
        assert!(!a.equals(&b));
        b.set_bool(0, true);
        assert!(a.equals(&b));

        // string 字段不同
        a.set_string(0, "hello".into());
        assert!(!a.equals(&b));
        b.set_string(0, "hello".into());
        assert!(a.equals(&b));
    }

    /// 不同大小的池一定不相等
    #[test]
    fn test_a4_value_pool_equals_different_size() {
        let c1 = TypeCount { int_count: 1, ..TypeCount::zero() };
        let c2 = TypeCount { int_count: 2, ..TypeCount::zero() };
        let a = FixedFieldValuePool::new(&c1);
        let b = FixedFieldValuePool::new(&c2);
        assert!(!a.equals(&b));
    }
}
