use crate::engines::ExpressionEngine::ExpressionEngine;
use serde_json::Value;
use crate::engines::ExpressionEngineCache::ExpressionEngineCache;
use std::rc::Rc;
use crate::engines::ExpressionEngineDefault::ExpressionEngineDefault;

#[derive(Clone)]
pub struct ExpressionEngineProxy<T, R> {
    expressionEngine: Rc<ExpressionEngine<T, R>>,
    cache: ExpressionEngineCache<T>,
}

impl<'a, T: Clone, R: Clone> ExpressionEngine<T, R> for ExpressionEngineProxy<T, R> {
    fn Name(&self) -> String {
        return self.expressionEngine.Name();
    }

    fn Lexer(&self, lexerArg: String) -> Result<T, String> {
        return self.expressionEngine.Lexer(lexerArg);
    }

    fn Eval(&self, lexerResult: &T, arg: &Value) -> Result<R, String> {
        return self.expressionEngine.Eval(lexerResult, arg);
    }
}

impl<T: Clone, R: Clone> ExpressionEngineProxy<T, R> {
    pub fn new(expressionEngine: Rc<ExpressionEngine<T, R>>, expressionEngineCache: ExpressionEngineCache<T>) -> Self {
        Self {
            expressionEngine: expressionEngine,
            cache: expressionEngineCache,
        }
    }

    pub fn LexerAndEval(&mut self, lexerArg: &str, arg: &Value) -> Result<R, String> {
        let cached = self.cache.get(lexerArg);
        if cached.is_none() {
            let nodes = self.Lexer(lexerArg.to_string()).unwrap();
            self.cache.put(lexerArg.to_string(), nodes.clone());
            return self.Eval(&nodes, arg);
        } else {
            let c = cached.unwrap().clone();
            return self.Eval(&c, arg);
        }
    }
}