#![allow(dead_code)]

use gorge_core::diagnostics::{Diagnostics, Span};
use gorge_core::ir::{CodeWithSpan, ValueType};
use gorge_core::bytecode::DelegateImpl;

use crate::ast::*;
use crate::codegen::CodeGenerator;
use crate::progress::{CompileProgress, ProgressReporter, SilentReporter};
use crate::symbol::*;

/// 注入器字段定义（序列化到字节码）
#[derive(Debug, Clone)]
pub struct InjectorFieldDef {
    pub name: String,
    pub value_type: ValueType,
    pub has_default: bool,
}

/// 字段偏移计数器（按值类型分组）
///
/// 运行时 `FixedFieldValuePool` 按 int/float/bool/string/object 五个独立数组
/// 分离存储字段值，因此字段的 offset 应为该字段在其所属值类型分组内的下标。
/// 每种类型的计数器从 0 开始独立递增。
#[derive(Debug)]
struct FieldOffsetCounters {
    int_offset: usize,
    float_offset: usize,
    bool_offset: usize,
    string_offset: usize,
    object_offset: usize,
}

impl FieldOffsetCounters {
    fn new() -> Self {
        Self {
            int_offset: 0,
            float_offset: 0,
            bool_offset: 0,
            string_offset: 0,
            object_offset: 0,
        }
    }

    /// 获取当前值类型对应的偏移并递增该类型计数器
    fn next(&mut self, vt: ValueType) -> usize {
        let counter = match vt {
            ValueType::Int => &mut self.int_offset,
            ValueType::Float => &mut self.float_offset,
            ValueType::Bool => &mut self.bool_offset,
            ValueType::String => &mut self.string_offset,
            ValueType::Object => &mut self.object_offset,
        };
        let current = *counter;
        *counter += 1;
        current
    }
}

/// 编译任务
///
/// Pass 3 产出的编译任务列表，每个任务代表一个需要在 Pass 4（代码生成）
/// 中生成中间代码的方法体、构造方法体或字段初始化器。
#[derive(Debug, Clone)]
pub struct CompileTask {
    pub kind: TaskKind,
    pub span: Span,
}

/// 编译任务类型
#[derive(Debug, Clone)]
pub enum TaskKind {
    /// 方法体编译
    Method { method_id: MethodId },
    /// 构造方法体编译
    Constructor { constructor_id: ConstructorId },
    /// 字段初始化器编译
    FieldInitializer { field_id: FieldId, class_id: ClassId },
}

/// 编译管线编排器
///
/// 负责将输入源文件通过多个 Pass 逐步编译，每个 Pass
/// 填充和丰富符号表。参考 C# 版本的四轮 Pass 设计。
pub struct Compiler {
    pub symbol_table: SymbolTable,
    pub diagnostics: Diagnostics,

    /// 当前命名空间 ID（用于 Pass 1 中将类型注册到正确的命名空间）
    current_namespace_id: Option<NamespaceId>,

    /// Pass 3 产出的编译任务列表
    pub tasks: Vec<CompileTask>,

    /// Pass 4 产出的编译方法
    pub compiled_methods: Vec<CompiledMethodContents>,

    /// 注入器字段定义列表（Pass 3 收集）
    pub injector_fields: Vec<InjectorFieldDef>,
    /// 注入器常量池（G2）：编译时求值的注入器字面量，按类组织
    pub injector_constants: std::collections::HashMap<String, Vec<gorge_core::bytecode::InjectorConstantDef>>,

    /// 委托实现列表（Pass 4 代码生成时收集）
    pub delegate_impls: Vec<DelegateImpl>,

    /// 进度报告器
    pub progress_reporter: Box<dyn ProgressReporter>,
}

/// 编译后的方法内容（简化）
#[derive(Debug, Clone)]
pub struct CompiledMethodContents {
    pub name: String,
    pub codes: Vec<CodeWithSpan>,
    pub total_locals: usize,
    /// 所属类 ID（用于把编译方法精确归属到类，避免同名方法跨类错配）
    pub class_id: Option<ClassId>,
    /// 是否为构造方法
    pub is_constructor: bool,
}

