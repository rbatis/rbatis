use html_parser::{Dom, Node, Result};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Write};

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
                f.write_str("`")?;
                f.write_str(&self.data)?;
                f.write_str("`")?;
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

pub fn as_element(args: &Vec<Node>) -> Vec<Element> {
    let mut els = vec![];
    for x in args {
        let mut el = Element {
            tag: "".to_string(),
            data: "".to_string(),
            attrs: HashMap::new(),
            childs: vec![],
        };
        match x {
            Node::Text(txt) => {
                if txt.is_empty() {
                    continue;
                }
                let t = txt.trim();
                if t.starts_with("`") || t.ends_with("`") {
                    el.data = t.replace("`", "").to_string();
                } else {
                    el.data = t.to_string();
                }
                el.data = el.data.replace("``", "").to_string();
            }
            Node::Element(element) => {
                el.tag = element.name.to_string();
                if element.id.is_some() {
                    el.attrs.insert(
                        "id".to_string(),
                        element.id.as_ref().unwrap_or(&String::new()).clone(),
                    );
                }
                for (k, v) in &element.attributes {
                    el.attrs
                        .insert(k.clone(), v.as_ref().unwrap_or(&String::new()).clone());
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

pub fn load_html(html: &str) -> Result<Vec<Element>> {
    let dom = Dom::parse(html)?;
    let els = as_element(&dom.children);
    return Ok(els);
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

#[cfg(test)]
mod test {
    use crate::codegen::html_loader::load_html;

    #[test]
    fn test_parser() {
        let nodes = load_html(r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
<mapper>
    <insert id="insert">
        'insert into biz_activity'
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            '${key}'
        </foreach>
        'values'
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            '${item}'
        </foreach>
    </insert></mapper>"#).unwrap();
        println!("{:?}", nodes);
    }

    #[test]
    fn test_item() {
        let nodes = load_html(r#"
    <insert id="insert">
        'insert into biz_activity'
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            '${key}'
        </foreach>
        'values'
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            '${item}'
        </foreach>
    </insert>"#).unwrap();
        println!("{:?}", nodes);
    }
}
