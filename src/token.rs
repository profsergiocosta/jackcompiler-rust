

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


pub enum TokenType {
    Keyword(Keyword),
    Symbol(Symbol),
    IntegerLiteral(u16),
    StringLiteral (String),
    Identifier (String),
    TkEOF,
}

pub struct Token {
    token_type: TokenType,
    line: u32,
}

impl Token {

    pub fn new (token_type: TokenType, line: u32) -> Self {
        Token{token_type, line}
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Keyword(k) => {
                let s = format!("<keyword>{:?}</keyword>",k).to_lowercase();
                return write!(f,"{}",s);

            },
            TokenType::IntegerLiteral(i) => write!(f,"<integerConstant>{}</integerConstant>",i),
            TokenType::Identifier(i) => write!(f,"<identifier>{}</identifier>",i),
            TokenType::StringLiteral(i) => write!(f,"<stringConstant>{}</stringConstant>",i),
            TokenType::Symbol(s) => write!(f,"<stringConstant>{}</stringConstant>",symbol2char(s)),
            TokenType::TkEOF =>  write!(f,""),
        }
        
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.token_type)
    }
}