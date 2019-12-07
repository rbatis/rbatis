use crate::ast::node_type::NodeType;
use crate::ast::node::{SqlNode, DoChildNodes, print_child, create_deep};
use serde_json::Value;
use crate::ast::node_config_holder::NodeConfigHolder;

#[derive(Clone)]
pub struct InsertTempleteNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}


impl SqlNode for InsertTempleteNode{
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        return DoChildNodes(&mut self.childs, env,holder);
    }

    fn print(&self,deep:i32) -> String {
        let mut result="#{templete_space1}<insertTemplete id=\"#{templete_id}\" >#{templete_child}#{templete_space2}</insertTemplete>".to_string();
        result=result.replace("#{templete_space1}",create_deep(deep).as_str());
        result=result.replace("#{templete_space2}",create_deep(deep).as_str());

        result=result.replace("#{templete_id}",self.id.as_str());
        result=result.replace("#{templete_child}",print_child(self.childs.as_ref(),deep+1).as_str());
        return result;
    }
}
