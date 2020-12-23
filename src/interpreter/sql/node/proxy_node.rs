use std::ops::Deref;
use std::ptr::NonNull;

use serde::export::fmt::Debug;
use serde_json::Value;

use crate::core::Error;
use crate::core::runtime::{Arc, Mutex};
use crate::interpreter::expr::runtime::ExprRuntime;
use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node_type::NodeType;

///CustomNode Generate,you can custom py lang parse
pub trait CustomNodeGenerate: Send + Sync + Debug {
    ///generate return an Option<CustomNode>,if return None,parser will be skip this build
    fn generate(&self, express: &str, child_nodes: Vec<NodeType>) -> Result<Option<ProxyNode>, Error>;
}

#[derive(Clone, Debug)]
pub struct ProxyNode {
    pub childs: Vec<NodeType>,
    ptr: Arc<Box<dyn RbatisAST>>,
}

impl ProxyNode {
    pub fn from<T>(body: T, childs: Vec<NodeType>) -> Self where T: RbatisAST + 'static {
        Self {
            childs,
            ptr: Arc::new(Box::new(body)),
        }
    }
}

impl RbatisAST for ProxyNode {
    fn name() -> &'static str where Self: Sized {
        "proxy"
    }

    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &ExprRuntime, arg_array: &mut Vec<Value>, arg_sql: &mut String) -> Result<serde_json::Value, crate::core::Error> {
        self.ptr.deref().eval(convert, env, engine, arg_array, arg_sql)
    }
}