#![allow(dead_code)]

use std::collections::HashMap;

use gorge_core::diagnostics::Span;

use crate::ast;

/// 类型安全的标识符定义宏
///
/// 为每种符号类型生成独立的 newtype ID，防止不同类型 ID 混用。
macro_rules! define_id {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub usize);
    };
}

define_id!(NamespaceId, "命名空间 ID");
define_id!(ClassId, "类 ID");
define_id!(InterfaceId, "接口 ID");
define_id!(EnumId, "枚举 ID");
define_id!(EnumValueId, "枚举值 ID");
define_id!(FieldId, "字段 ID");
define_id!(MethodId, "方法 ID");
define_id!(ConstructorId, "构造方法 ID");
define_id!(InjectorId, "注入器 ID");
define_id!(ParameterId, "参数 ID");
define_id!(AnnotationId, "注解 ID");
define_id!(ScopeId, "作用域 ID");
define_id!(LocalVarId, "局部变量 ID");

// ==================== Arena ====================

/// 符号存储容器
///
/// 类似 ECS 的组件存储，通过 ID 索引访问。比 `Vec` 更语义化，
/// 未来可以扩展为带有空的槽位、支持删除等操作。
#[derive(Debug)]
pub struct Arena<T> {
    items: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// 分配一个新元素，返回其索引
    pub fn alloc(&mut self, item: T) -> usize {
        let id = self.items.len();
        self.items.push(item);
        id
    }

    /// 按索引引用元素
    pub fn get(&self, id: usize) -> &T {
        &self.items[id]
    }

    /// 按索引可变引用元素
    pub fn get_mut(&mut self, id: usize) -> &mut T {
        &mut self.items[id]
    }

    /// 元素数量
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 迭代所有元素
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== 符号信息结构 ====================

/// 命名空间信息
#[derive(Debug, Clone)]
pub struct NamespaceInfo {
    pub name: String,
    pub scope_id: ScopeId,
    /// 在该命名空间中声明的类 ID 列表
    pub classes: Vec<ClassId>,
    pub interfaces: Vec<InterfaceId>,
    pub enums: Vec<EnumId>,
    pub using_scopes: Vec<ScopeId>,
}

/// 类信息
#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub namespace_scope_id: ScopeId,
    pub name: String,
    pub scope_id: ScopeId,
    /// 父类（`extends`）
    pub super_class: Option<ClassId>,
    /// 实现的接口
    pub super_interfaces: Vec<InterfaceId>,
    /// 修饰符
    pub is_native: bool,
    pub is_static: bool,
    pub is_abstract: bool,
    /// 成员
    pub fields: Vec<FieldId>,
    pub methods: Vec<MethodId>,
    pub constructors: Vec<ConstructorId>,
    pub injector: Option<InjectorId>,
    pub annotations: Vec<AnnotationId>,
    /// 来源 span
    pub span: Span,
    /// 本类方法（静态+实例混合）的全局起始编号 = 父类的 method_count_total
    ///
    /// 继承编号冻结（B-3）后填充。本类第 i 个方法的全局 ID = method_start_id + i。
    pub method_start_id: usize,
    /// 含继承的方法总数 = method_start_id + 本类方法数
    pub method_count_total: usize,
    /// 本类构造方法的全局起始编号 = 父类的 constructor_count_total
    pub constructor_start_id: usize,
    /// 含继承的构造方法总数
    pub constructor_count_total: usize,
    /// 重写映射：被重写的父类方法全局 ID → 本类重写方法全局 ID
    pub method_override_id: std::collections::HashMap<usize, usize>,
    /// 本类实例字段各值类型的起始索引（= 父类的 field_type_count_total）
    pub field_start_type_count: crate::symbol::FrozenTypeCount,
    /// 含继承的实例字段各值类型总数
    pub field_type_count_total: crate::symbol::FrozenTypeCount,
    /// 接口方法实现映射：接口全名 → [接口方法本地ID → 类实现方法全局ID]（F1）
    pub interface_method_impl_id: std::collections::HashMap<String, Vec<usize>>,
    /// 声明冻结（K2）：成员声明完成，不允许再添加新成员
    pub declaration_frozen: bool,
    /// 继承冻结（K2）：继承链已固定
    pub inheritance_frozen: bool,
    /// 泛型参数名列表 `class Foo<T, U>`（J1）
    pub generic_params: Vec<String>,
}

impl ClassInfo {
    /// 确保声明尚未冻结（调用前检查，对齐 C# `EnsureDeclarationNotFreeze`）。
    /// 若已冻结则返回错误信息，否则返回 Ok。
    pub fn check_declaration_not_frozen(&self) -> Result<(), String> {
        if self.declaration_frozen {
            Err(format!("类 `{}` 的声明已冻结，不允许再添加成员", self.name))
        } else {
            Ok(())
        }
    }

    /// 确保继承尚未冻结（修改继承关系前检查，对齐 C# `EnsureInheritanceNotFreeze`）。
    pub fn check_inheritance_not_frozen(&self) -> Result<(), String> {
        if self.inheritance_frozen {
            Err(format!("类 `{}` 的继承关系已冻结，不允许修改", self.name))
        } else {
            Ok(())
        }
    }
}

