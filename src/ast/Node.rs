use super::NodeType::NodeType;
use std::collections::HashMap;
use serde_json::Value;
use crate::utils::xml_loader::Element;

/**
* Abstract syntax tree node
*/
pub trait SqlNode {
    fn eval(&mut self, env: &mut Value) -> Result<String, String>;
}


//执行子所有节点
pub fn DoChildNodes(childNodes: &mut Vec<NodeType>, env: &mut Value) -> Result<String, String> {
    let mut s = String::new();
    for item in childNodes {
        let itemResult = item.eval(env);
        if !itemResult.is_ok() {
            return itemResult;
        }
        s = s + itemResult.unwrap().as_str();
    }
    return Result::Ok(s);
}

//TODO decode xml
pub fn LoopDecodeXml(xml_vec:Vec<Element>) -> Result<String, String> {
    for xml in xml_vec {
       if xml.childs.len()!=0{
            let child_result= LoopDecodeXml(xml.childs);
       }
       let tag_str=xml.tag.as_str();
       println!("tag_str:{}",tag_str);
       if tag_str == ""{
           println!("str>>>>  {}",xml.data);
       }
       match tag_str {
           "select" => println!("select"),
           "update" => println!("update"),
           "insert" => println!("insert"),
           "delete" => println!("delete"),
           _ => {}
       }
    }
    return Result::Ok("ssd".to_string());
}