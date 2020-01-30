use serde_json::Value;

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::node::string_node::StringNode;
use crate::ast::node::trim_node::TrimNode;
use crate::ast::node::foreach_node::ForEachNode;
use crate::ast::node::node_type::NodeType;


pub struct PyInterpreter {}

impl PyInterpreter {
    pub fn parser(&self, arg: &str) {
        let mut node_types =vec![];
        let ls = arg.lines();
        for x in ls {
            let trim_x = x.trim();
            if trim_x.starts_with("if ") && trim_x.ends_with(":") {
                //if node
            } else if trim_x.starts_with("for ") && trim_x.ends_with(":") {
                //for node
                node_types.push(NodeType::NForEach(ForEachNode{
                    childs: vec![],
                    collection: "".to_string(),
                    index: "".to_string(),
                    item: "".to_string(),
                    open: "".to_string(),
                    close: "".to_string(),
                    separator: "".to_string()
                }))
            } else if trim_x.starts_with("trim ") && trim_x.ends_with(":") {
                //trim node
                node_types.push(NodeType::NTrim(TrimNode{
                    childs: vec![],
                    prefix: "".to_string(),
                    suffix: "".to_string(),
                    suffix_overrides: "".to_string(),
                    prefix_overrides: "".to_string()
                }));
            } else if trim_x.ends_with(":") {
                //unknow node
            } else {
                //string node
                node_types.push(NodeType::NString(StringNode::new(trim_x)));
            }
        }
    }
}


#[test]
pub fn test_service2() {
    let s = "select * from biz_activity
    if  name!=null:
      and delete_flag = #{del}
    and a = 0
    for item in ids:
      #{item}
    trim 'and':
      and delete_flag = #{del}
    where id  = '2';";
    println!("{}", s);

    let p = PyInterpreter {};
    p.parser(s);
}