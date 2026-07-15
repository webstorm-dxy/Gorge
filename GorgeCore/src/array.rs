use std::fmt::Debug;
use std::sync::Arc;

use crate::native::NativeClass;
use crate::native::NativeContext;
use crate::object::GorgeObject;
use crate::class::GorgeClass;
use crate::types::TypeCount;

// ==================== 数据结构 ====================

/// 原生整数数组
#[derive(Debug, Clone)]
pub struct IntArray {
    pub items: Vec<i64>,
}

/// 原生浮点数组
#[derive(Debug, Clone)]
pub struct FloatArray {
    pub items: Vec<f64>,
}

/// 布尔数组
#[derive(Debug, Clone)]
pub struct BoolArray { pub items: Vec<bool> }

/// 字符串数组
#[derive(Debug, Clone)]
pub struct StringArray { pub items: Vec<String> }

/// 对象数组（对象 ID）
#[derive(Debug, Clone)]
pub struct ObjectArray { pub items: Vec<usize> }

// ==================== IntArray NativeClass ====================

#[derive(Debug, Clone)]
pub struct IntArrayClass;

impl NativeClass for IntArrayClass {
    fn full_name(&self) -> &str { "IntArray" }
    fn field_type_count(&self) -> &TypeCount {
        static TC: std::sync::OnceLock<TypeCount> = std::sync::OnceLock::new();
        TC.get_or_init(|| TypeCount { int_count: 1, ..TypeCount::zero() })
    }

