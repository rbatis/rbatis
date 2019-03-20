use std::collections::HashMap;
use core::borrow::Borrow;
use crate::lib::RustExpressionEngine::node::{Node, NullNode, OptNode, BoolNode, StringNode, NumberNode, ArgNode, BinaryNode};
use crate::lib::RustExpressionEngine::node::NodeType::NOpt;
use crate::lib::RustExpressionEngine::runtime::IsNumber;
use std::collections::linked_list::LinkedList;

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

    pub fn isOpt(&self, arg: String) -> bool {
        let opt = self.Map.get(arg.as_str());
        return opt.unwrap_or(&false).clone();
    }
}

pub fn Parser(data: String, optMap: &OptMap) -> (Box<Node>, String) {
    let tokens = ParserTokens(&data);

    let mut nodes = vec![];
    for item in tokens {
        let (boxNode, err) = parserNode(&data, &item,&optMap);
        if err != "" {
            return (Box::new(NullNode::new()), err);
        }
        nodes.push(boxNode);
    }

    for item in optMap.List.clone() {}

    for item in nodes{
        println!("{}:{}",item.Type(),item.Value());
    }


    return (Box::new(NullNode::new()), "".to_string());
}

/**
 * 将原始字符串解析为 去除空格的token数组
**/
pub fn ParserTokens(s: &String) -> Vec<String> {
    let optMap = OptMap::new();
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
        if item == '\'' || item== '`' {
            if find_str {
                //第二次找到
                find_str = false;
                temp_str.push(item);
                trimPushBack(temp_str.clone(), &mut result);
                temp_str = String::new();
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
        let needReset = item != '`' && item != '\'' && optMap.isOpt(item.to_string()) == false && !find_str;
        if needReset {
            temp_arg.push(item);
            if (index + 1) == charsLen {
                trimPushBack(temp_arg.clone(), &mut result);
            }
        } else {
            trimPushBack(temp_arg, &mut result);
            temp_arg = String::new();
        }
        //opt node
        if optMap.isOpt(item.to_string()) {
            //println!("is opt:{}", item);
            if result.len() > 0 {
                let def = String::new();
                let back = result.back().unwrap_or(&def).clone();
                if back != "" && optMap.isOpt(back.clone()) {
                    result.pop_back();
                    let mut newItem = back.clone().to_string();
                    newItem.push(item);
                    trimPushBack(newItem.clone(), &mut result);
                    continue;
                }
            }
            trimPushBack(item.to_string(), &mut result);
            continue;
        }
    }
    let mut v = vec![];
    for item in result {
        v.push(item);
    }
    return v;
}

fn trimPushBack(arg: String, list: &mut LinkedList<String>) {
    let trimStr = arg.trim().to_string();
    if trimStr == "" {
        return;
    }
    list.push_back(trimStr);
}

//解析表达式生成一个抽象语法节点，express:表达式，v:操作符
fn parserNode(express: &String, v: &String,opt:&OptMap) -> (Box<Node>, String) {
    if v == "" {
        let nullNode = NullNode::new();
        return (Box::new(nullNode), "".to_string());
    }
    //TODO NotSupportOptMap[v]
    //opt node
    if opt.isOpt(v.clone()) {
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

fn findReplaceOpt<T:Node+Sized+Clone>(express: &String, operator: &String, nodeArg: &mut Vec<T>) -> String {
    let mut index = 1 as i32;

    for item in nodeArg.clone(){
        let itemType = item.Type();
        if itemType as i32 == NOpt as i32 {
            let leftIndex = (index - 1) as usize;
            let rightIndex = (index + 1) as usize;
            BinaryNode::new(nodeArg[leftIndex].clone(), nodeArg[rightIndex].clone(), item.Value().as_str().unwrap().to_string());
        }
        index = index + 1;
    }
    return "".to_string();
}