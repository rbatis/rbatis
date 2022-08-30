use std::collections::{BTreeMap, HashSet, LinkedList};
use std::io::Read;

//2020-11-15 00:31:25.803227700 +08:00 INFO rbatis::plugin::log
pub const LOG_SPACE: &'static str = "                                                      ";

/// convert name to snake name
pub fn to_snake_name(name: &str) -> String {
    let len = name.len();
    let bytes = name.as_bytes();
    let mut new_name = String::with_capacity(name.len());
    let mut index = 0;
    let mut c = ' ';
    for x in bytes {
        c = *x as char;
        if c.is_ascii_uppercase() {
            if index != 0 && (index + 1) != len {
                new_name.push('_');
            }
            new_name.push(c.to_ascii_lowercase() as char);
        } else {
            new_name.push(c);
        }
        index += 1;
    }
    return new_name;
}
