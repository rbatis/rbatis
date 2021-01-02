use std::collections::HashMap;
use std::collections::linked_list::LinkedList;
use std::sync::RwLock;

use serde_json::Value;

use crate::interpreter::expr::ast::Node;
use crate::interpreter::expr::lexer::lexer;
use crate::interpreter::expr::parser::parse;
use crate::interpreter::expr::token::TokenMap;
use dashmap::DashMap;
use dashmap::mapref::one::Ref;

/// the express engine for  exe code on runtime
#[derive(Debug)]
pub struct ExprRuntime {
    pub expr_cache: DashMap<String, Node>,
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
    fn cache_read(&self, arg: &str) -> Option<Ref<String, Node>> {
        let cache_read = self.expr_cache.get(arg);
        if cache_read.is_none() {
            return Option::None;
        }
        let cache_read = cache_read.unwrap();
        return Some(cache_read);
    }

    /// save to cache,if fail nothing to do.
    fn cache_insert(&self, key: String, node: Node) -> Option<Node> {
        let cache_write = self.expr_cache.insert(key, node);
        return cache_write;
    }

    /// no cache mode to run engine
    pub fn eval_no_cache(&self, lexer_arg: &str, arg: &Value) -> Result<Value, crate::core::Error> {
        let tokens = lexer(lexer_arg, &self.token_map)?;
        let node = parse(&self.token_map, &tokens, lexer_arg)?;
        return node.eval(arg);
    }
}


#[cfg(test)]
mod test {
    use crate::interpreter::expr::runtime::ExprRuntime;
    use crate::utils::bencher::QPS;

    //cargo test --release --package rbatis --lib interpreter::expr::runtime::test::test_bench --no-fail-fast -- --exact -Z unstable-options --show-output
    #[test]
    fn test_bench() {
        let runtime = ExprRuntime::new();
        runtime.eval("1+1", &serde_json::Value::Null);
        runtime.eval("1+1", &serde_json::Value::Null);

        let total = 1000000;
        let now = std::time::Instant::now();
        for _ in 0..total {
            //(Windows10 6Core16GBMem) use Time: 84.0079ms ,each:84 ns/op use QPS: 11900823 QPS/s
            let r = runtime.eval("1+1", &serde_json::Value::Null).unwrap();//use Time: 1.5752844s ,each:1575 ns/op use QPS: 634793 QPS/s
            //println!("{}",r);
        }
        now.time(total);
        now.qps(total);
    }
}