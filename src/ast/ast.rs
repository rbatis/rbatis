use serde_json::Value;
use crate::ast::config_holder::ConfigHolder;

/**
* Abstract syntax tree node
*/
pub trait Ast {
    /**
    env: &mut Value,因为bind node 会绑定变量，env必须为可修改的值
    */
    fn eval(&self, env: &mut Value,holder:&mut ConfigHolder,arg_result:&mut Vec<Value>) -> Result<String, String>;
}