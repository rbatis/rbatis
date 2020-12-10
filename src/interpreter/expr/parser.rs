use crate::core::Error;
use crate::interpreter::expr::ast::Node;
use crate::interpreter::expr::ast::NodeType::NOpt;
use crate::interpreter::expr::token::TokenMap;

/// parse token to node
pub fn parse(token_map: &TokenMap, tokens: &Vec<String>, express: &str) -> Result<Node, Error> {
    check_tokens_open_close(&tokens, &express)?;
    let mut nodes = loop_parse_temp_node(&tokens, token_map, &express)?;
    return to_binary_node(&mut nodes, token_map, &express);
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


fn loop_parse_temp_node(tokens: &[String], token_map: &TokenMap, express: &str) -> Result<Vec<Node>, Error> {
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
            result.push(to_binary_node(&mut temp_nodes, &token_map, &express)?);
            temp_nodes.clear();
            continue;
        }
        if item == "(" {
            let end = find_eq_end(tokens, index) as usize;
            let sub_tokens = &tokens[index as usize..end];
            let new_nodes = loop_parse_temp_node(&sub_tokens, token_map, express)?;
            for node in new_nodes {
                if node.node_type == NOpt {
                    let is_allow_token = token_map.is_allow_token(item.as_str());
                    if !is_allow_token {
                        return Err(Error::from(format!("[rbatis] py lexer find not support token: '{}' ,in express: '{}'", &item, &express)));
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
            let node = Node::parse(item.as_str(), token_map);
            if node.node_type == NOpt {
                let is_allow_token = token_map.is_allow_token(item.as_str());
                if !is_allow_token {
                    return Err(Error::from(format!("[rbatis] py lexer find not support token: '{}' ,in express: '{}'", &item, &express)));
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


fn to_binary_node(nodes: &mut Vec<Node>, token_map: &TokenMap, express: &str) -> Result<Node, Error> {
    let nodes_len = nodes.len();
    if nodes_len == 0 {
        return Result::Err(crate::core::Error::from(format!("[rbatis] lexer express '{}' fail", express)));
    }
    if nodes_len == 1 {
        return Ok(nodes[0].to_owned());
    }
    for item in token_map.priority_array() {
        loop_replace_to_binary_node(token_map, express, &item, nodes);
    }
    if nodes.len() > 0 {
        return Result::Ok(nodes[0].to_owned());
    } else {
        return Result::Err(crate::core::Error::from(format!("[rbatis] lexer express '{}' fail", express)));
    }
}

fn find_eq_end(arg: &[String], start: i32) -> i32 {
    let mut index = -1;
    let mut open = 0;
    let mut close = 0;
    for x in arg {
        index += 1;
        if index <= start {
            if index == start {
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

fn loop_replace_to_binary_node(token_map: &TokenMap, express: &str, operator: &str, node_arg: &mut Vec<Node>) {
    let node_arg_len = node_arg.len();
    if node_arg_len == 1 {
        return;
    }
    for index in 1..(node_arg_len - 1) {
        let item = node_arg.get(index).unwrap();
        let item_type = item.node_type();
        let left_index = index - 1;
        let right_index = index + 1;
        if item_type == NOpt && operator == item.token().unwrap() {
            let left = node_arg[left_index].clone();
            let right = node_arg[right_index].clone();
            let binary_node = Node::new_binary(left, right, item.token().unwrap());
            node_arg.remove(right_index);
            node_arg.remove(index);
            node_arg.remove(left_index);
            node_arg.insert(left_index, binary_node);
            if have_token(node_arg) {
                loop_replace_to_binary_node(token_map, express, operator, node_arg);
                return;
            }
        }
    }
}

fn have_token(node_arg: &Vec<Node>) -> bool {
    for item in node_arg {
        if item.node_type() as i32 == NOpt as i32 {
            return true;
        }
    }
    return false;
}
