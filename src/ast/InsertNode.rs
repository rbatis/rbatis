use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct InsertNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}

impl SqlNode for InsertNode{
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {

        return DoChildNodes(&mut self.childs, env,holder);
    }

    fn print(&self) -> String {
        let mut result="\n<insert ".to_string();
        result=result+"id=\""+self.id.as_str()+"\"";
        result=result+">";
        for x in &self.childs {
            result=result+x.print().as_str();
        }
        result=result+" </insert>";
        return result;
    }
}
