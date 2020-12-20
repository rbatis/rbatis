use std::collections::HashMap;
use std::collections::linked_list::LinkedList;
use std::sync::RwLock;

use serde_json::Value;

use crate::interpreter::expr::ast::Node;
use crate::interpreter::expr::lexer::lexer;
use crate::interpreter::expr::parser::parse;
use crate::interpreter::expr::token::TokenMap;

/// the express engine for  exe code on runtime
#[derive(Debug)]
pub struct ExprRuntime {
    pub expr_cache: RwLock<HashMap<String, Node>>,
    pub token_map: TokenMap<'static>,
}

impl ExprRuntime {
    pub fn new() -> Self {
        return Self {
            expr_cache: Default::default(),
            token_map: TokenMap::new(),
        };
    }

    ///eval express with arg value,if cache have value it will no run lexer expr.
    pub fn eval(&self, expr: &str, arg: &Value) -> Result<Value, crate::core::Error> {
        let cached = self.cache_read(expr);
        if cached.is_none() {
            let tokens = lexer(expr, &self.token_map)?;
            let node = parse(&self.token_map, &tokens, expr)?;
            self.cache_insert(expr.to_string(), node.clone());
            return node.eval(arg);
        } else {
            let nodes = cached.unwrap();
            return nodes.eval(arg);
        }
    }

    /// read from cache,if not exist return null
    fn cache_read(&self, arg: &str) -> Option<Node> {
        let cache_read = self.expr_cache.try_read();
        if cache_read.is_err() {
            return Option::None;
        }
        let cache_read = cache_read.unwrap();
        let r = cache_read.get(arg);
        return if r.is_none() {
            Option::None
        } else {
            r.cloned()
        };
    }

    /// save to cache,if fail nothing to do.
    fn cache_insert(&self, key: String, node: Node) -> Result<(), crate::core::Error> {
        let cache_write = self.expr_cache.try_write();
        if cache_write.is_err() {
            return Err(crate::core::Error::from(cache_write.err().unwrap().to_string()));
        }
        let mut cache_write = cache_write.unwrap();
        cache_write.insert(key, node);
        return Ok(());
    }

    /// no cache mode to run engine
    pub fn eval_no_cache(&self, lexer_arg: &str, arg: &Value) -> Result<Value, crate::core::Error> {
        let tokens = lexer(lexer_arg, &self.token_map)?;
        let node = parse(&self.token_map, &tokens, lexer_arg)?;
        return node.eval(arg);
    }
}

