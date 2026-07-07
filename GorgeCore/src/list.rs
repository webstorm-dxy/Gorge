use std::fmt::Debug;
use std::sync::Arc;
use crate::object::GorgeObject;
use crate::class::GorgeClass;

/// 原生整数列表
#[derive(Debug)]
pub struct IntList {
    pub items: Vec<i64>,
}

impl IntList {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, value: i64) {
        self.items.push(value);
    }

    pub fn get(&self, index: usize) -> i64 {
        self.items[index]
    }

    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}

impl GorgeObject for IntList {
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

/// 原生浮点列表
#[derive(Debug)]
pub struct FloatList {
    pub items: Vec<f64>,
}

impl FloatList {
    pub fn new() -> Self { Self { items: Vec::new() } }
    pub fn add(&mut self, value: f64) { self.items.push(value); }
    pub fn get(&self, index: usize) -> f64 { self.items[index] }
    pub fn remove(&mut self, index: usize) { self.items.remove(index); }
    pub fn count(&self) -> usize { self.items.len() }
}

impl GorgeObject for FloatList {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_list_add_get() {
        let mut list = IntList::new();
        list.add(1);
        list.add(2);
        assert_eq!(list.get(0), 1);
        assert_eq!(list.get(1), 2);
        assert_eq!(list.count(), 2);
    }

    #[test]
    fn test_int_list_remove() {
        let mut list = IntList::new();
        list.add(1);
        list.add(2);
        list.remove(0);
        assert_eq!(list.count(), 1);
        assert_eq!(list.get(0), 2);
    }
}