/// 按值类型分组的计数（编译器内部用，避免依赖 gorge_core）
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FrozenTypeCount {
    pub int: usize,
    pub float: usize,
    pub bool: usize,
    pub string: usize,
    pub object: usize,
}

/// 接口信息
#[derive(Debug, Clone)]
pub struct InterfaceInfo {
    pub namespace_scope_id: ScopeId,
    pub name: String,
    pub scope_id: ScopeId,
    pub super_interfaces: Vec<InterfaceId>,
    pub methods: Vec<MethodId>,
    pub span: Span,
}

/// 枚举信息
#[derive(Debug, Clone)]
pub struct EnumInfo {
    pub namespace_scope_id: ScopeId,
    pub name: String,
    pub scope_id: ScopeId,
    pub values: Vec<EnumValueId>,
    pub span: Span,
}

/// 枚举值信息
#[derive(Debug, Clone)]
pub struct EnumValueInfo {
    pub name: String,
    pub enum_id: EnumId,
    pub value: Option<i64>,
    pub span: Span,
}

/// 字段信息
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub class_id: ClassId,
    pub field_type: TypeInfo,
    pub is_static: bool,
    pub is_native: bool,
    /// 在对象内存布局中的偏移（编译后确定）
    pub offset: Option<usize>,
    pub span: Span,
}

/// 方法信息
#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub class_id: Option<ClassId>,
    pub interface_id: Option<InterfaceId>,
    pub return_type: TypeInfo,
    pub parameters: Vec<ParameterId>,
    pub is_static: bool,
    pub is_native: bool,
    pub is_override: bool,
    pub is_abstract: bool,
    /// 方法体对应的作用域（如果有实现的话）
    pub body_scope_id: Option<ScopeId>,
    pub span: Span,
}

/// 构造方法信息
#[derive(Debug, Clone)]
pub struct ConstructorInfo {
    pub class_id: ClassId,
    pub parameters: Vec<ParameterId>,
    pub is_native: bool,
    pub body_scope_id: Option<ScopeId>,
    pub span: Span,
}

/// 注入器信息
#[derive(Debug, Clone)]
pub struct InjectorInfo {
    pub class_id: ClassId,
    pub fields: Vec<InjectorFieldInfo>,
    pub span: Span,
}

/// 注入器字段信息
#[derive(Debug, Clone)]
pub struct InjectorFieldInfo {
    pub name: String,
    pub field_type: TypeInfo,
    pub span: Span,
}

/// 参数信息
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub param_type: TypeInfo,
    pub owner_method_id: Option<MethodId>,
    pub owner_constructor_id: Option<ConstructorId>,
    pub index: usize,
    pub span: Span,
}

/// 注解信息
#[derive(Debug, Clone)]
pub struct AnnotationInfo {
    pub name: String,
    pub fields: Vec<FieldId>,
    pub span: Span,
}

/// 局部变量信息
#[derive(Debug, Clone)]
pub struct LocalVarInfo {
    pub name: String,
    pub var_type: TypeInfo,
    /// 在栈帧中的偏移
    pub offset: usize,
    pub span: Span,
}

// ==================== 类型信息 ====================

/// 编译时类型信息
///
/// 区别于 AST 中的 `TypeRef`（语法层的类型引用），
/// `TypeInfo` 是符号表解析后的具体类型信息。
#[derive(Debug, Clone, PartialEq)]
pub enum TypeInfo {
    /// 基本类型
    Int,
    Float,
    Bool,
    String,
    /// void 类型
    Void,
    /// 对象类型（指向具体类）
    Object(ClassId),
    /// 接口类型
    Interface(InterfaceId),
    /// 委托类型
    Delegate {
        return_type: Box<TypeInfo>,
        param_types: Vec<TypeInfo>,
    },
    /// 枚举类型
    Enum(EnumId),
    /// 泛型参数占位（尚未解析）
    GenericParam(String),
    /// 泛型实例化后的类型
    GenericInstance {
        base: Box<TypeInfo>,
        type_args: Vec<TypeInfo>,
    },
    /// 数组类型
    Array(Box<TypeInfo>),
    /// 尚未解析（var 等）
    Unresolved,
}

impl TypeInfo {
    /// 从基本类型关键字创建 TypeInfo
    pub fn from_keyword(name: &str) -> Option<Self> {
        match name {
            "int" => Some(TypeInfo::Int),
            "float" => Some(TypeInfo::Float),
            "bool" => Some(TypeInfo::Bool),
            "string" => Some(TypeInfo::String),
            "void" => Some(TypeInfo::Void),
            "object" => Some(TypeInfo::Unresolved), // object 在运行时解析为 GorgeObject
            _ => None,
        }
    }

