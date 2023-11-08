



use jackcompiler_rust::token::*;
use jackcompiler_rust::scanner::*;



#[test]
fn test_integer_scan() {
    let mut scan = Scanner::new("20".to_string());
    let tk = scan.next_token();
    assert_eq!(tk.to_string(),"<integerConstant>20</integerConstant>");
}



