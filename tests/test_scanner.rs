

use std::fs::File;
use std::io::Read;


use jackcompiler_rust::token::*;
use jackcompiler_rust::scanner::*;



#[test]
fn test_integer_scan() {
    let mut scan = Scanner::new("20".to_string());
    let tk = scan.next_token().unwrap();
    assert_eq!(tk.to_string(),"<integerConstant> 20 </integerConstant>");
}


#[test]
fn test_identifier() {
    let mut scan = Scanner::new("variavel".to_string());
    let tk = scan.next_token().unwrap();
    println! ("{}", tk);
    assert_eq!(tk.to_string(),"<identifier> variavel </identifier>");
}


#[test]
fn test_keyword() {
    let mut scan = Scanner::new("while".to_string());
    let tk = scan.next_token().unwrap();
    println! ("{}", tk);
    assert_eq!(tk.to_string(),"<keyword> while </keyword>");
}

#[test]
fn test_string() {
    let mut scan = Scanner::new("\"ola\"".to_string());
    let tk = scan.next_token().unwrap();
    println! ("{}", tk);
    assert_eq!(tk.to_string(),"<stringConstant> ola </stringConstant>");
}

#[test]
fn test_symbol() {
    let mut scan = Scanner::new("+".to_string());
    let tk = scan.next_token().unwrap();
    println! ("{}", tk);
    assert_eq!(tk.to_string(),"<symbol> + </symbol>");
}


#[test]
fn test_whitespace() {
    let mut scan = Scanner::new("       while".to_string());
    let tk = scan.next_token().unwrap();
    println! ("{}", tk);
    assert_eq!(tk.to_string(),"<keyword> while </keyword>");
}


#[test]
fn test_line_comments() {
    let mut scan = Scanner::new(
    "
    // comentario qualquer
    while
    ".to_string());
    let tk = scan.next_token().unwrap();
    println! ("{}", tk);
    assert_eq!(tk.to_string(),"<keyword> while </keyword>");
}


#[test]
fn test_block_comments() {
    let mut scan = Scanner::new(
    "
    /*  
    comentario qualquer
    continua mais um pouco
    */
    while
    ".to_string());
    let tk = scan.next_token().unwrap();
    println! ("{}", tk);
    assert_eq!(tk.to_string(),"<keyword> while </keyword>");
}



#[test]
fn test_square() {


    let mut file = File::open("tests/resource/Square.jack").unwrap();
    // Cria um buffer para armazenar os dados lidos
    let mut contents = String::new();
    // Lê o conteúdo do arquivo para o buffer
    file.read_to_string(&mut contents);
    
    let mut scan = Scanner::new(contents);
    let mut file = File::open("tests/resource/SquareT.xml").unwrap();
    let mut expected = String::new();
    file.read_to_string(&mut expected);

    let mut xml_str = ("<tokens>\n").to_string();

    println!("{}",xml_str);

    while let Ok(tk) = scan.next_token() {
        xml_str
        .push_str(&format!("{}\n",tk));
    }

    xml_str
    .push_str(&format!("{}\n","</tokens>"));

    
    assert_eq!(xml_str,expected);
}

