use std::collections::linked_list::LinkedList;
use std::collections::HashMap;
use serde_json::Value;
use crate::engines::RustExpressionEngine::parser::Parser;
use crate::engines::RustExpressionEngine::node::Node;

#[derive(Clone)]
pub struct ExEngine {
    pub cache:HashMap<String,Node>,
    pub optMap:OptMap<'static>,
}

impl ExEngine{

    pub fn new()-> Self{
        return Self{
            cache: Default::default(),
            optMap: OptMap::new(),
        }
    }

    pub fn Eval(&mut self,lexerArg: &str, arg: &Value) -> Result<Value, String>{
        let cached = self.cache.get(lexerArg);
        if (&cached).is_none() {
            let nodes = Parser(lexerArg.to_string(),&self.optMap);
            if nodes.is_err(){
                return Result::Err(nodes.err().unwrap());
            }
            let node=nodes.unwrap();
            self.cache.insert(lexerArg.to_string(), node.clone());
            return node.eval(arg);
        } else {
            let nodes = cached.unwrap().clone();
            return nodes.eval(arg);
        }
    }

    pub fn Eval_No_Cache(&self,lexerArg: &str, arg: &Value) -> Result<Value, String>{
            let nodes = Parser(lexerArg.to_string(),&self.optMap);
            if nodes.is_err(){
                return Result::Err(nodes.err().unwrap());
            }
            let node=nodes.unwrap();
            return node.eval(arg);
    }

    pub fn clearCache(&mut self){
        self.cache.clear();
    }

    pub fn removeCache(&mut self,lexerArg: &str){
        self.cache.remove(lexerArg);
    }


}



pub fn IsNumber(arg: &String) -> bool {
    let chars = arg.chars();
    for item in chars {
        if item == '.' ||
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


/**
 * 将原始字符串解析为 去除空格的token数组
**/
pub fn ParserTokens(s: &String, optMap: &OptMap) -> Vec<String> {
    let chars = s.chars();
    let charsLen = s.len() as i32;
    let mut result = LinkedList::new();
    //str
    let mut find_str = false;
    let mut temp_str = String::new();
    //opt
    let mut temp_arg = String::new();
    let mut index: i32 = -1;
    for item in chars {
        index = index + 1;
        let isOpt = optMap.isOpt(item.to_string().as_str());
        if item == '\'' || item == '`' {
            if find_str {
                //第二次找到
                find_str = false;
                temp_str.push(item);
                trimPushBack(&temp_str, &mut result);
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
        if item != '`' && item != '\'' && isOpt == false && !find_str {
            //need reset
            temp_arg.push(item);
            if (index + 1) == charsLen {
                trimPushBack(&temp_arg, &mut result);
            }
        } else {
            trimPushBack(&temp_arg, &mut result);
            temp_arg.clear();
        }
        //opt node
        if isOpt {
            //println!("is opt:{}", item);
            if result.len() > 0 {
                let def = String::new();
                let back = result.back().unwrap_or(&def).clone();
                if back != "" && optMap.isOpt(back.as_str()) {
                    result.pop_back();
                    let mut newItem = back.clone().to_string();
                    newItem.push(item);
                    trimPushBack(&newItem, &mut result);
                    continue;
                }
            }
            trimPushBack(&item.to_string(), &mut result);
            continue;
        }
    }
    let mut v = vec![];
    for item in result {
        v.push(item);
    }
    return v;
}

fn trimPushBack(arg: &String, list: &mut LinkedList<String>) {
    let trimStr = arg.trim().to_string();
    if trimStr.is_empty() {
        return;
    }
    list.push_back(trimStr);
}

#[derive(Clone)]
pub struct OptMap<'a> {
    //列表
    pub List: Vec<&'a str>,
    //全部操作符
    pub Map: HashMap<&'a str, bool>,
    //复合操作符
    pub MulOpsMap: HashMap<&'a str, bool>,
    //单操作符
    pub SingleOptMap: HashMap<&'a str, bool>,

    pub allowPriorityArray: Vec<&'a str>,
}

impl<'a> OptMap<'a> {
    pub fn new() -> Self {
        let mut list = Vec::new();
        let mut defMap = HashMap::new();
        let mut MulOpsMap = HashMap::new();
        let mut SingleOptMap = HashMap::new();

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
            defMap.insert(*item, true);
        }
        //加入单操作符和多操作符
        for item in &mut list {
            if item.len() > 1 {
                MulOpsMap.insert(item.clone(), true);
            } else {
                SingleOptMap.insert(item.clone(), true);
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
            List: list,
            Map: defMap,
            MulOpsMap: MulOpsMap,
            SingleOptMap: SingleOptMap,
            allowPriorityArray: vecs,
        }
    }

    //乘除优先于加减 计算优于比较,
    pub fn priorityArray(&self) -> Vec<&str> {
        return self.allowPriorityArray.clone();
    }

    //是否是操作符
    pub fn isOpt(&self, arg: &str) -> bool {
        let opt = self.Map.get(arg);
        return opt.is_none() == false;
    }

    //是否为有效的操作符
    pub fn isAllowOpt(&self, arg: &str) -> bool {
        for item in &self.allowPriorityArray{
            if arg==*item{
                return true;
            }
        }
        return false;
    }
}