use std::collections::HashMap;
use core::borrow::Borrow;
use crate::lib::RustExpressionEngine::node::{Node, NullNode, OptNode, BoolNode, StringNode, NumberNode, ArgNode, BinaryNode, NodeItem};
use crate::lib::RustExpressionEngine::node::NodeType::NOpt;
use crate::lib::RustExpressionEngine::runtime::{IsNumber, OptMap, ParserTokens};
use std::collections::linked_list::LinkedList;


//TODO 解决bug

pub fn Parser(express: String, optMap: &OptMap) -> (NodeItem, String) {
    let tokens = ParserTokens(&express);

    let mut nodes = vec![];
    for item in tokens {
        let node = NodeItem::New(item);
//        if err != "" {
//            return (NodeItem::New("".to_string()), err);
//        }
        nodes.push(node);
    }
//    //TODO check nodes
//
//
    for item in optMap.priorityArray() {
          findReplaceOpt(&express,&item,&mut nodes);
    }
//
    for item in nodes{
        println!("{}:{}",item.Type(),item.Value());
    }


    return (NodeItem::New("".to_string()), "".to_string());
}

fn findReplaceOpt(express: &String, operator: &str, nodeArg: &mut Vec<NodeItem>) -> String {
    let mut index = 1 as i32;
    let nodeArgLen=nodeArg.len();
    for item in nodeArg.clone(){
        let itemType = item.Type();
        if itemType as i32 == NOpt as i32 {
            let leftIndex = (index - 1) as usize;
            let rightIndex = (index + 1) as usize;
            let left:NodeItem;
            let right:NodeItem;
            if leftIndex<0{
                left=NodeItem::New("".to_string());
            }else{
                left=nodeArg[leftIndex].clone();
            }

            if rightIndex > (nodeArgLen-1){
                right=NodeItem::New("".to_string());
            }else{
                right=nodeArg[rightIndex].clone();
            }

            let binaryNode=BinaryNode::NewItem(left, right, item.Value().as_str().unwrap().to_string());
            println!("binNode={}",&binaryNode.Type());
        }
        index = index + 1;
    }
    return "".to_string();
}