    fn invoke_native_method(&self, ctx: &mut NativeContext, obj_id: usize, method_id: usize) {
        let exists = ctx.native_payloads.contains_key(&obj_id);
        if exists {
            // 借用拆分：分别处理 get(只读) 和 set(可变)
            match method_id {
                0 => { // get(index) → int
                    let i = ctx.get_int_param(0) as usize;
                    let v = ctx.native_payloads.get(&obj_id)
                        .and_then(|p| p.downcast_ref::<IntArray>())
                        .and_then(|a| a.items.get(i).copied())
                        .unwrap_or(0);
                    ctx.set_int_return(v);
                }
                1 => { // set(index, value) → void
                    let i = ctx.get_int_param(0) as usize;
                    let v = ctx.get_int_param(1);
                    let mut payload = ctx.native_payloads.get_mut(&obj_id);
                    if let Some(p) = &mut payload {
                        if let Some(a) = p.downcast_mut::<IntArray>() {
                            if i < a.items.len() { a.items[i] = v; }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _: Option<usize>, _: usize) -> usize {
        let id = *ctx.next_object_id; *ctx.next_object_id += 1;
        let length = ctx.get_int_param(0) as usize;
        ctx.native_payloads.insert(id, Box::new(IntArray { items: vec![0i64; length] }));
        id
    }
}

// ==================== FloatArray NativeClass ====================

#[derive(Debug, Clone)]
pub struct FloatArrayClass;

impl NativeClass for FloatArrayClass {
    fn full_name(&self) -> &str { "FloatArray" }
    fn field_type_count(&self) -> &TypeCount {
        static TC: std::sync::OnceLock<TypeCount> = std::sync::OnceLock::new();
        TC.get_or_init(|| TypeCount { int_count: 1, ..TypeCount::zero() })
    }
    fn invoke_native_method(&self, ctx: &mut NativeContext, obj_id: usize, method_id: usize) {
        match method_id {
            0 => { let i = ctx.get_int_param(0) as usize; let v = ctx.native_payloads.get(&obj_id).and_then(|p| p.downcast_ref::<FloatArray>()).and_then(|a| a.items.get(i).copied()).unwrap_or(0.0); ctx.set_float_return(v); }
            1 => { let i = ctx.get_int_param(0) as usize; let v = ctx.get_float_param(1); let mut p = ctx.native_payloads.get_mut(&obj_id); if let Some(p) = &mut p { if let Some(a) = p.downcast_mut::<FloatArray>() { if i < a.items.len() { a.items[i] = v; } } } }
            _ => {}
        }
    }
    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _: Option<usize>, _: usize) -> usize {
        let id = *ctx.next_object_id; *ctx.next_object_id += 1;
        let length = ctx.get_int_param(0) as usize;
        ctx.native_payloads.insert(id, Box::new(FloatArray { items: vec![0.0f64; length] }));
        id
    }
}

// ==================== BoolArray NativeClass ====================

#[derive(Debug, Clone)]
pub struct BoolArrayClass;
impl NativeClass for BoolArrayClass {
    fn full_name(&self) -> &str { "BoolArray" }
    fn field_type_count(&self) -> &TypeCount {
        static TC: std::sync::OnceLock<TypeCount> = std::sync::OnceLock::new();
        TC.get_or_init(|| TypeCount { int_count: 1, ..TypeCount::zero() })
    }
    fn invoke_native_method(&self, ctx: &mut NativeContext, obj_id: usize, method_id: usize) {
        match method_id { 0 => { let i = ctx.get_int_param(0) as usize; let v = ctx.native_payloads.get(&obj_id).and_then(|p| p.downcast_ref::<BoolArray>()).and_then(|a| a.items.get(i).copied()).unwrap_or(false); ctx.set_bool_return(v); } 1 => { let i = ctx.get_int_param(0) as usize; let v = ctx.get_bool_param(1); let mut p = ctx.native_payloads.get_mut(&obj_id); if let Some(p) = &mut p { if let Some(a) = p.downcast_mut::<BoolArray>() { if i < a.items.len() { a.items[i] = v; } } } } _ => {} }
    }
    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _: Option<usize>, _: usize) -> usize {
        let id = *ctx.next_object_id; *ctx.next_object_id += 1;
        let length = ctx.get_int_param(0) as usize;
        ctx.native_payloads.insert(id, Box::new(BoolArray { items: vec![false; length] }));
        id
    }
}

// ==================== StringArray NativeClass ====================

#[derive(Debug, Clone)]
pub struct StringArrayClass;
impl NativeClass for StringArrayClass {
    fn full_name(&self) -> &str { "StringArray" }
    fn field_type_count(&self) -> &TypeCount {
        static TC: std::sync::OnceLock<TypeCount> = std::sync::OnceLock::new();
        TC.get_or_init(|| TypeCount { int_count: 1, ..TypeCount::zero() })
    }
    fn invoke_native_method(&self, ctx: &mut NativeContext, obj_id: usize, method_id: usize) {
        match method_id { 0 => { let i = ctx.get_int_param(0) as usize; let v = ctx.native_payloads.get(&obj_id).and_then(|p| p.downcast_ref::<StringArray>()).and_then(|a| a.items.get(i).cloned()).unwrap_or_default(); ctx.set_string_return(v); } 1 => { let i = ctx.get_int_param(0) as usize; let v = ctx.get_string_param(1); let mut p = ctx.native_payloads.get_mut(&obj_id); if let Some(p) = &mut p { if let Some(a) = p.downcast_mut::<StringArray>() { if i < a.items.len() { a.items[i] = v; } } } } _ => {} }
    }
    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _: Option<usize>, _: usize) -> usize {
        let id = *ctx.next_object_id; *ctx.next_object_id += 1;
        let length = ctx.get_int_param(0) as usize;
        ctx.native_payloads.insert(id, Box::new(StringArray { items: vec![String::new(); length] }));
        id
    }
}

// ==================== ObjectArray NativeClass ====================

#[derive(Debug, Clone)]
pub struct ObjectArrayClass;
impl NativeClass for ObjectArrayClass {
    fn full_name(&self) -> &str { "ObjectArray" }
    fn field_type_count(&self) -> &TypeCount {
        static TC: std::sync::OnceLock<TypeCount> = std::sync::OnceLock::new();
        TC.get_or_init(|| TypeCount { int_count: 1, ..TypeCount::zero() })
    }
    fn invoke_native_method(&self, ctx: &mut NativeContext, obj_id: usize, method_id: usize) {
        match method_id { 0 => { let i = ctx.get_int_param(0) as usize; let v = ctx.native_payloads.get(&obj_id).and_then(|p| p.downcast_ref::<ObjectArray>()).and_then(|a| a.items.get(i).copied()).unwrap_or(0); ctx.set_object_return(v); } 1 => { let i = ctx.get_int_param(0) as usize; let v = ctx.get_object_param(1); let mut p = ctx.native_payloads.get_mut(&obj_id); if let Some(p) = &mut p { if let Some(a) = p.downcast_mut::<ObjectArray>() { if i < a.items.len() { a.items[i] = v; } } } } _ => {} }
    }
    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _: Option<usize>, _: usize) -> usize {
        let id = *ctx.next_object_id; *ctx.next_object_id += 1;
        let length = ctx.get_int_param(0) as usize;
        ctx.native_payloads.insert(id, Box::new(ObjectArray { items: vec![0usize; length] }));
        id
    }
}

/// 原生数组 trait（对齐 C# NativeArray）
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

impl GorgeObject for IntArray {
    fn gorge_class(&self) -> &Arc<dyn GorgeClass> { unimplemented!() }
    fn get_int_field(&self, _: usize) -> i64 { 0 } fn get_float_field(&self, _: usize) -> f64 { 0.0 } fn get_bool_field(&self, _: usize) -> bool { false } fn get_string_field(&self, _: usize) -> String { String::new() } fn get_object_field(&self, _: usize) -> usize { 0 }
    fn set_int_field(&mut self, _: usize, _: i64) {} fn set_float_field(&mut self, _: usize, _: f64) {} fn set_bool_field(&mut self, _: usize, _: bool) {} fn set_string_field(&mut self, _: usize, _: String) {} fn set_object_field(&mut self, _: usize, _: usize) {}
    fn invoke_method(&mut self, _: usize) {}
}
impl NativeArray for IntArray { fn length(&self) -> usize { self.items.len() } fn get_item_int(&self, i: usize) -> i64 { self.items[i] } fn set_item_int(&mut self, i: usize, v: i64) { self.items[i] = v; } }

impl GorgeObject for FloatArray {
    fn gorge_class(&self) -> &Arc<dyn GorgeClass> { unimplemented!() }
    fn get_int_field(&self, _: usize) -> i64 { 0 } fn get_float_field(&self, _: usize) -> f64 { 0.0 } fn get_bool_field(&self, _: usize) -> bool { false } fn get_string_field(&self, _: usize) -> String { String::new() } fn get_object_field(&self, _: usize) -> usize { 0 }
    fn set_int_field(&mut self, _: usize, _: i64) {} fn set_float_field(&mut self, _: usize, _: f64) {} fn set_bool_field(&mut self, _: usize, _: bool) {} fn set_string_field(&mut self, _: usize, _: String) {} fn set_object_field(&mut self, _: usize, _: usize) {}
    fn invoke_method(&mut self, _: usize) {}
}
impl NativeArray for FloatArray { fn length(&self) -> usize { self.items.len() } fn get_item_float(&self, i: usize) -> f64 { self.items[i] } fn set_item_float(&mut self, i: usize, v: f64) { self.items[i] = v; } }

impl GorgeObject for BoolArray {
    fn gorge_class(&self) -> &Arc<dyn GorgeClass> { unimplemented!() }
    fn get_int_field(&self, _: usize) -> i64 { 0 } fn get_float_field(&self, _: usize) -> f64 { 0.0 } fn get_bool_field(&self, _: usize) -> bool { false } fn get_string_field(&self, _: usize) -> String { String::new() } fn get_object_field(&self, _: usize) -> usize { 0 }
    fn set_int_field(&mut self, _: usize, _: i64) {} fn set_float_field(&mut self, _: usize, _: f64) {} fn set_bool_field(&mut self, _: usize, _: bool) {} fn set_string_field(&mut self, _: usize, _: String) {} fn set_object_field(&mut self, _: usize, _: usize) {}
    fn invoke_method(&mut self, _: usize) {}
}
impl NativeArray for BoolArray { fn length(&self) -> usize { self.items.len() } fn get_item_bool(&self, i: usize) -> bool { self.items[i] } fn set_item_bool(&mut self, i: usize, v: bool) { self.items[i] = v; } }

impl GorgeObject for StringArray {
    fn gorge_class(&self) -> &Arc<dyn GorgeClass> { unimplemented!() }
    fn get_int_field(&self, _: usize) -> i64 { 0 } fn get_float_field(&self, _: usize) -> f64 { 0.0 } fn get_bool_field(&self, _: usize) -> bool { false } fn get_string_field(&self, _: usize) -> String { String::new() } fn get_object_field(&self, _: usize) -> usize { 0 }
    fn set_int_field(&mut self, _: usize, _: i64) {} fn set_float_field(&mut self, _: usize, _: f64) {} fn set_bool_field(&mut self, _: usize, _: bool) {} fn set_string_field(&mut self, _: usize, _: String) {} fn set_object_field(&mut self, _: usize, _: usize) {}
    fn invoke_method(&mut self, _: usize) {}
}
impl NativeArray for StringArray { fn length(&self) -> usize { self.items.len() } fn get_item_string(&self, i: usize) -> String { self.items[i].clone() } fn set_item_string(&mut self, i: usize, v: String) { self.items[i] = v; } }

impl GorgeObject for ObjectArray {
    fn gorge_class(&self) -> &Arc<dyn GorgeClass> { unimplemented!() }
    fn get_int_field(&self, _: usize) -> i64 { 0 } fn get_float_field(&self, _: usize) -> f64 { 0.0 } fn get_bool_field(&self, _: usize) -> bool { false } fn get_string_field(&self, _: usize) -> String { String::new() } fn get_object_field(&self, _: usize) -> usize { 0 }
    fn set_int_field(&mut self, _: usize, _: i64) {} fn set_float_field(&mut self, _: usize, _: f64) {} fn set_bool_field(&mut self, _: usize, _: bool) {} fn set_string_field(&mut self, _: usize, _: String) {} fn set_object_field(&mut self, _: usize, _: usize) {}
    fn invoke_method(&mut self, _: usize) {}
}
impl NativeArray for ObjectArray { fn length(&self) -> usize { self.items.len() } fn get_item_object(&self, i: usize) -> usize { self.items[i] } fn set_item_object(&mut self, i: usize, v: usize) { self.items[i] = v; } }

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_int_array_length() {
        let arr = IntArray { items: vec![1, 2, 3] };
        assert_eq!(arr.items.len(), 3);
    }

