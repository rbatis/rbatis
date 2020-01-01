use crate::ast::xml::foreach_node::ForEachNode;
use crate::ast::xml::node::SqlNode;
use serde_json::json;
use crate::ast::xml::node_type::NodeType;
use crate::ast::xml::string_node::StringNode;
use std::rc::Rc;
use crate::ast::convert::sql_arg_type_convert_default::SqlArgTypeConvertDefault;
use crate::ast::config_holder::ConfigHolder;


#[test]
pub fn test_for_each_node(){
    let mut holder= ConfigHolder::new();
    let n=ForEachNode{
        childs: vec![NodeType::NString(StringNode::new("index:#{index},item:#{item}"))],
        collection: "arg".to_string(),
        index: "index".to_string(),
        item: "item".to_string(),
        open: "(".to_string(),
        close: ")".to_string(),
        separator: ",".to_string()
    };
    let mut john = json!({
        "arg": 1,
    });

    let r=n.eval(&mut john,&mut holder);
    println!("{}", r.unwrap_or("null".to_string()));
}