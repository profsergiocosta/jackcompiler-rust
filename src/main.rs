
//use ast::lexer::Token;
//use crate::ast::lexer::TokenKind;
//use crate::ast::lexer::TextSpan;

mod token;
mod scanner;




use token::*;
use scanner::*;

fn main() {

    let mut x = Token::Keyword(Keyword::While);
    println!("{}", x);
    x = Token::IntegerLiteral(20);
    
    println!("{}", x.to_string());
    x = Token::Identifier("var1".to_string());
    println!("{}", x);

    x = Token::StringLiteral("Joao pe de feijao".to_string());
    println!("{}", x);

    x = Token::Symbol(Symbol::LParen);
    println!("{}", x);

    //let y = TextSpan::new(10, 100, "ola".to_string());
    //println!("Hello, world! {}", y.length());

    let scan = Scanner::new("let x = 10;".to_string());

    // Agora, você pode trabalhar com o conteúdo lido
    println!("{}", scan.current_char());
    println!("{}", scan.current_char());
    println!("{}", scan.current_char());
}
