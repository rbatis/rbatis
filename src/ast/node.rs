use super::node_type::NodeType;
use std::collections::HashMap;
use serde_json::Value;
use crate::utils::xml_loader::Element;
use crate::ast::string_node::StringNode;
use crate::ast::config_holder::ConfigHolder;
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
use crate::ast::where_node::WhereNode;

/**
* Abstract syntax tree node
*/
pub trait SqlNode {
    /**
    env: &mut Value,因为bind node 会绑定变量，env必须为可修改的值
    */
    fn eval(&mut self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String, String>;

    fn print(&self,deep:i32) -> String;
}


//执行子所有节点
pub fn do_child_nodes(child_nodes: &mut Vec<NodeType>, env: &mut Value, holder:&mut ConfigHolder) -> Result<String, String> {
    let mut s = String::new();
    for item in child_nodes {
        let item_result = item.eval(env, holder);
        if item_result.is_err() {
            return item_result;
        }
        s = s + item_result.unwrap().as_str();
    }
    return Result::Ok(s);
}

//TODO decode xml
pub fn loop_decode_xml(xml_vec: &Vec<Element>, holder:&ConfigHolder) -> Vec<NodeType> {
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
               id: xml.get_attr("id"),
               result_map: xml.get_attr("result_map"),
               childs: child_nodes,
           })),
           "update" => nodes.push(NodeType::NUpdateNode(UpdateNode{
               id: xml.get_attr("id"),
               childs: child_nodes,
           })),
           "insert" => nodes.push(NodeType::NInsertNode(InsertNode{
               id: xml.get_attr("id"),
               childs: child_nodes,
           })),
           "delete" => nodes.push(NodeType::NDeleteNode(DeleteNode{
               id: xml.get_attr("id"),
               childs: child_nodes,
           })),
           "if" => nodes.push(NodeType::NIf(IfNode{
               childs: child_nodes,
               test: xml.get_attr("test"),
           })),
           "trim" => nodes.push(NodeType::NTrim(TrimNode{
               childs: child_nodes,
               prefix: xml.get_attr("prefix"),
               suffix: xml.get_attr("suffix"),
               suffix_overrides: xml.get_attr("suffix_overrides"),
               prefix_overrides: xml.get_attr("prefix_overrides")
           })),
           "foreach" => nodes.push(NodeType::NForEach(ForEachNode{
               childs: child_nodes,
               collection: xml.get_attr("collection"),
               index: xml.get_attr("index"),
               item: xml.get_attr("item"),
               open: xml.get_attr("open"),
               close: xml.get_attr("close"),
               separator: xml.get_attr("separator")
           })),
           "choose" => nodes.push(NodeType::NChoose(ChooseNode{
               //todo filter nodes from child nodes
               when_nodes: filter_when_nodes(child_nodes.clone()),
               otherwise_node: filter_otherwise_nodes(child_nodes),
           })),
           "when" => nodes.push(NodeType::NWhen(WhenNode{
               childs: child_nodes,
               test: xml.get_attr("test"),
           })),
           "where" => nodes.push(NodeType::NWhere(WhereNode{
               childs: child_nodes,
           })),
           "otherwise" => nodes.push(NodeType::NOtherwise(OtherwiseNode{
               childs: child_nodes,
           })),
           "bind" => nodes.push(NodeType::NBind(BindNode{
               name: xml.get_attr("name"),
               value: xml.get_attr("value"),
           })),
           "include" => nodes.push(NodeType::NInclude(IncludeNode{
               refid: xml.get_attr("refid"),
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
        if let NodeType::NWhen(when_node) = x {
            data.push(NodeType::NWhen(when_node))
        } else {}
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
        if let NodeType::NOtherwise(node) = x {
            data.push(NodeType::NOtherwise(node))
        } else {}
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