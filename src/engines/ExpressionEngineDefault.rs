use crate::engines::ExpressionEngine::ExpressionEngine;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

struct ExpressionEngineDefault {

}

impl <T,R>ExpressionEngine<T,R> for ExpressionEngineDefault{
    fn Name() -> String {
       return  String::from("ExpressionEngineDefault");
    }

    fn Lexer(lexerArg: String) -> (T, String) {
        unimplemented!()
    }

    fn Eval(lexerResult: T, arg: HashMap<&str, &str, RandomState>) -> (R, String) {
        unimplemented!()
    }

    fn LexerAndEval(lexerArg: String, arg: HashMap<&str, &str, RandomState>) -> (R, String) {
        unimplemented!()
    }
}