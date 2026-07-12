use std::fmt::Debug;
use std::sync::Arc;

use crate::declaration::ClassDeclaration;
use crate::ir::ValueType;

/// 注入器 trait
///
/// 注入器是一种特殊的对象，提供编译时的字段值注入能力。
/// 每个字段存储 (值, 是否使用默认值) 对。
pub trait Injector: Debug {
    /// 获取注入器对应的类声明
    fn injection_class_declaration(&self) -> &ClassDeclaration;

    fn get_injector_int(&self, index: usize) -> i64;
    fn set_injector_int(&mut self, index: usize, value: i64);
    fn get_injector_float(&self, index: usize) -> f64;
    fn set_injector_float(&mut self, index: usize, value: f64);
    fn get_injector_bool(&self, index: usize) -> bool;
    fn set_injector_bool(&mut self, index: usize, value: bool);
    fn get_injector_string(&self, index: usize) -> String;
    fn set_injector_string(&mut self, index: usize, value: String);
    fn get_injector_object(&self, index: usize) -> usize;
    fn set_injector_object(&mut self, index: usize, value: usize);

    fn get_injector_int_default_value(&self, index: usize) -> bool;
    fn set_injector_int_default_value(&mut self, index: usize);
    fn get_injector_float_default_value(&self, index: usize) -> bool;
    fn set_injector_float_default_value(&mut self, index: usize);
    fn get_injector_bool_default_value(&self, index: usize) -> bool;
    fn set_injector_bool_default_value(&mut self, index: usize);
    fn get_injector_string_default_value(&self, index: usize) -> bool;
    fn set_injector_string_default_value(&mut self, index: usize);
    fn get_injector_object_default_value(&self, index: usize) -> bool;
    fn set_injector_object_default_value(&mut self, index: usize);
}

/// 编译版注入器
///
/// 每个字段存储 (值, 是否使用默认值) 的元组。
#[derive(Debug, Clone)]
pub struct RuntimeInjector {
    pub class_decl: Arc<ClassDeclaration>,
    int_fields: Vec<(i64, bool)>,
    float_fields: Vec<(f64, bool)>,
    bool_fields: Vec<(bool, bool)>,
    string_fields: Vec<(String, bool)>,
    object_fields: Vec<(usize, bool)>,
}

impl RuntimeInjector {
    pub fn new(class_decl: Arc<ClassDeclaration>) -> Self {
        Self {
            int_fields: vec![(0, true); class_decl.injector_field_type_count.int_count],
            float_fields: vec![(0.0, true); class_decl.injector_field_type_count.float_count],
            bool_fields: vec![(false, true); class_decl.injector_field_type_count.bool_count],
            string_fields: vec![(String::new(), true); class_decl.injector_field_type_count.string_count],
            object_fields: vec![(0, true); class_decl.injector_field_type_count.object_count],
            class_decl,
        }
    }

    /// 从字段定义列表动态构造注入器
    ///
    /// 所有字段初始化为默认值标记。
    pub fn from_defs(
        class_decl: Arc<ClassDeclaration>,
        defs: &[crate::bytecode::InjectorFieldDef],
    ) -> Self {
        let mut int_count = 0;
        let mut float_count = 0;
        let mut bool_count = 0;
        let mut string_count = 0;
        let mut object_count = 0;

        for def in defs {
            match def.value_type {
                ValueType::Int => int_count += 1,
                ValueType::Float => float_count += 1,
                ValueType::Bool => bool_count += 1,
                ValueType::String => string_count += 1,
                ValueType::Object => object_count += 1,
            }
        }

        let mut injector = Self {
            class_decl,
            int_fields: vec![(0, true); int_count],
            float_fields: vec![(0.0, true); float_count],
            bool_fields: vec![(false, true); bool_count],
            string_fields: vec![(String::new(), true); string_count],
            object_fields: vec![(0, true); object_count],
        };

        let mut ii = 0; let mut fi = 0; let mut bi = 0;
        let mut si = 0; let mut oi = 0;

        for def in defs {
            match def.value_type {
                ValueType::Int => {
                    if def.has_default {
                        injector.int_fields[ii].1 = true;
                    }
                    ii += 1;
                }
                ValueType::Float => {
                    if def.has_default { injector.float_fields[fi].1 = true; }
                    fi += 1;
                }
                ValueType::Bool => {
                    if def.has_default { injector.bool_fields[bi].1 = true; }
                    bi += 1;
                }
                ValueType::String => {
                    if def.has_default { injector.string_fields[si].1 = true; }
                    si += 1;
                }
                ValueType::Object => {
                    if def.has_default { injector.object_fields[oi].1 = true; }
                    oi += 1;
                }
            }
        }

        injector
    }

