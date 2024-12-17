use std::{fs::File, process::Output};
use std::io::Read;

use jackcompiler_rust::parser::{ Parser};


#[test]
fn test_const() {
    let input = "10";
    let mut parser = Parser::new(input.to_string());
    parser.parseTerm();

    let expected = "push constant 10\n";
    let output = parser.vm_output();

    assert_eq!(output,expected);

}