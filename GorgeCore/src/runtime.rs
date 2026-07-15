use std::collections::HashMap;
use std::sync::Arc;
use crate::class::{GorgeClass, RuntimeClass};
use crate::declaration::{EnumDef};
use crate::interface::GorgeInterface;
use crate::native::NativeClass;
use crate::types::{GorgeType, BasicType};
use crate::vm::VirtualMachine;

/// Gorge 运行时
///
/// 对应 C# 的 GorgeLanguageRuntime，是所有编译产物的注册中心和类型转换判定的最终权威。
pub struct GorgeRuntime {
    pub classes: HashMap<String, Arc<RuntimeClass>>,
    /// Native 类注册表：类全名 → NativeClass 实现
    pub native_classes: HashMap<String, Arc<dyn NativeClass>>,
    pub interfaces: HashMap<String, Arc<GorgeInterface>>,
    pub enums: HashMap<String, Arc<EnumDef>>,
    pub vm: VirtualMachine,
}

impl GorgeRuntime {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            native_classes: HashMap::new(),
            interfaces: HashMap::new(),
            enums: HashMap::new(),
            vm: VirtualMachine::new(),
        }
    }

    /// 注册一个编译好的类
    pub fn register_class(&mut self, class: RuntimeClass) -> Arc<RuntimeClass> {
        let key = class.declaration().class_type.full_name();
        let arc = Arc::new(class);
        self.classes.insert(key, arc.clone());
        arc
    }

    /// 按全名获取类
    pub fn get_class(&self, full_name: &str) -> Option<&Arc<RuntimeClass>> {
        self.classes.get(full_name)
    }

    /// 注册一个 native 类
    ///
    /// native 类与编译类共享同一命名空间的全名查找。注册后可被 VM 分派
    /// 静态方法、实例方法与构造方法。
    pub fn register_native_class(&mut self, class: Arc<dyn NativeClass>) {
        let key = class.full_name().to_string();
        self.native_classes.insert(key.clone(), class.clone());
        // 同步注册到 VM，供运行期分派
        self.vm.register_native_class(&key, class);
    }

    /// 按全名获取 native 类
    pub fn get_native_class(&self, full_name: &str) -> Option<&Arc<dyn NativeClass>> {
        self.native_classes.get(full_name)
    }

    /// 判断某全名是否为已注册的 native 类
    pub fn is_native_class(&self, full_name: &str) -> bool {
        self.native_classes.contains_key(full_name)
    }

    /// 注册接口
    pub fn register_interface(&mut self, iface: GorgeInterface) {
        self.interfaces.insert(iface.full_name.clone(), Arc::new(iface));
    }

    /// 注册枚举
    pub fn register_enum(&mut self, enum_def: EnumDef) {
        self.enums.insert(enum_def.full_name.clone(), Arc::new(enum_def));
    }

    /// 判断是否可以自动（隐式）转换
    ///
    /// 除了 TypeInfo 的基本规则外，还检查运行时类层次：
    /// - 子类 → 父类
    /// - 类 → 实现的接口
    /// - null → 任意 object/interface
    pub fn can_auto_cast_to(&self, from: &GorgeType, to: &GorgeType) -> bool {
        if from == to {
            return true;
        }
        // Int → Float
        if from.basic_type == BasicType::Int && to.basic_type == BasicType::Float {
            return true;
        }
        // Enum → Int
        if from.basic_type == BasicType::Enum && to.basic_type == BasicType::Int {
            return true;
        }
        // Null → 任意 Object / Interface / Delegate / String
        if from.is_null()
            && matches!(
                to.basic_type,
                BasicType::Object | BasicType::Interface | BasicType::Delegate | BasicType::String
            )
        {
            return true;
        }
        // object 到 object 的基础规则（通过类层次检查）
        if from.basic_type == BasicType::Object && to.basic_type == BasicType::Object {
            // 数组协变：元素类型可自动转换（sub_types[0] 为元素类型）
            if !from.sub_types.is_empty() && !to.sub_types.is_empty() {
                if self.can_auto_cast_to(&from.sub_types[0], &to.sub_types[0]) {
                    return true;
                }
            }
            return self.is_subclass_of(from, to);
        }
        // 类到接口
        if from.basic_type == BasicType::Object && to.basic_type == BasicType::Interface {
            return self.class_implements_interface(from, to);
        }
        // 接口 → Object
        if from.basic_type == BasicType::Interface && to.basic_type == BasicType::Object {
            return true;
        }
        // Delegate 协变（返回）/逆变（参数）：sub_types[0]=返回类型，其余为参数类型
        if from.basic_type == BasicType::Delegate && to.basic_type == BasicType::Delegate {
            if from.sub_types.is_empty() || to.sub_types.is_empty() {
                return false;
            }
            if from.sub_types.len() != to.sub_types.len() {
                return false;
            }
            // 返回协变
            if !self.can_auto_cast_to(&from.sub_types[0], &to.sub_types[0]) {
                return false;
            }
            // 参数逆变
            for i in 1..from.sub_types.len() {
                if !self.can_auto_cast_to(&to.sub_types[i], &from.sub_types[i]) {
                    return false;
                }
            }
            return true;
        }
        false
    }

    /// 判断是否可以强制转换
    pub fn can_cast_to(&self, from: &GorgeType, to: &GorgeType) -> bool {
        self.can_auto_cast_to(from, to) || self.can_auto_cast_to(to, from)
    }

    /// 检查 from 是否为 to 的子类
    fn is_subclass_of(&self, from: &GorgeType, to: &GorgeType) -> bool {
        let from_name = from.full_name();
        let to_name = to.full_name();
        
        let mut current = self.classes.get(&from_name);
        while let Some(cls) = current {
            let decl = cls.declaration();
            if decl.class_type.full_name() == to_name {
                return true;
            }
            // 上溯父类
            if let Some(sup) = &decl.super_class {
                let sup_name = sup.class_type.full_name();
                current = self.classes.get(&sup_name);
            } else {
                break;
            }
        }
        false
    }

    /// 检查类是否实现了某接口
    fn class_implements_interface(&self, class_type: &GorgeType, iface_type: &GorgeType) -> bool {
        let class_name = class_type.full_name();
        let iface_name = iface_type.full_name();
        
        if let Some(cls) = self.classes.get(&class_name) {
            if cls.declaration().super_interfaces.iter().any(|i| i == &iface_name) {
                return true;
            }
        }
        false
    }
}

