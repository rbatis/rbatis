use std::collections::{BTreeMap, HashSet, LinkedList};
use std::io::Read;

use serde_json::map::Map;
use serde_json::Value;

//2020-11-15 00:31:25.803227700 +08:00 INFO rbatis::plugin::log
pub const LOG_SPACE: &'static str =
    "                                                                ";

//find like #{*,*},${*,*} value *
pub fn find_convert_string(arg: &str) -> LinkedList<(String, String)> {
    let mut list = LinkedList::new();
    let mut cache = HashSet::new();
    let chars: Vec<u8> = arg.bytes().collect();
    let mut item = String::with_capacity(arg.len());
    let mut index: i32 = -1;
    for v in &chars {
        index = index + 1;
        if !item.is_empty() {
            item.push(*v as char);
            if *v == '}' as u8 {
                if cache.get(&item).is_some() {
                    item.clear();
                    continue;
                }
                let key = item[2..item.len() - 1].to_string();
                cache.insert(item.clone());
                list.push_back((key, item.clone()));
                item.clear();
            }
            continue;
        }
        if (*v == '#' as u8 || *v == '$' as u8)
            && chars.get(index as usize + 1).eq(&Some(&('{' as u8)))
        {
            item.push(*v as char);
        }
    }
    return list;
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

pub fn to_snake_name(name: &str) -> String {
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

///input 'strings' => strings
pub fn un_packing_string(column: &str) -> &str {
    if column.len() >= 2 {
        if column.starts_with("'") && column.ends_with("'") {
            return &column[1..column.len() - 1];
        }
        if column.starts_with("`") && column.ends_with("`") {
            return &column[1..column.len() - 1];
        }
        if column.starts_with("\"") && column.ends_with("\"") {
            return &column[1..column.len() - 1];
        }
    }
    return column;
}
