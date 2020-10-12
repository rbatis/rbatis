use std::collections::HashMap;
use std::collections::linked_list::LinkedList;
use std::sync::RwLock;

use serde_json::Value;

use crate::engine::node::Node;
use crate::engine::parser::parse;

lazy_static! {
   /// for engine: if cache not have expr value,it will be redo parser code.not wait cache return for no blocking
   /// global expr cache,use RwLock but not blocking
   static ref  EXPR_CACHE: RwLock<HashMap<String, Node>> = RwLock::new(HashMap::new());
}

/// the express engine for  exe code on runtime
#[derive(Clone, Debug)]
pub struct RbatisEngine {
    pub opt_map: OptMap<'static>,
}

impl RbatisEngine {
    pub fn new() -> Self {
        return Self {
            opt_map: OptMap::new(),
        };
    }

    ///eval express with arg value,if cache have value it will no run parser expr.
    pub fn eval(&self, expr: &str, arg: &Value) -> Result<Value, rbatis_core::Error> {
        let mut lexer_arg = expr.to_string();
        if expr.find(" and ").is_some() {
            lexer_arg = lexer_arg.replace(" and ", " && ");
        }
        let cached = self.cache_read(lexer_arg.as_str());
        if cached.is_none() {
            let nodes = parse(lexer_arg.as_str(), &self.opt_map);
            if nodes.is_err() {
                return Result::Err(nodes.err().unwrap());
            }
            let node = nodes.unwrap();
            self.cache_insert(lexer_arg.to_string(), node.clone());
            return node.eval(arg);
        } else {
            let nodes = cached.unwrap();
            return nodes.eval(arg);
        }
    }

    /// read from cache,if not exist return null
    fn cache_read(&self, arg: &str) -> Option<Node> {
        let cache_read = EXPR_CACHE.try_read();
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
    fn cache_insert(&self, key: String, node: Node) -> Result<(), rbatis_core::Error> {
        let cache_write = EXPR_CACHE.try_write();
        if cache_write.is_err() {
            return Err(rbatis_core::Error::from(cache_write.err().unwrap().to_string()));
        }
        let mut cache_write = cache_write.unwrap();
        cache_write.insert(key, node);
        return Ok(());
    }

    /// no cache mode to run engine
    pub fn eval_no_cache(&self, lexer_arg: &str, arg: &Value) -> Result<Value, rbatis_core::Error> {
        let nodes = parse(lexer_arg, &self.opt_map);
        if nodes.is_err() {
            return Result::Err(nodes.err().unwrap());
        }
        let node = nodes.unwrap();
        return node.eval(arg);
    }
}


pub fn is_number(arg: &String) -> bool {
    let chars = arg.chars();
    for item in chars {
        if item == '-' ||
            item == '.' ||
            item == '0' ||
            item == '1' ||
            item == '2' ||
            item == '3' ||
            item == '4' ||
            item == '5' ||
            item == '6' ||
            item == '7' ||
            item == '8' ||
            item == '9'
        {
            // nothing do
        } else {
            return false;
        }
    }
    return true;
}


///将原始字符串解析为 去除空格的token数组
pub fn parser_tokens(s: &String, opt_map: &OptMap) -> Vec<String> {
    let chars = s.chars();
    let chars_len = s.len() as i32;
    let mut result = LinkedList::new();
    //str
    let mut find_str = false;
    let mut temp_str = String::new();

    //opt
    let mut temp_arg = String::new();
    let mut index: i32 = -1;
    for item in chars {
        index = index + 1;
        let is_opt = opt_map.is_opt(item.to_string().as_str());
        if item == '\'' || item == '`' {
            if find_str {
                //第二次找到
                find_str = false;
                temp_str.push(item);
                trim_push_back(&temp_str, &mut result);
                temp_str.clear();
                continue;
            }
            find_str = true;
            temp_str.push(item);
            continue;
        }
        if find_str {
            temp_str.push(item);
            continue;
        }
        if item != '`' && item != '\'' && is_opt == false && !find_str {
            //need reset
            temp_arg.push(item);
            if (index + 1) == chars_len {
                trim_push_back(&temp_arg, &mut result);
            }
        } else {
            trim_push_back(&temp_arg, &mut result);
            temp_arg.clear();
        }
        //opt node
        if is_opt {
            //println!("is opt:{}", item);
            if item.eq(&'-') && (result.len() == 0 || opt_map.is_opt(result.back().unwrap_or(&"".to_string()))) {
                trim_push_back("0", &mut result);
            }
            if item.eq(&'+') && (result.len() == 0 || opt_map.is_opt(result.back().unwrap_or(&"".to_string()))) {
                trim_push_back("0", &mut result);
            }
            if result.len() > 0 {
                let def = String::new();
                let back = result.back().unwrap_or(&def).clone();
                if back != "" && opt_map.is_opt(back.as_str()) {
                    result.pop_back();
                    let mut new_item = back.clone().to_string();
                    new_item.push(item);
                    trim_push_back(&new_item, &mut result);
                    continue;
                }
            }
            trim_push_back(&item.to_string(), &mut result);
            continue;
        }
    }
    let mut v = vec![];
    for item in result {
        v.push(item);
    }
    return v;
}

fn trim_push_back(arg: &str, list: &mut LinkedList<String>) {
    let trim_str = arg.trim().to_string();
    if trim_str.is_empty() {
        return;
    }
    list.push_back(trim_str);
}

#[derive(Clone, Debug)]
pub struct OptMap<'a> {
    //列表
    pub list: Vec<&'a str>,
    //全部操作符
    pub map: HashMap<&'a str, bool>,
    //复合操作符
    pub mul_ops_map: HashMap<&'a str, bool>,
    //单操作符
    pub single_opt_map: HashMap<&'a str, bool>,

