use serde_json::{json, Value};

use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node::do_child_nodes;
use crate::interpreter::sql::node::node_type::NodeType;
use crate::interpreter::sql::node::string_node::StringNode;
use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;
use crate::interpreter::expr::runtime::ExprRuntime;

#[derive(Clone, Debug)]
pub struct TrimNode {
    pub childs: Vec<NodeType>,
    pub prefix: String,
    pub suffix: String,
    pub suffix_overrides: String,
    pub prefix_overrides: String,
}

impl TrimNode {
    pub fn from(source: &str, express: &str, childs: Vec<NodeType>) -> Result<Self, crate::core::Error> {
        let express = express["trim ".len()..].trim();
        if express.starts_with("'") && express.ends_with("'") {
            let express = express[1..express.len() - 1].trim();
            return Ok(TrimNode {
                childs: childs,
                prefix: "".to_string(),
                suffix: "".to_string(),
                suffix_overrides: express.to_string(),
                prefix_overrides: express.to_string(),
            });
        } else {
            return Err(crate::core::Error::from(format!("[rbatis] express trim value must be string value, for example:  trim 'value',error express: {}", source)));
        }
    }
}

impl RbatisAST for TrimNode {
    fn name() -> &'static str {
        "trim"
    }
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &ExprRuntime, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        let result_value = do_child_nodes(convert, &self.childs, env, engine, arg_array)?;
        let mut result = result_value.as_str().trim();
        if !self.prefix_overrides.is_empty() {
            let splits: Vec<&str> = self.prefix_overrides.split("|").collect();
            for item in splits {
                result = result.trim_start_matches(item);
            }
        }
        if !self.suffix_overrides.is_empty() {
            let splits: Vec<&str> = self.suffix_overrides.split("|").collect();
            for item in splits {
                result = result.trim_end_matches(item);
            }
        }

        let mut new_buffer = String::new();
        new_buffer = new_buffer + " " + self.prefix.as_str() + " " + result + " " + self.suffix.as_str();
        return Result::Ok(new_buffer);
    }
}


#[test]
pub fn test_trim_node() {
    let mut engine = ExprRuntime::new();
    let node = TrimNode {
        childs: vec![NodeType::NString(StringNode::new("1trim value1"))],
        prefix: "(".to_string(),
        suffix: ")".to_string(),
        suffix_overrides: "1".to_string(),
        prefix_overrides: "1".to_string(),
    };
    let mut john = json!({
        "arg": 2,
    });
    let mut arg_array = vec![];

    let r = node.eval(&DriverType::Mysql, &mut john, &mut engine, &mut arg_array).unwrap();
    println!("{}", r)
}