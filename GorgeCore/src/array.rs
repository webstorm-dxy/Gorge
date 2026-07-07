use std::fmt::Debug;
use std::sync::Arc;
use crate::object::GorgeObject;
use crate::class::GorgeClass;

/// 原生数组 trait
///
/// 对应 C# 的 IntArray / FloatArray / BoolArray / StringArray / ObjectArray。
/// 泛型实现，编译时确定元素类型。
pub trait NativeArray: GorgeObject {
    fn length(&self) -> usize;
    fn get_item_int(&self, _index: usize) -> i64 { 0 }
    fn get_item_float(&self, _index: usize) -> f64 { 0.0 }
    fn get_item_bool(&self, _index: usize) -> bool { false }
    fn get_item_string(&self, _index: usize) -> String { String::new() }
    fn get_item_object(&self, _index: usize) -> usize { 0 }
    fn set_item_int(&mut self, _index: usize, _value: i64) {}
    fn set_item_float(&mut self, _index: usize, _value: f64) {}
    fn set_item_bool(&mut self, _index: usize, _value: bool) {}
    fn set_item_string(&mut self, _index: usize, _value: String) {}
    fn set_item_object(&mut self, _index: usize, _value: usize) {}
}

/// 原生整数数组
#[derive(Debug)]
pub struct IntArray {
    pub items: Vec<i64>,
}

impl IntArray {
    pub fn new(items: Vec<i64>) -> Self {
        Self { items }
    }
}

impl GorgeObject for IntArray {
    fn gorge_class(&self) -> &Arc<dyn GorgeClass> {
        unimplemented!("原生数组尚未绑定 GorgeClass")
    }
    fn get_int_field(&self, _index: usize) -> i64 { 0 }
    fn get_float_field(&self, _index: usize) -> f64 { 0.0 }
    fn get_bool_field(&self, _index: usize) -> bool { false }
    fn get_string_field(&self, _index: usize) -> String { String::new() }
    fn get_object_field(&self, _index: usize) -> usize { 0 }
    fn set_int_field(&mut self, _index: usize, _value: i64) {}
    fn set_float_field(&mut self, _index: usize, _value: f64) {}
    fn set_bool_field(&mut self, _index: usize, _value: bool) {}
    fn set_string_field(&mut self, _index: usize, _value: String) {}
    fn set_object_field(&mut self, _index: usize, _value: usize) {}
    fn invoke_method(&mut self, _method_id: usize) {}
}

impl NativeArray for IntArray {
    fn length(&self) -> usize {
        self.items.len()
    }
    fn get_item_int(&self, index: usize) -> i64 {
        self.items[index]
    }
    fn set_item_int(&mut self, index: usize, value: i64) {
        self.items[index] = value;
    }
}

/// 原生浮点数组
#[derive(Debug)]
pub struct FloatArray {
    pub items: Vec<f64>,
}

impl FloatArray {
    pub fn new(items: Vec<f64>) -> Self {
        Self { items }
    }
}

impl GorgeObject for FloatArray {
    fn gorge_class(&self) -> &Arc<dyn GorgeClass> { unimplemented!() }
    fn get_int_field(&self, _: usize) -> i64 { 0 }
    fn get_float_field(&self, _: usize) -> f64 { 0.0 }
    fn get_bool_field(&self, _: usize) -> bool { false }
    fn get_string_field(&self, _: usize) -> String { String::new() }
    fn get_object_field(&self, _: usize) -> usize { 0 }
    fn set_int_field(&mut self, _: usize, _: i64) {}
    fn set_float_field(&mut self, _: usize, _: f64) {}
    fn set_bool_field(&mut self, _: usize, _: bool) {}
    fn set_string_field(&mut self, _: usize, _: String) {}
    fn set_object_field(&mut self, _: usize, _: usize) {}
    fn invoke_method(&mut self, _: usize) {}
}

impl NativeArray for FloatArray {
    fn length(&self) -> usize { self.items.len() }
    fn get_item_float(&self, index: usize) -> f64 { self.items[index] }
    fn set_item_float(&mut self, index: usize, value: f64) { self.items[index] = value; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_array_length() {
        let arr = IntArray::new(vec![1, 2, 3]);
        assert_eq!(arr.length(), 3);
    }

    #[test]
    fn test_int_array_get_item() {
        let arr = IntArray::new(vec![10, 20]);
        assert_eq!(arr.get_item_int(0), 10);
        assert_eq!(arr.get_item_int(1), 20);
    }
}
