use serde_json::Value;

use crate::core::convert::StmtConvert;
use crate::core::Error;

use crate::ast::ast::RbatisAST;
use crate::ast::node::node::{SqlNodePrint, create_deep, print_child};
use crate::ast::node::node_type::NodeType;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone, Debug)]
pub struct CustomNode {
    pub childs: Vec<NodeType>,
}

impl RbatisAST for CustomNode {
    fn eval(&self, convert: &impl StmtConvert, env: &mut Value, engine: &RbatisEngine, arg_result: &mut Vec<Value>) -> Result<String, Error> {
        //TODO
        unimplemented!()
    }
}

impl SqlNodePrint for CustomNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<c>";
        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</c>";
        return result;
    }
}