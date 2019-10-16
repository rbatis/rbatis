use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;
use serde_json::ser::State::Rest;

#[derive(Clone)]
pub struct IfNode{
    pub childs: Vec<NodeType>,
    pub test: String,
    pub holder: NodeConfigHolder,
}

impl SqlNode for IfNode {
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        let result = self.holder.engine.LexerAndEval(self.test.as_str(), env);
        if result.is_err() {
            return Result::Err(result.err().unwrap());
        }
        let b = &result.unwrap();
        if !b.is_boolean() {
           return  Result::Err("[RustMybatis] express:'".to_owned() + self.test.as_str() + "' is not return bool value!");
        }
        if b.as_bool().unwrap() {
            return DoChildNodes(&mut self.childs, env);
        }
        return Result::Ok("".to_string());
    }
}