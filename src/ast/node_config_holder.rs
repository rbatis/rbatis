use std::rc::Rc;
use crate::ast::convert::sql_arg_type_convert::SqlArgTypeConvert;
use crate::engines;
use serde_json::Value;

use crate::ast::convert::sql_arg_type_convert_default::SqlArgTypeConvertDefault;
use engines::rbatis_engine::node::Node;
use crate::engines::rbatis_engine::runtime::RbatisEngine;

#[derive(Clone)]
pub struct NodeConfigHolder{
    pub sqlConvert: Rc<SqlArgTypeConvert>,
    pub engine: RbatisEngine,
}

impl NodeConfigHolder{
    pub fn new() -> Self{
        let engine= RbatisEngine::new();
        let convert=Rc::new(SqlArgTypeConvertDefault::new());

        return NodeConfigHolder{
            sqlConvert:convert,
            engine:engine,
        }
    }
}