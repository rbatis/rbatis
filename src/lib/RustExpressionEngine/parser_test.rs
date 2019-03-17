use crate::lib::RustExpressionEngine::parser;

#[test]
fn TestParser(){
    parser::Parser();
}

#[test]
fn TestParserString(){
    let s=String::from("a + b");
    println!("{}",s);
}