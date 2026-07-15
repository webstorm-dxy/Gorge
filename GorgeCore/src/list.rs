use std::fmt::Debug;

use crate::native::NativeClass;
use crate::native::NativeContext;
use crate::types::TypeCount;

// ==================== 数据结构 ====================

/// 原生整数列表
#[derive(Debug, Clone)]
pub struct IntList {
    pub items: Vec<i64>,
}

/// 原生浮点列表
#[derive(Debug, Clone)]
pub struct FloatList {
    pub items: Vec<f64>,
}

/// 布尔列表
#[derive(Debug, Clone)]
pub struct BoolList { pub items: Vec<bool> }

/// 字符串列表
#[derive(Debug, Clone)]
pub struct StringList { pub items: Vec<String> }

/// 对象列表（对象 ID）
#[derive(Debug, Clone)]
pub struct ObjectList { pub items: Vec<usize> }

// ==================== IntList NativeClass ====================

#[derive(Debug, Clone)]
pub struct IntListClass;

impl NativeClass for IntListClass {
    fn full_name(&self) -> &str { "IntList" }
    fn field_type_count(&self) -> &TypeCount {
        static TC: std::sync::OnceLock<TypeCount> = std::sync::OnceLock::new();
        TC.get_or_init(|| TypeCount { int_count: 1, ..TypeCount::zero() })
    }

    fn invoke_native_method(&self, ctx: &mut NativeContext, obj_id: usize, method_id: usize) {
        match method_id {
            0 => {
                let i = ctx.get_int_param(0) as usize;
                if let Some(v) = ctx.native_payloads.get(&obj_id)
                    .and_then(|p| p.downcast_ref::<IntList>())
                    .and_then(|l| if i < l.items.len() { Some(l.items[i]) } else { None })
                {
                    ctx.set_int_return(v);
                }
            }
            1 => {
                let i = ctx.get_int_param(0) as usize;
                let v = ctx.get_int_param(1);
                if let Some(payload) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(list) = payload.downcast_mut::<IntList>() {
                        if i < list.items.len() { list.items[i] = v; }
                    }
                }
            }
            2 => {
                let v = ctx.get_int_param(0);
                if let Some(payload) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(list) = payload.downcast_mut::<IntList>() {
                        list.items.push(v);
                    }
                }
            }
            3 => {
                let i = ctx.get_int_param(0) as usize;
                if let Some(payload) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(list) = payload.downcast_mut::<IntList>() {
                        if i < list.items.len() { list.items.remove(i); }
                    }
                }
            }
            _ => {}
        }
    }

    fn invoke_native_static(&self, _ctx: &mut NativeContext, _method_id: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _target: Option<usize>, _ctor_id: usize) -> usize {
        let obj_id = *ctx.next_object_id; *ctx.next_object_id += 1;
        ctx.native_payloads.insert(obj_id, Box::new(IntList { items: Vec::new() }));
        obj_id
    }
}

// ==================== FloatList NativeClass ====================

#[derive(Debug, Clone)]
pub struct FloatListClass;

impl NativeClass for FloatListClass {
    fn full_name(&self) -> &str { "FloatList" }
    fn field_type_count(&self) -> &TypeCount {
        static TC: std::sync::OnceLock<TypeCount> = std::sync::OnceLock::new();
        TC.get_or_init(|| TypeCount { int_count: 1, ..TypeCount::zero() })
    }
    fn invoke_native_method(&self, ctx: &mut NativeContext, obj_id: usize, method_id: usize) {
        match method_id {
            0 => {
                let i = ctx.get_int_param(0) as usize;
                if let Some(v) = ctx.native_payloads.get(&obj_id)
                    .and_then(|p| p.downcast_ref::<FloatList>())
                    .and_then(|l| if i < l.items.len() { Some(l.items[i]) } else { None })
                {
                    ctx.set_float_return(v);
                }
            }
            1 => {
                let i = ctx.get_int_param(0) as usize;
                let v = ctx.get_float_param(1);
                if let Some(payload) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(list) = payload.downcast_mut::<FloatList>() {
                        if i < list.items.len() { list.items[i] = v; }
                    }
                }
            }
            2 => {
                let v = ctx.get_float_param(0);
                if let Some(payload) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(list) = payload.downcast_mut::<FloatList>() {
                        list.items.push(v);
                    }
                }
            }
            3 => {
                let i = ctx.get_int_param(0) as usize;
                if let Some(payload) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(list) = payload.downcast_mut::<FloatList>() {
                        if i < list.items.len() { list.items.remove(i); }
                    }
                }
            }
            _ => {}
        }
    }
    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _: Option<usize>, _: usize) -> usize {
        let id = *ctx.next_object_id; *ctx.next_object_id += 1;
        ctx.native_payloads.insert(id, Box::new(FloatList { items: Vec::new() }));
        id
    }
}

