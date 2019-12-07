use crate::ast::node_type::NodeType;
use crate::ast::node::SqlNode;
use serde_json::{json, Value};
use crate::ast::string_node::StringNode;
//use test::Bencher;
use crate::ast::convert::sql_arg_type_convert_default::SqlArgTypeConvertDefault;
use std::rc::Rc;

use crate::ast::node_config_holder::NodeConfigHolder;
use crate::core::rbatis::Rbatis;



#[test]
fn test_string_node() {
    let mut holder=NodeConfigHolder::new();
    let mut john = json!({
        "name": "John Doe",
    });
    let mut strNode = NodeType::NString(StringNode::new("select * from ${name} where name = #{name}"));

    let result = strNode.eval(&mut john,&mut holder).unwrap();
    println!("{}", result);
}

//#[bench]
//fn bencher_parser(b: &mut Bencher) {
//    let mut holder=NodeConfigHolder::new();
//    let mut john =  json!({
//        "name": "John Doe",
//    });
//
//
//    let mut strNode = NodeType::NString(StringNode::new("vvvvvvvvvv#{name}vvvvvvvv"));
//
//    b.iter(|| {
//        &strNode.eval(&mut john,&mut holder);
//    });
//}


