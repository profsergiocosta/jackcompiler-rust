use std::{fs::File, process::Output};
use std::io::Read;

use jackcompiler_rust::parser::{ Parser};


#[test]
fn test_const() {
    let input = "20";
    let mut parser = Parser::new(input.to_string());
 

    let expected = "push constant 20\n";
    

    match parser.parse_term() {
        Ok(_) => {
            let output = parser.vm_output();
            assert_eq!(output,expected);
        }
        Err(err) => {
            // Em caso de erro, o teste falha e exibe a mensagem
            eprintln!("Erro: {}", err);
            panic!("Test failed due to error"); // Garante que o teste falhe
        }
    }

   

}