// ==================== BoolList NativeClass ====================

#[derive(Debug, Clone)]
pub struct BoolListClass;
impl NativeClass for BoolListClass {
    fn full_name(&self) -> &str { "BoolList" }
    fn field_type_count(&self) -> &TypeCount {
        static TC: std::sync::OnceLock<TypeCount> = std::sync::OnceLock::new();
        TC.get_or_init(|| TypeCount { int_count: 1, ..TypeCount::zero() })
    }
    fn invoke_native_method(&self, ctx: &mut NativeContext, obj_id: usize, method_id: usize) {
        match method_id {
            0 => {
                let i = ctx.get_int_param(0) as usize;
                if let Some(v) = ctx.native_payloads.get(&obj_id)
                    .and_then(|p| p.downcast_ref::<BoolList>())
                    .and_then(|l| if i < l.items.len() { Some(l.items[i]) } else { None })
                {
                    ctx.set_bool_return(v);
                }
            }
            1 => {
                let i = ctx.get_int_param(0) as usize;
                let v = ctx.get_bool_param(1);
                if let Some(p) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(l) = p.downcast_mut::<BoolList>() {
                        if i < l.items.len() { l.items[i] = v; }
                    }
                }
            }
            2 => {
                let v = ctx.get_bool_param(0);
                if let Some(p) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(l) = p.downcast_mut::<BoolList>() {
                        l.items.push(v);
                    }
                }
            }
            3 => {
                let i = ctx.get_int_param(0) as usize;
                if let Some(p) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(l) = p.downcast_mut::<BoolList>() {
                        if i < l.items.len() { l.items.remove(i); }
                    }
                }
            }
            _ => {}
        }
    }
    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _: Option<usize>, _: usize) -> usize {
        let id = *ctx.next_object_id; *ctx.next_object_id += 1;
        ctx.native_payloads.insert(id, Box::new(BoolList { items: Vec::new() }));
        id
    }
}

// ==================== StringList NativeClass ====================

#[derive(Debug, Clone)]
pub struct StringListClass;
impl NativeClass for StringListClass {
    fn full_name(&self) -> &str { "StringList" }
    fn field_type_count(&self) -> &TypeCount {
        static TC: std::sync::OnceLock<TypeCount> = std::sync::OnceLock::new();
        TC.get_or_init(|| TypeCount { int_count: 1, ..TypeCount::zero() })
    }
    fn invoke_native_method(&self, ctx: &mut NativeContext, obj_id: usize, method_id: usize) {
        match method_id {
            0 => {
                let i = ctx.get_int_param(0) as usize;
                if let Some(v) = ctx.native_payloads.get(&obj_id)
                    .and_then(|p| p.downcast_ref::<StringList>())
                    .and_then(|l| if i < l.items.len() { Some(l.items[i].clone()) } else { None })
                {
                    ctx.set_string_return(v);
                }
            }
            1 => {
                let i = ctx.get_int_param(0) as usize;
                let v = ctx.get_string_param(1);
                if let Some(p) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(l) = p.downcast_mut::<StringList>() {
                        if i < l.items.len() { l.items[i] = v; }
                    }
                }
            }
            2 => {
                let v = ctx.get_string_param(0);
                if let Some(p) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(l) = p.downcast_mut::<StringList>() {
                        l.items.push(v);
                    }
                }
            }
            3 => {
                let i = ctx.get_int_param(0) as usize;
                if let Some(p) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(l) = p.downcast_mut::<StringList>() {
                        if i < l.items.len() { l.items.remove(i); }
                    }
                }
            }
            _ => {}
        }
    }
    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _: Option<usize>, _: usize) -> usize {
        let id = *ctx.next_object_id; *ctx.next_object_id += 1;
        ctx.native_payloads.insert(id, Box::new(StringList { items: Vec::new() }));
        id
    }
}

