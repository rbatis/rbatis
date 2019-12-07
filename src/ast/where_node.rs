use crate::ast::node_config_holder::NodeConfigHolder;
use serde_json::Value;
use crate::ast::node::{do_child_nodes, SqlNode, create_deep, print_child};
use crate::ast::node_type::NodeType;

#[derive(Clone)]
pub struct  WhereNode{
    pub childs: Vec<NodeType>,
}

impl SqlNode for WhereNode{

    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        let result= do_child_nodes(&mut self.childs, env, holder);
        if result.is_ok() {
            let r=result.unwrap();
            let s=r.trim();
            if s.is_empty(){
                return Result::Ok(" ".to_string());
            }else{
                return Result::Ok(" where ".to_string()+s.trim_start_matches("and "));
            }
        }else{
            return Result::Err(result.err().unwrap());
        }
    }

    fn print(&self,deep:i32) -> String {
        let mut result = create_deep(deep)+"<where";
        result = result + ">";
        result=result+print_child(self.childs.as_ref(),deep+1).as_str();
        result = result + create_deep(deep).as_str()+"</where>";
        return result;
    }
}