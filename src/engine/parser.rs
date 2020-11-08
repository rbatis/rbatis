use core::borrow::Borrow;
use std::collections::HashMap;
use std::collections::linked_list::LinkedList;
use std::ops::Deref;

use crate::engine::node::Node;
use crate::engine::node::NodeType::{NBinary, NOpt};
use crate::engine::runtime::{is_number, OptMap, parse_tokens};

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
    //TODO check nodes
    for item in opt_map.priority_array() {
        find_replace_opt(opt_map, &express, &item, &mut nodes);
    }
    if nodes.len() > 0 {
        return Result::Ok(nodes[0].deref().clone());
    } else {
        return Result::Err(rbatis_core::Error::from("[rbatis] parser express fail".to_string()));
    }
}

fn find_replace_opt(opt_map: &OptMap, express: &String, operator: &str, node_arg: &mut Vec<Box<Node>>) {
    //let nodes=vec![];
    let mut index = 0 as i32;
    let node_arg_len = node_arg.len();
    let null_node = Box::new(Node::new_null());
    for item in node_arg.clone() {
        let item_type = item.node_type();
        if item_type as i32 == NOpt as i32 && operator == item.opt().unwrap() {
            let left_index = (index - 1);
            let right_index = (index + 1) as usize;

            let left_node;
            if left_index < 0 {
                left_node = None;
            } else {
                left_node = node_arg.get(left_index as usize);
            }
            let right_node = node_arg.get(right_index);
            let have_left = left_node.is_some();
            let have_right = right_node.is_some();

            let left = left_node.unwrap_or(&null_node).clone();
            let right = right_node.unwrap_or(&null_node).clone();
            let binary_node = Node::new_binary(left, right, item.opt().unwrap());

            if have_right {
                node_arg.remove(right_index);
            }
            node_arg.remove(index as usize);
            if have_left {
                node_arg.remove(left_index as usize);
            }
            if left_index >= 0 {
                node_arg.insert(left_index as usize, Box::new(binary_node));
            } else {
                node_arg.push(Box::new(binary_node));
            }
            if have_opt(node_arg) {
                find_replace_opt(opt_map, express, operator, node_arg);
            }
            break;
        }
        index = index + 1;
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