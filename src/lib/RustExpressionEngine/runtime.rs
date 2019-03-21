use crate::lib::RustExpressionEngine::parser::OptMap;
use std::collections::linked_list::LinkedList;
use std::collections::HashMap;

pub fn IsNumber(arg: &String) -> bool{
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
            return true;
        }
    }
    return false;
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


pub struct OptMap<'a> {
    //列表
    pub List: Vec<&'a str>,
    //全部操作符
    pub Map: HashMap<&'a str, bool>,
    //复合操作符
    pub MulOpsMap: HashMap<&'a str, bool>,
    //单操作符
    pub SingleOptMap: HashMap<&'a str, bool>,

    pub priorityArray:Vec<&'a str>,
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


        let  mut vecs=vec![];
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
            priorityArray:vecs,
        }
    }

    //乘除优先于加减 计算优于比较,
    pub fn priorityArray(&self)->Vec<&str>{
        return self.priorityArray.clone();
    }

    pub fn isOpt(&self, arg: String) -> bool {
        let opt = self.Map.get(arg.as_str());
        return opt.unwrap_or(&false).clone();
    }
}