
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