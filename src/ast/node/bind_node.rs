use serde_json::{json, Value};

use crate::core::convert::StmtConvert;

use crate::ast::ast::RbatisAST;
use crate::engine;
use crate::engine::runtime::RbatisEngine;
use crate::core::db::DriverType;

#[derive(Clone, Debug)]
pub struct BindNode {
    pub name: String,
    pub value: String,
}

impl RbatisAST for BindNode {
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        let r = engine.eval(self.value.as_str(), env)?;
        env[self.name.as_str()] = r;
        return Result::Ok("".to_string());
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