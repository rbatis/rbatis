use crate::lib::RustExpressionEngine::parser;
use crate::lib::RustExpressionEngine::runtime::OptMap;
use chrono::Local;
use serde_json::json;
use test::Bencher;

#[bench]
fn Bench_Parser(b: &mut Bencher) {
    let (mut boxNode,_ )= parser::Parser(String::from("n == 1"), &OptMap::new());
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