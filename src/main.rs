
//use ast::lexer::Token;
//use crate::ast::lexer::TokenKind;
//use crate::ast::lexer::TextSpan;

mod token;
mod scanner;
mod parser;


use std::fs::File;
use std::io::Read;


use token::*;
use scanner::*;
use parser::*;




fn main() {

    let x = TokenType::IntegerLiteral(20);
    let tk = Token::new(x, 0);
    println!("{}", tk);

    let p = Parser::new("10".to_string());
    
}
