mod ast;
mod codegen;
mod compiler;
mod highlight;
mod lexer;
mod optimizer;
mod parser;
mod progress;
mod symbol;

use std::env;
use std::fs;
use std::path::Path;

use compiler::Compiler;
use crate::symbol::{SymbolTable, ScopeId, SymbolEntry};
use gorge_core::bytecode::{CompiledModule, CompiledClass};
use gorge_core::types::{GorgeType, TypeCount};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("用法: gorgec <输入文件.g> [-o <输出文件.gorge>]");
        eprintln!("示例: gorgec program.g -o program.gorge");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = if args.len() >= 4 && args[2] == "-o" {
        args[3].clone()
    } else {
        // 默认输出：替换 .g 为 .gorge
        Path::new(input_path)
            .with_extension("gorge")
            .to_string_lossy()
            .into_owned()
    };

    let source = match fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("错误：无法读取文件 `{}`: {}", input_path, e);
            std::process::exit(1);
        }
    };

    // 词法分析
    let (tokens, lexer_diags) = lexer::tokenize(&source, 0);
    if !lexer_diags.is_empty() {
        eprintln!("词法错误:");
        let mut d = gorge_core::diagnostics::Diagnostics::new();
        for diag in lexer_diags {
            d.emit(diag);
        }
        let sources: Vec<&str> = vec![&source];
        eprintln!("{}", d.render(&sources));
        std::process::exit(1);
    }

    // 语法分析
    let mut parser = parser::Parser::new(tokens);
    let source_file = match parser.parse_source_file() {
        Ok(ast) => ast,
        Err(diags) => {
            eprintln!("语法错误:");
            let sources: Vec<&str> = vec![&source];
            eprintln!("{}", diags.render(&sources));
            std::process::exit(1);
        }
    };

    // 编译
    let mut compiler = Compiler::new();
    if compiler.compile(&[source_file]).is_err() {
        eprintln!("编译错误:");
        let sources: Vec<&str> = vec![&source];
        eprintln!("{}", compiler.into_diagnostics().render(&sources));
        std::process::exit(1);
    }

    // 优化 + 收集编译方法
    let mut methods: Vec<gorge_core::ir::CompiledMethod> = Vec::new();
    for compiled in &compiler.compiled_methods {
        let optimized = optimizer::IntermediateCodeOptimizer::optimize(&compiled.codes);
        methods.push(gorge_core::ir::CompiledMethod {
            name: compiled.name.clone(),
            codes: optimized,
            local_count: compiled.total_locals,
        });
    }

    // 从符号表构建类元数据
    let mut classes: Vec<CompiledClass> = Vec::new();
    collect_classes(&compiler.symbol_table, compiler.symbol_table.global_scope, &mut classes, &methods);

    // 将注入器字段附加到对应类
    if !compiler.injector_fields.is_empty() {
        let bytecode_fields: Vec<gorge_core::bytecode::InjectorFieldDef> = compiler
            .injector_fields
            .iter()
            .map(|f| gorge_core::bytecode::InjectorFieldDef {
                name: f.name.clone(),
                value_type: f.value_type,
                has_default: f.has_default,
            })
            .collect();
        for class in &mut classes {
            class.injector_fields = bytecode_fields.clone();
        }
    }

    for class in &mut classes {
        class.delegate_impls = compiler.delegate_impls.clone();
    }

    if classes.is_empty() {
        // 如果没有类声明，创建一个默认类包装所有方法
        classes.push(CompiledClass {
            class_type: GorgeType::class("Module", None),
            is_native: false,
            super_class_name: None,
            super_interfaces: vec![],
            field_counts: TypeCount::zero(),
            methods,
            constructors: vec![],
            injector_fields: vec![],
            delegate_impls: vec![],
        });
    }

    let module = CompiledModule { version: 2, classes };
    let bytecode = match gorge_core::bytecode::serialize_module(&module) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("序列化错误: {}", e);
            std::process::exit(1);
        }
    };

    match fs::write(&output_path, bytecode) {
        Ok(_) => println!("编译成功: {} -> {}", input_path, output_path),
        Err(e) => {
            eprintln!("错误：无法写入文件 `{}`: {}", output_path, e);
            std::process::exit(1);
        }
    }
}

/// 从符号表收集类元数据
fn collect_classes(
    st: &SymbolTable,
    scope_id: ScopeId,
    classes: &mut Vec<CompiledClass>,
    all_methods: &[gorge_core::ir::CompiledMethod],
) {
    let scope = &st.scopes.get(scope_id.0);

    for (_name, entry) in &scope.symbols {
        match entry {
            SymbolEntry::Class(class_id) => {
                let info = &st.classes.get(class_id.0);
                let class_methods: Vec<gorge_core::ir::CompiledMethod> = all_methods
                    .iter()
                    .filter(|m| {
                        info.methods.iter().any(|mid| {
                            let mi = st.methods.get(mid.0);
                            mi.name == m.name
                        })
                    })
                    .cloned()
                    .collect();

                // 匹配构造方法
                let class_ctors: Vec<gorge_core::ir::CompiledMethod> = all_methods
                    .iter()
                    .filter(|m| m.name == "constructor")
                    .filter(|_m| {
                        info.constructors.iter().any(|cid| {
                            let ci = st.constructors.get(cid.0);
                            *class_id == ci.class_id
                        })
                    })
                    .cloned()
                    .collect();

                let mut field_counts = TypeCount::zero();
                for fid in &info.fields {
                    let fi = st.fields.get(fid.0);
                    match &fi.field_type {
                        crate::symbol::TypeInfo::Int => field_counts.int_count += 1,
                        crate::symbol::TypeInfo::Float => field_counts.float_count += 1,
                        crate::symbol::TypeInfo::Bool => field_counts.bool_count += 1,
                        crate::symbol::TypeInfo::String => field_counts.string_count += 1,
                        _ => field_counts.object_count += 1,
                    }
                }

                classes.push(CompiledClass {
                    class_type: GorgeType::class(info.name.clone(), None),
                    is_native: info.is_native,
                    super_class_name: info.super_class.map(|sid| {
                        st.classes.get(sid.0).name.clone()
                    }),
                    super_interfaces: info.super_interfaces.iter().map(|iid| {
                        st.interfaces.get(iid.0).name.clone()
                    }).collect(),
                    field_counts,
                    methods: class_methods,
                    constructors: class_ctors,
                    injector_fields: vec![],
                    delegate_impls: vec![],
                });

                let class_scope = info.scope_id;
                collect_classes(st, class_scope, classes, all_methods);
            }
            SymbolEntry::Namespace(ns_id) => {
                let ns_info = st.namespaces.get(ns_id.0);
                collect_classes(st, ns_info.scope_id, classes, all_methods);
            }
            _ => {}
        }
    }

    for child_id in &scope.children {
        collect_classes(st, *child_id, classes, all_methods);
    }
}
