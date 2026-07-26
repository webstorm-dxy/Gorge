//! 成员链、数组元素与方法调用的类型传播回归测试。

use gorge_compiler::compiler::Compiler;
use gorge_compiler::frontend::lexer;
use gorge_compiler::frontend::parser::Parser;
use gorge_core::virtual_machine::ir::IntermediateOperator;

fn parse_source(source: &str) -> gorge_compiler::frontend::ast::SourceFile {
    let (tokens, diagnostics) = lexer::tokenize(source, 0);
    assert!(diagnostics.is_empty(), "词法错误: {diagnostics:?}");
    Parser::new(tokens).parse_source_file().expect("语法错误")
}

#[test]
fn object_array_access_and_member_chain_keep_element_type() {
    let source = parse_source(r#"
class Automaton {
    string GetState() { return "Accepted"; }
}

class Note {
    float hitTime;
}

class Holder {
    Automaton automaton;
    Note[] notes;

    bool Evaluate() {
        string state = automaton.GetState();
        if (state == "Accepted" && notes[0] == null) {
            return true;
        }
        return notes[0].hitTime > 0.0;
    }
}
"#);

    let mut compiler = Compiler::new();
    compiler.compile(&[source]).expect("类型传播正确时应编译成功");

    let operators: Vec<_> = compiler.compiled_methods.iter()
        .flat_map(|method| method.codes.iter())
        .map(|code| code.code.operator.clone())
        .collect();
    assert!(operators.iter().any(|operator| matches!(operator, IntermediateOperator::StringEqual)));
    assert!(operators.iter().any(|operator| matches!(operator, IntermediateOperator::ObjectEqual)));
    assert!(operators.iter().any(|operator| matches!(operator, IntermediateOperator::LoadFloatField(_))));
}

#[test]
fn primitive_and_null_equality_remains_invalid() {
    let source = parse_source(r#"
class InvalidEquality {
    bool Evaluate() {
        int value = 1;
        return value == null;
    }
}
"#);

    let mut compiler = Compiler::new();
    assert!(compiler.compile(&[source]).is_err(), "int 与 null 的比较必须拒绝");
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.message.contains("相等运算的两操作数类型不同")
    }));
}
