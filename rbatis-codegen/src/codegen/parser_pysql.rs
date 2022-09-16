use crate::codegen::parser_html::parse_html_str;
use crate::codegen::proc_macro::TokenStream;
use crate::codegen::syntax_tree::bind_node::BindNode;
use crate::codegen::syntax_tree::choose_node::ChooseNode;
use crate::codegen::syntax_tree::continue_node::ContinueNode;
use crate::codegen::syntax_tree::error::Error;
use crate::codegen::syntax_tree::foreach_node::ForEachNode;
use crate::codegen::syntax_tree::if_node::IfNode;
use crate::codegen::syntax_tree::otherwise_node::OtherwiseNode;
use crate::codegen::syntax_tree::set_node::SetNode;
use crate::codegen::syntax_tree::string_node::StringNode;
use crate::codegen::syntax_tree::trim_node::TrimNode;
use crate::codegen::syntax_tree::when_node::WhenNode;
use crate::codegen::syntax_tree::where_node::WhereNode;
use crate::codegen::syntax_tree::{DefaultName, Name, NodeType};
use quote::ToTokens;
use std::collections::HashMap;
use syn::{AttributeArgs, ItemFn};

pub trait ParsePySql {
    fn parse_pysql(arg: &str) -> Result<Vec<NodeType>, crate::codegen::syntax_tree::error::Error>;
}

pub fn impl_fn_py(m: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let fn_name = m.sig.ident.to_string();
    let mut data = args.get(0).to_token_stream().to_string();
    if data.ne("\"\"") && data.starts_with("\"") && data.ends_with("\"") {
        data = data[1..data.len() - 1].to_string();
    }
    data = data.replace("\\n", "\n");
    let t;
    let mut format_char = '?';
    if args.len() > 1 {
        for x in args.get(1).to_token_stream().to_string().chars() {
            if x != '\'' && x != '"' {
                format_char = x;
                break;
            }
        }
    }
    let nodes = NodeType::parse_pysql(&data).expect("[rbatis] parse py_sql fail!");
    let htmls = crate::codegen::syntax_tree::to_html(
        &nodes,
        data.starts_with("select") || data.starts_with(" select"),
        &fn_name,
    );
    t = parse_html_str(&htmls, &fn_name, &mut vec![]);
    return t.into();
}

impl ParsePySql for NodeType {
    //TODO maybe this use Rust parser crates？
    fn parse_pysql(arg: &str) -> Result<Vec<NodeType>, Error> {
        let line_space_map = Self::create_line_space_map(&arg);
        let mut main_node = vec![];
        let ls = arg.lines();
        let mut space = -1;
        let mut line = -1;
        let mut skip = -1;
        for x in ls {
            line += 1;
            if x.is_empty() || (skip != -1 && line <= skip) {
                continue;
            }
            let count_index = *line_space_map
                .get(&line)
                .ok_or_else(|| Error::from(format!("line_space_map not heve line:{}", line)))?;
            if space == -1 {
                space = count_index;
            }
            let (child_str, do_skip) =
                Self::find_child_str(line, count_index, arg, &line_space_map);
            if do_skip != -1 && do_skip >= skip {
                skip = do_skip;
            }
            let parserd;
            if !child_str.is_empty() {
                parserd = Self::parse_pysql(child_str.as_str())?;
            } else {
                parserd = vec![];
            }
            Self::parse_pysql_node(
                &mut main_node,
                x,
                *line_space_map
                    .get(&line)
                    .ok_or_else(|| Error::from(format!("line:{} not exist！", line)))?
                    as usize,
                parserd,
            )?;
        }
        return Ok(main_node);
    }
}

impl NodeType {
    fn parse_pysql_node(
        main_node: &mut Vec<NodeType>,
        x: &str,
        space: usize,
        mut childs: Vec<NodeType>,
    ) -> Result<(), Error> {
        let mut trim_x = x.trim();
        if trim_x.starts_with("//") {
            return Ok(());
        }
        if trim_x.ends_with(":") {
            trim_x = trim_x[0..trim_x.len() - 1].trim();
            if trim_x.contains(": ") {
                let vecs: Vec<&str> = trim_x.split(": ").collect();
                if vecs.len() > 1 {
                    let len = vecs.len();
                    for index in 0..len {
                        let index = len - 1 - index;
                        let item = vecs[index];
                        childs = vec![Self::parse_trim_node(item, x, childs)?];
                        if index == 0 {
                            for x in &childs {
                                main_node.push(x.clone());
                            }
                            return Ok(());
                        }
                    }
                }
            }
            let node = Self::parse_trim_node(trim_x, x, childs)?;
            main_node.push(node);
            return Ok(());
        } else {
            //string,replace space to only one
            let mut data = x.to_owned();
            if space <= 1 {
                data = x.to_string();
            } else {
                data = x[(space - 1)..].to_string();
            }
            main_node.push(NodeType::NString(StringNode {
                value: {
                    let t = data.trim();
                    if t.starts_with("`") && t.ends_with("`") {
                        t.trim_start_matches("`").trim_end_matches("`").to_string()
                    } else {
                        t.to_string()
                    }
                },
            }));
            for x in childs {
                main_node.push(x);
            }
            return Ok(());
        }
    }

    fn count_space(arg: &str) -> i32 {
        let cs = arg.chars();
        let mut index = 0;
        for x in cs {
            match x {
                ' ' => {
                    index += 1;
                }
                _ => {
                    break;
                }
            }
        }
        return index;
    }

