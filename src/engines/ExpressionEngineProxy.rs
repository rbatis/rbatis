use crate::engines::ExpressionEngine::ExpressionEngine;
use serde_json::Value;
use crate::engines::ExpressionEngineCache::ExpressionEngineCache;
use std::rc::Rc;

pub struct ExpressionEngineProxy<'a, T, R> {
    expressionEngine: Rc<ExpressionEngine<T, R>>,
    cache: ExpressionEngineCache<'a>,
}

impl<'a, T, R> ExpressionEngine<T, R> for ExpressionEngineProxy<'a, T, R> {
    fn Name(&self) -> String {
        return self.expressionEngine.Name();
    }

    fn Lexer(&self, lexerArg: String) -> (T, String) {
        return self.expressionEngine.Lexer(lexerArg);
    }

    fn Eval(&self, lexerResult: &T, arg: &Value) -> (R, String) {
        return self.expressionEngine.Eval(lexerResult, arg);
    }
}

impl<'a, T, R> ExpressionEngineProxy<'a, T, R> {
    pub fn new(expressionEngine: Rc<ExpressionEngine<T, R>>, expressionEngineCache: ExpressionEngineCache<'a>) -> Self {
        Self {
            expressionEngine: expressionEngine,
            cache: expressionEngineCache,
        }
    }

    pub fn LexerAndEval(&mut self, lexerArg: &'a str, arg: &Value) -> (R,String) {
        unimplemented!();
//        let cached = self.cache.get(lexerArg);
//        if cached.is_none() {
//            let (nodes, e) = self.Lexer(lexerArg.to_string());
//            &self.cache.put(lexerArg, nodes);
//            return self.Eval(&nodes,arg);
//        } else {
//            return self.Eval(cached.unwrap(),arg);
//        }
    }
}