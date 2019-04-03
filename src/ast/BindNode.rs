use serde_json::Value;
use std::rc::Rc;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use crate::lib;
use crate::ast::Node::SqlNode;

#[derive(Clone)]
pub struct BindNode<'a> {
    pub name: &'a str,
    pub value: &'a str,

    pub engine: ExpressionEngineProxy<'a, lib::RustExpressionEngine::node::Node, Value>,
}

impl<'a> SqlNode for BindNode<'a> {
    fn eval(&mut self, env: &mut Value) -> Result<String,String> {
        let r= self.engine.LexerAndEval(self.value,env);
        env[self.name]=r.unwrap_or(Value::Null);
        return Result::Ok("".to_string());
    }
}