    /// 判断是否可以自动（隐式）转换为目标类型
    ///
    /// 基本类型转换规则（参考 C# 版 SymbolicGorgeType.CanAutoCastTo）：
    /// - Int → Float（精度无损扩展）
    /// - Enum → Int（枚举值可作为整数使用）
    /// - Null → 任意 Object/Interface/Delegate/String 类型
    /// 类继承/接口实现的转换需要在 SymbolTable 中查询层次关系。
    pub fn can_auto_cast_to(&self, target: &TypeInfo) -> bool {
        match (self, target) {
            // Int 可隐式转为 Float
            (TypeInfo::Int, TypeInfo::Float) => true,
            // 相同类型当然可以
            (a, b) if a == b => true,
            // Unresolved 宽松处理
            (TypeInfo::Unresolved, _) | (_, TypeInfo::Unresolved) => true,
            _ => false,
        }
    }

    /// 判断是否可以强制转换为目标类型
    ///
    /// 强制转换 = 自动转换 ∪ 反向自动转换（参考 C# 版 SymbolicGorgeType.CanCastTo）
    pub fn can_cast_to(&self, target: &TypeInfo) -> bool {
        self.can_auto_cast_to(target) || target.can_auto_cast_to(self)
    }
}

// ==================== Scope 作用域体系 ====================

/// 作用域
///
/// 编译器使用嵌套的 Scope 树来管理符号的可见性。
/// 每个 Scope 维护一个从名称到符号的映射表。
#[derive(Debug, Clone)]
pub struct Scope {
    pub kind: ScopeKind,
    pub parent: Option<ScopeId>,
    pub children: Vec<ScopeId>,
    /// 本地符号表（名称 → 符号标识）
    pub symbols: HashMap<String, SymbolEntry>,
    pub using_scopes: Vec<ScopeId>,
}

/// 作用域类型
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    /// 全局作用域（根）
    Global,
    /// 命名空间作用域
    Namespace { name: String },
    /// 类作用域
    Class { class_id: ClassId },
    /// 接口作用域
    Interface { interface_id: InterfaceId },
    /// 枚举作用域
    Enum { enum_id: EnumId },
    /// 方法作用域
    Method { method_id: MethodId },
    /// 构造方法作用域
    Constructor { constructor_id: ConstructorId },
    /// 注入器作用域
    Injector { injector_id: InjectorId },
    /// 注解作用域
    Annotation { annotation_id: AnnotationId },
    /// 代码块作用域（方法体内部的块）
    CodeBlock { context: BlockContext },
}

/// 代码块上下文类型
///
/// 影响变量查找策略和 this 引用的解析方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockContext {
    /// 常量求值上下文
    Constant,
    /// 静态方法内部
    StaticMethod,
    /// 注入器构造上下文
    Injector,
    /// 字段初始化器
    FieldInitializer,
    /// 实例方法内部
    Instance,
}

/// 符号表中的条目
#[derive(Debug, Clone)]
pub enum SymbolEntry {
    Namespace(NamespaceId),
    Class(ClassId),
    Interface(InterfaceId),
    Enum(EnumId),
    EnumValue(EnumValueId),
    Field(FieldId),
    Method(Vec<MethodId>), // 支持重载，一组同名方法
    Constructor(Vec<ConstructorId>),
    Injector(InjectorId),
    Annotation(AnnotationId),
    LocalVar(LocalVarId),
    Parameter(ParameterId),
}

// ==================== 符号表主结构 ====================

/// Gorge 编译器符号表
///
/// 使用 Arena 模式存储所有符号，通过 ID 索引访问。
/// Scope 树管理符号的可见性和生命周期。
pub struct SymbolTable {
    // Arena 存储
    pub namespaces: Arena<NamespaceInfo>,
    pub classes: Arena<ClassInfo>,
    pub interfaces: Arena<InterfaceInfo>,
    pub enums: Arena<EnumInfo>,
    pub enum_values: Arena<EnumValueInfo>,
    pub fields: Arena<FieldInfo>,
    pub methods: Arena<MethodInfo>,
    pub constructors: Arena<ConstructorInfo>,
    pub injectors: Arena<InjectorInfo>,
    pub parameters: Arena<ParameterInfo>,
    pub annotations: Arena<AnnotationInfo>,
    pub local_vars: Arena<LocalVarInfo>,

    // Scope 树
    pub scopes: Arena<Scope>,
    pub global_scope: ScopeId,
}

impl SymbolTable {
    /// 创建一个新的空符号表，包含全局作用域
    pub fn new() -> Self {
        let mut scopes = Arena::new();
        let global_scope_id = ScopeId(scopes.alloc(Scope {
            kind: ScopeKind::Global,
            parent: None,
            children: Vec::new(),
            symbols: HashMap::new(),
            using_scopes: Vec::new(),
        }));

        Self {
            namespaces: Arena::new(),
            classes: Arena::new(),
            interfaces: Arena::new(),
            enums: Arena::new(),
            enum_values: Arena::new(),
            fields: Arena::new(),
            methods: Arena::new(),
            constructors: Arena::new(),
            injectors: Arena::new(),
            parameters: Arena::new(),
            annotations: Arena::new(),
            local_vars: Arena::new(),
            scopes,
            global_scope: global_scope_id,
        }
    }

    // ==================== Scope 管理 ====================