    #[test]
    fn test_native_array_construct() {
        let mut pool = crate::param_pool::InvokeParameterPool::new();
        let mut objects = HashMap::new();
        let mut next = 100;
        let mut payloads: HashMap<usize, Box<dyn std::any::Any>> = HashMap::new();
        pool.set_int_param(0, 5); // length = 5
        let mut ctx = NativeContext::new(&mut pool, &mut objects, &mut next, &mut payloads);

        let cls = IntArrayClass;
        let id = cls.do_construct_native(&mut ctx, None, 0);
        assert!(id >= 100);
        assert!(payloads.contains_key(&id));
        // 验证数组有 5 个元素（全为 0）
        if let Some(p) = payloads.get(&id) {
            let arr = p.downcast_ref::<IntArray>().unwrap();
            assert_eq!(arr.items.len(), 5);
            assert!(arr.items.iter().all(|&v| v == 0));
        }
    }

    #[test]
    fn test_native_array_set_get() {
        let mut pool = crate::param_pool::InvokeParameterPool::new();
        let mut objects = HashMap::new();
        let mut next = 100;
        let mut payloads: HashMap<usize, Box<dyn std::any::Any>> = HashMap::new();
        pool.set_int_param(0, 3);
        let mut ctx = NativeContext::new(&mut pool, &mut objects, &mut next, &mut payloads);

        let cls = FloatArrayClass;
        let id = cls.do_construct_native(&mut ctx, None, 0);

        // set(1, 3.14)
        pool.set_int_param(0, 1);
        pool.set_float_param(1, 3.14);
        let mut ctx2 = NativeContext::new(&mut pool, &mut objects, &mut next, &mut payloads);
        cls.invoke_native_method(&mut ctx2, id, 1);

        // get(1) → 3.14
        pool.set_int_param(0, 1);
        let mut ctx3 = NativeContext::new(&mut pool, &mut objects, &mut next, &mut payloads);
        cls.invoke_native_method(&mut ctx3, id, 0);
        assert!((pool.get_float_return() - 3.14).abs() < 0.001);
    }
}
