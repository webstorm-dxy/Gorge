//! Gorge 编译器库
//!
//! 提供 Gorge 语言的编译功能，支持将 `.g` 源码编译为字节码模块。

pub mod compile_context;
pub mod compiler;
pub mod frontend;
pub mod highlighting;
pub mod optimizer;
pub mod progress_merger;
pub mod visitors;

use compiler::Compiler;
use crate::compile_context::symbol::{SymbolTable, ScopeId, SymbolEntry, ClassId, TypeInfo};
use crate::progress_merger::progress::ConsolePercentageReporter;
use frontend::ast::SourceFile;
use gorge_core::diagnostics::Diagnostics;
use gorge_core::objective::bytecode::{CompiledModule, CompiledClass};
use gorge_core::objective::types::{GorgeType, TypeCount};

/// 编译多个源文件，返回编译后的模块
///
/// # 参数
/// * `sources` - Gorge 源文件列表
/// * `show_progress` - 是否显示编译进度
///
/// # 返回
/// * `Ok(CompiledModule)` - 编译成功，返回包含所有类的模块
/// * `Err(Diagnostics)` - 编译失败，返回诊断信息
pub fn compile_sources(sources: &[SourceFile], show_progress: bool) -> Result<CompiledModule, Diagnostics> {
    let mut compiler = Compiler::new();
    if show_progress {
        compiler.progress_reporter = Box::new(ConsolePercentageReporter);
    }

    if compiler.compile(sources).is_err() {
        return Err(compiler.into_diagnostics());
    }

    // 优化 + 收集编译方法（保留所属类 ID 与是否构造，供精确归属）
    let mut methods: Vec<gorge_core::virtual_machine::ir::CompiledMethod> = Vec::new();
    let mut method_meta: Vec<(Option<ClassId>, bool)> = Vec::new();
    for compiled in &compiler.compiled_methods {
        let optimized = crate::optimizer::optimizer::IntermediateCodeOptimizer::optimize(&compiled.codes);
        if std::env::var("GORGE_DUMP_IR").is_ok() {
            eprintln!("=== {} ===", compiled.name);
            for (i, c) in optimized.iter().enumerate() {
                eprintln!("  {:3}: {:?} L={:?} R={:?} => {:?}", i, c.code.operator, c.code.left, c.code.right, c.code.result);
            }
        }
        methods.push(gorge_core::virtual_machine::ir::CompiledMethod {
            name: compiled.name.clone(),
            codes: optimized,
            local_count: compiled.total_locals,
        });
        method_meta.push((compiled.class_id, compiled.is_constructor));
    }

    // 从符号表构建类元数据
    let mut classes: Vec<CompiledClass> = Vec::new();
    collect_classes(&compiler.symbol_table, compiler.symbol_table.global_scope, &mut classes, &methods, &method_meta);

    // 将注入器字段按类名分发给对应的 CompiledClass
    for class in &mut classes {
        let key = &class.class_type.name();
        if let Some(fields) = compiler.injector_fields.get(key) {
            class.injector_fields = fields.iter().map(|f| gorge_core::objective::bytecode::InjectorFieldDef {
                name: f.name.clone(), value_type: f.value_type, has_default: f.has_default,
                default_value: f.default_value.clone(),
            }).collect();
        }
    }

    // 附加注入器常量池（G2）
    for class in &mut classes {
        let key = &class.class_type.full_name();
        if let Some(ics) = compiler.injector_constants.get(key) {
            class.injector_constants = ics.clone();
        }
    }

    // B-5: 按类分发注入器构造方法实现映射
    for class in &mut classes {
        let key = &class.class_type.name();
        if let Some(impl_ids) = compiler.injector_constructor_impl_id.get(key) {
            class.injector_constructor_impl_id = impl_ids.clone();
        }
    }

    // I-D: 按类分发委托实现
    for class in &mut classes {
        let key = &class.class_type.name();
        if let Some(&(start, end)) = compiler.class_delegate_ranges.get(key) {
            class.delegate_impls = compiler.delegate_impls[start..end].to_vec();
        }
    }

    // S3b: 将隐藏方法按全局 ID 插入每个类的 methods 列表
    for class in &mut classes {
        let key = &class.class_type.name();
        if let Some(hidden) = compiler.hidden_methods.get(key) {
            let mut sorted: Vec<&(usize, compiler::CompiledMethodContents)> = hidden.iter().collect();
            sorted.sort_by_key(|(gid, _)| *gid);
            for (gid, contents) in &sorted {
                let optimized = crate::optimizer::optimizer::IntermediateCodeOptimizer::optimize(&contents.codes);
                // 确保 methods 容量足够容纳隐藏方法（按全局 ID 定位可能需要扩充）
                while class.methods.len() <= *gid - class.method_start_id {
                    class.methods.push(gorge_core::virtual_machine::ir::CompiledMethod {
                        name: String::new(), codes: vec![], local_count: 0,
                    });
                }
                class.methods.push(gorge_core::virtual_machine::ir::CompiledMethod {
                    name: contents.name.clone(),
                    codes: optimized,
                    local_count: contents.total_locals,
                });
            }
        }
    }

    // Phase P: 按类分发字段初始化器
    for class in &mut classes {
        let key = &class.class_type.name();
        if let Some(initials) = compiler.field_initializers.get(key) {
            class.field_initializers = initials.clone();
        }
    }

    // Phase Q3: 按类分发注解
    for class in &mut classes {
        let key = &class.class_type.name();
        if let Some(anns) = compiler.class_annotations.get(key) {
            class.annotations = anns.clone();
        }
    }

    // S3: 按类分发方法注解和构造方法注解
    for class in &mut classes {
        let key = &class.class_type.name();
        if let Some(anns) = compiler.method_annotations.get(key) {
            class.method_annotations = anns.clone();
        }
        if let Some(anns) = compiler.constructor_annotations.get(key) {
            class.constructor_annotations = anns.clone();
        }
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
            method_start_id: 0,
            method_count_total: 0,
            constructor_start_id: 0,
            method_override_id: vec![],
            field_start_counts: [0; 5],
            interface_method_impl_id: vec![],
            injector_constants: vec![],
            injector_constructor_impl_id: vec![],
            field_initializers: vec![],
            annotations: vec![],
            method_annotations: std::collections::HashMap::new(),
            constructor_annotations: std::collections::HashMap::new(),
        });
    }

    Ok(CompiledModule { version: 5, classes })
}

