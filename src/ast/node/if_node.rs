use serde_json::{json, Value};
use serde_json::ser::State::Rest;

use crate::ast::ast::RbatisSqlAST;
use crate::ast::node::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::ast::node::string_node::StringNode;
use crate::convert::stmt_convert::StmtConvert;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone, Debug)]
pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl RbatisSqlAST for IfNode {
    fn eval(&self, convert: &impl StmtConvert, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, rbatis_core::Error> {
        let result = engine.eval(self.test.as_str(), env)?;
        if !result.is_boolean() {
            return Result::Err(rbatis_core::Error::from("[rbatis] express:'".to_owned() + self.test.as_str() + "' is not return bool value!"));
        }
        if result.as_bool().unwrap() {
            return do_child_nodes(convert, &self.childs, env, engine, arg_array);
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
    let mut engine = RbatisEngine::new();
    let mut arg_array = vec![];

    println!("{}", node.eval(&mut john, &mut engine, &mut arg_array).unwrap());
}