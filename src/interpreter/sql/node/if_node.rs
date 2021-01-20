use serde_json::ser::State::Rest;
use serde_json::{json, Value};

use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;
use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node::do_child_nodes;
use crate::interpreter::sql::node::node_type::NodeType;
use crate::interpreter::sql::node::string_node::StringNode;
use rexpr::runtime::RExprRuntime;

#[derive(Clone, Debug)]
pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl IfNode {
    pub fn from(express: &str, childs: Vec<NodeType>) -> Result<Self, crate::core::Error> {
        let express = express[Self::name().len()..].trim();
        return Ok(IfNode {
            childs: childs,
            test: express.to_string(),
        });
    }
}

impl RbatisAST for IfNode {
    fn name() -> &'static str {
        "if"
    }
    fn eval(
        &self,
        convert: &crate::core::db::DriverType,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::core::Error> {
        let result = engine.eval(self.test.as_str(), env)?;
        if !result.is_boolean() {
            return Result::Err(crate::core::Error::from(
                "[rbatis] express:'".to_owned()
                    + self.test.as_str()
                    + "' is not return bool value!",
            ));
        }
        if result.as_bool().unwrap_or(false) {
            return do_child_nodes(convert, &self.childs, env, engine, arg_array, arg_sql);
        }
        return Result::Ok(serde_json::Value::Null);
    }
}

#[test]
pub fn test_if_node() {
    let node = IfNode {
        childs: vec![NodeType::NString(StringNode::new("yes"))],
        test: "arg == 1".to_string(),
    };
    let mut john = json!({
        "arg": 1,
    });
    let mut engine = RExprRuntime::new();
    let mut arg_array = vec![];
    let mut sql = String::new();
    node.eval(
        &DriverType::Mysql,
        &mut john,
        &mut engine,
        &mut arg_array,
        &mut sql,
    )
    .unwrap();
    println!("{}", sql);
}
