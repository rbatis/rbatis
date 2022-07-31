use crate::html_loader::Element;
use crate::py_sql::NodeType;
use std::collections::HashMap;


pub fn as_elements(arg: Vec<NodeType>) -> Vec<Element> {
    let mut res = vec![];
    for x in arg {
        res.push(Element::from(x));
    }
    res
}


impl From<NodeType> for Element {
    fn from(arg: NodeType) -> Self {
        match arg {
            NodeType::NString(n) => {
                return Element {
                    tag: "".to_string(),
                    data: n.value,
                    attrs: Default::default(),
                    childs: vec![],
                };
            }
            NodeType::NIf(n) => {
                let mut m = HashMap::new();
                m.insert("test".to_string(), n.test);
                return Element {
                    tag: "if".to_string(),
                    data: "".to_string(),
                    attrs: m,
                    childs: as_elements(n.childs),
                };
            }
            NodeType::NTrim(n) => {
                let mut m = HashMap::new();
                m.insert("trim".to_string(), n.trim);
                return Element {
                    tag: "trim".to_string(),
                    data: "".to_string(),
                    attrs: m,
                    childs: as_elements(n.childs),
                };
            }
            NodeType::NForEach(n) => {
                let mut m = HashMap::new();
                m.insert("collection".to_string(), n.collection);
                m.insert("index".to_string(), n.index);
                m.insert("item".to_string(), n.item);
                return Element {
                    tag: "foreach".to_string(),
                    data: "".to_string(),
                    attrs: m,
                    childs: as_elements(n.childs),
                };
            }
            NodeType::NChoose(n) => {
                let mut whens = as_elements(n.when_nodes);
                if let Some(v) = n.otherwise_node {
                    whens.push(Element::from(*v));
                }
                return Element {
                    tag: "choose".to_string(),
                    data: "".to_string(),
                    attrs: Default::default(),
                    childs: whens,
                };
            }
            NodeType::NOtherwise(n) => {
                return Element {
                    tag: "otherwise".to_string(),
                    data: "".to_string(),
                    attrs: Default::default(),
                    childs: as_elements(n.childs),
                };
            }
            NodeType::NWhen(n) => {
                let mut m = HashMap::new();
                m.insert("test".to_string(), n.test);
                return Element {
                    tag: "when".to_string(),
                    data: "".to_string(),
                    attrs: m,
                    childs: as_elements(n.childs),
                };
            }
            NodeType::NBind(n) => {
                let mut m = HashMap::new();
                m.insert("name".to_string(), n.name);
                m.insert("value".to_string(), n.value);
                return Element {
                    tag: "bind".to_string(),
                    data: "".to_string(),
                    attrs: m,
                    childs: vec![],
                };
            }
            NodeType::NSet(n) => {
                return Element {
                    tag: "set".to_string(),
                    data: "".to_string(),
                    attrs: Default::default(),
                    childs: as_elements(n.childs),
                };
            }
            NodeType::NWhere(n) => {
                return Element {
                    tag: "where".to_string(),
                    data: "".to_string(),
                    attrs: Default::default(),
                    childs: as_elements(n.childs),
                };
            }
            NodeType::NPrint(n) => {
                return Element {
                    tag: "println".to_string(),
                    data: "".to_string(),
                    attrs: Default::default(),
                    childs: vec![],
                };
            }
        }
    }
}