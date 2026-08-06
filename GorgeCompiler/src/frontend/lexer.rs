#![allow(dead_code)]

use logos::Logos;

use gorge_core::diagnostics::Span;

/// 词法单元（Token）枚举，表示 Gorge 语言源代码的最小有意义的词法元素。
///
/// 使用 `logos` 库自动生成词法分析器。跳过的内容：
/// - 空白字符（空格、制表符、换行、回车）
/// - 单行注释 `// ...`
/// - 块注释 `/* ... */`
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\r]+")]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")]
pub enum Token {
    // ======================== 关键字 ========================

    /// 类声明关键字 `class`
    #[token("class")]
    KwClass,

    /// 接口声明关键字 `interface`
    #[token("interface")]
    KwInterface,

    /// 枚举声明关键字 `enum`
    #[token("enum")]
    KwEnum,

    /// 继承关键字 `extends`
    #[token("extends")]
    KwExtends,

    /// 原生方法标记 `native`，用于声明由宿主环境实现的方法
    #[token("native")]
    KwNative,

    /// 静态成员标记 `static`
    #[token("static")]
    KwStatic,

    /// 当前实例引用 `this`
    #[token("this")]
    KwThis,

    /// 父类引用 `super`
    #[token("super")]
    KwSuper,

    /// 对象创建关键字 `new`
    #[token("new")]
    KwNew,

    /// 依赖注入标记 `injector`
    #[token("injector")]
    KwInjector,

    /// 注入声明关键字 `inject`
    #[token("inject")]
    KwInject,

    /// 委托调用关键字 `invokes`
    #[token("invokes")]
    KwInvokes,

    /// 条件分支关键字 `if`
    #[token("if")]
    KwIf,

    /// 条件分支关键字 `else`
    #[token("else")]
    KwElse,

    /// 循环关键字 `while`
    #[token("while")]
    KwWhile,

    /// do-while 循环关键字 `do`
    #[token("do")]
    KwDo,

    /// 计数循环关键字 `for`
    #[token("for")]
    KwFor,

    /// 分支选择关键字 `switch`
    #[token("switch")]
    KwSwitch,

    /// 分支标签关键字 `case`
    #[token("case")]
    KwCase,

    /// 默认分支关键字 `default`
    #[token("default")]
    KwDefault,

    /// 循环退出关键字 `break`
    #[token("break")]
    KwBreak,

    /// 循环继续关键字 `continue`
    #[token("continue")]
    KwContinue,

    /// 函数返回关键字 `return`
    #[token("return")]
    KwReturn,

    /// 模块导入关键字 `using`
    #[token("using")]
    KwUsing,

    /// 命名空间关键字 `namespace`
    #[token("namespace")]
    KwNamespace,

    /// 委托类型关键字 `delegate`
    #[token("delegate")]
    KwDelegate,

    /// 布尔字面量 `true`
    #[token("true")]
    KwTrue,

    /// 布尔字面量 `false`
    #[token("false")]
    KwFalse,

    /// 空值字面量 `null`
    #[token("null")]
    KwNull,

    /// 自动类型推导关键字 `auto`
    #[token("auto")]
    KwAuto,

    // ======================== 类型关键字 ========================

    /// 整型关键字 `int`
    #[token("int")]
    TypeInt,

    /// 浮点型关键字 `float`
    #[token("float")]
    TypeFloat,

    /// 布尔型关键字 `bool`
    #[token("bool")]
    TypeBool,

    /// 字符串型关键字 `string`
    #[token("string")]
    TypeString,

    /// 空类型关键字 `void`，表示函数无返回值
    #[token("void")]
    TypeVoid,

    /// 对象类型关键字 `object`
    #[token("object")]
    TypeObject,

    // ======================== 字面量 ========================

    /// 整数字面量（如 `42`），内部存储解析后的 `i64` 值。
    /// 只接受 `0` 或非零开头的数字串，禁止前导零（如 `012`）
    #[regex(r"0|[1-9][0-9]*", |lex| lex.slice().parse().ok())]
    IntLiteral(i64),

