
//use ast::lexer::Token;
//use crate::ast::lexer::TokenKind;
//use crate::ast::lexer::TextSpan;

mod token;


use token::*;


fn main() {

    let mut x = Token::Keyword(Keyword::While);
    println!("{}", x);
    x = Token::IntegerLiteral(20);
    println!("{}", x);
    x = Token::Identifier("var1".to_string());
    println!("{}", x);

    x = Token::StringLiteral("Joao pe de feijao".to_string());
    println!("{}", x);

    x = Token::Symbol(Symbol::LParen);
    println!("{}", x);

    //let y = TextSpan::new(10, 100, "ola".to_string());
    //println!("Hello, world! {}", y.length());
}
