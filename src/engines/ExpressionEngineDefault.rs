//use engines::ExpressionEngine::ExpressionEngine;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use crate::engines::ExpressionEngine::ExpressionEngine;

pub struct ExpressionEngineDefault<R> {
    result: R,
}

impl<R> ExpressionEngine<String, R> for ExpressionEngineDefault<R> {
    fn Name(&self) -> String {
        return String::from("ExpressionEngineDefault");
    }

    fn Lexer(&self, lexerArg: String) -> (String, String) {
        unimplemented!()
    }

    fn Eval(&self, lexerResult: String, arg: HashMap<&str, &str, RandomState>) -> (R, String) {
        unimplemented!()
    }

    fn LexerAndEval(&self, lexerArg: String, arg: HashMap<&str, &str, RandomState>) -> (R, String) {
        unimplemented!()
    }
}








#[test]
fn TestExpressionEngineDefault() {
    let engine = ExpressionEngineDefault {
        result: false,
    };
    println!("{}", engine.Name())
}