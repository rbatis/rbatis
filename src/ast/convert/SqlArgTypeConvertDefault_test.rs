use crate::ast::convert::SqlArgTypeConvert::SqlArgTypeConvert;
use crate::ast::convert::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
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
        let a=json!(null);
        convert.convert(a);
    });
}