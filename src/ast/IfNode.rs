use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes};
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

    fn print(&self) -> String {
        let mut result="\n<if ".to_string();
        result=result+" test=\""+self.test.as_str() +"\" >";
        for x in &self.childs {
            result=result+x.print().as_str();
        }
        result=result+" \n</if>";
        return result;
    }
}