impl Default for GorgeRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::declaration::ClassDeclaration;
    use crate::types::TypeCount;

    fn make_decl(name: &str, super_name: Option<&str>) -> ClassDeclaration {
        let mut super_class = None;
        if let Some(sn) = super_name {
            super_class = Some(Box::new(ClassDeclaration {
                class_type: GorgeType::class(sn, None),
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
                method_count: 0, static_method_count: 0, constructor_count: 0,
                injector_field_type_count: TypeCount::zero(),
                injector_field_default_value_type_count: TypeCount::zero(),
                method_start_id: 0, constructor_start_id: 0,
                interface_method_impl_id: HashMap::new(),
                method_override_id: HashMap::new(),
                injector_constructor_impl_id: vec![],
            }));
        }
        ClassDeclaration {
            class_type: GorgeType::class(name, None),
            is_native: false,
            annotations: vec![],
            fields: vec![],
            methods: vec![],
            static_methods: vec![],
            constructors: vec![],
            injector_fields: vec![],
            super_class,
            super_interfaces: vec![],
            field_type_count: TypeCount::zero(),
            method_count: 0, static_method_count: 0, constructor_count: 0,
            injector_field_type_count: TypeCount::zero(),
            injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: 0, constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
            injector_constructor_impl_id: vec![],
        }
    }

    #[test]
    fn test_register_and_get_class() {
        let mut runtime = GorgeRuntime::new();
        let decl = make_decl("MyClass", None);
        let cls = RuntimeClass::new(decl, None);
        runtime.register_class(cls);
        assert!(runtime.get_class("MyClass").is_some());
    }

    #[test]
    fn test_subclass_check() {
        let mut runtime = GorgeRuntime::new();
        let base_decl = make_decl("Base", None);
        let base = RuntimeClass::new(base_decl, None);
        runtime.register_class(base);

        let derived_decl = make_decl("Derived", Some("Base"));
        let derived = RuntimeClass::new(derived_decl, None);
        runtime.register_class(derived);

        let from = GorgeType::class("Derived", None);
        let to = GorgeType::class("Base", None);
        assert!(runtime.can_auto_cast_to(&from, &to));
    }

    #[test]
    fn test_cast_rules_e2() {
        let runtime = GorgeRuntime::new();
        // Int → Float
        assert!(runtime.can_auto_cast_to(&GorgeType::new(BasicType::Int), &GorgeType::new(BasicType::Float)));
        // Enum → Int
        assert!(runtime.can_auto_cast_to(&GorgeType::new(BasicType::Enum), &GorgeType::new(BasicType::Int)));
        // null → String
        assert!(runtime.can_auto_cast_to(&GorgeType::null(), &GorgeType::new(BasicType::String)));
        // 接口 → Object
        assert!(runtime.can_auto_cast_to(&GorgeType::new(BasicType::Interface), &GorgeType::new(BasicType::Object)));
        // Float → Int 不自动转换，但可强制转换
        assert!(!runtime.can_auto_cast_to(&GorgeType::new(BasicType::Float), &GorgeType::new(BasicType::Int)));
        assert!(runtime.can_cast_to(&GorgeType::new(BasicType::Float), &GorgeType::new(BasicType::Int)));
    }

    #[test]
    fn test_delegate_variance_e2() {
        let runtime = GorgeRuntime::new();
        // delegate<Float(Int)> vs delegate<Float(Int)> 相同 → 可转
        let mk = |ret: BasicType, param: BasicType| {
            let mut d = GorgeType::new(BasicType::Delegate);
            d.sub_types = vec![GorgeType::new(ret), GorgeType::new(param)];
            d
        };
        // 返回协变：源返回 Int 可转目标返回 Float
        let from = mk(BasicType::Int, BasicType::Float);
        let to = mk(BasicType::Float, BasicType::Float);
        assert!(runtime.can_auto_cast_to(&from, &to), "返回协变应成立");
        // 参数逆变：目标参数 Int 可转源参数 Float（源参数更宽）
        let from2 = mk(BasicType::Float, BasicType::Float);
        let to2 = mk(BasicType::Float, BasicType::Int);
        assert!(runtime.can_auto_cast_to(&from2, &to2), "参数逆变应成立");
    }
}
