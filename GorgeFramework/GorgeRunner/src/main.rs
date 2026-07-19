use std::env;
use std::fs;
use std::collections::HashMap;
use std::sync::Arc;

use gorge_core::objective::bytecode::deserialize_module;
use gorge_core::objective::class::RuntimeClass;
use gorge_core::objective::declaration::ClassDeclaration;
use gorge_core::objective::native::{NativeClass, NativeContext};
use gorge_core::objective::types::TypeCount;
use gorge_core::virtual_machine::vm::VirtualMachine;

fn simple_name(full: &str) -> String { full.rsplit('.').next().unwrap_or(full).to_string() }

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { eprintln!("用法: gorge_runner <文件.gorge> [类.方法]"); std::process::exit(1); }
    let input_path = &args[1];
    let data = fs::read(input_path).unwrap();
    let module = deserialize_module(&data).unwrap();

    // ==================== Test8N native 类（P-4） ====================
    /// Test8N 是一个测试用的 native 类，模拟 C# 原生类与 Gorge 编译类的交互。
    /// 含 1 个 Gorge 字段（gorgeField, int index 0）和 1 个隐藏 C# 字段（cSharpField, int index 1）。
    #[derive(Debug, Clone)]
    struct Test8NClass;
    impl NativeClass for Test8NClass {
        fn full_name(&self) -> &str { "Test8N" }
        fn field_type_count(&self) -> &TypeCount {
            static TC: std::sync::OnceLock<TypeCount> = std::sync::OnceLock::new();
            TC.get_or_init(|| TypeCount { int_count: 2, ..TypeCount::zero() })
        }
        fn invoke_native_method(&self, ctx: &mut NativeContext, obj_id: usize, method_id: usize) {
            match method_id {
                0 => { // GetGorgeField()
                    let v = ctx.get_object_int_field(obj_id, 0);
                    ctx.set_int_return(v);
                }
                1 => { // SetGorgeField(int)
                    let v = ctx.get_int_param(0);
                    ctx.set_object_int_field(obj_id, 0, v);
                }
                2 => { // GetCSharpField()
                    let v = ctx.get_object_int_field(obj_id, 1);
                    ctx.set_int_return(v);
                }
                3 => { // SetCSharpField(int)
                    let v = ctx.get_int_param(0);
                    ctx.set_object_int_field(obj_id, 1, v);
                }
                _ => {}
            }
        }
        fn invoke_native_static(&self, ctx: &mut NativeContext, method_id: usize) {
            match method_id {
                0 => { ctx.set_int_return(1); } // GetConst() → 1
                1 => { ctx.set_int_return(ctx.get_int_param(0)); } // Echo(i) → i
                2 => { ctx.set_int_return(ctx.get_int_param(0) + ctx.get_int_param(1)); } // Add(a,b)
                _ => {}
            }
        }
        fn do_construct_native(&self, ctx: &mut NativeContext, target: Option<usize>, _ctor_id: usize) -> usize {
            if let Some(obj_id) = target {
                // 继承场景：在已有编译对象上设置 native 字段
                ctx.set_object_int_field(obj_id, 0, ctx.get_int_param(0));
                ctx.set_object_int_field(obj_id, 1, ctx.get_int_param(1));
                obj_id
            } else {
                // 新建场景：创建 native 对象
                let id = ctx.vm.next_object_id;
                ctx.vm.next_object_id += 1;
                let mut tc = TypeCount::zero();
                tc.int_count = 2;
                let obj = gorge_core::objective::object::RuntimeObject::new_simple("Test8N".to_string(), &tc);
                ctx.vm.objects.insert(id, obj);
                ctx.set_object_int_field(id, 0, ctx.get_int_param(0));
                ctx.set_object_int_field(id, 1, ctx.get_int_param(1));
                id
            }
        }
    }

    let mut vm = VirtualMachine::new();
    // 注册 Test8N
    vm.register_native_class("Test8N", Arc::new(Test8NClass));
    for cls in gorge_framework::native_classes() {
        vm.register_native_class(&simple_name(cls.full_name()), cls.clone());
    }

    let compiled_map: HashMap<String, &gorge_core::objective::bytecode::CompiledClass> = module.classes.iter()
        .filter(|c| !c.is_native).map(|c| (simple_name(&c.class_type.full_name()), c)).collect();
    let mut ordered: Vec<&gorge_core::objective::bytecode::CompiledClass> = module.classes.iter()
        .filter(|c| !c.is_native).collect();
    ordered.sort_by_key(|c| {
        let mut depth = 0; let mut cur = c.super_class_name.clone();
        while let Some(ref n) = cur { depth += 1; cur = compiled_map.get(&simple_name(n)).and_then(|x| x.super_class_name.clone()); }
        depth
    });

    let mut rc_map: HashMap<String, Arc<RuntimeClass>> = HashMap::new();
    for cc in ordered {
        let name = simple_name(&cc.class_type.full_name());
        let mut mp: Vec<(gorge_core::virtual_machine::ir::CompiledMethod, Vec<gorge_core::virtual_machine::ir::ValueType>)> = Vec::new();
        for m in &cc.methods { mp.push((m.clone(), vec![])); }
        vm.register_class_methods(&name, mp);
        let tf = TypeCount {
            int_count: cc.field_start_counts[0] + cc.field_counts.int_count,
            float_count: cc.field_start_counts[1] + cc.field_counts.float_count,
            bool_count: cc.field_start_counts[2] + cc.field_counts.bool_count,
            string_count: cc.field_start_counts[3] + cc.field_counts.string_count,
            object_count: cc.field_start_counts[4] + cc.field_counts.object_count,
        };
        vm.register_class_field_counts(&name, tf);
        // Phase P: 注册字段初始化器
        if !cc.field_initializers.is_empty() {
            vm.register_class_field_initializers(&name, cc.field_initializers.clone());
        }
        if let Some(ref sn) = cc.super_class_name { vm.register_class_super(&name, &simple_name(sn)); }
        let sa = cc.super_class_name.as_ref().and_then(|sn| rc_map.get(&simple_name(sn)).cloned());
        let iface_map: HashMap<String, Vec<usize>> = cc.interface_method_impl_id.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        let decl = ClassDeclaration {
            class_type: cc.class_type.clone(), is_native: false, annotations: vec![], fields: vec![],
            methods: vec![], static_methods: vec![], constructors: vec![], injector_fields: vec![],
            super_class: sa.as_ref().map(|a| Box::new(a.declaration.clone())),
            super_interfaces: cc.super_interfaces.clone(),
            field_type_count: cc.field_counts.clone(), method_count: cc.methods.len(),
            static_method_count: 0, constructor_count: cc.constructors.len(),
            injector_field_type_count: TypeCount::zero(), injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: cc.method_start_id, constructor_start_id: cc.constructor_start_id,
            interface_method_impl_id: iface_map,
            method_override_id: cc.method_override_id.iter().cloned().collect(),
            injector_constructor_impl_id: cc.injector_constructor_impl_id.clone(),
            method_annotations: cc.method_annotations.clone(),
            constructor_annotations: cc.constructor_annotations.clone(),
        };
        let mut rc = RuntimeClass::new(decl, sa);
        for (i, m) in cc.methods.iter().enumerate() { rc.register_method(i, m.clone()); }
        for (i, ct) in cc.constructors.iter().enumerate() { rc.register_constructor(i, ct.clone()); }
        let arc = Arc::new(rc);
        rc_map.insert(name.clone(), arc.clone());
        vm.register_runtime_class(&name, arc);

        // V5: 注册委托实现
        let mut cls_delegates: Vec<(gorge_core::virtual_machine::ir::CompiledMethod, Vec<gorge_core::virtual_machine::ir::ValueType>, gorge_core::virtual_machine::ir::ValueType, Vec<gorge_core::virtual_machine::ir::ValueType>)> = Vec::new();
        for delegate in &cc.delegate_impls {
            cls_delegates.push((gorge_core::virtual_machine::ir::CompiledMethod {
                name: "lambda".into(),
                codes: delegate.body_ir.clone(),
                local_count: 16,
            }, delegate.param_types.clone(), delegate.return_type, delegate.captured_var_types.clone()));
        }
        vm.register_class_delegates(&name, cls_delegates);
    }

    let (entry_cls, entry_method) = if args.len() >= 3 {
        let (cls_name, method) = args[2].rsplit_once('.').unwrap_or((&args[2], ""));
        (cls_name.to_string(), module.classes.iter()
            .find(|c| simple_name(&c.class_type.full_name()) == cls_name)
            .and_then(|c| c.methods.iter().find(|m| m.name == method)).unwrap().clone())
    } else {
        let c = module.classes.iter().find(|c| !c.is_native && !c.methods.is_empty()).unwrap();
        (simple_name(&c.class_type.full_name()), c.methods[0].clone())
    };

    vm.push_frame(entry_method.local_count);
    vm.set_current_class(&entry_cls);
    vm.execute(&entry_method).unwrap();
    if let Some(v) = vm.get_return_int() { println!("{} -> 返回 (int): {}", entry_method.name, v); }
    if let Some(v) = vm.get_return_float() { println!("{} -> 返回 (float): {}", entry_method.name, v); }
    if let Some(v) = vm.get_return_bool() { println!("{} -> 返回 (bool): {}", entry_method.name, v); }
    if let Some(v) = vm.get_return_string() { println!("{} -> 返回 (string): {}", entry_method.name, v); }
    vm.pop_frame();
}
