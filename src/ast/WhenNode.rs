use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;
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
        let resultValue = holder.engine.LexerAndEval(self.test.as_str(), env);
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

    fn print(&self) -> String {
        let mut result="<when ".to_string();
        result=result+self.test.as_str() +">";
        for x in &self.childs {
            result=result+x.print().as_str();
        }
        return result;
    }
}