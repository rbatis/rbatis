use std::collections::LinkedList;

use crate::core::Error;
use crate::interpreter::expr::ast::Node;
use crate::interpreter::expr::parser::parse;
use crate::interpreter::expr::token::TokenMap;

///lexer
pub fn lexer(express: &str, token_map: &TokenMap) -> Result<Vec<String>, Error> {
    let express = express.replace("none", "null").replace("None", "null");
    let mut tokens = parse_tokens(&express, token_map);
    loop_fill_lost_token(0, &mut tokens, token_map);
    return Ok(tokens);
}

//fill lost node to  '+1'  =>  ['(','null',"+",'1',')']
fn loop_fill_lost_token(start_index: usize, arg: &mut Vec<String>, opt_map: &TokenMap) {
    let len = arg.len();
    let mut last = "".to_string();
    for index in start_index..len {
        let item = arg[index].clone();
        if index == 0 && item != "(" && opt_map.is_token(&item) {
            let mut right = "null".to_string();
            if arg.get((index + 1) as usize).is_some() {
                right = arg.remove((index + 1) as usize);
            }
            let current = arg.remove(0);
            arg.insert(0, ")".to_string());
            arg.insert(0, right);
            arg.insert(0, current);
            arg.insert(0, "null".to_string());
            arg.insert(0, "(".to_string());
            return loop_fill_lost_token(4, arg, opt_map);
        }
        if index >= 1 &&
            last != ")"
            && item != "(" && item != ")"
            && (opt_map.is_token(&last))
            && opt_map.is_token(&item) {
            let mut right = "null".to_string();
            if arg.get((index + 1) as usize).is_some() {
                right = arg.remove((index + 1) as usize);
            }
            let current = arg.remove(index);
            arg.insert(index, ")".to_string());
            arg.insert(index, right);
            arg.insert(index, current);
            arg.insert(index, "null".to_string());
            arg.insert(index, "(".to_string());
            return loop_fill_lost_token(index + 5, arg, opt_map);
        }
        if (index + 1) as usize == len && item != ")" && opt_map.is_token(&item) {
            let right = "null".to_string();
            let current = arg.remove(index);
            let last;
            if (index - 1) as i32 >= 0 {
                last = arg.remove(index - 1);
            } else {
                last = "null".to_string();
            }
            let index = index - 1;
            arg.insert(index, ")".to_string());
            arg.insert(index, right);
            arg.insert(index, current);
            arg.insert(index, last);
            arg.insert(index, "(".to_string());
        }
        last = item.to_string();
    }
}

/// lexer and parse
pub fn lexer_parse_node(express: &str, token_map: &TokenMap) -> Result<Node, Error> {
    let tokens = lexer(express, token_map)?;
    return Ok(parse(token_map, &tokens, express)?);
}

///parse token to vec
pub fn parse_tokens(s: &String, token_map: &TokenMap) -> Vec<String> {
    let chars = s.chars();
    let chars_len = s.len() as i32;
    let mut result = LinkedList::new();
    //str
    let mut find_str = false;
    let mut temp_str = String::new();

    //token
    let mut temp_arg = String::new();
    let mut index: i32 = -1;
    for item in chars {
        index = index + 1;
        let is_token = token_map.is_token(item.to_string().as_str());
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
        if item != '`' && item != '\'' && is_token == false && !find_str {
            //need reset
            temp_arg.push(item);
            if (index + 1) == chars_len {
                trim_push_back(&temp_arg, &mut result);
            }
        } else {
            trim_push_back(&temp_arg, &mut result);
            temp_arg.clear();
        }
        //token node
        if is_token {
            if result.len() > 0 {
                let def = String::new();
                let back = result.back().unwrap_or(&def).clone();
                if token_map.is_token(&format!("{}{}", &back, &item)) == false {
                    trim_push_back(&item.to_string(), &mut result);
                    continue;
                }
                if back != "" && token_map.is_token(back.as_str()) {
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

#[cfg(test)]
mod test {
    use crate::interpreter::expr::lexer::lexer;
    use crate::interpreter::expr::token::TokenMap;

    #[test]
    fn test_fill() {
        let l = lexer("-1 == -a", &TokenMap::new()).unwrap();
        println!("{:?}", &l);
        assert_eq!(l, vec!["(", "null", "-", "1", ")", "==", "(", "null", "-", "a", ")"])
    }

    #[test]
    fn test_fill_first() {
        let l = lexer("-1 == -1", &TokenMap::new()).unwrap();
        println!("{:?}", &l);
        assert_eq!(l, vec!["(", "null", "-", "1", ")", "==", "(", "null", "-", "1", ")"])
    }

    #[test]
    fn test_fill_last() {
        let l = lexer("-1 == 1-", &TokenMap::new()).unwrap();
        println!("{:?}", &l);
        assert_eq!(l, vec!["(", "null", "-", "1", ")", "==", "(", "1", "-", "null", ")"])
    }

    #[test]
    fn test_fill_center() {
        let l = lexer("-1 == -1 && -1 == -2", &TokenMap::new()).unwrap();
        println!("{:?}", &l);
        assert_eq!(l, vec!["(", "null", "-", "1", ")", "==", "(", "null", "-", "1", ")", "&&", "(", "null", "-", "1", ")", "==", "(", "null", "-", "2", ")"])
    }

    #[test]
    fn test_fill_center_n() {
        let l = lexer("-1 -1 -1 --1", &TokenMap::new()).unwrap();
        println!("{:?}", &l);
        assert_eq!(l, vec!["(", "null", "-", "1", ")", "-", "1", "-", "1", "-", "(", "null", "-", "1", ")"])
    }
}