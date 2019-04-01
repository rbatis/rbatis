use std::collections::HashMap;
use crate::lib::RustExpressionEngine::node::Node;
use crate::ast::StringNode::StringNode;
use crate::ast::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use std::rc::Rc;
use serde_json::Value;

pub struct ExpressionEngineCache<'a,T> {
    cache: HashMap<&'a str, T>,
}

impl<'a,T> ExpressionEngineCache<'a,T> {
    pub fn new() -> Self {
        return Self {
            cache: HashMap::new(),
        };
    }

    pub fn put(&mut self, k: &'a str, v: T) {
        &self.cache.insert(k, v);
    }

    pub fn get(&mut self, k: &str) -> Option<&T> {
        return self.cache.get(k);
    }
}

#[test]
fn TestCache() {
    let mut cache = ExpressionEngineCache::new();
    let v;
    {
        cache.put("sadf", Node::newString("asdf"));
        v = cache.get("sadf");
    }
    println!("{}", v.unwrap().eval(&Value::Null));
}