//use super::{Node::Node,NodeType::NodeType};

use crate::ast::Node::Node;
use crate::ast::NodeType::NodeType;
use std::collections::HashMap;

pub struct NodeString {
    pub buf: String,
}

impl Node for NodeString {
    fn Type(&self) -> NodeType {
        return NodeType::String;
    }

    fn Eval(&self,  mut arg: &HashMap<&str, String>) -> String {
        //  self.buf.replace()

//          for (k,v)in arg{
//          }

        let mut s = String::new();
        s.push_str(&"sadf");
        return s;
    }
}

impl NodeString {
    pub fn ToString(&self) -> String {
        let mut s = String::new();
        s.push_str("NodeString");
        return s;
    }
}

