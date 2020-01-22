use serde_json::{json, Value};

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::xml::node_type::NodeType;

#[derive(Clone,Debug)]
pub struct WhereNode {
    pub childs: Vec<NodeType>,
}

impl Ast for WhereNode {
    fn eval(&self, env: &mut Value, holder: &mut ConfigHolder,arg_array:&mut Vec<Value>) -> Result<String, String> {
        let result = do_child_nodes(&self.childs, env,holder,arg_array);
        if result.is_ok() {
            let r = result.unwrap();
            let s = r.trim();
            if s.is_empty() {
                return Result::Ok(" ".to_string());
            } else {
                return Result::Ok(" where ".to_string() + s.trim_start_matches("and "));
            }
        } else {
            return Result::Err(result.err().unwrap());
        }
    }
}

impl SqlNodePrint for WhereNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<where";
        result = result + ">";
        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</where>";
        return result;
    }
}