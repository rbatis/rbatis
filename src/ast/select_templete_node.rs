use crate::ast::node_type::NodeType;
use crate::ast::node::{SqlNode, print_child, create_deep, do_child_nodes};
use serde_json::Value;
use crate::ast::node_config_holder::NodeConfigHolder;

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
        return do_child_nodes(&mut self.childs, env, holder);
    }

    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<selectTemplete ";
        result=result+"id=\""+self.id.as_str()+"\" ";
        result=result+"resultMap=\""+self.resultMap.as_str()+"\" ";
        result=result+"lang=\""+self.lang.as_str()+"\" ";
        result=result+"tables=\""+self.tables.as_str()+"\" ";
        result=result+"columns=\""+self.columns.as_str()+"\" ";
        result=result+"wheres=\""+self.wheres.as_str()+"\" ";


        result=result+">";
        result=result+print_child(self.childs.as_ref(),deep+1).as_str();
        result=result+create_deep(deep).as_str()+"</selectTemplete>";
        return result;
    }
}