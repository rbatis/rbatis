use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, print_child};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct SelectTempleteNode {
    pub id:String,
    pub resultMap:String,
    pub lang:String,
    pub tables:String,
    pub columns:String,
    pub wheres:String,
    pub childs: Vec<NodeType>,
}


impl SqlNode for SelectTempleteNode{
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        unimplemented!()
    }

    fn print(&self) -> String {
        let mut result="\n<selectTemplete ".to_string();
        result=result+"id=\""+self.id.as_str()+"\" ";
        result=result+"resultMap=\""+self.resultMap.as_str()+"\" ";
        result=result+"lang=\""+self.lang.as_str()+"\" ";
        result=result+"tables=\""+self.tables.as_str()+"\" ";
        result=result+"columns=\""+self.columns.as_str()+"\" ";
        result=result+"wheres=\""+self.wheres.as_str()+"\" ";


        result=result+">";
        result=print_child(result,self.childs.as_ref());
        result=result+"\n</selectTemplete>";
        return result;
    }
}