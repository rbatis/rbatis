use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct InsertTempleteNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}

impl SqlNode for InsertTempleteNode{
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        return DoChildNodes(&mut self.childs, env,holder);
    }

    fn print(&self) -> String {
        let mut result="<insertTemplete ".to_string();
        result=result+"id="+self.id.as_str();
        result=result+">";

        for x in &self.childs {
            result=result+x.print().as_str();
        }
        return result;
    }
}
