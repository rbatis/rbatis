use crate::ast::xml::bind_node::BindNode;
use std::rc::Rc;
use crate::ast::xml::node::SqlNode;
use serde_json::json;

use crate::ast::config_holder::ConfigHolder;

#[test]
fn test_bind_node(){
    let mut holder= ConfigHolder::new();
    let bind_node =BindNode{
        name: "a".to_string(),
        value: "a+1".to_string(),
    };

    let mut john = json!({
        "a": 1,
    });


    let r= bind_node.eval(& mut john, &mut holder).unwrap();


    println!("r={}",r);
    println!("john[a]={}",john["a"]);
}