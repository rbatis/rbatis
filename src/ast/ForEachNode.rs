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
        unimplemented!();
//        let collection=env.get(self.collection.as_str());
//        if collection.is_none(){
//            return Result::Err("[RustMybatis] collection name:".to_owned()+self.collection.as_str()+" is none value!");
//        }
//
//
//
//        return Result::Err("".to_string());
    }
}