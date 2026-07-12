use std::env;
use std::fs;
use std::collections::HashMap;
use std::sync::Arc;

use gorge_core::bytecode::deserialize_module;
use gorge_core::class::RuntimeClass;
use gorge_core::declaration::ClassDeclaration;
use gorge_core::types::TypeCount;
use gorge_core::vm::VirtualMachine;

fn simple_name(full: &str) -> String { full.rsplit('.').next().unwrap_or(full).to_string() }

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { eprintln!("用法: gorge_runner <文件.gorge> [类.方法]"); std::process::exit(1); }
    let input_path = &args[1];
    let data = fs::read(input_path).unwrap();
    let module = deserialize_module(&data).unwrap();

    let mut vm = VirtualMachine::new();
    for cls in gorge_framework::native_classes() {
        vm.register_native_class(&simple_name(cls.full_name()), cls.clone());
    }

    let compiled_map: HashMap<String, &gorge_core::bytecode::CompiledClass> = module.classes.iter()
        .filter(|c| !c.is_native).map(|c| (simple_name(&c.class_type.full_name()), c)).collect();
    let mut ordered: Vec<&gorge_core::bytecode::CompiledClass> = module.classes.iter()
        .filter(|c| !c.is_native).collect();
    ordered.sort_by_key(|c| {
        let mut depth = 0; let mut cur = c.super_class_name.clone();
        while let Some(n) = cur { depth += 1; cur = compiled_map.get(&simple_name(&n)).and_then(|x| x.super_class_name.clone()); }
        depth
    });

    let mut rc_map: HashMap<String, Arc<RuntimeClass>> = HashMap::new();
    for cc in ordered {
        let name = simple_name(&cc.class_type.full_name());
        let mut mp = Vec::new();
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
        if let Some(sn) = &cc.super_class_name { vm.register_class_super(&name, &simple_name(sn)); }
        let sa = cc.super_class_name.as_ref().and_then(|sn| rc_map.get(&simple_name(sn)).cloned());
        let decl = ClassDeclaration {
            class_type: cc.class_type.clone(), is_native: false, annotations: vec![], fields: vec![],
            methods: vec![], static_methods: vec![], constructors: vec![], injector_fields: vec![],
            super_class: sa.as_ref().map(|a| Box::new(a.declaration.clone())),
            super_interfaces: cc.super_interfaces.clone(),
            field_type_count: cc.field_counts.clone(), method_count: cc.methods.len(),
            static_method_count: 0, constructor_count: cc.constructors.len(),
            injector_field_type_count: TypeCount::zero(), injector_field_default_value_type_count: TypeCount::zero(),
            method_start_id: cc.method_start_id, constructor_start_id: cc.constructor_start_id,
            interface_method_impl_id: cc.interface_method_impl_id.iter().map(|(k,v)| (k.clone(),v.clone())).collect(),
            method_override_id: cc.method_override_id.iter().cloned().collect(),
        };
        let mut rc = RuntimeClass::new(decl, sa);
        for (i, m) in cc.methods.iter().enumerate() { rc.register_method(i, m.clone()); }
        for (i, ct) in cc.constructors.iter().enumerate() { rc.register_constructor(i, ct.clone()); }
        let arc = Arc::new(rc);
        rc_map.insert(name.clone(), arc.clone());
        vm.register_runtime_class(&name, arc);
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
    vm.pop_frame();
}
