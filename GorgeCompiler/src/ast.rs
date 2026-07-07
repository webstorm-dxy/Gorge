#![allow(dead_code)]

use gorge_core::diagnostics::Span;

// === 顶层结构 ===

/// 代表一个 Gorge 源文件，是 AST（抽象语法树）的根节点。
///
/// 每个源文件包含可选的命名空间声明、多个 `using` 导入指令、
/// 以及零个或多个顶层成员（类、接口、枚举、注解）。
///
/// # 字段
///
/// * `namespace` - 可选的命名空间声明，用于组织代码逻辑层级
/// * `usings` - 导入指令列表，等价于 C# 的 `using`，用于引入外部类型
/// * `members` - 源文件中定义的顶层成员（类、接口、枚举、注解）
/// * `span` - 该源文件在原始输入中的源代码位置信息，用于报错定位
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub namespace: Option<QualifiedName>,
    pub usings: Vec<UsingDirective>,
    pub members: Vec<TopLevelMember>,
    pub span: Span,
}

/// 限定名，由多个标识符段组成的路径，如 `System.Collections.Generic`。
///
/// 用于表示命名空间、类型全名等需要多级引用的场景。
///
/// # 字段
///
/// * `parts` - 名称的各段，按从左到右的顺序排列
/// * `span` - 该限定名在源代码中的位置信息
#[derive(Debug, Clone)]
pub struct QualifiedName {
    pub parts: Vec<String>,
    pub span: Span,
}

impl QualifiedName {
    /// 构造一个只有单段的简单限定名。
    ///
    /// 当名称仅由一个标识符组成（如 `List`）时使用此便捷构造方法。
    ///
    /// # 参数
    ///
    /// * `name` - 名称字符串，支持 `impl Into<String>`，可传入 `&str` 或 `String`
    /// * `span` - 源代码位置信息
    ///
    /// # 返回值
    ///
    /// 返回 `parts` 仅包含一个元素的 `QualifiedName`
    pub fn simple(name: impl Into<String>, span: Span) -> Self {
        Self { parts: vec![name.into()], span }
    }
}

/// 导入指令，对应 C# 中的 `using` 语句。
///
/// 用于引入其他命名空间或类型，使得当前文件可以直接使用其成员而无需完整限定名。
///
/// # 字段
///
/// * `name` - 导入的目标限定名
/// * `span` - 该指令在源代码中的位置信息
#[derive(Debug, Clone)]
pub struct UsingDirective {
    pub name: QualifiedName,
    pub span: Span,
}

/// 源文件的顶层成员，表示文件一级可以声明的四种构造。
///
/// 一个源文件可能包含若干类、接口、枚举或注解定义，它们之间可能相互引用。
#[derive(Debug, Clone)]
pub enum TopLevelMember {
    /// 类声明，包含字段、方法、构造函数等成员
    Class(ClassDeclaration),
    /// 接口声明，仅包含方法签名，不含实现
    Interface(InterfaceDeclaration),
    /// 枚举声明，定义一组命名常量
    Enum(EnumDeclaration),
}

// === 类声明 ===

/// 类声明，表示一个完整的类定义。
///
/// Gorge 中的类支持单继承（通过 `super_class`）和多接口实现（通过 `super_interfaces`），
/// 并可选地包含一个注入器（`injector`）用于依赖注入配置。
///
/// # 字段
///
/// * `annotations` - 应用于该类的注解列表（如 `@Serializable`）
/// * `modifiers` - 修饰符列表，控制访问级别和行为（`public`、`static`、`abstract` 等）
/// * `name` - 类名称
/// * `super_class` - 直接父类类型引用，`None` 表示无显式继承
/// * `super_interfaces` - 实现的接口列表
/// * `members` - 类成员：字段、方法、构造函数
/// * `injector` - 可选的依赖注入声明，定义该类需要注入的外部依赖
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct ClassDeclaration {
    pub annotations: Vec<Annotation>,
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub super_class: Option<TypeRef>,
    pub super_interfaces: Vec<TypeRef>,
    pub members: Vec<ClassMember>,
    pub injector: Option<InjectorDeclaration>,
    pub span: Span,
}

/// 类成员，表示类体中可以出现的三种声明。
#[derive(Debug, Clone)]
pub enum ClassMember {
    /// 字段声明（成员变量）
    Field(FieldDeclaration),
    /// 方法声明（成员函数）
    Method(MethodDeclaration),
    /// 构造函数声明
    Constructor(ConstructorDeclaration),
}