impl Compiler {
    /// 创建一个新的编译器实例
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            diagnostics: Diagnostics::new(),
            current_namespace_id: None,
            tasks: Vec::new(),
            compiled_methods: Vec::new(),
            injector_fields: Vec::new(),
            injector_constants: std::collections::HashMap::new(),
            delegate_impls: Vec::new(),
            progress_reporter: Box::new(SilentReporter),
        }
    }

    /// 主编译入口
    ///
    /// 按顺序执行所有编译 Pass。
    pub fn compile(&mut self, sources: &[SourceFile]) -> Result<(), ()> {
        let total = 4;
        self.report(1, total, "一轮编译：收集类型标识符");
        self.pass1_type_identifier(sources)?;

        self.report(2, total, "二轮编译：扩展类型信息");
        self.pass2_type_extension(sources)?;

        if self.diagnostics.has_errors() {
            return Err(());
        }

        self.report(3, total, "三轮编译：声明类型成员");
        self.pass3_type_declaration(sources)?;

        if self.diagnostics.has_errors() {
            return Err(());
        }

        // 继承编号冻结（B-3）：为每个类计算含继承的方法/构造/字段编号
        self.freeze_inheritance();

        self.report(4, total, "四轮编译：生成中间代码");
        self.pass4_code_generation(sources)?;

        Ok(())
    }

    fn report(&self, step: usize, total: usize, description: &str) {
        self.progress_reporter.report(&CompileProgress {
            current_step: step,
            total_steps: total,
            description: description.to_string(),
        });
    }

    /// 获取编译过程中收集的诊断信息
    pub fn into_diagnostics(self) -> Diagnostics {
        self.diagnostics
    }

    // ==================== Pass 1: 类型标识收集 ====================

    /// Pass 1 — 收集类型标识符
    ///
    /// 遍历所有源文件的 AST，建立命名空间/类/接口/枚举/注解的符号表机构。
    /// 此阶段**不**解析继承关系或类型成员体。
    fn pass1_type_identifier(&mut self, sources: &[SourceFile]) -> Result<(), ()> {
        for source in sources {
            self.pass1_process_source(source);
        }
        if self.diagnostics.has_errors() {
            Err(())
        } else {
            Ok(())
        }
    }

    /// 处理单个源文件的 Pass 1
    fn pass1_process_source(&mut self, source: &SourceFile) {
        // 确定当前命名空间作用域
        let scope = if let Some(ref ns_name) = source.namespace {
            // 查找或创建命名空间
            let ns_id = self.lookup_or_create_namespace(ns_name);
            self.current_namespace_id = Some(ns_id);
            self.symbol_table.namespaces.get(ns_id.0).scope_id
        } else {
            self.current_namespace_id = None;
            self.symbol_table.global_scope
        };

        // 将每个顶层成员声明到符号表中
        for member in &source.members {
            match member {
                TopLevelMember::Class(class_decl) => {
                    self.pass1_declare_class(scope, class_decl);
                }
                TopLevelMember::Interface(iface_decl) => {
                    self.pass1_declare_interface(scope, iface_decl);
                }
                TopLevelMember::Enum(enum_decl) => {
                    self.pass1_declare_enum(scope, enum_decl);
                }
            }
        }
    }

    /// 在命名空间链中查找或逐级创建命名空间
    ///
    /// 如 `System.Collections.Generic` 会依次创建 System、Collections、Generic 三级命名空间。
    fn lookup_or_create_namespace(&mut self, name: &QualifiedName) -> NamespaceId {
        let mut current_scope = self.symbol_table.global_scope;
        let mut ns_id = None;

        for part in &name.parts {
            // 在当前作用域中查找子命名空间
            let found = match self.symbol_table.lookup_local(current_scope, part) {
                Some(SymbolEntry::Namespace(id)) => Some(*id),
                _ => None,
            };

            ns_id = Some(match found {
                Some(id) => {
                    current_scope = self.symbol_table.namespaces.get(id.0).scope_id;
                    id
                }
                None => {
                    let new_id = self.symbol_table.declare_namespace(part, current_scope);
                    current_scope = self.symbol_table.namespaces.get(new_id.0).scope_id;
                    new_id
                }
            });
        }

        ns_id.unwrap()
    }

    /// Pass 1：将类声明注册到符号表
    fn pass1_declare_class(&mut self, scope: ScopeId, decl: &ClassDeclaration) {
        let is_native = decl.modifiers.iter().any(|m| matches!(m, Modifier::Native));
        let class_id = self.symbol_table.declare_class(
            &decl.name,
            scope,
            None,       // 父类在 Pass 2 中解析
            vec![],     // 接口在 Pass 2 中解析
            is_native,
            decl.span,
        );

        // 将类注册到所属命名空间
        if let Some(ns_id) = self.current_namespace_id {
            self.symbol_table.namespaces.get_mut(ns_id.0).classes.push(class_id);
        }

        // 设置修饰符
        let class_info = self.symbol_table.classes.get_mut(class_id.0);
        class_info.is_static = decl.modifiers.iter().any(|m| matches!(m, Modifier::Static));
        class_info.is_abstract = decl.modifiers.iter().any(|m| matches!(m, Modifier::Abstract));
    }

    /// Pass 1：将接口声明注册到符号表
    fn pass1_declare_interface(&mut self, scope: ScopeId, decl: &InterfaceDeclaration) {
        let iface_id = self.symbol_table.declare_interface(
            &decl.name,
            scope,
            vec![],     // 父接口在 Pass 2 中解析
            decl.span,
        );

        if let Some(ns_id) = self.current_namespace_id {
            self.symbol_table.namespaces.get_mut(ns_id.0).interfaces.push(iface_id);
        }
    }

    /// Pass 1：将枚举声明注册到符号表
    fn pass1_declare_enum(&mut self, scope: ScopeId, decl: &EnumDeclaration) {
        let enum_id = self.symbol_table.declare_enum(&decl.name, scope, decl.span);

        if let Some(ns_id) = self.current_namespace_id {
            self.symbol_table.namespaces.get_mut(ns_id.0).enums.push(enum_id);
        }
    }

    // ==================== Pass 2: 类型扩展 ====================

    /// Pass 2 — 类型扩展
    ///
    /// 建立继承关系（superclass / superInterfaces），
    /// 声明枚举值，解析 using 命名空间引用。
    fn pass2_type_extension(&mut self, sources: &[SourceFile]) -> Result<(), ()> {
        for source in sources {
            self.pass2_process_source(source);
        }
        if self.diagnostics.has_errors() {
            Err(())
        } else {
            Ok(())
        }
    }

    /// 处理单个源文件的 Pass 2
    fn pass2_process_source(&mut self, source: &SourceFile) {
        // 确定作用域
        let search_scope = if let Some(ref ns_name) = source.namespace {
            // 用与 Pass 1 相同的方式获取命名空间作用域
            self.lookup_namespace_scope(ns_name)
        } else {
            self.symbol_table.global_scope
        };

        // 解析 using 指令中的命名空间引用
        for using_directive in &source.usings {
            let usings_name = &using_directive.name;
            let mut current_scope = self.symbol_table.global_scope;
            let mut found = true;

            for part in &usings_name.parts {
                match self.symbol_table.lookup_local_only(current_scope, part) {
                    Some(SymbolEntry::Namespace(ns_id)) => {
                        current_scope = self.symbol_table.namespaces.get(ns_id.0).scope_id;
                    }
                    _ => {
                        self.diagnostics.emit_error(
                            usings_name.span,
                            format!("未找到命名空间 `{}`", part),
                        );
                        found = false;
                        break;
                    }
                }
            }

            if found {
                self.add_using_to_members(search_scope, current_scope, &source.members);
            }
        }

        for member in &source.members {
            match member {
                TopLevelMember::Class(class_decl) => {
                    self.pass2_resolve_class_hierarchy(search_scope, class_decl);
                }
                TopLevelMember::Interface(iface_decl) => {
                    self.pass2_resolve_interface_hierarchy(search_scope, iface_decl);
                }
                TopLevelMember::Enum(enum_decl) => {
                    self.pass2_resolve_enum_values(search_scope, enum_decl);
                }
            }
        }
    }

    /// 查找命名空间的作用域
    fn lookup_namespace_scope(&self, name: &QualifiedName) -> ScopeId {
        let mut current_scope = self.symbol_table.global_scope;
        for part in &name.parts {
            match self.symbol_table.lookup_local(current_scope, part) {
                Some(SymbolEntry::Namespace(ns_id)) => {
                    current_scope = self.symbol_table.namespaces.get(ns_id.0).scope_id;
                }
                _ => break,
            }
        }
        current_scope
    }

    /// Pass 2：解析类的继承关系
    fn pass2_resolve_class_hierarchy(&mut self, scope: ScopeId, decl: &ClassDeclaration) {
        // 查找类 ID
        let class_id = match self.symbol_table.lookup_class(scope, &decl.name) {
            Some(id) => id,
            None => {
                self.diagnostics.emit_error(
                    decl.span,
                    format!("未找到类 `{}` 的声明", decl.name),
                );
                return;
            }
        };

        // 解析父类
        if let Some(ref super_type) = decl.super_class {
            match self.resolve_type_or_diagnose(scope, super_type) {
                Ok(Some(TypeInfo::Object(super_id))) => {
                    self.symbol_table.set_super_class(class_id, super_id);
                }
                Ok(_) => {
                    self.diagnostics.emit_error(
                        super_type.span(),
                        "父类必须是具体的类类型",
                    );
                }
                Err(msg) => {
                    self.diagnostics.emit_error(super_type.span(), msg);
                }
            }
        }

        // 解析实现的接口
        let mut interfaces = Vec::new();
        for iface_type in &decl.super_interfaces {
            match self.resolve_type_or_diagnose(scope, iface_type) {
                Ok(Some(TypeInfo::Interface(iface_id))) => {
                    interfaces.push(iface_id);
                }
                Ok(_) => {
                    self.diagnostics.emit_error(
                        iface_type.span(),
                        "`implements` 后必须是接口类型",
                    );
                }
                Err(msg) => {
                    self.diagnostics.emit_error(iface_type.span(), msg);
                }
            }
        }
        if !interfaces.is_empty() {
            self.symbol_table.set_super_interfaces(class_id, interfaces);
        }
    }

    /// Pass 2：解析接口的父接口继承关系
    fn pass2_resolve_interface_hierarchy(&mut self, scope: ScopeId, decl: &InterfaceDeclaration) {
        let iface_id = match self.symbol_table.lookup_interface(scope, &decl.name) {
            Some(id) => id,
            None => {
                self.diagnostics.emit_error(
                    decl.span,
                    format!("未找到接口 `{}` 的声明", decl.name),
                );
                return;
            }
        };

        // 解析父接口
        let mut super_ifaces = Vec::new();
        for super_type in &decl.super_interfaces {
            match self.resolve_type_or_diagnose(scope, super_type) {
                Ok(Some(TypeInfo::Interface(super_id))) => {
                    super_ifaces.push(super_id);
                }
                Ok(_) => {
                    self.diagnostics.emit_error(
                        super_type.span(),
                        "`extends` 后必须是接口类型",
                    );
                }
                Err(msg) => {
                    self.diagnostics.emit_error(super_type.span(), msg);
                }
            }
        }
        if !super_ifaces.is_empty() {
            self.symbol_table.interfaces.get_mut(iface_id.0).super_interfaces = super_ifaces;
        }
    }

    /// Pass 2：解析枚举值
    fn pass2_resolve_enum_values(&mut self, scope: ScopeId, decl: &EnumDeclaration) {
        // 按名称查找枚举 ID
        let enum_id = self.find_enum_by_name(scope, &decl.name);

        let enum_id = match enum_id {
            Some(id) => id,
            None => {
                self.diagnostics.emit_error(
                    decl.span,
                    format!("未找到枚举 `{}` 的声明", decl.name),
                );
                return;
            }
        };

        for value in &decl.values {
            self.symbol_table.declare_enum_value(
                &value.name,
                enum_id,
                value.value,
                value.span,
            );
        }
    }

    /// 按名称查找枚举 ID
    fn find_enum_by_name(&self, scope: ScopeId, name: &str) -> Option<EnumId> {
        self.symbol_table.find_enum_by_name(scope, name)
    }

    /// 将 using 作用域添加到源代码中所有顶层类型成员的 using_scopes
    fn add_using_to_members(
        &mut self,
        search_scope: ScopeId,
        using_scope: ScopeId,
        members: &[TopLevelMember],
    ) {
        for member in members {
            match member {
                TopLevelMember::Class(decl) => {
                    if let Some(class_id) = self.symbol_table.lookup_class(search_scope, &decl.name) {
                        let class_scope = self.symbol_table.classes.get(class_id.0).scope_id;
                        if !self.symbol_table.scopes.get(class_scope.0).using_scopes.contains(&using_scope) {
                            self.symbol_table.scopes.get_mut(class_scope.0).using_scopes.push(using_scope);
                        }
                    }
                }
                TopLevelMember::Interface(decl) => {
                    if let Some(iface_id) = self.symbol_table.lookup_interface(search_scope, &decl.name) {
                        let iface_scope = self.symbol_table.interfaces.get(iface_id.0).scope_id;
                        if !self.symbol_table.scopes.get(iface_scope.0).using_scopes.contains(&using_scope) {
                            self.symbol_table.scopes.get_mut(iface_scope.0).using_scopes.push(using_scope);
                        }
                    }
                }
                TopLevelMember::Enum(decl) => {
                    if let Some(enum_id) = self.symbol_table.find_enum_by_name(search_scope, &decl.name) {
                        let enum_scope = self.symbol_table.enums.get(enum_id.0).scope_id;
                        if !self.symbol_table.scopes.get(enum_scope.0).using_scopes.contains(&using_scope) {
                            self.symbol_table.scopes.get_mut(enum_scope.0).using_scopes.push(using_scope);
                        }
                    }
                }
            }
        }
    }

    /// 解析类型引用并诊断错误
    fn resolve_type_or_diagnose(
        &self,
        scope: ScopeId,
        type_ref: &TypeRef,
    ) -> Result<Option<TypeInfo>, String> {
        match self.symbol_table.resolve_type(scope, type_ref) {
            Some(ti) => Ok(Some(ti)),
            None => {
                let name = type_ref_name(type_ref);
                Err(format!("未找到类型 `{}`", name))
            }
        }
    }

    // ==================== Pass 3: 类型声明 ====================

    /// Pass 3 — 类型声明
    ///
    /// 遍历每个类/接口的成员声明，将字段、方法、构造方法注册到符号表。
    /// 同时为 Pass 4 产出编译任务列表。
    fn pass3_type_declaration(&mut self, sources: &[SourceFile]) -> Result<(), ()> {
        for source in sources {
            self.pass3_process_source(source);
        }
        if self.diagnostics.has_errors() {
            Err(())
        } else {
            Ok(())
        }
    }

    /// 处理单个源文件的 Pass 3
    fn pass3_process_source(&mut self, source: &SourceFile) {
        let search_scope = if let Some(ref ns_name) = source.namespace {
            self.lookup_namespace_scope(ns_name)
        } else {
            self.symbol_table.global_scope
        };

        for member in &source.members {
            match member {
                TopLevelMember::Class(class_decl) => {
                    self.pass3_declare_class_members(search_scope, class_decl);
                }
                TopLevelMember::Interface(iface_decl) => {
                    self.pass3_declare_interface_members(search_scope, iface_decl);
                }
                _ => {}
            }
        }
    }

    /// Pass 3：声明类的字段、方法、构造方法
    fn pass3_declare_class_members(&mut self, scope: ScopeId, decl: &ClassDeclaration) {
        let class_id = match self.symbol_table.lookup_class(scope, &decl.name) {
            Some(id) => id,
            None => return,
        };
        let class_scope = self.symbol_table.classes.get(class_id.0).scope_id;

        // 字段偏移按值类型分组计数，每种类型从 0 开始独立递增
        // 与运行时 FixedFieldValuePool 按类型分离存储一致
        let mut counters = FieldOffsetCounters::new();

        for member in &decl.members {
            match member {
                ClassMember::Field(field_decl) => {
                    self.pass3_declare_field(class_id, class_scope, field_decl, &mut counters);
                }
                ClassMember::Method(method_decl) => {
                    self.pass3_declare_method(class_id, class_scope, method_decl);
                }
                ClassMember::Constructor(ctor_decl) => {
                    self.pass3_declare_constructor(class_id, class_scope, ctor_decl);
                }
            }
        }

        // 从 @Inject 注解自动派生注入器字段（G4）
        for member in &decl.members {
            if let ClassMember::Field(field_decl) = member {
                for annotation in &field_decl.annotations {
                    if annotation.name == "Inject" {
                        // 注入器字段名：优先注解参数 name，否则用字段本身名
                        let inj_name = field_decl.name.clone();
                        let vt = match &annotation.generic_type {
                            Some(tr) => type_ref_to_value_type(tr),
                            None => type_ref_to_value_type(&field_decl.field_type),
                        };
                        let has_default = annotation.metadatas.iter().any(|m| m.name == "defaultValue");
                        self.injector_fields.push(InjectorFieldDef { name: inj_name.clone(), value_type: vt, has_default });
                    }
                }
            }
        }

        // 收集注入器字段（显式 injector { } 块）
        if let Some(ref injector_decl) = decl.injector {
            for field in &injector_decl.fields {
                let vt = match &field.field_type {
                    TypeRef::Simple { name, .. } => match name.as_str() {
                        "int" => ValueType::Int,
                        "float" => ValueType::Float,
                        "bool" => ValueType::Bool,
                        "string" => ValueType::String,
                        _ => ValueType::Object,
                    },
                    _ => ValueType::Object,
                };
                self.injector_fields.push(InjectorFieldDef {
                    name: field.name.clone(),
                    value_type: vt,
                    has_default: true,
                });
            }
        }
    }

    /// Pass 3：声明一个字段
    fn pass3_declare_field(
        &mut self,
        class_id: ClassId,
        class_scope: ScopeId,
        decl: &FieldDeclaration,
        counters: &mut FieldOffsetCounters,
    ) {
        let field_type = self.resolve_ast_type(class_scope, &decl.field_type);
        let is_static = decl.modifiers.iter().any(|m| matches!(m, Modifier::Static));

        let field_id = self.symbol_table.declare_field(
            &decl.name,
            class_id,
            field_type.clone(),
            is_static,
            decl.span,
        );

        // 为非静态字段按值类型分组分配偏移
        // offset = 该字段在其所属值类型分组内的下标（每种类型从 0 开始独立计数）
        if !is_static {
            let vt = type_info_to_value_type(&field_type);
            let offset = counters.next(vt);
            self.symbol_table.allocate_field_offset(field_id, offset);
        }

        // 如果有初始化表达式，创建编译任务
        if decl.initializer.is_some() && !decl.is_native() {
            self.tasks.push(CompileTask {
                kind: TaskKind::FieldInitializer { field_id, class_id },
                span: decl.span,
            });
        }
    }

    /// Pass 3：声明一个方法
    fn pass3_declare_method(
        &mut self,
        class_id: ClassId,
        class_scope: ScopeId,
        decl: &MethodDeclaration,
    ) {
        let return_type = self.resolve_ast_type(class_scope, &decl.return_type);
        let is_static = decl.modifiers.iter().any(|m| matches!(m, Modifier::Static));
        let is_native = decl.modifiers.iter().any(|m| matches!(m, Modifier::Native));

        // 声明参数
        let params: Vec<ParameterId> = decl.parameters.iter().enumerate().map(|(i, p)| {
            let pt = self.resolve_ast_type(class_scope, &p.param_type);
            self.symbol_table.declare_parameter(&p.name, pt, i, p.span)
        }).collect();

        let method_id = self.symbol_table.declare_method(
            &decl.name,
            Some(class_id),
            None,
            return_type,
            params,
            is_static,
            is_native,
            decl.span,
        );

        // 非 native、有方法体的方法 → 创建编译任务
        if decl.body.is_some() && !is_native {
            let body_scope = self.symbol_table.push_scope(
                class_scope,
                ScopeKind::Method { method_id },
            );
            self.symbol_table.set_method_body_scope(method_id, body_scope);

            self.tasks.push(CompileTask {
                kind: TaskKind::Method { method_id },
                span: decl.span,
            });
        }
    }

    /// Pass 3：声明一个构造方法
    fn pass3_declare_constructor(
        &mut self,
        class_id: ClassId,
        class_scope: ScopeId,
        decl: &ConstructorDeclaration,
    ) {
        let is_native = decl.modifiers.iter().any(|m| matches!(m, Modifier::Native));

        let params: Vec<ParameterId> = decl.parameters.iter().enumerate().map(|(i, p)| {
            let pt = self.resolve_ast_type(class_scope, &p.param_type);
            self.symbol_table.declare_parameter(&p.name, pt, i, p.span)
        }).collect();

        let ctor_id = self.symbol_table.declare_constructor(
            class_id,
            params,
            is_native,
            decl.span,
        );

        // 非 native、有方法体的构造方法 → 创建编译任务
        if decl.body.is_some() && !is_native {
            let body_scope = self.symbol_table.push_scope(
                class_scope,
                ScopeKind::Constructor { constructor_id: ctor_id },
            );
            self.symbol_table.set_constructor_body_scope(ctor_id, body_scope);

            self.tasks.push(CompileTask {
                kind: TaskKind::Constructor { constructor_id: ctor_id },
                span: decl.span,
            });
        }
    }

    /// Pass 3：声明接口的方法签名
    fn pass3_declare_interface_members(&mut self, scope: ScopeId, decl: &InterfaceDeclaration) {
        let iface_id = match self.symbol_table.lookup_interface(scope, &decl.name) {
            Some(id) => id,
            None => return,
        };
        let iface_scope = self.symbol_table.interfaces.get(iface_id.0).scope_id;

        for method_sig in &decl.methods {
            let return_type = self.resolve_ast_type(iface_scope, &method_sig.return_type);
            let params: Vec<ParameterId> = method_sig.parameters.iter().enumerate().map(|(i, p)| {
                let pt = self.resolve_ast_type(iface_scope, &p.param_type);
                self.symbol_table.declare_parameter(&p.name, pt, i, p.span)
            }).collect();

            self.symbol_table.declare_method(
                &method_sig.name,
                None,
                Some(iface_id),
                return_type,
                params,
                false,
                false,
                method_sig.span,
            );
        }
    }

    /// 将 AST TypeRef 解析为 TypeInfo，解析失败时使用 Unresolved 并报错
    fn resolve_ast_type(&mut self, scope: ScopeId, type_ref: &TypeRef) -> TypeInfo {
        match self.symbol_table.resolve_type(scope, type_ref) {
            Some(ti) => ti,
            None => {
                let name = type_ref_name(type_ref);
                self.diagnostics.emit_error(
                    type_ref.span(),
                    format!("未找到类型 `{}`", name),
                );
                TypeInfo::Unresolved
            }
        }
    }

    // ==================== Pass 4: 代码生成 ====================

    /// 继承编号冻结（B-3）
    ///
    /// 为每个类计算含继承的方法/构造方法编号与实例字段索引：
    /// - 方法（静态+实例混合空间）：本类方法全局 ID = 父类 method_count_total + 本类局部序号
    /// - 构造方法：本类构造全局 ID = 父类 constructor_count_total + 本类局部序号
    /// - 实例字段：按值类型分组，本类字段起始索引 = 父类 field_type_count_total
    /// - 重写映射：本类方法若与祖先类同名同参数，记录 被重写全局 ID → 本类全局 ID
    ///
    /// 处理顺序保证父类先于子类（按继承深度排序）。
    fn freeze_inheritance(&mut self) {
        // 按继承深度升序排列所有类，确保父类先处理
        let mut class_ids: Vec<ClassId> = (0..self.symbol_table.classes.len())
            .map(ClassId)
            .collect();
        class_ids.sort_by_key(|cid| self.inheritance_depth(*cid));

        for cid in class_ids {
            let super_id = self.symbol_table.classes.get(cid.0).super_class;

            // 从父类继承起始值
            let (method_start, ctor_start, field_start) = if let Some(sid) = super_id {
                let sup = self.symbol_table.classes.get(sid.0);
                (
                    sup.method_count_total,
                    sup.constructor_count_total,
                    sup.field_type_count_total.clone(),
                )
            } else {
                (0, 0, FrozenTypeCount::default())
            };

            let own_methods = self.symbol_table.classes.get(cid.0).methods.clone();
            let own_ctors = self.symbol_table.classes.get(cid.0).constructors.clone();
            let own_fields = self.symbol_table.classes.get(cid.0).fields.clone();

            // 方法总数与构造总数
            let method_total = method_start + own_methods.len();
            let ctor_total = ctor_start + own_ctors.len();

            // 计算实例字段各值类型总数（从父类起始值累加本类非静态字段）
            let mut field_total = field_start.clone();
            for fid in &own_fields {
                let fi = self.symbol_table.fields.get(fid.0);
                if fi.is_static {
                    continue;
                }
                bump_frozen_type_count(&mut field_total, &fi.field_type);
            }

            // 构建重写映射：本类方法与祖先类同名同参数 → 记录 被重写全局 ID → 本类全局 ID
            let mut override_map: std::collections::HashMap<usize, usize> =
                std::collections::HashMap::new();
            if let Some(sid) = super_id {
                for (local_idx, mid) in own_methods.iter().enumerate() {
                    let mi = self.symbol_table.methods.get(mid.0).clone();
                    if mi.is_static {
                        continue;
                    }
                    if let Some(overridden_global) =
                        self.find_overridden_method(sid, &mi)
                    {
                        let own_global = method_start + local_idx;
                        override_map.insert(overridden_global, own_global);
                    }
                }
            }

            // 构建接口方法实现映射（F1）：为本类实现的每个接口，按名字+签名
            // 匹配类的实例方法（含继承链），得到 [接口方法本地ID → 类方法全局ID]
            let iface_map = self.build_interface_impl_map(cid);

            // 写回
            let ci = self.symbol_table.classes.get_mut(cid.0);
            ci.method_start_id = method_start;
            ci.method_count_total = method_total;
            ci.constructor_start_id = ctor_start;
            ci.constructor_count_total = ctor_total;
            ci.field_start_type_count = field_start;
            ci.field_type_count_total = field_total;
            ci.method_override_id = override_map;
            ci.interface_method_impl_id = iface_map;
        }
    }

    /// 为类构建接口方法实现映射
    ///
    /// 对类实现的每个接口，遍历接口方法（按声明顺序 = 接口方法本地ID），
    /// 在类的实例方法中按「名字 + 参数签名」匹配实现方法，记录其全局方法编号。
    /// 返回 `Map<接口全名, Vec<类方法全局ID>>`。
    fn build_interface_impl_map(&self, class_id: ClassId) -> std::collections::HashMap<String, Vec<usize>> {
        let mut result = std::collections::HashMap::new();
        let ci = self.symbol_table.classes.get(class_id.0);
        let ifaces = ci.super_interfaces.clone();
        for iface_id in ifaces {
            let iface = self.symbol_table.interfaces.get(iface_id.0);
            let iface_name = iface.name.clone();
            let iface_methods = iface.methods.clone();
            let mut impl_ids: Vec<usize> = Vec::with_capacity(iface_methods.len());
            for &imid in &iface_methods {
                let im = self.symbol_table.methods.get(imid.0).clone();
                let im_params: Vec<ParameterId> = im.parameters.clone();
                // 在类（含继承链）中按名字+签名找实现方法的全局编号
                let global = self.find_impl_method_global_id(class_id, &im.name, &im_params);
                impl_ids.push(global.unwrap_or(usize::MAX));
            }
            result.insert(iface_name, impl_ids);
        }
        result
    }

    /// 在类（含继承链）中按名字+参数签名查找实例方法的全局编号
    fn find_impl_method_global_id(
        &self,
        class_id: ClassId,
        name: &str,
        iface_params: &[ParameterId],
    ) -> Option<usize> {
        let mut cur = Some(class_id);
        while let Some(cid) = cur {
            let ci = self.symbol_table.classes.get(cid.0);
            for (local_idx, &mid) in ci.methods.iter().enumerate() {
                let mi = self.symbol_table.methods.get(mid.0);
                if mi.name == name && !mi.is_static && self.same_parameter_types(&mi.parameters, iface_params) {
                    return Some(ci.method_start_id + local_idx);
                }
            }
            cur = ci.super_class;
        }
        None
    }

    /// 计算类的继承深度（无父类为 0）
    fn inheritance_depth(&self, class_id: ClassId) -> usize {
        let mut depth = 0;
        let mut cur = self.symbol_table.classes.get(class_id.0).super_class;
        while let Some(sid) = cur {
            depth += 1;
            cur = self.symbol_table.classes.get(sid.0).super_class;
            if depth > 1000 {
                break; // 防御循环继承
            }
        }
        depth
    }

    /// 在祖先链中查找与给定方法同名同参数的方法，返回其全局 ID
    fn find_overridden_method(&self, super_id: ClassId, method: &MethodInfo) -> Option<usize> {
        let mut cur = Some(super_id);
        while let Some(cid) = cur {
            let ci = self.symbol_table.classes.get(cid.0);
            for (local_idx, mid) in ci.methods.iter().enumerate() {
                let mi = self.symbol_table.methods.get(mid.0);
                if mi.is_static == method.is_static
                    && mi.name == method.name
                    && self.same_parameter_types(&mi.parameters, &method.parameters)
                {
                    return Some(ci.method_start_id + local_idx);
                }
            }
            cur = ci.super_class;
        }
        None
    }

    /// 比较两个参数列表的类型是否一致
    fn same_parameter_types(&self, a: &[ParameterId], b: &[ParameterId]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        for (pa, pb) in a.iter().zip(b.iter()) {
            let ta = &self.symbol_table.parameters.get(pa.0).param_type;
            let tb = &self.symbol_table.parameters.get(pb.0).param_type;
            if !type_info_eq(ta, tb) {
                return false;
            }
        }
        true
    }

    /// Pass 4 — 代码生成
    ///
    /// 遍历 Pass 3 产出的编译任务，将每个方法体/构造方法体的 AST 转换为 IR 指令序列。
    fn pass4_code_generation(&mut self, sources: &[SourceFile]) -> Result<(), ()> {
        // 暂存当前 tasks，避免借用冲突
        let tasks = self.tasks.clone();

        for task in &tasks {
            match &task.kind {
                TaskKind::Method { method_id } => {
                    let method_info = self.symbol_table.methods.get(method_id.0).clone();
                    if method_info.is_native {
                        continue;
                    }
                    // 查找对应 AST 并生成代码
                    self.generate_method_ir(sources, *method_id, &method_info);
                }
                TaskKind::Constructor { constructor_id } => {
                    let ctor_info = self.symbol_table.constructors.get(constructor_id.0).clone();
                    if ctor_info.is_native {
                        continue;
                    }
                    self.generate_constructor_ir(sources, *constructor_id, &ctor_info);
                }
                _ => {} // 字段初始化器暂不处理
            }
        }

        if self.diagnostics.has_errors() {
            Err(())
        } else {
            Ok(())
        }
    }

    /// 生成方法体的 IR
    fn generate_method_ir(
        &mut self,
        sources: &[SourceFile],
        _method_id: MethodId,
        method_info: &MethodInfo,
    ) {
        let class_name = method_info.class_id
            .map(|cid| self.symbol_table.classes.get(cid.0).name.clone())
            .unwrap_or_default();

        // 在 AST 中搜索该方法的声明
        for source in sources {
            for member in &source.members {
                if let TopLevelMember::Class(class_decl) = member {
                    if !class_name.is_empty() && class_decl.name != class_name {
                        continue;
                    }
                    if let Some(stmts) = self.find_matching_method_body(class_decl, method_info) {
                        let mut cg = CodeGenerator::new(&self.symbol_table, &mut self.diagnostics, &mut self.delegate_impls);

                        cg.set_class_context(&class_decl.name);

                        // 注册参数
                        let params: Vec<(String, ValueType)> = method_info.parameters.iter()
                            .map(|pid| {
                                let p = self.symbol_table.parameters.get(pid.0);
                                (p.name.clone(), type_info_to_value_type(&p.param_type))
                            })
                            .collect();
                        cg.register_parameters(&params);

                        // 记录类类型参数的类名，供实例方法解析
                        for pid in &method_info.parameters {
                            let p = self.symbol_table.parameters.get(pid.0);
                            // 记录参数完整类型（供 E1 类型推导）
                            cg.register_var_type(&p.name, p.param_type.clone());
                            if let TypeInfo::Object(cid) = &p.param_type {
                                let cname = self.symbol_table.classes.get(cid.0).name.clone();
                                cg.register_var_class(&p.name, &cname);
                            }
                        }

                        // 生成语句的 IR
                        for stmt in stmts {
                            cg.generate_statement(stmt);
                        }

                        cg.report_unresolved_leaves();
                        let class_key = cg.current_class_name.clone().unwrap_or_default();
                        let ic = std::mem::take(&mut cg.injector_constants);
                        if !ic.is_empty() {
                            self.injector_constants.entry(class_key).or_default().extend(ic);
                        }
                        let total_locals = cg.total_locals();
                        let codes = cg.into_codes();

                        self.compiled_methods.push(CompiledMethodContents {
                            name: method_info.name.clone(),
                            codes,
                            total_locals,
                            class_id: method_info.class_id,
                            is_constructor: false,
                        });
                        return;
                    }
                }
            }
        }
    }

    /// 生成构造方法体的 IR
    fn generate_constructor_ir(
        &mut self,
        sources: &[SourceFile],
        _constructor_id: ConstructorId,
        ctor_info: &ConstructorInfo,
    ) {
        let class_name = &self.symbol_table.classes.get(ctor_info.class_id.0).name.clone();

        for source in sources {
            for member in &source.members {
                if let TopLevelMember::Class(class_decl) = member {
                    if class_decl.name != *class_name {
                        continue;
                    }
                    // 按参数类型精确匹配构造方法声明（区分重载），而非取第一个
                    if let Some(ctor_decl) = self.find_matching_constructor_decl(class_decl, ctor_info) {
                        let stmts = match &ctor_decl.body {
                            Some(s) => s,
                            None => return,
                        };
                        let mut cg = CodeGenerator::new(&self.symbol_table, &mut self.diagnostics, &mut self.delegate_impls);

                        cg.set_class_context(&class_decl.name);

                        let params: Vec<(String, ValueType)> = ctor_info.parameters.iter()
                            .map(|pid| {
                                let p = self.symbol_table.parameters.get(pid.0);
                                (p.name.clone(), type_info_to_value_type(&p.param_type))
                            })
                            .collect();
                        cg.register_parameters(&params);

                        // 记录类类型参数的类名，供实例方法解析
                        for pid in &ctor_info.parameters {
                            let p = self.symbol_table.parameters.get(pid.0);
                            cg.register_var_type(&p.name, p.param_type.clone());
                            if let TypeInfo::Object(cid) = &p.param_type {
                                let cname = self.symbol_table.classes.get(cid.0).name.clone();
                                cg.register_var_class(&p.name, &cname);
                            }
                        }

                        // 若有父类，先生成 super(...) 调用初始化继承字段
                        if let Some(super_id) = self.symbol_table.classes.get(ctor_info.class_id.0).super_class {
                            let super_name = self.symbol_table.classes.get(super_id.0).name.clone();
                            cg.emit_super_constructor_call(&super_name, &ctor_decl.base_arguments, ctor_decl.span);
                        }

                        for stmt in stmts {
                            cg.generate_statement(stmt);
                        }

                        cg.report_unresolved_leaves();
                        let class_key = cg.current_class_name.clone().unwrap_or_default();
                        let ic = std::mem::take(&mut cg.injector_constants);
                        if !ic.is_empty() {
                            self.injector_constants.entry(class_key).or_default().extend(ic);
                        }
                        let total_locals = cg.total_locals();

                        self.compiled_methods.push(CompiledMethodContents {
                            name: "constructor".into(),
                            codes: cg.into_codes(),
                            total_locals,
                            class_id: Some(ctor_info.class_id),
                            is_constructor: true,
                        });
                        return;
                    }
                }
            }
        }
    }

    /// 在类声明中查找指定名称的方法体
    fn find_method_body<'a>(
        &self,
        class_decl: &'a ClassDeclaration,
        method_name: &str,
    ) -> Option<&'a Vec<Statement>> {
        for member in &class_decl.members {
            if let ClassMember::Method(m) = member {
                if m.name == method_name {
                    return m.body.as_ref();
                }
            }
        }
        None
    }

    /// 按名字+参数类型匹配方法体，区分重载
    ///
    /// 与 `find_method_body` 的区别：同名方法有多个重载时，按参数类型序列精确匹配，
    /// 确保每个方法编译任务定位到正确的方法体。
    fn find_matching_method_body<'a>(
        &self,
        class_decl: &'a ClassDeclaration,
        method_info: &MethodInfo,
    ) -> Option<&'a Vec<Statement>> {
        let target: Vec<TypeInfo> = method_info
            .parameters
            .iter()
            .map(|pid| self.symbol_table.parameters.get(pid.0).param_type.clone())
            .collect();
        for member in &class_decl.members {
            if let ClassMember::Method(m) = member {
                if m.name != method_info.name || m.parameters.len() != target.len() {
                    continue;
                }
                let all_match = m.parameters.iter().zip(target.iter()).all(|(p, t)| {
                    self.type_ref_matches_type_info(&p.param_type, t)
                });
                if all_match {
                    return m.body.as_ref();
                }
            }
        }
        None
    }

    /// 在类声明中查找构造方法体
    fn find_constructor_body<'a>(
        &self,
        class_decl: &'a ClassDeclaration,
    ) -> Option<&'a Vec<Statement>> {
        for member in &class_decl.members {
            if let ClassMember::Constructor(c) = member {
                return c.body.as_ref();
            }
        }
        None
    }

    /// 查找类的构造方法声明（用于获取 base_arguments 等完整信息）
    fn find_constructor_decl<'a>(
        &self,
        class_decl: &'a ClassDeclaration,
    ) -> Option<&'a ConstructorDeclaration> {
        for member in &class_decl.members {
            if let ClassMember::Constructor(c) = member {
                return Some(c);
            }
        }
        None
    }

    /// 按参数类型匹配构造方法声明，区分重载
    ///
    /// 遍历类的构造方法声明，选择参数类型序列与 `ctor_info.parameters` 一致的那个，
    /// 从而为每个构造方法编译任务定位到正确的方法体（避免多个构造方法都取第一个）。
    fn find_matching_constructor_decl<'a>(
        &self,
        class_decl: &'a ClassDeclaration,
        ctor_info: &ConstructorInfo,
    ) -> Option<&'a ConstructorDeclaration> {
        // 目标参数类型序列
        let target: Vec<TypeInfo> = ctor_info
            .parameters
            .iter()
            .map(|pid| self.symbol_table.parameters.get(pid.0).param_type.clone())
            .collect();
        for member in &class_decl.members {
            if let ClassMember::Constructor(c) = member {
                if c.parameters.len() != target.len() {
                    continue;
                }
                // 逐个比对参数类型（用只读方式比较声明 TypeRef 与目标 TypeInfo）
                let all_match = c.parameters.iter().zip(target.iter()).all(|(p, t)| {
                    self.type_ref_matches_type_info(&p.param_type, t)
                });
                if all_match {
                    return Some(c);
                }
            }
        }
        None
    }

    /// 只读比较 AST 的 TypeRef 与已解析的 TypeInfo 是否表示同一类型
    fn type_ref_matches_type_info(&self, tr: &TypeRef, ti: &TypeInfo) -> bool {
        match tr {
            TypeRef::Simple { name, .. } => match name.as_str() {
                "int" => matches!(ti, TypeInfo::Int),
                "float" => matches!(ti, TypeInfo::Float),
                "bool" => matches!(ti, TypeInfo::Bool),
                "string" => matches!(ti, TypeInfo::String),
                "void" => matches!(ti, TypeInfo::Void),
                other => match ti {
                    TypeInfo::Object(cid) => self.symbol_table.classes.get(cid.0).name == *other,
                    TypeInfo::Enum(eid) => self.symbol_table.enums.get(eid.0).name == *other,
                    _ => false,
                },
            },
            _ => false,
        }
    }
}

