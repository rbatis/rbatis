use std::rc::Rc;
use crate::engine;
use serde_json::Value;

use engine::node::Node;
use crate::engine::runtime::RbatisEngine;
use crate::convert::sql_arg_type_convert::SqlArgTypeConvert;
use crate::convert::sql_arg_type_convert_default::SqlArgTypeConvertDefault;

#[derive(Clone)]
pub struct ConfigHolder {
    pub sql_convert: Rc<dyn SqlArgTypeConvert>,
    pub engine: RbatisEngine,
}

impl ConfigHolder {
    pub fn new() -> Self{
        let engine= RbatisEngine::new();
        let convert=Rc::new(SqlArgTypeConvertDefault::new());

        return ConfigHolder {
            sql_convert:convert,
            engine:engine,
        }
    }
}