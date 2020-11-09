use core::borrow::Borrow;
use std::collections::HashMap;
use std::collections::linked_list::LinkedList;
use std::ops::Deref;

use crate::engine::node::Node;
use crate::engine::node::NodeType::{NBinary, NOpt};
use crate::engine::runtime::{is_number, OptMap};
use log::kv::Source;

pub fn parse(express: &str, opt_map: &OptMap) -> Result<Node, rbatis_core::Error> {
    let express = express.replace("none", "null").replace("None", "null");
    let tokens = parse_tokens(&express, opt_map);
    let mut nodes = vec![];
    for item in tokens {
        let node = Node::parse(item.as_str(), opt_map);
        if node.node_type == NOpt {
            let is_allow_opt = opt_map.is_allow_opt(item.as_str());
            if !is_allow_opt {
                panic!("[rbatis] py parser find not support opt: {} ,in express: {}", &item, &express);
            }
        }
        nodes.push(Box::new(node));
    }
    fix_null_items(&mut nodes);
    for item in opt_map.priority_array() {
        find_replace_opt(opt_map, &express, &item, &mut nodes);
    }
    if nodes.len() > 0 {
        return Result::Ok(nodes[0].deref().clone());
    } else {
        return Result::Err(rbatis_core::Error::from("[rbatis] parser express fail".to_string()));
    }
}


fn fix_null_items(node_arg: &mut Vec<Box<Node>>) {
    let mut len = node_arg.len();
    if len == 0 {
        return;
    }
    if node_arg.get(0).unwrap().node_type() == NOpt {
        node_arg.insert(0, Box::new(Node::new_null()));
        len = node_arg.len();
    }
    if len != 0 && node_arg.get(len - 1).unwrap().node_type() == NOpt {
        node_arg.push(Box::new(Node::new_null()));
        len = node_arg.len();
    }
    let index = 1;
    for index in 1..len {
        let last_index = (index - 1) as usize;
        let last = node_arg.get(last_index).unwrap();
        let current = node_arg.get(index).unwrap();
        if current.node_type() == NOpt && last.node_type() == NOpt {
            node_arg.insert(index, Box::new(Node::new_null()));
            fix_null_items(node_arg);
            return;
        }
    }
    return;
}

fn find_replace_opt(opt_map: &OptMap, express: &String, operator: &str, node_arg: &mut Vec<Box<Node>>) {
    //let nodes=vec![];
    let node_arg_len = node_arg.len();
    if node_arg_len == 1 {
        return;
    }
    for index in 1..(node_arg_len - 1) {
        let item = node_arg.get(index).unwrap();
        let item_type = item.node_type();
        let left_index = index - 1;
        let right_index = index + 1;
        if item_type == NOpt && operator == item.opt().unwrap() {
            let left = node_arg[left_index].clone();
            let right = node_arg[right_index].clone();
            let binary_node = Node::new_binary(left, right, item.opt().unwrap());
            node_arg.remove(right_index);
            node_arg.remove(index);
            node_arg.remove(left_index);
            node_arg.insert(left_index, Box::new(binary_node));
            if have_opt(node_arg) {
                find_replace_opt(opt_map, express, operator, node_arg);
                return;
            }
        }
    }
}

fn have_opt(node_arg: &Vec<Box<Node>>) -> bool {
    for item in node_arg {
        if item.node_type() as i32 == NOpt as i32 {
            return true;
        }
    }
    return false;
}

///parse token to vec
pub fn parse_tokens(s: &String, opt_map: &OptMap) -> Vec<String> {
    let chars = s.chars();
    let chars_len = s.len() as i32;
    let mut result = LinkedList::new();
    //str
    let mut find_str = false;
    let mut temp_str = String::new();

    //opt
    let mut temp_arg = String::new();
    let mut index: i32 = -1;
    for item in chars {
        index = index + 1;
        let is_opt = opt_map.is_opt(item.to_string().as_str());
        if item == '\'' || item == '`' {
            if find_str {
                //第二次找到
                find_str = false;
                temp_str.push(item);
                trim_push_back(&temp_str, &mut result);
                temp_str.clear();
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
        if item != '`' && item != '\'' && is_opt == false && !find_str {
            //need reset
            temp_arg.push(item);
            if (index + 1) == chars_len {
                trim_push_back(&temp_arg, &mut result);
            }
        } else {
            trim_push_back(&temp_arg, &mut result);
            temp_arg.clear();
        }
        //opt node
        if is_opt {
            if result.len() > 0 {
                let def = String::new();
                let back = result.back().unwrap_or(&def).clone();
                if opt_map.is_opt(&format!("{}{}", &back, &item)) == false {
                    trim_push_back(&item.to_string(), &mut result);
                    continue;
                }
                if back != "" && opt_map.is_opt(back.as_str()) {
                    result.pop_back();
                    let mut new_item = back.clone();
                    new_item.push(item);
                    trim_push_back(&new_item, &mut result);
                    continue;
                }
            }
            trim_push_back(&item.to_string(), &mut result);
            continue;
        }
    }
    let mut v = vec![];
    for item in result {
        v.push(item);
    }
    return v;
}

fn trim_push_back(arg: &str, list: &mut LinkedList<String>) {
    let trim_str = arg.trim().to_string();
    if trim_str.is_empty() {
        return;
    }
    list.push_back(trim_str);
}