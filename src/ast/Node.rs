use super::NodeType::NodeType;
use std::collections::HashMap;
use serde_json::Value;

/**
* Abstract syntax tree node
*/
pub trait SqlNode {
    fn eval(&mut self, env: &Value) -> String;
}

