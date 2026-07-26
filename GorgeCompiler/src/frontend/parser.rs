#![allow(dead_code)]

use gorge_core::diagnostics::{Diagnostics, Span};

use crate::frontend::ast::*;
use crate::frontend::lexer::{Token, TokenSpan};

// === Pratt 解析器优先级常量 ===

/// 无优先级（哨兵值）
const PREC_NONE: u8 = 0;
/// 赋值运算符 `=`, `+=`, `-=`, `*=`, `/=`, `%=`
const PREC_ASSIGNMENT: u8 = 1;
/// 条件表达式 `?:`
const PREC_CONDITIONAL: u8 = 2;
/// 逻辑或 `||`
const PREC_LOGICAL_OR: u8 = 3;
/// 逻辑与 `&&`
const PREC_LOGICAL_AND: u8 = 4;
/// 相等比较 `==`, `!=`
const PREC_EQUALITY: u8 = 5;
/// 大小比较 `<`, `<=`, `>`, `>=`
const PREC_COMPARISON: u8 = 6;
/// 加减 `+`, `-`
const PREC_ADDITION: u8 = 7;
/// 乘除取模 `*`, `/`, `%`
const PREC_MULTIPLICATION: u8 = 8;
/// 一元前缀 `-`, `!`, 强制转换
const PREC_UNARY: u8 = 9;
/// 后缀操作 `.` `()` `[]`
const PREC_POSTFIX: u8 = 10;
/// 主表达式（字面量、标识符、`new` 等）
const PREC_PRIMARY: u8 = 11;

/// 递归下降语法分析器
///
/// 使用 Pratt 解析算法处理表达式，手写递归下降处理语句和顶层声明。
/// 参考 C# 版本的 Gorge.g4 / GorgeExpression.g4 / GorgeStatement.g4 语法规则。
pub struct Parser {
    /// Token 序列
    tokens: Vec<TokenSpan>,
    /// 当前读取位置
    pos: usize,
    /// 诊断信息收集器
    diagnostics: Diagnostics,
    /// 抑制 `:` 预检（用于 `?:` 表达式的真/假分支解析，防止 `:` 被误认为 Lambda/注入器中缀）
    suppress_colon_precheck: bool,
}

impl Parser {
    /// 创建新的解析器实例
    pub fn new(tokens: Vec<TokenSpan>) -> Self {
        Self {
            tokens,
            pos: 0,
            diagnostics: Diagnostics::new(),
            suppress_colon_precheck: false,
        }
    }

    // ==================== 辅助方法 ====================

    /// 查看当前 token，不移动位置
    fn peek(&self) -> Option<&TokenSpan> {
        self.tokens.get(self.pos)
    }

    /// 查看向前第 n 个 token（n=0 等同于 peek）
    fn peek_ahead(&self, n: usize) -> Option<&TokenSpan> {
        self.tokens.get(self.pos + n)
    }

