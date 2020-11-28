use std::collections::LinkedList;

use crate::core::Error;
use crate::interpreter::json::ast::Node;
use crate::interpreter::json::token::TokenMap;
use crate::interpreter::json::parser::parse;

///lexer
pub fn lexer(express: &str, token_map: &TokenMap) -> Result<Vec<String>, Error> {
    let express = express.replace("none", "null").replace("None", "null");
    let tokens = parse_tokens(&express, token_map);
    return Ok(tokens);
}

/// lexer and parse
pub fn lexer_parse_node(express: &str, token_map: &TokenMap) -> Result<Node, Error> {
    let tokens=lexer(express,token_map)?;
    return Ok(parse(token_map,&tokens,express)?);
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