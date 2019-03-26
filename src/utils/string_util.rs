use std::io::Read;
use std::collections::HashMap;

//find like #{*} value *
pub fn findConvertString(arg: String) -> Vec<String> {
    let mut finds = HashMap::new();
    let chars = arg.bytes();
    let mut item = &mut String::new();
    let mut lastIndex: i32 = -1;
    let mut startIndex: i32 = -1;
    let strBytes: Vec<u8> = arg.bytes().collect();

    let mut index = -1;
    for v in chars {
        index = index + 1;
        if v == 35 {
            lastIndex = index;
        }
        if v == 123 && lastIndex == (index - 1) {
            startIndex = index + 1;
        }
        if v == 125 && startIndex != -1 {
            *item = String::from_utf8(strBytes[startIndex as usize..index as usize].to_vec()).unwrap();
            //去掉逗号之后的部分
            if item.contains(',') {
                let vecs: Vec<&str> = item.split(",").collect();
                *item = vecs[0].to_string();
            }
            finds.insert(item.clone(), 1);
            item.clear();
            startIndex = -1;
            lastIndex = -1;
        }
    }
    let mut result = vec![];
    for (item, _) in finds {
        result.push(item);
    }
    return result;
}


//find like ${*} value *
pub fn findNoConvertString(arg: String) -> Vec<String> {
    let mut finds = HashMap::new();
    let chars = arg.bytes();
    let mut item =  String::new();
    let mut lastIndex: i32 = -1;
    let mut startIndex: i32 = -1;
    let strBytes: Vec<u8> = arg.bytes().collect();

    let mut index = -1;
    for v in chars {
        index = index + 1;
        if v == 36 {
            lastIndex = index;
        }
        if v == 123 && lastIndex == (index - 1) {
            startIndex = index + 1;
        }
        if v == 125 && startIndex != -1 {
            item = String::from_utf8(strBytes[startIndex as usize..index as usize].to_vec()).unwrap();
            //去掉逗号之后的部分
            if item.contains(',') {
                let vecs: Vec<&str> = item.split(",").collect();
                item = vecs[0].to_string();
            }
            finds.insert(item.clone(), 1);
            item.clear();
            startIndex = -1;
            lastIndex = -1;
        }
    }
    let mut result = vec![];
    for (item, _) in finds {
        result.push(item);
    }
    return result;
}