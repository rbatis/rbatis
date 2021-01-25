use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node::do_child_nodes;
use crate::interpreter::sql::node::node_type::NodeType;
use crate::interpreter::sql::node::string_node::StringNode;
use rexpr::ast::Node;
use rexpr::runtime::RExprRuntime;
use serde_json::ser::State::Rest;
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub test: String,
    pub test_func: Node,
}

impl IfNode {
    pub fn from(
        runtime: &RExprRuntime,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::core::Error> {
        let express = express[Self::name().len()..].trim();
        return Ok(IfNode {
            childs: childs,
            test: express.to_string(),
            test_func: runtime.parse(express)?,
        });
    }
}

impl RbatisAST for IfNode {
    fn name() -> &'static str {
        "if"
    }
    fn eval(
        &self,
        convert: &dyn crate::interpreter::sql::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::core::Error> {
        let result = self.test_func.eval(env)?;
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
