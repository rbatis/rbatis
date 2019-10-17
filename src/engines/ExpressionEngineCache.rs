use std::collections::HashMap;
use crate::lib::RustExpressionEngine::node::Node;
use crate::ast::StringNode::StringNode;
use crate::ast::convert::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use std::rc::Rc;
use serde_json::Value;

#[derive(Clone, PartialEq)]
pub struct ExpressionEngineCache<T> {
    cache: HashMap<String, T>,
}

impl<T> ExpressionEngineCache<T> {
    pub fn new() -> Self {
        return Self {
            cache: HashMap::new(),
        };
    }

    pub fn put(&mut self, k: String, v: T) {
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
        cache.put("sadf".to_string(), Node::newString("asdf"));
        v = cache.get("sadf");
    }
    println!("{}", v.unwrap().eval(&Value::Null).unwrap());
}