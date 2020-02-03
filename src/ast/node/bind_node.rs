use std::rc::Rc;

use serde_json::{json, Value};

use crate::ast::ast::Ast;

use crate::ast::node::node::{create_deep, SqlNodePrint};
use crate::engine;
use crate::engine::runtime::RbatisEngine;

const TEMPLETE_BIND: &'static str = "<bind #{attr}>#{body}</bind>";

#[derive(Clone,Debug)]
pub struct BindNode {
    pub name: String,
    pub value: String,
}

impl Ast for BindNode {
    fn eval(&self, env: &mut Value, holder: &mut RbatisEngine,arg_array:&mut Vec<Value>) -> Result<String, String> {
        let r = holder.eval(self.value.as_str(), env);
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


#[test]
fn test_bind_node() {
    let mut holder = RbatisEngine::new();
    let bind_node = BindNode {
        name: "a".to_string(),
        value: "a+1".to_string(),
    };

    let mut john = json!({
        "a": 1,
    });

    let mut arg_array=vec![];

    let r = bind_node.eval(&mut john,&mut holder, &mut arg_array).unwrap();


    println!("r={}", r);
    println!("john[a]={}", john["a"]);
}