    /// 浮点数字面量（如 `3.14`），内部存储解析后的 `f64` 值
    #[regex(r"[1-9][0-9]*\.[0-9]+|0\.[0-9]+", |lex| lex.slice().parse().ok())]
    FloatLiteral(f64),

    /// 字符串字面量（如 `"hello"`），内部存储经过转义处理后的 `String` 值。
    /// 支持转义序列：`\\`, `\"`, `\n`, `\r`, `\t`
    #[regex(r#""([^"\\]|\\[\\"nrt])*""#, parse_string)]
    StringLiteral(String),

    // ======================== 标识符 ========================

    /// 标识符（如变量名、函数名），以字母或下划线开头，后接字母、数字或下划线
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // ======================== 操作符 ========================

    /// 加法运算符 `+`
    #[token("+")]
    Plus,

    /// 减法运算符 `-`
    #[token("-")]
    Minus,

    /// 乘法运算符 `*`
    #[token("*")]
    Star,

    /// 除法运算符 `/`
    #[token("/")]
    Slash,

    /// 取模运算符 `%`
    #[token("%")]
    Percent,

    /// 赋值运算符 `=`
    #[token("=")]
    Assign,

    /// 相等比较运算符 `==`
    #[token("==")]
    EqualEqual,

    /// 不等比较运算符 `!=`
    #[token("!=")]
    NotEqual,

    /// 小于比较运算符 `<`
    #[token("<")]
    Less,

    /// 小于等于比较运算符 `<=`
    #[token("<=")]
    LessEqual,

    /// 大于比较运算符 `>`
    #[token(">")]
    Greater,

    /// 大于等于比较运算符 `>=`
    #[token(">=")]
    GreaterEqual,

    /// 逻辑与运算符 `&&`
    #[token("&&")]
    AndAnd,

    /// 逻辑或运算符 `||`
    #[token("||")]
    OrOr,

    /// 逻辑非运算符 `!`
    #[token("!")]
    Bang,

    /// 注入器字段访问符 `^`
    #[token("^")]
    Caret,

    /// Lambda 箭头 `->`，用于 Lambda 表达式
    #[token("->")]
    LambdaArrow,

    // ======================== 标点符号 ========================

    /// 成员访问符 `.`
    #[token(".")]
    Dot,

    /// 分隔符 `,`
    #[token(",")]
    Comma,

    /// 冒号 `:`，用于类型标注等
    #[token(":")]
    Colon,

    /// 双冒号 `::`，用于接口继承分隔
    #[token("::")]
    DoubleColon,

    /// 语句结束符 `;`
    #[token(";")]
    Semicolon,

    /// 左花括号 `{`
    #[token("{")]
    LBrace,

    /// 右花括号 `}`
    #[token("}")]
    RBrace,

    /// 左圆括号 `(`
    #[token("(")]
    LParen,

    /// 右圆括号 `)`
    #[token(")")]
    RParen,

    /// 左方括号 `[`
    #[token("[")]
    LBracket,

    /// 右方括号 `]`
    #[token("]")]
    RBracket,

    /// 问号 `?`（用于可空类型等）
    #[token("?")]
    Question,

    /// 注解标识 `@`，用于注解声明
    #[token("@")]
    At,

    /// 箭头 `=>`（保留备用）
    #[token("=>")]
    Arrow,

    // ======================== 复合赋值运算符 ========================

    /// 自增运算符 `++`
    #[token("++")]
    PlusPlus,

    /// 自减运算符 `--`
    #[token("--")]
    MinusMinus,

    /// 加法复合赋值 `+=`
    #[token("+=")]
    PlusAssign,

    /// 减法复合赋值 `-=`
    #[token("-=")]
    MinusAssign,

    /// 乘法复合赋值 `*=`
    #[token("*=")]
    StarAssign,

    /// 除法复合赋值 `/=`
    #[token("/=")]
    SlashAssign,

    /// 取模复合赋值 `%=`
    #[token("%=")]
    PercentAssign,
}

