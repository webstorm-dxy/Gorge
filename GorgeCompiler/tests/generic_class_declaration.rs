//! 泛型类声明的端到端回归测试。

use gorge_compiler::compile_sources;
use gorge_compiler::frontend::lexer;
use gorge_compiler::frontend::ast::TopLevelMember;
use gorge_compiler::frontend::parser::Parser;

#[test]
fn generic_class_declaration_resolves_type_parameters() {
    let (tokens, diags) = lexer::tokenize(r#"
namespace Gorge;

native class ObjectArray<TItem>
{
    @Inject
    int length;

    TItem first;
    TItem[] items;

    ObjectArray();

    TItem Get(int index);
    void Set(int index, TItem value);
    void Replace(TItem[] values);
}
"#, 0);
    assert!(diags.is_empty(), "词法错误: {:?}", diags);
    let src = Parser::new(tokens).parse_source_file().expect("语法错误");
    let TopLevelMember::Class(class_decl) = &src.members[0] else {
        panic!("应解析出类声明");
    };
    assert_eq!(class_decl.name, "ObjectArray");
    assert_eq!(class_decl.generic_params, ["TItem"]);

    compile_sources(&[src], false).expect("泛型类成员中的 TItem 和 TItem[] 应能解析并编译");
}
