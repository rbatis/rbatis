use std::collections::HashMap;
use crate::ast::node::node_type::NodeType;
use crate::utils::xml_loader::{load_xml, Element};
use crate::ast::config_holder::ConfigHolder;
use crate::ast::node::select_node::SelectNode;
use crate::ast::node::insert_node::InsertNode;
use crate::ast::node::if_node::IfNode;
use crate::ast::node::trim_node::TrimNode;
use crate::ast::node::foreach_node::ForEachNode;
use crate::ast::node::choose_node::ChooseNode;
use crate::ast::node::when_node::WhenNode;
use crate::ast::node::where_node::WhereNode;
use crate::ast::node::otherwise_node::OtherwiseNode;
use crate::ast::node::include_node::IncludeNode;
use crate::ast::node::set_node::SetNode;
use crate::ast::node::result_map_id_node::ResultMapIdNode;
use crate::ast::node::result_map_result_node::ResultMapResultNode;
use crate::ast::node::result_map_node::ResultMapNode;
use crate::ast::node::string_node::StringNode;
use crate::ast::node::bind_node::BindNode;
use crate::ast::node::delete_node::DeleteNode;
use crate::ast::node::update_node::UpdateNode;

pub fn parser(xml_content: String, holder:&ConfigHolder) -> HashMap<String, NodeType> {
    let nodes = load_xml(xml_content);
    let data = loop_decode_xml(&nodes, holder);
    let mut m = HashMap::new();
    for x in &data {
        match x{
            NodeType::NResultMapNode(node) => m.insert(node.id.clone(), x.clone()),
            _ => {
                continue;
            }
        };
    }
    for x in &data {
        match x{
            NodeType::NSelectNode(node) => m.insert(node.id.clone(), x.clone()),
            NodeType::NDeleteNode(node) => m.insert(node.id.clone(), x.clone()),
            NodeType::NUpdateNode(node) => m.insert(node.id.clone(), x.clone()),
            NodeType::NInsertNode(node) => m.insert(node.id.clone(), x.clone()),
            _ => m.insert("unknow".to_string(), NodeType::Null),
        };

    }
    return m;
}

