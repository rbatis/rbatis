use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes, print_child, create_deep};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;
use serde_json::ser::State::Rest;

#[derive(Clone)]
pub struct IfNode{
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl SqlNode for IfNode {
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        let result = holder.engine.LexerAndEval(self.test.as_str(), env);
        if result.is_err() {
            return Result::Err(result.err().unwrap());
        }
        let b = &result.unwrap();
        if !b.is_boolean() {
           return  Result::Err("[Rbatis] express:'".to_owned() + self.test.as_str() + "' is not return bool value!");
        }
        if b.as_bool().unwrap() {
            return DoChildNodes(&mut self.childs, env,holder);
        }
        return Result::Ok("".to_string());
    }

    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<if ";
        result=result+" test=\""+self.test.as_str() +"\" >";
        result=result+print_child(self.childs.as_ref(),deep+1).as_str();
        result=result+create_deep(deep).as_str()+"</if>";
        return result;
    }
}