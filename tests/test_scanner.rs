



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




