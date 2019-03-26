use crate::ast::NodeType::{NodeType, StringNode};
use crate::ast::Node::Node;
use serde_json::json;

#[test]
fn TestNode(){
    let john = json!({
        "a":1,
        "name": "John Doe",
        "age": {
           "yes":"sadf"
        },
         "sex":{
            "a":"i'm a",
            "b":"i'm b",
         },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let strNode=NodeType::NString(StringNode{
        value: "vvvvvvvvvvvvvvvvvvvvv".to_string(),
        expressMap: vec![],
        noConvertExpressMap: vec![]
    });

   let result=  strNode.eval(john);
    println!("{}",result);
}
