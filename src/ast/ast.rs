use serde_json::Value;
use crate::ast::config_holder::ConfigHolder;

/**
* Abstract syntax tree node
*/
pub trait Ast {
    /**
    env: &mut Value,因为bind node 会绑定变量，env必须为可修改的值
    */
    fn eval(&self, env: &mut Value, arg_array:&mut Vec<Value>,holder:&mut ConfigHolder) -> Result<String, String>;
}