    ///find_child_str
    fn find_child_str(
        line_index: i32,
        space_index: i32,
        arg: &str,
        m: &HashMap<i32, i32>,
    ) -> (String, i32) {
        let mut result = String::new();
        let mut skip_line = -1;
        let mut line = -1;
        let lines = arg.lines();
        for x in lines {
            line += 1;
            if line > line_index {
                let cached_space = *m.get(&line).expect("line not exists");
                if cached_space > space_index {
                    result = result + x + "\n";
                    skip_line = line;
                } else {
                    break;
                }
            }
        }
        return (result, skip_line);
    }

    ///Map<line,space>
    fn create_line_space_map(arg: &str) -> HashMap<i32, i32> {
        let mut m = HashMap::new();
        let lines = arg.lines();
        let mut line = -1;
        for x in lines {
            line += 1;
            let space = Self::count_space(x);
            //dothing
            m.insert(line, space);
        }
        return m;
    }

    fn parse_trim_node(
        trim_express: &str,
        source_str: &str,
        childs: Vec<NodeType>,
    ) -> Result<NodeType, Error> {
        if trim_express.starts_with(IfNode::name()) {
            return Ok(NodeType::NIf(IfNode {
                childs,
                test: trim_express.trim_start_matches("if ").to_string(),
            }));
        } else if trim_express.starts_with(ForEachNode::name()) {
            let for_tag = "for";
            if !trim_express.starts_with(for_tag) {
                return Err(Error::from(
                    "[rbatis] parser express fail:".to_string() + source_str,
                ));
            }
            let in_tag = " in ";
            if !trim_express.contains(in_tag) {
                return Err(Error::from(
                    "[rbatis] parser express fail:".to_string() + source_str,
                ));
            }
            let in_index = trim_express
                .find(in_tag)
                .ok_or_else(|| Error::from(format!("{} not have {}", trim_express, in_tag)))?;
            let col = trim_express[in_index + in_tag.len()..].trim();
            let mut item = trim_express[for_tag.len()..in_index].trim();
            let mut index = "";
            if item.contains(",") {
                let splits: Vec<&str> = item.split(",").collect();
                if splits.len() != 2 {
                    panic!("[rbatis_codegen] for node must be 'for key,item in col:'");
                }
                index = splits[0];
                item = splits[1];
            }
            return Ok(NodeType::NForEach(ForEachNode {
                childs,
                collection: col.to_string(),
                index: index.to_string(),
                item: item.to_string(),
            }));
        } else if trim_express.starts_with(TrimNode::name()) {
            let trim_express = trim_express.trim().trim_start_matches("trim ").trim();
            if trim_express.starts_with("'") && trim_express.ends_with("'") {
                let express = trim_express[1..trim_express.len() - 1].trim();
                return Ok(NodeType::NTrim(TrimNode {
                    childs,
                    trim: express.to_string(),
                }));
            } else {
                return Err(Error::from(format!("[rbatis] express trim value must be string value, for example:  trim 'value',error express: {}", trim_express)));
            }
        } else if trim_express.starts_with(ChooseNode::name()) {
            let mut node = ChooseNode {
                when_nodes: vec![],
                otherwise_node: None,
            };
            for x in childs {
                match x {
                    NodeType::NWhen(_) => {
                        node.when_nodes.push(x);
                    }
                    NodeType::NOtherwise(_) => {
                        node.otherwise_node = Some(Box::new(x));
                    }
                    _ => {
                        return Err(Error::from("[rbatis] parser node fail,choose node' child must be when and otherwise nodes!".to_string()));
                    }
                }
            }
            return Ok(NodeType::NChoose(node));
        } else if trim_express.starts_with(OtherwiseNode::default_name())
            || trim_express.starts_with(OtherwiseNode::name())
        {
            return Ok(NodeType::NOtherwise(OtherwiseNode { childs }));
        } else if trim_express.starts_with(WhenNode::name()) {
            let trim_express = trim_express[WhenNode::name().len()..].trim();
            return Ok(NodeType::NWhen(WhenNode {
                childs,
                test: trim_express.to_string(),
            }));
        } else if trim_express.starts_with(BindNode::default_name())
            || trim_express.starts_with(BindNode::name())
        {
            let mut express = "";
            if trim_express.starts_with(BindNode::default_name()) {
                express = trim_express[BindNode::default_name().len()..].trim();
            } else {
                express = trim_express[BindNode::name().len()..].trim();
            }
            let name_value: Vec<&str> = express.split("=").collect();
            if name_value.len() != 2 {
                return Err(Error::from(
                    "[rbatis] parser bind express fail:".to_string() + trim_express,
                ));
            }
            return Ok(NodeType::NBind(BindNode {
                name: name_value[0].to_owned(),
                value: name_value[1].to_owned(),
            }));
        } else if trim_express.starts_with(SetNode::name()) {
            return Ok(NodeType::NSet(SetNode { childs }));
        } else if trim_express.starts_with(WhereNode::name()) {
            return Ok(NodeType::NWhere(WhereNode { childs }));
        } else if trim_express.starts_with(ContinueNode::name()) {
            return Ok(NodeType::NContinue(ContinueNode {}));
        } else {
            // unkonw tag
            return Err(Error::from(
                "[rbatis] unknow tag: ".to_string() + source_str,
            ));
        }
    }
}
