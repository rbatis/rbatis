use serde_json::Value;
use std::rc::Rc;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use crate::lib;
use crate::ast::Node::SqlNode;

pub struct BindNode<'a> {
    pub name: String,
    pub value: String,

    pub engine: Rc<ExpressionEngineProxy<'a, lib::RustExpressionEngine::node::Node, Value>>,
}

impl<'a> SqlNode for BindNode<'a> {
    fn eval(&self, env: &Value) -> String {
        unimplemented!()
    }
}
