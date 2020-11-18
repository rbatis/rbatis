use crate::ast::ast::RbatisAST;
use serde_json::Value;
use crate::engine::runtime::RbatisEngine;
use std::ptr::NonNull;
use async_std::sync::{Mutex, Arc};
use std::ops::Deref;
use crate::ast::node::node_type::NodeType;
use crate::core::Error;

///CustomNode Generate,you can custom py lang parse
pub trait CustomNodeGenerate: Send + Sync {
    ///generate return an Option<CustomNode>,if return None,parser will be skip this build
    fn generate(&self,express: &str, child_nodes: Vec<NodeType>) -> Result<Option<CustomNode>, Error>;
}

#[derive(Clone, Debug)]
pub struct CustomNode {
    pub childs: Vec<NodeType>,
    ptr: Arc<Box<dyn RbatisAST>>,
}

impl RbatisAST for CustomNode {
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        self.ptr.deref().eval(convert, env, engine, arg_array)
    }
}