use crate::ast::lang::py::Py;
use crate::error::RbatisError;
use std::collections::HashMap;
use crate::ast::node::node_type::NodeType;

pub struct Rbatis<'r> {
    node_type: HashMap<&'r str, Vec<NodeType>>
}


impl<'r> Rbatis<'r> {
    pub fn new() -> Rbatis<'r> {
        return Rbatis { node_type: HashMap::new() };
    }
    pub fn load_py(&mut self, name: &'r str, data: &str) -> Result<(), RbatisError> {
        let data = Py::parser(data)?;
        self.node_type.insert(name, data);
        return Ok(());
    }
}