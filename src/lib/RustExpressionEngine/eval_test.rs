use crate::lib::RustExpressionEngine::parser;
use crate::lib::RustExpressionEngine::runtime::OptMap;
use chrono::Local;
use serde_json::json;
use test::Bencher;

#[bench]
fn Bench_Parser(b: &mut Bencher) {
    let mut boxNode= parser::Parser(String::from("'1'+'1'"), &OptMap::new()).unwrap();
    let john = json!({
        "n":1,
        "name": "John Doe",
         "age": {
           "yes":"sadf"
        }
    });
    let now=Local::now();
    b.iter(|| {
        boxNode.eval(&john);
    });
}