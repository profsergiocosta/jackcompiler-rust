mod ast ;

use ast::lexer::Token;
use crate::ast::lexer::TokenKind;
use crate::ast::lexer::TextSpan;

fn main() {

    
    let y = TextSpan::new(10, 100, "ola".to_string());
    println!("Hello, world! {}", y.length());
}
