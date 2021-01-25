use serde_json::{json, Value};

use crate::core::convert::StmtConvert;

use crate::core::Error;
use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node_type::NodeType;
use rexpr;
use rexpr::ast::Node;
use rexpr::runtime::RExprRuntime;
use crate::core::db::DriverType;

#[derive(Clone, Debug)]
pub struct BindNode {
    pub name: String,
    pub value: String,
    pub func: Node,
}

impl BindNode {
    pub fn def_name() -> &'static str {
        "let"
    }
    pub fn from(
        runtime: &RExprRuntime,
        source: &str,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::core::Error> {
        let source = source.trim();
        if express.starts_with(Self::def_name()) {
            let express = express[Self::def_name().len()..].trim();
            let name_value: Vec<&str> = express.split("=").collect();
            if name_value.len() != 2 {
                return Err(crate::core::Error::from(
                    "[rbatis] parser bind express fail:".to_string() + source,
                ));
            }
            return Ok(BindNode {
                name: name_value[0].to_owned(),
                value: name_value[1].to_owned(),
                func: runtime.parse(name_value[1])?,
            });
        } else if express.starts_with(Self::name()) {
            let express = express[Self::name().len()..].trim();
            let name_value: Vec<&str> = express.split("=").collect();
            if name_value.len() != 2 {
                return Err(crate::core::Error::from(
                    "[rbatis] parser bind express fail:".to_string() + source,
                ));
            }
            return Ok(BindNode {
                name: name_value[0].to_owned(),
                value: name_value[1].to_owned(),
                func: runtime.parse(name_value[1])?,
            });
        } else {
            return Err(Error::from(
                "[rbaits] OtherwiseNode must start with '_:' or 'otherwise:'",
            ));
        }
    }
}

impl RbatisAST for BindNode {
    fn name() -> &'static str {
        "bind"
    }
    fn eval(
        &self,
        convert: &dyn crate::interpreter::sql::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::core::Error> {
        let r = self.func.eval(env)?;
        env[self.name.as_str()] = r;
        return Result::Ok(serde_json::Value::Null);
    }
}

#[test]
fn test_bind_node() {
    let mut engine = RExprRuntime::new();
    let bind_node = BindNode {
        name: "a".to_string(),
        value: "a+1".to_string(),
        func: engine.parse("a+1").unwrap(),
    };

    let mut john = json!({
        "a": 1,
    });

    let mut arg_array = vec![];

    let mut r = "".to_string();
    bind_node
        .eval(
            &DriverType::Mysql,
            &mut john,
            &mut engine,
            &mut arg_array,
            &mut r,
        )
        .unwrap();
    println!("r={}", r);
    println!("john[a]={}", john["a"]);
}
