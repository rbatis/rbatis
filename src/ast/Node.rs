use super::NodeType::NodeType;
use super::NodeString::NodeString;
use std::collections::HashMap;

/**
* Abstract syntax tree node
*/
pub trait Node {
    //return node type
    fn Type(&self) -> NodeType;
    //run node impl,and return result string
    fn Eval(&self, mut arg: &HashMap<&str, String>) -> String;
}