/// 解析字符串字面量内容，处理转义序列。
///
/// 输入为双引号包裹的原始字符串（含外层引号），输出为去掉引号并处理转义后的字符串。
/// 支持的转义序列：`\\` → `\`, `\"` → `"`, `\n` → 换行, `\r` → 回车, `\t` → 制表符。
/// 未识别的转义序列保留原始字符（如 `\x` → `\x`）。
///
/// 此函数由 `logos` 的 `#[regex]` 属性回调调用，必须返回 `Option<String>`。
fn parse_string(lex: &mut logos::Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    // 去掉首尾的双引号
    let inner = &slice[1..slice.len() - 1];
    // 预分配容量，转义后长度通常不超过原始内层长度
    let mut result = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some(c) => {
                    // 未识别的转义序列，保持原样
                    result.push('\\');
                    result.push(c);
                }
                None => break,
            }
        } else {
            result.push(c);
        }
    }
    Some(result)
}

/// 带有源代码位置信息的词法单元。
///
/// 将 `Token` 与其在源代码中的位置（`Span`）绑定在一起，
/// 便于后续语法分析和错误报告。
#[derive(Debug, Clone)]
pub struct TokenSpan {
    /// 词法单元本身
    pub token: Token,
    /// 词法单元在源代码中的位置信息
    pub span: Span,
}

impl TokenSpan {
    /// 创建一个新的 `TokenSpan`。
    pub fn new(token: Token, span: Span) -> Self {
        Self { token, span }
    }
}

