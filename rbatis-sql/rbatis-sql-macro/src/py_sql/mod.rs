pub mod bind_node;
pub mod choose_node;
pub mod error;
pub mod foreach_node;
pub mod if_node;
pub mod impl_node;
pub mod otherwise_node;
pub mod print_node;
pub mod set_node;
pub mod string_node;
pub mod trim_node;
pub mod when_node;
pub mod where_node;

use crate::py_sql::bind_node::BindNode;
use crate::py_sql::choose_node::ChooseNode;
use crate::py_sql::foreach_node::ForEachNode;
use crate::py_sql::if_node::IfNode;
use crate::py_sql::otherwise_node::OtherwiseNode;
use crate::py_sql::print_node::PrintNode;
use crate::py_sql::set_node::SetNode;
use crate::py_sql::string_node::StringNode;
use crate::py_sql::trim_node::TrimNode;
use crate::py_sql::when_node::WhenNode;
use crate::py_sql::where_node::WhereNode;

#[derive(Clone, Debug)]
pub enum NodeType {
    NString(StringNode),
    NIf(IfNode),
    NTrim(TrimNode),
    NForEach(ForEachNode),
    NChoose(ChooseNode),
    NOtherwise(OtherwiseNode),
    NWhen(WhenNode),
    NBind(BindNode),
    NSet(SetNode),
    NWhere(WhereNode),
    NPrint(PrintNode),
}

pub trait Name {
    fn name() -> &'static str;
}

pub trait DefName {
    fn def_name() -> &'static str;
}

pub trait ParsePySql {
    fn parse(arg: &str) -> Result<Vec<NodeType>, crate::py_sql::error::Error>;
}

pub trait AsHtml {
    fn as_html(&self) -> String;
}

impl AsHtml for StringNode {
    fn as_html(&self) -> String {
        if self.value.starts_with("`") && self.value.starts_with("`"){
            self.value.to_string()
        }else{
            format!("`{}`",self.value)
        }
    }
}

impl AsHtml for IfNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!("<if test=\"{}\">{}</if>", self.test, childs)
    }
}

impl AsHtml for TrimNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!(
            "<trim prefixOverrides=\"{}\" suffixOverrides=\"{}\">{}</trim>",
            self.trim, self.trim, childs
        )
    }
}

impl AsHtml for ForEachNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!(
            "<foreach collection=\"{}\" index=\"{}\" item=\"{}\" >{}</foreach>",
            self.collection, self.index, self.item, childs
        )
    }
}

impl AsHtml for ChooseNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.when_nodes {
            childs.push_str(&x.as_html());
        }
        let mut other_html = String::new();
        match &self.otherwise_node {
            None => {}
            Some(v) => {
                other_html = v.as_html();
            }
        }
        format!("<choose>{}{}</choose>", childs, other_html)
    }
}

impl AsHtml for OtherwiseNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!("<otherwise>{}</otherwise>", childs)
    }
}

impl AsHtml for WhenNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!("<when test=\"{}\">{}</when>", self.test, childs)
    }
}

impl AsHtml for BindNode {
    fn as_html(&self) -> String {
        format!("<bind name=\"{}\" value=\"{}\"/>", self.name, self.value)
    }
}

impl AsHtml for SetNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!("<set>{}</set>", childs)
    }
}

impl AsHtml for WhereNode {
    fn as_html(&self) -> String {
        let mut childs = String::new();
        for x in &self.childs {
            childs.push_str(&x.as_html());
        }
        format!("<where>{}</where>", childs)
    }
}

impl AsHtml for PrintNode {
    fn as_html(&self) -> String {
        format!("<println value=\"{}\" />", self.value)
    }
}

impl AsHtml for NodeType {
    fn as_html(&self) -> String {
        match self {
            NodeType::NString(n) => n.as_html(),
            NodeType::NIf(n) => n.as_html(),
            NodeType::NTrim(n) => n.as_html(),
            NodeType::NForEach(n) => n.as_html(),
            NodeType::NChoose(n) => n.as_html(),
            NodeType::NOtherwise(n) => n.as_html(),
            NodeType::NWhen(n) => n.as_html(),
            NodeType::NBind(n) => n.as_html(),
            NodeType::NSet(n) => n.as_html(),
            NodeType::NWhere(n) => n.as_html(),
            NodeType::NPrint(n) => n.as_html(),
        }
    }
}

impl AsHtml for Vec<NodeType> {
    fn as_html(&self) -> String {
        let mut htmls = String::new();
        for x in self {
            htmls.push_str(&x.as_html());
        }
        htmls
    }
}

pub fn to_html(args: &Vec<NodeType>, is_select: bool, fn_name: &str) -> String {
    let htmls = args.as_html();
    if is_select {
        format!(
            "<mapper><select id=\"{}\">{}</select></mapper>",
            fn_name, htmls
        )
    } else {
        format!(
            "<mapper><update id=\"{}\">{}</update></mapper>",
            fn_name, htmls
        )
    }
}
