use std::collections::HashMap;
use core::borrow::Borrow;
use crate::lib::RustExpressionEngine::node::{Node, NullNode, OptNode, BoolNode, StringNode, NumberNode, ArgNode, BinaryNode, NodeItem};
use crate::lib::RustExpressionEngine::node::NodeType::NOpt;
use crate::lib::RustExpressionEngine::runtime::{IsNumber, OptMap, ParserTokens};
use std::collections::linked_list::LinkedList;


//TODO 解决bug

pub fn Parser(express: String, optMap: &OptMap) -> (NodeItem, String) {
   // let tokens = ParserTokens(&express);

//    let mut nodes:Vec<Box<Node>> = vec![];
//    for item in tokens {
//        let (boxNode, err) = parserNode(&express, &item,&optMap);
//        if err != "" {
//            return (Box::new(NullNode::new()), err);
//        }
//        nodes.push(boxNode);
//    }
//    //TODO check nodes
//
//
//    for item in optMap.priorityArray() {
//         // findReplaceOpt(&express,&item,&mut nodes);
//    }
//
//    for item in nodes{
//        println!("{}:{}",item.Type(),item.Value());
//    }


    return (NodeItem::New("".to_string()), "".to_string());
}

//
////解析表达式生成一个抽象语法节点，express:表达式，v:操作符
//fn parserNode(express: &String, v: &String,opt:&OptMap) -> (Box<Node>, String) {
//    if v == "" {
//        let nullNode = NullNode::new();
//        return (Box::new(nullNode), "".to_string());
//    }
//    //TODO NotSupportOptMap[v]
//    //opt node
//    if opt.isOpt(v.clone()) {
//        let optNode = OptNode::new(v.clone());
//        return (Box::new(optNode), "".to_string());
//    }
//    if v == "true" || v == "false" {
//        let boolNode = BoolNode::new(v.to_string());
//        return (Box::new(boolNode), "".to_string());
//    }
//    let firstIndex = v.find("'").unwrap_or_default();
//    let lastIndex = v.rfind("'").unwrap_or_default();
//
//    if firstIndex == 0 && lastIndex == (v.len() - 1) && firstIndex != lastIndex {
//        let strNode = StringNode::new(v.to_string());
//        return (Box::new(strNode), "".to_string());
//    }
//    if IsNumber(v) {
//        let numberNode = NumberNode::new(v);
//        return (Box::new(numberNode), "".to_string());
//    } else {
//        let argNode = ArgNode::new(v);
//        return (Box::new(argNode), "".to_string());
//    }
//    return (Box::new(NullNode::new()), "".to_string());
//}
//
//fn findReplaceOpt<T:Node+Clone>(express: &String, operator: &str, nodeArg: &mut Vec<Box<T>>) -> String {
//    let mut index = 1 as i32;
//    for item in nodeArg.clone(){
//        let itemType = item.Type();
//        if itemType as i32 == NOpt as i32 {
//            let leftIndex = (index - 1) as usize;
//            let rightIndex = (index + 1) as usize;
//            BinaryNode::new(nodeArg[leftIndex].clone(), nodeArg[rightIndex].clone(), item.Value().as_str().unwrap().to_string());
//        }
//        index = index + 1;
//    }
//    return "".to_string();
//}