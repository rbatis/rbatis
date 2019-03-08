use super::NodeType::NodeType;
use super::NodeString::NodeString;
use std::collections::HashMap;

/**
* 抽象语法树节点
*/
pub trait Node {
    fn Type(&self) -> NodeType;
    fn Eval(&self, mut arg: &HashMap<&str, String>) -> String;
}
