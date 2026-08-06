//! 临时复现：Song.g 解析挂起定位（完成后删除）
//! 测试不同栈大小下解析 Song.g 是否栈溢出/挂起。

use gorge_compiler::frontend::lexer;
use gorge_compiler::frontend::parser::Parser;

#[test]
fn repro_song_parse() {
    let real = std::env::var("SONG_PATH").unwrap_or_else(|_| {
        "C:\\Users\\daxingyi\\AppData\\Local\\Temp\\opencode\\song_real.g".to_string()
    });
    let song = std::fs::read_to_string(&real).expect("read song");
    let cut: usize = std::env::var("CUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(song.len());
    let song = song.chars().take(cut).collect::<String>();
    eprintln!("cut={} chars", song.chars().count());
    let stack: usize = std::env::var("STACK_KB")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8 * 1024);
    let handle = std::thread::Builder::new()
        .stack_size(stack * 1024)
        .spawn(move || {
            eprintln!("stack_kb={}", stack);
            let (tokens, diags) = lexer::tokenize(&song, 555);
            std::fs::write("C:\\Users\\daxingyi\\AppData\\Local\\Temp\\opencode\\m_tokenize.txt", "tok").unwrap();
            eprintln!("tokenize done, {} tokens, {} diags", tokens.len(), diags.len());
            assert!(diags.is_empty(), "lexer diags {:?}", diags);
            let mut parser = Parser::new(tokens);
            std::fs::write("C:\\Users\\daxingyi\\AppData\\Local\\Temp\\opencode\\m_parse.txt", "parse").unwrap();
            let result = parser.parse_source_file();
            std::fs::write("C:\\Users\\daxingyi\\AppData\\Local\\Temp\\opencode\\m_done.txt", "done").unwrap();
            assert!(result.is_ok(), "parse failed");
        })
        .expect("spawn");
    handle.join().expect("join");
}