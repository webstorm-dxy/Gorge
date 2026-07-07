use std::fmt::Debug;
use std::sync::Arc;

use crate::class::GorgeClass;
use crate::types::TypeCount;
use crate::value_pool::FixedFieldValuePool;

/// Gorge 对象 trait
///
/// 所有运行时对象的抽象接口。每种值类型提供独立的字段读写方法，
/// 方法调用委托给所属的 GorgeClass。
pub trait GorgeObject: Debug {
    /// 获取所属类
    fn gorge_class(&self) -> &Arc<dyn GorgeClass>;

    fn get_int_field(&self, index: usize) -> i64;
    fn get_float_field(&self, index: usize) -> f64;
    fn get_bool_field(&self, index: usize) -> bool;
    fn get_string_field(&self, index: usize) -> String;
    fn get_object_field(&self, index: usize) -> usize;
    fn set_int_field(&mut self, index: usize, value: i64);
    fn set_float_field(&mut self, index: usize, value: f64);
    fn set_bool_field(&mut self, index: usize, value: bool);
    fn set_string_field(&mut self, index: usize, value: String);
    fn set_object_field(&mut self, index: usize, value: usize);
    fn invoke_method(&mut self, method_id: usize);
}

/// 编译生成的运行时对象
///
/// 字段按 Native/Compiled 分界存储。index < native_bounds 时读 native_object，
/// 否则从 compiled_fields 读取。
#[derive(Debug)]
pub struct RuntimeObject {
    pub class: Arc<dyn GorgeClass>,
    pub native_object: Option<Box<dyn GorgeObject>>,
    pub compiled_fields: FixedFieldValuePool,
    pub native_field_bounds: TypeCount,
}

impl RuntimeObject {
    pub fn new(class: Arc<dyn GorgeClass>) -> Self {
        let field_type_count = class.declaration().field_type_count.clone();
        Self {
            class,
            native_object: None,
            compiled_fields: FixedFieldValuePool::new(&field_type_count),
            native_field_bounds: TypeCount::zero(),
        }
    }
}

impl GorgeObject for RuntimeObject {
    fn gorge_class(&self) -> &Arc<dyn GorgeClass> {
        &self.class
    }

    fn get_int_field(&self, index: usize) -> i64 {
        if index < self.native_field_bounds.int_count {
            self.native_object.as_ref().unwrap().get_int_field(index)
        } else {
            self.compiled_fields.get_int(index - self.native_field_bounds.int_count)
        }
    }

    fn get_float_field(&self, index: usize) -> f64 {
        if index < self.native_field_bounds.float_count {
            self.native_object.as_ref().unwrap().get_float_field(index)
        } else {
            self.compiled_fields.get_float(index - self.native_field_bounds.float_count)
        }
    }

    fn get_bool_field(&self, index: usize) -> bool {
        if index < self.native_field_bounds.bool_count {
            self.native_object.as_ref().unwrap().get_bool_field(index)
        } else {
            self.compiled_fields.get_bool(index - self.native_field_bounds.bool_count)
        }
    }

    fn get_string_field(&self, index: usize) -> String {
        if index < self.native_field_bounds.string_count {
            self.native_object.as_ref().unwrap().get_string_field(index)
        } else {
            self.compiled_fields.get_string(index - self.native_field_bounds.string_count).to_string()
        }
    }

    fn get_object_field(&self, index: usize) -> usize {
        if index < self.native_field_bounds.object_count {
            self.native_object.as_ref().unwrap().get_object_field(index)
        } else {
            self.compiled_fields.get_object(index - self.native_field_bounds.object_count)
        }
    }

    fn set_int_field(&mut self, index: usize, value: i64) {
        if index < self.native_field_bounds.int_count {
            self.native_object.as_mut().unwrap().set_int_field(index, value);
        } else {
            self.compiled_fields.set_int(index - self.native_field_bounds.int_count, value);
        }
    }

    fn set_float_field(&mut self, index: usize, value: f64) {
        if index < self.native_field_bounds.float_count {
            self.native_object.as_mut().unwrap().set_float_field(index, value);
        } else {
            self.compiled_fields.set_float(index - self.native_field_bounds.float_count, value);
        }
    }

    fn set_bool_field(&mut self, index: usize, value: bool) {
        if index < self.native_field_bounds.bool_count {
            self.native_object.as_mut().unwrap().set_bool_field(index, value);
        } else {
            self.compiled_fields.set_bool(index - self.native_field_bounds.bool_count, value);
        }
    }

    fn set_string_field(&mut self, index: usize, value: String) {
        if index < self.native_field_bounds.string_count {
            self.native_object.as_mut().unwrap().set_string_field(index, value);
        } else {
            self.compiled_fields.set_string(index - self.native_field_bounds.string_count, value);
        }
    }

    fn set_object_field(&mut self, index: usize, value: usize) {
        if index < self.native_field_bounds.object_count {
            self.native_object.as_mut().unwrap().set_object_field(index, value);
        } else {
            self.compiled_fields.set_object(index - self.native_field_bounds.object_count, value);
        }
    }

    fn invoke_method(&mut self, method_id: usize) {
        let cls = Arc::clone(&self.class);
        cls.invoke_method(self, method_id);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_runtime_object_placeholder() {
        // RuntimeObject 依赖 Arc<dyn GorgeClass> 构造，此处作为集成测试占位
    }
}
