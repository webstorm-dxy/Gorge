use crate::types::TypeCount;

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
}
