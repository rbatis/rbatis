//use super::{Node::Node,NodeType::NodeType};

use super::Node::Node;
use super::NodeType::NodeType;
use std::collections::HashMap;

pub struct NodeString {
    pub buf: String,
}

impl Node for NodeString {
    fn Type(&self) -> NodeType {
        return NodeType::String;
    }

    fn Eval(&self,  mut arg: &HashMap<&str, String>) -> String {
        let mut s = String::new();
        s.push_str(&"sadf");
        return s;
    }
}

