
use std::io::Read;
use serde_json::map::Map;
use serde_json::Value;

//2020-11-15 00:31:25.803227700 +08:00 INFO rbatis::plugin::log
pub const LOG_SPACE: &'static str = "                                                                ";

//find like #{*,*},${*,*} value *
pub fn find_convert_string(arg: &str) -> Map<String,Value> {
    let mut finds = Map::new();
    let chars = arg.bytes();
    let item = &mut String::new();
    let mut last_index: i32 = -1;
    let mut start_index: i32 = -1;
    let str_bytes: Vec<u8> = arg.bytes().collect();

    let mut index = -1;
    for v in chars {
        index = index + 1;
        if v == '#' as u8 || v == '$' as u8 {
            last_index = index;
        }
        if v == '{' as u8 && last_index == (index - 1) {
            start_index = index + 1;
        }
        if v == '}' as u8 && start_index != -1 {
            *item = String::from_utf8(str_bytes[start_index as usize..index as usize].to_vec()).unwrap();
            let value = String::from_utf8(str_bytes[(start_index-2) as usize..(index+1) as usize].to_vec()).unwrap();
            finds.insert(item.clone(), serde_json::Value::String(value));
            item.clear();
            start_index = -1;
            last_index = -1;
        }
    }
    return finds;
}


pub fn count_string_num(s: &String, c: char) -> usize {
    let cs = s.chars();
    let mut num = 0;
    for x in cs {
        if x == c {
            num += 1;
        }
    }
    return num;
}


pub fn to_snake_name(name: &String) -> String {
    let chs = name.chars();
    let mut new_name = String::new();
    let mut index = 0;
    let chs_len = name.len();
    for x in chs {
        if x.is_uppercase() {
            if index != 0 && (index + 1) != chs_len {
                new_name.push_str("_");
            }
            new_name.push_str(x.to_lowercase().to_string().as_str());
        } else {
            new_name.push(x);
        }
        index += 1;
    }
    return new_name;
}