    /// 从注入器常量定义构造注入器（G2）
    ///
    /// 常量中的所有字段值都是显式设置的（非默认）。字段按类型分组存储，
    /// 索引基于常量定义中该类型字段出现的顺序。
    pub fn from_constant(constant: &crate::bytecode::InjectorConstantDef) -> Self {
        // 统计各类型字段数
        let mut int_count = 0; let mut float_count = 0; let mut bool_count = 0;
        let mut str_count = 0; let mut obj_count = 0;
        for f in &constant.fields {
            match f {
                crate::bytecode::InjectorConstField::Int(..) => int_count += 1,
                crate::bytecode::InjectorConstField::Float(..) => float_count += 1,
                crate::bytecode::InjectorConstField::Bool(..) => bool_count += 1,
                crate::bytecode::InjectorConstField::String(..) => str_count += 1,
                crate::bytecode::InjectorConstField::Object(..) => obj_count += 1,
            }
        }
        let mut result = Self {
            class_decl: Arc::new(crate::declaration::ClassDeclaration::dummy(constant.class_name.clone())),
            int_fields: vec![(0, true); int_count],
            float_fields: vec![(0.0, true); float_count],
            bool_fields: vec![(false, true); bool_count],
            string_fields: vec![(String::new(), true); str_count],
            object_fields: vec![(0, true); obj_count],
        };
        let mut ii = 0; let mut fi = 0; let mut bi = 0; let mut si = 0; let mut oi = 0;
        for f in &constant.fields {
            match f {
                crate::bytecode::InjectorConstField::Int(_, v) => {
                    result.int_fields[ii] = (*v, false);
                    ii += 1;
                }
                crate::bytecode::InjectorConstField::Float(_, v) => {
                    result.float_fields[fi] = (*v, false);
                    fi += 1;
                }
                crate::bytecode::InjectorConstField::Bool(_, v) => {
                    result.bool_fields[bi] = (*v, false);
                    bi += 1;
                }
                crate::bytecode::InjectorConstField::String(_, v) => {
                    result.string_fields[si] = (v.clone(), false);
                    si += 1;
                }
                crate::bytecode::InjectorConstField::Object(_, v) => {
                    result.object_fields[oi] = (*v, false);
                    oi += 1;
                }
            }
        }
        result
    }
}

impl Injector for RuntimeInjector {
    fn injection_class_declaration(&self) -> &ClassDeclaration {
        &self.class_decl
    }

    fn get_injector_int(&self, index: usize) -> i64 {
        self.int_fields[index].0
    }
    fn set_injector_int(&mut self, index: usize, value: i64) {
        self.int_fields[index].0 = value;
        self.int_fields[index].1 = false;
    }
    fn get_injector_float(&self, index: usize) -> f64 {
        self.float_fields[index].0
    }
    fn set_injector_float(&mut self, index: usize, value: f64) {
        self.float_fields[index].0 = value;
        self.float_fields[index].1 = false;
    }
    fn get_injector_bool(&self, index: usize) -> bool {
        self.bool_fields[index].0
    }
    fn set_injector_bool(&mut self, index: usize, value: bool) {
        self.bool_fields[index].0 = value;
        self.bool_fields[index].1 = false;
    }
    fn get_injector_string(&self, index: usize) -> String {
        self.string_fields[index].0.clone()
    }
    fn set_injector_string(&mut self, index: usize, value: String) {
        self.string_fields[index].0 = value;
        self.string_fields[index].1 = false;
    }
    fn get_injector_object(&self, index: usize) -> usize {
        self.object_fields[index].0
    }
    fn set_injector_object(&mut self, index: usize, value: usize) {
        self.object_fields[index].0 = value;
        self.object_fields[index].1 = false;
    }

    fn get_injector_int_default_value(&self, index: usize) -> bool {
        self.int_fields[index].1
    }
    fn set_injector_int_default_value(&mut self, index: usize) {
        self.int_fields[index].1 = true;
    }
    fn get_injector_float_default_value(&self, index: usize) -> bool {
        self.float_fields[index].1
    }
    fn set_injector_float_default_value(&mut self, index: usize) {
        self.float_fields[index].1 = true;
    }
    fn get_injector_bool_default_value(&self, index: usize) -> bool {
        self.bool_fields[index].1
    }
    fn set_injector_bool_default_value(&mut self, index: usize) {
        self.bool_fields[index].1 = true;
    }
    fn get_injector_string_default_value(&self, index: usize) -> bool {
        self.string_fields[index].1
    }
    fn set_injector_string_default_value(&mut self, index: usize) {
        self.string_fields[index].1 = true;
    }
    fn get_injector_object_default_value(&self, index: usize) -> bool {
        self.object_fields[index].1
    }
    fn set_injector_object_default_value(&mut self, index: usize) {
        self.object_fields[index].1 = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{GorgeType, TypeCount};
    use std::collections::HashMap;

    fn make_dummy_decl() -> Arc<ClassDeclaration> {
        Arc::new(ClassDeclaration {
            class_type: GorgeType::class("Test", None),
            is_native: false,
            annotations: vec![],
            fields: vec![],
            methods: vec![],
            static_methods: vec![],
            constructors: vec![],
            injector_fields: vec![],
            super_class: None,
            super_interfaces: vec![],
            field_type_count: TypeCount::zero(),
            method_count: 0,
            static_method_count: 0,
            constructor_count: 0,
            injector_field_type_count: TypeCount {
                int_count: 2,
                ..TypeCount::zero()
            },
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0,
            constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
        })
    }

    #[test]
    fn test_injector_set_and_get() {
        let decl = make_dummy_decl();
        let mut injector = RuntimeInjector::new(decl);
        injector.set_injector_int(0, 42);
        assert_eq!(injector.get_injector_int(0), 42);
        assert!(!injector.get_injector_int_default_value(0));
    }

    #[test]
    fn test_injector_default_marker() {
        let decl = make_dummy_decl();
        let mut injector = RuntimeInjector::new(decl);
        assert!(injector.get_injector_int_default_value(0));
        injector.set_injector_int(0, 99);
        assert!(!injector.get_injector_int_default_value(0));
        injector.set_injector_int_default_value(0);
        assert!(injector.get_injector_int_default_value(0));
    }
}
