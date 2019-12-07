use super::node_type::NodeType;
use std::collections::HashMap;
use serde_json::Value;
use crate::utils::xml_loader::Element;
use crate::ast::string_node::StringNode;
use crate::ast::node_config_holder::NodeConfigHolder;
use std::rc::Rc;
use crate::ast::if_node::IfNode;
use crate::ast::trim_node::TrimNode;
use crate::ast::foreach_node::ForEachNode;
use crate::ast::choose_node::ChooseNode;
use crate::ast::node_type::NodeType::NWhen;
use crate::ast::when_node::WhenNode;
use crate::ast::otherwise_node::OtherwiseNode;
use crate::ast::bind_node::BindNode;
use crate::ast::include_node::IncludeNode;
use crate::ast::set_node::SetNode;
use crate::ast::select_node::SelectNode;
use crate::ast::update_node::UpdateNode;
use crate::ast::insert_node::InsertNode;
use crate::ast::delete_node::DeleteNode;
use crate::ast::delete_templete_node::DeleteTempleteNode;
use crate::ast::insert_templete_node::InsertTempleteNode;
use crate::ast::update_templete_node::UpdateTempleteNode;
use crate::ast::select_templete_node::SelectTempleteNode;
use crate::ast::where_node::WhereNode;

/**
* Abstract syntax tree node
*/
pub trait SqlNode {
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String>;

    fn print(&self,deep:i32) -> String;
}


//执行子所有节点
pub fn do_child_nodes(childNodes: &mut Vec<NodeType>, env: &mut Value, holder:&mut NodeConfigHolder) -> Result<String, String> {
    let mut s = String::new();
    for item in childNodes {
        let itemResult = item.eval(env,holder);
        if itemResult.is_err() {
            return itemResult;
        }
        s = s + itemResult.unwrap().as_str();
    }
    return Result::Ok(s);
}

//TODO decode xml
pub fn loop_decode_xml(xml_vec: &Vec<Element>, holder:&NodeConfigHolder) -> Vec<NodeType> {
    let mut nodes=vec![];
    for xml in xml_vec {
        let child_nodes;
        if xml.childs.len() > 0 {
            child_nodes = loop_decode_xml(&(&xml).childs, holder);
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
               id: xml.getAttr("id"),
               resultMap: xml.getAttr("resultMap"),
               childs: child_nodes,
           })),
           "update" => nodes.push(NodeType::NUpdateNode(UpdateNode{
               id: xml.getAttr("id"),
               childs: child_nodes,
           })),
           "insert" => nodes.push(NodeType::NInsertNode(InsertNode{
               id: xml.getAttr("id"),
               childs: child_nodes,
           })),
           "delete" => nodes.push(NodeType::NDeleteNode(DeleteNode{
               id: xml.getAttr("id"),
               childs: child_nodes,
           })),

           "selectTemplete" => nodes.push(NodeType::NSelectTempleteNode(SelectTempleteNode{
               id: xml.getAttr("id"),
               resultMap: xml.getAttr("resultMap"),
               lang: xml.getAttr("lang"),
               tables: xml.getAttr("tables"),
               columns: xml.getAttr("columns"),
               wheres: xml.getAttr("wheres"),
               childs: child_nodes,
           })),
           "updateTemplete" => nodes.push(NodeType::NUpdateTempleteNode(UpdateTempleteNode{
               id: xml.getAttr("id"),
               childs: child_nodes,
           })),
           "insertTemplete" => nodes.push(NodeType::NInsertTempleteNode(InsertTempleteNode{
               id: xml.getAttr("id"),
               childs: child_nodes,
           })),
           "deleteTemplete" => nodes.push(NodeType::NDeleteTempleteNode(DeleteTempleteNode{
               id: xml.getAttr("id"),
               childs: child_nodes,
           })),


           "if" => nodes.push(NodeType::NIf(IfNode{
               childs: child_nodes,
               test: xml.getAttr("test"),
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
           })),
           "where" => nodes.push(NodeType::NWhere(WhereNode{
               childs: child_nodes,
           })),
           "otherwise" => nodes.push(NodeType::NOtherwise(OtherwiseNode{
               childs: child_nodes,
           })),
           "bind" => nodes.push(NodeType::NBind(BindNode{
               name: xml.getAttr("name"),
               value: xml.getAttr("value"),
           })),
           "include" => nodes.push(NodeType::NInclude(IncludeNode{
               refid: xml.getAttr("refid"),
               childs: child_nodes,
           })),
           "set" => nodes.push(NodeType::NSet(SetNode{
               childs: child_nodes,
           })),
           "" => {
               let data= xml.data.as_str();
               let tag= xml.tag.as_str();
               let n = StringNode::new(data);
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


pub fn print_child(arg:&Vec<NodeType>,deep:i32)->String{
    let mut result=String::new();
    for x in arg{
        let item=x.print(deep);
        result=result+""+item.as_str();
    }
    return result;
}

pub fn create_deep(deep:i32)-> String {
    let mut s="\n".to_string();
    for index in 0..deep{
        s=s+"  ";
    }
    return s;
}