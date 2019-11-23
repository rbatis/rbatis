use std::rc::Rc;
use crate::ast::convert::SqlArgTypeConvert::SqlArgTypeConvert;
use crate::engines;
use serde_json::Value;

use crate::ast::convert::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use engines::RustExpressionEngine::node::Node;
use crate::engines::RustExpressionEngine::runtime::ExEngine;

#[derive(Clone)]
pub struct NodeConfigHolder{
    pub sqlConvert: Rc<SqlArgTypeConvert>,
    pub engine: ExEngine,
}

impl NodeConfigHolder{
    pub fn new() -> Self{
        let engine=ExEngine::new();
        let convert=Rc::new(SqlArgTypeConvertDefault::new());

        return NodeConfigHolder{
            sqlConvert:convert,
            engine:engine,
        }
    }
}