// ==================== ObjectList NativeClass ====================

#[derive(Debug, Clone)]
pub struct ObjectListClass;
impl NativeClass for ObjectListClass {
    fn full_name(&self) -> &str { "ObjectList" }
    fn field_type_count(&self) -> &TypeCount {
        static TC: std::sync::OnceLock<TypeCount> = std::sync::OnceLock::new();
        TC.get_or_init(|| TypeCount { int_count: 1, ..TypeCount::zero() })
    }
    fn invoke_native_method(&self, ctx: &mut NativeContext, obj_id: usize, method_id: usize) {
        match method_id {
            0 => {
                let i = ctx.get_int_param(0) as usize;
                if let Some(v) = ctx.native_payloads.get(&obj_id)
                    .and_then(|p| p.downcast_ref::<ObjectList>())
                    .and_then(|l| if i < l.items.len() { Some(l.items[i]) } else { None })
                {
                    ctx.set_object_return(v);
                }
            }
            1 => {
                let i = ctx.get_int_param(0) as usize;
                let v = ctx.get_object_param(1);
                if let Some(p) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(l) = p.downcast_mut::<ObjectList>() {
                        if i < l.items.len() { l.items[i] = v; }
                    }
                }
            }
            2 => {
                let v = ctx.get_object_param(0);
                if let Some(p) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(l) = p.downcast_mut::<ObjectList>() {
                        l.items.push(v);
                    }
                }
            }
            3 => {
                let i = ctx.get_int_param(0) as usize;
                if let Some(p) = ctx.native_payloads.get_mut(&obj_id) {
                    if let Some(l) = p.downcast_mut::<ObjectList>() {
                        if i < l.items.len() { l.items.remove(i); }
                    }
                }
            }
            _ => {}
        }
    }
    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _: Option<usize>, _: usize) -> usize {
        let id = *ctx.next_object_id; *ctx.next_object_id += 1;
        ctx.native_payloads.insert(id, Box::new(ObjectList { items: Vec::new() }));
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_list_ops() {
        let mut list = IntList { items: Vec::new() };
        list.items.push(1);
        list.items.push(2);
        assert_eq!(list.items[0], 1);
        assert_eq!(list.items[1], 2);
        list.items.remove(0);
        assert_eq!(list.items.len(), 1);
        assert_eq!(list.items[0], 2);
        list.items[0] = 99;
        assert_eq!(list.items[0], 99);
    }

    #[test]
    fn test_float_list_ops() {
        let mut list = FloatList { items: Vec::new() };
        list.items.push(1.5);
        assert!((list.items[0] - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_native_class_construct() {
        use std::collections::HashMap;
        let pool = crate::param_pool::InvokeParameterPool::new();
        let mut objects = HashMap::new();
        let mut next = 100;
        let mut payloads: HashMap<usize, Box<dyn std::any::Any>> = HashMap::new();
        let mut ctx = NativeContext::new(&pool, &mut objects, &mut next, &mut payloads);

        let cls = IntListClass;
        let id = cls.do_construct_native(&mut ctx, None, 0);
        assert!(id >= 100);
        assert!(payloads.contains_key(&id));
    }

    #[test]
    fn test_native_class_add_get() {
        use std::collections::HashMap;
        let pool = crate::param_pool::InvokeParameterPool::new();
        let mut objects = HashMap::new();
        let mut next = 100;
        let mut payloads: HashMap<usize, Box<dyn std::any::Any>> = HashMap::new();
        let mut ctx = NativeContext::new(&pool, &mut objects, &mut next, &mut payloads);

        let cls = IntListClass;
        let id = cls.do_construct_native(&mut ctx, None, 0);

        pool.set_int_param(0, 42);
        let mut ctx2 = NativeContext::new(&pool, &mut objects, &mut next, &mut payloads);
        cls.invoke_native_method(&mut ctx2, id, 2); // add(42)

        pool.set_int_param(0, 0);
        let mut ctx3 = NativeContext::new(&pool, &mut objects, &mut next, &mut payloads);
        cls.invoke_native_method(&mut ctx3, id, 0); // get(0)
        assert_eq!(pool.get_int_return(), 42);
    }
}
