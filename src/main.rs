
//use ast::lexer::Token;
//use crate::ast::lexer::TokenKind;
//use crate::ast::lexer::TextSpan;

mod token;
mod scanner;
///mod parser;


//use std::fs::File;
//use std::io::Read;


//use token::*;
use scanner::*;
//use parser::*;


use std::fs::File;
use std::io::Read;

fn test_scanner() {
    let x = token::TokenType::IntegerLiteral(20);
    let tk = token::Token::new(x, 0);
    println!("{}", tk);

    //let p = Parser::new("10".to_string());

    let mut file = File::open("tests/resource/Square.jack").unwrap();
    // Cria um buffer para armazenar os dados lidos
    let mut contents = String::new();
    // Lê o conteúdo do arquivo para o buffer
    let _ = file.read_to_string(&mut contents);
    
    let mut scan = Scanner::new(contents);
    let mut file = File::open("tests/resource/SquareT.xml").unwrap();
    let mut expected = String::new();
    let _ = file.read_to_string(&mut expected);

    let mut xml_str = ("<tokens>\n").to_string();

    println!("{}",xml_str);

    while let Ok(tk) = scan.next_token() {
        xml_str
        .push_str(&format!("{}\n",tk));
    }

    xml_str
    .push_str(&format!("{}\n","</tokens>"));
}

fn main() {


    
}