    /// 在当前作用域下创建一个子作用域
    pub fn push_scope(&mut self, parent: ScopeId, kind: ScopeKind) -> ScopeId {
        let parent_using = self.scopes.get(parent.0).using_scopes.clone();
        let child_id = ScopeId(self.scopes.alloc(Scope {
            kind,
            parent: Some(parent),
            children: Vec::new(),
            symbols: HashMap::new(),
            using_scopes: parent_using,
        }));
        self.scopes.get_mut(parent.0).children.push(child_id);
        child_id
    }

    /// 在指定作用域中注册一个符号
    pub fn define_symbol(&mut self, scope_id: ScopeId, name: &str, entry: SymbolEntry) {
        let scope = self.scopes.get_mut(scope_id.0);
        scope.symbols.insert(name.to_string(), entry);
    }

    /// 仅在本级 + Parent 链查找（不查 using）
    pub fn lookup_local_only(&self, scope_id: ScopeId, name: &str) -> Option<&SymbolEntry> {
        let mut current = Some(scope_id);
        while let Some(sid) = current {
            let scope = self.scopes.get(sid.0);
            if let Some(entry) = scope.symbols.get(name) {
                return Some(entry);
            }
            current = scope.parent;
        }
        None
    }

    /// 在作用域链中查找符号（三级：本级 Symbols → 向上 Parent → 横向 Usings）
    ///
    /// 横向 usings 查找不递归查 using 的 using，避免循环引用。
    pub fn lookup(&self, scope_id: ScopeId, name: &str) -> Option<(&SymbolEntry, ScopeId)> {
        let scope = self.scopes.get(scope_id.0);

        // 1. 本级 Symbols 查找
        if let Some(entry) = scope.symbols.get(name) {
            return Some((entry, scope_id));
        }

        // 2. 向上 Parent 查找
        if let Some(parent_id) = scope.parent {
            if let result @ Some(_) = self.lookup(parent_id, name) {
                return result;
            }
        }

        // 3. 横向 Usings 查找（不递归查 using 的 using）
        for &using_scope_id in &scope.using_scopes {
            if let Some(entry) = self.lookup_local_only(using_scope_id, name) {
                return Some((entry, using_scope_id));
            }
        }

        None
    }

    /// 仅在指定作用域中查找（不搜索父作用域）
    pub fn lookup_local(&self, scope_id: ScopeId, name: &str) -> Option<&SymbolEntry> {
        self.scopes.get(scope_id.0).symbols.get(name)
    }

    /// 获取作用域的父作用域
    pub fn parent_scope(&self, scope_id: ScopeId) -> Option<ScopeId> {
        self.scopes.get(scope_id.0).parent
    }

    /// 向上查找第一个满足条件的 Scope
    ///
    /// 用于找到"所属的类"或"所属的方法"等语义上下文。
    pub fn find_ancestor_scope<F>(&self, scope_id: ScopeId, predicate: F) -> Option<ScopeId>
    where
        F: Fn(&ScopeKind) -> bool,
    {
        let mut current = Some(scope_id);
        while let Some(sid) = current {
            let scope = self.scopes.get(sid.0);
            if predicate(&scope.kind) {
                return Some(sid);
            }
            current = scope.parent;
        }
        None
    }

    /// 查找包含当前作用域的类作用域
    pub fn enclosing_class_scope(&self, scope_id: ScopeId) -> Option<ScopeId> {
        self.find_ancestor_scope(scope_id, |kind| matches!(kind, ScopeKind::Class { .. }))
    }

    /// 查找包含当前作用域的方法作用域
    pub fn enclosing_method_scope(&self, scope_id: ScopeId) -> Option<ScopeId> {
        self.find_ancestor_scope(scope_id, |kind| matches!(kind, ScopeKind::Method { .. }))
    }

    /// 获取作用域的类型
    pub fn scope_kind(&self, scope_id: ScopeId) -> &ScopeKind {
        &self.scopes.get(scope_id.0).kind
    }

    // ==================== 符号声明快捷方法 ====================

    /// 声明一个命名空间
    ///
    /// 创建 NamespaceScope 作为指定父作用域的子节点。
    pub fn declare_namespace(
        &mut self,
        name: &str,
        parent_scope_id: ScopeId,
    ) -> NamespaceId {
        let ns_scope_id = self.push_scope(
            parent_scope_id,
            ScopeKind::Namespace { name: name.to_string() },
        );
        let ns_id = NamespaceId(self.namespaces.alloc(NamespaceInfo {
            name: name.to_string(),
            scope_id: ns_scope_id,
            classes: Vec::new(),
            interfaces: Vec::new(),
            enums: Vec::new(),
            using_scopes: self.scopes.get(ns_scope_id.0).using_scopes.clone(),
        }));

        self.define_symbol(parent_scope_id, name, SymbolEntry::Namespace(ns_id));
        ns_id
    }

