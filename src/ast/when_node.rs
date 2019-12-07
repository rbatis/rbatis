use crate::ast::node_type::NodeType;
use crate::ast::node::{SqlNode, do_child_nodes, print_child, create_deep};
use serde_json::Value;
use crate::ast::config_holder::ConfigHolder;
use std::borrow::BorrowMut;


pub struct WhenNode {
    pub childs: Vec<NodeType>,
    pub test: String,

}

impl Clone for WhenNode{
    fn clone(&self) -> Self {
        return Self{
            childs: self.childs.clone(),
            test: self.test.clone(),
        }
    }
}

impl  SqlNode for WhenNode{
    fn eval(&mut self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String,String> {
        let result_value = holder.engine.eval(self.test.as_str(), env);
        if result_value.is_err(){
            return Result::Err(result_value.err().unwrap());
        }
        let result= result_value.unwrap();
        if !result.is_boolean(){
            return Result::Err("[rbatis] test:'".to_owned()+self.test.as_str()+"' is not return bool!");
        }
        if result.as_bool().unwrap() {
            return do_child_nodes(&mut self.childs, env, holder);
        }
        return Result::Ok("".to_string());
    }

    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<when ";
        result=result+" test=\""+self.test.as_str()+"\" >";
        result=result+print_child(self.childs.as_ref(),deep+1).as_str();
        result=result+create_deep(deep).as_str()+"</when>";
        return result;
    }
}