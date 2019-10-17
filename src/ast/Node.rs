use super::NodeType::NodeType;
use std::collections::HashMap;
use serde_json::Value;
use crate::utils::xml_loader::Element;
use crate::ast::StringNode::StringNode;
use crate::ast::NodeConfigHolder::NodeConfigHolder;
use std::rc::Rc;
use crate::ast::IfNode::IfNode;
use crate::ast::TrimNode::TrimNode;
use crate::ast::ForEachNode::ForEachNode;
use crate::ast::ChooseNode::ChooseNode;
use crate::ast::NodeType::NodeType::NWhen;
use crate::ast::WhenNode::WhenNode;
use crate::ast::OtherwiseNode::OtherwiseNode;
use crate::ast::BindNode::BindNode;
use crate::ast::IncludeNode::IncludeNode;
use crate::ast::SetNode::SetNode;
use crate::ast::SelectNode::SelectNode;
use crate::ast::UpdateNode::UpdateNode;
use crate::ast::InsertNode::InsertNode;
use crate::ast::DeleteNode::DeleteNode;

/**
* Abstract syntax tree node
*/
pub trait SqlNode {
    fn eval(&mut self, env: &mut Value) -> Result<String, String>;

    fn print(&self) -> String;
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
pub fn LoopDecodeXml(xml_vec:Vec<Element>,holder:NodeConfigHolder) -> Vec<NodeType> {
    let mut nodes=vec![];
    for xml in xml_vec {
        let child_nodes;
        if xml.childs.len() > 0 {
            child_nodes = LoopDecodeXml(xml.clone().childs, holder.clone());
        }else{
            child_nodes = vec![];
        }
       let tag_str=xml.tag.as_str();
       //println!("tag_str:{}",tag_str);
       match tag_str {
           "mapper" => {
               //mapper 不做处理，直接返回子节点
               return child_nodes;
           },
           "select" => nodes.push(NodeType::NSelectNode(SelectNode{
               childs: child_nodes,
           })),
           "update" => nodes.push(NodeType::NUpdateNode(UpdateNode{
               childs: child_nodes,
           })),
           "insert" => nodes.push(NodeType::NInsertNode(InsertNode{
               childs: child_nodes,
           })),
           "delete" => nodes.push(NodeType::NDeleteNode(DeleteNode{
               childs: child_nodes,
           })),
           "if" => nodes.push(NodeType::NIf(IfNode{
               childs: child_nodes,
               test: xml.getAttr("test"),
               holder:holder.clone(),
           })),
           "trim" => nodes.push(NodeType::NTrim(TrimNode{
               childs: child_nodes,
               prefix: xml.getAttr("prefix"),
               suffix: xml.getAttr("suffix"),
               suffixOverrides: xml.getAttr("suffixOverrides"),
               prefixOverrides: xml.getAttr("prefixOverrides")
           })),
           "foreach" => nodes.push(NodeType::NForEach(ForEachNode{
               childs: child_nodes,
               collection: xml.getAttr("collection"),
               index: xml.getAttr("index"),
               item: xml.getAttr("item"),
               open: xml.getAttr("open"),
               close: xml.getAttr("close"),
               separator: xml.getAttr("separator")
           })),
           "choose" => nodes.push(NodeType::NChoose(ChooseNode{
               //todo filter nodes from child nodes
               whenNodes: filter_when_nodes(child_nodes.clone()),
               otherwiseNode: filter_otherwise_nodes(child_nodes),
           })),
           "when" => nodes.push(NodeType::NWhen(WhenNode{
               childs: child_nodes,
               test: xml.getAttr("test"),
               holder:holder.clone(),
           })),
           "otherwise" => nodes.push(NodeType::NOtherwise(OtherwiseNode{
               childs: child_nodes,
           })),
           "bind" => nodes.push(NodeType::NBind(BindNode{
               name: xml.getAttr("name"),
               value: xml.getAttr("value"),
               holder:holder.clone(),
           })),
           "include" => nodes.push(NodeType::NInclude(IncludeNode{
               childs: child_nodes,
           })),
           "set" => nodes.push(NodeType::NSet(SetNode{
               childs: child_nodes,
           })),
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
    return nodes;
}

pub fn filter_when_nodes(arg:Vec<NodeType>) -> Option<Vec<NodeType>>{
    let mut data=vec![];
    for x in arg {
        match x {
            NodeType::NWhen(whenNode) => data.push( NodeType::NWhen(whenNode)),
            _ => {}
        }
    }
    if data.len()==0 {
        return Option::None;
    }else{
        return Some(data);
    }
}

pub fn filter_otherwise_nodes(arg:Vec<NodeType>) -> Option<Box<NodeType>>{
    let mut data=vec![];
    for x in arg {
        match x {
            NodeType::NOtherwise(nOtherwise) => data.push(NodeType::NOtherwise(nOtherwise)),
            _ => {}
        }
    }
    if data.len()>0 {
        if data.len()>1{
            panic!("otherwise_nodes length can not > 1;")
        }
        let d0=data[0].clone();
        return Option::Some(Box::new(d0));
    }else{
        return Option::None;
    }
}