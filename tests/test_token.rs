



use jackcompiler_rust::token::*;
use jackcompiler_rust::scanner::*;


#[test]
fn test_integer() {
    let x = Token::IntegerLiteral(20);

    assert_eq!(x.to_string(),"<integerConstant>20</integerConstant>");
}




