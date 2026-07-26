use std::env;
use std::fs;
use std::path::Path;

use gorge_compiler::compile_sources;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("用法: gorgec <输入文件.g> [-o <输出文件.gorge>] [--progress]");
        eprintln!("示例: gorgec program.g -o program.gorge");
        eprintln!("选项: --progress  显示编译进度百分比");
        std::process::exit(1);
    }

    // 解析 --progress 标志（位置无关）
    let show_progress = args.iter().any(|a| a == "--progress");
    let filtered_args: Vec<&String> = args.iter().skip(1).filter(|a| *a != "--progress").collect();

    if filtered_args.is_empty() {
        eprintln!("错误：需要输入文件");
        std::process::exit(1);
    }

    let input_path = filtered_args[0];
    let output_path = if filtered_args.len() >= 3 && filtered_args[1] == "-o" {
        filtered_args[2].clone()
    } else {
        // 默认输出：替换 .g 为 .gorge
        Path::new(input_path)
            .with_extension("gorge")
            .to_string_lossy()
            .into_owned()
    };

    let source = match fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("错误：无法读取文件 `{}`: {}", input_path, e);
            std::process::exit(1);
        }
    };

    // 词法分析
    let (tokens, lexer_diags) = gorge_compiler::frontend::lexer::tokenize(&source, 0);
    if !lexer_diags.is_empty() {
        eprintln!("词法错误:");
        let mut d = gorge_core::diagnostics::Diagnostics::new();
        for diag in lexer_diags {
            d.emit(diag);
        }
        let sources: Vec<&str> = vec![&source];
        eprintln!("{}", d.render(&sources));
        std::process::exit(1);
    }

    // 语法分析
    let mut parser = gorge_compiler::frontend::parser::Parser::new(tokens);
    let source_file = match parser.parse_source_file() {
        Ok(ast) => ast,
        Err(diags) => {
            eprintln!("语法错误:");
            let sources: Vec<&str> = vec![&source];
            eprintln!("{}", diags.render(&sources));
            std::process::exit(1);
        }
    };

    // 编译
    let module = match compile_sources(&[source_file], show_progress) {
        Ok(m) => m,
        Err(diagnostics) => {
            eprintln!("编译错误:");
            let sources: Vec<&str> = vec![&source];
            eprintln!("{}", diagnostics.render(&sources));
            std::process::exit(1);
        }
    };

    let bytecode = match gorge_core::objective::bytecode::serialize_module(&module) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("序列化错误: {}", e);
            std::process::exit(1);
        }
    };

    match fs::write(&output_path, bytecode) {
        Ok(_) => println!("编译成功: {} -> {}", input_path, output_path),
        Err(e) => {
            eprintln!("错误：无法写入文件 `{}`: {}", output_path, e);
            std::process::exit(1);
        }
    }
}
