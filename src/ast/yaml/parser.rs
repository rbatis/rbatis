use std::collections::{BTreeMap, HashMap};
use std::fs;

use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

use crate::ast::ast::Ast;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MdElement {
    pub tag: &'static str,
    pub string: Option<String>,
    pub attrs: Option<HashMap<&'static str, String>>,
    //属性
    pub childs: Option<Vec<MdElement>>,
}

pub fn parser_md_element(express: &str) -> Option<Vec<MdElement>> {
    let s = express.replace("#{", "\\#{");
    let map: serde_yaml::Value = serde_yaml::from_str(s.as_str()).unwrap();
    let mapping = map.as_mapping().unwrap();
    return parser_md_element_arr(mapping);
}

pub fn parser_md_element_arr(mapping: &Mapping) -> Option<Vec<MdElement>> {
    //表达式组
    let mut arr = vec![];
    for (_key, _value) in mapping {
        if _value.is_sequence() {
            let data = _value.as_sequence().unwrap();
            for x in data {
                filter_str_mapping(_key, x, &mut arr);
            }
        } else {
            filter_str_mapping(_key, _value, &mut arr);
        }
    }
    return Option::Some(arr);
}

fn filter_value(arg: &String) -> String {
    return arg.replace("\\#{", "#{");
}

fn filter_str_mapping(_key: &serde_yaml::Value, _value: &serde_yaml::Value, arr: &mut Vec<MdElement>) {
    let key = _key.as_str().unwrap();
    let mut md = MdElement {
        tag: create_tag(key),
        string: None,
        attrs: None,
        childs: None,
    };
    if _value.is_string() {
        let s = _value.as_str().unwrap();
        md.string = Option::from(filter_value(&s.to_string()));
        arr.push(md);
    } else if _value.is_mapping() {
        let mut prop = key.replace(md.tag, "");
        prop = filter_value(&prop);
        if !prop.is_empty() {
            let mut map = HashMap::new();
            match md.tag {
                "select" => {
                    map.insert("id", prop.clone());
                }
                "update" => {
                    map.insert("id", prop.clone());
                }
                "insert" => {
                    map.insert("id", prop.clone());
                }
                "delete" => {
                    map.insert("id", prop.clone());
                }

                "choose" => {
                    map.insert("id", prop.clone());
                }
                "bind" => {
                    map.insert("id", prop.clone());
                }
                "include" => {
                    map.insert("refid", prop.clone());
                }
                "otherwise" => {}
                "when" => {
                    map.insert("test", prop.clone());
                }
                "set" => {

                }
                "sql" => {

                }
                "if" => {
                    map.insert("test", prop.clone());
                }
                "trim" => {
                    map.insert("value", prop.clone());
                }
                "for" => {
                    let ins: Vec<&str> = prop.split(" in ").collect();
                    let col = ins[1];
                    map.insert("in", col.to_string());
                    let expr = ins[0];
                    if expr.contains(",") {
                        let exprs: Vec<&str> = expr.split(",").collect();
                        map.insert("index", exprs[0].to_string());
                        map.insert("item", exprs[1].to_string());
                    }
                }
                _ => {}
            }
            md.attrs = Option::Some(map);
        }
        let map: &Mapping = _value.as_mapping().unwrap();
        let nodes = parser_md_element_arr(map);
        md.childs = nodes;
        arr.push(md);
    }
}


const TAG_UN_KNOW: &'static str = "unknow";
const TAG_STRING: &'static str = "sql";
const TAG_IF: &'static str = "if";
const TAG_ELSE: &'static str = "else";
const TAG_WHERE: &'static str = "where";
const TAG_FOR: &'static str = "for";
const TAG_TRIM: &'static str = "trim";

const TAG_SELECT: &'static str = "select";
const TAG_Update: &'static str = "update";
const TAG_Delete: &'static str = "delete";
const TAG_Insert: &'static str = "insert";

fn create_tag(arg: &str) -> &'static str {
    let expr = arg.trim();
    if expr.starts_with(TAG_STRING) {
        return TAG_STRING;
    } else if expr.starts_with(TAG_IF) {
        return TAG_IF;
    } else if expr.starts_with(TAG_ELSE) {
        return TAG_ELSE;
    } else if expr.starts_with(TAG_WHERE) {
        return TAG_WHERE;
    } else if expr.starts_with(TAG_FOR) {
        return TAG_FOR;
    } else if expr.starts_with(TAG_TRIM) {
        return TAG_TRIM;
    } else if expr.starts_with(TAG_SELECT) {
        return TAG_SELECT;
    } else if expr.starts_with(TAG_Update) {
        return TAG_Update;
    } else if expr.starts_with(TAG_Delete) {
        return TAG_Delete;
    } else if expr.starts_with(TAG_Insert) {
        return TAG_Insert;
    }
    return TAG_UN_KNOW;
}

fn count_space(express: &str) -> i32 {
    let bytes = express.chars();
    let mut size = 0;
    let mut last = false;
    for x in bytes {
        if x == ' ' {
            if last == true {
                break;
            }
            size += 1;
        } else {
            last = true;
        }
    }
    return size;
}

pub fn parser(express: &'static str) -> Option<Vec<Box<dyn Ast>>> {
    //TODO 表达式组
    let express_array: Vec<&str> = express.split("\n").collect();
    for item in express_array {}
    return Option::None;
}

#[test]
pub fn test_count_space() {
    println!("{}", count_space(""));
    println!("{}", count_space(" if"));
    println!("{}", count_space("  if else"));
}

#[test]
pub fn test_parser_md_element() {
    let s = fs::read_to_string("./src/example/Example_ActivityMapper.yaml").unwrap();
    let arr = parser_md_element(s.as_str()).unwrap();
    for x in arr {
        println!("js:{}", serde_json::to_string(&x).unwrap());
    }
}
