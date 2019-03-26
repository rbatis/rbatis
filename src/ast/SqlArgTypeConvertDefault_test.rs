use crate::ast::SqlArgTypeConvert::SqlArgTypeConvert;
use crate::ast::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use serde_json::json;

#[test]
fn TestConvert(){
    let convert=SqlArgTypeConvertDefault{};
    let mut result;
    result =  convert.convert(json!(1));
    println!("number(i64)=>{}",result);
    result =  convert.convert(json!(1.2));
    println!("number(f64)=>{}",result);
    result =  convert.convert(json!("abc"));
    println!("string=>{}",result);
    result =  convert.convert(json!(null));
    println!("null=>{}",result);
}