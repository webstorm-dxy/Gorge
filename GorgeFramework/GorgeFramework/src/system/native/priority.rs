//! `GorgeFramework.Priority` —— 优先级 native 类（委托包装）。
//!
//! 对齐 C# 参考实现。Priority 持有 GorgeDelegate 对象 ID，
//! 调用 get_value() 时 invoke 该委托获取实际优先级浮点值。

use gorge_core::objective::native::NativeContext;
use gorge_macros::{gorge_native_class, gorge_native_impl};

/// 优先级委托包装
///
/// 字段 `get_priority` 存储委托对象 ID，
/// 实例方法 `get_value` 调用委托返回 float 优先级值。
#[gorge_native_class(namespace = "GorgeFramework")]
pub struct Priority {
    /// 优先级委托对象 ID（object 槽）
    #[gorge_field]
    pub get_priority: usize,
}

#[gorge_native_impl]
impl Priority {
    #[gorge_ctor]
    pub fn new(ctx: &mut NativeContext, this: usize, get_priority: usize) {
        ctx.set_object_object_field(this, Self::FIELD_INDEX_get_priority, get_priority);
    }

    /// 实例方法 0：调用委托获取优先级值
    ///
    /// 读取自身 `get_priority` 委托 ID，经 `invoke_delegate` 执行委托，
    /// 返回 float 优先级值。委托 ID 为空时返回 0.0。
    #[gorge_method]
    pub fn get_value(ctx: &mut NativeContext, this: usize) -> f32 {
        let delegate_id = ctx.get_object_object_field(this, Self::FIELD_INDEX_get_priority);
        if delegate_id == 0 {
            return 0.0;
        }
        ctx.invoke_delegate(delegate_id);
        (ctx.get_float_return() as f64) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::objective::native::NativeClass;
    use gorge_core::objective::delegate::RuntimeDelegate;
    use gorge_core::objective::types::{GorgeType, BasicType};
    use gorge_core::objective::value_pool::FixedFieldValuePool;
    use gorge_core::virtual_machine::vm::VirtualMachine;
    use gorge_core::virtual_machine::ir::{CompiledMethod, CodeWithSpan, IntermediateCode, Operand, Address, ValueType};

    struct Fixture {
        vm: VirtualMachine,
    }

    impl Fixture {
        fn new() -> Self {
            let mut vm = VirtualMachine::new();
            vm.next_object_id = 100;
            Self { vm }
        }

        fn ctx(&mut self) -> NativeContext<'_> { NativeContext::new(&mut self.vm) }

        /// 创建返回固定 float 值的委托
        fn make_float_delegate(&mut self, value: f32) -> usize {
            let result_addr = Address::new(ValueType::Float, 0);
            let method = CompiledMethod {
                name: "const_float".into(),
                codes: vec![
                    CodeWithSpan::new(
                        IntermediateCode::assign(result_addr, Operand::float(value as f64)),
                        gorge_core::diagnostics::Span::dummy(),
                    ),
                    CodeWithSpan::new(
                        IntermediateCode::return_value(ValueType::Float),
                        gorge_core::diagnostics::Span::dummy(),
                    ),
                ],
                local_count: 1,
            };
            let delegate = RuntimeDelegate {
                delegate_type: GorgeType::new(BasicType::Delegate),
                method_impl: method,
                captured_values: FixedFieldValuePool::default(),
                param_types: vec![],
                captured_var_types: vec![],
                creator_this: None,
            };
            let id = self.vm.next_object_id;
            self.vm.next_object_id += 1;
            self.vm.runtime_delegates.insert(id, delegate);
            id
        }
    }

    #[test]
    fn test_priority_construct_with_delegate() {
        let p = Priority { get_priority: 0 };
        let mut fx = Fixture::new();
        let delegate_id = fx.make_float_delegate(42.0);
        fx.vm.param_pool.set_object_param(0, delegate_id);
        let id = { let mut ctx = fx.ctx(); p.do_construct_native(&mut ctx, None, 0) };
        assert!(id > 0);
        // 验证字段写入
        let stored = { let ctx = fx.ctx(); ctx.get_object_object_field(id, Priority::FIELD_INDEX_get_priority) };
        assert_eq!(stored, delegate_id);
    }

    #[test]
    fn test_priority_get_value_via_delegate() {
        let p = Priority { get_priority: 0 };
        let mut fx = Fixture::new();
        let delegate_id = fx.make_float_delegate(3.14);
        fx.vm.param_pool.set_object_param(0, delegate_id);
        let id = { let mut ctx = fx.ctx(); p.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = fx.ctx(); p.invoke_native_method(&mut ctx, id, 0); }
        let result = (fx.vm.param_pool.get_float_return() as f64) as f32;
        assert!((result - 3.14).abs() < 0.01, "预期 3.14，实际 {}", result);
    }

    #[test]
    fn test_priority_get_value_null_delegate_returns_zero() {
        let p = Priority { get_priority: 0 };
        let mut fx = Fixture::new();
        fx.vm.param_pool.set_object_param(0, 0); // 空委托
        let id = { let mut ctx = fx.ctx(); p.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = fx.ctx(); p.invoke_native_method(&mut ctx, id, 0); }
        let result = (fx.vm.param_pool.get_float_return() as f64) as f32;
        assert!((result - 0.0).abs() < 0.01, "空委托应返回 0.0，实际 {}", result);
    }

    #[test]
    fn test_priority_usage_regression_simulator_pattern() {
        // 模拟 simulators/impls.rs 中的使用模式：
        // invoke_native_method_on("GorgeFramework.Priority", pid, 0) 应返回 float
        let p = Priority { get_priority: 0 };
        let mut fx = Fixture::new();
        fx.vm.register_native_class(p.full_name(), std::sync::Arc::new(Priority { get_priority: 0 }));
        let delegate_id = fx.make_float_delegate(100.0);
        fx.vm.param_pool.set_object_param(0, delegate_id);
        let id = { let mut ctx = fx.ctx(); p.do_construct_native(&mut ctx, None, 0) };
        // 模拟 simulators 的调用方式
        {
            let mut ctx = fx.ctx();
            ctx.invoke_native_method_on("GorgeFramework.Priority", id, 0);
        }
        let result = (fx.vm.param_pool.get_float_return() as f64) as f32;
        assert!((result - 100.0).abs() < 0.01, "预期 100.0，实际 {}", result);
    }

    #[test]
    fn test_priority_get_value_with_small_delegate_value() {
        let p = Priority { get_priority: 0 };
        let mut fx = Fixture::new();
        let delegate_id = fx.make_float_delegate(-5.5);
        fx.vm.param_pool.set_object_param(0, delegate_id);
        let id = { let mut ctx = fx.ctx(); p.do_construct_native(&mut ctx, None, 0) };
        { let mut ctx = fx.ctx(); p.invoke_native_method(&mut ctx, id, 0); }
        let result = (fx.vm.param_pool.get_float_return() as f64) as f32;
        assert!((result - (-5.5)).abs() < 0.01, "预期 -5.5，实际 {}", result);
    }
}
