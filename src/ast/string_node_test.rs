use crate::ast::string_node::StringNode;
use std::rc::Rc;

use crate::ast::convert::sql_arg_type_convert_default::SqlArgTypeConvertDefault;
use crate::engines::RbatisEngine::node::NodeType::NString;
use crate::ast::node::SqlNode;
use serde_json::json;
use test::Bencher;
use crate::ast::node_config_holder::NodeConfigHolder;
use crate::engines::RbatisEngine::runtime::RbatisEngine;

#[test]
pub fn TestStringNode(){
    let mut john = json!({
        "arg": 2,
    });
    let mut holder=NodeConfigHolder::new();
    let mut sNode = StringNode::new("arg+1=#{arg+1}");

    let r=sNode.eval(&mut john,&mut holder).unwrap();
    println!("{}",r);
}


#[bench]
fn Bench_StringNode(b: &mut Bencher) {
    let mut john = json!({
        "arg": 2,
    });

    let engine= RbatisEngine::new();

    let mut sNode = StringNode::new("arg+1=#{arg}");
    let mut holder=NodeConfigHolder::new();
    b.iter(|| {
        sNode.eval(&mut john,&mut holder).unwrap();
    });
}