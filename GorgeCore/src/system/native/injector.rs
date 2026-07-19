use std::fmt::Debug;
use std::sync::Arc;

use crate::objective::declaration::ClassDeclaration;
use crate::virtual_machine::ir::ValueType;

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
        defs: &[crate::objective::bytecode::InjectorFieldDef],
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
    pub fn from_constant(constant: &crate::objective::bytecode::InjectorConstantDef) -> Self {
        // 统计各类型字段数
        let mut int_count = 0; let mut float_count = 0; let mut bool_count = 0;
        let mut str_count = 0; let mut obj_count = 0;
        for f in &constant.fields {
            match f {
                crate::objective::bytecode::InjectorConstField::Int(..) => int_count += 1,
                crate::objective::bytecode::InjectorConstField::Float(..) => float_count += 1,
                crate::objective::bytecode::InjectorConstField::Bool(..) => bool_count += 1,
                crate::objective::bytecode::InjectorConstField::String(..) => str_count += 1,
                crate::objective::bytecode::InjectorConstField::Object(..) => obj_count += 1,
                crate::objective::bytecode::InjectorConstField::InjectObject(..) => obj_count += 1,
                crate::objective::bytecode::InjectorConstField::Array(..) => obj_count += 1,
            }
        }
        let mut result = Self {
            class_decl: Arc::new(crate::objective::declaration::ClassDeclaration::dummy(constant.class_name.clone())),
            int_fields: vec![(0, true); int_count],
            float_fields: vec![(0.0, true); float_count],
            bool_fields: vec![(false, true); bool_count],
            string_fields: vec![(String::new(), true); str_count],
            object_fields: vec![(0, true); obj_count],
        };
        let mut ii = 0; let mut fi = 0; let mut bi = 0; let mut si = 0; let mut oi = 0;
        for f in &constant.fields {
            match f {
                crate::objective::bytecode::InjectorConstField::Int(_, v) => {
                    result.int_fields[ii] = (*v, false);
                    ii += 1;
                }
                crate::objective::bytecode::InjectorConstField::Float(_, v) => {
                    result.float_fields[fi] = (*v, false);
                    fi += 1;
                }
                crate::objective::bytecode::InjectorConstField::Bool(_, v) => {
                    result.bool_fields[bi] = (*v, false);
                    bi += 1;
                }
                crate::objective::bytecode::InjectorConstField::String(_, v) => {
                    result.string_fields[si] = (v.clone(), false);
                    si += 1;
                }
                crate::objective::bytecode::InjectorConstField::Object(_, v) => {
                    result.object_fields[oi] = (*v, false);
                    oi += 1;
                }
                // 嵌套注入器和数组在常量中占 object 槽位，值由运行时填充
                crate::objective::bytecode::InjectorConstField::InjectObject(..) => {
                    result.object_fields[oi] = (0, false);
                    oi += 1;
                }
                crate::objective::bytecode::InjectorConstField::Array(..) => {
                    result.object_fields[oi] = (0, false);
                    oi += 1;
                }
            }
        }
        result
    }

    /// 注入器所属类的简单名（用于编辑期比较判定同类）。
    pub fn class_name(&self) -> String {
        self.class_decl.class_type.name()
    }

    /// object 类型字段的数量（供 VM 递归比较/哈希遍历）。
    pub fn object_field_count(&self) -> usize {
        self.object_fields.len()
    }

    /// int/float/bool/string 类型字段的数量（供 VM 递归哈希遍历）。
    pub fn int_field_count(&self) -> usize { self.int_fields.len() }
    pub fn float_field_count(&self) -> usize { self.float_fields.len() }
    pub fn bool_field_count(&self) -> usize { self.bool_fields.len() }
    pub fn string_field_count(&self) -> usize { self.string_fields.len() }

    /// 编辑期比较：仅比较非 object 字段（int/float/bool/string）。
    ///
    /// 对齐 C# `Injector.EditableEquals` 中值类型字段的判定逻辑：
    /// - 类名不同直接不相等；
    /// - 每个字段先比较「是否使用默认值」标记，两者都为默认则视为相等（跳过值比较），
    ///   否则比较实际值。
    ///
    /// **注意**：object 类型字段的比较需要对象图上下文（可能嵌套注入器/列表），
    /// 由 VM 层的 `editable_equals_objects` 递归完成；本方法只负责值类型字段，
    /// object 字段的完整比较请调用 VM 层入口。
    pub fn editable_equals_values(&self, other: &RuntimeInjector) -> bool {
        if self.class_name() != other.class_name() {
            return false;
        }
        if self.int_fields.len() != other.int_fields.len()
            || self.float_fields.len() != other.float_fields.len()
            || self.bool_fields.len() != other.bool_fields.len()
            || self.string_fields.len() != other.string_fields.len()
            || self.object_fields.len() != other.object_fields.len()
        {
            return false;
        }
        for (a, b) in self.int_fields.iter().zip(other.int_fields.iter()) {
            if a.1 != b.1 { return false; }
            if !a.1 && a.0 != b.0 { return false; }
        }
        for (a, b) in self.float_fields.iter().zip(other.float_fields.iter()) {
            if a.1 != b.1 { return false; }
            if !a.1 && a.0 != b.0 { return false; }
        }
        for (a, b) in self.bool_fields.iter().zip(other.bool_fields.iter()) {
            if a.1 != b.1 { return false; }
            if !a.1 && a.0 != b.0 { return false; }
        }
        for (a, b) in self.string_fields.iter().zip(other.string_fields.iter()) {
            if a.1 != b.1 { return false; }
            if !a.1 && a.0 != b.0 { return false; }
        }
        true
    }

    /// 读取第 `index` 个 object 字段的 (对象ID, 是否默认值)。
    pub fn object_field(&self, index: usize) -> (usize, bool) {
        self.object_fields[index]
    }

    /// 将本注入器的所有字段值（含默认值标记）拷贝到目标注入器。
    ///
    /// 对齐 C# `CompiledInjector.Clone` 的字段复制语义：值类型逐个复制，
    /// object 字段复制对象 ID（浅拷贝引用，与 C# 对 object 槽位的处理一致）。
    /// 仅复制两者共有范围内的字段（按各类型取较小长度），避免越界。
    pub fn clone_to(&self, target: &mut RuntimeInjector) {
        let n = self.int_fields.len().min(target.int_fields.len());
        target.int_fields[..n].clone_from_slice(&self.int_fields[..n]);
        let n = self.float_fields.len().min(target.float_fields.len());
        target.float_fields[..n].clone_from_slice(&self.float_fields[..n]);
        let n = self.bool_fields.len().min(target.bool_fields.len());
        target.bool_fields[..n].clone_from_slice(&self.bool_fields[..n]);
        let n = self.string_fields.len().min(target.string_fields.len());
        target.string_fields[..n].clone_from_slice(&self.string_fields[..n]);
        let n = self.object_fields.len().min(target.object_fields.len());
        target.object_fields[..n].clone_from_slice(&self.object_fields[..n]);
    }

    /// 将本注入器的值类型字段（int/float/bool/string）混入哈希器。
    ///
    /// 对齐 C# `Injector.EditableHashCode`：默认值字段混入固定标记 `true`，
    /// 否则混入实际值。object 字段的哈希需要对象图上下文，由 VM 层
    /// `editable_hash_code_object` 递归完成。
    pub fn hash_values<H: std::hash::Hasher>(&self, state: &mut H) {
        use std::hash::Hash;
        self.class_name().hash(state);
        for (v, is_def) in &self.int_fields {
            if *is_def { true.hash(state); } else { v.hash(state); }
        }
        for (v, is_def) in &self.float_fields {
            if *is_def { true.hash(state); } else { v.to_bits().hash(state); }
        }
        for (v, is_def) in &self.bool_fields {
            if *is_def { true.hash(state); } else { v.hash(state); }
        }
        for (v, is_def) in &self.string_fields {
            if *is_def { true.hash(state); } else { v.hash(state); }
        }
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
    use crate::objective::types::{GorgeType, TypeCount};
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
            injector_constructor_impl_id: vec![],
            method_annotations: HashMap::new(),
            constructor_annotations: HashMap::new(),
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

    #[test]
    fn test_editable_equals_values_same() {
        // 两个全默认的同类注入器应相等
        let a = RuntimeInjector::new(make_dummy_decl());
        let b = RuntimeInjector::new(make_dummy_decl());
        assert!(a.editable_equals_values(&b));
    }

    #[test]
    fn test_editable_equals_values_default_marker_diff() {
        // 一个字段被显式赋值（default-marker 不同）→ 不相等
        let a = RuntimeInjector::new(make_dummy_decl());
        let mut b = RuntimeInjector::new(make_dummy_decl());
        b.set_injector_int(0, 5);
        assert!(!a.editable_equals_values(&b));
    }

    #[test]
    fn test_editable_equals_values_value_diff() {
        // 两个都显式赋值但值不同 → 不相等
        let mut a = RuntimeInjector::new(make_dummy_decl());
        let mut b = RuntimeInjector::new(make_dummy_decl());
        a.set_injector_int(0, 1);
        b.set_injector_int(0, 2);
        assert!(!a.editable_equals_values(&b));
        // 值相同 → 相等
        b.set_injector_int(0, 1);
        assert!(a.editable_equals_values(&b));
    }

    #[test]
    fn test_editable_hash_code_equal_injectors_same_hash() {
        use std::hash::{DefaultHasher, Hasher};
        let mut a = RuntimeInjector::new(make_dummy_decl());
        let mut b = RuntimeInjector::new(make_dummy_decl());
        a.set_injector_int(0, 7);
        b.set_injector_int(0, 7);
        let mut ha = DefaultHasher::new();
        let mut hb = DefaultHasher::new();
        a.hash_values(&mut ha);
        b.hash_values(&mut hb);
        assert_eq!(ha.finish(), hb.finish());
    }

    #[test]
    fn test_clone_to_copies_fields_and_markers() {
        let mut src = RuntimeInjector::new(make_dummy_decl());
        src.set_injector_int(0, 42); // 字段0 显式赋值
        // 字段1 保持默认
        let mut dst = RuntimeInjector::new(make_dummy_decl());
        src.clone_to(&mut dst);
        assert_eq!(dst.get_injector_int(0), 42);
        assert!(!dst.get_injector_int_default_value(0)); // 复制了非默认标记
        assert!(dst.get_injector_int_default_value(1));  // 字段1 仍为默认
    }
}
