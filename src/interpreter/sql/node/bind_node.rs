use serde_json::{json, Value};

use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;
use crate::interpreter::expr;
use crate::interpreter::expr::runtime::ExprRuntime;
use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node_type::NodeType;

#[derive(Clone, Debug)]
pub struct BindNode {
    pub name: String,
    pub value: String,
}

impl BindNode {
    pub fn from(source: &str, express: &str, childs: Vec<NodeType>) -> Result<Self, crate::core::Error> {
        let express = express["bind ".len()..].trim();
        let name_value: Vec<&str> = express.split("=").collect();
        if name_value.len() != 2 {
            return Err(crate::core::Error::from("[rbatis] parser bind express fail:".to_string() + source));
        }
        return Ok(BindNode {
            name: name_value[0].to_owned(),
            value: name_value[1].to_owned(),
        });
    }
}

impl RbatisAST for BindNode {
    fn name() -> &'static str {
        "bind"
    }
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &ExprRuntime, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        let r = engine.eval(self.value.as_str(), env)?;
        env[self.name.as_str()] = r;
        return Result::Ok("".to_string());
    }
}


#[test]
fn test_bind_node() {
    let mut engine = ExprRuntime::new();
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