use serde_json::Value;
use std::rc::Rc;
use crate::lib::RustExpressionEngine::node::NodeType;

pub struct Node2 {
    pub value: Value,
    pub leftBinaryNode: Option<Rc<Node2>>,
    pub rightBinaryNode: Option<Rc<Node2>>,
    pub nodeType: NodeType,
    pub countResulted: bool,
}


impl Node2 {
    pub fn eval(&self, env: &Value) -> Value {
        if (!self.countResulted) {
            let mut l = self.countLeft(env);
            let mut r = self.countRight(env);
            let ls = l.as_str().unwrap();
            let rs = r.as_str().unwrap();
            return Value::String(ls.to_owned() + rs);
        } else {
            return self.value.clone();
        }
    }


    pub fn countLeft(&self, env: &Value) -> Value {
        if self.leftBinaryNode.is_none() {
            return Value::Null;
        }
        return self.leftBinaryNode.clone().unwrap().eval(env);
    }
    pub fn countRight(&self, env: &Value) -> Value {
        if self.rightBinaryNode.is_none() {
            return Value::Null;
        }
        return self.rightBinaryNode.clone().unwrap().eval(env);
    }
}


#[test]
fn TestNode2() {
    let n1 = Node2 {
        value: serde_json::Value::String("1".to_string()),
        leftBinaryNode: Option::None,
        rightBinaryNode: Option::None,
        nodeType: NodeType::NString,
        countResulted: true,
    };
    let n2 = Node2 {
        value: serde_json::Value::String("1".to_string()),
        leftBinaryNode: Option::None,
        rightBinaryNode: Option::None,
        nodeType: NodeType::NString,
        countResulted: true,
    };
    let n3 = Node2 {
        value: serde_json::Value::String("1".to_string()),
        leftBinaryNode: Option::Some(Rc::new(n1)),
        rightBinaryNode: Option::Some(Rc::new(n2)),
        nodeType: NodeType::NString,
        countResulted: false,
    };


    let result = n3.eval(&Value::Null);
    println!("{}", result);
}
