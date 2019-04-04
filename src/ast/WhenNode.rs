use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct WhenNode {
    pub childs: Vec<NodeType>,
    pub test: String,

    pub holder: Box<NodeConfigHolder>,
}

impl  SqlNode for WhenNode{
    fn eval(&mut self, env: &mut Value) -> Result<String,String> {
        let resultValue = self.holder.engine.LexerAndEval(self.test.as_str(), env);
        if resultValue.is_err(){
            return Result::Err(resultValue.err().unwrap());
        }
        let result=resultValue.unwrap();
        if !result.is_boolean(){
            return Result::Err("[RustMybatis] test:'".to_owned()+self.test.as_str()+"' is not return bool!");
        }
        if result.as_bool().unwrap() {
            return DoChildNodes(&mut self.childs, env);
        }
        return Result::Ok("".to_string());
    }
}