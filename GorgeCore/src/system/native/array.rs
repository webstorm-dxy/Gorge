use std::fmt::Debug;
use std::sync::Arc;

use crate::objective::native::NativeClass;
use crate::objective::native::NativeContext;
use crate::objective::object::GorgeObject;
use crate::objective::class::GorgeClass;
use crate::objective::types::TypeCount;

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
        let exists = ctx.vm.native_payloads.contains_key(&obj_id);
        if exists {
            // 借用拆分：分别处理 get(只读) 和 set(可变)
            match method_id {
                0 => { // get(index) → int
                    let i = ctx.get_int_param(0) as usize;
                    let v = ctx.vm.native_payloads.get(&obj_id)
                        .and_then(|p| p.downcast_ref::<IntArray>())
                        .and_then(|a| a.items.get(i).copied())
                        .unwrap_or(0);
                    ctx.set_int_return(v);
                }
                1 => { // set(index, value) → void
                    let i = ctx.get_int_param(0) as usize;
                    let v = ctx.get_int_param(1);
                    let mut payload = ctx.vm.native_payloads.get_mut(&obj_id);
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
    fn do_construct_native(&self, ctx: &mut NativeContext, _target: Option<usize>, _ctor_id: usize) -> usize {
        let id = ctx.vm.next_object_id; ctx.vm.next_object_id += 1;
        let inj_len = ctx.injector_int(0).unwrap_or(0) as usize;
        let param_len = if ctx.get_int_param(0) > 0 { ctx.get_int_param(0) as usize } else { 0 };
        let length = if inj_len > 0 { inj_len } else { param_len };
        ctx.vm.native_payloads.insert(id, Box::new(IntArray { items: vec![0i64; length] }));
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
            0 => { let i = ctx.get_int_param(0) as usize; let v = ctx.vm.native_payloads.get(&obj_id).and_then(|p| p.downcast_ref::<FloatArray>()).and_then(|a| a.items.get(i).copied()).unwrap_or(0.0); ctx.set_float_return(v); }
            1 => { let i = ctx.get_int_param(0) as usize; let v = ctx.get_float_param(1); let mut p = ctx.vm.native_payloads.get_mut(&obj_id); if let Some(p) = &mut p { if let Some(a) = p.downcast_mut::<FloatArray>() { if i < a.items.len() { a.items[i] = v; } } } }
            _ => {}
        }
    }
    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _target: Option<usize>, _ctor_id: usize) -> usize {
        let id = ctx.vm.next_object_id; ctx.vm.next_object_id += 1;
        let inj_len = ctx.injector_int(0).unwrap_or(0) as usize;
        let param_len = if ctx.get_int_param(0) > 0 { ctx.get_int_param(0) as usize } else { 0 };
        let length = if inj_len > 0 { inj_len } else { param_len };
        ctx.vm.native_payloads.insert(id, Box::new(FloatArray { items: vec![0.0f64; length] }));
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
        match method_id { 0 => { let i = ctx.get_int_param(0) as usize; let v = ctx.vm.native_payloads.get(&obj_id).and_then(|p| p.downcast_ref::<BoolArray>()).and_then(|a| a.items.get(i).copied()).unwrap_or(false); ctx.set_bool_return(v); } 1 => { let i = ctx.get_int_param(0) as usize; let v = ctx.get_bool_param(1); let mut p = ctx.vm.native_payloads.get_mut(&obj_id); if let Some(p) = &mut p { if let Some(a) = p.downcast_mut::<BoolArray>() { if i < a.items.len() { a.items[i] = v; } } } } _ => {} }
    }
    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _: Option<usize>, _: usize) -> usize {
        let id = ctx.vm.next_object_id; ctx.vm.next_object_id += 1;
        let inj_len = ctx.injector_int(0).unwrap_or(0) as usize;
        let param_len = if ctx.get_int_param(0) > 0 { ctx.get_int_param(0) as usize } else { 0 };
        let length = if inj_len > 0 { inj_len } else { param_len };
        ctx.vm.native_payloads.insert(id, Box::new(BoolArray { items: vec![false; length] }));
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
        match method_id { 0 => { let i = ctx.get_int_param(0) as usize; let v = ctx.vm.native_payloads.get(&obj_id).and_then(|p| p.downcast_ref::<StringArray>()).and_then(|a| a.items.get(i).cloned()).unwrap_or_default(); ctx.set_string_return(v); } 1 => { let i = ctx.get_int_param(0) as usize; let v = ctx.get_string_param(1); let mut p = ctx.vm.native_payloads.get_mut(&obj_id); if let Some(p) = &mut p { if let Some(a) = p.downcast_mut::<StringArray>() { if i < a.items.len() { a.items[i] = v; } } } } _ => {} }
    }
    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _: Option<usize>, _: usize) -> usize {
        let id = ctx.vm.next_object_id; ctx.vm.next_object_id += 1;
        let inj_len = ctx.injector_int(0).unwrap_or(0) as usize;
        let param_len = if ctx.get_int_param(0) > 0 { ctx.get_int_param(0) as usize } else { 0 };
        let length = if inj_len > 0 { inj_len } else { param_len };
        ctx.vm.native_payloads.insert(id, Box::new(StringArray { items: vec![String::new(); length] }));
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
        match method_id { 0 => { let i = ctx.get_int_param(0) as usize; let v = ctx.vm.native_payloads.get(&obj_id).and_then(|p| p.downcast_ref::<ObjectArray>()).and_then(|a| a.items.get(i).copied()).unwrap_or(0); ctx.set_object_return(v); } 1 => { let i = ctx.get_int_param(0) as usize; let v = ctx.get_object_param(1); let mut p = ctx.vm.native_payloads.get_mut(&obj_id); if let Some(p) = &mut p { if let Some(a) = p.downcast_mut::<ObjectArray>() { if i < a.items.len() { a.items[i] = v; } } } } 2 => { let v = ctx.get_object_param(0); if let Some(p) = ctx.vm.native_payloads.get_mut(&obj_id) { if let Some(a) = p.downcast_mut::<ObjectArray>() { a.items.push(v); } } } 3 => { let len = ctx.vm.native_payloads.get(&obj_id).and_then(|p| p.downcast_ref::<ObjectArray>()).map(|a| a.items.len() as i64).unwrap_or(0); ctx.set_int_return(len); } _ => {} }
    }
    fn invoke_native_static(&self, _: &mut NativeContext, _: usize) {}
    fn do_construct_native(&self, ctx: &mut NativeContext, _target: Option<usize>, _ctor_id: usize) -> usize {
        let id = ctx.vm.next_object_id; ctx.vm.next_object_id += 1;
        let inj_len = ctx.injector_int(0).unwrap_or(0) as usize;
        let param_len = if ctx.get_int_param(0) > 0 { ctx.get_int_param(0) as usize } else { 0 };
        let length = if inj_len > 0 { inj_len } else { param_len };
        ctx.vm.native_payloads.insert(id, Box::new(ObjectArray { items: vec![0usize; length] }));
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
    

    #[test]
    fn test_int_array_length() {
        let arr = IntArray { items: vec![1, 2, 3] };
        assert_eq!(arr.items.len(), 3);
    }

    #[test]
    fn test_native_array_construct() {
        let mut vm = crate::virtual_machine::vm::VirtualMachine::new();
        vm.next_object_id = 100;
        vm.param_pool.set_int_param(0, 5); // length = 5
        let mut ctx = NativeContext::new(&mut vm);

        let cls = IntArrayClass;
        let id = cls.do_construct_native(&mut ctx, None, 0);
        assert!(id >= 100);
        // ctx 已 dropped，vm 可访问
        assert!(vm.native_payloads.contains_key(&id));
        // 验证数组有 5 个元素（全为 0）
        if let Some(p) = vm.native_payloads.get(&id) {
            let arr = p.downcast_ref::<IntArray>().unwrap();
            assert_eq!(arr.items.len(), 5);
            assert!(arr.items.iter().all(|&v| v == 0));
        }
    }

    #[test]
    fn test_native_array_set_get() {
        let mut vm = crate::virtual_machine::vm::VirtualMachine::new();
        vm.next_object_id = 100;
        vm.param_pool.set_int_param(0, 3);
        let mut ctx = NativeContext::new(&mut vm);

        let cls = FloatArrayClass;
        let id = cls.do_construct_native(&mut ctx, None, 0);

        // set(1, 3.14)
        vm.param_pool.set_int_param(0, 1);
        vm.param_pool.set_float_param(1, 3.14);
        let mut ctx2 = NativeContext::new(&mut vm);
        cls.invoke_native_method(&mut ctx2, id, 1);

        // get(1) → 3.14
        vm.param_pool.set_int_param(0, 1);
        let mut ctx3 = NativeContext::new(&mut vm);
        cls.invoke_native_method(&mut ctx3, id, 0);
        assert!((vm.param_pool.get_float_return() - 3.14).abs() < 0.001);
    }

    // ==================== A-1 注入器构造测试 ====================

    /// ObjectArray 通过注入器 length 字段预分配容量
    #[test]
    fn test_a1_objectarray_injector_construct() {
        use crate::system::native::injector::{RuntimeInjector, Injector};
        use crate::objective::types::TypeCount;
        use crate::objective::declaration::ClassDeclaration;
        use std::sync::Arc;

        let mut vm = crate::virtual_machine::vm::VirtualMachine::new();
        vm.next_object_id = 100;

        let decl = Arc::new(ClassDeclaration {
            injector_field_type_count: TypeCount { int_count: 1, ..TypeCount::zero() },
            ..ClassDeclaration::dummy("ObjectArray".into())
        });
        let mut inj = RuntimeInjector::new(decl);
        inj.set_injector_int(0, 3); // length = 3
        let inj_id = vm.next_object_id;
        vm.next_object_id += 1;
        vm.injectors.insert(inj_id, inj);

        let mut ctx = NativeContext::with_injector(&mut vm, inj_id);
        let cls = ObjectArrayClass;
        let id = cls.do_construct_native(&mut ctx, None, 0);
        let len = vm.native_payloads.get(&id)
            .and_then(|p| p.downcast_ref::<ObjectArray>())
            .map(|a| a.items.len())
            .unwrap_or(999);
        assert_eq!(len, 3, "通过注入器 length=3 构造 ObjectArray 应有 3 个元素");
    }
}