/// 字段声明，表示类中的一个成员变量。
///
/// 字段可以携带注解和修饰符，并可选地包含一个初始化表达式。
///
/// # 字段
///
/// * `annotations` - 应用于该字段的注解
/// * `modifiers` - 修饰符（如 `public`、`private`、`static`）
/// * `field_type` - 字段的类型引用
/// * `name` - 字段名称
/// * `initializer` - 可选的初始化表达式，`None` 表示声明时不赋值
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct FieldDeclaration {
    pub annotations: Vec<Annotation>,
    pub modifiers: Vec<Modifier>,
    pub field_type: TypeRef,
    pub name: String,
    pub initializer: Option<Expression>,
    pub span: Span,
}

/// 方法声明，表示类中的一个成员函数。
///
/// 方法可以携带注解和修饰符，并可选地包含一个方法体（`body`）。
/// 当 `body` 为 `None` 时，表示该方法只有签名而无实现（如抽象方法或接口方法）。
///
/// # 字段
///
/// * `annotations` - 应用于该方法的注解
/// * `modifiers` - 修饰符（如 `public`、`static`、`override`、`abstract`）
/// * `return_type` - 返回值类型，`void` 用特定类型引用表示
/// * `name` - 方法名称
/// * `parameters` - 形式参数列表
/// * `body` - 可选的方法体（语句列表），`None` 表示无实现
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct MethodDeclaration {
    pub annotations: Vec<Annotation>,
    pub modifiers: Vec<Modifier>,
    pub return_type: TypeRef,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Option<Vec<Statement>>,
    pub span: Span,
}

/// 方法签名，仅包含方法的声明信息，不含实现体。
///
/// 与方法声明（`MethodDeclaration`）的区别在于此类不包含 `modifiers` 和 `body` 字段，
/// 主要用于接口中声明方法以及跨文件引用方法签名。
///
/// # 字段
///
/// * `annotations` - 应用于该方法的注解
/// * `return_type` - 返回值类型
/// * `name` - 方法名称
/// * `parameters` - 形式参数列表
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub annotations: Vec<Annotation>,
    pub return_type: TypeRef,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub span: Span,
}

/// 构造函数声明，表示类的构造器。
///
/// 构造函数不返回值，可以携带参数，并通过 `base_arguments` 调用父类构造函数。
///
/// # 字段
///
/// * `annotations` - 应用于该构造函数的注解
/// * `modifiers` - 修饰符
/// * `parameters` - 形式参数列表
/// * `base_arguments` - 传递给父类构造函数的实际参数表达式列表
/// * `body` - 可选的构造函数体
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct ConstructorDeclaration {
    pub annotations: Vec<Annotation>,
    pub modifiers: Vec<Modifier>,
    pub parameters: Vec<Parameter>,
    pub base_arguments: Vec<Expression>,
    pub body: Option<Vec<Statement>>,
    pub span: Span,
}

// === 接口声明 ===

/// 接口声明，定义一组方法签名，供类实现。
///
/// Gorge 接口支持多重继承（`super_interfaces`），接口中的方法均为签名，
/// 不含实现体。
///
/// # 字段
///
/// * `annotations` - 应用于该接口的注解
/// * `modifiers` - 修饰符
/// * `name` - 接口名称
/// * `super_interfaces` - 继承的父接口列表
/// * `methods` - 接口中声明的方法签名列表
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct InterfaceDeclaration {
    pub annotations: Vec<Annotation>,
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub super_interfaces: Vec<TypeRef>,
    pub methods: Vec<MethodSignature>,
    pub span: Span,
}

// === 枚举声明 ===

/// 枚举声明，定义一组命名常量值。
///
/// Gorge 枚举的每个值可以显式指定一个整数值（`value` 字段），
/// 若为 `None` 则由编译器自动递增分配。
///
/// # 字段
///
/// * `annotations` - 应用于该枚举的注解
/// * `modifiers` - 修饰符
/// * `name` - 枚举名称
/// * `values` - 枚举值列表，每个值包含名称和可选的整数值
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct EnumDeclaration {
    pub annotations: Vec<Annotation>,
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub values: Vec<EnumValue>,
    pub span: Span,
}

