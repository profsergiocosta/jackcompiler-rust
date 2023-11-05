
pub enum TokenKind {
    Number(i64),
    // Operators
    Plus,
    Minus, 
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
}

pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String
}

impl TextSpan {
    pub fn new (start:usize, end:usize, literal: String) -> Self {
        TextSpan{start, end, literal}
    }
    pub fn length (&self) -> usize {
        self.end - self.start
    }
}

pub struct Token {
    kind:TokenKind,
    span: TextSpan,

}

impl Token {
    pub fn new (kind:TokenKind, span: TextSpan) -> Self {
        Token {kind, span}
    }
}