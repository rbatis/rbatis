use crate::ast::node_type::NodeType;
use crate::ast::node::{SqlNode, DoChildNodes, print_child, create_deep};
use serde_json::Value;
use crate::ast::node_config_holder::NodeConfigHolder;
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
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String,String> {
        let resultValue = holder.engine.Eval(self.test.as_str(), env);
        if resultValue.is_err(){
            return Result::Err(resultValue.err().unwrap());
        }
        let result=resultValue.unwrap();
        if !result.is_boolean(){
            return Result::Err("[Rbatis] test:'".to_owned()+self.test.as_str()+"' is not return bool!");
        }
        if result.as_bool().unwrap() {
            return DoChildNodes(&mut self.childs, env,holder);
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