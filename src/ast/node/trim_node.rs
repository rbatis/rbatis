use serde_json::{json, Value};

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::node::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::ast::node::string_node::StringNode;

#[derive(Clone,Debug)]
pub struct TrimNode {
    pub childs: Vec<NodeType>,
    pub prefix: String,
    pub suffix: String,
    pub suffix_overrides: String,
    pub prefix_overrides: String,
}

impl Ast for TrimNode {
    fn eval(&self, env: &mut Value, holder: &mut ConfigHolder,arg_array:&mut Vec<Value>) -> Result<String, String> {
        let result_value = do_child_nodes(&self.childs, env, holder,arg_array);
        let is_error = result_value.is_err();
        if is_error {
            return Result::Err(result_value.clone().err().unwrap());
        }
        let result_str = result_value.unwrap();
        let mut result = result_str.as_str().trim();

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

impl SqlNodePrint for TrimNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<trim ";
        result = result + " prefix=\"" + self.prefix.as_str() + "\"";
        result = result + " suffix=\"" + self.suffix.as_str() + "\"";
        result = result + " suffix_overrides=\"" + self.suffix_overrides.as_str() + "\"";
        result = result + " prefix_overrides=\"" + self.prefix_overrides.as_str() + "\"";
        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</trim>";
        return result;
    }
}


#[test]
pub fn test_trim_node() {
    let mut holder = ConfigHolder::new();
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
    let mut arg_array=vec![];

    let r = node.eval(&mut john,&mut holder, &mut arg_array).unwrap();
    println!("{}", r)
}