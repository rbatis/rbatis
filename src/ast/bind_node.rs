use serde_json::Value;
use std::rc::Rc;
use crate::engine;
use crate::ast::node::{SqlNode, create_deep, SqlNodePrint};
use crate::ast::config_holder::ConfigHolder;


const TEMPLETE_BIND:&'static str ="<bind #{attr}>#{body}</bind>";

#[derive(Clone)]
pub struct BindNode {
    pub name: String,
    pub value: String,
}

impl SqlNode for BindNode {
    fn eval(&self, env: &mut Value, holder: &mut ConfigHolder) -> Result<String, String> {
        let r = holder.engine.eval(self.value.as_str(), env);
        env[self.name.as_str()] = r.unwrap_or(Value::Null);
        return Result::Ok("".to_string());
    }
}

impl SqlNodePrint for BindNode {
    fn print(&self, deep: i32) -> String {
        let mut data = create_deep(deep) + TEMPLETE_BIND.replace("#{attr}", (self.name.clone() + "=\"" + self.value.as_str() + "\"").as_str()).as_str();
        data = data.replace("#{body}", create_deep(deep).as_str());
        return data;
    }
}