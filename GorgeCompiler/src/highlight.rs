#![allow(dead_code)]

use gorge_core::diagnostics::Span;
use crate::lexer::Token;

/// 高亮种类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    Keyword,
    Type,
    String,
    Number,
    Comment,
    Identifier,
    Operator,
    Annotation,
}

/// 高亮区间
#[derive(Debug, Clone)]
pub struct HighlightSpan {
    pub span: Span,
    pub kind: HighlightKind,
}

/// 对源代码执行语法高亮分析
///
/// 通过复用词法分析的 token 流，将每个 token 映射为对应的高亮类别。
pub fn highlight(source: &str) -> Vec<HighlightSpan> {
    use crate::lexer::tokenize;
    let (tokens, _) = tokenize(source, 0);
    let mut spans = Vec::new();

    for ts in &tokens {
        let kind = token_highlight_kind(&ts.token);
        spans.push(HighlightSpan { span: ts.span, kind });
    }

    spans
}

/// 将 Token 映射为高亮类别
fn token_highlight_kind(token: &Token) -> HighlightKind {
    match token {
        // 关键字
        Token::KwClass | Token::KwInterface | Token::KwEnum | Token::KwExtends
        | Token::KwNative | Token::KwStatic | Token::KwThis | Token::KwSuper
        | Token::KwNew | Token::KwInjector | Token::KwIf | Token::KwElse
        | Token::KwWhile | Token::KwFor | Token::KwDo | Token::KwSwitch
        | Token::KwCase | Token::KwDefault | Token::KwBreak | Token::KwContinue
        | Token::KwReturn | Token::KwUsing | Token::KwNamespace | Token::KwDelegate
        | Token::KwInject | Token::KwInvokes | Token::KwAuto | Token::KwNull => HighlightKind::Keyword,

        // 类型关键字
        Token::TypeInt | Token::TypeFloat | Token::TypeBool
        | Token::TypeString | Token::TypeVoid | Token::TypeObject => HighlightKind::Type,

        // 字面量
        Token::StringLiteral(_) => HighlightKind::String,
        Token::IntLiteral(_) | Token::FloatLiteral(_) => HighlightKind::Number,
        Token::KwTrue | Token::KwFalse => HighlightKind::Number,

        // 注解
        Token::At => HighlightKind::Annotation,

        // 操作符
        Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Percent
        | Token::Assign | Token::EqualEqual | Token::NotEqual | Token::Less
        | Token::LessEqual | Token::Greater | Token::GreaterEqual | Token::AndAnd
        | Token::OrOr | Token::Bang | Token::PlusPlus | Token::MinusMinus
        | Token::PlusAssign | Token::MinusAssign | Token::StarAssign | Token::SlashAssign
        | Token::PercentAssign | Token::Question | Token::Arrow | Token::LambdaArrow
        | Token::DoubleColon | Token::Caret | Token::Dot => HighlightKind::Operator,

        // 界符
        Token::Comma | Token::Colon | Token::Semicolon | Token::LParen | Token::RParen
        | Token::LBrace | Token::RBrace | Token::LBracket | Token::RBracket => HighlightKind::Operator,

        // 标识符
        Token::Identifier(_) => HighlightKind::Identifier,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_keywords() {
        let result = highlight("class int return");
        assert_eq!(result.len(), 3);
        assert!(matches!(result[0].kind, HighlightKind::Keyword));
        assert!(matches!(result[1].kind, HighlightKind::Type));
        assert!(matches!(result[2].kind, HighlightKind::Keyword));
    }

    #[test]
    fn test_highlight_string() {
        let result = highlight(r#""hello""#);
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0].kind, HighlightKind::String));
    }

    #[test]
    fn test_highlight_operators() {
        let result = highlight("+ - * /");
        assert!(result.iter().all(|h| matches!(h.kind, HighlightKind::Operator)));
    }
}
