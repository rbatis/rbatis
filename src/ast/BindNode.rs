use serde_json::Value;
use std::rc::Rc;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use crate::lib;
use crate::ast::Node::SqlNode;

#[derive(Clone)]
pub struct BindNode {
    pub name: String ,
    pub value: String ,

    pub engine: ExpressionEngineProxy<lib::RustExpressionEngine::node::Node, Value>,
}

impl SqlNode for BindNode {
    fn eval(&mut self, env: &mut Value) -> Result<String,String> {
        let r= self.engine.LexerAndEval(self.value.as_str(),env);
        env[self.name.as_str()]=r.unwrap_or(Value::Null);
        return Result::Ok("".to_string());
    }
}
