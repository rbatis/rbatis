use crate::codegen::syntax_tree_pysql::{NodeType, ToHtml};

impl ToHtml for NodeType {
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
            NodeType::NContinue(n) => n.as_html(),
            NodeType::NBreak(n) => n.as_html(),
            NodeType::NSql(n) => n.as_html(),
        }
    }
}

impl ToHtml for Vec<NodeType> {
    fn as_html(&self) -> String {
        let mut htmls = String::new();
        for x in self {
            htmls.push_str(&x.as_html());
        }
        htmls
    }
}

pub fn to_html_mapper(args: &Vec<NodeType>, is_select: bool, fn_name: &str) -> String {
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
