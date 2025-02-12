use ruskey::repl::Repl;

#[test]
fn test_repl_basic_functionality() {
    let mut input = "let x = 5;\n".as_bytes();
    let mut output = Vec::new();

    let mut repl = Repl::new();
    repl.start(&mut input, &mut output).unwrap();

    let output_str = String::from_utf8(output).unwrap();

    assert!(output_str.contains("Type:Let"));
    assert!(output_str.contains("Literal:let"));
    assert!(output_str.contains("Type:Ident"));
    assert!(output_str.contains("Literal:x"));
    assert!(output_str.contains("Type:Assign"));
    assert!(output_str.contains("Type:Int"));
    assert!(output_str.contains("Literal:5"));
}
