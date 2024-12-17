
//use ast::lexer::Token;
//use crate::ast::lexer::TokenKind;
//use crate::ast::lexer::TextSpan;

mod token;
mod scanner;
mod parser;
mod vmwriter;


//use std::fs::File;
//use std::io::Read;


//use parser::Parser;
//use token::*;
use scanner::*;
use parser::*;
use vmwriter::*;

use std::fs::File;
use std::io::Read;

fn main() {

    //let p = Parser::new("10".to_string());
    let mut writer = VMWriter::new();

    writer.write_push(Segment::Local, 0);
    writer.write_pop(Segment::Arg, 2);
    writer.write_arithmetic(Command::Add);
    writer.write_label("LOOP_START");
    writer.write_goto("LOOP_START");
    writer.write_if("END");
    writer.write_call("Main.run", 2);
    writer.write_function("Main.run", 4);
    writer.write_return();

    println!("{}", writer.vm_output());
    
}
