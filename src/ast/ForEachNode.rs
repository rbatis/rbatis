use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

pub struct ForEachNode<'a> {
    pub childs: Vec<NodeType<'a>>,
    pub collection: String,
    pub index: String,
    pub item: String,
    pub open: String,
    pub close: String,
    pub separator: String,
}

impl <'a> SqlNode for ForEachNode<'a> {
    fn eval(&mut self, env: &mut Value) -> String {
        unimplemented!()
    }
}