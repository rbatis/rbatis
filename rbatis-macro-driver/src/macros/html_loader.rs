use std::collections::HashMap;
use html_parser::{Dom, Node};
use std::fmt::{Debug};

#[derive(Clone, Eq, PartialEq)]
pub struct Element {
    pub tag: String,
    pub data: String,
    pub attributes: HashMap<String, String>,
    pub childs: Vec<Element>,
}


impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("");
        match self.tag.as_str() {
            "" => {
                s.field("data", &self.data);
            }
            _ => {
                s.field("tag", &self.tag);
                if !self.attributes.is_empty() {
                    s.field("attributes", &self.attributes);
                }
                if !self.childs.is_empty() {
                    s.field("childs", &self.childs);
                }
            }
        }
        return s.finish();
    }
}


pub fn as_element(args: &Vec<Node>) -> Vec<Element> {
    let mut els = vec![];
    for x in args {
        let mut el = Element {
            tag: "".to_string(),
            data: "".to_string(),
            attributes: HashMap::new(),
            childs: vec![],
        };
        match x {
            Node::Text(txt) => {
                el.data = txt.to_string();
            }
            Node::Element(element) => {
                el.tag = element.name.to_string();
                if element.id.is_some() {
                    el.attributes.insert("id".to_string(), element.id.as_ref().unwrap_or(&String::new()).clone());
                }
                for (k, v) in &element.attributes {
                    el.attributes.insert(k.clone(), v.as_ref().unwrap_or(&String::new()).clone());
                }
                if !element.children.is_empty() {
                    let childs = as_element(&element.children);
                    el.childs = childs;
                }
            }
            Node::Comment(comment) => {
                println!("comment:{}", comment);
            }
        }
        els.push(el);
    }
    els
}

pub fn load_html(html: &str) -> html_parser::Result<Vec<Element>> {
    let dom = Dom::parse(html)?;
    let els = as_element(&dom.children);
    return Ok(els);
}
