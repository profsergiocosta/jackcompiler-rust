
//use ast::lexer::Token;
//use crate::ast::lexer::TokenKind;
//use crate::ast::lexer::TextSpan;

mod token;
mod scanner;




use token::*;
use scanner::*;

fn main() {

    let x = TokenType::IntegerLiteral(20);
    let tk = Token::new(x, 0);
    println!("{}", tk);
}