/// 枚举值，表示枚举中的一个命名常量。
///
/// # 字段
///
/// * `annotations` - 应用于该枚举值的注解
/// * `name` - 枚举值名称
/// * `value` - 可选的显式整数值，`None` 表示由编译器自动分配
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct EnumValue {
    pub annotations: Vec<Annotation>,
    pub name: String,
    pub value: Option<i64>,
    pub span: Span,
}

// === 注解 ===

/// 注解字段，表示注解声明中的一个属性字段。
///
/// # 字段
///
/// * `annotations` - 应用于该字段的注解（支持元注解）
/// * `field_type` - 字段的类型引用
/// * `name` - 字段名称
/// * `default` - 字段的默认值表达式，`None` 表示该字段为必填项
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct AnnotationField {
    pub annotations: Vec<Annotation>,
    pub field_type: TypeRef,
    pub name: String,
    pub default: Option<Expression>,
    pub span: Span,
}

/// 注解实例，表示代码中实际使用的注解引用（如 `@Serializable(name = "MyClass")`)。
///
/// # 字段
///
/// * `name` - 注解名称
/// * `arguments` - 传递给注解的实际参数表达式列表
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct Annotation {
    pub name: String,
    pub generic_type: Option<TypeRef>,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

// === 注入器 ===

/// 注入器声明，定义类的外部依赖注入配置。
///
/// 注入器类似于构造函数的参数声明，但专用于依赖注入框架，
/// 支持自动装配外部依赖到类的字段中。
///
/// # 字段
///
/// * `fields` - 需要注入的字段列表
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct InjectorDeclaration {
    pub fields: Vec<InjectorField>,
    pub span: Span,
}

/// 注入器字段，表示一个需要被注入的依赖项。
///
/// # 字段
///
/// * `name` - 注入字段的名称
/// * `field_type` - 注入字段的类型引用
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct InjectorField {
    pub name: String,
    pub field_type: TypeRef,
    pub span: Span,
}

// === 参数 ===

/// 函数参数，表示方法或构造函数的形式参数。
///
/// # 字段
///
/// * `name` - 参数名称
/// * `param_type` - 参数的类型引用
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: TypeRef,
    pub span: Span,
}

// === 修饰符 ===

/// 修饰符，用于控制类、成员、方法的访问级别和行为特性。
///
/// 修饰符可以组合使用（如 `public static`），编译期会检查组合的合法性。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modifier {
    /// 外部实现标记，表示该方法或类由外部（通常是 C++）代码实现
    Native,
    /// 静态成员，属于类型本身而非实例
    Static,
    /// 抽象标记，表示该方法或类不提供具体实现，需子类实现
    Abstract,
}

// === 类型引用 ===

/// 类型引用，表示 AST 中对类型的引用方式。
///
/// 支持三种形式：简单类型（如 `int`）、泛型类型（如 `List<string>`）、
/// 数组类型（如 `int[]`）。`Array` 中的元素类型通过 `Box` 间接持有，
/// 避免无限递归占用。
#[derive(Debug, Clone)]
pub enum TypeRef {
    /// 简单类型，如 `int`、`string`、`MyClass`
    Simple { name: String, span: Span },
    /// 泛型类型，如 `List<string>`、`Dictionary<int, string>`，
    /// `type_args` 为类型参数列表
    Generic { name: String, type_args: Vec<TypeRef>, span: Span },
    /// 数组类型，如 `int[]`、`string[]`，
    /// `element_type` 为数组元素类型
    Array { element_type: Box<TypeRef>, span: Span },
    /// 委托类型引用 `delegate<ReturnType, ParamType, ...>`
    Delegate {
        return_type: Box<TypeRef>,
        param_types: Vec<TypeRef>,
        span: Span,
    },
}

impl TypeRef {
    /// 构造一个简单类型引用，不包含泛型参数或数组维度。
    ///
    /// # 参数
    ///
    /// * `name` - 类型名称
    /// * `span` - 源代码位置信息
    pub fn simple(name: impl Into<String>, span: Span) -> Self {
        TypeRef::Simple { name: name.into(), span }
    }

    /// 获取该类型引用在源代码中的位置信息。
    ///
    /// 为诊断信息（如类型错误提示）提供精确的源码定位。
    pub fn span(&self) -> Span {
        match self {
            TypeRef::Simple { span, .. } => *span,
            TypeRef::Generic { span, .. } => *span,
            TypeRef::Array { span, .. } => *span,
            TypeRef::Delegate { span, .. } => *span,
        }
    }
}

