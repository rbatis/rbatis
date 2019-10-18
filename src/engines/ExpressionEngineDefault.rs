//use engines::ExpressionEngine::ExpressionEngine;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use crate::engines::ExpressionEngine::ExpressionEngine;
use crate::lib::RustExpressionEngine::runtime::OptMap;
use crate::lib::RustExpressionEngine;
use serde_json::Value;
use crate::lib::RustExpressionEngine::node::Node;
use serde_json::json;
use std::ops::Index;

pub struct ExpressionEngineDefault<'a> {
    optMap: OptMap<'a>,
}

impl <'a>ExpressionEngine<Node, Value> for ExpressionEngineDefault<'a> {
    fn Name(&self) -> String {
        return String::from("ExpressionEngineDefault");
    }

    fn Lexer(&self, lexerArg: String) -> Result<Node,String> {
        let mut newLexerArg=lexerArg;
        if newLexerArg.find(" and ").is_some(){
            newLexerArg=newLexerArg.replace(" and "," && ");
        }
        return RustExpressionEngine::parser::Parser(newLexerArg, &self.optMap);
    }

    fn Eval(&self, lexerResult: &Node, env: &Value) -> Result<Value, String> {
        return lexerResult.eval(env);
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

    let node=engine.Lexer("1 + 1".to_string());

    let result=node.unwrap().eval(&json!(1));
    println!("result={}",result.unwrap());
}