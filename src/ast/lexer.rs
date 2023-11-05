
pub enum TokenKind {
    Number(i64),
    // Operators
    Plus,
}

pub struct Token {
    pub kind:TokenKind

}