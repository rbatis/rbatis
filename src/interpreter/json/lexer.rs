use core::borrow::Borrow;
use std::collections::HashMap;
use std::collections::linked_list::LinkedList;
use std::ops::Deref;

use crate::core::Error;
use crate::interpreter::json::node::Node;
use crate::interpreter::json::node::NodeType::{NBinary, NOpt};
use crate::interpreter::json::runtime::OptMap;

pub fn parse(express: &str, opt_map: &OptMap) -> Result<Node, Error> {
    let express = express.replace("none", "null").replace("None", "null");
    let mut tokens = parse_tokens(&express, opt_map);
    tokens = fill_lost_token(&tokens, opt_map);
    check_tokens_open_close(&tokens, &express)?;
    let mut nodes = loop_parse_temp_node(&tokens, opt_map, &express)?;
    return to_binary_node(&mut nodes, opt_map, &express);
}

fn fill_lost_token(arg: &Vec<String>, opt_map: &OptMap) -> Vec<String> {
    let mut new = vec![];
    let len = arg.len();
    let mut index = -1;
    let mut last = "".to_string();
    let mut jump: i32 = -1;
    for item in arg {
        index += 1;
        if jump != -1 && index == jump {
            jump = -1;
            last = item.to_string();
            continue;
        }
        if item != "(" && index == 0 && opt_map.is_opt(item) {
            new.push("null".to_string());
        }
        if last != ")"
            && item != "(" && item != ")"
            && index >= 1
            && (opt_map.is_opt(&last))
            && opt_map.is_opt(item) {
            new.push("(".to_string());
            new.push("null".to_string());
            new.push(item.to_string());
            new.push(arg[(index + 1) as usize].to_string());
            new.push(")".to_string());
            jump = index + 1;
            last = item.to_string();
            continue;
        } else {
            new.push(item.to_string());
        }
        if item != ")" && (index + 1) as usize == len && opt_map.is_opt(item) {
            new.push("null".to_string());
        }
        last = item.to_string();
    }
    new
}

fn loop_parse_temp_node(tokens: &[String], opt_map: &OptMap, express: &str) -> Result<Vec<Node>, Error> {
    let len = tokens.len();
    let mut result = vec![];
    let mut temp_nodes = vec![];
    let mut find_open = false;
    let mut index: i32 = -1;
    //skip
    let mut skip_start: i32 = -1;
    let mut skip_end: i32 = -1;
    for item in tokens {
        index += 1;
        if skip_start != -1 && skip_end != -1 {
            if index >= skip_start && index <= skip_end {
                continue;
            }
        }
        if find_open == false && item == "(" {
            find_open = true;
            continue;
        }
        if find_open == true && item == ")" {
            find_open = false;
            result.push(to_binary_node(&mut temp_nodes, &opt_map, &express)?);
            temp_nodes.clear();
            continue;
        }
        if item == "(" {
            let end = find_eq_end(tokens, index) as usize;
            let sub_tokens = &tokens[index as usize..end];
            let new_nodes = loop_parse_temp_node(&sub_tokens, opt_map, express)?;
            for node in new_nodes {
                if node.node_type == NOpt {
                    let is_allow_opt = opt_map.is_allow_opt(item.as_str());
                    if !is_allow_opt {
                        return Err(Error::from(format!("[rbatis] py lexer find not support opt: '{}' ,in express: '{}'", &item, &express)));
                    }
                }
                if find_open {
                    temp_nodes.push(node);
                } else {
                    result.push(node);
                }
            }
            skip_start = index;
            skip_end = skip_start + (sub_tokens.len() - 1) as i32;
        } else {
            let node = Node::parse(item.as_str(), opt_map);
            if node.node_type == NOpt {
                let is_allow_opt = opt_map.is_allow_opt(item.as_str());
                if !is_allow_opt {
                    return Err(Error::from(format!("[rbatis] py lexer find not support opt: '{}' ,in express: '{}'", &item, &express)));
                }
            }
            if find_open {
                temp_nodes.push(node);
            } else {
                result.push(node);
            }
        }
    }
    return Ok(result);
}


fn find_eq_end(arg: &[String], start: i32) -> i32 {
    let mut index = -1;
    let mut open = 0;
    let mut close = 0;
    for x in arg {
        index += 1;
        if index <= start {
            if index == start{
                open += 1;
            }
            continue;
        }
        if x == "(" {
            open += 1;
        }
        if x == ")" {
            close += 1;
            if close == open {
                return index + 1;
            }
        }
    }
    return index;
}

/// check '(',')' num
fn check_tokens_open_close(tokens: &Vec<String>, express: &str) -> Result<(), Error> {
    let mut open_nums = 0;
    let mut close_nums = 0;
    for x in tokens {
        if x == "(" {
            open_nums += 1;
        }
        if x == ")" {
            close_nums += 1;
        }
    }
    if open_nums != close_nums {
        return Err(Error::from(format!("[rbatis] py lexer find '(' num not equal ')' num,in express: '{}'", &express)));
    }
    Ok(())
}


fn to_binary_node(nodes: &mut Vec<Node>, opt_map: &OptMap, express: &str) -> Result<Node, Error> {
    let nodes_len = nodes.len();
    if nodes_len == 0 {
        return Result::Err(crate::core::Error::from(format!("[rbatis] lexer express '{}' fail", express)));
    }
    if nodes_len == 1 {
        return Ok(nodes[0].to_owned());
    }
    for item in opt_map.priority_array() {
        replace_to_binary_node(opt_map, express, &item, nodes);
    }
    if nodes.len() > 0 {
        return Result::Ok(nodes[0].to_owned());
    } else {
        return Result::Err(crate::core::Error::from(format!("[rbatis] lexer express '{}' fail", express)));
    }
}


fn replace_to_binary_node(opt_map: &OptMap, express: &str, operator: &str, node_arg: &mut Vec<Node>) {
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
            node_arg.insert(left_index, binary_node);
            if have_opt(node_arg) {
                replace_to_binary_node(opt_map, express, operator, node_arg);
                return;
            }
        }
    }
}

fn have_opt(node_arg: &Vec<Node>) -> bool {
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