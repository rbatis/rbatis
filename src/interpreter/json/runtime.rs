use std::collections::HashMap;
use std::collections::linked_list::LinkedList;
use std::sync::RwLock;

use serde_json::Value;

use crate::interpreter::json::node::Node;
use crate::interpreter::json::parser::parse;

/// the express engine for  exe code on runtime
#[derive(Debug)]
pub struct RbatisEngine {
    pub expr_cache: RwLock<HashMap<String, Node>>,
    pub opt_map: OptMap<'static>,
}

impl RbatisEngine {
    pub fn new() -> Self {
        return Self {
            expr_cache: Default::default(),
            opt_map: OptMap::new(),
        };
    }

    ///eval express with arg value,if cache have value it will no run parser expr.
    pub fn eval(&self, expr: &str, arg: &Value) -> Result<Value, crate::core::Error> {
        let cached = self.cache_read(expr);
        if cached.is_none() {
            let nodes = parse(expr, &self.opt_map);
            if nodes.is_err() {
                return Result::Err(nodes.err().unwrap());
            }
            let node = nodes.unwrap();
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
        let nodes = parse(lexer_arg, &self.opt_map);
        if nodes.is_err() {
            return Result::Err(nodes.err().unwrap());
        }
        let node = nodes.unwrap();
        return node.eval(arg);
    }
}

#[derive(Clone, Debug)]
pub struct OptMap<'a> {
    pub all_opt: HashMap<&'a str, bool>,
    pub group_opt_map: HashMap<&'a str, bool>,
    pub single_opt_map: HashMap<&'a str, bool>,
    pub allow_opt_sorted: Vec<&'a str>,
}

impl<'a> OptMap<'a> {
    pub fn new() -> Self {
        let mut all = HashMap::new();
        let mut mul_ops_map = HashMap::new();
        let mut single_opt_map = HashMap::new();

        //all opt
        let list = vec![
            "(", ")",
            "%", "^", "*", "**", "/", "+", "-",
            "@", "#", "$", "=", "!", ">", "<", "&", "|",
            "==", "!=", ">=", "<=", "&&", "||"
        ];

        //all opt map
        for item in &list {
            all.insert(item.to_owned(), true);
        }
        //single opt and mul opt
        for item in &list {
            if item.len() > 1 {
                mul_ops_map.insert(item.to_owned(), true);
            } else {
                single_opt_map.insert(item.to_owned(), true);
            }
        }

        Self {
            all_opt: all,
            group_opt_map: mul_ops_map,
            single_opt_map,
            allow_opt_sorted: vec!["%", "^", "*", "**", "/", "+", "-", "<=", "<", ">=", ">", "!=", "==", "&&", "||"],
        }
    }

    ///The or operation in the nonoperational > arithmetic operator > relational operator > logical operator and operation > logical operator
    pub fn priority_array(&self) -> &Vec<&str> {
        return &self.allow_opt_sorted;
    }

    pub fn is_opt(&self, arg: &str) -> bool {
        let opt = self.all_opt.get(arg);
        return opt.is_none() == false;
    }

    pub fn is_allow_opt(&self, arg: &str) -> bool {
        for item in &self.allow_opt_sorted {
            if arg == *item {
                return true;
            }
        }
        return false;
    }
}