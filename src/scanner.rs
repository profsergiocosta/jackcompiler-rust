
use crate::token::*;

pub struct Scanner {
    index: usize,
    source: String,
    line: u32,
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

    pub fn next_token(&mut self) -> Result<Token, String> {
        
        let length = self.source.chars().count();

        if self.index < length {

            let character = self.current_char();

            if character.is_numeric() {
                return Ok(self.scan_integer_literal());
            }

            if character.is_alphanumeric() {
                return Ok(self.scan_identifier_or_keyword());
            }


        }
        return Err("End of file".to_string());
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
        return Token::new (TokenType::IntegerLiteral(integer),self.line)
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

        let tk_type = match buffer.as_str() {
            "class" => TokenType::Keyword(Keyword::Class),
            "constructor" => TokenType::Keyword(Keyword::Constructor),
            "function" => TokenType::Keyword(Keyword::Function),
            "method" => TokenType::Keyword(Keyword::Method),
            "field" => TokenType::Keyword(Keyword::Field),
            "static" => TokenType::Keyword(Keyword::Static),
            "var" => TokenType::Keyword(Keyword::Var),
            "int" => TokenType::Keyword(Keyword::Int),
            "char" => TokenType::Keyword(Keyword::Char),
            "boolean" => TokenType::Keyword(Keyword::Boolean),
            "void" => TokenType::Keyword(Keyword::Void),
            "true" => TokenType::Keyword(Keyword::True),
            "false" => TokenType::Keyword(Keyword::False),
            "null" => TokenType::Keyword(Keyword::Null),
            "this" => TokenType::Keyword(Keyword::This),
            "let" => TokenType::Keyword(Keyword::Let),
            "do" => TokenType::Keyword(Keyword::Do),
            "if" => TokenType::Keyword(Keyword::If),
            "else" => TokenType::Keyword(Keyword::Else),
            "while" => TokenType::Keyword(Keyword::While),
            "return" => TokenType::Keyword(Keyword::Return),
            _ => TokenType::Identifier(buffer),
        };

        return Token::new(tk_type, 0);
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
