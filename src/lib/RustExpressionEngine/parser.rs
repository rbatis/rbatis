use std::collections::HashMap;
use core::borrow::Borrow;
use crate::lib::RustExpressionEngine::node::{Node, NullNode, OptNode, BoolNode, StringNode, NumberNode, ArgNode};
use crate::lib::RustExpressionEngine::node::NodeType::NOpt;
use crate::lib::RustExpressionEngine::runtime::IsNumber;

pub struct OptMap<'a> {
    //列表
    pub List: Vec<&'a str>,
    //全部操作符
    pub Map: HashMap<&'a str, bool>,
    //复合操作符
    pub MulOpsMap: HashMap<&'a str, bool>,
    //单操作符
    pub SingleOptMap: HashMap<&'a str, bool>,
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

        Self {
            List: list,
            Map: defMap,
            MulOpsMap: MulOpsMap,
            SingleOptMap: SingleOptMap,
        }
    }
}

pub fn Parser(data: String) -> (Box<Node>,String) {
   let tokens= ParserOperators(&data);

   return (Box::new(NullNode::new()),"".to_string())
}

pub fn ParserOperators(data:&String)->Vec<String>{
    let optMap = OptMap::new();
    let mut dataString = &mut data.clone();
    parseSingle(dataString, &optMap);
    parseMul(dataString, &optMap);
    let splis: Vec<&str> = dataString.split(" ").collect();
    let mut result = vec![];
    for item in splis {
        if item == " " || item == "" {
            continue;
        }
        result.push(item.to_string());
    }
    return result;
}


//express:表达式，v:操作符
fn parserNode(express: &String, v: &'static String) -> (Box<Node>, String) {
    if v == "" {
        let nullNode = NullNode::new();
        return (Box::new(nullNode), "".to_string());
    }
    //TODO NotSupportOptMap[v]
    //opt node
    if isOperatorsAction(v) {
        let optNode = OptNode::new(v.clone());
        return (Box::new(optNode), "".to_string());
    }
    if v == "true" || v == "false" {
        let boolNode = BoolNode::new(v.to_string());
        return (Box::new(boolNode), "".to_string());
    }
    let firstIndex = v.find("'").unwrap_or_default();
    let lastIndex = v.rfind("'").unwrap_or_default();

    if firstIndex == 0 && lastIndex == (v.len() - 1) && firstIndex != lastIndex {
        let strNode = StringNode::new(v.to_string());
        return (Box::new(strNode), "".to_string());
    }
    if IsNumber(v) {
        let numberNode = NumberNode::new(v);
        return (Box::new(numberNode), "".to_string());
    } else {
        let argNode = ArgNode::new(v);
        return (Box::new(argNode), "".to_string());
    }
    return (Box::new(NullNode::new()), "".to_string());
}

fn isOperatorsAction(s: &String) -> bool {
    let arg = s.as_str();
//计算操作符
    if arg == "+" ||
        arg == "-" ||
        arg == "*" ||
        arg == "/" ||
        //比较操作符
        arg == "&&" ||
        arg == "||" ||
        arg == "==" ||
        arg == "!=" ||
        arg == "<" ||
        arg == "<=" ||
        arg == ">" ||
        arg == ">=" {
        return true;
    }
    return false;
}


//处理单个操作符
fn parseSingle(dataString: &mut String, optMap: &OptMap) {
    for (k, _) in &optMap.SingleOptMap {
        let mut newStr = String::from(" ");
        &newStr.push_str(k);
        &newStr.push_str(" ");
        let newDataStr = dataString.replace(k, &newStr);
        *dataString = newDataStr;
    }
}

//处理多个操作符
fn parseMul(dataString: &mut String, optMap: &OptMap) {
    for (k, _) in &optMap.MulOpsMap {
        let mut newStr = String::from(" ");

        let mut s = &mut k.clone().to_string();
        parseSingle(s, optMap);
        *s = s.trim().to_string();

        newStr.push_str(s.as_str());
        newStr.push_str(" ");


        let mut to = String::from(" ");
        to.push_str(&k);
        to.push_str(" ");

        let newDataStr = dataString.replace(newStr.as_str(), to.as_str());
        *dataString = newDataStr;
    }
}

