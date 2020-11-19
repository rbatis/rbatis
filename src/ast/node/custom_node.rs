use std::ops::Deref;
use std::ptr::NonNull;

use crate::core::runtime::{Arc, Mutex};
use serde_json::Value;

use crate::ast::ast::RbatisAST;
use crate::ast::node::node_type::NodeType;
use crate::core::Error;
use crate::engine::runtime::RbatisEngine;

///CustomNode Generate,you can custom py lang parse
pub trait CustomNodeGenerate: Send + Sync {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    ///generate return an Option<CustomNode>,if return None,parser will be skip this build
    fn generate(&self, express: &str, child_nodes: Vec<NodeType>) -> Result<Option<CustomNode>, Error>;
}

#[derive(Clone, Debug)]
pub struct CustomNode {
    pub childs: Vec<NodeType>,
    ptr: Arc<Box<dyn RbatisAST>>,
}

impl CustomNode {
    pub fn from<T>(body: T, childs: Vec<NodeType>) -> Self where T: RbatisAST + 'static {
        Self {
            childs,
            ptr: Arc::new(Box::new(body)),
        }
    }
}

impl RbatisAST for CustomNode {
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        self.ptr.deref().eval(convert, env, engine, arg_array)
    }
}