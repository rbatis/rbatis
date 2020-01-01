use serde_json::{json, Value};

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;

//if a != b:

//if a != b:
//
//else:
pub struct IfNode {
    pub express: &'static str,
    pub _if: String,
    pub ifs: Option<Vec<Box<dyn Ast>>>,
    pub elses: Option<Vec<Box<dyn Ast>>>,
}

impl IfNode {
    pub fn new(express: &'static str) -> IfNode {
        // let expr=express.replace("if ","");
        let express_array: Vec<&str> = express.split("\n").collect();
        let mut if_express = "".to_string();

        let mut if_str = "".to_string();
        let mut else_str = "".to_string();

        let mut if_start=false;
        let mut else_start=false;
        for mut x in express_array {
            x = x.trim();
            if if_start && else_start == false {
                if_str += x;
            }
            if if_start == false && else_start {
                else_str += x;
            }
            if x.starts_with("if ") && x.ends_with(":") {
                if_start = true;
                if_express = x["if ".len()..(x.len() - 1)].to_string();
            }
            if x.eq("else:") {
                else_start = true;
            }
        }
        return Self {
            express: express,
            _if:if_express,
            ifs: None,
            elses: None
        };
    }
}

impl Ast for IfNode {
    fn eval(&self, env: &mut Value, holder: &mut ConfigHolder) -> Result<String, String> {
        println!("express:{}",self.express);
        println!("_if:{}",self._if);
        return Result::Ok("".to_string())
    }
}


#[test]
pub fn test_if_node() {
    let if_node = IfNode::new("if a!=b:");
    if_node.eval(&mut json!("1"),&mut ConfigHolder::new());
}