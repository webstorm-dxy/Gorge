use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use crate::declaration::ClassDeclaration;
use crate::ir::CompiledMethod;
use crate::object::GorgeObject;
use crate::value_pool::FixedFieldValuePool;

/// Gorge 类的 trait
///
/// 负责方法分发、构造方法调用、Injector 默认值管理。
pub trait GorgeClass: Debug + Send + Sync {
    /// 获取类声明元数据
    fn declaration(&self) -> &ClassDeclaration;
    /// 获取父类
    fn super_class(&self) -> Option<&Arc<dyn GorgeClass>>;

    /// 调用实例方法
    fn invoke_method(&self, obj: &mut dyn GorgeObject, method_id: usize);
    /// 调用静态方法
    fn invoke_static_method(&self, method_id: usize);
    /// 调用构造方法，返回新对象
    fn invoke_constructor(&self, ctor_id: usize) -> Box<dyn GorgeObject>;
    /// 在已有对象上执行构造
    fn do_construct(&self, target: &mut dyn GorgeObject, ctor_id: usize);

    /// Injector 整数字段默认值
    fn get_injector_int_default(&self, _index: usize) -> i64 { 0 }
    fn get_injector_float_default(&self, _index: usize) -> f64 { 0.0 }
    fn get_injector_bool_default(&self, _index: usize) -> bool { false }
    fn get_injector_string_default(&self, _index: usize) -> String { String::new() }
    fn get_injector_object_default(&self, _index: usize) -> usize { 0 }
}

/// 编译生成的运行时类
///
/// 对应 C# 的 CompiledGorgeClass，持有方法/构造方法的编译实现（IR 字节码）。
#[derive(Debug, Clone)]
pub struct RuntimeClass {
    pub declaration: ClassDeclaration,
    pub super_class: Option<Arc<RuntimeClass>>,
    /// 实例方法实现映射：method_id → CompiledMethod
    pub method_impls: HashMap<usize, CompiledMethod>,
    /// 静态方法实现映射
    pub static_method_impls: HashMap<usize, CompiledMethod>,
    /// 构造方法实现映射
    pub constructor_impls: HashMap<usize, CompiledMethod>,
    /// Injector 字段默认值池
    pub injector_defaults: FixedFieldValuePool,
}

impl RuntimeClass {
    pub fn new(declaration: ClassDeclaration, super_class: Option<Arc<RuntimeClass>>) -> Self {
        let defaults = FixedFieldValuePool::new(&declaration.injector_field_default_value_type_count);
        Self {
            declaration,
            super_class,
            method_impls: HashMap::new(),
            static_method_impls: HashMap::new(),
            constructor_impls: HashMap::new(),
            injector_defaults: defaults,
        }
    }

    /// 注册方法实现
    pub fn register_method(&mut self, method_id: usize, code: CompiledMethod) {
        self.method_impls.insert(method_id, code);
    }

    /// 按局部索引查找实例方法（含继承链上溯和重写映射）
    ///
    /// 对应 C# CompiledGorgeClass.InvokeMethod 的查找逻辑。
    /// method_id 是全局方法 ID（相对于本类声明的 method_start_id）。
    pub fn find_method(&self, method_id: usize) -> Option<CompiledMethod> {
        // 1. 检查 method_id 是否在本类声明范围内
        let start = self.declaration.method_start_id;
        let end = start + self.declaration.method_count;
        if method_id >= start && method_id < end {
            let local_idx = method_id - start;
            return self.method_impls.get(&local_idx).cloned();
        }
        // 2. 检查本类是否重写了该父类方法
        if let Some(&real_id) = self.declaration.method_override_id.get(&method_id) {
            let local_idx = real_id - start;
            return self.method_impls.get(&local_idx).cloned();
        }
        // 3. 向上委托给父类
        if let Some(super_cls) = &self.super_class {
            return super_cls.find_method(method_id);
        }
        None
    }

    /// 注册构造方法实现
    pub fn register_constructor(&mut self, ctor_id: usize, code: CompiledMethod) {
        self.constructor_impls.insert(ctor_id, code);
    }

