use super::NodeType::NodeType;
use std::collections::HashMap;
use serde_json::Value;
use crate::utils::xml_loader::Element;
use crate::ast::StringNode::StringNode;
use crate::ast::NodeConfigHolder::NodeConfigHolder;
use std::rc::Rc;

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
pub fn LoopDecodeXml(xml_vec:Vec<Element>,holder:NodeConfigHolder) -> Result<Vec<NodeType>, String> {
    let mut nodes=vec![];
    for xml in xml_vec {
        if xml.childs.len() != 0 {
            let child_result = LoopDecodeXml(xml.childs, holder.clone());
        }
       let tag_str=xml.tag.as_str();
       //println!("tag_str:{}",tag_str);
       match tag_str {
           "select" => println!("<select>"),
           "update" => println!("<update>"),
           "insert" => println!("<insert>"),
           "delete" => println!("<delete>"),
           "if" => println!("<if>"),
           "trim" => println!("<trim>"),
           "foreach" => println!("<foreach>"),
           "choose" => println!("<choose>"),
           "when" => println!("<when>"),
           "otherwise" => println!("<otherwise>"),
           "bind" => println!("<bind>"),
           "include" => println!("<include>"),
           "" => {
               let string = xml.data.replace("\n", "");
               let data=string.as_str();
               let tag=xml.tag.as_str();
               let n = StringNode::new(data, holder.clone());
               println!("{}",data);
               nodes.push(NodeType::NString(n));
           },
           _ => {}
       }
    }
    return Result::Ok(nodes);
}