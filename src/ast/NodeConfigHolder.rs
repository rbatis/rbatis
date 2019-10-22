use std::rc::Rc;
use crate::ast::convert::SqlArgTypeConvert::SqlArgTypeConvert;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use crate::engines;
use serde_json::Value;
use crate::engines::ExpressionEngineDefault::ExpressionEngineDefault;
use crate::engines::ExpressionEngineCache::ExpressionEngineCache;
use crate::ast::convert::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use engines::RustExpressionEngine::node::Node;

#[derive(Clone)]
pub struct NodeConfigHolder{
    pub sqlConvert: Rc<SqlArgTypeConvert>,
    pub engine: ExpressionEngineProxy<Node, Value>,
}

impl NodeConfigHolder{
    pub fn new() -> Self{
        let engine=ExpressionEngineProxy::new(
            Rc::new(ExpressionEngineDefault::new()),
            ExpressionEngineCache::new());
        let convert=Rc::new(SqlArgTypeConvertDefault::new());

        return NodeConfigHolder{
            sqlConvert:convert,
            engine:engine,
        }
    }
}