pub fn loop_decode_xml(xml_vec: &Vec<Element>, holder: &ConfigHolder) -> Vec<NodeType> {
    let mut nodes = vec![];
    for xml in xml_vec {
        let child_nodes;
        if xml.childs.len() > 0 {
            child_nodes = loop_decode_xml(&(&xml).childs, holder);
        } else {
            child_nodes = vec![];
        }
        let tag_str = xml.tag.as_str();
        //println!("tag_str:{}",tag_str);
        match tag_str {
            "mapper" => {
                //mapper 不做处理，直接返回子节点
                return child_nodes;
            }
            "select" => nodes.push(NodeType::NSelectNode(SelectNode {
                id: xml.get_attr("id"),
                childs: child_nodes,
            })),
            "update" => nodes.push(NodeType::NUpdateNode(UpdateNode {
                id: xml.get_attr("id"),
                childs: child_nodes,
            })),
            "insert" => nodes.push(NodeType::NInsertNode(InsertNode {
                id: xml.get_attr("id"),
                childs: child_nodes,
            })),
            "delete" => nodes.push(NodeType::NDeleteNode(DeleteNode {
                id: xml.get_attr("id"),
                childs: child_nodes,
            })),
            "if" => nodes.push(NodeType::NIf(IfNode {
                childs: child_nodes,
                test: xml.get_attr("test"),
            })),
            "trim" => nodes.push(NodeType::NTrim(TrimNode {
                childs: child_nodes,
                prefix: xml.get_attr("prefix"),
                suffix: xml.get_attr("suffix"),
                suffix_overrides: xml.get_attr("suffix_overrides"),
                prefix_overrides: xml.get_attr("prefix_overrides"),
            })),
            "foreach" => nodes.push(NodeType::NForEach(ForEachNode {
                childs: child_nodes,
                collection: xml.get_attr("collection"),
                index: xml.get_attr("index"),
                item: xml.get_attr("item"),
                open: xml.get_attr("open"),
                close: xml.get_attr("close"),
                separator: xml.get_attr("separator"),
            })),
            "choose" => nodes.push(NodeType::NChoose(ChooseNode {
                when_nodes: filter_when_nodes(&child_nodes),
                otherwise_node: filter_otherwise_nodes(child_nodes),
            })),
            "when" => nodes.push(NodeType::NWhen(WhenNode {
                childs: child_nodes,
                test: xml.get_attr("test"),
            })),
            "where" => nodes.push(NodeType::NWhere(WhereNode {
                childs: child_nodes,
            })),
            "otherwise" => nodes.push(NodeType::NOtherwise(OtherwiseNode {
                childs: child_nodes,
            })),
            "bind" => nodes.push(NodeType::NBind(BindNode {
                name: xml.get_attr("name"),
                value: xml.get_attr("value"),
            })),
            "include" => nodes.push(NodeType::NInclude(IncludeNode {
                refid: xml.get_attr("refid"),
                childs: child_nodes,
            })),
            "set" => nodes.push(NodeType::NSet(SetNode {
                childs: child_nodes,
            })),

            "id" => nodes.push(NodeType::NResultMapIdNode(ResultMapIdNode {
                column: xml.get_attr("column"),
                property: xml.get_attr("property"),
                lang_type: xml.get_attr("lang_type"),
            })),

            "result" => nodes.push(NodeType::NResultMapResultNode(ResultMapResultNode {
                column: xml.get_attr("column"),
                property: xml.get_attr("property"),
                lang_type: xml.get_attr("lang_type"),
                version_enable: xml.get_attr("version_enable"),
                logic_enable: xml.get_attr("logic_enable"),
                logic_undelete: xml.get_attr("logic_undelete"),
                logic_deleted: xml.get_attr("logic_deleted"),
            })),

            "result_map" => nodes.push(NodeType::NResultMapNode(ResultMapNode::new( xml.get_attr("id"),
                                                                                    xml.get_attr("table"),
                                                                                    filter_result_map_id_nodes(&child_nodes),
                                                                                    filter_result_map_result_nodes(&child_nodes),))),
            "" => {
                let data = xml.data.as_str();
                let tag = xml.tag.as_str();
                let n = StringNode::new(data);
                nodes.push(NodeType::NString(n));
            }
            _ => {}
        }
    }
    return nodes;
}


pub fn filter_result_map_result_nodes(arg: &Vec<NodeType>) -> Vec<ResultMapResultNode> {
    let mut data = vec![];
    for x in arg {
        if let NodeType::NResultMapResultNode(result_node) = x {
            data.push(result_node.clone());
        }
    }
    return data;
}

pub fn filter_result_map_id_nodes(arg: &Vec<NodeType>) -> Option<ResultMapIdNode> {
    for x in arg {
        if let NodeType::NResultMapIdNode(id_node) = x {
            return Option::Some(id_node.clone());
        }
    }
    return Option::None;
}

pub fn filter_when_nodes(arg: &Vec<NodeType>) -> Option<Vec<NodeType>> {
    let mut data = vec![];
    for x in arg {
        if let NodeType::NWhen(when_node) = x {
            data.push(NodeType::NWhen(when_node.clone()))
        } else {}
    }
    if data.len() == 0 {
        return Option::None;
    } else {
        return Some(data);
    }
}


pub fn filter_otherwise_nodes(arg: Vec<NodeType>) -> Option<Box<NodeType>> {
    let mut data = vec![];
    for x in arg {
        if let NodeType::NOtherwise(node) = x {
            data.push(NodeType::NOtherwise(node))
        } else {}
    }
    if data.len() > 0 {
        if data.len() > 1 {
            panic!("otherwise_nodes length can not > 1;")
        }
        let d0 = data[0].clone();
        return Option::Some(Box::new(d0));
    } else {
        return Option::None;
    }
}


