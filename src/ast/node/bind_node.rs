use serde_json::{json, Value};

use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;

use crate::ast::ast::RbatisAST;
use crate::ast::node::node::{create_deep, SqlNodePrint};
use crate::engine;
use crate::engine::runtime::RbatisEngine;

const TEMPLETE_BIND: &'static str = "<bind #{attr}>#{body}</bind>";

#[derive(Clone, Debug)]
pub struct BindNode {
    pub name: String,
    pub value: String,
}

impl RbatisAST for BindNode {
    fn eval(&self, convert: &impl StmtConvert, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        let r = engine.eval(self.value.as_str(), env)?;
        env[self.name.as_str()] = r;
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
    let mut engine = RbatisEngine::new();
    let bind_node = BindNode {
        name: "a".to_string(),
        value: "a+1".to_string(),
    };

    let mut john = json!({
        "a": 1,
    });

    let mut arg_array = vec![];

    let r = bind_node.eval(&DriverType::Mysql, &mut john, &mut engine, &mut arg_array).unwrap();


    println!("r={}", r);
    println!("john[a]={}", john["a"]);
}