// === Lambda 体 ===

/// Lambda 体，可以是表达式体或语句块体
#[derive(Debug, Clone)]
pub enum LambdaBody {
    /// 表达式体 `x -> expr`
    Expression(Box<Expression>),
    /// 语句块体 `x -> { stmts }`
    Block(Vec<Statement>),
}

// === 表达式 ===

/// 表达式，表示 AST 中所有可求值的节点。
///
/// 表达式涵盖字面量、标识符引用、成员访问、方法调用、运算符、
/// 条件表达式、赋值、构造对象、类型转换、Lambda 和注入器等。
/// 每个变体都携带 `Span` 用于错误定位。
#[derive(Debug, Clone)]
pub enum Expression {
    /// 字面量表达式，如 `42`、`"hello"`、`true`
    Literal(Literal, Span),
    /// 标识符引用，如变量名 `x`
    Identifier(String, Span),
    /// 成员访问表达式，如 `obj.field`
    MemberAccess {
        object: Box<Expression>,
        member: String,
        span: Span,
    },
    /// 实例方法调用，如 `obj.Method(arg1, arg2)`
    MethodCall {
        receiver: Box<Expression>,
        method: String,
        arguments: Vec<Expression>,
        span: Span,
    },
    /// 静态方法调用，如 `ClassName.Method(arg1, arg2)`
    StaticMethodCall {
        class_name: String,
        method: String,
        arguments: Vec<Expression>,
        span: Span,
    },
    /// 数组元素访问，如 `arr[3]`
    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
        span: Span,
    },
    /// 二元运算，如 `a + b`、`x && y`
    Binary {
        left: Box<Expression>,
        operator: BinaryOp,
        right: Box<Expression>,
        span: Span,
    },
    /// 一元运算，如 `-x`、`!flag`、`++i`
    Unary {
        operator: UnaryOp,
        operand: Box<Expression>,
        span: Span,
    },
    /// 三元条件表达式 `condition ? then_branch : else_branch`，
    /// `else_branch` 为 `None` 时表示简化形式（无 else 部分）
    Conditional {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
        span: Span,
    },
    /// 赋值表达式，如 `x = 5`、`a += 3`
    Assignment {
        target: AssignmentTarget,
        operator: AssignmentOp,
        value: Box<Expression>,
        span: Span,
    },
    /// 对象构造表达式 `new ClassName(args)`
    New {
        class_type: TypeRef,
        arguments: Vec<Expression>,
        span: Span,
    },
    /// 类型转换表达式 `(TargetType)expr`
    Cast {
        target_type: TypeRef,
        expression: Box<Expression>,
        span: Span,
    },
    /// Lambda 表达式（匿名函数），如 `(x, y) => x + y`
    Lambda {
        parameters: Vec<Parameter>,
        body: LambdaBody,
        span: Span,
    },
    /// 注入器对象字面量，用于 DI 容器中构造对象
    InjectorObject {
        fields: Vec<(String, Expression)>,
        span: Span,
    },
    /// 注入器数组字面量，用于 DI 容器中构造数组
    InjectorArray {
        elements: Vec<Expression>,
        span: Span,
    },
    /// `this` 关键字，引用当前实例
    This(Span),
    /// `super` 关键字，引用父类实例
    Super(Span),
    /// `null` 字面量
    Null(Span),
}

impl Expression {
    /// 获取该表达式在源代码中的位置信息。
    ///
    /// 返回表达式的 `Span`，支持所有变体。对于包含嵌套表达式的变体
    /// （如 `Binary`、`MethodCall`），返回的是整个表达式范围的 `Span`，
    /// 包含所有子表达式。
    pub fn span(&self) -> Span {
        match self {
            Expression::Literal(_, span) => *span,
            Expression::Identifier(_, span) => *span,
            Expression::MemberAccess { span, .. } => *span,
            Expression::MethodCall { span, .. } => *span,
            Expression::StaticMethodCall { span, .. } => *span,
            Expression::ArrayAccess { span, .. } => *span,
            Expression::Binary { span, .. } => *span,
            Expression::Unary { span, .. } => *span,
            Expression::Conditional { span, .. } => *span,
            Expression::Assignment { span, .. } => *span,
            Expression::New { span, .. } => *span,
            Expression::Cast { span, .. } => *span,
            Expression::Lambda { span, .. } => *span,
            Expression::InjectorObject { span, .. } => *span,
            Expression::InjectorArray { span, .. } => *span,
            Expression::This(span) => *span,
            Expression::Super(span) => *span,
            Expression::Null(span) => *span,
        }
    }
}

