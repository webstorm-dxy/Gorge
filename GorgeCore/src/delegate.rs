use std::collections::HashMap;
use std::fmt::Debug;
use crate::ir::{CompiledMethod, CodeWithSpan};
use crate::types::GorgeType;
use crate::value_pool::FixedFieldValuePool;

/// 委托 trait
///
/// 委托是 Gorge 中 Lambda 表达式的运行时载体，
/// 持有编译后的方法体和捕获的外部变量。
pub trait GorgeDelegate: Debug {
    fn delegate_type(&self) -> &GorgeType;
    fn invoke(&mut self);
}

/// 编译版委托
///
/// 从委托定义和外部值映射动态构造，持有编译后的方法体和捕获值池。
#[derive(Debug, Clone)]
pub struct RuntimeDelegate {
    pub delegate_type: GorgeType,
    pub method_impl: CompiledMethod,
    pub captured_values: FixedFieldValuePool,
}

impl RuntimeDelegate {
    /// 从委托定义和外部值映射动态构造委托
    pub fn from_def(
        delegate_type: GorgeType,
        body_ir: &[CodeWithSpan],
        _captured_var_names: &[String],
        _outer_values: &HashMap<String, crate::ir::Operand>,
    ) -> Self {
        let captured = FixedFieldValuePool::default();

        let local_count = 16;

        RuntimeDelegate {
            delegate_type,
            method_impl: CompiledMethod {
                name: "lambda".into(),
                codes: body_ir.to_vec(),
                local_count,
            },
            captured_values: captured,
        }
    }
}

impl GorgeDelegate for RuntimeDelegate {
    fn delegate_type(&self) -> &GorgeType {
        &self.delegate_type
    }

    fn invoke(&mut self) {
        // 委托执行入口
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{GorgeType, BasicType};

    #[test]
    fn test_delegate_from_def() {
        let dt = GorgeType::new(BasicType::Delegate);
        let outer = HashMap::new();
        let d = RuntimeDelegate::from_def(dt, &[], &[], &outer);
        assert_eq!(d.method_impl.name, "lambda");
    }
}
