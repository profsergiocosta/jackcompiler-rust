

//use std::fs::File;
//use std::io::Read;


//use token::*;
//use scanner::*;

use crate::scanner::Scanner;

pub struct Parser {
    scanner: Scanner,
    //curent_token: Token,
    //next_token: Token,
}

impl Parser {
    pub fn new(source_code: String) -> Parser {
        Parser {
            scanner: Scanner::new(source_code),
        }
    }

}