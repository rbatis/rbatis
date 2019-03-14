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

    fn Eval(&self,   arg: &mut HashMap<&str, String>) -> String {
        let mut s = String::new();
        s.push_str(&self.buf);
        return s;
    }
}



#[test]
pub fn Test_NodeString(){

    let nodeString=NodeString{buf:String::from("sdafsdf")};
    let mut m=HashMap::new();
    m.insert("adsf",String::from("dsaf"));
    let newStr=nodeString.Eval(&mut m);
    println!("{}",newStr);
}