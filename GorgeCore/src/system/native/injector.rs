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

    /// 从注入器常量定义构造注入器的标量部分（G2）
    ///
    /// 常量中的所有字段值都是显式设置的（非默认）。字段按类型分组存储，
    /// **索引对齐类声明**（`field_index_map`）：按 Int/Float/Bool/String/Object
    /// 分组独立编号、继承链父类字段在前，与运行时布局、codegen 编号及
    /// `materialize_injector` 的 JSON 物化路径完全一致——常量缺失类中某些
    /// 字段时不会造成索引错位（此前按"常量内字段顺序"编号，常量缺字段会
    /// 与声明布局错位，导致字段读写越界或错值）。
    ///
    /// 嵌套的 `InjectObject`/`Array` 字段在常量中占 object 槽位，本方法
    /// 无法在无 VM 上下文时物化（需要分配对象 ID 并注册到对象表），因此
    /// 只按 (0, 非默认) 占槽；由 VM 的 `LoadInjectorConstant` 处理器
    /// （`VirtualMachine::materialize_injector_constant`）在标量填充完成后
    /// 沿常量定义二次遍历，递归物化嵌套对象并写入 object 槽位。
    ///
    /// `class_decl` 为注入器所属类的声明（调用方负责按常量中的类名解析，
    /// 未注册类可传哑声明——此时回退按常量内字段顺序编号，保证数据不丢）。
    pub fn from_constant(
        constant: &crate::objective::bytecode::InjectorConstantDef,
        class_decl: Arc<ClassDeclaration>,
    ) -> Self {
        let index_map = Self::field_index_map(&class_decl);
        let has_decl = !index_map.is_empty();

        // 数组大小按类声明统计（含继承）；哑声明（未注册类）回退常量内统计
        let (int_count, float_count, bool_count, str_count, obj_count) = if has_decl {
            let mut counts = [0usize; 5];
            for (_, (_, vt)) in &index_map {
                match vt {
                    ValueType::Int => counts[0] += 1,
                    ValueType::Float => counts[1] += 1,
                    ValueType::Bool => counts[2] += 1,
                    ValueType::String => counts[3] += 1,
                    ValueType::Object => counts[4] += 1,
                }
            }
            (counts[0], counts[1], counts[2], counts[3], counts[4])
        } else {
            let mut counts = [0usize; 5];
            for f in &constant.fields {
                match f {
                    crate::objective::bytecode::InjectorConstField::Int(..) => counts[0] += 1,
                    crate::objective::bytecode::InjectorConstField::Float(..) => counts[1] += 1,
                    crate::objective::bytecode::InjectorConstField::Bool(..) => counts[2] += 1,
                    crate::objective::bytecode::InjectorConstField::String(..) => counts[3] += 1,
                    crate::objective::bytecode::InjectorConstField::Object(..)
                    | crate::objective::bytecode::InjectorConstField::InjectObject(..)
                    | crate::objective::bytecode::InjectorConstField::Array(..) => counts[4] += 1,
                }
            }
            (counts[0], counts[1], counts[2], counts[3], counts[4])
        };

        let mut result = Self {
            class_decl,
            int_fields: vec![(0, true); int_count],
            float_fields: vec![(0.0, true); float_count],
            bool_fields: vec![(false, true); bool_count],
            string_fields: vec![(String::new(), true); str_count],
            object_fields: vec![(0, true); obj_count],
        };

        // 常量内各类型的相对顺序（哑声明兜底索引）
        let mut seq = [0usize; 5];
        for f in &constant.fields {
            match f {
                crate::objective::bytecode::InjectorConstField::Int(name, v) => {
                    if let Some(&(idx, ValueType::Int)) = index_map.get(name) {
                        result.int_fields[idx] = (*v, false);
                    } else if !has_decl && seq[0] < result.int_fields.len() {
                        result.int_fields[seq[0]] = (*v, false);
                    }
                    seq[0] += 1;
                }
                crate::objective::bytecode::InjectorConstField::Float(name, v) => {
                    if let Some(&(idx, ValueType::Float)) = index_map.get(name) {
                        result.float_fields[idx] = (*v, false);
                    } else if !has_decl && seq[1] < result.float_fields.len() {
                        result.float_fields[seq[1]] = (*v, false);
                    }
                    seq[1] += 1;
                }
                crate::objective::bytecode::InjectorConstField::Bool(name, v) => {
                    if let Some(&(idx, ValueType::Bool)) = index_map.get(name) {
                        result.bool_fields[idx] = (*v, false);
                    } else if !has_decl && seq[2] < result.bool_fields.len() {
                        result.bool_fields[seq[2]] = (*v, false);
                    }
                    seq[2] += 1;
                }
                crate::objective::bytecode::InjectorConstField::String(name, v) => {
                    if let Some(&(idx, ValueType::String)) = index_map.get(name) {
                        result.string_fields[idx] = (v.clone(), false);
                    } else if !has_decl && seq[3] < result.string_fields.len() {
                        result.string_fields[seq[3]] = (v.clone(), false);
                    }
                    seq[3] += 1;
                }
                crate::objective::bytecode::InjectorConstField::Object(name, v) => {
                    if let Some(&(idx, ValueType::Object)) = index_map.get(name) {
                        result.object_fields[idx] = (*v, false);
                    } else if !has_decl && seq[4] < result.object_fields.len() {
                        result.object_fields[seq[4]] = (*v, false);
                    }
                    seq[4] += 1;
                }
                // 嵌套注入器和数组在常量中占 object 槽位，值由 VM 在
                // 物化阶段递归填充（本方法只按 0 占槽）。
                // 两者在常量中均无名（InjectObject 首槽位是类名而非字段名），
                // 按常量内 object 相对顺序占槽——常量字段按声明顺序输出时
                // 该顺序即声明分组索引
                crate::objective::bytecode::InjectorConstField::InjectObject(..) => {
                    if seq[4] < result.object_fields.len() {
                        result.object_fields[seq[4]] = (0, false);
                    }
                    seq[4] += 1;
                }
                crate::objective::bytecode::InjectorConstField::Array(..) => {
                    if seq[4] < result.object_fields.len() {
                        result.object_fields[seq[4]] = (0, false);
                    }
                    seq[4] += 1;
                }
            }
        }
        result
    }

    /// 构建类声明的注入器字段名 → (分组索引, 值类型) 映射。
    ///
    /// 索引按 Int/Float/Bool/String/Object 五种值类型**分组独立编号**，
    /// 且继承链父类字段在前，与运行时注入器布局（本类型分组数组）、
    /// codegen 编号（对齐 C# `ClassMemberCounter.InjectorFieldIndex`）一致。
    /// `injector_fields` 为空（哑声明）时返回空表，调用方回退常量顺序。
    pub fn field_index_map(
        class_decl: &ClassDeclaration,
    ) -> std::collections::HashMap<String, (usize, ValueType)> {
        let mut map = std::collections::HashMap::new();
        let mut int_i = 0usize;
        let mut float_i = 0usize;
        let mut bool_i = 0usize;
        let mut string_i = 0usize;
        let mut object_i = 0usize;
        for df in &class_decl.injector_fields {
            let (idx, vt) = match df.field_type.basic_type {
                crate::objective::types::BasicType::Int
                | crate::objective::types::BasicType::Enum => {
                    let i = int_i;
                    int_i += 1;
                    (i, ValueType::Int)
                }
                crate::objective::types::BasicType::Float => {
                    let i = float_i;
                    float_i += 1;
                    (i, ValueType::Float)
                }
                crate::objective::types::BasicType::Bool => {
                    let i = bool_i;
                    bool_i += 1;
                    (i, ValueType::Bool)
                }
                crate::objective::types::BasicType::String => {
                    let i = string_i;
                    string_i += 1;
                    (i, ValueType::String)
                }
                _ => {
                    let i = object_i;
                    object_i += 1;
                    (i, ValueType::Object)
                }
            };
            map.insert(df.name.clone(), (idx, vt));
        }
        map
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

    /// 构造带声明字段的类声明（模拟 DremuLane：string name + 3 个 float + object）
    fn make_lane_decl() -> Arc<ClassDeclaration> {
        use crate::objective::declaration::InjectorFieldInfo;
        use crate::objective::types::BasicType;
        let decl = ClassDeclaration {
            class_type: GorgeType::class("DremuLane", None),
            is_native: false,
            annotations: vec![],
            fields: vec![],
            methods: vec![],
            static_methods: vec![],
            constructors: vec![],
            injector_fields: vec![
                InjectorFieldInfo { name: "name".to_string(), field_type: GorgeType::new(BasicType::String), is_array: false, has_default_value: false, default_value: None },
                InjectorFieldInfo { name: "generateTime".to_string(), field_type: GorgeType::new(BasicType::Float), is_array: false, has_default_value: false, default_value: None },
                InjectorFieldInfo { name: "keepTime".to_string(), field_type: GorgeType::new(BasicType::Float), is_array: false, has_default_value: false, default_value: None },
                InjectorFieldInfo { name: "laneLines".to_string(), field_type: GorgeType::new(BasicType::Object), is_array: true, has_default_value: false, default_value: None },
                InjectorFieldInfo { name: "positionZ".to_string(), field_type: GorgeType::new(BasicType::Float), is_array: false, has_default_value: false, default_value: None },
            ],
            super_class: None,
            super_interfaces: vec![],
            field_type_count: TypeCount::zero(),
            method_count: 0,
            static_method_count: 0,
            constructor_count: 0,
            injector_field_type_count: TypeCount {
                string_count: 1,
                float_count: 3,
                object_count: 1,
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
        };
        Arc::new(decl)
    }

    /// from_constant：常量缺失类声明中的部分字段时，按类声明分组索引填充，
    /// 数组大小按类声明统计（对齐 codegen 与 materialize 的布局约定）
    #[test]
    fn test_from_constant_indexes_by_declaration() {
        use crate::objective::bytecode::{InjectorConstantDef, InjectorConstField};
        let constant = InjectorConstantDef {
            class_name: "DremuLane".to_string(),
            fields: vec![
                // 缺 generateTime（类声明中的第 0 个 float）
                InjectorConstField::String("name".to_string(), "Main1".to_string()),
                InjectorConstField::Float("keepTime".to_string(), 1.5),
                InjectorConstField::Float("positionZ".to_string(), -2.0),
            ],
        };
        let injector = RuntimeInjector::from_constant(&constant, make_lane_decl());

        // 数组大小按类声明（含未在常量出现的字段）：float 应为 3（不是常量中的 2）
        assert_eq!(injector.float_field_count(), 3);
        assert_eq!(injector.string_field_count(), 1);
        assert_eq!(injector.object_field_count(), 1);
        // name → String 组 0
        assert_eq!(injector.get_injector_string(0), "Main1");
        assert!(!injector.get_injector_string_default_value(0));
        // keepTime → Float 组 1（声明中 generateTime 在前占 0）
        assert_eq!(injector.get_injector_float(1), 1.5);
        assert!(!injector.get_injector_float_default_value(1));
        // positionZ → Float 组 2
        assert_eq!(injector.get_injector_float(2), -2.0);
        // 未出现在常量的 generateTime（Float 组 0）保持默认值
        assert!(injector.get_injector_float_default_value(0));
        assert_eq!(injector.get_injector_float(0), 0.0);
    }

    /// from_constant：哑声明（未注册类）回退按常量内字段顺序编号，数据不丢
    #[test]
    fn test_from_constant_dummy_decl_fallback() {
        use crate::objective::bytecode::{InjectorConstantDef, InjectorConstField};
        let constant = InjectorConstantDef {
            class_name: "UnknownClass".to_string(),
            fields: vec![
                InjectorConstField::Float("a".to_string(), 1.0),
                InjectorConstField::Float("b".to_string(), 2.0),
            ],
        };
        let dummy = Arc::new(ClassDeclaration::dummy("UnknownClass".to_string()));
        let injector = RuntimeInjector::from_constant(&constant, dummy);
        assert_eq!(injector.float_field_count(), 2);
        assert_eq!(injector.get_injector_float(0), 1.0);
        assert_eq!(injector.get_injector_float(1), 2.0);
    }
}