/// 将 TypeInfo 转为 ValueType
fn type_info_to_value_type(ti: &TypeInfo) -> ValueType {
    match ti {
        TypeInfo::Int => ValueType::Int,
        TypeInfo::Float => ValueType::Float,
        TypeInfo::Bool => ValueType::Bool,
        TypeInfo::String => ValueType::String,
        _ => ValueType::Object,
    }
}

/// 从 AST TypeRef 推导 ValueType（用于注入器字段类型推断）
fn type_ref_to_value_type(tr: &TypeRef) -> ValueType {
    match tr {
        TypeRef::Simple { name, .. } => match name.as_str() {
            "int" => ValueType::Int, "float" => ValueType::Float,
            "bool" => ValueType::Bool, "string" => ValueType::String,
            _ => ValueType::Object,
        },
        _ => ValueType::Object,
    }
}

/// 按字段类型对 FrozenTypeCount 的对应分组 +1（枚举计入 int）
fn bump_frozen_type_count(tc: &mut FrozenTypeCount, ti: &TypeInfo) {
    match ti {
        TypeInfo::Int | TypeInfo::Enum(_) => tc.int += 1,
        TypeInfo::Float => tc.float += 1,
        TypeInfo::Bool => tc.bool += 1,
        TypeInfo::String => tc.string += 1,
        _ => tc.object += 1,
    }
}

