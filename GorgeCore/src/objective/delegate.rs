use std::collections::HashMap;
use std::fmt::Debug;
use crate::virtual_machine::ir::{CompiledMethod, CodeWithSpan};
use crate::objective::types::GorgeType;
use crate::objective::value_pool::FixedFieldValuePool;

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
    /// Lambda 自身参数的类型列表（供 InvokeDelegate 构造 ParamMode::ByType 联合类型）
    pub param_types: Vec<crate::virtual_machine::ir::ValueType>,
    /// 捕获变量的值类型列表（与 captured_var_names 一一对应）
    pub captured_var_types: Vec<crate::virtual_machine::ir::ValueType>,
    /// 创建此委托的实例对象 ID（实例方法中创建委托时为 Some(this_id)，静态方法中为 None）
    /// 用于委托执行时恢复 this 指针，使捕获的字段访问能正确定位到创建时的实例
    pub creator_this: Option<usize>,
}

impl RuntimeDelegate {
    /// 从委托定义、捕获变量名/类型和外部值映射动态构造委托
    ///
    /// 捕获值按类型分组存入 FixedFieldValuePool（对齐 C# 捕获变量语义）。
    pub fn from_def(
        delegate_type: GorgeType,
        body_ir: &[CodeWithSpan],
        captured_var_names: &[String],
        captured_var_types: &[crate::virtual_machine::ir::ValueType],
        outer_values: &HashMap<String, crate::virtual_machine::ir::Operand>,
    ) -> Self {
        use crate::virtual_machine::ir::Operand;
        use crate::virtual_machine::ir::ImmediateValue;

        // 按值类型分组收集捕获变量的实际值
        let mut int_vals: Vec<i64> = Vec::new();
        let mut float_vals: Vec<f64> = Vec::new();
        let mut bool_vals: Vec<bool> = Vec::new();
        let mut string_vals: Vec<String> = Vec::new();
        let mut object_vals: Vec<usize> = Vec::new();

        for (name, vt) in captured_var_names.iter().zip(captured_var_types.iter()) {
            if let Some(op) = outer_values.get(name.as_str()) {
                match vt {
                    crate::virtual_machine::ir::ValueType::Int => {
                        match op {
                            Operand::Immediate(ImmediateValue::Int(v)) => int_vals.push(*v),
                            _ => int_vals.push(0),
                        }
                    }
                    crate::virtual_machine::ir::ValueType::Float => {
                        match op {
                            Operand::Immediate(ImmediateValue::Float(v)) => float_vals.push(*v),
                            _ => float_vals.push(0.0),
                        }
                    }
                    crate::virtual_machine::ir::ValueType::Bool => {
                        match op {
                            Operand::Immediate(ImmediateValue::Bool(v)) => bool_vals.push(*v),
                            _ => bool_vals.push(false),
                        }
                    }
                    crate::virtual_machine::ir::ValueType::String => {
                        match op {
                            Operand::Immediate(ImmediateValue::String(s)) => string_vals.push(s.clone()),
                            _ => string_vals.push(String::new()),
                        }
                    }
                    crate::virtual_machine::ir::ValueType::Object => {
                        // 对象捕获暂取 0（后续运行时解析栈地址）
                        object_vals.push(0);
                    }
                }
            } else {
                match vt {
                    crate::virtual_machine::ir::ValueType::Int => int_vals.push(0),
                    crate::virtual_machine::ir::ValueType::Float => float_vals.push(0.0),
                    crate::virtual_machine::ir::ValueType::Bool => bool_vals.push(false),
                    crate::virtual_machine::ir::ValueType::String => string_vals.push(String::new()),
                    crate::virtual_machine::ir::ValueType::Object => object_vals.push(0),
                }
            }
        }

        let mut captured = FixedFieldValuePool::default();
        captured.ints = int_vals;
        captured.floats = float_vals;
        captured.bools = bool_vals;
        captured.strings = string_vals;
        captured.objects = object_vals;

        let local_count = 16;

        RuntimeDelegate {
            delegate_type,
            method_impl: CompiledMethod {
                name: "lambda".into(),
                codes: body_ir.to_vec(),
                local_count,
            },
            captured_values: captured,
            param_types: vec![],
            captured_var_types: captured_var_types.to_vec(),
            creator_this: None,
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
    use crate::objective::types::{GorgeType, BasicType};

    #[test]
    fn test_delegate_from_def() {
        let dt = GorgeType::new(BasicType::Delegate);
        let outer = HashMap::new();
        let d = RuntimeDelegate::from_def(dt, &[], &[], &[], &outer);
        assert_eq!(d.method_impl.name, "lambda");
    }
}