    /// 声明一个类
    pub fn declare_class(
        &mut self,
        name: &str,
        scope_id: ScopeId,
        super_class: Option<ClassId>,
        super_interfaces: Vec<InterfaceId>,
        is_native: bool,
        span: Span,
    ) -> ClassId {
        let class_scope_id = self.push_scope(
            scope_id,
            ScopeKind::Class { class_id: ClassId(usize::MAX) },
        );
        let class_id = ClassId(self.classes.alloc(ClassInfo {
            namespace_scope_id: scope_id,
            name: name.to_string(),
            scope_id: class_scope_id,
            super_class,
            super_interfaces,
            is_native,
            is_static: false,
            is_abstract: false,
            fields: Vec::new(),
            methods: Vec::new(),
            constructors: Vec::new(),
            injector: None,
            annotations: Vec::new(),
            span,
            method_start_id: 0,
            method_count_total: 0,
            constructor_start_id: 0,
            constructor_count_total: 0,
            method_override_id: std::collections::HashMap::new(),
            field_start_type_count: FrozenTypeCount::default(),
            field_type_count_total: FrozenTypeCount::default(),
            interface_method_impl_id: std::collections::HashMap::new(),
            declaration_frozen: false,
            inheritance_frozen: false,
            generic_params: Vec::new(),
        }));

        // 回填 class_id 到 Scope 中
        self.scopes.get_mut(class_scope_id.0).kind = ScopeKind::Class { class_id };

        // 将所属命名空间加入 class 的 using_scopes
        self.scopes.get_mut(class_scope_id.0).using_scopes.push(scope_id);

        self.define_symbol(scope_id, name, SymbolEntry::Class(class_id));
        class_id
    }

    /// 声明一个接口
    pub fn declare_interface(
        &mut self,
        name: &str,
        scope_id: ScopeId,
        super_interfaces: Vec<InterfaceId>,
        span: Span,
    ) -> InterfaceId {
        let iface_scope_id = self.push_scope(
            scope_id,
            ScopeKind::Interface { interface_id: InterfaceId(usize::MAX) },
        );
        let interface_id = InterfaceId(self.interfaces.alloc(InterfaceInfo {
            namespace_scope_id: scope_id,
            name: name.to_string(),
            scope_id: iface_scope_id,
            super_interfaces,
            methods: Vec::new(),
            span,
        }));

        self.scopes.get_mut(iface_scope_id.0).kind =
            ScopeKind::Interface { interface_id };

        // 将所属命名空间加入 interface 的 using_scopes
        self.scopes.get_mut(iface_scope_id.0).using_scopes.push(scope_id);

        self.define_symbol(scope_id, name, SymbolEntry::Interface(interface_id));
        interface_id
    }

    /// 声明一个枚举
    pub fn declare_enum(
        &mut self,
        name: &str,
        scope_id: ScopeId,
        span: Span,
    ) -> EnumId {
        let enum_scope_id = self.push_scope(
            scope_id,
            ScopeKind::Enum { enum_id: EnumId(usize::MAX) },
        );
        let enum_id = EnumId(self.enums.alloc(EnumInfo {
            namespace_scope_id: scope_id,
            name: name.to_string(),
            scope_id: enum_scope_id,
            values: Vec::new(),
            span,
        }));

        self.scopes.get_mut(enum_scope_id.0).kind = ScopeKind::Enum { enum_id };

        // 将所属命名空间加入 enum 的 using_scopes
        self.scopes.get_mut(enum_scope_id.0).using_scopes.push(scope_id);

        self.define_symbol(scope_id, name, SymbolEntry::Enum(enum_id));
        enum_id
    }

    /// 声明一个枚举值
    ///
    /// 将枚举值注册到对应枚举的作用域中。
    pub fn declare_enum_value(
        &mut self,
        name: &str,
        enum_id: EnumId,
        value: Option<i64>,
        span: Span,
    ) -> EnumValueId {
        let enum_scope_id = self.enums.get(enum_id.0).scope_id;
        let val_id = EnumValueId(self.enum_values.alloc(EnumValueInfo {
            name: name.to_string(),
            enum_id,
            value,
            span,
        }));

        self.enums.get_mut(enum_id.0).values.push(val_id);
        self.define_symbol(enum_scope_id, name, SymbolEntry::EnumValue(val_id));
        val_id
    }

    /// 声明一个注解
    pub fn declare_annotation(
        &mut self,
        name: &str,
        scope_id: ScopeId,
        span: Span,
    ) -> AnnotationId {
        let annotation_scope_id = self.push_scope(
            scope_id,
            ScopeKind::Annotation { annotation_id: AnnotationId(usize::MAX) },
        );
        let annotation_id = AnnotationId(self.annotations.alloc(AnnotationInfo {
            name: name.to_string(),
            fields: Vec::new(),
            span,
        }));

        self.scopes.get_mut(annotation_scope_id.0).kind =
            ScopeKind::Annotation { annotation_id };

        self.define_symbol(scope_id, name, SymbolEntry::Annotation(annotation_id));
        annotation_id
    }

    /// 更新类的父类（在 Pass 2 中调用）
    pub fn set_super_class(&mut self, class_id: ClassId, super_class: ClassId) {
        self.classes.get_mut(class_id.0).super_class = Some(super_class);
    }

    /// 更新类实现的接口列表（在 Pass 2 中调用）
    pub fn set_super_interfaces(&mut self, class_id: ClassId, interfaces: Vec<InterfaceId>) {
        self.classes.get_mut(class_id.0).super_interfaces = interfaces;
    }