// === 字面量 ===

/// 字面量，表示源代码中直接写出的常量值。
///
/// 支持四种基本类型：整数、浮点数、布尔值和字符串。
/// 字面量在编译期即可确定其值，不依赖运行时上下文。
#[derive(Debug, Clone)]
pub enum Literal {
    /// 整数字面量，如 `42`、`-7`
    Int(i64),
    /// 浮点数字面量，如 `3.14`、`-0.5`
    Float(f64),
    /// 布尔字面量，`true` 或 `false`
    Bool(bool),
    /// 字符串字面量，如 `"hello"`
    String(String),
}

// === 二元运算符 ===

/// 二元运算符，用于 `Expression::Binary` 中表达两个操作数之间的运算关系。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    /// 加法 `+`
    Add,
    /// 减法 `-`
    Subtract,
    /// 乘法 `*`
    Multiply,
    /// 除法 `/`
    Divide,
    /// 取模 `%`
    Modulo,
    /// 小于 `<`
    Less,
    /// 小于等于 `<=`
    LessEqual,
    /// 大于 `>`
    Greater,
    /// 大于等于 `>=`
    GreaterEqual,
    /// 等于 `==`
    Equal,
    /// 不等于 `!=`
    NotEqual,
    /// 逻辑与 `&&`
    LogicAnd,
    /// 逻辑或 `||`
    LogicOr,
}

// === 一元运算符 ===

/// 一元运算符，用于 `Expression::Unary` 中表达单个操作数的运算。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// 算术取负 `-x`
    Negate,
    /// 逻辑取反 `!flag`
    Not,
    /// 前置自增 `++x`
    PreIncrement,
    /// 前置自减 `--x`
    PreDecrement,
    /// 后置自增 `x++`
    PostIncrement,
    /// 后置自减 `x--`
    PostDecrement,
}

// === 赋值 ===

/// 赋值运算符，用于复合赋值表达式。
///
/// `Assign` 为普通赋值，其余变体为先运算再赋值的复合形式。
/// 如 `a += b` 等价于 `a = a + b`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOp {
    /// 普通赋值 `=`
    Assign,
    /// 加后赋值 `+=`
    PlusAssign,
    /// 减后赋值 `-=`
    MinusAssign,
    /// 乘后赋值 `*=`
    StarAssign,
    /// 除后赋值 `/=`
    SlashAssign,
}

/// 赋值目标，表示赋值语句左侧可被赋值的对象。
///
/// 支持三种赋值目标：简单变量、对象字段、数组元素。
/// 与 C# 的赋值语义一致。
#[derive(Debug, Clone)]
pub enum AssignmentTarget {
    /// 简单变量赋值，如 `x = 5`
    Variable(String, Span),
    /// 字段赋值，如 `obj.field = 5`
    Field { object: Box<Expression>, field: String, span: Span },
    /// 数组元素赋值，如 `arr[i] = 5`
    ArrayElement { array: Box<Expression>, index: Box<Expression>, span: Span },
    /// 注入器字段赋值目标 `obj.^field`
    InjectorField {
        object: Box<Expression>,
        field: String,
        span: Span,
    },
}

impl AssignmentTarget {
    /// 获取赋值目标在源代码中的位置信息。
    pub fn span(&self) -> Span {
        match self {
            AssignmentTarget::Variable(_, span) => *span,
            AssignmentTarget::Field { span, .. } => *span,
            AssignmentTarget::ArrayElement { span, .. } => *span,
            AssignmentTarget::InjectorField { span, .. } => *span,
        }
    }
}

// === 语句 ===

