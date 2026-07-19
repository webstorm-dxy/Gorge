use std::env;
use std::fs;

use gorge_core::virtual_machine::ir::ValueType;
use gorge_core::virtual_machine::vm::VirtualMachine;
use gorge_core::objective::runtime::GorgeRuntime;
use gorge_core::objective::bytecode::InjectorConstField;
use gorge_core::virtual_machine::ir::CompiledMethod;
use gorge_core::objective::class::RuntimeClass;
use gorge_core::objective::declaration::ClassDeclaration;
use gorge_core::objective::types::TypeCount;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("用法: gorge <输入文件.gorge>");
        eprintln!("示例: gorge program.gorge");
        std::process::exit(1);
    }

    let input_path = &args[1];

    // 读取字节码文件
    let data = match fs::read(input_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("错误：无法读取文件 `{}`: {}", input_path, e);
            std::process::exit(1);
        }
    };

    // 反序列化为模块
    let module = match gorge_core::objective::bytecode::deserialize_module(&data) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("错误：无效的字节码文件: {}", e);
            std::process::exit(1);
        }
    };

    if module.classes.is_empty() {
        eprintln!("警告：字节码文件中没有类");
        return;
    }

    // 构建运行时并注册类
    let mut runtime = GorgeRuntime::new();
    for compiled_class in &module.classes {
        // G4: 统计各类型注入器默认值数量
        let mut default_type_count = TypeCount::zero();
        for f in &compiled_class.injector_fields {
            if f.default_value.is_some() {
                match f.value_type {
                    ValueType::Int => default_type_count.int_count += 1,
                    ValueType::Float => default_type_count.float_count += 1,
                    ValueType::Bool => default_type_count.bool_count += 1,
                    ValueType::String => default_type_count.string_count += 1,
                    ValueType::Object => default_type_count.object_count += 1,
                }
            }
        }

        let decl = ClassDeclaration {
            class_type: compiled_class.class_type.clone(),
            is_native: compiled_class.is_native,
            annotations: vec![],
            fields: vec![],
            methods: vec![],
            static_methods: vec![],
            constructors: vec![],
            injector_fields: vec![],
            super_class: None,
            super_interfaces: compiled_class.super_interfaces.clone(),
            field_type_count: compiled_class.field_counts.clone(),
            method_count: compiled_class.methods.len(),
            static_method_count: 0,
            constructor_count: compiled_class.constructors.len(),
            injector_field_type_count: TypeCount::zero(),
            injector_field_default_value_type_count: default_type_count.clone(),
            method_start_id: 0,
            constructor_start_id: 0,
            interface_method_impl_id: HashMap::new(),
            method_override_id: HashMap::new(),
            injector_constructor_impl_id: vec![],
            method_annotations: compiled_class.method_annotations.clone(),
            constructor_annotations: compiled_class.constructor_annotations.clone(),
        };

        let mut cls = RuntimeClass::new(decl, None);
        for (i, method) in compiled_class.methods.iter().enumerate() {
            cls.register_method(i, method.clone());
        }
        for (i, ctor) in compiled_class.constructors.iter().enumerate() {
            cls.register_constructor(i, ctor.clone());
        }

        // G4: 将注入器字段默认值写入 RuntimeClass.injector_defaults
        for f in &compiled_class.injector_fields {
            if let Some(ref dv) = f.default_value {
                // 计算当前字段在同类型默认值中的偏移
                let mut idx = 0;
                for fd in &compiled_class.injector_fields {
                    if fd.name == f.name { break; }
                    if fd.default_value.is_some() && fd.value_type == f.value_type { idx += 1; }
                }
                match (f.value_type, dv) {
                    (ValueType::Int, InjectorConstField::Int(_, v)) => cls.injector_defaults.set_int(idx, *v),
                    (ValueType::Float, InjectorConstField::Float(_, v)) => cls.injector_defaults.set_float(idx, *v),
                    (ValueType::Bool, InjectorConstField::Bool(_, v)) => cls.injector_defaults.set_bool(idx, *v),
                    (ValueType::String, InjectorConstField::String(_, v)) => cls.injector_defaults.set_string(idx, v.clone()),
                    _ => {}
                }
            }
        }

        let full_name = compiled_class.class_type.full_name();
        runtime.register_class(cls);
        println!("加载类: {}", full_name);

        // 组装注入器
        if !compiled_class.injector_fields.is_empty() {
            use std::sync::Arc;
            let injector_decl = ClassDeclaration {
                class_type: compiled_class.class_type.clone(),
                is_native: compiled_class.is_native,
                annotations: vec![],
                fields: vec![],
                methods: vec![],
                static_methods: vec![],
                constructors: vec![],
                injector_fields: vec![],
                super_class: None,
                super_interfaces: compiled_class.super_interfaces.clone(),
                field_type_count: compiled_class.field_counts.clone(),
                method_count: compiled_class.methods.len(),
                static_method_count: 0,
                constructor_count: 0,
                injector_field_type_count: TypeCount::zero(),
                injector_field_default_value_type_count: TypeCount::zero(),
                method_start_id: 0,
                constructor_start_id: 0,
                interface_method_impl_id: HashMap::new(),
                method_override_id: HashMap::new(),
                injector_constructor_impl_id: vec![],
                method_annotations: compiled_class.method_annotations.clone(),
                constructor_annotations: compiled_class.constructor_annotations.clone(),
            };
            let arc_decl = Arc::new(injector_decl);
            let _injector = gorge_core::system::native::injector::RuntimeInjector::from_defs(
                arc_decl,
                &compiled_class.injector_fields,
            );
            println!("组装注入器: {} 个字段", compiled_class.injector_fields.len());
        }
    }

    // 执行所有方法（含构造方法）
    for compiled_class in &module.classes {
        let class_name = compiled_class.class_type.full_name();

        // 注册类静态方法表到 VM（供 InvokeStatic 查找）
        let mut method_params: Vec<(CompiledMethod, Vec<gorge_core::virtual_machine::ir::ValueType>)> = Vec::new();
        for method in &compiled_class.methods {
            method_params.push((method.clone(), vec![]));
        }

        // 收集所有需执行的方法：普通方法 + 构造方法
        let mut all_to_run: Vec<&CompiledMethod> = compiled_class.methods.iter().collect();
        all_to_run.extend(compiled_class.constructors.iter());

        for method in all_to_run {
            let mut vm = VirtualMachine::new();

            vm.register_class_methods(&class_name, method_params.clone());
            vm.register_class_field_counts(&class_name, compiled_class.field_counts.clone());
            // Phase P: 注册字段初始化器
            if !compiled_class.field_initializers.is_empty() {
                vm.register_class_field_initializers(&class_name, compiled_class.field_initializers.clone());
            }

            // 将运行时类注册到 VM（供 InvokeInstance 方法分派）
            if let Some(runtime_cls) = runtime.classes.get(&class_name) {
                vm.register_runtime_class(&class_name, runtime_cls.clone());
            }

            // 注册当前类的委托实现到 VM（V5: 含捕获变量类型）
            let mut cls_delegates: Vec<(CompiledMethod, Vec<gorge_core::virtual_machine::ir::ValueType>, gorge_core::virtual_machine::ir::ValueType, Vec<gorge_core::virtual_machine::ir::ValueType>)> = Vec::new();
            for delegate in &compiled_class.delegate_impls {
                cls_delegates.push((CompiledMethod {
                    name: "lambda".into(),
                    codes: delegate.body_ir.clone(),
                    local_count: 16,
                }, delegate.param_types.clone(), delegate.return_type, delegate.captured_var_types.clone()));
            }
            vm.register_class_delegates(&class_name, cls_delegates);

            vm.push_frame(method.local_count);
            vm.set_current_class(&class_name);
            match vm.execute(method) {
                Ok(()) => {
                    if let Some(v) = vm.get_return_int() {
                        println!("{} -> 返回 (int): {}", method.name, v);
                    }
                }
                Err(e) => {
                    eprintln!("运行时错误: {}", e);
                }
            }
            vm.pop_frame();
        }
    }

    println!("执行完成");
}
