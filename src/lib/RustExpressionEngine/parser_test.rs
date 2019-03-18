use crate::lib::RustExpressionEngine::parser;
use crate::lib::RustExpressionEngine::parser::OptMap;

#[test]
fn TestParser() {
    let vecStr = parser::Parser(String::from("a<=b+1-2>=1"));
    for item in vecStr {
        println!("item:{}", item);
    }
}