    pub allow_priority_array: Vec<&'a str>,
}

impl<'a> OptMap<'a> {
    pub fn new() -> Self {
        let mut list = Vec::new();
        let mut def_map = HashMap::new();
        let mut mul_ops_map = HashMap::new();
        let mut single_opt_map = HashMap::new();

        //list 顺序加入操作符
        list.push("*");
        list.push("/");
        list.push("%");
        list.push("^");
        list.push("+");
        list.push("-");

        list.push("(");
        list.push(")");
        list.push("@");
        list.push("#");
        list.push("$");
        list.push("&");
        list.push("|");
        list.push("=");
        list.push("!");
        list.push(">");
        list.push("<");

        list.push("&&");
        list.push("||");
        list.push("==");
        list.push("!=");
        list.push(">=");
        list.push("<=");


        //全部加入map集合
        for item in &mut list {
            def_map.insert(*item, true);
        }
        //加入单操作符和多操作符
        for item in &mut list {
            if item.len() > 1 {
                mul_ops_map.insert(item.clone(), true);
            } else {
                single_opt_map.insert(item.clone(), true);
            }
        }


        let mut vecs = vec![];
        vecs.push("*");
        vecs.push("/");
        vecs.push("+");
        vecs.push("-");
        vecs.push("<=");
        vecs.push("<");
        vecs.push(">=");
        vecs.push(">");
        vecs.push("!=");
        vecs.push("==");
        vecs.push("&&");
        vecs.push("||");


        Self {
            list: list,
            map: def_map,
            mul_ops_map: mul_ops_map,
            single_opt_map: single_opt_map,
            allow_priority_array: vecs,
        }
    }

    //乘除优先于加减 计算优于比较,
    pub fn priority_array(&self) -> Vec<&str> {
        return self.allow_priority_array.clone();
    }

    //是否是操作符
    pub fn is_opt(&self, arg: &str) -> bool {
        let opt = self.map.get(arg);
        return opt.is_none() == false;
    }

    //是否为有效的操作符
    pub fn is_allow_opt(&self, arg: &str) -> bool {
        for item in &self.allow_priority_array {
            if arg == *item {
                return true;
            }
        }
        return false;
    }
}