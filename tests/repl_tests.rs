use ruskey::repl::Repl;
use std::io::Cursor;

#[test]
fn test_repl_parser_functionality() {
    let input = "let x = 5 + 5;\n".as_bytes();
    let mut output = Vec::new();

    let mut repl = Repl::new();
    repl.start_parser_mode(&mut Cursor::new(input), &mut output)
        .unwrap();

    let output_str = String::from_utf8(output).unwrap();

    assert!(!output_str.is_empty());
}