/// 语句，表示 AST 中执行操作的节点，不返回值（表达式语句除外）。
///
/// 语句涵盖表达式语句、变量声明、块、控制流（if/while/for/foreach/switch）、
/// 返回、中断和继续。每个变体均携带 `Span` 用于错误定位。
#[derive(Debug, Clone)]
pub enum Statement {
    /// 表达式语句，将表达式作为语句执行（如 `foo()`），表达式的结果被丢弃
    Expression(Expression, Span),
    /// 变量声明语句，如 `int x = 5;`，
    /// `initializer` 为 `None` 时仅声明不初始化
    VariableDeclaration {
        var_type: TypeRef,
        name: String,
        initializer: Option<Expression>,
        span: Span,
    },
    /// 块语句，由大括号包围的若干语句组成，形成一个新的作用域
    Block {
        statements: Vec<Statement>,
        span: Span,
    },
    /// 条件分支语句 `if (condition) then_branch [else else_branch]`
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
        span: Span,
    },
    /// 前置条件循环语句 `while (condition) body`
    While {
        condition: Expression,
        body: Box<Statement>,
        span: Span,
    },
    /// do-while 循环语句 `do { ... } while (condition);`
    /// 先执行循环体再检查条件，至少执行一次
    DoWhile {
        body: Box<Statement>,
        condition: Expression,
        span: Span,
    },
    /// C 风格 for 循环 `for (initializer; condition; update) body`，
    /// 三个部分均可选
    For {
        initializer: Option<Box<Statement>>,
        condition: Option<Expression>,
        update: Option<Expression>,
        body: Box<Statement>,
        span: Span,
    },
    /// switch 多路分支语句，包含若干 case 块和一个可选的 default 分支
    Switch {
        expression: Expression,
        cases: Vec<CaseBlock>,
        default_body: Option<Box<Statement>>,
        span: Span,
    },
    /// 返回语句 `return [value]`，`value` 为 `None` 时表示无返回值
    Return {
        value: Option<Expression>,
        span: Span,
    },
    /// `break` 语句，用于跳出当前循环或 switch，可携带多层跳出目标
    Break {
        targets: Vec<BreakTarget>,
        span: Span,
    },
    /// `continue` 语句，用于跳过当前循环迭代的剩余部分，可携带多层跳出目标
    Continue {
        targets: Vec<BreakTarget>,
        span: Span,
    },
}

impl Statement {
    /// 获取该语句在源代码中的位置信息。
    pub fn span(&self) -> Span {
        match self {
            Statement::Expression(_, span) => *span,
            Statement::VariableDeclaration { span, .. } => *span,
            Statement::Block { span, .. } => *span,
            Statement::If { span, .. } => *span,
            Statement::While { span, .. } => *span,
            Statement::DoWhile { span, .. } => *span,
            Statement::For { span, .. } => *span,
            Statement::Switch { span, .. } => *span,
            Statement::Return { span, .. } => *span,
            Statement::Break { span, .. } => *span,
            Statement::Continue { span, .. } => *span,
        }
    }
}

/// 跳出目标，描述 `break`/`continue` 语句要跳出的循环层级。
///
/// 支持按层级数跳出（`break 2`）或按标签名跳出（`break outer`）。
#[derive(Debug, Clone)]
pub enum BreakTarget {
    /// 按层级数跳出，数值表示要跳出的循环层数（1 表示当前循环）
    ByLayer(u32),
    /// 按循环标签名跳出
    ByKeyword(String),
}

/// case 块，表示 `switch` 语句中的一个分支。
///
/// 每个 case 块可以匹配多个值（`values`），执行体为语句列表（`body`），
/// 不支持 C# 风格的贯穿（fall-through），每个分支相互独立。
///
/// # 字段
///
/// * `values` - 该分支匹配的表达式值列表（如 `case 1, 2, 3`）
/// * `body` - 匹配成功后执行的语句列表
/// * `span` - 源代码位置信息
#[derive(Debug, Clone)]
pub struct CaseBlock {
    pub values: Vec<Expression>,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_span() -> Span {
        Span::new(0, 1, 1, 1, 0)
    }

    #[test]
    fn test_literal_expression() {
        let expr = Expression::Literal(Literal::Int(42), dummy_span());
        assert!(matches!(expr, Expression::Literal(Literal::Int(42), _)));
    }

    #[test]
    fn test_binary_expression() {
        let left = Expression::Literal(Literal::Int(1), dummy_span());
        let right = Expression::Literal(Literal::Int(2), dummy_span());
        let expr = Expression::Binary {
            left: Box::new(left),
            operator: BinaryOp::Add,
            right: Box::new(right),
            span: dummy_span(),
        };
        assert!(matches!(expr, Expression::Binary { operator: BinaryOp::Add, .. }));
    }

    #[test]
    fn test_statement_span() {
        let stmt = Statement::Return {
            value: None,
            span: dummy_span(),
        };
        assert_eq!(stmt.span().line, 1);
    }
}
