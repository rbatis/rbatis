use std::collections::HashMap;
use std::io::Read;

//find like #{*} value *
pub fn find_convert_string(arg: &str) -> Vec<String> {
    let mut finds = HashMap::new();
    let chars = arg.bytes();
    let item = &mut String::new();
    let mut last_index: i32 = -1;
    let mut start_index: i32 = -1;
    let str_bytes: Vec<u8> = arg.bytes().collect();

    let mut index = -1;
    for v in chars {
        index = index + 1;
        if v == 35 {
            last_index = index;
        }
        if v == 123 && last_index == (index - 1) {
            start_index = index + 1;
        }
        if v == 125 && start_index != -1 {
            *item = String::from_utf8(str_bytes[start_index as usize..index as usize].to_vec()).unwrap();
            //去掉逗号之后的部分
            if item.contains(',') {
                let vecs: Vec<&str> = item.split(",").collect();
                *item = vecs[0].to_string();
            }
            finds.insert(item.clone(), 1);
            item.clear();
            start_index = -1;
            last_index = -1;
        }
    }
    let mut result = vec![];
    for (item, _) in finds {
        result.push(item);
    }
    return result;
}


//find like ${*} value *
pub fn find_no_convert_string(arg: &str) -> Vec<String> {
    let mut finds = HashMap::new();
    let chars = arg.bytes();
    let mut item =  String::new();
    let mut last_index: i32 = -1;
    let mut start_index: i32 = -1;
    let str_bytes: Vec<u8> = arg.bytes().collect();

    let mut index = -1;
    for v in chars {
        index = index + 1;
        if v == 36 {
            last_index = index;
        }
        if v == 123 && last_index == (index - 1) {
            start_index = index + 1;
        }
        if v == 125 && start_index != -1 {
            item = String::from_utf8(str_bytes[start_index as usize..index as usize].to_vec()).unwrap();
            //去掉逗号之后的部分
            if item.contains(',') {
                let vecs: Vec<&str> = item.split(",").collect();
                item = vecs[0].to_string();
            }
            finds.insert(item.clone(), 1);
            start_index = -1;
            last_index = -1;
        }
    }
    let mut result = vec![];
    for (item, _) in finds {
        result.push(item);
    }
    return result;
}