    /// 按局部索引查找构造方法（含继承链上溯）
    pub fn find_constructor(&self, ctor_id: usize) -> Option<CompiledMethod> {
        let start = self.declaration.constructor_start_id;
        let end = start + self.declaration.constructor_count;
        if ctor_id >= start && ctor_id < end {
            let local_idx = ctor_id - start;
            return self.constructor_impls.get(&local_idx).cloned();
        }
        if let Some(super_cls) = &self.super_class {
            return super_cls.find_constructor(ctor_id);
        }
        None
    }
}

impl GorgeClass for RuntimeClass {
    fn declaration(&self) -> &ClassDeclaration {
        &self.declaration
    }

    fn super_class(&self) -> Option<&Arc<dyn GorgeClass>> {
        // RuntimeClass 不是 dyn GorgeClass，需要转换。简化返回 None。
        None
    }

    fn invoke_method(&self, _obj: &mut dyn GorgeObject, _method_id: usize) {
        // 查找方法实现并执行
    }

    fn invoke_static_method(&self, _method_id: usize) {
        // 查找静态方法实现并执行
    }

    fn invoke_constructor(&self, _ctor_id: usize) -> Box<dyn GorgeObject> {
        // 创建对象 + 执行构造方法
        Box::new(crate::object::RuntimeObject::new(Arc::new(self.clone())))
    }

    fn do_construct(&self, _target: &mut dyn GorgeObject, _ctor_id: usize) {
        // 在已有对象上执行构造初始化
    }

    fn get_injector_int_default(&self, index: usize) -> i64 {
        self.injector_defaults.get_int(index)
    }

    fn get_injector_float_default(&self, index: usize) -> f64 {
        self.injector_defaults.get_float(index)
    }

    fn get_injector_bool_default(&self, index: usize) -> bool {
        self.injector_defaults.get_bool(index)
    }

    fn get_injector_string_default(&self, index: usize) -> String {
        self.injector_defaults.get_string(index).to_string()
    }

    fn get_injector_object_default(&self, index: usize) -> usize {
        self.injector_defaults.get_object(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{GorgeType, TypeCount};
    use std::collections::HashMap;

    fn make_dummy_class() -> RuntimeClass {
        RuntimeClass::new(
            ClassDeclaration {
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
                injector_field_type_count: TypeCount::zero(),
                injector_field_default_value_type_count: TypeCount::zero(),
                method_start_id: 0,
                constructor_start_id: 0,
                interface_method_impl_id: HashMap::new(),
                method_override_id: HashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_runtime_class_construction() {
        let cls = make_dummy_class();
        assert_eq!(cls.declaration().class_type.full_name(), "Test");
    }

    #[test]
    fn test_register_method() {
        let mut cls = make_dummy_class();
        let method = CompiledMethod {
            name: "test".into(),
            codes: vec![],
            local_count: 0,
        };
        cls.register_method(0, method);
        assert!(cls.method_impls.contains_key(&0));
    }

    #[test]
    fn test_find_method_local() {
        let mut cls = make_dummy_class();
        cls.declaration.method_count = 2;
        let m = CompiledMethod { name: "foo".into(), codes: vec![], local_count: 1 };
        cls.register_method(0, m.clone());
        let found = cls.find_method(0);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "foo");
    }

    #[test]
    fn test_find_method_super_class() {
        let mut parent = make_dummy_class();
        parent.declaration.method_count = 1;
        let pm = CompiledMethod { name: "parentMethod".into(), codes: vec![], local_count: 1 };
        parent.register_method(0, pm);
        // 子类的 method_start_id = 1（跳过父类的 0）
        let mut child = RuntimeClass::new(
            ClassDeclaration {
                class_type: GorgeType::class("Child", None),
                method_start_id: 1,
                method_count: 1,
                ..parent.declaration.clone()
            },
            Some(Arc::new(parent)),
        );
        let cm = CompiledMethod { name: "childMethod".into(), codes: vec![], local_count: 1 };
        child.register_method(0, cm);
        // 子类方法（局部 0 → 全局 1）
        assert!(child.find_method(1).is_some());
        // 父类方法（全局 0），子类未重写 → 上溯
        assert!(child.find_method(0).is_some());
        assert_eq!(child.find_method(0).unwrap().name, "parentMethod");
        // 不存在的全局 ID
        assert!(child.find_method(99).is_none());
    }
}
