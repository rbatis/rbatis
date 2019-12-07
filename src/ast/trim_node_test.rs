use crate::ast::trim_node::TrimNode;
use serde_json::json;
use crate::ast::node::SqlNode;
use crate::ast::node_type::NodeType;
use crate::ast::string_node::StringNode;
use crate::ast::node_config_holder::NodeConfigHolder;

#[test]
pub fn test_trim_node(){
    let mut holder=NodeConfigHolder::new();
    let mut node =TrimNode{
        childs: vec![NodeType::NString(StringNode::new("1trim value1"))],
        prefix: "(".to_string(),
        suffix: ")".to_string(),
        suffix_overrides: "1".to_string(),
        prefix_overrides: "1".to_string()
    };
    let mut john = json!({
        "arg": 2,
    });

   let r= node.eval(&mut john,&mut holder).unwrap();
    println!("{}",r)
}