    /// 声明一个字段
    pub fn declare_field(
        &mut self,
        name: &str,
        class_id: ClassId,
        field_type: TypeInfo,
        is_static: bool,
        span: Span,
    ) -> FieldId {
        let field_id = FieldId(self.fields.alloc(FieldInfo {
            name: name.to_string(),
            class_id,
            field_type,
            is_static,
            is_native: false,
            offset: None,
            span,
        }));

        self.classes.get_mut(class_id.0).fields.push(field_id);
        let class_scope = self.classes.get(class_id.0).scope_id;
        self.define_symbol(class_scope, name, SymbolEntry::Field(field_id));

        field_id
    }

    /// 声明一个方法
    pub fn declare_method(
        &mut self,
        name: &str,
        class_id: Option<ClassId>,
        interface_id: Option<InterfaceId>,
        return_type: TypeInfo,
        parameters: Vec<ParameterId>,
        is_static: bool,
        is_native: bool,
        span: Span,
    ) -> MethodId {
        let method_id = MethodId(self.methods.alloc(MethodInfo {
            name: name.to_string(),
            class_id,
            interface_id,
            return_type,
            parameters,
            is_static,
            is_native,
            is_override: false,
            is_abstract: false,
            body_scope_id: None,
            span,
        }));

        // 注册到对应的类或接口
        if let Some(cid) = class_id {
            let class_scope = self.classes.get(cid.0).scope_id;
            self.classes.get_mut(cid.0).methods.push(method_id);

            // 方法以 Vec<MethodId> 存储，支持重载
            let entry = self.scopes.get(class_scope.0)
                .symbols
                .get(name)
                .cloned();

            match entry {
                Some(SymbolEntry::Method(mut existing)) => {
                    existing.push(method_id);
                    self.scopes.get_mut(class_scope.0)
                        .symbols
                        .insert(name.to_string(), SymbolEntry::Method(existing));
                }
                _ => {
                    self.define_symbol(class_scope, name, SymbolEntry::Method(vec![method_id]));
                }
            }
        } else if let Some(iid) = interface_id {
            let iface_scope = self.interfaces.get(iid.0).scope_id;
            self.interfaces.get_mut(iid.0).methods.push(method_id);
            self.define_symbol(iface_scope, name, SymbolEntry::Method(vec![method_id]));
        }

        method_id
    }

    /// 声明一个构造方法
    ///
    /// 构造方法以 `constructor` 为名注册，支持重载。
    pub fn declare_constructor(
        &mut self,
        class_id: ClassId,
        parameters: Vec<ParameterId>,
        is_native: bool,
        span: Span,
    ) -> ConstructorId {
        let constructor_id = ConstructorId(self.constructors.alloc(ConstructorInfo {
            class_id,
            parameters,
            is_native,
            body_scope_id: None,
            span,
        }));

        let class_scope = self.classes.get(class_id.0).scope_id;
        self.classes.get_mut(class_id.0).constructors.push(constructor_id);

        let ctor_name = "constructor";
        let entry = self.scopes.get(class_scope.0)
            .symbols
            .get(ctor_name)
            .cloned();

        match entry {
            Some(SymbolEntry::Constructor(mut existing)) => {
                existing.push(constructor_id);
                self.scopes.get_mut(class_scope.0)
                    .symbols
                    .insert(ctor_name.to_string(), SymbolEntry::Constructor(existing));
            }
            _ => {
                self.define_symbol(class_scope, ctor_name, SymbolEntry::Constructor(vec![constructor_id]));
            }
        }

        constructor_id
    }

    /// 为字段分配内存偏移
    ///
    /// 在对象布局确定后，为每个字段设置其在实例数据中的偏移位置。
    pub fn allocate_field_offset(&mut self, field_id: FieldId, offset: usize) {
        self.fields.get_mut(field_id.0).offset = Some(offset);
    }

    /// 设置方法的方法体作用域
    pub fn set_method_body_scope(&mut self, method_id: MethodId, scope_id: ScopeId) {
        self.methods.get_mut(method_id.0).body_scope_id = Some(scope_id);
    }

    /// 设置构造方法的方法体作用域
    pub fn set_constructor_body_scope(&mut self, constructor_id: ConstructorId, scope_id: ScopeId) {
        self.constructors.get_mut(constructor_id.0).body_scope_id = Some(scope_id);
    }

    /// 声明一个参数
    pub fn declare_parameter(
        &mut self,
        name: &str,
        param_type: TypeInfo,
        index: usize,
        span: Span,
    ) -> ParameterId {
        let param_id = ParameterId(self.parameters.alloc(ParameterInfo {
            name: name.to_string(),
            param_type,
            owner_method_id: None,
            owner_constructor_id: None,
            index,
            span,
        }));

        param_id
    }

    /// 声明一个局部变量并分配栈偏移
    pub fn declare_local_var(
        &mut self,
        scope_id: ScopeId,
        name: &str,
        var_type: TypeInfo,
        offset: usize,
        span: Span,
    ) -> LocalVarId {
        let var_id = LocalVarId(self.local_vars.alloc(LocalVarInfo {
            name: name.to_string(),
            var_type,
            offset,
            span,
        }));

        self.define_symbol(scope_id, name, SymbolEntry::LocalVar(var_id));
        var_id
    }

