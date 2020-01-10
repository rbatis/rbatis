use serde_json::{json, Value};
use serde_json::ser::State::Rest;

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::xml::node_type::NodeType;
use crate::ast::xml::string_node::StringNode;

#[derive(Clone)]
pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl Ast for IfNode {
    fn eval(&self, env: &mut Value,arg_array:&mut Vec<Value>, holder: &mut ConfigHolder) -> Result<String, String> {
        let result = holder.engine.eval(self.test.as_str(), env);
        if result.is_err() {
            return Result::Err(result.err().unwrap());
        }
        let b = &result.unwrap();
        if !b.is_boolean() {
            return Result::Err("[rbatis] express:'".to_owned() + self.test.as_str() + "' is not return bool value!");
        }
        if b.as_bool().unwrap() {
            return do_child_nodes(&self.childs, env, arg_array,holder);
        }
        return Result::Ok("".to_string());
    }
}

impl SqlNodePrint for IfNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<if ";
        result = result + " test=\"" + self.test.as_str() + "\" >";
        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</if>";
        return result;
    }
}


#[test]
pub fn test_if_node() {
    let node = IfNode {
        childs: vec![NodeType::NString(StringNode::new("yes"))],
        test: "arg == 1".to_string(),
    };
    let mut john = json!({
        "arg": 1,
    });
    let mut holder = ConfigHolder::new();
    let mut arg_array=vec![];

    println!("{}", node.eval(&mut john, &mut arg_array,&mut holder).unwrap());
}