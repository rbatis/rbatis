//use engines::ExpressionEngine::ExpressionEngine;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use crate::engines::ExpressionEngine::ExpressionEngine;
use crate::lib::RustExpressionEngine::runtime::OptMap;
use crate::lib::RustExpressionEngine;
use serde_json::Value;
use crate::lib::RustExpressionEngine::node::Node;
use serde_json::json;

pub struct ExpressionEngineDefault<'a> {
    optMap: OptMap<'a>,
}

impl <'a>ExpressionEngine<Node, Value> for ExpressionEngineDefault<'a> {
    fn Name(&self) -> String {
        return String::from("ExpressionEngineDefault");
    }

    fn Lexer(&self, lexerArg: String) -> (Node, String) {
        return RustExpressionEngine::parser::Parser(lexerArg, &self.optMap);
    }

    fn Eval(&self, lexerResult: &Node, env: &Value) -> (Value, String) {
        return (lexerResult.eval(env), String::new());
    }
}

impl<'a> ExpressionEngineDefault<'a> {
    pub fn new() -> Self {
        Self {
            optMap: OptMap::new(),
        }
    }
}


#[test]
fn TestExpressionEngineDefault() {
    let engine = ExpressionEngineDefault::new();
    println!("engine={}", engine.Name());

    let (node,_)=engine.Lexer("1 + 1".to_string());

    println!("result={}",node.eval(&json!(1)))
}