    // ==================== 查询方法 ====================

    /// 根据名称在给定作用域中查找类
    pub fn lookup_class(&self, scope_id: ScopeId, name: &str) -> Option<ClassId> {
        match self.lookup(scope_id, name) {
            Some((SymbolEntry::Class(id), _)) => Some(*id),
            _ => None,
        }
    }

    /// 根据名称在给定作用域中查找接口
    pub fn lookup_interface(&self, scope_id: ScopeId, name: &str) -> Option<InterfaceId> {
        match self.lookup(scope_id, name) {
            Some((SymbolEntry::Interface(id), _)) => Some(*id),
            _ => None,
        }
    }

    /// 根据名称在给定作用域中查找枚举
    pub fn find_enum_by_name(&self, scope: ScopeId, name: &str) -> Option<EnumId> {
        match self.lookup(scope, name) {
            Some((SymbolEntry::Enum(id), _)) => Some(*id),
            _ => None,
        }
    }

    /// 根据名称在给定作用域中查找字段
    pub fn lookup_field(&self, scope_id: ScopeId, name: &str) -> Option<FieldId> {
        match self.lookup(scope_id, name) {
            Some((SymbolEntry::Field(id), _)) => Some(*id),
            _ => None,
        }
    }

    /// 根据名称在给定作用域中查找方法（返回所有重载）
    pub fn lookup_method(&self, scope_id: ScopeId, name: &str) -> Vec<MethodId> {
        match self.lookup(scope_id, name) {
            Some((SymbolEntry::Method(ids), _)) => ids.clone(),
            _ => Vec::new(),
        }
    }

    /// 根据名称在给定作用域中查找局部变量
    pub fn lookup_local_var(&self, scope_id: ScopeId, name: &str) -> Option<LocalVarId> {
        match self.lookup(scope_id, name) {
            Some((SymbolEntry::LocalVar(id), _)) => Some(*id),
            _ => None,
        }
    }

