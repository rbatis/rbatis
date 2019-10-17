use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

#[derive(Clone)]
pub struct DeleteNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}

impl SqlNode for DeleteNode{
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        unimplemented!()
    }

    fn print(&self) -> String {
        let mut result="<delete ".to_string();
        result=result+"id="+self.id.as_str();
        result=result+">";
        for x in &self.childs {
            result=result+x.print().as_str();
        }
        result=result+"</delete>";
        return result;
    }
}