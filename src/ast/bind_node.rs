use serde_json::Value;
use std::rc::Rc;
use crate::engines;
use crate::ast::node::{SqlNode, create_deep};
use crate::ast::node_config_holder::NodeConfigHolder;

#[derive(Clone)]
pub struct BindNode {
    pub name: String,
    pub value: String,
}

impl SqlNode for BindNode {
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        let r = holder.engine.eval(self.value.as_str(), env);
        env[self.name.as_str()] = r.unwrap_or(Value::Null);
        return Result::Ok("".to_string());
    }

    fn print(&self,deep:i32) -> String {
        return create_deep(deep)+"<bind "+self.name.as_str()+"=\""+self.value.as_str()+"\" >"+create_deep(deep).as_str()+"</bind>";
    }
}