    /// 解析 AST 中的类型引用为 TypeInfo
    ///
    /// 在给定作用域中查找类型名，返回解析后的 TypeInfo。
    pub fn resolve_type(&self, scope_id: ScopeId, type_ref: &ast::TypeRef) -> Option<TypeInfo> {
        match type_ref {
            ast::TypeRef::Simple { name, .. } => {
                // 先检查基本类型
                if let Some(ti) = TypeInfo::from_keyword(name) {
                    return Some(ti);
                }
                // 再查找类、接口、枚举
                if let Some(class_id) = self.lookup_class(scope_id, name) {
                    return Some(TypeInfo::Object(class_id));
                }
                if let Some(iface_id) = self.lookup_interface(scope_id, name) {
                    return Some(TypeInfo::Interface(iface_id));
                }
                None
            }
            ast::TypeRef::Array { element_type, .. } => {
                let inner = self.resolve_type(scope_id, element_type)?;
                Some(TypeInfo::Array(Box::new(inner)))
            }
            ast::TypeRef::Delegate { return_type, param_types, .. } => {
                let ret = self.resolve_type(scope_id, return_type)?;
                let params: Vec<TypeInfo> = param_types.iter()
                    .filter_map(|p| self.resolve_type(scope_id, p))
                    .collect();
                if params.len() != param_types.len() {
                    return None;
                }
                Some(TypeInfo::Delegate {
                    return_type: Box::new(ret),
                    param_types: params,
                })
            }
            ast::TypeRef::Generic { name, type_args, .. } => {
                // 解析基础类型和泛型参数
                let base = match TypeInfo::from_keyword(name) {
                    Some(ti) => ti,
                    None => {
                        let class_id = self.lookup_class(scope_id, name)?;
                        TypeInfo::Object(class_id)
                    }
                };
                let resolved_args: Vec<TypeInfo> = type_args
                    .iter()
                    .map(|ta| self.resolve_type(scope_id, ta))
                    .collect::<Option<_>>()?;
                Some(TypeInfo::GenericInstance {
                    base: Box::new(base),
                    type_args: resolved_args,
                })
            }
            ast::TypeRef::Injector { base_type, .. } => {
                self.resolve_type(scope_id, base_type)
            }
        }
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_span() -> Span {
        Span::new(0, 1, 1, 1, 0)
    }

    #[test]
    fn test_create_symbol_table() {
        let st = SymbolTable::new();
        assert_eq!(st.scopes.len(), 1); // 只有全局作用域
    }

    #[test]
    fn test_push_scope() {
        let mut st = SymbolTable::new();
        let ns_scope = st.push_scope(
            st.global_scope,
            ScopeKind::Namespace { name: "Test".into() },
        );
        assert_eq!(st.scopes.len(), 2);
        assert!(st.scopes.get(st.global_scope.0).children.contains(&ns_scope));
    }

    #[test]
    fn test_declare_and_lookup_class() {
        let mut st = SymbolTable::new();
        let ns_scope = st.push_scope(
            st.global_scope,
            ScopeKind::Namespace { name: "Test".into() },
        );
        let class_id = st.declare_class("MyClass", ns_scope, None, vec![], false, dummy_span());
        assert_eq!(st.classes.get(class_id.0).name, "MyClass");

        let found = st.lookup_class(ns_scope, "MyClass");
        assert_eq!(found, Some(class_id));
    }

    #[test]
    fn test_declare_and_lookup_field() {
        let mut st = SymbolTable::new();
        let class_id = st.declare_class("Point", st.global_scope, None, vec![], false, dummy_span());
        let field_id = st.declare_field("x", class_id, TypeInfo::Int, false, dummy_span());

        let class_scope = st.classes.get(class_id.0).scope_id;
        let found = st.lookup_field(class_scope, "x");
        assert_eq!(found, Some(field_id));
    }

    #[test]
    fn test_declare_and_lookup_method() {
        let mut st = SymbolTable::new();
        let class_id = st.declare_class("Math", st.global_scope, None, vec![], false, dummy_span());
        let param = st.declare_parameter("n", TypeInfo::Int, 0, dummy_span());
        let method_id = st.declare_method(
            "abs",
            Some(class_id),
            None,
            TypeInfo::Int,
            vec![param],
            false,
            false,
            dummy_span(),
        );

        let class_scope = st.classes.get(class_id.0).scope_id;
        let found = st.lookup_method(class_scope, "abs");
        assert_eq!(found, vec![method_id]);
    }

    #[test]
    fn test_method_overloading() {
        let mut st = SymbolTable::new();
        let class_id = st.declare_class("Printer", st.global_scope, None, vec![], false, dummy_span());

        let m1 = st.declare_method(
            "print",
            Some(class_id),
            None,
            TypeInfo::Void,
            vec![],
            false,
            false,
            dummy_span(),
        );
        let param = st.declare_parameter("msg", TypeInfo::String, 0, dummy_span());
        let m2 = st.declare_method(
            "print",
            Some(class_id),
            None,
            TypeInfo::Void,
            vec![param],
            false,
            false,
            dummy_span(),
        );

        let class_scope = st.classes.get(class_id.0).scope_id;
        let found = st.lookup_method(class_scope, "print");
        assert_eq!(found.len(), 2);
        assert!(found.contains(&m1));
        assert!(found.contains(&m2));
    }

    #[test]
    fn test_scoped_lookup() {
        let mut st = SymbolTable::new();
        // 全局作用域声明类
        let class_id = st.declare_class("Outer", st.global_scope, None, vec![], false, dummy_span());
        let class_scope = st.classes.get(class_id.0).scope_id;

        // 类作用域内声明字段
        st.declare_field("value", class_id, TypeInfo::Int, false, dummy_span());

        // 方法作用域
        let method_scope = st.push_scope(
            class_scope,
            ScopeKind::CodeBlock { context: BlockContext::Instance },
        );
        st.declare_local_var(method_scope, "x", TypeInfo::Int, 0, dummy_span());

        // 在方法作用域中查找：能找到局部变量 x 和字段 value
        assert!(st.lookup_local_var(method_scope, "x").is_some());
        // value 字段需要在类作用域中查找（通过父作用域链）
        assert!(st.lookup_field(method_scope, "value").is_some());
    }

    #[test]
    fn test_resolve_basic_type() {
        let st = SymbolTable::new();
        let tref = ast::TypeRef::simple("int", dummy_span());
        let resolved = st.resolve_type(st.global_scope, &tref);
        assert_eq!(resolved, Some(TypeInfo::Int));
    }

    #[test]
    fn test_resolve_class_type() {
        let mut st = SymbolTable::new();
        st.declare_class("Vector", st.global_scope, None, vec![], false, dummy_span());

        let tref = ast::TypeRef::simple("Vector", dummy_span());
        let resolved = st.resolve_type(st.global_scope, &tref);
        assert!(matches!(resolved, Some(TypeInfo::Object(_))));
    }

    #[test]
    fn test_undeclared_type_returns_none() {
        let st = SymbolTable::new();
        let tref = ast::TypeRef::simple("UnknownType", dummy_span());
        let resolved = st.resolve_type(st.global_scope, &tref);
        assert_eq!(resolved, None);
    }

    #[test]
    fn test_enclosing_class_scope() {
        let mut st = SymbolTable::new();
        let class_id = st.declare_class("Container", st.global_scope, None, vec![], false, dummy_span());
        let class_scope = st.classes.get(class_id.0).scope_id;

        let method_scope = st.push_scope(
            class_scope,
            ScopeKind::Method { method_id: MethodId(0) },
        );
        let block_scope = st.push_scope(
            method_scope,
            ScopeKind::CodeBlock { context: BlockContext::Instance },
        );

        let found = st.enclosing_class_scope(block_scope);
        assert_eq!(found, Some(class_scope));
    }

    #[test]
    fn test_declare_interface_and_enum() {
        let mut st = SymbolTable::new();
        let iface_id = st.declare_interface("IComparable", st.global_scope, vec![], dummy_span());
        assert_eq!(st.interfaces.get(iface_id.0).name, "IComparable");

        let enum_id = st.declare_enum("Color", st.global_scope, dummy_span());
        assert_eq!(st.enums.get(enum_id.0).name, "Color");
    }
}
