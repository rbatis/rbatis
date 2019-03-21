use crate::lib::RustExpressionEngine::parser;
use serde_json::json;
use serde_json::Value;
use crate::lib::RustExpressionEngine::runtime::OptMap;

#[test]
fn TestParser() {
    let (boxNode,_ )= parser::Parser(String::from("a<=b+1-2>=1=='asdf + sdaf '"),&OptMap::new());
    let john = json!({
        "name": "John Doe",
        "age": Value::Null,
         "sex":{
            "a":"i'm a",
            "b":"i'm b",
         },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    let (v,_)=boxNode.Eval(&john);
    println!("item:{}", v);

}