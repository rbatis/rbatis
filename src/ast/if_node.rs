use crate::ast::node_type::NodeType;
use crate::ast::node::{SqlNode, do_child_nodes, print_child, create_deep, SqlNodePrint};
use serde_json::Value;
use crate::ast::config_holder::ConfigHolder;
use serde_json::ser::State::Rest;

#[derive(Clone)]
pub struct IfNode{
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl SqlNode for IfNode {
    fn eval(&self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String, String> {
        let result = holder.engine.eval(self.test.as_str(), env);
        if result.is_err() {
            return Result::Err(result.err().unwrap());
        }
        let b = &result.unwrap();
        if !b.is_boolean() {
           return  Result::Err("[rbatis] express:'".to_owned() + self.test.as_str() + "' is not return bool value!");
        }
        if b.as_bool().unwrap() {
            return do_child_nodes(&self.childs, env, holder);
        }
        return Result::Ok("".to_string());
    }
}

impl SqlNodePrint for IfNode{
    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<if ";
        result=result+" test=\""+self.test.as_str() +"\" >";
        result=result+print_child(self.childs.as_ref(),deep+1).as_str();
        result=result+create_deep(deep).as_str()+"</if>";
        return result;
    }
}