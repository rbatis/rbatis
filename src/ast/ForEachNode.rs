use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

#[derive(Clone)]
pub struct ForEachNode {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub index: String,
    pub item: String,
    pub open: String,
    pub close: String,
    pub separator: String,
}

impl  SqlNode for ForEachNode {
    fn eval(&mut self, env: &mut Value) -> Result<String,String> {
        unimplemented!()
    }
}