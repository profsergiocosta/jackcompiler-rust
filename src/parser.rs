

//use std::fs::File;
//use std::io::Read;


//use token::*;
//use scanner::*;



//use crate::vmwriter::{VMWriter};

use crate::{scanner::Scanner, token::{Token, TokenType}, vmwriter::{Segment, VMWriter}};


pub struct Parser {
    scanner: Scanner,
    vmwriter: VMWriter,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    //curent_token: Token,
    //next_token: Token,
}

impl Parser {
    pub fn new(source_code: String) -> Parser {
        Parser {
            scanner: Scanner::new(source_code),
            vmwriter: VMWriter::new(),
            current_token: None,
            peek_token: None,
        }
    }

    pub fn next_token(&mut self) {

        self.current_token = self.peek_token.take();
        self.peek_token = self.scanner.next_token();
    }


    pub fn parse_term(&mut self) -> Result<(), String> {
        match self.peek_token.as_ref() {
            Some(token) => match token.token_type {
                TokenType::IntegerLiteral(i)   => {
                    self.expect_peek(TokenType::IntegerLiteral(i));
                    Ok(())
                }
                _ => Err(self.error(token, "term expected")),
            },
            None => Err("Unexpected end of input: term expected".to_string()),
        }
    }

    /* 
    pub fn parse_term(&mut self) {
        self.vmwriter.write_push(Segment::Const, 10);
    }
    */

    pub fn vm_output(&self) -> &str {
        &self.vmwriter.vm_output()
    }

    /*
        /// Verifica se `peek_token` corresponde a qualquer um dos tipos fornecidos.
        fn expect_peek_multiple(&mut self, types: &[TokenType]) -> Result<(), String> {
            for &token_type in types {
                if let Some(peek) = &self.peek_token {
                    if peek.token_type == token_type {
                        return self.expect_peek(token_type); // Chamando `expect_peek` diretamente
                    }
                }
            }
    
            // Gera um erro se nenhum tipo corresponder
            let peek = self.peek_token.clone().unwrap_or(Token::eof());
            Err(self.error(&peek, "Expected a statement"))
        }

         */
    
         fn expect_peek(&mut self, expected_type: TokenType) -> Result<(), String> {
            match self.peek_token.as_ref() {
                Some(peek) if expected_type == peek.token_type => {
                    self.next_token(); // Avança para o próximo token
                     Ok(())
                }
                Some(peek) => {
                    // Gera um erro se o tipo não corresponder
                    Err(self.error(peek, &format!("Expected {}", expected_type)))
                }
                None => {
                    // Se o `peek_token` for None, é o final do input
                    Err("Unexpected end of input".to_string())
                }
            }
        }
    
        /// Gera um erro formatado.
        fn error(&self, token: &Token, message: &str) -> String {

            

            //if token.token_type == TokenType::EOF {
              //  self.report(token.line, " at end", message)
//            } else {
                self.report(token.line, &format!(" at '{}'", token.lexeme), message)
           // }
        }
    
        /// Reporta um erro no formato desejado.
        fn report(&self, line: u32, where_: &str, message: &str) -> String {
            let error_message = format!("[line {}] Error{}: {}", line, where_, message);
            eprintln!("{}", error_message);
            error_message
        }

}
