

use core::fmt;



#[derive(Debug)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

pub enum Symbol {
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Dot,
    Comma,
    Semicolon,
    
    Plus,
    Minus,
    Asterisk,
    Slash,
    
    Ampersand,
    VerticalBar,

    LessThan,
    GreaterThan,
    Equal,
    Tilde,
}

fn symbol2char (s: &Symbol) -> char {
    match s {
        Symbol::LBrace => '{',
        Symbol::RBrace => '}',
        Symbol::LParen => '(',
        Symbol::RParen => ')',
        Symbol::LBracket => '[',
        Symbol::RBracket => ']',
        Symbol::Dot => '.',
        Symbol::Comma => ',',
        Symbol::Semicolon => ';',
        Symbol::Plus => '+',
        Symbol::Minus => '-',
        Symbol::Asterisk => '*',
        Symbol::Slash => '/',
        Symbol::Ampersand => '&',
        Symbol::VerticalBar => '|',
        Symbol::LessThan => '<',
        Symbol::GreaterThan => '>',
        Symbol::Equal => '=',
        Symbol::Tilde => '~',
    }
}


pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    IntegerLiteral(u16),
    StringLiteral (String),
    Identifier (String),
    TkEOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Keyword(k) => {
                let s = format!("<keyword>{:?}</keyword>",k).to_lowercase();
                return write!(f,"{}",s);

            },
            Token::IntegerLiteral(i) => write!(f,"<integerConstant>{}</integerConstant>",i),
            Token::Identifier(i) => write!(f,"<identifier>{}</identifier>",i),
            Token::StringLiteral(i) => write!(f,"<stringConstant>{}</stringConstant>",i),
            Token::Symbol(s) => write!(f,"<stringConstant>{}</stringConstant>",symbol2char(s)),
            Token::TkEOF =>  write!(f,""),
        }
        
    }
}