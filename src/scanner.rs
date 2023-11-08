
use crate::token::*;

pub struct Scanner {
    index: usize,
    source: String,
    line: usize,
    column: usize,
}

impl Scanner {
    pub fn new(source_code: String) -> Scanner {
        Scanner {
            index: 0,
            source: source_code,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        
        let length = self.source.chars().count();

        if self.index < length {

            let character = self.current_char();

            if character.is_numeric() {
                return self.scan_integer_literal();
            }

            if character.is_alphanumeric() {
                return self.scan_identifier_or_keyword();
            }


        }
        return Token::TkEOF;
    }


    fn scan_integer_literal(&mut self) -> Token {
        let length = self.source.chars().count();
        let mut buffer = String::new();
        let mut character = self.current_char();

        while character.is_numeric() && self.index < length {
            buffer.push(character);
            self.index += 1;
            self.column += 1;
            character = self.current_char();
        }

        let integer: u16 = buffer.parse().expect("Integer literal expected");
        return Token::IntegerLiteral(integer)
    }



    fn scan_identifier_or_keyword(&mut self) -> Token {
        let length = self.source.chars().count();
        let mut buffer = String::new();
        let mut character = self.current_char();

        while character.is_alphanumeric() && self.index < length {
            buffer.push(character);
            self.index += 1;
            self.column += 1;
            character = self.current_char();
        }

        match buffer.as_str() {
            "class" => Token::Keyword(Keyword::Class),
            "constructor" => Token::Keyword(Keyword::Constructor),
            "function" => Token::Keyword(Keyword::Function),
            "method" => Token::Keyword(Keyword::Method),
            "field" => Token::Keyword(Keyword::Field),
            "static" => Token::Keyword(Keyword::Static),
            "var" => Token::Keyword(Keyword::Var),
            "int" => Token::Keyword(Keyword::Int),
            "char" => Token::Keyword(Keyword::Char),
            "boolean" => Token::Keyword(Keyword::Boolean),
            "void" => Token::Keyword(Keyword::Void),
            "true" => Token::Keyword(Keyword::True),
            "false" => Token::Keyword(Keyword::False),
            "null" => Token::Keyword(Keyword::Null),
            "this" => Token::Keyword(Keyword::This),
            "let" => Token::Keyword(Keyword::Let),
            "do" => Token::Keyword(Keyword::Do),
            "if" => Token::Keyword(Keyword::If),
            "else" => Token::Keyword(Keyword::Else),
            "while" => Token::Keyword(Keyword::While),
            "return" => Token::Keyword(Keyword::Return),
            _ => Token::Identifier(buffer),
        }
    }

    pub fn current_char(&self) -> char {
      self.source
          .get(self.index..=self.index)
          .unwrap_or_else(|| "")
          .chars()
          .next()
          .unwrap_or_else(|| '\0')
    }

    fn next_char(&self) -> char {
        self.source
            .get(self.index + 1..=self.index + 1)
            .unwrap_or_else(|| "")
            .chars()
            .next()
            .unwrap_or_else(|| '\0')
    }
}