/// 对源代码进行词法分析，生成词法单元序列。
///
/// # 参数
/// - `source`: 源代码字符串
/// - `source_id`: 源文件编号，用于跨文件的错误定位
///
/// # 返回值
/// 返回一个元组：
/// - `Vec<TokenSpan>`: 成功识别的词法单元序列，每个都带有位置信息
/// - `Vec<Diagnostic>`: 词法分析过程中遇到的错误（如无法识别的字符）
///
/// # 注意事项
/// - 空白字符和注释在词法分析阶段被自动跳过，不会产生 Token
/// - 遇到无法识别的字符时不会中断，而是记录错误后继续分析后续内容
pub fn tokenize(source: &str, source_id: usize) -> (Vec<TokenSpan>, Vec<gorge_core::diagnostics::Diagnostic>) {
    let lexer = Token::lexer(source);
    let mut tokens = Vec::new();
    let mut diagnostics = Vec::new();

    // 行列游标（对齐 C# 参考实现 ANTLR 运行时的增量维护方式）：
    // logos 产出的 range 单调递增，只需逐段推进游标即可得到每个 token
    // 的行列号，避免为每个 token 从文件头重新扫描（O(n²) → O(n)）。
    // 被跳过的空白/注释位于上一 token 末尾与当前 token 起点之间，
    // 扫描 last_offset..start 段时一并计入，语义与全量扫描完全一致。
    let mut last_offset = 0usize;
    let mut line = 1usize;
    let mut column = 1usize;

    for (result, range) in lexer.spanned() {
        let start = range.start;
        let end = range.end;

        // 推进游标至 token 起点（含中间被跳过的空白/注释），得到该 token 的行列号
        for c in source[last_offset..start].chars() {
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        let (token_line, token_column) = (line, column);

        // 推进游标越过 token 自身，多行 token 内部的换行计入后续 token
        for c in source[start..end].chars() {
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        last_offset = end;

        let span = Span::new(start, end, token_line, token_column, source_id);

        match result {
            Ok(token) => {
                tokens.push(TokenSpan { token, span });
            }
            Err(()) => {
                // 词法错误：无法识别的字符序列，截取最多一个字符用于报错
                diagnostics.push(
                    gorge_core::diagnostics::Diagnostic::error(span, format!(
                        "unrecognized character: '{}'",
                        &source[start..end.min(source.len())]
                    ))
                );
            }
        }
    }

    (tokens, diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let (tokens, diags) = tokenize("class interface enum native static", 0);
        assert!(diags.is_empty());
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0].token, Token::KwClass));
        assert!(matches!(tokens[1].token, Token::KwInterface));
        assert!(matches!(tokens[2].token, Token::KwEnum));
        assert!(matches!(tokens[3].token, Token::KwNative));
        assert!(matches!(tokens[4].token, Token::KwStatic));
    }

    #[test]
    fn test_new_keywords() {
        let (tokens, diags) = tokenize("super inject invokes do auto", 0);
        assert!(diags.is_empty());
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0].token, Token::KwSuper));
        assert!(matches!(tokens[1].token, Token::KwInject));
        assert!(matches!(tokens[2].token, Token::KwInvokes));
        assert!(matches!(tokens[3].token, Token::KwDo));
        assert!(matches!(tokens[4].token, Token::KwAuto));
    }

    #[test]
    fn test_new_symbols() {
        let (tokens, diags) = tokenize("@ ^ -> ::", 0);
        assert!(diags.is_empty());
        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0].token, Token::At));
        assert!(matches!(tokens[1].token, Token::Caret));
        assert!(matches!(tokens[2].token, Token::LambdaArrow));
        assert!(matches!(tokens[3].token, Token::DoubleColon));
    }

    #[test]
    fn test_integer_literal() {
        let (tokens, diags) = tokenize("42 0 123456", 0);
        assert!(diags.is_empty());
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token, Token::IntLiteral(42));
        assert_eq!(tokens[1].token, Token::IntLiteral(0));
        assert_eq!(tokens[2].token, Token::IntLiteral(123456));
    }

    #[test]
    fn test_integer_leading_zero_split() {
        // "012" 不能作为一个整数 token 被识别，应拆分为 `0` 和 `12`
        let (tokens, diags) = tokenize("012", 0);
        assert!(diags.is_empty());
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::IntLiteral(0));
        assert_eq!(tokens[1].token, Token::IntLiteral(12));
    }

    #[test]
    fn test_float_literal() {
        let (tokens, diags) = tokenize("3.14 0.5", 0);
        assert!(diags.is_empty());
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0].token, Token::FloatLiteral(f) if (f - 3.14).abs() < 0.001));
        assert!(matches!(tokens[1].token, Token::FloatLiteral(f) if (f - 0.5).abs() < 0.001));
    }

    #[test]
    fn test_string_literal() {
        let (tokens, diags) = tokenize(r#""hello" "world\n""#, 0);
        assert!(diags.is_empty());
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::StringLiteral("hello".to_string()));
        assert_eq!(tokens[1].token, Token::StringLiteral("world\n".to_string()));
    }

    #[test]
    fn test_identifiers() {
        let (tokens, diags) = tokenize("myVar x123 _private", 0);
        assert!(diags.is_empty());
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token, Token::Identifier("myVar".to_string()));
        assert_eq!(tokens[1].token, Token::Identifier("x123".to_string()));
        assert_eq!(tokens[2].token, Token::Identifier("_private".to_string()));
    }

    #[test]
    fn test_operators() {
        let (tokens, diags) = tokenize("+ - * / % = == != < <= > >= && || !", 0);
        assert!(diags.is_empty());
        assert_eq!(tokens.len(), 15);
        assert!(matches!(tokens[0].token, Token::Plus));
        assert!(matches!(tokens[4].token, Token::Percent));
        assert!(matches!(tokens[5].token, Token::Assign));
        assert!(matches!(tokens[8].token, Token::Less));
        assert!(matches!(tokens[9].token, Token::LessEqual));
        assert!(matches!(tokens[13].token, Token::OrOr));
    }

    #[test]
    fn test_punctuation() {
        let (tokens, diags) = tokenize(". , : ; { } ( ) [ ] ? =>", 0);
        assert!(diags.is_empty());
        assert_eq!(tokens.len(), 12);
        assert!(matches!(tokens[0].token, Token::Dot));
        assert!(matches!(tokens[11].token, Token::Arrow));
    }

    #[test]
    fn test_comments_are_skipped() {
        let (tokens, diags) = tokenize("x // line comment\ny /* block comment */ z", 0);
        assert!(diags.is_empty());
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token, Token::Identifier("x".to_string()));
        assert_eq!(tokens[1].token, Token::Identifier("y".to_string()));
        assert_eq!(tokens[2].token, Token::Identifier("z".to_string()));
    }

    #[test]
    fn test_class_declaration() {
        let source = r#"
class MyClass extends BaseClass :: IFoo, IBar {
    int x = 0;
    string name = "hello";
}
"#;
        let (tokens, diags) = tokenize(source, 0);
        assert!(diags.is_empty());

        let token_kinds: Vec<_> = tokens.iter().map(|t| &t.token).collect();

        assert!(token_kinds.contains(&&Token::KwClass));
        assert!(token_kinds.contains(&&Token::Identifier("MyClass".into())));
        assert!(token_kinds.contains(&&Token::KwExtends));
        assert!(token_kinds.contains(&&Token::Identifier("BaseClass".into())));
        assert!(token_kinds.contains(&&Token::DoubleColon));
        assert!(token_kinds.contains(&&Token::Identifier("IFoo".into())));
        assert!(token_kinds.contains(&&Token::Identifier("IBar".into())));
    }

    #[test]
    fn test_line_column_tracking() {
        let source = "x\ny\nz";
        let (tokens, _) = tokenize(source, 0);

        assert_eq!(tokens[0].span.line, 1);
        assert_eq!(tokens[0].span.column, 1);

        assert_eq!(tokens[1].span.line, 2);
        assert_eq!(tokens[1].span.column, 1);

        assert_eq!(tokens[2].span.line, 3);
        assert_eq!(tokens[2].span.column, 1);
    }

    /// 旧版朴素实现：从文件头逐字符扫描计算行列号。
    ///
    /// 已由增量游标替代，此处保留作为回归对照，验证新实现行列号语义完全一致。
    fn naive_offset_to_line_column(source: &str, offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        for (i, c) in source.char_indices() {
            if i >= offset {
                break;
            }
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        (line, column)
    }

    /// 增量游标与朴素实现的等价性：任意 token（含词法错误）的行列号一致
    #[test]
    fn test_incremental_position_matches_naive() {
        // 覆盖多字节字符、跨行块注释、字符串字面量、空白与词法错误字符
        let source = "abc\n  中文\n/* 多行\n注释 */ x = \"str\";\n+@?";
        let (tokens, diagnostics) = tokenize(source, 0);
        assert!(!diagnostics.is_empty(), "应存在无法识别的字符以覆盖错误分支");

        // 每个产出项（token 与错误）的 span 都必须与朴素实现一致
        for t in &tokens {
            let (line, column) = naive_offset_to_line_column(source, t.span.start);
            assert_eq!(
                (t.span.line, t.span.column),
                (line, column),
                "token start={} 行列不一致",
                t.span.start
            );
        }
        for d in &diagnostics {
            let (line, column) = naive_offset_to_line_column(source, d.span.start);
            assert_eq!(
                (d.span.line, d.span.column),
                (line, column),
                "错误 span start={} 行列不一致",
                d.span.start
            );
        }
    }

    /// 多字节字符与跨行块注释后的行列定位
    #[test]
    fn test_position_with_multibyte_and_multiline_comment() {
        // "ab\n 中z\n/* 注释\n跨行 */ q"
        // 行2 = " 中z"：空格=1, 中=2（3 字节按 1 列计）, z=3
        // 行4 = "跨行 */ q"：跨=1, 行=2, 空格=3, *=4, /=5, 空格=6, q=7
        let source = "ab\n 中z\n/* 注释\n跨行 */ q";
        let (tokens, _) = tokenize(source, 0);

        // z 位于第 2 行第 3 列（多字节字符按 1 列计）
        assert_eq!(tokens[1].span.line, 2);
        assert_eq!(tokens[1].span.column, 3);
        // 跨行块注释后 q 位于第 4 行第 7 列（注释内换行已计入）
        let last = tokens.last().unwrap();
        assert_eq!(last.span.line, 4);
        assert_eq!(last.span.column, 7);
    }
}
