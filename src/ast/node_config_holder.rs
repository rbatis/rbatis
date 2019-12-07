use std::rc::Rc;
use crate::ast::convert::sql_arg_type_convert::SqlArgTypeConvert;
use crate::engine;
use serde_json::Value;

use crate::ast::convert::sql_arg_type_convert_default::SqlArgTypeConvertDefault;
use engine::node::Node;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone)]
pub struct NodeConfigHolder{
    pub sql_convert: Rc<SqlArgTypeConvert>,
    pub engine: RbatisEngine,
}

impl NodeConfigHolder{
    pub fn new() -> Self{
        let engine= RbatisEngine::new();
        let convert=Rc::new(SqlArgTypeConvertDefault::new());

        return NodeConfigHolder{
            sql_convert:convert,
            engine:engine,
        }
    }
}