use crate::ast::NodeConfigHolder::NodeConfigHolder;
use serde_json::Value;
use crate::ast::Node::{DoChildNodes, SqlNode};
use crate::ast::NodeType::NodeType;

#[derive(Clone)]
pub struct  WhereNode{
    pub childs: Vec<NodeType>,
}

impl SqlNode for WhereNode{

    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        let result= DoChildNodes(&mut self.childs, env,holder);
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

    fn print(&self) -> String {
        let mut result = "<where".to_string();
        result = result + ">";
        for x in &self.childs {
            result = result + x.print().as_str();
        }
        result = result + "</select>";
        return result;
    }
}