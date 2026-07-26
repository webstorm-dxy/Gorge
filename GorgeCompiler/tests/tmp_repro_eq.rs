// 临时复现测试：相等运算剩余模式（诊断后会删除）
use gorge_compiler::compile_sources;
use gorge_compiler::frontend::lexer;
use gorge_compiler::frontend::parser::Parser;

fn parse(code: &str, source_id: usize) -> gorge_compiler::frontend::ast::SourceFile {
    let (tokens, diags) = lexer::tokenize(code, source_id);
    assert!(diags.is_empty(), "词法错误: {:?}", diags);
    Parser::new(tokens).parse_source_file().expect("语法错误")
}

#[test]
fn repro_b() {
    // B. 方法返回值赋给 string 局部变量后与字符串比较（跨文件命名空间，贴近真实 Dremu）
    let gorge = parse(r#"
namespace Gorge;
native class Automaton
{
    string GetState();
}
"#, 0);
    let dremu = parse(r#"
namespace Dremu;
using Gorge;
class Note
{
    Automaton^ automaton;
}
class Transformer
{
    Note note;

    bool Check()
    {
        string automatonState = note.automaton.GetState();
        if (automatonState == "Accepted" || automatonState == "Denied")
        {
            return true;
        }
        return false;
    }
}
"#, 1);
    match compile_sources(&[gorge, dremu], false) {
        Ok(_) => println!("B_string_eq: 编译成功"),
        Err(d) => println!("B_string_eq DIAG: {:?}", d),
    }
}
