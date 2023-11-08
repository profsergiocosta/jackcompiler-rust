



use jackcompiler_rust::token::*;
use jackcompiler_rust::scanner::*;


#[test]
fn test_integer() {
    
    let tk = Token::new(TokenType::IntegerLiteral(20), 0);

    assert_eq!(tk.to_string(),"<integerConstant> 20 </integerConstant>");
}




