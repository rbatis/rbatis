use std::collections::{BTreeMap, HashSet, LinkedList};
use std::io::Read;

//2020-11-15 00:31:25.803227700 +08:00 INFO rbatis::plugin::log
pub const LOG_SPACE: &str = "                                                         ";

//find like #{*,*},${*,*} value *
pub fn find_convert_string(arg: &str) -> LinkedList<(String, String)> {
    let mut list = LinkedList::new();
    let mut cache = HashSet::new();
    let chars: Vec<u8> = arg.bytes().collect();
    let mut item = String::with_capacity(arg.len());
    let mut index: i32 = -1;
    for v in &chars {
        index += 1;
        if !item.is_empty() {
            item.push(*v as char);
            if *v == b'}' {
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
        if (*v == b'#' || *v == b'$')
            && chars.get(index as usize + 1).eq(&Some(&b'{'))
        {
            item.push(*v as char);
        }
    }
    list
}
/// convert name to snake name
pub fn to_snake_name(name: &str) -> String {
    let chs = name.chars();
    let mut new_name = String::new();
    let chs_len = name.len();
    for (index, x) in chs.enumerate() {
        if x.is_uppercase() {
            if index != 0 && (index + 1) != chs_len {
                new_name.push('_');
            }
            new_name.push(x.to_ascii_lowercase());
        } else {
            new_name.push(x);
        }
    }
    new_name
}