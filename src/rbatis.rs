use crate::ast::lang::py::Py;
use crate::error::RbatisError;
use std::collections::HashMap;
use crate::ast::node::node_type::NodeType;

pub struct Rbatis<'a> {
    node_type: HashMap<&'a str, Vec<NodeType>>
}


impl<'a> Rbatis<'a> {
    pub fn new() -> Rbatis<'a> {
        return Rbatis { node_type: HashMap::new() };
    }
    pub fn load_py(&mut self, name: &'a str, data: &str) -> Result<(), RbatisError> {
        let data = Py::parser(data)?;
        self.node_type.insert(name, data);
        return Ok(());
    }
}