use std::cell::RefCell;

/// 调用参数池
///
/// 对应 C# 的 InvokeParameterPool，用于在方法调用间传递参数和返回值。
/// 每种值类型有独立数组，默认容量 256。
const POOL_SIZE: usize = 256;

/// 参数池条目（值 + 是否设置标记）
#[derive(Debug, Clone)]
pub struct PoolEntry<T: Clone> {
    pub value: T,
    pub is_set: bool,
}

impl<T: Clone + Default> Default for PoolEntry<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
            is_set: false,
        }
    }
}

/// 调用参数池
#[derive(Debug, Clone)]
pub struct InvokeParameterPool {
    pub int_params: RefCell<[PoolEntry<i64>; POOL_SIZE]>,
    pub float_params: RefCell<[PoolEntry<f64>; POOL_SIZE]>,
    pub bool_params: RefCell<[PoolEntry<bool>; POOL_SIZE]>,
    pub string_params: RefCell<[PoolEntry<String>; POOL_SIZE]>,
    pub object_params: RefCell<[PoolEntry<usize>; POOL_SIZE]>,

    /// 返回值
    pub int_return: RefCell<i64>,
    pub float_return: RefCell<f64>,
    pub bool_return: RefCell<bool>,
    pub string_return: RefCell<String>,
    pub object_return: RefCell<usize>,
}

impl InvokeParameterPool {
    pub fn new() -> Self {
        Self {
            int_params: RefCell::new(std::array::from_fn(|_| PoolEntry::default())),
            float_params: RefCell::new(std::array::from_fn(|_| PoolEntry::default())),
            bool_params: RefCell::new(std::array::from_fn(|_| PoolEntry::default())),
            string_params: RefCell::new(std::array::from_fn(|_| PoolEntry::default())),
            object_params: RefCell::new(std::array::from_fn(|_| PoolEntry::default())),
            int_return: RefCell::new(0),
            float_return: RefCell::new(0.0),
            bool_return: RefCell::new(false),
            string_return: RefCell::new(String::new()),
            object_return: RefCell::new(0),
        }
    }

    /// 设置整数参数
    pub fn set_int_param(&self, index: usize, value: i64) {
        let mut params = self.int_params.borrow_mut();
        params[index] = PoolEntry {
            value,
            is_set: true,
        };
    }

    /// 获取整数参数
    pub fn get_int_param(&self, index: usize) -> i64 {
        self.int_params.borrow()[index].value
    }

    /// 设置浮点参数
    pub fn set_float_param(&self, index: usize, value: f64) {
        let mut params = self.float_params.borrow_mut();
        params[index] = PoolEntry {
            value,
            is_set: true,
        };
    }

    /// 获取浮点参数
    pub fn get_float_param(&self, index: usize) -> f64 {
        self.float_params.borrow()[index].value
    }

    /// 设置布尔参数
    pub fn set_bool_param(&self, index: usize, value: bool) {
        let mut params = self.bool_params.borrow_mut();
        params[index] = PoolEntry {
            value,
            is_set: true,
        };
    }

    /// 获取布尔参数
    pub fn get_bool_param(&self, index: usize) -> bool {
        self.bool_params.borrow()[index].value
    }

    /// 设置字符串参数
    pub fn set_string_param(&self, index: usize, value: String) {
        let mut params = self.string_params.borrow_mut();
        params[index] = PoolEntry {
            value,
            is_set: true,
        };
    }

    /// 获取字符串参数
    pub fn get_string_param(&self, index: usize) -> String {
        self.string_params.borrow()[index].value.clone()
    }

    /// 设置对象参数
    pub fn set_object_param(&self, index: usize, value: usize) {
        let mut params = self.object_params.borrow_mut();
        params[index] = PoolEntry {
            value,
            is_set: true,
        };
    }

    /// 获取对象参数
    pub fn get_object_param(&self, index: usize) -> usize {
        self.object_params.borrow()[index].value
    }

    /// 获取整数返回值
    pub fn get_int_return(&self) -> i64 {
        *self.int_return.borrow()
    }

    /// 设置整数返回值
    pub fn set_int_return(&self, value: i64) {
        *self.int_return.borrow_mut() = value;
    }

    /// 获取浮点返回值
    pub fn get_float_return(&self) -> f64 {
        *self.float_return.borrow()
    }

    /// 设置浮点返回值
    pub fn set_float_return(&self, value: f64) {
        *self.float_return.borrow_mut() = value;
    }

    /// 获取布尔返回值
    pub fn get_bool_return(&self) -> bool {
        *self.bool_return.borrow()
    }

    /// 设置布尔返回值
    pub fn set_bool_return(&self, value: bool) {
        *self.bool_return.borrow_mut() = value;
    }

    /// 获取字符串返回值
    pub fn get_string_return(&self) -> String {
        self.string_return.borrow().clone()
    }

    /// 设置字符串返回值
    pub fn set_string_return(&self, value: String) {
        *self.string_return.borrow_mut() = value;
    }

    /// 获取对象返回值
    pub fn get_object_return(&self) -> usize {
        *self.object_return.borrow()
    }

    /// 设置对象返回值
    pub fn set_object_return(&self, value: usize) {
        *self.object_return.borrow_mut() = value;
    }

    /// 重置所有参数
    pub fn reset(&self) {
        for p in self.int_params.borrow_mut().iter_mut() {
            *p = PoolEntry::default();
        }
        for p in self.float_params.borrow_mut().iter_mut() {
            *p = PoolEntry::default();
        }
        for p in self.bool_params.borrow_mut().iter_mut() {
            *p = PoolEntry::default();
        }
        for p in self.string_params.borrow_mut().iter_mut() {
            *p = PoolEntry::default();
        }
        for p in self.object_params.borrow_mut().iter_mut() {
            *p = PoolEntry::default();
        }
    }
}

impl Default for InvokeParameterPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_pool_set_get() {
        let pool = InvokeParameterPool::new();
        pool.set_int_param(0, 42);
        assert_eq!(pool.get_int_param(0), 42);
    }

    #[test]
    fn test_param_pool_return() {
        let pool = InvokeParameterPool::new();
        pool.set_int_return(99);
        assert_eq!(pool.get_int_return(), 99);
    }

    #[test]
    fn test_param_pool_reset() {
        let pool = InvokeParameterPool::new();
        pool.set_int_param(0, 42);
        assert_eq!(pool.get_int_param(0), 42);
        pool.reset();
        assert_eq!(pool.get_int_param(0), 0);
    }
}
