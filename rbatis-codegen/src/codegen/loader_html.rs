use crate::error::Error;
use scraper::{Html, Node};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Element {
    pub tag: String,
    pub data: String,
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.tag.as_str() {
            "" => {
                f.write_str(&self.data)?;
            }
            _ => {
                f.write_str("<")?;
                f.write_str(&self.tag)?;
                for (k, v) in &self.attrs {
                    f.write_str(" ")?;
                    f.write_str(k)?;
                    f.write_str("=\"")?;
                    f.write_str(v)?;
                    f.write_str("\"")?;
                }
                f.write_str(">")?;
                for x in &self.childs {
                    std::fmt::Display::fmt(x, f)?;
                }
                //</if>
                f.write_str("</")?;
                f.write_str(&self.tag)?;
                f.write_str(">")?;
            }
        }
        return Ok(());
    }
}

fn collect_elements(el: scraper::ElementRef<'_>) -> Vec<Element> {
    let mut els = vec![];
    for node in el.children() {
        match node.value() {
            Node::Text(text) => {
                let txt = text.to_string();
                if txt.trim().is_empty() {
                    continue;
                }
                els.push(Element {
                    tag: String::new(),
                    data: txt,
                    attrs: HashMap::new(),
                    childs: vec![],
                });
            }
            Node::Element(elem) => {
                if let Some(child_ref) = scraper::ElementRef::wrap(node) {
                    let mut attrs = HashMap::new();
                    for (k, v) in elem.attrs() {
                        attrs.insert(k.to_string(), v.to_string());
                    }
                    els.push(Element {
                        tag: elem.name().to_string(),
                        data: String::new(),
                        attrs,
                        childs: collect_elements(child_ref),
                    });
                }
            }
            Node::Comment(_) => {}
            _ => {}
        }
    }
    els
}

pub fn load_html(html: &str) -> std::result::Result<Vec<Element>, String> {
    let document = Html::parse_fragment(html);
    Ok(collect_elements(document.root_element()))
}

impl Element {
    /// get all strings
    pub fn child_strings(&self) -> Vec<&str> {
        let mut elements = vec![];
        for x in &self.childs {
            if x.tag.eq("") {
                elements.push(x.data.as_str());
            }
            let v = x.child_strings();
            for x in v {
                elements.push(x);
            }
        }
        elements
    }
    /// get all strings
    pub fn child_string_cup(&self) -> usize {
        let mut u = 0;
        for x in self.child_strings() {
            u += x.len();
        }
        u
    }
}

/// Loads HTML content into a vector of elements
pub fn load_mapper_vec(html: &str) -> std::result::Result<Vec<Element>, Error> {
    let elements = load_html(html).map_err(|e| Error::from(e))?;

    let mut mappers = Vec::new();
    for element in elements {
        if element.tag == "mapper" {
            mappers.extend(element.childs);
        } else {
            mappers.push(element);
        }
    }

    Ok(mappers)
}