/// 从符号表收集类元数据
fn collect_classes(
    st: &SymbolTable,
    scope_id: ScopeId,
    classes: &mut Vec<CompiledClass>,
    all_methods: &[gorge_core::virtual_machine::ir::CompiledMethod],
    method_meta: &[(Option<ClassId>, bool)],
) {
    let scope = &st.scopes.get(scope_id.0);

    for (_name, entry) in &scope.symbols {
        match entry {
            SymbolEntry::Class(class_id) => {
                let info = &st.classes.get(class_id.0);
                // 按声明顺序收集本类的编译方法实现。
                // all_methods 中属于本类的非构造方法按生成顺序（= 声明顺序）排列，
                // 与 info.methods 的声明顺序一致，故按顺序一一对应（正确处理同名重载）。
                let class_methods: Vec<gorge_core::virtual_machine::ir::CompiledMethod> = all_methods
                    .iter()
                    .zip(method_meta.iter())
                    .filter(|(_m, (cid, is_ctor))| !*is_ctor && *cid == Some(*class_id))
                    .map(|(m, _)| m.clone())
                    .collect();

                // 匹配构造方法（按 class_id 精确归属）
                let class_ctors: Vec<gorge_core::virtual_machine::ir::CompiledMethod> = all_methods
                    .iter()
                    .zip(method_meta.iter())
                    .filter(|(_m, (cid, is_ctor))| *is_ctor && *cid == Some(*class_id))
                    .map(|(m, _)| m.clone())
                    .collect();

                let mut field_counts = TypeCount::zero();
                for fid in &info.fields {
                    let fi = st.fields.get(fid.0);
                    match &fi.field_type {
                        TypeInfo::Int => field_counts.int_count += 1,
                        TypeInfo::Float => field_counts.float_count += 1,
                        TypeInfo::Bool => field_counts.bool_count += 1,
                        TypeInfo::String => field_counts.string_count += 1,
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
                    // 继承编号冻结（B-3）
                    method_start_id: info.method_start_id,
                    method_count_total: info.method_count_total,
                    constructor_start_id: info.constructor_start_id,
                    method_override_id: info.method_override_id.iter().map(|(k, v)| (*k, *v)).collect(),
                    field_start_counts: [
                        info.field_start_type_count.int,
                        info.field_start_type_count.float,
                        info.field_start_type_count.bool,
                        info.field_start_type_count.string,
                        info.field_start_type_count.object,
                    ],
                    interface_method_impl_id: info.interface_method_impl_id
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                    injector_constants: vec![],
                    injector_constructor_impl_id: vec![],
                    field_initializers: vec![],
                    annotations: vec![],
                    method_annotations: std::collections::HashMap::new(),
                    constructor_annotations: std::collections::HashMap::new(),
                });

                let class_scope = info.scope_id;
                collect_classes(st, class_scope, classes, all_methods, method_meta);
            }
            SymbolEntry::Namespace(ns_id) => {
                let ns_info = st.namespaces.get(ns_id.0);
                collect_classes(st, ns_info.scope_id, classes, all_methods, method_meta);
            }
            _ => {}
        }
    }

    for child_id in &scope.children {
        collect_classes(st, *child_id, classes, all_methods, method_meta);
    }
}
