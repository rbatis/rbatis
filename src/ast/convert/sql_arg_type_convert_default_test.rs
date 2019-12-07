use crate::ast::convert::sql_arg_type_convert::SqlArgTypeConvert;
use crate::ast::convert::sql_arg_type_convert_default::SqlArgTypeConvertDefault;
use serde_json::{json, Value};
use chrono::Local;
use test::Bencher;

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


#[bench]
fn Bench_Convert(b: &mut Bencher) {
   // let a=json!(1.27);
    let convert=SqlArgTypeConvertDefault{};
    b.iter(|| {
        let node=json!("1234");
        convert.convert(node);
    });
}