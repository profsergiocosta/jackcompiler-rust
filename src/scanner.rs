
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
/*
    pub fn next_token(&mut self) -> Result<Token, String> {
        
        let length = self.source.chars().count();

        if self.index < length {

            let character = self.current_char();
            let next_character = self.next_char();

            if character == '\n' {
                self.line += 1;
                self.column = 1;
            }

            if character == '/' && next_character == '/' {
                self.skip_line_comments();
                return self.next_token();
            }

            if character == '/' && next_character == '*' {
                self.skip_block_comments();
                return self.next_token();
            }

        
            if character.is_whitespace() {
                self.index += 1;
                self.column += 1;
                return self.next_token();
            }


            if character.is_numeric() {
                return Ok(self.scan_integer_literal());
            }

            if character.is_alphanumeric() {
                return Ok(self.scan_identifier_or_keyword());
            }

            if character == '"' {
                self.index += 1;
                self.column += 1;
                return Ok(self.scan_string_literal());
            }

            return Ok (self.scan_symbol());

        }
        return Err("End of file".to_string());
    }
 */


    pub fn next_token(&mut self) -> Option<Token> {
        let length = self.source.chars().count();

        while self.index < length {
            let character = self.current_char();
            let next_character = self.next_char();

            if character == '\n' {
                self.line += 1;
                self.column = 1;
            }

            if character == '/' && next_character == '/' {
                self.skip_line_comments();
                continue; // Ignora comentário e busca o próximo token
            }

            if character == '/' && next_character == '*' {
                self.skip_block_comments();
                continue; // Ignora comentário e busca o próximo token
            }

            if character.is_whitespace() {
                self.index += 1;
                self.column += 1;
                continue; // Ignora espaços em branco e busca o próximo token
            }

            if character.is_numeric() {
                return Some(self.scan_integer_literal());
            }

            if character.is_alphanumeric() {
                return Some(self.scan_identifier_or_keyword());
            }

            if character == '"' {
                self.index += 1;
                self.column += 1;
                return Some(self.scan_string_literal());
            }

            return Some(self.scan_symbol());
        }

        // Retorna None ao atingir o fim do arquivo
        None
    }

    fn skip_line_comments(&mut self) {
        let length = self.source.chars().count();
        let mut character = self.current_char();

        while character != '\n' && self.index < length {
             self.index += 1;
             self.column += 1;

             character = self.current_char();
        }
        // next line
        self.index += 1;
        self.line += 1;
        self.column = 1;
        
    }

    fn skip_block_comments(&mut self) {
        let length = self.source.chars().count();
        let mut character = self.current_char();
        let mut next_character = self.next_char();

        self.index += 2;
        self.column += 2;

        while !(character == '*' && next_character == '/') && self.index < length {
                self.index += 1;
                self.column += 1;
                if character == '\n' {
                    self.line += 1;
                    self.column = 1;
                }

                character = self.current_char();
                next_character = self.next_char();
        }
        // consume * and /
        self.index += 2;
        self.column += 2;
    }


    fn scan_string_literal(&mut self) -> Token {
        let length = self.source.chars().count();
        let mut buffer = String::new();
        let mut character = self.current_char();

        while character != '"' && self.index < length {
            buffer.push(character);
            self.index += 1;
            self.column += 1;
            character = self.current_char();
        }

        self.index += 1;
        
        return Token::new (TokenType::StringLiteral(buffer),self.line)
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

        return Token::new(tk_type, self.line);
    }

    fn scan_symbol(&mut self) -> Token {
        let character = self.current_char();
        self.index += 1;
        self.column += 1;

        let tk_type = match character {
            '{' => TokenType::Symbol(Symbol::LeftCurlyBraces),
            '}' => TokenType::Symbol(Symbol::RightCurlyBraces),
            '(' => TokenType::Symbol(Symbol::LeftParenthesis),
            ')' => TokenType::Symbol(Symbol::RightParenthesis),
            '[' => TokenType::Symbol(Symbol::LeftSquareBrackets),
            ']' => TokenType::Symbol(Symbol::RightSquareBrackets),
            '.' => TokenType::Symbol(Symbol::Dot),
            ',' => TokenType::Symbol(Symbol::Comma),
            ';' => TokenType::Symbol(Symbol::Semicolon),
            '+' => TokenType::Symbol(Symbol::Plus),
            '-' => TokenType::Symbol(Symbol::Minus),
            '*' => TokenType::Symbol(Symbol::Asterisk),
            '/' => TokenType::Symbol(Symbol::Slash),
            '&' => TokenType::Symbol(Symbol::Ampersand),
            '|' => TokenType::Symbol(Symbol::VerticalBar),
            '<' => TokenType::Symbol(Symbol::LessThan),
            '>' => TokenType::Symbol(Symbol::GreaterThan),
            '=' => TokenType::Symbol(Symbol::Equal),
            '~' => TokenType::Symbol(Symbol::Tilde),
            _ => panic!(
                "Unknown character: {:?} at {}:{}",
                character, self.line, self.column
            ),
        };

        return Token::new(tk_type, self.line);
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
