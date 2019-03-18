use std::collections::HashMap;
use core::borrow::Borrow;

pub struct OptMap<'a> {
    pub Map: HashMap<&'a str, bool>,
    //全部操作符
    pub MulOpsMap: HashMap<&'a str, bool>,
    //复合操作符
    pub SingleOptMap: HashMap<&'a str, bool>, //单操作符
}

impl<'a> OptMap<'a> {
    pub fn new() -> Self {
        let mut defMap = HashMap::new();
        defMap.insert("+", true);
        defMap.insert("-", true);
        defMap.insert("*", true);
        defMap.insert("/", true);

        defMap.insert("==", true);
        defMap.insert("!=", true);
        defMap.insert(">=", true);
        defMap.insert("<=", true);

        let mut MulOpsMap = HashMap::new();
        let mut SingleOptMap = HashMap::new();
        for (k, v) in &defMap {
            if k.len() > 1 {
                MulOpsMap.insert(k.clone(), v.clone());
            } else {
                SingleOptMap.insert(k.clone(), v.clone());
            }
        }

        Self {
            Map: defMap,
            MulOpsMap: MulOpsMap,
            SingleOptMap: SingleOptMap,
        }
    }
}

pub fn Parser(data: String) -> Vec<String> {
    let optMap = OptMap::new();
    let mut dataString = &mut data.clone();

    for (k, _) in &optMap.MulOpsMap {
        let mut newStr = String::from(" ");
        &newStr.push_str(k);
        &newStr.push_str(" ");
        let newDataStr = dataString.replace(k, &newStr);
        *dataString = newDataStr;
    }

    for (k, _) in &optMap.SingleOptMap {
        let mut newStr = String::from(" ");
        &newStr.push_str(k);
        &newStr.push_str(" ");
        let newDataStr = dataString.replace(k, &newStr);
        *dataString = newDataStr;
    }


    let splis: Vec<&str> = dataString.split(" ").collect();
    let mut result = vec![];
    for item in splis {
        if item==" "||item==""{
            continue;
        }
        result.push(item.to_string());
    }
    return result;
}