    /// 消费当前 token 并返回，位置前进 1
    fn advance(&mut self) -> Option<TokenSpan> {
        let token = self.tokens.get(self.pos).cloned();
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    /// 当前 token 是否匹配指定类型
    fn check(&self, token: &Token) -> bool {
        matches!(self.peek().map(|t| &t.token), Some(t) if std::mem::discriminant(t) == std::mem::discriminant(token))
    }

    /// 检查当前 token 是否为关键字（用于语句解析分派）
    fn check_keyword(&self, keyword: &Token) -> bool {
        self.check(keyword)
    }

    /// 检查当前 token 是否为标识符
    fn check_identifier(&self) -> bool {
        matches!(self.peek().map(|t| &t.token), Some(Token::Identifier(_)))
    }

    /// 获取当前标识符名称（如果是标识符的话）
    fn peek_identifier_name(&self) -> Option<&str> {
        match self.peek() {
            Some(TokenSpan { token: Token::Identifier(name), .. }) => Some(name.as_str()),
            _ => None,
        }
    }

    /// 是否已到达 token 流末尾
    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// 尝试匹配指定 token 类型，匹配成功则消费并返回 true
    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// 尝试匹配标识符并返回其名称
    fn match_identifier(&mut self) -> Option<String> {
        match self.peek() {
            Some(TokenSpan { token: Token::Identifier(name), .. }) => {
                let name = name.clone();
                self.advance();
                Some(name)
            }
            _ => None,
        }
    }

    /// 尝试匹配标识符或可在注解/标识符位置出现的 Gorge 关键字并返回名称
    ///
    /// 与 `match_identifier` 不同，此函数额外识别在 `@` 注解名等上下文中的关键字 token
    ///（如 `KwInject`/`KwInjector`/`KwDelegate`），返回首字母大写的 PascalCase 名称以对齐 C# 注解名约定。
    fn match_identifier_or_keyword(&mut self) -> Option<String> {
        match self.peek() {
            Some(TokenSpan { token: Token::Identifier(name), .. }) => {
                let name = name.clone();
                self.advance();
                Some(name)
            }
            Some(TokenSpan { token: Token::KwInject, .. }) => {
                self.advance();
                Some("Inject".to_string())
            }
            Some(TokenSpan { token: Token::KwInjector, .. }) => {
                self.advance();
                Some("Injector".to_string())
            }
            Some(TokenSpan { token: Token::KwDelegate, .. }) => {
                self.advance();
                Some("Delegate".to_string())
            }
            _ => None,
        }
    }

    /// 期望当前 token 为指定类型，匹配成功则消费返回，失败则产生错误
    fn expect(&mut self, expected: &str) -> Result<TokenSpan, ()> {
        if self.is_at_end() {
            let span = self.last_span();
            self.diagnostics.emit_error(span, format!("期望 {}，但已到达文件末尾", expected));
            return Err(());
        }
        let token = self.advance().unwrap();
        Ok(token)
    }

    /// 期望当前 token 为指定的 Token 类型
    fn expect_token(&mut self, expected: &Token) -> Result<TokenSpan, ()> {
        if self.check(expected) {
            Ok(self.advance().unwrap())
        } else {
            let span = self.current_span();
            self.diagnostics
                .emit_error(span, format!("期望 {:?}，但遇到了其他 token", expected));
            Err(())
        }
    }

    /// 期望并消费一个分号，否则报错
    fn expect_semicolon(&mut self) -> Result<(), ()> {
        if self.match_token(&Token::Semicolon) {
            Ok(())
        } else {
            let span = self.current_span();
            self.diagnostics.emit_error(span, "期望分号 `;`");
            Err(())
        }
    }

    /// 获取当前位置的 span（用于错误报告）
    fn current_span(&self) -> Span {
        self.peek()
            .map(|t| t.span)
            .unwrap_or_else(Span::dummy)
    }

    /// 获取上一个已消费 token 的 span
    fn last_span(&self) -> Span {
        if self.pos > 0 {
            self.tokens[self.pos - 1].span
        } else {
            Span::dummy()
        }
    }

    /// 获取已收集的诊断信息
    pub fn into_diagnostics(self) -> Diagnostics {
        self.diagnostics
    }

    // ==================== 顶层解析 ====================

    /// 解析整个源文件
    ///
    /// 按 Gorge.g4 语法规则，顶层为可任意交错出现的声明序列：
    /// namespace、using、class、interface、enum。
    pub fn parse_source_file(&mut self) -> Result<SourceFile, Diagnostics> {
        let mut namespace: Option<QualifiedName> = None;
        let mut usings: Vec<UsingDirective> = Vec::new();
        let mut members: Vec<TopLevelMember> = Vec::new();

        while !self.is_at_end() {
            // namespace 声明（可多次出现，后声明的覆盖前面的）
            if self.match_token(&Token::KwNamespace) {
                if let Ok(ns) = self.parse_qualified_name() {
                    namespace = Some(ns);
                }
                let _ = self.expect_semicolon();
                continue;
            }

            // using 声明（可多次出现）
            // K3: 支持 `using Alias = expr;` 别名语法
            if self.match_token(&Token::KwUsing) {
                let first = self.match_identifier();
                // 检测别名语法：using Alias = ...
                if first.is_some() && self.match_token(&Token::Assign) {
                    if let Ok(name) = self.parse_qualified_name() {
                        usings.push(UsingDirective { name, alias: first, span: Span::dummy() });
                    }
                } else if let Some(first_part) = first {
                    // 普通 using 语法：using QualifiedName
                    let mut parts = vec![first_part];
                    let span_start = self.current_span();
                    while self.match_token(&Token::Dot) {
                        if let Some(part) = self.match_identifier() {
                            parts.push(part);
                        } else {
                            break;
                        }
                    }
                    usings.push(UsingDirective { name: QualifiedName { parts, span: span_start }, alias: None, span: Span::dummy() });
                }
                let _ = self.expect_semicolon();
                continue;
            }

            match self.parse_top_level_member() {
                Ok(member) => members.push(member),
                Err(()) => {
                    self.synchronize();
                }
            }
        }

        if self.diagnostics.has_errors() {
            return Err(self.diagnostics.clone());
        }

        Ok(SourceFile {
            namespace,
            usings,
            members,
            span: Span::dummy(),
        })
    }

    /// 跳过出错 token，尝试同步到下一个安全恢复点
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            // 在语句边界处恢复：}、; 或下一个关键字
            match self.peek() {
                Some(t) => match &t.token {
                    Token::RBrace | Token::Semicolon => {
                        self.advance();
                        return;
                    }
                    Token::KwClass
                    | Token::KwInterface
                    | Token::KwEnum
                    | Token::KwNamespace
                    | Token::KwUsing => {
                        return;
                    }
                    _ => {
                        self.advance();
                    }
                },
                None => return,
            }
        }
    }

    /// 解析可选的 namespace 声明
    fn parse_namespace(&mut self) -> Option<QualifiedName> {
        if self.match_token(&Token::KwNamespace) {
            self.parse_qualified_name().ok()
        } else {
            None
        }
    }

    /// 解析 using 指令列表
    fn parse_usings(&mut self) -> Vec<UsingDirective> {
        let mut usings = Vec::new();
        while self.match_token(&Token::KwUsing) {
            if let Ok(name) = self.parse_qualified_name() {
                usings.push(UsingDirective {
                    name,
                    alias: None,
                    span: Span::dummy(),
                });
            }
            let _ = self.expect_semicolon();
        }
        usings
    }

    /// 解析一个顶层成员声明
    fn parse_top_level_member(&mut self) -> Result<TopLevelMember, ()> {
        let annotations = self.parse_annotations();
        let modifiers = self.parse_modifiers();

        match self.peek() {
            Some(TokenSpan { token: Token::KwClass, .. }) => {
                self.advance();
                self.parse_class_declaration(annotations, modifiers)
                    .map(TopLevelMember::Class)
            }
            Some(TokenSpan { token: Token::KwInterface, .. }) => {
                self.advance();
                self.parse_interface_declaration(annotations, modifiers)
                    .map(TopLevelMember::Interface)
            }
            Some(TokenSpan { token: Token::KwEnum, .. }) => {
                self.advance();
                self.parse_enum_declaration(annotations, modifiers)
                    .map(TopLevelMember::Enum)
            }
            _ => {
                let span = self.current_span();
                self.diagnostics
                    .emit_error(span, "顶层只能声明 class、interface 或 enum");
                Err(())
            }
        }
    }

    /// 解析类型引用
    ///
    /// 支持简单类型名（`int`、`MyClass`）、泛型类型（`List<MyClass>`）、
    /// 数组类型（`int[]`）、注入器类型（`Type^`）及其组合（`Type^[]^`）。
    fn parse_type_ref(&mut self) -> Result<TypeRef, ()> {
        let span = self.current_span();

        let mut result = if self.match_token(&Token::TypeInt) {
            TypeRef::simple("int", span)
        } else if self.match_token(&Token::TypeFloat) {
            TypeRef::simple("float", span)
        } else if self.match_token(&Token::TypeBool) {
            TypeRef::simple("bool", span)
        } else if self.match_token(&Token::TypeString) {
            TypeRef::simple("string", span)
        } else if self.match_token(&Token::TypeVoid) {
            TypeRef::simple("void", span)
        } else if self.match_token(&Token::TypeObject) {
            TypeRef::simple("object", span)
        } else if self.match_token(&Token::KwAuto) {
            TypeRef::simple("auto", span)
        } else if self.match_token(&Token::KwDelegate) {
            // delegate<ReturnType, ParamType, ...>
            self.expect_token(&Token::Less)?;
            let return_type = self.parse_type_ref()?;
            let mut param_types = Vec::new();
            while self.match_token(&Token::Colon) || self.match_token(&Token::Comma) {
                param_types.push(self.parse_type_ref()?);
            }
            self.expect_token(&Token::Greater)?;
            TypeRef::Delegate {
                return_type: Box::new(return_type),
                param_types,
                span,
            }
        } else {
            // 用户自定义类型名（支持限定名 Name1.Name2...）
            // 使用回溯机制：限定名解析失败时恢复位置，只取第一个标识符
            let first = self.match_identifier().ok_or_else(|| {
                self.diagnostics
                    .emit_error(self.current_span(), "期望类型名称");
                ()
            })?;
            let mut name = first.clone();
            let mut saved_pos = self.pos;

            // 限定名: Name1.Name2.Name3
            // 仅在 `.` 后紧跟标识符时才消费（通过位置恢复避免误消费成员访问的点）
            while self.match_token(&Token::Dot) {
                match self.match_identifier() {
                    Some(next) => {
                        name.push('.');
                        name.push_str(&next);
                        saved_pos = self.pos;
                    }
                    None => {
                        // 点号后不是标识符，恢复位置并停止
                        self.pos = saved_pos;
                        break;
                    }
                }
            }

            // 泛型参数
            if self.match_token(&Token::Less) {
                let mut type_args = Vec::new();
                loop {
                    type_args.push(self.parse_type_ref()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
                self.expect_token(&Token::Greater).map_err(|_| ())?;
                TypeRef::Generic { name, type_args, span }
            } else {
                TypeRef::simple(name, span)
            }
        };

        // 后缀链: [] 和 ^ 的任意组合（[] 只在紧接 RBracket 时才是类型后缀）
        loop {
            if self.check(&Token::LBracket) && matches!(self.peek_ahead(1).map(|t| &t.token), Some(Token::RBracket)) {
                self.advance(); // [
                self.advance(); // ]
                result = TypeRef::Array {
                    element_type: Box::new(result),
                    span,
                };
            } else if self.match_token(&Token::Caret) {
                result = TypeRef::Injector {
                    base_type: Box::new(result),
                    span,
                };
            } else {
                break;
            }
        }

        Ok(result)
    }

    /// 解析限定名（如 `System.Collections.Generic`）
    fn parse_qualified_name(&mut self) -> Result<QualifiedName, ()> {
        let span = self.current_span();
        let mut parts = Vec::new();

        let first = self.match_identifier().ok_or_else(|| {
            self.diagnostics
                .emit_error(self.current_span(), "期望限定名");
            ()
        })?;
        parts.push(first);

        while self.match_token(&Token::Dot) {
            let part = self.match_identifier().ok_or_else(|| {
                self.diagnostics
                    .emit_error(self.current_span(), "期望标识符");
                ()
            })?;
            parts.push(part);
        }

        Ok(QualifiedName { parts, span })
    }

    // ==================== 类声明 ====================

    /// 解析类声明中的成员
    ///
    /// 通过 lookahead 区分三种成员声明：字段、方法和构造函数。
    /// 构造函数由类名标识符后跟 `(` 开头，而字段和方法都由类型引用开头。
    fn parse_class_member(&mut self) -> Result<ClassMember, ()> {
        let annotations = self.parse_annotations();
        let modifiers = self.parse_modifiers();

        // 保存位置用于回退到构造函数路径
        let saved_pos = self.pos;

        // 尝试解析为 TypeRef + Identifier（方法或字段）
        if let Ok(member_type) = self.parse_type_ref() {
            if let Some(name) = self.match_identifier() {
                match self.peek() {
                    Some(TokenSpan { token: Token::LParen, .. }) => {
                        return self
                            .parse_method_declaration(annotations, modifiers, member_type, name)
                            .map(ClassMember::Method);
                    }
                    _ => {
                        return self
                            .parse_field_declaration(annotations, modifiers, member_type, name)
                            .map(ClassMember::Field);
                    }
                }
            }
        }

        // 回退到 modifiers 之后：尝试构造函数路径
        // 构造函数语法：annotation* [injector] Identifier ( params ) [: super(args)] { body }
        self.pos = saved_pos;

        if let Some(name) = self.match_identifier() {
            if self.check(&Token::LParen) {
                return self
                    .parse_constructor_declaration(annotations, modifiers, name)
                    .map(ClassMember::Constructor);
            }
        }

        let span = self.current_span();
        self.diagnostics
            .emit_error(span, "期望字段、方法或构造函数声明");
        Err(())
    }

    /// 解析字段声明
    fn parse_field_declaration(
        &mut self,
        annotations: Vec<Annotation>,
        modifiers: Vec<Modifier>,
        field_type: TypeRef,
        name: String,
    ) -> Result<FieldDeclaration, ()> {
        let span = self.current_span();
        // Gorge 语法中字段声明不允许修饰符（fieldDeclaration 无 modifier，
        // 只有方法可带 static 等）。检测到修饰符即报编译错误，对齐 C# 语法。
        if !modifiers.is_empty() {
            self.diagnostics.emit_error(
                span,
                format!("字段 `{}` 不允许修饰符（Gorge 中只有方法可带 static/native 等修饰符）", name),
            );
        }
        let initializer = if self.match_token(&Token::Assign) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect_semicolon()?;

        Ok(FieldDeclaration {
            annotations,
            modifiers,
            field_type,
            name,
            initializer,
            span,
        })
    }

    /// 解析方法声明
    fn parse_method_declaration(
        &mut self,
        annotations: Vec<Annotation>,
        modifiers: Vec<Modifier>,
        return_type: TypeRef,
        name: String,
    ) -> Result<MethodDeclaration, ()> {
        let span = self.current_span();
        self.advance(); // 消费 '('
        let parameters = self.parse_parameters()?;
        self.expect_token(&Token::RParen)?;

        let body = if self.check(&Token::LBrace) {
            let statements = self.parse_block()?;
            Some(statements)
        } else {
            self.expect_semicolon()?; // 接口方法或抽象方法无方法体
            None
        };

        Ok(MethodDeclaration {
            annotations,
            modifiers,
            return_type,
            name,
            parameters,
            body,
            span,
        })
    }

    /// 解析构造函数声明（关键字 `(` 还未消费，需由调用方确认）
    ///
    /// Gorge 构造函数语法：`[injector] ClassName(params) [: super(args)] { body }`
    fn parse_constructor_declaration(
        &mut self,
        annotations: Vec<Annotation>,
        modifiers: Vec<Modifier>,
        _name: String,
    ) -> Result<ConstructorDeclaration, ()> {
        let span = self.current_span();
        self.advance(); // 消费 '('
        let parameters = self.parse_parameters()?;
        self.expect_token(&Token::RParen)?;

        // 可选的 super(...) 调用
        let base_arguments = if self.match_token(&Token::Colon) {
            self.expect_token(&Token::KwSuper)?;
            self.expect_token(&Token::LParen)?;
            let args = self.parse_argument_list()?;
            self.expect_token(&Token::RParen)?;
            args
        } else {
            Vec::new()
        };

        // 构造方法体
        let body = if self.check(&Token::LBrace) {
            let statements = self.parse_block()?;
            Some(statements)
        } else {
            self.expect_semicolon()?;
            None
        };

        Ok(ConstructorDeclaration {
            annotations,
            modifiers,
            parameters,
            base_arguments,
            body,
            span,
        })
    }

    /// 解析参数列表
    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, ()> {
        let mut params = Vec::new();

        if self.check(&Token::RParen) {
            return Ok(params);
        }

        loop {
            let param_type = self.parse_type_ref()?;
            let name = self.match_identifier().ok_or_else(|| {
                self.diagnostics
                    .emit_error(self.current_span(), "期望参数名称");
                ()
            })?;
            let span = self.current_span();

            params.push(Parameter { name, param_type, span });

            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        Ok(params)
    }

    /// 解析带类型的参数列表 `Type name, Type name`
    fn parse_typed_parameters(&mut self) -> Result<Vec<Parameter>, ()> {
        let mut params = Vec::new();
        if self.check(&Token::RParen) {
            return Ok(params);
        }
        loop {
            let param_type = self.parse_type_ref()?;
            let name = self.match_identifier().ok_or_else(|| {
                self.diagnostics.emit_error(self.current_span(), "期望参数名");
                ()
            })?;
            let span = self.current_span();
            params.push(Parameter { name, param_type, span });
            if !self.match_token(&Token::Comma) {
                break;
            }
        }
        Ok(params)
    }

    /// 解析类型关键字，若后跟 `:(params) -> body` 则解析为 Lambda，否则返回标识符
    fn parse_type_keyword_or_lambda(&mut self, type_name: &str, span: Span) -> Result<Expression, ()> {
        if self.check(&Token::Colon) {
            if let Some(TokenSpan { token: Token::LParen, .. }) = self.peek_ahead(1) {
                self.advance(); // 消费 ':'
                self.advance(); // 消费 '('
                let params = self.parse_typed_parameters()?;
                self.expect_token(&Token::RParen)?;
                self.expect_token(&Token::LambdaArrow)?;
                let body = if self.check(&Token::LBrace) {
                    LambdaBody::Block(self.parse_block()?)
                } else {
                    LambdaBody::Expression(Box::new(self.parse_expression()?))
                };
                return Ok(Expression::Lambda {
                    parameters: params,
                    body,
                    span,
                });
            }
        }
        Ok(Expression::Identifier(type_name.to_string(), span))
    }

    // ==================== 类/接口/枚举声明 ====================

    /// 解析类声明（关键字已消费）
    fn parse_class_declaration(
        &mut self,
        annotations: Vec<Annotation>,
        modifiers: Vec<Modifier>,
    ) -> Result<ClassDeclaration, ()> {
        let span = self.current_span();
        let name = self.match_identifier().ok_or_else(|| {
            self.diagnostics.emit_error(self.current_span(), "期望类名");
            ()
        })?;

        // J1: 泛型参数 `class Foo<T, U>`
        let generic_params = self.parse_generic_params()?;

        // 继承关系
        let super_class = if self.match_token(&Token::Colon) {
            Some(self.parse_type_ref()?)
        } else {
            None
        };

        let super_interfaces = if self.match_token(&Token::DoubleColon) {
            let mut interfaces = Vec::new();
            loop {
                interfaces.push(self.parse_type_ref()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
            interfaces
        } else {
            Vec::new()
        };

        // injector 声明
        let injector = if self.match_token(&Token::KwInjector) {
            self.expect_token(&Token::LBrace)?;
            let fields = self.parse_injector_fields()?;
            self.expect_token(&Token::RBrace)?;
            Some(InjectorDeclaration { fields, span })
        } else {
            None
        };

        // 类体
        self.expect_token(&Token::LBrace)?;
        let mut members = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            members.push(self.parse_class_member()?);
        }
        self.expect_token(&Token::RBrace)?;

        Ok(ClassDeclaration {
            annotations,
            modifiers,
            name,
            generic_params,
            super_class,
            super_interfaces,
            injector,
            members,
            span,
        })
    }

    /// 解析泛型参数列表 `<T, U, ...>`（J1）
    fn parse_generic_params(&mut self) -> Result<Vec<String>, ()> {
        if !self.match_token(&Token::Less) {
            return Ok(Vec::new());
        }
        let mut params = Vec::new();
        loop {
            let name = self.match_identifier().ok_or_else(|| {
                self.diagnostics.emit_error(self.current_span(), "泛型参数期望标识符");
                ()
            })?;
            params.push(name);
            if !self.match_token(&Token::Comma) {
                break;
            }
        }
        self.expect_token(&Token::Greater)?;
        Ok(params)
    }

    /// 解析接口声明（关键字已消费）
    fn parse_interface_declaration(
        &mut self,
        annotations: Vec<Annotation>,
        modifiers: Vec<Modifier>,
    ) -> Result<InterfaceDeclaration, ()> {
        let span = self.current_span();
        let name = self.match_identifier().ok_or_else(|| {
            self.diagnostics.emit_error(self.current_span(), "期望接口名");
            ()
        })?;

        // 父接口
        let super_interfaces = if self.match_token(&Token::KwExtends) {
            let mut interfaces = Vec::new();
            loop {
                interfaces.push(self.parse_type_ref()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
            interfaces
        } else {
            Vec::new()
        };

        // 接口体
        self.expect_token(&Token::LBrace)?;
        let mut methods = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            let method_annotations = self.parse_annotations();
            let return_type = self.parse_type_ref()?;
            let method_name = self.match_identifier().ok_or_else(|| {
                self.diagnostics.emit_error(self.current_span(), "期望方法名");
                ()
            })?;

            self.expect_token(&Token::LParen)?;
            let parameters = self.parse_parameters()?;
            self.expect_token(&Token::RParen)?;
            self.expect_semicolon()?;

            methods.push(MethodSignature {
                annotations: method_annotations,
                return_type,
                name: method_name,
                parameters,
                span,
            });
        }
        self.expect_token(&Token::RBrace)?;

        Ok(InterfaceDeclaration {
            annotations,
            modifiers,
            name,
            super_interfaces,
            methods,
            span,
        })
    }

    /// 解析枚举声明（关键字已消费）
    fn parse_enum_declaration(
        &mut self,
        annotations: Vec<Annotation>,
        modifiers: Vec<Modifier>,
    ) -> Result<EnumDeclaration, ()> {
        let span = self.current_span();
        let name = self.match_identifier().ok_or_else(|| {
            self.diagnostics.emit_error(self.current_span(), "期望枚举名");
            ()
        })?;

        self.expect_token(&Token::LBrace)?;
        let mut values = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            let val_annotations = self.parse_annotations();
            let val_name = self.match_identifier().ok_or_else(|| {
                self.diagnostics.emit_error(self.current_span(), "期望枚举值名称");
                ()
            })?;

            let val = if self.match_token(&Token::Assign) {
                // 枚举值可以指定数值
                match self.parse_expression()? {
                    Expression::Literal(Literal::Int(v), _) => Some(v),
                    _ => None,
                }
            } else {
                None
            };

            values.push(EnumValue {
                annotations: val_annotations,
                name: val_name,
                value: val,
                span,
            });

            if !self.match_token(&Token::Comma) {
                break;
            }
        }
        self.expect_token(&Token::RBrace)?;

        Ok(EnumDeclaration {
            annotations,
            modifiers,
            name,
            values,
            span,
        })
    }

    /// 解析注解列表
    ///
    /// Gorge 注解语法：`metadata? '@' Identifier ('(' key '=' expr (',' key '=' expr)* ')')?`
    /// metadata 块为 `[ type name = expr (',' type name = expr)* ]`
    fn parse_annotations(&mut self) -> Vec<Annotation> {
        let mut annotations = Vec::new();
        let mut pending_metadatas: Vec<MetadataEntry> = Vec::new();
        loop {
            if self.check(&Token::LBracket) {
                let m = self.parse_metadata_block();
                if !m.is_empty() { pending_metadatas.extend(m); continue; }
            }
            if self.match_token(&Token::At) {
                let span = self.current_span();
                let name = match self.match_identifier_or_keyword() {
                    Some(n) => n,
                    None => { self.diagnostics.emit_error(self.current_span(), "期望注解名"); break; }
                };
                let mut arguments = Vec::new();
                let generic_type = if self.match_token(&Token::Less) { let gt = self.parse_type_ref().ok(); self.expect_token(&Token::Greater).ok(); gt } else { None };
                if self.match_token(&Token::LParen) {
                    if !self.check(&Token::RParen) { loop {
                        let param_name = self.match_identifier().unwrap_or_default();
                        if self.match_token(&Token::Assign) {
                            if let Ok(v) = self.parse_expression() { arguments.push((param_name, v)); }
                        }
                        if !self.match_token(&Token::Comma) { break; }
                    } }
                    self.expect_token(&Token::RParen).ok();
                }
                annotations.push(Annotation { name, generic_type, arguments, metadatas: std::mem::take(&mut pending_metadatas), span });
            } else if pending_metadatas.is_empty() {
                break;
            } else {
                pending_metadatas.clear();
                break;
            }
        }
        annotations
    }

    /// 解析 metadata 块 `[ type name = expr , ... ]`，返回条目列表（G4）
    fn parse_metadata_block(&mut self) -> Vec<MetadataEntry> {
        let mut entries = Vec::new();
        if !self.match_token(&Token::LBracket) { return entries; }
        while !self.is_at_end() && !self.check(&Token::RBracket) {
            // 用 parse_type_ref() 替代硬编码类型匹配，支持复杂类型名
            // 如 delegate<float:DremuLane^>、ColorArgb^、FunctionCurve^[]^
            let saved = self.pos;
            let type_name = match self.parse_type_ref() {
                Ok(type_ref) => type_ref.to_string(),
                Err(()) => {
                    self.pos = saved;
                    self.skip_until_rbracket_or_comma();
                    continue;
                }
            };
            if type_name.is_empty() { break; }
            let name = self.match_identifier().unwrap_or_default();
            if name.is_empty() { break; }
            let value = if self.match_token(&Token::Assign) { self.parse_expression().ok() } else { None };
            entries.push(MetadataEntry { type_name, name, value });
            self.match_token(&Token::Comma);
        }
        self.match_token(&Token::RBracket);
        entries
    }

    fn skip_metadata_block(&mut self) {
        // 已改为 parse，保留兼容旧调用
        let _ = self.parse_metadata_block();
    }

    /// 在元数据块中跳过无法解析的条目，直到遇到 `,`、`]` 或文件末尾
    fn skip_until_rbracket_or_comma(&mut self) {
        while !self.is_at_end() && !self.check(&Token::RBracket) && !self.check(&Token::Comma) {
            self.advance();
        }
        self.match_token(&Token::Comma);
    }

    /// 解析修饰符列表
    fn parse_modifiers(&mut self) -> Vec<Modifier> {
        let mut modifiers = Vec::new();
        loop {
            let m = match self.peek().map(|t| &t.token) {
                Some(Token::KwNative) => Some(Modifier::Native),
                Some(Token::KwStatic) => Some(Modifier::Static),
                Some(Token::KwInjector) => Some(Modifier::Injector),
                _ => None,
            };
            if let Some(modifier) = m {
                self.advance();
                // K1c: 检测重复修饰符
                if modifiers.contains(&modifier) {
                    self.diagnostics.emit_error(
                        self.current_span(),
                        format!("重复的修饰符 `{:?}`", modifier),
                    );
                }
                modifiers.push(modifier);
            } else {
                break;
            }
        }
        modifiers
    }

    /// 解析注入器字段定义
    fn parse_injector_fields(&mut self) -> Result<Vec<InjectorField>, ()> {
        let mut fields = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            let field_type = self.parse_type_ref()?;
            let name = self.match_identifier().ok_or_else(|| {
                self.diagnostics.emit_error(self.current_span(), "期望注入器字段名");
                ()
            })?;
            let span = self.current_span();
            self.expect_semicolon()?;
            fields.push(InjectorField { name, field_type, span });
        }
        Ok(fields)
    }

    // ==================== 语句解析 ====================

    /// 解析一条语句
    fn parse_statement(&mut self) -> Result<Statement, ()> {
        match self.peek() {
            Some(TokenSpan { token: Token::LBrace, .. }) => {
                let statements = self.parse_block()?;
                let span = self.last_span();
                Ok(Statement::Block { statements, span })
            }
            Some(TokenSpan { token: Token::KwIf, .. }) => self.parse_if_statement(),
            Some(TokenSpan { token: Token::KwDo, .. }) => self.parse_do_while_statement(),
            Some(TokenSpan { token: Token::KwWhile, .. }) => self.parse_while_statement(),
            Some(TokenSpan { token: Token::KwFor, .. }) => self.parse_for_statement(),
            Some(TokenSpan { token: Token::KwSwitch, .. }) => self.parse_switch_statement(),
            Some(TokenSpan { token: Token::KwReturn, .. }) => self.parse_return_statement(),
            Some(TokenSpan { token: Token::KwBreak, .. }) => {
                let span = self.advance().unwrap().span;
                let targets = self.parse_break_targets();
                self.expect_semicolon()?;
                Ok(Statement::Break { targets, span })
            }
            Some(TokenSpan { token: Token::KwContinue, .. }) => {
                let span = self.advance().unwrap().span;
                let targets = self.parse_break_targets();
                self.expect_semicolon()?;
                Ok(Statement::Continue { targets, span })
            }
            // var 声明或表达式语句
            _ => self.parse_declaration_or_expression_statement(),
        }
    }

    /// 解析 break/continue 的多层离块目标序列
    ///
    /// 语法：`break`/`continue` 后可跟若干目标，直到 `;`：
    /// - 整数字面量 → `BreakTarget::ByLayer(n)`（跳出 n 层）
    /// - 关键字 `for`/`while`/`switch`/`do`/`if`/`else` → `BreakTarget::ByKeyword`（按块类型跳出）
    ///
    /// 无目标时默认 `[ByLayer(1)]`（跳出当前一层）。
    fn parse_break_targets(&mut self) -> Vec<BreakTarget> {
        let mut targets = Vec::new();
        loop {
            match self.peek().map(|t| &t.token) {
                Some(Token::IntLiteral(n)) => {
                    let n = *n;
                    self.advance();
                    targets.push(BreakTarget::ByLayer(n.max(0) as u32));
                }
                Some(Token::KwFor) => { self.advance(); targets.push(BreakTarget::ByKeyword("for".into())); }
                Some(Token::KwWhile) => { self.advance(); targets.push(BreakTarget::ByKeyword("while".into())); }
                Some(Token::KwSwitch) => { self.advance(); targets.push(BreakTarget::ByKeyword("switch".into())); }
                Some(Token::KwDo) => { self.advance(); targets.push(BreakTarget::ByKeyword("do".into())); }
                Some(Token::KwIf) => { self.advance(); targets.push(BreakTarget::ByKeyword("if".into())); }
                Some(Token::KwElse) => { self.advance(); targets.push(BreakTarget::ByKeyword("else".into())); }
                _ => break,
            }
        }
        if targets.is_empty() {
            targets.push(BreakTarget::ByLayer(1));
        }
        targets
    }

    /// 解析代码块 `{ statements }`
    fn parse_block(&mut self) -> Result<Vec<Statement>, ()> {
        self.expect_token(&Token::LBrace)?;
        let mut statements = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        self.expect_token(&Token::RBrace)?;
        Ok(statements)
    }

    /// 解析 if 语句（关键字已检查）
    fn parse_if_statement(&mut self) -> Result<Statement, ()> {
        let span = self.advance().unwrap().span;
        self.expect_token(&Token::LParen)?;
        let condition = self.parse_expression()?;
        self.expect_token(&Token::RParen)?;
        let then_branch = Box::new(self.parse_statement()?);
        let else_branch = if self.match_token(&Token::KwElse) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
            span,
        })
    }

    /// 解析 while 语句（关键字已检查）
    fn parse_while_statement(&mut self) -> Result<Statement, ()> {
        let span = self.advance().unwrap().span;
        self.expect_token(&Token::LParen)?;
        let condition = self.parse_expression()?;
        self.expect_token(&Token::RParen)?;
        let body = Box::new(self.parse_statement()?);

        Ok(Statement::While { condition, body, span })
    }

    /// 解析 for 语句（关键字已检查）
    fn parse_for_statement(&mut self) -> Result<Statement, ()> {
        let span = self.advance().unwrap().span;
        self.expect_token(&Token::LParen)?;

        // 初始器：直接解析（类型声明或表达式），不消费 ; 作为语句终结符
        let initializer = if self.check(&Token::Semicolon) {
            None
        } else if self.is_type_start() {
            let var_type = self.parse_type_ref()?;
            let name = self.match_identifier().ok_or_else(|| {
                self.diagnostics.emit_error(self.current_span(), "期望变量名");
                ()
            })?;
            let init_expr = if self.match_token(&Token::Assign) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            Some(Box::new(Statement::VariableDeclaration {
                var_type,
                name,
                initializer: init_expr,
                span,
            }))
        } else {
            let expr = self.parse_expression()?;
            Some(Box::new(Statement::Expression(expr, span)))
        };
        self.expect_token(&Token::Semicolon)?;

        let condition = if self.check(&Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect_token(&Token::Semicolon)?;

        let update = if self.check(&Token::RParen) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect_token(&Token::RParen)?;

        let body = Box::new(self.parse_statement()?);

        Ok(Statement::For {
            initializer,
            condition,
            update,
            body,
            span,
        })
    }

    /// 解析 do-while 语句：`do { body } while (condition);`
    fn parse_do_while_statement(&mut self) -> Result<Statement, ()> {
        let span = self.advance().unwrap().span;
        let body = Box::new(self.parse_statement()?);
        self.expect_token(&Token::KwWhile)?;
        self.expect_token(&Token::LParen)?;
        let condition = self.parse_expression()?;
        self.expect_token(&Token::RParen)?;
        self.expect_semicolon()?;
        Ok(Statement::DoWhile { body, condition, span })
    }

    /// 解析 switch 语句（关键字已检查）
    fn parse_switch_statement(&mut self) -> Result<Statement, ()> {
        let span = self.advance().unwrap().span;
        self.expect_token(&Token::LParen)?;
        let expression = self.parse_expression()?;
        self.expect_token(&Token::RParen)?;

        self.expect_token(&Token::LBrace)?;
        let mut cases = Vec::new();
        let mut default_body = None;

        while !self.check(&Token::RBrace) && !self.is_at_end() {
            if self.match_token(&Token::KwCase) {
                let case_span = self.current_span();
                // case 可以有多个值：`case 1, 2:`
                let mut values = Vec::new();
                loop {
                    values.push(self.parse_expression()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
                self.expect_token(&Token::Colon)?;

                let mut body = Vec::new();
                while !self.check(&Token::KwCase)
                    && !self.check(&Token::KwDefault)
                    && !self.check(&Token::RBrace)
                    && !self.is_at_end()
                {
                    body.push(self.parse_statement()?);
                }
                cases.push(CaseBlock { values, body, span: case_span });
            } else if self.match_token(&Token::KwDefault) {
                self.expect_token(&Token::Colon)?;
                let mut body = Vec::new();
                while !self.check(&Token::KwCase)
                    && !self.check(&Token::KwDefault)
                    && !self.check(&Token::RBrace)
                    && !self.is_at_end()
                {
                    body.push(self.parse_statement()?);
                }
                default_body = Some(Box::new(Statement::Block {
                    statements: body,
                    span,
                }));
            } else {
                self.advance();
            }
        }
        self.expect_token(&Token::RBrace)?;

        Ok(Statement::Switch {
            expression,
            cases,
            default_body,
            span,
        })
    }

    /// 解析 return 语句（关键字已检查）
    fn parse_return_statement(&mut self) -> Result<Statement, ()> {
        let span = self.advance().unwrap().span;
        let value = if self.check(&Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect_semicolon()?;
        Ok(Statement::Return { value, span })
    }

    /// 解析变量声明或表达式语句
    ///
    /// 策略：如果当前 token 看起来像类型声明（var 或已知类型关键字后跟标识符），
    /// 尝试解析为变量声明；否则解析为表达式。
    fn parse_declaration_or_expression_statement(&mut self) -> Result<Statement, ()> {
        // 检查是否为 var 声明 `var name = expr;`
        if self.match_token(&Token::KwAuto) {
            let name = self.match_identifier().ok_or_else(|| {
                self.diagnostics.emit_error(self.current_span(), "期望变量名");
                ()
            })?;
            let span = self.current_span();

            // 暂时用 TypeRef::simple("var") 代替真实类型
            let var_type = TypeRef::simple("var", span);

            let initializer = if self.match_token(&Token::Assign) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            self.expect_semicolon()?;

            return Ok(Statement::VariableDeclaration {
                var_type,
                name,
                initializer,
                span,
            });
        }

        // 检查是否为类型声明 `TypeName name = expr;` 或 `TypeName name;`
        if self.is_type_start() {
            // 尝试解析类型名 + 标识符 + 可能有的 = expr
            let saved_pos = self.pos;
            let _saved_diags_count = self.diagnostics.error_count();

            if let Ok(var_type) = self.parse_type_ref() {
                if let Some(name) = self.match_identifier() {
                    let span = self.current_span();
                    let initializer = if self.match_token(&Token::Assign) {
                        match self.parse_expression() {
                            Ok(expr) => Some(expr),
                            Err(_) => {
                                // 恢复位置
                                self.pos = saved_pos;
                                // 但诊断已经被添加，跳转到表达式解析
                                return self.parse_expression_statement();
                            }
                        }
                    } else {
                        None
                    };
                    self.expect_semicolon()?;

                    return Ok(Statement::VariableDeclaration {
                        var_type,
                        name,
                        initializer,
                        span,
                    });
                }
            }

            // 恢复位置，按表达式处理
            self.pos = saved_pos;
        }

        self.parse_expression_statement()
    }

    /// 判断当前位置是否可能是类型声明的开头
    fn is_type_start(&self) -> bool {
        matches!(
            self.peek().map(|t| &t.token),
            Some(Token::TypeInt)
                | Some(Token::TypeFloat)
                | Some(Token::TypeBool)
                | Some(Token::TypeString)
                | Some(Token::TypeVoid)
                | Some(Token::TypeObject)
                | Some(Token::KwDelegate)
                | Some(Token::Identifier(_))
        )
    }

    /// 表达式语句 `expr;`
    fn parse_expression_statement(&mut self) -> Result<Statement, ()> {
        let expr = self.parse_expression()?;
        let span = expr.span();
        self.expect_semicolon()?;
        Ok(Statement::Expression(expr, span))
    }

    // ==================== 表达式解析 (Pratt) ====================

    /// 解析表达式（从最低优先级开始）
    pub fn parse_expression(&mut self) -> Result<Expression, ()> {
        self.parse_expression_with_precedence(PREC_ASSIGNMENT)
    }

    /// Pratt 解析器核心循环
    ///
    /// 先解析前缀表达式（操作数），然后循环解析中缀/后缀操作符。
    /// 只要下一个操作符的优先级不低于 min_precedence，就继续绑定。
    fn parse_expression_with_precedence(&mut self, min_precedence: u8) -> Result<Expression, ()> {
        let mut left = self.parse_prefix()?;

        while !self.is_at_end() {
            // `:` 只在后跟 `{`（注入器）或 `(`（Lambda）时作为中缀操作符
            // 否则留给条件表达式 `?:` 的 else 分支消费
            // 但当 suppress_colon_precheck 为 true 时跳过，避免在 `?:` 内部误消费 `:`
            if !self.suppress_colon_precheck && self.check(&Token::Colon) {
                if let Some(tok) = self.peek_ahead(1) {
                    if matches!(&tok.token, Token::LBrace | Token::LParen) {
                        left = self.parse_infix(left, PREC_POSTFIX)?;
                        continue;
                    }
                }
                // `:` 后不是 { 或 ( → 退出，让上层 ? 处理
                break;
            }

            let precedence = self.current_infix_precedence();
            if precedence < min_precedence {
                break;
            }
            left = self.parse_infix(left, precedence)?;
        }

        Ok(left)
    }

    /// 尝试解析强制类型转换 `(Type)expr`（调用时 `(` 已被消费）
    ///
    /// 消歧策略：
    /// - 内建类型关键字 `(int)`/`(float)`/`(bool)`/`(string)`/`(object)` 后接 `)` → 一定是转换；
    /// - 标识符 `(ClassName)` 后接 `)` 且其后紧跟可开启一元表达式的 token（标识符/字面量/
    ///   `(`/`this`/`new`/`!`/`-`）→ 视为转换，避免与括号表达式 `(x) op y` 混淆。
    ///
    /// 返回 `Ok(Some(cast))` 表示已解析为转换；`Ok(None)` 表示不是转换，调用方回退为括号表达式。
    fn try_parse_cast(&mut self, span: Span) -> Result<Option<Expression>, ()> {
        // 当前 token 是否为类型名（内建关键字或标识符），且其后为 `)`
        let (is_builtin, is_ident) = match self.peek().map(|t| &t.token) {
            Some(Token::TypeInt) | Some(Token::TypeFloat) | Some(Token::TypeBool)
            | Some(Token::TypeString) | Some(Token::TypeObject) => (true, false),
            Some(Token::Identifier(_)) => (false, true),
            _ => (false, false),
        };
        if !is_builtin && !is_ident {
            return Ok(None);
        }
        // 其后必须是 `)`
        if !matches!(self.peek_ahead(1).map(|t| &t.token), Some(Token::RParen)) {
            return Ok(None);
        }
        // 标识符情形需进一步确认 `)` 后是可开启表达式的 token（消歧）
        if is_ident {
            let after = self.peek_ahead(2).map(|t| &t.token);
            let starts_expr = matches!(
                after,
                Some(Token::Identifier(_))
                    | Some(Token::IntLiteral(_))
                    | Some(Token::FloatLiteral(_))
                    | Some(Token::StringLiteral(_))
                    | Some(Token::KwTrue)
                    | Some(Token::KwFalse)
                    | Some(Token::LParen)
                    | Some(Token::KwThis)
                    | Some(Token::KwNew)
                    | Some(Token::Bang)
            );
            if !starts_expr {
                return Ok(None);
            }
        }
        // 解析类型引用
        let target_type = self.parse_type_ref()?;
        self.expect_token(&Token::RParen)?;
        // 解析被转换的一元表达式（右结合，绑定到 cast）
        // 按一元优先级解析（含后缀 `.`/`()`/`[]`，高于二元运算），
        // 使 `(T) a.b()` 正确解析为 `(T)(a.b())` 而非 `((T)a).b()`
        let expression = Box::new(self.parse_expression_with_precedence(PREC_UNARY)?);
        Ok(Some(Expression::Cast { target_type, expression, span }))
    }

    /// 解析前缀表达式
    fn parse_prefix(&mut self) -> Result<Expression, ()> {
        let token_span = match self.advance() {
            Some(t) => t,
            None => {
                self.diagnostics
                    .emit_error(Span::dummy(), "期望表达式，但已到达文件末尾");
                return Err(());
            }
        };
        let span = token_span.span;

        match token_span.token {
            // 字面量
            Token::IntLiteral(v) => Ok(Expression::Literal(Literal::Int(v), span)),
            Token::FloatLiteral(v) => Ok(Expression::Literal(Literal::Float(v), span)),
            Token::StringLiteral(v) => Ok(Expression::Literal(Literal::String(v), span)),
            Token::KwTrue => Ok(Expression::Literal(Literal::Bool(true), span)),
            Token::KwFalse => Ok(Expression::Literal(Literal::Bool(false), span)),
            Token::KwNull => Ok(Expression::Null(span)),

            // 注入器字段引用 `^fieldName`
            Token::Caret => {
                let name = self.match_identifier().ok_or_else(|| {
                    self.diagnostics.emit_error(self.current_span(), "注入器字段引用 `^` 后期望标识符");
                    ()
                })?;
                Ok(Expression::InjectorFieldRef(name, span))
            }

            // 标识符（可能是变量引用、类型名或函数调用的一部分）
            Token::Identifier(name) => {
                // 检查是否为 lambda 参数：`x => expr`
                if self.match_token(&Token::Arrow) {
                    let body = if self.check(&Token::LBrace) {
                        LambdaBody::Block(self.parse_block()?)
                    } else {
                        LambdaBody::Expression(Box::new(self.parse_expression()?))
                    };
                    return Ok(Expression::Lambda {
                        parameters: vec![Parameter {
                            name,
                            param_type: TypeRef::simple("var", span),
                            span,
                        }],
                        body,
                        span,
                    });
                }
                // 检查是否为带类型 Lambda（含数组/注入器后缀）：
                // `Name:(params) -> body` 或 `Name[]:(params) -> body` 或 `Name^:(params) -> body`
                // 用 peek_ahead 检测，不消费 token、不产生 diagnostic
                let mut la: usize = 0;
                while self.peek_ahead(la).map(|t| &t.token) == Some(&Token::LBracket)
                    && self.peek_ahead(la + 1).map(|t| &t.token) == Some(&Token::RBracket)
                {
                    la += 2;
                }
                while self.peek_ahead(la).map(|t| &t.token) == Some(&Token::Caret) {
                    la += 1;
                }
                if self.peek_ahead(la).map(|t| &t.token) == Some(&Token::Colon)
                    && self.peek_ahead(la + 1).map(|t| &t.token) == Some(&Token::LParen)
                {
                    // 确认是带类型 Lambda：`Name:(params) ->` 或 `Name[]:(params) ->` 或 `Name^:(params) ->`
                    // name 已被 parse_prefix 的 Token::Identifier(name) 消费，无需再次 parse_type_ref
                    // 先消费 [] 和 ^ 后缀（与 lookahead 中 la 的步进一致）
                    for _ in 0..la {
                        self.advance();
                    }
                    self.advance(); // 消费 ':'
                    self.advance(); // 消费 '('
                    let params = self.parse_typed_parameters()?;
                    self.expect_token(&Token::RParen)?;
                    self.expect_token(&Token::LambdaArrow)?;
                    let body = if self.check(&Token::LBrace) {
                        LambdaBody::Block(self.parse_block()?)
                    } else {
                        LambdaBody::Expression(Box::new(self.parse_expression()?))
                    };
                    return Ok(Expression::Lambda { parameters: params, body, span });
                }
                Ok(Expression::Identifier(name, span))
            }

            // 类型关键字作为表达式前缀（用于 Lambda 语法 `int:(params) -> body`）
            Token::TypeInt => self.parse_type_keyword_or_lambda("int", span),
            Token::TypeFloat => self.parse_type_keyword_or_lambda("float", span),
            Token::TypeBool => self.parse_type_keyword_or_lambda("bool", span),
            Token::TypeString => self.parse_type_keyword_or_lambda("string", span),
            Token::TypeVoid => self.parse_type_keyword_or_lambda("void", span),
            Token::TypeObject => self.parse_type_keyword_or_lambda("object", span),

            // 一元前缀操作符
            Token::Minus => {
                let operand = self.parse_expression_with_precedence(PREC_UNARY)?;
                Ok(Expression::Unary {
                    operator: UnaryOp::Negate,
                    operand: Box::new(operand),
                    span,
                })
            }
            Token::Bang => {
                let operand = self.parse_expression_with_precedence(PREC_UNARY)?;
                Ok(Expression::Unary {
                    operator: UnaryOp::Not,
                    operand: Box::new(operand),
                    span,
                })
            }
            Token::PlusPlus => {
                let operand = self.parse_expression_with_precedence(PREC_UNARY)?;
                Ok(Expression::Unary {
                    operator: UnaryOp::PreIncrement,
                    operand: Box::new(operand),
                    span,
                })
            }
            Token::MinusMinus => {
                let operand = self.parse_expression_with_precedence(PREC_UNARY)?;
                Ok(Expression::Unary {
                    operator: UnaryOp::PreDecrement,
                    operand: Box::new(operand),
                    span,
                })
            }

            // 括号分组
            Token::LParen => {
                // 尝试识别强制类型转换 `(Type)expr`
                if let Some(cast) = self.try_parse_cast(span)? {
                    return Ok(cast);
                }
                let inner = self.parse_expression()?;
                self.expect_token(&Token::RParen)?;
                Ok(inner)
            }

            // new 表达式
            Token::KwNew => self.parse_new_expression(span),

            // this / base 引用
            Token::KwThis => Ok(Expression::This(span)),
            Token::KwSuper => Ok(Expression::Super(span)),

            // 注入器对象 `{ key: value, ... }`
            Token::LBrace => self.parse_injector_object(span),

            // 注入器数组 `[elem1, elem2, ...]`
            Token::LBracket => self.parse_injector_or_array(span),

            _ => {
                self.diagnostics.emit_error(
                    span,
                    format!("意外的 token，期望表达式"),
                );
                Err(())
            }
        }
    }

    /// 解析 new 表达式：`new Type(args)` / `new Type[size]` / `new ^field(args)` / `new var(args)`
    fn parse_new_expression(&mut self, span: Span) -> Result<Expression, ()> {
        // 注入器字段构造：`new ^field(args)` —— 在类型路径检查之前处理，避免
        // `parse_expression()` 将 `^field(args)` 整体消耗为 MethodCall 导致参数丢失
        if self.check(&Token::Caret) {
            self.advance(); // ^
            let field_name = self.match_identifier().ok_or_else(|| {
                self.diagnostics.emit_error(self.current_span(), "期望注入器字段名");
                ()
            })?;
            let arguments = if self.match_token(&Token::LParen) {
                let args = self.parse_argument_list()?;
                self.expect_token(&Token::RParen)?;
                args
            } else {
                Vec::new()
            };
            return Ok(Expression::InjectorNew {
                injector_field: field_name,
                args: arguments,
                span,
            });
        }

        let saved = self.pos;

        // 如果当前 token 是类型开头（关键字/标识符/delegate），尝试类型路径
        let is_type_start = matches!(
            self.peek().map(|t| &t.token),
            Some(Token::TypeInt)
                | Some(Token::TypeFloat)
                | Some(Token::TypeBool)
                | Some(Token::TypeString)
                | Some(Token::TypeVoid)
                | Some(Token::TypeObject)
                | Some(Token::KwDelegate)
                | Some(Token::Identifier(_))
        );

        if is_type_start {
            if let Ok(class_type) = self.parse_type_ref() {
                // new Type(args)
                if self.check(&Token::LParen) {
                    let arguments = if self.match_token(&Token::LParen) {
                        let args = self.parse_argument_list()?;
                        self.expect_token(&Token::RParen)?;
                        args
                    } else {
                        Vec::new()
                    };
                    // 可选注入器初始化 `: { fields }` 或直接 `{ fields }`
                    let injector = if self.check(&Token::Colon)
                        && matches!(self.peek_ahead(1).map(|t| &t.token), Some(Token::LBrace))
                    {
                        self.advance(); // :
                        self.advance(); // {
                        let fields = self.parse_ctor_injector_fields()?;
                        Some(fields)
                    } else if self.check(&Token::LBrace) {
                        // 直接 `{ fields }` 无冒号形式
                        // 用 lookahead 区分注入器 `{ key: value }` 与代码块 `{ stmts }`
                        let la1 = self.peek_ahead(1).map(|t| &t.token);
                        let is_injector = match la1 {
                            Some(Token::Identifier(_)) => {
                                matches!(self.peek_ahead(2).map(|t| &t.token), Some(Token::Colon))
                            }
                            Some(Token::Colon) | Some(Token::Comma) => true,
                            Some(Token::RBrace) => true, // 空注入器 {}
                            _ => false,
                        };
                        if is_injector {
                            self.advance(); // {
                            let fields = self.parse_ctor_injector_fields()?;
                            Some(fields)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    return Ok(Expression::New { class_type, arguments, injector, span });
                }
                // new Type[size] — 数组构造
                if self.check(&Token::LBracket) {
                    self.advance(); // [
                    let size = self.parse_expression()?;
                    self.expect_token(&Token::RBracket)?;
                    let mut args = vec![size];
                    // 可选的数组初始器 { elem1, elem2, ... } 或 {,}（空数组）
                    if self.check(&Token::LBrace) {
                        self.advance(); // {
                        if !self.check(&Token::Comma) {
                            while !self.is_at_end() && !self.check(&Token::RBrace) {
                                args.push(self.parse_expression()?);
                                self.match_token(&Token::Comma);
                            }
                        } else {
                            self.advance(); // 跳过逗号，处理空数组 {,}
                        }
                        self.expect_token(&Token::RBrace)?;
                    }
                    // 用 StaticMethodCall 暂代数组构造
                    let type_name = match &class_type {
                        TypeRef::Simple { name, .. } => name.clone(),
                        _ => "array".into(),
                    };
                    return Ok(Expression::StaticMethodCall {
                        class_name: type_name,
                        method: "new_array".into(),
                        arguments: args,
                        span,
                    });
                }
            }
            // 类型路径失败，错误已记录
            return Err(());
        }

        // 表达式路径: new ^field(args) 或 new var(args) 或 new (expr)(args)
        // 对于 new (expr)，需确保 LParen handler 只消费括号内的表达式而不消费 `[index]`、
        // `.member` 等后缀算子；否则 `new (^f)[i]` 会把 `[i]` 吸入 target_expr
        self.pos = saved;
        let target_expr = if self.check(&Token::LParen) {
            self.advance(); // (
            let inner = self.parse_expression_with_precedence(PREC_POSTFIX + 1)?;
            self.expect_token(&Token::RParen)?;
            inner
        } else {
            self.parse_prefix()?
        };
        let arguments = if self.check(&Token::LParen) {
            self.advance(); // (
            let args = self.parse_argument_list()?;
            self.expect_token(&Token::RParen)?;
            args
        } else {
            Vec::new()
        };

        match &target_expr {
            Expression::InjectorFieldRef(name, _) => Ok(Expression::InjectorNew {
                injector_field: name.clone(),
                args: arguments,
                span,
            }),
            Expression::Identifier(name, _) => Ok(Expression::StaticMethodCall {
                class_name: name.clone(),
                method: String::new(),
                arguments,
                span,
            }),
            _ => Ok(Expression::StaticMethodCall {
                class_name: String::new(),
                method: String::new(),
                arguments,
                span,
            }),
        }
    }

    /// 解析构造器调用后的注入器字段初始化 `{ key: value, ... }` 并消费 `}`
    ///
    /// 调用前需已消费 `{`。支持 `{:}`/`{,}` 空注入器。
    fn parse_ctor_injector_fields(&mut self) -> Result<Vec<(String, Expression)>, ()> {
        let fields = if self.check(&Token::Colon) {
            self.advance();
            Vec::new()
        } else if self.check(&Token::Comma) {
            self.advance();
            Vec::new()
        } else {
            let mut f = Vec::new();
            while !self.check(&Token::RBrace) && !self.is_at_end() {
                let key = self.match_identifier().ok_or_else(|| {
                    self.diagnostics.emit_error(self.current_span(), "期望注入器字段名");
                    ()
                })?;
                self.expect_token(&Token::Colon)?;
                let val = self.parse_expression()?;
                f.push((key, val));
                if !self.match_token(&Token::Comma) { break; }
            }
            f
        };
        self.expect_token(&Token::RBrace)?;
        Ok(fields)
    }

    /// 与 `parse_ctor_injector_fields` 相同，但不消费结尾 `}`
    fn parse_ctor_injector_fields_no_rbrace(&mut self) -> Result<Vec<(String, Expression)>, ()> {
        if self.check(&Token::Colon) {
            self.advance();
            Ok(Vec::new())
        } else if self.check(&Token::Comma) {
            self.advance();
            Ok(Vec::new())
        } else {
            let mut f = Vec::new();
            while !self.check(&Token::RBrace) && !self.is_at_end() {
                let key = self.match_identifier().ok_or_else(|| {
                    self.diagnostics.emit_error(self.current_span(), "期望注入器字段名");
                    ()
                })?;
                self.expect_token(&Token::Colon)?;
                let val = self.parse_expression()?;
                f.push((key, val));
                if !self.match_token(&Token::Comma) { break; }
            }
            Ok(f)
        }
    }

    /// 解析注入器对象字面量 `{ key: value, ... }`
    ///
    /// 在表达式上下文中，`{` 后的内容通过 key: value 模式识别为注入器对象。
    fn parse_injector_object(&mut self, span: Span) -> Result<Expression, ()> {
        let mut fields = Vec::new();

        if self.check(&Token::RBrace) {
            self.advance();
            return Ok(Expression::InjectorObject { class_name: String::new(), fields, span });
        }

        loop {
            // 字段名：标识符
            let key = self.match_identifier().ok_or_else(|| {
                self.diagnostics.emit_error(self.current_span(), "期望注入器字段名");
                ()
            })?;

            // 冒号分隔
            self.expect_token(&Token::Colon)?;

            // 字段值
            let value = self.parse_expression()?;
            fields.push((key, value));

            if self.match_token(&Token::Comma) {
                if self.check(&Token::RBrace) {
                    // 尾随逗号允许
                    break;
                }
            } else {
                break;
            }
        }

        self.expect_token(&Token::RBrace)?;
        Ok(Expression::InjectorObject { class_name: String::new(), fields, span })
    }

    /// 解析注入器数组或普通数组 `[elem1, elem2, ...]`
    fn parse_injector_or_array(&mut self, span: Span) -> Result<Expression, ()> {
        let elements = if self.check(&Token::RBracket) {
            Vec::new()
        } else {
            self.parse_argument_list()?
        };
        self.expect_token(&Token::RBracket)?;
        Ok(Expression::InjectorArray { elements, span })
    }

    /// 解析注入器字面量内容 `{ ... }`（前面的 `{` 已被消费）
    ///
    /// 在 `expr : { ... }` 语法中使用。根据内容格式自动区分配置对象注入器
    /// （`{ key: value, ... }`）和数组注入器（`{ elem, elem, ... }`）。
    fn parse_injector_literal_content(&mut self, span: Span) -> Result<Expression, ()> {
        // 空注入器对象 `{:}`
        if self.match_token(&Token::Colon) {
            self.expect_token(&Token::RBrace)?;
            return Ok(Expression::InjectorObject { class_name: String::new(), fields: Vec::new(), span });
        }
        // 空注入器数组 `{,}`
        if self.match_token(&Token::Comma) {
            self.expect_token(&Token::RBrace)?;
            return Ok(Expression::InjectorArray { elements: Vec::new(), span });
        }
        // 直接闭合 `{}` — 视为空对象注入器
        if self.check(&Token::RBrace) {
            self.advance();
            return Ok(Expression::InjectorObject { class_name: String::new(), fields: Vec::new(), span });
        }

        // 有内容：lookahead 判读是 key:value 对还是纯表达式列表
        let saved = self.pos;
        let is_object = self.match_identifier().is_some() && self.check(&Token::Colon);
        self.pos = saved; // 回退

        if is_object {
            // 对象注入器：`{ key: value, key: value, ... }`
            let mut fields = Vec::new();
            loop {
                let key = self.match_identifier().ok_or_else(|| {
                    self.diagnostics.emit_error(self.current_span(), "期望注入器字段名");
                    ()
                })?;
                self.expect_token(&Token::Colon)?;
                let value = self.parse_expression()?;
                fields.push((key, value));

                if self.match_token(&Token::Comma) {
                    if self.check(&Token::RBrace) {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.expect_token(&Token::RBrace)?;
            Ok(Expression::InjectorObject { class_name: String::new(), fields, span })
        } else {
            // 数组注入器：`{ expr, expr, ... }`
            let mut elements = Vec::new();
            loop {
                elements.push(self.parse_expression()?);
                if self.match_token(&Token::Comma) {
                    if self.check(&Token::RBrace) {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.expect_token(&Token::RBrace)?;
            Ok(Expression::InjectorArray { elements, span })
        }
    }

    /// 解析参数/元素列表（逗号分隔的表达式）
    fn parse_argument_list(&mut self) -> Result<Vec<Expression>, ()> {
        let mut args = Vec::new();
        if self.check(&Token::RParen) || self.check(&Token::RBracket) {
            return Ok(args);
        }
        loop {
            args.push(self.parse_expression()?);
            if !self.match_token(&Token::Comma) {
                break;
            }
            // 允许尾随逗号
            if self.check(&Token::RParen) || self.check(&Token::RBracket) {
                break;
            }
        }
        Ok(args)
    }

    /// 获取当前 token 作为中缀操作符的优先级
    fn current_infix_precedence(&self) -> u8 {
        match self.peek().map(|t| &t.token) {
            Some(Token::Assign)
            | Some(Token::PlusAssign)
            | Some(Token::MinusAssign)
            | Some(Token::StarAssign)
            | Some(Token::SlashAssign)
            | Some(Token::PercentAssign) => PREC_ASSIGNMENT,

            Some(Token::Question) => PREC_CONDITIONAL,

            Some(Token::OrOr) => PREC_LOGICAL_OR,
            Some(Token::AndAnd) => PREC_LOGICAL_AND,

            Some(Token::EqualEqual) | Some(Token::NotEqual) => PREC_EQUALITY,

            Some(Token::Less)
            | Some(Token::LessEqual)
            | Some(Token::Greater)
            | Some(Token::GreaterEqual) => PREC_COMPARISON,

            Some(Token::Plus) | Some(Token::Minus) => PREC_ADDITION,

            Some(Token::Star) | Some(Token::Slash) | Some(Token::Percent) => PREC_MULTIPLICATION,

            // 后缀操作符（最高中缀优先级）
            Some(Token::Dot)
            | Some(Token::LParen)
            | Some(Token::LBracket)
            | Some(Token::Caret)
            | Some(Token::PlusPlus)
            | Some(Token::MinusMinus) => PREC_POSTFIX,

            _ => PREC_NONE,
        }
    }

    /// 解析中缀/后缀表达式
    fn parse_infix(&mut self, left: Expression, precedence: u8) -> Result<Expression, ()> {
        let token_span = self.advance().unwrap();
        let span = token_span.span;
        let span_start = left.span();

        match token_span.token {
            // 二元操作符
            Token::Plus
            | Token::Minus
            | Token::Star
            | Token::Slash
            | Token::Percent
            | Token::Less
            | Token::LessEqual
            | Token::Greater
            | Token::GreaterEqual
            | Token::EqualEqual
            | Token::NotEqual
            | Token::AndAnd
            | Token::OrOr => {
                // 左结合：用 precedence + 1 解析右操作数
                let right = self.parse_expression_with_precedence(precedence + 1)?;
                let op = token_to_binary_op(&token_span.token);
                let combined_span = Span::new(
                    span_start.start,
                    right.span().end,
                    span_start.line,
                    span_start.column,
                    span_start.source_id,
                );
                Ok(Expression::Binary {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                    span: combined_span,
                })
            }

            // 赋值操作符（右结合）
            Token::Assign
            | Token::PlusAssign
            | Token::MinusAssign
            | Token::StarAssign
            | Token::SlashAssign
            | Token::PercentAssign => {
                // 右结合：用相同优先级解析右操作数
                let right = self.parse_expression_with_precedence(precedence)?;
                let target = match left {
                    Expression::Identifier(name, s) => AssignmentTarget::Variable(name, s),
                    Expression::MemberAccess { object, member, span: _ms } => {
                        if let Some(field) = member.strip_prefix('^') {
                            AssignmentTarget::InjectorField { object, field: field.to_string(), span: _ms }
                        } else {
                            AssignmentTarget::Field { object, field: member, span: _ms }
                        }
                    }
                    Expression::ArrayAccess { array, index, span: aspan } => {
                        AssignmentTarget::ArrayElement { array, index, span: aspan }
                    }
                    _ => {
                        self.diagnostics.emit_error(
                            left.span(),
                            "赋值目标必须是变量、字段或数组元素",
                        );
                        return Err(());
                    }
                };
                let op = token_to_assignment_op(&token_span.token);
                Ok(Expression::Assignment {
                    target,
                    operator: op,
                    value: Box::new(right),
                    span,
                })
            }

            // 条件表达式 `?:`
            Token::Question => {
                // 抑制 `:` 预检，防止 `a ? expr1 : (expr2)` 中 `:` 被
                // parse_expression_with_precedence 的冒号预检误消费
                self.suppress_colon_precheck = true;
                let then_branch = self.parse_expression_with_precedence(PREC_ASSIGNMENT)?;
                self.suppress_colon_precheck = false;
                self.expect_token(&Token::Colon)?;
                self.suppress_colon_precheck = true;
                let else_branch = self.parse_expression_with_precedence(precedence)?;
                self.suppress_colon_precheck = false;
                Ok(Expression::Conditional {
                    condition: Box::new(left),
                    then_branch: Box::new(then_branch),
                    else_branch: Some(Box::new(else_branch)),
                    span,
                })
            }

            // 成员访问 `object.member`
            Token::Dot => {
                // 注入器字段访问 obj.^field
                if self.match_token(&Token::Caret) {
                    let member = self.match_identifier().ok_or_else(|| {
                        self.diagnostics.emit_error(self.current_span(), "期望注入器字段名");
                        ()
                    })?;
                    return Ok(Expression::MemberAccess {
                        object: Box::new(left),
                        member: format!("^{}", member),
                        span,
                    });
                }
                let member = self.match_identifier().ok_or_else(|| {
                    self.diagnostics.emit_error(self.current_span(), "期望成员名");
                    ()
                })?;
                Ok(Expression::MemberAccess {
                    object: Box::new(left),
                    member,
                    span,
                })
            }

            // 方法调用 `receiver(args)`
            Token::LParen => {
                let args = self.parse_argument_list()?;
                self.expect_token(&Token::RParen)?;

                // 根据 left 的类型构造对应的调用表达式
                match left {
                    Expression::Identifier(name, _) => {
                        // `func(args)` — 可能是静态调用或本地函数调用
                        Ok(Expression::StaticMethodCall {
                            class_name: String::new(),
                            method: name,
                            arguments: args,
                            span,
                        })
                    }
                    Expression::MemberAccess { object, member, span: _ms } => {
                        Ok(Expression::MethodCall {
                            receiver: object,
                            method: member,
                            arguments: args,
                            span,
                        })
                    }
                    other => {
                        // `expr()` — 委托调用，暂用 MethodCall 表示
                        Ok(Expression::MethodCall {
                            receiver: Box::new(other),
                            method: String::new(),
                            arguments: args,
                            span,
                        })
                    }
                }
            }

            // 数组访问 `array[index]`
            Token::LBracket => {
                let index = self.parse_expression()?;
                self.expect_token(&Token::RBracket)?;
                Ok(Expression::ArrayAccess {
                    array: Box::new(left),
                    index: Box::new(index),
                    span,
                })
            }

            // 后缀自增/自减
            Token::PlusPlus => Ok(Expression::Unary {
                operator: UnaryOp::PostIncrement,
                operand: Box::new(left),
                span,
            }),
            Token::MinusMinus => Ok(Expression::Unary {
                operator: UnaryOp::PostDecrement,
                operand: Box::new(left),
                span,
            }),

            // 注入器类型后缀 expr^、注入器字段访问 obj.^field
            Token::Caret => Ok(Expression::MemberAccess {
                object: Box::new(left),
                member: "^".into(),
                span,
            }),

            // Lambda: typeRef : (params) -> body 或注入器字面量: expr : { ... }
            Token::Colon => {
                if self.check(&Token::LParen) {
                    self.advance(); // (
                    let params = self.parse_typed_parameters()?;
                    self.expect_token(&Token::RParen)?;
                    self.expect_token(&Token::LambdaArrow)?; // ->
                    let body = if self.check(&Token::LBrace) {
                        LambdaBody::Block(self.parse_block()?)
                    } else {
                        LambdaBody::Expression(Box::new(self.parse_expression()?))
                    };
                    return Ok(Expression::Lambda {
                        parameters: params,
                        body,
                        span,
                    });
                }
                // 注入器字面量: expr : { key: value } 或 expr : { elem, elem }
                if self.check(&Token::LBrace) {
                    self.advance(); // 消费 '{'
                    let class_name = match &left {
                        Expression::Identifier(name, _) => name.clone(),
                        _ => String::new(),
                    };
                    let result = self.parse_injector_literal_content(span)?;
                    match result {
                        Expression::InjectorObject { fields, span, .. } => {
                            return Ok(Expression::InjectorObject { class_name, fields, span });
                        }
                        Expression::InjectorArray { elements, span } => {
                            return Ok(Expression::InjectorArray { elements, span });
                        }
                        other => return Ok(other),
                    }
                }
                self.diagnostics.emit_error(span, "Colon 后期望 Lambda 参数列表或注入器字面量");
                Err(())
            }

            _ => {
                self.diagnostics
                    .emit_error(span, "意外的中缀操作符");
                Err(())
            }
        }
    }
}

/// Token 转换为二元操作符
fn token_to_binary_op(token: &Token) -> BinaryOp {
    match token {
        Token::Plus => BinaryOp::Add,
        Token::Minus => BinaryOp::Subtract,
        Token::Star => BinaryOp::Multiply,
        Token::Slash => BinaryOp::Divide,
        Token::Percent => BinaryOp::Modulo,
        Token::Less => BinaryOp::Less,
        Token::LessEqual => BinaryOp::LessEqual,
        Token::Greater => BinaryOp::Greater,
        Token::GreaterEqual => BinaryOp::GreaterEqual,
        Token::EqualEqual => BinaryOp::Equal,
        Token::NotEqual => BinaryOp::NotEqual,
        Token::AndAnd => BinaryOp::LogicAnd,
        Token::OrOr => BinaryOp::LogicOr,
        _ => BinaryOp::Add, // 不应该到达这里
    }
}

/// Token 转换为赋值操作符
fn token_to_assignment_op(token: &Token) -> AssignmentOp {
    match token {
        Token::Assign => AssignmentOp::Assign,
        Token::PlusAssign => AssignmentOp::PlusAssign,
        Token::MinusAssign => AssignmentOp::MinusAssign,
        Token::StarAssign => AssignmentOp::StarAssign,
        Token::SlashAssign => AssignmentOp::SlashAssign,
        Token::PercentAssign => AssignmentOp::Assign,
        _ => AssignmentOp::Assign,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::tokenize;

    /// 辅助函数：对源码进行词法 + 语法分析，返回解析结果
    fn parse_source(source: &str) -> Result<SourceFile, Diagnostics> {
        let (tokens, lexer_diags) = tokenize(source, 0);
        if !lexer_diags.is_empty() {
            eprintln!("词法错误: {:?}", lexer_diags);
        }
        let mut parser = Parser::new(tokens);
        parser.parse_source_file()
    }

    /// 辅助函数：解析表达式
    fn parse_expr(source: &str) -> Result<Expression, Diagnostics> {
        let (tokens, _) = tokenize(source, 0);
        let mut parser = Parser::new(tokens);
        match parser.parse_expression() {
            Ok(expr) => {
                if parser.diagnostics.has_errors() {
                    Err(parser.into_diagnostics())
                } else {
                    Ok(expr)
                }
            }
            Err(_) => Err(parser.into_diagnostics()),
        }
    }

    /// 辅助函数：解析单条语句
    fn parse_stmt(source: &str) -> Statement {
        let (tokens, _) = tokenize(source, 0);
        let mut parser = Parser::new(tokens);
        parser.parse_statement().expect("语句解析失败")
    }

    #[test]
    fn test_parse_break_default_target() {
        // 无目标 break 默认 [ByLayer(1)]
        match parse_stmt("break;") {
            Statement::Break { targets, .. } => {
                assert_eq!(targets.len(), 1);
                assert!(matches!(targets[0], BreakTarget::ByLayer(1)));
            }
            other => panic!("应为 Break，实为 {:?}", other),
        }
    }

    #[test]
    fn test_parse_break_by_layer() {
        match parse_stmt("break 2;") {
            Statement::Break { targets, .. } => {
                assert_eq!(targets.len(), 1);
                assert!(matches!(targets[0], BreakTarget::ByLayer(2)));
            }
            other => panic!("应为 Break，实为 {:?}", other),
        }
    }

    #[test]
    fn test_parse_break_by_keyword() {
        match parse_stmt("break while;") {
            Statement::Break { targets, .. } => {
                assert_eq!(targets.len(), 1);
                assert!(matches!(&targets[0], BreakTarget::ByKeyword(k) if k == "while"));
            }
            other => panic!("应为 Break，实为 {:?}", other),
        }
    }

    #[test]
    fn test_parse_break_multi_targets() {
        // break for for → 两个 for 关键字目标
        match parse_stmt("break for for;") {
            Statement::Break { targets, .. } => {
                assert_eq!(targets.len(), 2);
                assert!(matches!(&targets[0], BreakTarget::ByKeyword(k) if k == "for"));
                assert!(matches!(&targets[1], BreakTarget::ByKeyword(k) if k == "for"));
            }
            other => panic!("应为 Break，实为 {:?}", other),
        }
    }

    #[test]
    fn test_parse_continue_by_layer() {
        match parse_stmt("continue 3;") {
            Statement::Continue { targets, .. } => {
                assert!(matches!(targets[0], BreakTarget::ByLayer(3)));
            }
            other => panic!("应为 Continue，实为 {:?}", other),
        }
    }

    #[test]
    fn test_parse_integer_literal() {
        let result = parse_expr("42").unwrap();
        assert!(matches!(result, Expression::Literal(Literal::Int(42), _)));
    }

    #[test]
    fn test_parse_float_literal() {
        let result = parse_expr("3.14").unwrap();
        assert!(matches!(result, Expression::Literal(Literal::Float(v), _) if (v - 3.14).abs() < 0.001));
    }

    #[test]
    fn test_parse_string_literal() {
        let result = parse_expr(r#""hello""#).unwrap();
        assert!(matches!(result, Expression::Literal(Literal::String(s), _) if s == "hello"));
    }

    #[test]
    fn test_parse_identifier() {
        let result = parse_expr("myVar").unwrap();
        assert!(matches!(result, Expression::Identifier(name, _) if name == "myVar"));
    }

    #[test]
    fn test_parse_binary_add() {
        let result = parse_expr("1 + 2").unwrap();
        assert!(matches!(result, Expression::Binary { operator: BinaryOp::Add, .. }));
    }

    #[test]
    fn test_parse_binary_mul() {
        let result = parse_expr("3 * 4").unwrap();
        assert!(matches!(result, Expression::Binary { operator: BinaryOp::Multiply, .. }));
    }

    #[test]
    fn test_parse_comparison() {
        let result = parse_expr("x < y").unwrap();
        assert!(matches!(result, Expression::Binary { operator: BinaryOp::Less, .. }));
    }

    #[test]
    fn test_parse_equality() {
        let result = parse_expr("a == b").unwrap();
        assert!(matches!(result, Expression::Binary { operator: BinaryOp::Equal, .. }));
    }

    #[test]
    fn test_parse_logical_and() {
        let result = parse_expr("true && false").unwrap();
        assert!(matches!(result, Expression::Binary { operator: BinaryOp::LogicAnd, .. }));
    }

    #[test]
    fn test_parse_logical_or() {
        let result = parse_expr("a || b").unwrap();
        assert!(matches!(result, Expression::Binary { operator: BinaryOp::LogicOr, .. }));
    }

    #[test]
    fn test_parse_precedence_mul_before_add() {
        // 应该解析为 (1 + (2 * 3))
        let result = parse_expr("1 + 2 * 3").unwrap();
        match result {
            Expression::Binary { left, operator: BinaryOp::Add, right, .. } => {
                assert!(matches!(*left, Expression::Literal(Literal::Int(1), _)));
                assert!(matches!(*right, Expression::Binary { operator: BinaryOp::Multiply, .. }));
            }
            _ => panic!("期望加法为顶层操作"),
        }
    }

    #[test]
    fn test_parse_parentheses() {
        let result = parse_expr("(1 + 2) * 3").unwrap();
        match result {
            Expression::Binary { left, operator: BinaryOp::Multiply, .. } => {
                assert!(matches!(*left, Expression::Binary { operator: BinaryOp::Add, .. }));
            }
            _ => panic!("期望乘法为顶层操作"),
        }
    }

    #[test]
    fn test_parse_unary_negate() {
        let result = parse_expr("-5").unwrap();
        assert!(matches!(result, Expression::Unary { operator: UnaryOp::Negate, .. }));
    }

    #[test]
    fn test_parse_unary_not() {
        let result = parse_expr("!flag").unwrap();
        assert!(matches!(result, Expression::Unary { operator: UnaryOp::Not, .. }));
    }

    #[test]
    fn test_parse_assignment() {
        let result = parse_expr("x = 10").unwrap();
        assert!(matches!(result, Expression::Assignment { operator: AssignmentOp::Assign, .. }));
    }

    #[test]
    fn test_parse_member_access() {
        let result = parse_expr("obj.field").unwrap();
        assert!(matches!(result, Expression::MemberAccess { member, .. } if member == "field"));
    }

    #[test]
    fn test_parse_method_call() {
        let result = parse_expr("obj.method()").unwrap();
        assert!(matches!(result, Expression::MethodCall { method, .. } if method == "method"));
    }

    #[test]
    fn test_parse_conditional() {
        let result = parse_expr("a ? b : c").unwrap();
        assert!(matches!(result, Expression::Conditional { .. }));
    }

    #[test]
    fn test_parse_null() {
        let result = parse_expr("null").unwrap();
        assert!(matches!(result, Expression::Null(_)));
    }

    #[test]
    fn test_parse_bool_literal() {
        let t = parse_expr("true").unwrap();
        assert!(matches!(t, Expression::Literal(Literal::Bool(true), _)));
        let f = parse_expr("false").unwrap();
        assert!(matches!(f, Expression::Literal(Literal::Bool(false), _)));
    }

    #[test]
    fn test_parse_class_with_method() {
        let source = "class Test { int getValue() { return 42; } }";
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Test");
                assert_eq!(c.members.len(), 1);
            }
            _ => panic!("期望类声明"),
        }
    }

    #[test]
    fn test_parse_class_declaration() {
        let source = "class MyClass { }";
        let ast = parse_source(source).unwrap();
        assert_eq!(ast.members.len(), 1);
        match &ast.members[0] {
            TopLevelMember::Class(c) => assert_eq!(c.name, "MyClass"),
            _ => panic!("期望类声明"),
        }
    }

    #[test]
    fn test_parse_class_with_field() {
        let source = "class Point { int x; float y; }";
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Point");
                assert_eq!(c.members.len(), 2);
            }
            _ => panic!("期望类声明"),
        }
    }

    #[test]
    fn test_static_field_modifier_rejected() {
        // Gorge 语法不允许字段带修饰符（只有方法可 static），应报编译错误
        let source = "class Config { static int count; }";
        let result = parse_source(source);
        assert!(result.is_err(), "static 字段应被拒绝");
    }

    #[test]
    fn test_plain_field_no_modifier_ok() {
        // 普通字段（无修饰符）应正常解析
        let source = "class Config { int count; }";
        assert!(parse_source(source).is_ok());
    }

    #[test]
    fn test_parse_interface() {
        let source = "interface IDrawable { void draw(); }";
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Interface(i) => {
                assert_eq!(i.name, "IDrawable");
                assert_eq!(i.methods.len(), 1);
            }
            _ => panic!("期望接口声明"),
        }
    }

    #[test]
    fn test_parse_enum() {
        let source = "enum Color { Red, Green, Blue }";
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Enum(e) => {
                assert_eq!(e.name, "Color");
                assert_eq!(e.values.len(), 3);
            }
            _ => panic!("期望枚举声明"),
        }
    }

    #[test]
    fn test_parse_class_with_inheritance() {
        let source = "class Dog : Animal :: IPet , IBark { }";
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Dog");
                assert!(c.super_class.is_some());
                assert_eq!(c.super_interfaces.len(), 2);
            }
            _ => panic!("期望类声明"),
        }
    }

    #[test]
    fn test_parse_namespace() {
        let source = "namespace MyGame; class Player { }";
        let ast = parse_source(source).unwrap();
        assert!(ast.namespace.is_some());
    }

    #[test]
    fn test_parse_namespace_interleaved() {
        let source = "namespace A; class X { } namespace B; class Y { }";
        let ast = parse_source(source).unwrap();
        // 最后一个 namespace 生效
        assert!(ast.namespace.is_some());
        assert_eq!(ast.members.len(), 2);
    }

    #[test]
    fn test_parse_using() {
        let source = "using System; class Foo { }";
        let ast = parse_source(source).unwrap();
        assert_eq!(ast.usings.len(), 1);
    }

    // ==================== 构造函数解析测试 ====================

    #[test]
    fn test_parse_class_with_constructor() {
        let source = "class Test4 { Test4(int x) { this.x = x; } }";
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Test4");
                assert_eq!(c.members.len(), 1);
                assert!(matches!(&c.members[0], ClassMember::Constructor(_)));
            }
            _ => panic!("期望类声明"),
        }
    }

    #[test]
    fn test_parse_class_with_constructor_and_super() {
        let source = "class TestB : TestA { TestB(int x) : super(x + 1) { value = x; } }";
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "TestB");
                assert_eq!(c.members.len(), 1);
                match &c.members[0] {
                    ClassMember::Constructor(ctor) => {
                        assert_eq!(ctor.parameters.len(), 1);
                        assert_eq!(ctor.base_arguments.len(), 1);
                        assert!(ctor.body.is_some());
                    }
                    _ => panic!("期望构造函数"),
                }
            }
            _ => panic!("期望类声明"),
        }
    }

    #[test]
    fn test_parse_class_with_injector_constructor() {
        let source = "class Test11A { injector Test11A(int i) { value = value + i; } }";
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                match &c.members[0] {
                    ClassMember::Constructor(ctor) => {
                        assert!(ctor.modifiers.iter().any(|m| matches!(m, Modifier::Injector)));
                        assert_eq!(ctor.parameters.len(), 1);
                    }
                    _ => panic!("期望构造函数"),
                }
            }
            _ => panic!("期望类声明"),
        }
    }

    #[test]
    fn test_parse_class_with_field_method_and_constructor() {
        let source = r#"
class Test4 {
    int counter;
    int increasment;
    Test4(int inc) {
        this.increasment = inc;
    }
    void SelfIncreasement() {
        counter = counter + 1;
    }
}
"#;
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Test4");
                assert_eq!(c.members.len(), 4);
                let field_count = c.members.iter().filter(|m| matches!(m, ClassMember::Field(_))).count();
                let ctor_count = c.members.iter().filter(|m| matches!(m, ClassMember::Constructor(_))).count();
                let method_count = c.members.iter().filter(|m| matches!(m, ClassMember::Method(_))).count();
                assert_eq!(field_count, 2, "应有 2 个字段");
                assert_eq!(ctor_count, 1, "应有 1 个构造函数");
                assert_eq!(method_count, 1, "应有 1 个方法");
            }
            _ => panic!("期望类声明"),
        }
    }

    #[test]
    fn test_parse_class_with_native_constructor() {
        let source = "native class Test8N { int gorgeField; Test8N(int a, int b); static int GetConst(); }";
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Test8N");
                assert_eq!(c.members.len(), 3);
                let ctor_count = c.members.iter().filter(|m| matches!(m, ClassMember::Constructor(_))).count();
                assert_eq!(ctor_count, 1, "应有 1 个构造函数");
                match &c.members[1] {
                    ClassMember::Constructor(ctor) => {
                        assert!(ctor.body.is_none(), "native 构造函数应无方法体");
                        assert_eq!(ctor.parameters.len(), 2);
                    }
                    _ => panic!("第 2 个成员应是构造函数"),
                }
            }
            _ => panic!("期望类声明"),
        }
    }

    // ==================== 真实 .g 文件解析验证 ====================

    /// 解析 Test4.g 的核心结构并验证构造函数 AST
    #[test]
    fn test_parse_test4g_constructor_and_this() {
        let source = r#"
class Test4
{
    int counter;
    int increasment;
    int selfIncreasement = -1;

    Test4(int increasment)
    {
        this.increasment = increasment;
    }

    void SelfIncreasement()
    {
        counter = counter + selfIncreasement;
    }

    static int DoTest()
    {
        Test4 t = new Test4(2);
        t.counter = 0;
        for(int j = 0; j < 100000000; j = j + 1)
        {
            t.counter = t.counter + t.increasment;
            t.SelfIncreasement();
        }
        return t.counter;
    }
}
"#;
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Test4");
                assert_eq!(c.members.len(), 6);

                // 验证三个字段
                let fields: Vec<_> = c.members.iter()
                    .filter_map(|m| if let ClassMember::Field(f) = m { Some(f) } else { None })
                    .collect();
                assert_eq!(fields.len(), 3);
                assert_eq!(fields[0].name, "counter");
                assert_eq!(fields[1].name, "increasment");
                assert_eq!(fields[2].name, "selfIncreasement");
                assert!(fields[2].initializer.is_some(), "selfIncreasement 应有初始值 -1");

                // 验证构造函数
                let ctors: Vec<_> = c.members.iter()
                    .filter_map(|m| if let ClassMember::Constructor(ctor) = m { Some(ctor) } else { None })
                    .collect();
                assert_eq!(ctors.len(), 1, "应有 1 个构造函数");
                let ctor = ctors[0];
                assert_eq!(ctor.parameters.len(), 1);
                assert_eq!(ctor.parameters[0].name, "increasment");
                assert!(matches!(&ctor.parameters[0].param_type, TypeRef::Simple { name, .. } if name == "int"));
                assert!(ctor.base_arguments.is_empty(), "无 super() 调用");
                assert!(ctor.body.is_some(), "应有构造函数体");
                let body = ctor.body.as_ref().unwrap();
                assert_eq!(body.len(), 1);

                // 验证构造函数体中的赋值语句：this.increasment = increasment;
                match &body[0] {
                    Statement::Expression(expr, _) => {
                        match expr {
                            Expression::Assignment { target, operator, .. } => {
                                assert!(matches!(operator, AssignmentOp::Assign));
                                match target {
                                    AssignmentTarget::Field { object, field, .. } => {
                                        assert!(matches!(**object, Expression::This(_)), "赋值目标是 this.field");
                                        assert_eq!(field, "increasment");
                                    }
                                    _ => panic!("赋值目标应为字段"),
                                }
                            }
                            _ => panic!("构造函数首语句应为赋值"),
                        }
                    }
                    _ => panic!("构造函数体应有表达式语句"),
                }

                // 验证 void SelfIncreasement 方法
                let methods: Vec<_> = c.members.iter()
                    .filter_map(|m| if let ClassMember::Method(meth) = m { Some(meth) } else { None })
                    .collect();
                assert_eq!(methods.len(), 2);
                assert_eq!(methods[0].name, "SelfIncreasement");
                assert_eq!(methods[1].name, "DoTest");
                assert!(methods[1].modifiers.iter().any(|m| matches!(m, Modifier::Static)), "DoTest 应为 static");
            }
            _ => panic!("期望类声明"),
        }
    }

    /// 解析 Test5.g 的多类 + 继承 + super() 结构
    #[test]
    fn test_parse_test5g_multi_class_with_super() {
        let source = r#"
class Test5A
{
    int valueA;
    Test5A(int value)
    {
        valueA = value;
    }
    int GetValue() { return valueA; }
    int GetValueA() { return valueA; }
}
class Test5B : Test5A
{
    int valueB;
    Test5B(int value) : super(value + 1)
    {
        valueB = value;
    }
    int GetValue() { return valueB; }
    int GetValueB() { return valueB; }
}
"#;
        let ast = parse_source(source).unwrap();
        assert_eq!(ast.members.len(), 2, "应有两个类");

        // 验证 Test5A
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Test5A");
                assert!(c.super_class.is_none());
                assert_eq!(c.members.len(), 4);
                // 构造函数
                let ctors: Vec<_> = c.members.iter()
                    .filter_map(|m| if let ClassMember::Constructor(ctor) = m { Some(ctor) } else { None })
                    .collect();
                assert_eq!(ctors.len(), 1);
                assert!(ctors[0].base_arguments.is_empty());
            }
            _ => panic!("Test5A 应为类"),
        }

        // 验证 Test5B
        match &ast.members[1] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Test5B");
                assert!(c.super_class.is_some(), "应有父类 Test5A");
                match c.super_class.as_ref().unwrap() {
                    TypeRef::Simple { name, .. } => assert_eq!(name, "Test5A"),
                    _ => panic!("父类应为 Simple 类型"),
                }
                assert_eq!(c.members.len(), 4);
                // 验证 super(args) 调用
                let ctors: Vec<_> = c.members.iter()
                    .filter_map(|m| if let ClassMember::Constructor(ctor) = m { Some(ctor) } else { None })
                    .collect();
                assert_eq!(ctors.len(), 1);
                assert_eq!(ctors[0].base_arguments.len(), 1, "应有一个 super 参数");
                // super(value + 1) — 参数是二元加法表达式
                match &ctors[0].base_arguments[0] {
                    Expression::Binary { operator: BinaryOp::Add, .. } => {},
                    _ => panic!("super 参数应为 value + 1 加法表达式"),
                }
            }
            _ => panic!("Test5B 应为类"),
        }
    }

    /// 解析 Test9.g 的注入器构造函数
    #[test]
    fn test_parse_test9g_injector_constructor_with_super() {
        let source = r#"
class Test9Inner
{
    int innerFieldA;
    Test9Inner()
    {
    }
}
class Test9A : Test9N
{
    int gorgeIntField;
    Test9A() : super()
    {
    }
}
"#;
        let ast = parse_source(source).unwrap();
        assert_eq!(ast.members.len(), 2);

        // 验证 Test9A 的 super() 无参数调用
        match &ast.members[1] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Test9A");
                let ctors: Vec<_> = c.members.iter()
                    .filter_map(|m| if let ClassMember::Constructor(ctor) = m { Some(ctor) } else { None })
                    .collect();
                assert_eq!(ctors.len(), 1);
                assert_eq!(ctors[0].body.as_ref().unwrap().len(), 0, "空构造函数体");
                // super() 无参数
                assert!(ctors[0].base_arguments.is_empty(), "super() 无参数");
            }
            _ => panic!("Test9A 应为类"),
        }
    }

    /// 验证 `@Inject` 注解在关键字 `inject` 作为注解名时能被正确解析
    ///
    /// 由于词法器将 `inject` 识别为 `KwInject` 关键字而非 `Identifier`，
    /// `match_identifier_or_keyword()` 需能识别该 token 并返回 "Inject"。
    #[test]
    fn test_parse_annotation_with_keyword() {
        // 使用小写 `inject` 触发 KwInject token（小写才被词法为关键字）
        let source = r#"
class Foo
{
    @inject
    int bar;
}
"#;
        let ast = parse_source(source).unwrap();
        assert_eq!(ast.members.len(), 1);
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Foo");
                assert_eq!(c.members.len(), 1);
                match &c.members[0] {
                    ClassMember::Field(f) => {
                        assert_eq!(f.name, "bar");
                        assert_eq!(f.annotations.len(), 1);
                        assert_eq!(f.annotations[0].name, "Inject");
                    }
                    _ => panic!("bar 应为字段"),
                }
            }
            _ => panic!("Foo 应为类"),
        }
    }

    /// 验证 `@Inject` 注解使用大写形式也可正常解析
    #[test]
    fn test_parse_annotation_with_capitalized_identifier() {
        let source = r#"
class Foo
{
    @Inject
    int bar;
}
"#;
        let ast = parse_source(source).unwrap();
        assert_eq!(ast.members.len(), 1);
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Foo");
                assert_eq!(c.members.len(), 1);
                match &c.members[0] {
                    ClassMember::Field(f) => {
                        assert_eq!(f.name, "bar");
                        assert_eq!(f.annotations.len(), 1);
                        assert_eq!(f.annotations[0].name, "Inject");
                    }
                    _ => panic!("bar 应为字段"),
                }
            }
            _ => panic!("Foo 应为类"),
        }
    }

    /// 验证 `new ^field(args)` 语法正确解析为 `Expression::InjectorNew`
    #[test]
    fn test_parse_new_injector_field_constructor() {
        let source = "new ^myField(1, 2)";
        let result = parse_expr(source).unwrap();
        match result {
            Expression::InjectorNew { injector_field, args, .. } => {
                assert_eq!(injector_field, "myField", "注入器字段名应为 myField");
                assert_eq!(args.len(), 2, "参数个数应为 2");
                assert!(matches!(&args[0], Expression::Literal(Literal::Int(1), _)), "第一个参数为 1");
                assert!(matches!(&args[1], Expression::Literal(Literal::Int(2), _)), "第二个参数为 2");
            }
            _ => panic!("期望 Expression::InjectorNew，实际为 {:?}", result),
        }
    }

    /// 带类型 Lambda：`TypeName:(params) -> body`
    #[test]
    fn test_parse_typed_lambda_expr() {
        let source = "ElementLine:(int x) -> { return x; }";
        let result = parse_expr(source).unwrap();
        match result {
            Expression::Lambda { parameters, body, .. } => {
                assert_eq!(parameters.len(), 1);
                assert_eq!(parameters[0].name, "x");
                match &body {
                    LambdaBody::Block(_) => {}
                    _ => panic!("期望 LambdaBody::Block"),
                }
            }
            _ => panic!("期望 Expression::Lambda，实际为 {:?}", result),
        }
    }

    /// 带类型 Lambda + 注入器后缀：`TypeName^:(params) -> body`
    #[test]
    fn test_parse_typed_lambda_with_caret() {
        let source = "MyType^:(MyType^ arg) -> { return arg; }";
        let result = parse_expr(source).unwrap();
        match result {
            Expression::Lambda { parameters, .. } => {
                assert_eq!(parameters.len(), 1);
                assert_eq!(parameters[0].name, "arg");
            }
            _ => panic!("期望 Expression::Lambda，实际为 {:?}", result),
        }
    }

    /// 带类型 Lambda + 数组后缀：`TypeName[]:(params) -> body`
    #[test]
    fn test_parse_typed_lambda_with_brackets() {
        let source = "MyType[]:(int x) -> { return x; }";
        let result = parse_expr(source).unwrap();
        match result {
            Expression::Lambda { parameters, .. } => {
                assert_eq!(parameters.len(), 1);
                assert_eq!(parameters[0].name, "x");
            }
            _ => panic!("期望 Expression::Lambda，实际为 {:?}", result),
        }
    }

    /// metadata 块中包含带类型 Lambda 值（后跟 @ 注解）
    #[test]
    fn test_parse_metadata_with_typed_lambda_value() {
        let source = r#"
[ delegate<float:DremuLane^> display = string:(DremuLane^ arg) -> { return arg.^name; } ]
@Anno
class Foo { }
"#;
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.annotations.len(), 1);
                let meta = &c.annotations[0].metadatas;
                assert_eq!(meta.len(), 1, "metadata 应有 1 条");
                assert_eq!(meta[0].type_name, "delegate<float:DremuLane^>");
                assert_eq!(meta[0].name, "display");
                assert!(meta[0].value.is_some(), "value 不应为空");
            }
            _ => panic!("期望类"),
        }
    }

    /// metadata 块中包含多个带类型 Lambda 值（后跟 @ 注解）
    #[test]
    fn test_parse_metadata_with_multiple_typed_lambdas() {
        let source = r#"
[
    delegate<float:DremuLane^> display = string:(DremuLane^ arg) -> { return arg.^name; },
    ColorArgb^ color = ColorArgb : {a : 1.0, r : 0.5, g : 0.5, b : 0.5},
    delegate<ElementLine:DremuLane^> elementLine = ElementLine:(DremuLane^ lane) -> { return null; },
    string displayName = "Note"
]
@EditableElement(type = "Note")
class Foo { }
"#;
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.annotations.len(), 1);
                let meta = &c.annotations[0].metadatas;
                assert_eq!(meta.len(), 4, "metadata 应有 4 条");
                assert_eq!(meta[0].name, "display");
                assert_eq!(meta[1].name, "color");
                assert_eq!(meta[2].name, "elementLine");
                assert_eq!(meta[3].name, "displayName");
                assert!(meta[0].value.is_some(), "display value 不应为空");
                assert!(meta[1].value.is_some(), "color value 不应为空");
                assert!(meta[2].value.is_some(), "elementLine value 不应为空");
                assert!(meta[3].value.is_some(), "displayName value 不应为空");
            }
            _ => panic!("期望类"),
        }
    }

    /// DremuNote.g 精简版 — 复杂 metadata 块
    #[test]
    fn test_parse_dremu_note_metadata() {
        let source = r#"
using Gorge;
using GorgeFramework;
namespace Dremu;

[
    delegate<float:DremuLane^> display = string:(DremuLane^ laneInjector) ->
    {
        return laneInjector.^name;
    },
    ColorArgb^ color = ColorArgb : {a : 1.0, r : 0.2396693, g : 0.6370158, b : 0.8},
    delegate<ElementLine:DremuLane^> elementLine = ElementLine:(DremuLane^ lane) ->
    {
        ElementLinePoint[] points = new ElementLinePoint[2];
        return null;
    },
    string displayName = "Note"
]
@EditableElement(type = "Note")
class DremuNote : Note
{
    int x;
}
"#;
        let ast = parse_source(source).unwrap();
        assert!(!ast.members.is_empty(), "应有成员");
        // 在 namespace 内部查找类
        let mut found = false;
        for member in &ast.members {
            if let TopLevelMember::Class(c) = member {
                if c.name == "DremuNote" {
                    assert!(!c.annotations.is_empty(), "应有注解");
                    found = true;
                    break;
                }
            }
        }
        assert!(found, "应找到 DremuNote 类");
    }

    /// `new (^field)[index]` — 注入器字段 new 表达式 + 数组索引
    #[test]
    fn test_parse_new_injector_ref_with_index() {
        let source = "new (^laneLines)[^laneLines.length]";
        let result = parse_expr(source).unwrap();
        match result {
            Expression::ArrayAccess { array, index, .. } => {
                match array.as_ref() {
                    Expression::InjectorNew { injector_field, args, .. } => {
                        assert_eq!(injector_field, "laneLines");
                        assert_eq!(args.len(), 0);
                    }
                    _ => panic!("数组目标应为 InjectorNew，实际为 {:?}", array),
                }
                match index.as_ref() {
                    Expression::MemberAccess { object, member, .. } => {
                        assert_eq!(member, "length");
                        match object.as_ref() {
                            Expression::InjectorFieldRef(name, _) => assert_eq!(name, "laneLines"),
                            _ => panic!("成员访问对象应为 InjectorFieldRef，实际为 {:?}", object),
                        }
                    }
                    _ => panic!("索引应为 MemberAccess，实际为 {:?}", index),
                }
            }
            _ => panic!("期望 ArrayAccess，实际为 {:?}", result),
        }
    }

    /// `new (^field)[index]` 在条件表达式内部
    #[test]
    fn test_parse_conditional_with_new_injector_index() {
        let source = "(^laneLines == null) ? null : (new (^laneLines)[^laneLines.length])";
        let result = parse_expr(source).unwrap();
        match result {
            Expression::Conditional { condition, then_branch, else_branch, .. } => {
                match condition.as_ref() {
                    Expression::Binary { operator: BinaryOp::Equal, .. } => {}
                    _ => panic!("条件应为 == 比较"),
                }
                match then_branch.as_ref() {
                    Expression::Null(_) => {}
                    _ => panic!("真分支应为 null"),
                }
                match else_branch.as_ref() {
                    Some(expr) => match expr.as_ref() {
                        Expression::ArrayAccess { .. } => {}
                        _ => panic!("假分支应为 ArrayAccess，实际为 {:?}", expr),
                    },
                    None => panic!("假分支不应为空"),
                }
            }
            _ => panic!("期望 Conditional，实际为 {:?}", result),
        }
    }

    /// 字段声明：`Type field = (^f == null) ? null : (new (^f)[^f.len]);`
    #[test]
    fn test_parse_field_with_conditional_new_injector_index() {
        let source = r#"
class Foo
{
    FunctionCurve^[] laneLines = (^laneLines == null) ? null : (new (^laneLines)[^laneLines.length]);
}
"#;
        let ast = parse_source(source).unwrap();
        match &ast.members[0] {
            TopLevelMember::Class(c) => {
                assert_eq!(c.name, "Foo");
                assert_eq!(c.members.len(), 1, "应有 1 个类成员");
            }
            _ => panic!("期望类"),
        }
    }

    // ==================== 强制转换结合性测试 ====================

    /// `(T) a.b()` 应解析为 `(T)(a.b())`：方法调用属于 cast 的操作数
    #[test]
    fn test_cast_binds_postfix_method_call() {
        let expr = parse_expr("(Asset) Env.GetIt(\"x\")").unwrap();
        match expr {
            Expression::Cast { target_type, expression, .. } => {
                match target_type {
                    TypeRef::Simple { name, .. } => assert_eq!(name, "Asset"),
                    other => panic!("目标类型应为 Simple(Asset)，实际为 {:?}", other),
                }
                match expression.as_ref() {
                    Expression::MethodCall { receiver, method, .. } => {
                        assert_eq!(method, "GetIt");
                        match receiver.as_ref() {
                            Expression::Identifier(name, _) => assert_eq!(name, "Env"),
                            other => panic!("接收者应为 Identifier(Env)，实际为 {:?}", other),
                        }
                    }
                    other => panic!("cast 操作数应为 MethodCall，实际为 {:?}", other),
                }
            }
            other => panic!("应为 Cast，实际为 {:?}", other),
        }
    }

    /// `(T) a.b.c` 应解析为 `(T)(a.b.c)`：成员链属于 cast 的操作数
    #[test]
    fn test_cast_binds_member_chain() {
        let expr = parse_expr("(Node) lane.noteReferenceNode").unwrap();
        match expr {
            Expression::Cast { expression, .. } => match expression.as_ref() {
                Expression::MemberAccess { object, member, .. } => {
                    assert_eq!(member, "noteReferenceNode");
                    match object.as_ref() {
                        Expression::Identifier(name, _) => assert_eq!(name, "lane"),
                        other => panic!("应为 Identifier(lane)，实际为 {:?}", other),
                    }
                }
                other => panic!("cast 操作数应为 MemberAccess，实际为 {:?}", other),
            },
            other => panic!("应为 Cast，实际为 {:?}", other),
        }
    }

    /// `(T) a[i]` 应解析为 `(T)(a[i])`：数组访问属于 cast 的操作数
    #[test]
    fn test_cast_binds_array_access() {
        let expr = parse_expr("(Node) arr[0]").unwrap();
        match expr {
            Expression::Cast { expression, .. } => match expression.as_ref() {
                Expression::ArrayAccess { .. } => {}
                other => panic!("cast 操作数应为 ArrayAccess，实际为 {:?}", other),
            },
            other => panic!("应为 Cast，实际为 {:?}", other),
        }
    }

    /// `(int) x + 1` 应解析为 `((int)x) + 1`：二元运算不进入 cast 操作数
    #[test]
    fn test_cast_does_not_bind_binary() {
        let expr = parse_expr("(int) x + 1").unwrap();
        match expr {
            Expression::Binary { left, operator, .. } => {
                assert!(matches!(operator, BinaryOp::Add));
                match left.as_ref() {
                    Expression::Cast { .. } => {}
                    other => panic!("左操作数应为 Cast，实际为 {:?}", other),
                }
            }
            other => panic!("应为 Binary，实际为 {:?}", other),
        }
    }

    /// `(x) + y` 不是 cast（`)` 后为二元操作符），应保持括号表达式语义
    #[test]
    fn test_paren_expr_not_cast_before_binary_operator() {
        let expr = parse_expr("(x) + y").unwrap();
        match expr {
            Expression::Binary { left, operator, .. } => {
                assert!(matches!(operator, BinaryOp::Add));
                match left.as_ref() {
                    Expression::Identifier(name, _) => assert_eq!(name, "x"),
                    other => panic!("左操作数应为 Identifier(x)，实际为 {:?}", other),
                }
            }
            other => panic!("应为 Binary，实际为 {:?}", other),
        }
    }

    /// 嵌套 cast：`(A) (B) x` 内层仍是 cast
    #[test]
    fn test_nested_cast() {
        let expr = parse_expr("(A) (B) x").unwrap();
        match expr {
            Expression::Cast { target_type, expression, .. } => {
                match target_type {
                    TypeRef::Simple { name, .. } => assert_eq!(name, "A"),
                    other => panic!("外层目标应为 A，实际为 {:?}", other),
                }
                match expression.as_ref() {
                    Expression::Cast { .. } => {}
                    other => panic!("内层应为 Cast，实际为 {:?}", other),
                }
            }
            other => panic!("应为 Cast，实际为 {:?}", other),
        }
    }

}