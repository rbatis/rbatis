use serde_json::Value;
use std::rc::Rc;
use crate::engine;
use crate::ast::node::{SqlNode, create_deep, SqlNodePrint};
use crate::ast::config_holder::ConfigHolder;

#[derive(Clone)]
pub struct BindNode {
    pub name: String,
    pub value: String,
}

impl SqlNode for BindNode {
    fn eval(&self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String, String> {
        let r = holder.engine.eval(self.value.as_str(), env);
        env[self.name.as_str()] = r.unwrap_or(Value::Null);
        return Result::Ok("".to_string());
    }

}

impl SqlNodePrint for BindNode{
    fn print(&self,deep:i32) -> String {
        return create_deep(deep)+"<bind "+self.name.as_str()+"=\""+self.value.as_str()+"\" >"+create_deep(deep).as_str()+"</bind>";
    }
}