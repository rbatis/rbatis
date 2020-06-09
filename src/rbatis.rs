use crate::ast::lang::py::Py;
use crate::error::RbatisError;
use std::collections::HashMap;
use crate::ast::node::node_type::NodeType;
use crate::engine::runtime::RbatisEngine;
use crate::ast::lang::xml::Xml;
use crate::ast::node::insert_node::InsertNode;
use crate::ast::node::delete_node::DeleteNode;
use crate::ast::node::update_node::UpdateNode;
use crate::ast::node::select_node::SelectNode;

pub struct Rbatis<'r> {
    engine: RbatisEngine,
    /// map<mapper_name,map<method_name,NodeType>>
    mapper_node_map: HashMap<&'r str,HashMap<String, NodeType>>,
}


impl<'r> Rbatis<'r> {
    pub fn new() -> Rbatis<'r> {
        return Rbatis { mapper_node_map: HashMap::new(), engine: RbatisEngine::new() };
    }

    pub fn load_xml(&mut self, mapper_name: &'r str, data: &str) -> Result<(), RbatisError> {
        let xml = Xml::parser(data);
        self.mapper_node_map.insert(mapper_name, xml);
        return Ok(());
    }
}