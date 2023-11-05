mod ast ;

use ast::lexer::Token;
use ast::lexer::TokenKind;

fn main() {

    let x = Token{kind: TokenKind::Plus};
    println!("Hello, world! ");
}
