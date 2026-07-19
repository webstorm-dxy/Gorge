//! `GorgeFramework.ElementSimulator` —— 元素模拟器 native 类。
//!
//! 移植自 C# 参考实现 `ElementSimulator.cs`。
//! 持有 transformers ObjectArray，构造时拷贝为内部 payload Vec<usize>。
//! ISimulator 相关方法（ForwardSimulate 等）留待 S4/S7 实现。

use gorge_macros::{gorge_native_class, gorge_native_impl};
use gorge_core::objective::native::NativeContext;

// ==================== ElementSimulator 内部 payload ====================

/// ElementSimulator 内部存储（存于 vm.native_payloads）
#[derive(Debug)]
pub struct ElementSimulatorPayload {
    /// 变换器列表（对象 ID）
    pub transformers: Vec<usize>,
}

// ==================== ElementSimulator（native 注册） ====================

/// 元素模拟器
///
/// 对齐 C# `ElementSimulator`。持有变换器对象列表，
/// 在模拟时调用各变换器的 ITransformer.Transform 方法。
/// ISimulator 相关方法留待 S4/S7 实现。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct ElementSimulator {
    /// 变换器列表（ObjectArray 对象 ID）
    #[gorge_field]
    pub transformers: usize,
}

#[gorge_native_impl]
impl ElementSimulator {
    #[gorge_ctor]
    pub fn new_ctor(ctx: &mut NativeContext, this: usize, transformers: usize) {
        ctx.set_object_object_field(this, ElementSimulator::FIELD_INDEX_transformers, transformers);
        // 从 ObjectArray 拷贝变换器列表到内部 payload
        let items = ctx.object_array_items(transformers);
        ctx.insert_payload(this, Box::new(ElementSimulatorPayload { transformers: items }));
    }

    /// 获取内部变换器列表（对象 ID 列表）
    #[gorge_method]
    pub fn get_transformers(ctx: &mut NativeContext, this: usize) -> i32 {
        with_payload(ctx, this, |p| p.transformers.len() as i32)
    }
}

// ==================== 辅助函数 ====================

pub fn with_payload<T>(ctx: &NativeContext, this: usize, f: impl FnOnce(&ElementSimulatorPayload) -> T) -> T {
    let default = ElementSimulatorPayload { transformers: Vec::new() };
    let payload = ctx.get_payload::<ElementSimulatorPayload>(this).unwrap_or(&default);
    f(payload)
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::system::native::array::ObjectArrayClass;
    use gorge_core::virtual_machine::vm::VirtualMachine;

    #[test]
    fn test_element_simulator_construct_empty() {
        let es = ElementSimulator { transformers: 0 };
        let mut vm = VirtualMachine::new();
        // 构造空 ObjectArray
        let cls = ObjectArrayClass;
        let arr_id = { let mut ctx = NativeContext::new(&mut vm); cls.do_construct_native(&mut ctx, None, 0) };
        vm.param_pool.set_object_param(0, arr_id);
        let id = { let mut ctx = NativeContext::new(&mut vm); es.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        assert!(vm.native_payloads.contains_key(&id));
        assert_eq!(with_payload(&NativeContext::new(&mut vm), id, |p| p.transformers.len()), 0);
    }

    #[test]
    fn test_element_simulator_construct_with_items() {
        let es = ElementSimulator { transformers: 0 };
        let mut vm = VirtualMachine::new();
        let cls = ObjectArrayClass;
        let arr_id = { let mut ctx = NativeContext::new(&mut vm); cls.do_construct_native(&mut ctx, None, 0) };
        // 向 ObjectArray 添加元素
        {
            let mut ctx = NativeContext::new(&mut vm);
            ctx.object_array_add(arr_id, 100);
            ctx.object_array_add(arr_id, 200);
            ctx.object_array_add(arr_id, 300);
        }
        vm.param_pool.set_object_param(0, arr_id);
        let id = { let mut ctx = NativeContext::new(&mut vm); es.do_construct_native(&mut ctx, None, 0) };
        assert_eq!(with_payload(&NativeContext::new(&mut vm), id, |p| p.transformers.clone()), vec![100, 200, 300]);
    }

    #[test]
    fn test_element_simulator_get_transformers() {
        let es = ElementSimulator { transformers: 0 };
        let mut vm = VirtualMachine::new();
        let cls = ObjectArrayClass;
        let arr_id = { let mut ctx = NativeContext::new(&mut vm); cls.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = NativeContext::new(&mut vm); ctx.object_array_add(arr_id, 42); }
        vm.param_pool.set_object_param(0, arr_id);
        let id = { let mut ctx = NativeContext::new(&mut vm); es.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = NativeContext::new(&mut vm); es.invoke_native_method(&mut ctx, id, 0); }
        assert_eq!(vm.param_pool.get_int_return(), 1);
    }
}
