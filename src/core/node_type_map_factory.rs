use std::collections::HashMap;
use crate::ast::xml::node_type::NodeType;
use crate::utils::xml_loader::load_xml;
use crate::ast::xml::node::{loop_decode_xml};
use crate::ast::config_holder::ConfigHolder;

pub fn create_node_type_map(xml_content: String,holder:&ConfigHolder) -> HashMap<String, NodeType> {
    let nodes = load_xml(xml_content);
    let data = loop_decode_xml(&nodes, holder);
    let mut m = HashMap::new();
    for x in data {
        match x.clone() {

            NodeType::NResultMapNode(node) => m.insert(node.id, x),

            NodeType::NSelectNode(node) => m.insert(node.id, x),
            NodeType::NDeleteNode(node) => m.insert(node.id, x),
            NodeType::NUpdateNode(node) => m.insert(node.id, x),
            NodeType::NInsertNode(node) => m.insert(node.id, x),

            _ => m.insert("unknow".to_string(), NodeType::Null),
        };
    }
    return m;
}