/// 判断两个 TypeInfo 是否表示相同类型（用于重写方法的参数比对）
fn type_info_eq(a: &TypeInfo, b: &TypeInfo) -> bool {
    match (a, b) {
        (TypeInfo::Int, TypeInfo::Int)
        | (TypeInfo::Float, TypeInfo::Float)
        | (TypeInfo::Bool, TypeInfo::Bool)
        | (TypeInfo::String, TypeInfo::String)
        | (TypeInfo::Void, TypeInfo::Void) => true,
        (TypeInfo::Object(x), TypeInfo::Object(y)) => x == y,
        (TypeInfo::Interface(x), TypeInfo::Interface(y)) => x == y,
        (TypeInfo::Enum(x), TypeInfo::Enum(y)) => x == y,
        _ => false,
    }
}

// 辅助：判断 FieldDeclaration 是否为 native
trait FieldNative {
    fn is_native(&self) -> bool;
}

impl FieldNative for FieldDeclaration {
    fn is_native(&self) -> bool {
        false // Rust AST 中字段本身没有 native 修饰符
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

/// 获取 TypeRef 的用户可读名称
fn type_ref_name(type_ref: &TypeRef) -> String {
    match type_ref {
        TypeRef::Simple { name, .. } => name.clone(),
        TypeRef::Generic { name, .. } => name.clone(),
        TypeRef::Array { element_type, .. } => format!("{}[]", type_ref_name(element_type)),
        TypeRef::Delegate { return_type, param_types, .. } => {
            let params: Vec<String> = param_types.iter().map(type_ref_name).collect();
            format!("delegate<{}, {}>", type_ref_name(return_type), params.join(", "))
        }
        TypeRef::Injector { base_type, .. } => format!("{}^", type_ref_name(base_type)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gorge_core::diagnostics::Span;

    fn dummy_span() -> Span {
        Span::new(0, 1, 1, 1, 0)
    }

    /// 辅助：创建空的 SourceFile
    fn empty_source() -> SourceFile {
        SourceFile {
            namespace: None,
            usings: vec![],
            members: vec![],
            span: dummy_span(),
        }
    }

    #[test]
    fn test_pass1_class_declaration() {
        let source = SourceFile {
            members: vec![TopLevelMember::Class(ClassDeclaration {
                annotations: vec![],
                modifiers: vec![],
                name: "MyClass".into(),
                super_class: None,
                super_interfaces: vec![],
                members: vec![],
                injector: None,
                span: dummy_span(),
            })],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        let class_id = compiler.symbol_table
            .lookup_class(compiler.symbol_table.global_scope, "MyClass")
            .expect("MyClass 应该已在符号表中");
        assert_eq!(compiler.symbol_table.classes.get(class_id.0).name, "MyClass");
    }

    #[test]
    fn test_pass1_multiple_types() {
        let source = SourceFile {
            members: vec![
                TopLevelMember::Class(ClassDeclaration {
                    annotations: vec![],
                    modifiers: vec![],
                    name: "A".into(),
                    super_class: None,
                    super_interfaces: vec![],
                    members: vec![],
                    injector: None,
                    span: dummy_span(),
                }),
                TopLevelMember::Interface(InterfaceDeclaration {
                    annotations: vec![],
                    modifiers: vec![],
                    name: "IB".into(),
                    super_interfaces: vec![],
                    methods: vec![],
                    span: dummy_span(),
                }),
                TopLevelMember::Enum(EnumDeclaration {
                    annotations: vec![],
                    modifiers: vec![],
                    name: "Color".into(),
                    values: vec![],
                    span: dummy_span(),
                }),
            ],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        assert!(compiler.symbol_table.lookup_class(
            compiler.symbol_table.global_scope, "A").is_some());
        assert!(compiler.symbol_table.lookup_interface(
            compiler.symbol_table.global_scope, "IB").is_some());
    }

    #[test]
    fn test_pass1_namespace() {
        let ns_name = QualifiedName {
            parts: vec!["Game".into(), "Entities".into()],
            span: dummy_span(),
        };

        let source = SourceFile {
            namespace: Some(ns_name),
            members: vec![TopLevelMember::Class(ClassDeclaration {
                annotations: vec![],
                modifiers: vec![],
                name: "Player".into(),
                super_class: None,
                super_interfaces: vec![],
                members: vec![],
                injector: None,
                span: dummy_span(),
            })],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        // 验证命名空间链存在
        let game_scope = compiler.symbol_table.lookup_local(
            compiler.symbol_table.global_scope, "Game");
        assert!(game_scope.is_some(), "Game 命名空间应该存在");
    }

    #[test]
    fn test_pass2_class_inheritance() {
        let base = ClassDeclaration {
            annotations: vec![],
            modifiers: vec![Modifier::Native],
            name: "Base".into(),
            super_class: None,
            super_interfaces: vec![],
            members: vec![],
            injector: None,
            span: dummy_span(),
        };

        let derived = ClassDeclaration {
            annotations: vec![],
            modifiers: vec![],
            name: "Derived".into(),
            super_class: Some(TypeRef::simple("Base", dummy_span())),
            super_interfaces: vec![],
            members: vec![],
            injector: None,
            span: dummy_span(),
        };

        let source = SourceFile {
            members: vec![
                TopLevelMember::Class(base),
                TopLevelMember::Class(derived),
            ],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        let derived_id = compiler.symbol_table
            .lookup_class(compiler.symbol_table.global_scope, "Derived")
            .unwrap();
        let derived_info = compiler.symbol_table.classes.get(derived_id.0);
        assert!(derived_info.super_class.is_some(), "Derived 应该有父类");
    }

    #[test]
    fn test_pass2_enum_values() {
        let source = SourceFile {
            members: vec![TopLevelMember::Enum(EnumDeclaration {
                annotations: vec![],
                modifiers: vec![],
                name: "Suit".into(),
                values: vec![
                    EnumValue { annotations: vec![], name: "Hearts".into(), value: Some(1), span: dummy_span() },
                    EnumValue { annotations: vec![], name: "Diamonds".into(), value: Some(2), span: dummy_span() },
                    EnumValue { annotations: vec![], name: "Clubs".into(), value: None, span: dummy_span() },
                ],
                span: dummy_span(),
            })],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        // 验证枚举值已注册
        let enum_id = compiler.find_enum_by_name(
            compiler.symbol_table.global_scope, "Suit").unwrap();
        let enum_info = compiler.symbol_table.enums.get(enum_id.0);
        assert_eq!(enum_info.values.len(), 3);
    }

    #[test]
    fn test_pass2_class_implements_interface() {
        let iface = InterfaceDeclaration {
            annotations: vec![],
            modifiers: vec![],
            name: "IRunnable".into(),
            super_interfaces: vec![],
            methods: vec![],
            span: dummy_span(),
        };

        let class = ClassDeclaration {
            annotations: vec![],
            modifiers: vec![],
            name: "Task".into(),
            super_class: None,
            super_interfaces: vec![TypeRef::simple("IRunnable", dummy_span())],
            members: vec![],
            injector: None,
            span: dummy_span(),
        };

        let source = SourceFile {
            members: vec![
                TopLevelMember::Interface(iface),
                TopLevelMember::Class(class),
            ],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        let task_id = compiler.symbol_table
            .lookup_class(compiler.symbol_table.global_scope, "Task")
            .unwrap();
        let task_info = compiler.symbol_table.classes.get(task_id.0);
        assert_eq!(task_info.super_interfaces.len(), 1);
    }

    #[test]
    fn test_native_class_modifier() {
        let source = SourceFile {
            members: vec![TopLevelMember::Class(ClassDeclaration {
                annotations: vec![],
                modifiers: vec![Modifier::Native, Modifier::Static],
                name: "Console".into(),
                super_class: None,
                super_interfaces: vec![],
                members: vec![],
                injector: None,
                span: dummy_span(),
            })],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        let class_id = compiler.symbol_table
            .lookup_class(compiler.symbol_table.global_scope, "Console")
            .unwrap();
        let class_info = compiler.symbol_table.classes.get(class_id.0);
        assert!(class_info.is_native);
        assert!(class_info.is_static);
    }

    #[test]
    fn test_pass2_missing_super_class_generates_error() {
        let source = SourceFile {
            members: vec![TopLevelMember::Class(ClassDeclaration {
                annotations: vec![],
                modifiers: vec![],
                name: "Orphan".into(),
                super_class: Some(TypeRef::simple("NonexistentBase", dummy_span())),
                super_interfaces: vec![],
                members: vec![],
                injector: None,
                span: dummy_span(),
            })],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        let result = compiler.compile(&[source]);
        assert!(result.is_err(), "不存在的父类应该产生错误");
    }

    // ==================== Pass 3 测试 ====================

    #[test]
    fn test_pass3_field_declaration() {
        let source = SourceFile {
            members: vec![TopLevelMember::Class(ClassDeclaration {
                annotations: vec![],
                modifiers: vec![],
                name: "Point".into(),
                super_class: None,
                super_interfaces: vec![],
                members: vec![
                    ClassMember::Field(FieldDeclaration {
                        annotations: vec![],
                        modifiers: vec![],
                        field_type: TypeRef::simple("int", dummy_span()),
                        name: "x".into(),
                        initializer: Some(Expression::Literal(Literal::Int(0), dummy_span())),
                        span: dummy_span(),
                    }),
                    ClassMember::Field(FieldDeclaration {
                        annotations: vec![],
                        modifiers: vec![],
                        field_type: TypeRef::simple("int", dummy_span()),
                        name: "y".into(),
                        initializer: None,
                        span: dummy_span(),
                    }),
                ],
                injector: None,
                span: dummy_span(),
            })],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        let class_id = compiler.symbol_table
            .lookup_class(compiler.symbol_table.global_scope, "Point")
            .unwrap();
        let info = compiler.symbol_table.classes.get(class_id.0);
        assert_eq!(info.fields.len(), 2, "Point 应该有 2 个字段");

        // 含有初始化表达式的字段应有编译任务
        let field_init_tasks: Vec<_> = compiler.tasks.iter()
            .filter(|t| matches!(t.kind, TaskKind::FieldInitializer { .. }))
            .collect();
        assert_eq!(field_init_tasks.len(), 1, "字段 x 有初始化器，应产生 1 个任务");
    }

    #[test]
    fn test_pass3_method_declaration() {
        let source = SourceFile {
            members: vec![TopLevelMember::Class(ClassDeclaration {
                annotations: vec![],
                modifiers: vec![],
                name: "Calculator".into(),
                super_class: None,
                super_interfaces: vec![],
                members: vec![ClassMember::Method(MethodDeclaration {
                    annotations: vec![],
                    modifiers: vec![],
                    return_type: TypeRef::simple("int", dummy_span()),
                    name: "add".into(),
                    parameters: vec![
                        Parameter {
                            name: "a".into(),
                            param_type: TypeRef::simple("int", dummy_span()),
                            span: dummy_span(),
                        },
                        Parameter {
                            name: "b".into(),
                            param_type: TypeRef::simple("int", dummy_span()),
                            span: dummy_span(),
                        },
                    ],
                    body: Some(vec![]),
                    span: dummy_span(),
                })],
                injector: None,
                span: dummy_span(),
            })],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        let class_id = compiler.symbol_table
            .lookup_class(compiler.symbol_table.global_scope, "Calculator")
            .unwrap();
        let class_scope = compiler.symbol_table.classes.get(class_id.0).scope_id;
        let methods = compiler.symbol_table.lookup_method(class_scope, "add");
        assert_eq!(methods.len(), 1);

        let method_info = compiler.symbol_table.methods.get(methods[0].0);
        assert_eq!(method_info.parameters.len(), 2);
        assert!(method_info.body_scope_id.is_some(), "有方法体的方法应有 body scope");

        // 应有方法编译任务
        let method_tasks: Vec<_> = compiler.tasks.iter()
            .filter(|t| matches!(t.kind, TaskKind::Method { .. }))
            .collect();
        assert_eq!(method_tasks.len(), 1);
    }

    #[test]
    fn test_pass3_native_method_no_task() {
        let source = SourceFile {
            members: vec![TopLevelMember::Class(ClassDeclaration {
                annotations: vec![],
                modifiers: vec![Modifier::Native],
                name: "Console".into(),
                super_class: None,
                super_interfaces: vec![],
                members: vec![ClassMember::Method(MethodDeclaration {
                    annotations: vec![],
                    modifiers: vec![Modifier::Native],
                    return_type: TypeRef::simple("void", dummy_span()),
                    name: "print".into(),
                    parameters: vec![],
                    body: None,
                    span: dummy_span(),
                })],
                injector: None,
                span: dummy_span(),
            })],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        // Native 方法不应产生编译任务
        assert!(compiler.tasks.is_empty());
    }

    #[test]
    fn test_pass3_constructor_declaration() {
        let source = SourceFile {
            members: vec![TopLevelMember::Class(ClassDeclaration {
                annotations: vec![],
                modifiers: vec![],
                name: "Person".into(),
                super_class: None,
                super_interfaces: vec![],
                members: vec![ClassMember::Constructor(ConstructorDeclaration {
                    annotations: vec![],
                    modifiers: vec![],
                    parameters: vec![Parameter {
                        name: "name".into(),
                        param_type: TypeRef::simple("string", dummy_span()),
                        span: dummy_span(),
                    }],
                    base_arguments: vec![],
                    body: Some(vec![]),
                    span: dummy_span(),
                })],
                injector: None,
                span: dummy_span(),
            })],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        let class_id = compiler.symbol_table
            .lookup_class(compiler.symbol_table.global_scope, "Person")
            .unwrap();
        let info = compiler.symbol_table.classes.get(class_id.0);
        assert_eq!(info.constructors.len(), 1);

        let ctor_tasks: Vec<_> = compiler.tasks.iter()
            .filter(|t| matches!(t.kind, TaskKind::Constructor { .. }))
            .collect();
        assert_eq!(ctor_tasks.len(), 1);
    }

    #[test]
    fn test_pass3_interface_method_signatures() {
        let source = SourceFile {
            members: vec![TopLevelMember::Interface(InterfaceDeclaration {
                annotations: vec![],
                modifiers: vec![],
                name: "IComparable".into(),
                super_interfaces: vec![],
                methods: vec![MethodSignature {
                    annotations: vec![],
                    return_type: TypeRef::simple("bool", dummy_span()),
                    name: "compare".into(),
                    parameters: vec![Parameter {
                        name: "other".into(),
                        param_type: TypeRef::simple("object", dummy_span()),
                        span: dummy_span(),
                    }],
                    span: dummy_span(),
                }],
                span: dummy_span(),
            })],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        let iface_id = compiler.symbol_table
            .lookup_interface(compiler.symbol_table.global_scope, "IComparable")
            .unwrap();
        let info = compiler.symbol_table.interfaces.get(iface_id.0);
        assert_eq!(info.methods.len(), 1);
    }

    #[test]
    fn test_pass3_field_offset_allocation() {
        let source = SourceFile {
            members: vec![TopLevelMember::Class(ClassDeclaration {
                annotations: vec![],
                modifiers: vec![],
                name: "Vector3".into(),
                super_class: None,
                super_interfaces: vec![],
                members: vec![
                    field("x", "float"),
                    field("y", "float"),
                    field("z", "float"),
                ],
                injector: None,
                span: dummy_span(),
            })],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        let class_id = compiler.symbol_table
            .lookup_class(compiler.symbol_table.global_scope, "Vector3")
            .unwrap();
        let info = compiler.symbol_table.classes.get(class_id.0);

        // 验证字段偏移
        for (i, &field_id) in info.fields.iter().enumerate() {
            let fi = compiler.symbol_table.fields.get(field_id.0);
            assert_eq!(fi.offset, Some(i), "字段 {} 偏移应为 {}", fi.name, i);
        }
    }

    /// 验证混合类型字段的 offset 按值类型分组分配
    ///
    /// 字段声明顺序 `int a; float b; int c; float d; bool e;`，
    /// 正确偏移应为：
    ///   a=0(int组), b=0(float组), c=1(int组), d=1(float组), e=0(bool组)
    #[test]
    fn test_pass3_field_offset_by_value_type() {
        let source = SourceFile {
            members: vec![TopLevelMember::Class(ClassDeclaration {
                annotations: vec![],
                modifiers: vec![],
                name: "Mixed".into(),
                super_class: None,
                super_interfaces: vec![],
                members: vec![
                    field("a", "int"),
                    field("b", "float"),
                    field("c", "int"),
                    field("d", "float"),
                    field("e", "bool"),
                ],
                injector: None,
                span: dummy_span(),
            })],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        let class_id = compiler.symbol_table
            .lookup_class(compiler.symbol_table.global_scope, "Mixed")
            .unwrap();
        let info = compiler.symbol_table.classes.get(class_id.0);

        // 按字段名核对 offset
        let offsets: Vec<(&str, usize)> = info.fields.iter()
            .map(|&fid| {
                let fi = compiler.symbol_table.fields.get(fid.0);
                (fi.name.as_str(), fi.offset.unwrap())
            })
            .collect();

        // 预期：a(int)=0, b(float)=0, c(int)=1, d(float)=1, e(bool)=0
        let expected = vec![
            ("a", 0), // int 组第 0 个
            ("b", 0), // float 组第 0 个
            ("c", 1), // int 组第 1 个
            ("d", 1), // float 组第 1 个
            ("e", 0), // bool 组第 0 个
        ];

        assert_eq!(offsets.len(), expected.len(), "字段数量应匹配");
        for (&(name, offset), &(exp_name, exp_offset)) in offsets.iter().zip(expected.iter()) {
            assert_eq!(name, exp_name, "字段名应匹配");
            assert_eq!(offset, exp_offset, "字段 {} 的偏移应为 {}，实际为 {}", name, exp_offset, offset);
        }
    }

    fn field(name: &str, ty: &str) -> ClassMember {
        ClassMember::Field(FieldDeclaration {
            annotations: vec![],
            modifiers: vec![],
            field_type: TypeRef::simple(ty, dummy_span()),
            name: name.into(),
            initializer: None,
            span: dummy_span(),
        })
    }

    /// 构造一个无参、返回 int 的实例方法声明
    fn simple_method(name: &str) -> ClassMember {
        ClassMember::Method(MethodDeclaration {
            annotations: vec![],
            modifiers: vec![],
            return_type: TypeRef::simple("int", dummy_span()),
            name: name.into(),
            parameters: vec![],
            body: Some(vec![]),
            span: dummy_span(),
        })
    }

    /// 验证继承编号冻结（B-3）
    ///
    /// Base: methodA(0) methodB(1)；Derived extends Base: methodB(重写) methodC。
    /// 期望：
    ///   Base.method_start_id=0, method_count_total=2
    ///   Derived.method_start_id=2, method_count_total=4
    ///   Derived.method_override_id: {1 → 2}（重写 Base 的 methodB(全局1) 为 Derived 局部0(全局2)）
    #[test]
    fn test_freeze_inheritance_numbering() {
        let source = SourceFile {
            members: vec![
                TopLevelMember::Class(ClassDeclaration {
                    annotations: vec![],
                    modifiers: vec![],
                    name: "Base".into(),
                    super_class: None,
                    super_interfaces: vec![],
                    members: vec![simple_method("methodA"), simple_method("methodB")],
                    injector: None,
                    span: dummy_span(),
                }),
                TopLevelMember::Class(ClassDeclaration {
                    annotations: vec![],
                    modifiers: vec![],
                    name: "Derived".into(),
                    super_class: Some(TypeRef::simple("Base", dummy_span())),
                    super_interfaces: vec![],
                    members: vec![simple_method("methodB"), simple_method("methodC")],
                    injector: None,
                    span: dummy_span(),
                }),
            ],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        let g = compiler.symbol_table.global_scope;
        let base_id = compiler.symbol_table.lookup_class(g, "Base").unwrap();
        let derived_id = compiler.symbol_table.lookup_class(g, "Derived").unwrap();
        let base = compiler.symbol_table.classes.get(base_id.0);
        let derived = compiler.symbol_table.classes.get(derived_id.0);

        assert_eq!(base.method_start_id, 0, "Base 起始编号应为 0");
        assert_eq!(base.method_count_total, 2, "Base 方法总数应为 2");
        assert_eq!(derived.method_start_id, 2, "Derived 起始编号应为 2");
        assert_eq!(derived.method_count_total, 4, "Derived 方法总数应为 4");
        // Derived 的 methodB 重写 Base 的 methodB（全局1）→ Derived 局部0=全局2
        assert_eq!(derived.method_override_id.get(&1), Some(&2), "重写映射应为 1→2");
        // methodC 不是重写，不应在映射中
        assert_eq!(derived.method_override_id.len(), 1, "只应有一个重写映射");
    }

    /// 验证继承字段索引冻结（B-3）
    ///
    /// Base: int a; float b；Derived extends Base: int c; float d。
    /// 期望 Derived 字段起始 int=1,float=1，总计 int=2,float=2。
    #[test]
    fn test_freeze_inheritance_fields() {
        let source = SourceFile {
            members: vec![
                TopLevelMember::Class(ClassDeclaration {
                    annotations: vec![],
                    modifiers: vec![],
                    name: "Base".into(),
                    super_class: None,
                    super_interfaces: vec![],
                    members: vec![field("a", "int"), field("b", "float")],
                    injector: None,
                    span: dummy_span(),
                }),
                TopLevelMember::Class(ClassDeclaration {
                    annotations: vec![],
                    modifiers: vec![],
                    name: "Derived".into(),
                    super_class: Some(TypeRef::simple("Base", dummy_span())),
                    super_interfaces: vec![],
                    members: vec![field("c", "int"), field("d", "float")],
                    injector: None,
                    span: dummy_span(),
                }),
            ],
            ..empty_source()
        };

        let mut compiler = Compiler::new();
        compiler.compile(&[source]).unwrap();

        let g = compiler.symbol_table.global_scope;
        let derived_id = compiler.symbol_table.lookup_class(g, "Derived").unwrap();
        let derived = compiler.symbol_table.classes.get(derived_id.0);

        assert_eq!(derived.field_start_type_count.int, 1, "Derived int 字段起始应为 1");
        assert_eq!(derived.field_start_type_count.float, 1, "Derived float 字段起始应为 1");
        assert_eq!(derived.field_type_count_total.int, 2, "int 字段总数应为 2");
        assert_eq!(derived.field_type_count_total.float, 2, "float 字段总数应为 2");
    }

    /// 解析含构造函数的源码 → pass1-3 构造任务验证
    /// 注：pass4 代码生成（this.field 支持）留到步骤 5
    #[test]
    fn test_constructor_parse_to_compile_task() {
        let source_text = r#"
class Point {
    int x;
    int y;
    Point(int x, int y) {
        this.x = x;
        this.y = y;
    }
}
"#;
        let (tokens, _) = crate::lexer::tokenize(source_text, 0);
        let mut parser = crate::parser::Parser::new(tokens);
        let source_file = parser.parse_source_file().unwrap();

        let mut compiler = Compiler::new();
        compiler.pass1_type_identifier(&[source_file.clone()]).unwrap();
        compiler.pass2_type_extension(&[source_file.clone()]).unwrap();
        compiler.pass3_type_declaration(&[source_file]).unwrap();

        let class_id = compiler.symbol_table
            .lookup_class(compiler.symbol_table.global_scope, "Point")
            .unwrap();
        let class_info = compiler.symbol_table.classes.get(class_id.0);
        assert_eq!(class_info.constructors.len(), 1, "应有 1 个构造函数");

        let ctor_tasks: Vec<_> = compiler.tasks.iter()
            .filter(|t| matches!(t.kind, TaskKind::Constructor { .. }))
            .collect();
        assert_eq!(ctor_tasks.len(), 1, "应有 1 个构造编译任务");

        let ctor_id = class_info.constructors[0];
        let ctor_info = compiler.symbol_table.constructors.get(ctor_id.0);
        assert_eq!(ctor_info.parameters.len(), 2, "构造函数应有 2 个参数");
    }

    /// 含 super(args) 的构造函数 → pass1-3 完整验证
    #[test]
    fn test_constructor_with_super_compile() {
        let source_text = r#"
class Animal {
    int age;
    Animal(int a) { age = a; }
}
class Dog : Animal {
    int weight;
    Dog(int w) : super(w) { weight = w; }
}
"#;
        let (tokens, _) = crate::lexer::tokenize(source_text, 0);
        let mut parser = crate::parser::Parser::new(tokens);
        let source_file = parser.parse_source_file().unwrap();

        assert_eq!(source_file.members.len(), 2, "应有两个类");

        match &source_file.members[1] {
            TopLevelMember::Class(dog) => {
                assert_eq!(dog.name, "Dog");
                assert_eq!(dog.super_class.as_ref().map(|t| type_ref_name(t)), Some("Animal".into()));

                let ctors: Vec<_> = dog.members.iter()
                    .filter_map(|m| if let ClassMember::Constructor(c) = m { Some(c) } else { None })
                    .collect();
                assert_eq!(ctors.len(), 1);
                assert_eq!(ctors[0].base_arguments.len(), 1, "super(w) 应有 1 个参数");

                match &ctors[0].base_arguments[0] {
                    Expression::Identifier(name, _) => assert_eq!(name, "w"),
                    _ => panic!("super 参数应为标识符 w"),
                }
            }
            _ => panic!("Dog 应为类声明"),
        }

        let mut compiler = Compiler::new();
        compiler.pass1_type_identifier(&[source_file.clone()]).unwrap();
        compiler.pass2_type_extension(&[source_file.clone()]).unwrap();
        compiler.pass3_type_declaration(&[source_file]).unwrap();

        let dog_id = compiler.symbol_table
            .lookup_class(compiler.symbol_table.global_scope, "Dog")
            .unwrap();
        let dog_info = compiler.symbol_table.classes.get(dog_id.0);
        assert!(dog_info.super_class.is_some(), "Dog 应有父类");
        let animal_id = dog_info.super_class.unwrap();
        assert_eq!(compiler.symbol_table.classes.get(animal_id.0).name, "Animal");

        let animal_id2 = compiler.symbol_table
            .lookup_class(compiler.symbol_table.global_scope, "Animal")
            .unwrap();
        let animal_info = compiler.symbol_table.classes.get(animal_id2.0);
        assert_eq!(animal_info.constructors.len(), 1);

        let all_ctor_tasks: Vec<_> = compiler.tasks.iter()
            .filter(|t| matches!(t.kind, TaskKind::Constructor { .. }))
            .collect();
        assert_eq!(all_ctor_tasks.len(), 2, "Animal + Dog 共 2 个构造任务");
    }
}
