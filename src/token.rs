

use core::fmt;



#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    LeftCurlyBraces,
    RightCurlyBraces,
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBrackets,
    RightSquareBrackets,
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



fn symbol2string(s: &Symbol) -> String {
    match s {
        Symbol::LeftCurlyBraces => String::from("{"),
        Symbol::RightCurlyBraces => String::from("}"),
        Symbol::LeftParenthesis => String::from("("),
        Symbol::RightParenthesis => String::from(")"),
        Symbol::LeftSquareBrackets => String::from("["),
        Symbol::RightSquareBrackets => String::from("]"),
        Symbol::Dot => String::from("."),
        Symbol::Comma => String::from(","),
        Symbol::Semicolon => String::from(";"),
        Symbol::Plus => String::from("+"),
        Symbol::Minus => String::from("-"),
        Symbol::Asterisk => String::from("*"),
        Symbol::Slash => String::from("/"),
        Symbol::Ampersand => String::from("&amp;"),
        Symbol::VerticalBar => String::from("|"),
        Symbol::LessThan => String::from("&lt;"),
        Symbol::GreaterThan => String::from("&gt;"),
        Symbol::Equal => String::from("="),
        Symbol::Tilde => String::from("~"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Keyword(Keyword),
    Symbol(Symbol),
    IntegerLiteral(u16),
    StringLiteral (String),
    Identifier (String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
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
                let s = format!("<keyword> {:?} </keyword>",k).to_lowercase();
                return write!(f,"{}",s);

            },
            TokenType::IntegerLiteral(i) => write!(f,"<integerConstant> {} </integerConstant>",i),
            TokenType::Identifier(i) => write!(f,"<identifier> {} </identifier>",i),
            TokenType::StringLiteral(i) => write!(f,"<stringConstant> {} </stringConstant>",i),
            TokenType::Symbol(s) => write!(f,"<symbol> {} </symbol>",symbol2string(s)),
        }
        
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.token_type)
    }
}