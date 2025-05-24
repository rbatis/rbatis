use crate::codegen::parser_html::parse_html;
use crate::codegen::proc_macro::TokenStream;
use crate::codegen::syntax_tree_pysql::{
    bind_node::BindNode, break_node::BreakNode, choose_node::ChooseNode, continue_node::ContinueNode,
    error::Error, foreach_node::ForEachNode, if_node::IfNode, otherwise_node::OtherwiseNode,
    set_node::SetNode, sql_node::SqlNode, string_node::StringNode, trim_node::TrimNode,
    when_node::WhenNode, where_node::WhereNode, DefaultName, Name, NodeType,
};
use crate::codegen::ParseArgs;
use quote::ToTokens;
use std::collections::HashMap;
use syn::ItemFn;

///A handwritten recursive descent algorithm for parsing PySQL
pub trait ParsePySql {
    fn parse_pysql(arg: &str) -> Result<Vec<NodeType>, Error>;
}

pub fn impl_fn_py(m: &ItemFn, args: &ParseArgs) -> TokenStream {
    let fn_name = m.sig.ident.to_string();

    let mut data = args.sqls.iter()
        .map(|x| x.to_token_stream().to_string())
        .collect::<String>();

    if data.ne("\"\"") && data.starts_with('"') && data.ends_with('"') {
        data = data[1..data.len() - 1].to_string();
    }

    data = data.replace("\\n", "\n");

    let nodes = NodeType::parse_pysql(&data)
        .expect("[rbatis-codegen] parse py_sql fail!");

    let is_select = data.starts_with("select") || data.starts_with(" select");
    let htmls = crate::codegen::syntax_tree_pysql::to_html(&nodes, is_select, &fn_name);

    parse_html(&htmls, &fn_name, &mut vec![]).into()
}

impl ParsePySql for NodeType {
    fn parse_pysql(arg: &str) -> Result<Vec<NodeType>, Error> {
        let line_space_map = Self::create_line_space_map(arg);
        let mut main_node = Vec::new();
        let mut space = -1;
        let mut line = -1;
        let mut skip = -1;

        for x in arg.lines() {
            line += 1;

            if x.is_empty() || (skip != -1 && line <= skip) {
                continue;
            }

            let count_index = *line_space_map
                .get(&line)
                .ok_or_else(|| Error::from(format!("line_space_map not have line:{}", line)))?;

            if space == -1 {
                space = count_index;
            }

            let (child_str, do_skip) = Self::find_child_str(line, count_index, arg, &line_space_map);
            if do_skip != -1 && do_skip >= skip {
                skip = do_skip;
            }

            let parsed = if !child_str.is_empty() {
                Self::parse_pysql(&child_str)?
            } else {
                vec![]
            };

            let current_space = *line_space_map
                .get(&line)
                .ok_or_else(|| Error::from(format!("line:{} not exist!", line)))?;

            Self::parse(&mut main_node, x, current_space as usize, parsed)?;
        }

        Ok(main_node)
    }
}

impl NodeType {
    fn parse(
        main_node: &mut Vec<NodeType>,
        line: &str,
        space: usize,
        mut childs: Vec<NodeType>,
    ) -> Result<(), Error> {
        let mut trim_line = line.trim();

        if trim_line.starts_with("//") {
            return Ok(());
        }

        if trim_line.ends_with(':') {
            trim_line = trim_line[..trim_line.len() - 1].trim();

            if trim_line.contains(": ") {
                let parts: Vec<&str> = trim_line.split(": ").collect();
                if parts.len() > 1 {
                    for index in (0..parts.len()).rev() {
                        let item = parts[index];
                        childs = vec![Self::parse_node(item, line, childs)?];

                        if index == 0 {
                            main_node.extend(childs);
                            return Ok(());
                        }
                    }
                }
            }

            let node = Self::parse_node(trim_line, line, childs)?;
            main_node.push(node);
        } else {
            let data = if space <= 1 {
                line.to_string()
            } else {
                line[(space - 1)..].to_string()
            };

            main_node.push(NodeType::NString(StringNode {
                value: data.trim().to_string(),
            }));
            main_node.extend(childs);
        }

        Ok(())
    }

    fn count_space(arg: &str) -> i32 {
        arg.chars()
            .take_while(|&c| c == ' ')
            .count() as i32
    }

    fn find_child_str(
        line_index: i32,
        space_index: i32,
        arg: &str,
        line_space_map: &HashMap<i32, i32>,
    ) -> (String, i32) {
        let mut result = String::new();
        let mut skip_line = -1;
        let mut current_line = -1;

        for line in arg.lines() {
            current_line += 1;

            if current_line > line_index {
                let cached_space = *line_space_map.get(&current_line).expect("line not exists");

                if cached_space > space_index {
                    result.push_str(line);
                    result.push('\n');
                    skip_line = current_line;
                } else {
                    break;
                }
            }
        }

        (result, skip_line)
    }

    fn create_line_space_map(arg: &str) -> HashMap<i32, i32> {
        arg.lines()
            .enumerate()
            .map(|(i, line)| (i as i32, Self::count_space(line)))
            .collect()
    }

    fn parse_node(
        trim_express: &str,
        source_str: &str,
        childs: Vec<NodeType>,
    ) -> Result<NodeType, Error> {
        match trim_express {
            s if s.starts_with(IfNode::name()) => Ok(NodeType::NIf(IfNode {
                childs,
                test: s.trim_start_matches("if ").to_string(),
            })),

            s if s.starts_with(ForEachNode::name()) => Self::parse_for_each_node(s, source_str, childs),

            s if s.starts_with(TrimNode::name()) => Self::parse_trim_tag_node(s, source_str, childs),

            s if s.starts_with(ChooseNode::name()) => Self::parse_choose_node(childs),

            s if s.starts_with(OtherwiseNode::default_name()) || s.starts_with(OtherwiseNode::name()) => {
                Ok(NodeType::NOtherwise(OtherwiseNode { childs }))
            }

            s if s.starts_with(WhenNode::name()) => Ok(NodeType::NWhen(WhenNode {
                childs,
                test: s[WhenNode::name().len()..].trim().to_string(),
            })),

            s if s.starts_with(BindNode::default_name()) || s.starts_with(BindNode::name()) => {
                Self::parse_bind_node(s)
            }

            s if s.starts_with(SetNode::name()) => Self::parse_set_node(s,source_str,childs),

            s if s.starts_with(WhereNode::name()) => Ok(NodeType::NWhere(WhereNode { childs })),

            s if s.starts_with(ContinueNode::name()) => Ok(NodeType::NContinue(ContinueNode {})),

            s if s.starts_with(BreakNode::name()) => Ok(NodeType::NBreak(BreakNode {})),

            s if s.starts_with(SqlNode::name()) => Self::parse_sql_node(s, childs),

            _ => Err(Error::from("[rbatis-codegen] unknown tag: ".to_string() + source_str)),
        }
    }

    fn parse_for_each_node(express: &str, source_str: &str, childs: Vec<NodeType>) -> Result<NodeType, Error> {
        const FOR_TAG: &str = "for";
        const IN_TAG: &str = " in ";

        if !express.starts_with(FOR_TAG) {
            return Err(Error::from("[rbatis-codegen] parser express fail:".to_string() + source_str));
        }

        if !express.contains(IN_TAG) {
            return Err(Error::from("[rbatis-codegen] parser express fail:".to_string() + source_str));
        }

        let in_index = express.find(IN_TAG)
            .ok_or_else(|| Error::from(format!("{} not have {}", express, IN_TAG)))?;

        let col = express[in_index + IN_TAG.len()..].trim();
        let mut item = express[FOR_TAG.len()..in_index].trim();
        let mut index = "";

        if item.contains(',') {
            let splits: Vec<&str> = item.split(',').collect();
            if splits.len() != 2 {
                panic!("[rbatis-codegen_codegen] for node must be 'for key,item in col:'");
            }
            index = splits[0].trim();
            item = splits[1].trim();
        }

        Ok(NodeType::NForEach(ForEachNode {
            childs,
            collection: col.to_string(),
            index: index.to_string(),
            item: item.to_string(),
        }))
    }

    fn parse_trim_tag_node(express: &str, _source_str: &str, childs: Vec<NodeType>) -> Result<NodeType, Error> {
        let trim_express = express.trim().trim_start_matches("trim ").trim();

        if (trim_express.starts_with('\'') && trim_express.ends_with('\'')) ||
            (trim_express.starts_with('`') && trim_express.ends_with('`'))
        {
            let trimmed = if trim_express.starts_with('`') {
                trim_express.trim_matches('`')
            } else {
                trim_express.trim_matches('\'')
            };

            Ok(NodeType::NTrim(TrimNode {
                childs,
                start: trimmed.to_string(),
                end: trimmed.to_string(),
            }))
        } else if trim_express.contains('=') || trim_express.contains(',') {
            let mut prefix = "";
            let mut suffix = "";

            for expr in trim_express.split(',') {
                let expr = expr.trim();
                if expr.starts_with("start") {
                    prefix = expr.trim_start_matches("start")
                        .trim()
                        .trim_start_matches('=')
                        .trim()
                        .trim_matches(|c| c == '\'' || c == '`');
                } else if expr.starts_with("end") {
                    suffix = expr.trim_start_matches("end")
                        .trim()
                        .trim_start_matches('=')
                        .trim()
                        .trim_matches(|c| c == '\'' || c == '`');
                } else {
                    return Err(Error::from(format!(
                        "[rbatis-codegen] express trim node error, for example  trim 'value':  \
                        trim start='value': trim start='value',end='value':   express = {}",
                        trim_express
                    )));
                }
            }

            Ok(NodeType::NTrim(TrimNode {
                childs,
                start: prefix.to_string(),
                end: suffix.to_string(),
            }))
        } else {
            Err(Error::from(format!(
                "[rbatis-codegen] express trim node error, for example  trim 'value':  \
                trim start='value': trim start='value',end='value':   error express = {}",
                trim_express
            )))
        }
    }

    fn parse_choose_node(childs: Vec<NodeType>) -> Result<NodeType, Error> {
        let mut node = ChooseNode {
            when_nodes: vec![],
            otherwise_node: None,
        };

        for child in childs {
            match child {
                NodeType::NWhen(_) => node.when_nodes.push(child),
                NodeType::NOtherwise(_) => node.otherwise_node = Some(Box::new(child)),
                _ => return Err(Error::from(
                    "[rbatis-codegen] parser node fail,choose node' child must be when and otherwise nodes!".to_string()
                )),
            }
        }

        Ok(NodeType::NChoose(node))
    }

    fn parse_bind_node(express: &str) -> Result<NodeType, Error> {
        let expr = if express.starts_with(BindNode::default_name()) {
            express[BindNode::default_name().len()..].trim()
        } else {
            express[BindNode::name().len()..].trim()
        };

        let parts: Vec<&str> = expr.split('=').collect();
        if parts.len() != 2 {
            return Err(Error::from(
                "[rbatis-codegen] parser bind express fail:".to_string() + express,
            ));
        }

        Ok(NodeType::NBind(BindNode {
            name: parts[0].trim().to_string(),
            value: parts[1].trim().to_string(),
        }))
    }

    fn parse_sql_node(express: &str, childs: Vec<NodeType>) -> Result<NodeType, Error> {
        let expr = express[SqlNode::name().len()..].trim();

        if !expr.starts_with("id=") {
            return Err(Error::from(
                "[rbatis-codegen] parser sql express fail, need id param:".to_string() + express,
            ));
        }

        let id_value = expr.trim_start_matches("id=").trim();

        let id = if (id_value.starts_with('\'') && id_value.ends_with('\'')) ||
            (id_value.starts_with('"') && id_value.ends_with('"'))
        {
            id_value[1..id_value.len() - 1].to_string()
        } else {
            return Err(Error::from(
                "[rbatis-codegen] parser sql id value need quotes:".to_string() + express,
            ));
        };

        Ok(NodeType::NSql(SqlNode { childs, id }))
    }

    fn strip_quotes_for_attr(s: &str) -> String {
        let val = s.trim(); // Trim whitespace around the value first
        if val.starts_with('\'') && val.ends_with('\'') ||
           (val.starts_with('"') && val.ends_with('"')) {
            if val.len() >= 2 {
                return val[1..val.len()-1].to_string();
            }
        }
        val.to_string() // Return the trimmed string if no quotes or malformed quotes
    }
    
    fn parse_set_node(express: &str, source_str: &str,  childs: Vec<NodeType>) -> Result<NodeType, Error>  {
        let actual_attrs_str = if express.starts_with(SetNode::name()) {
            express[SetNode::name().len()..].trim()
        } else {
            // This case should ideally not happen if called correctly from the match arm
            return Err(Error::from(format!("[rbatis-codegen] SetNode expression '{}' does not start with '{}'", express, SetNode::name())));
        };
        if actual_attrs_str.is_empty() {
            return Err(Error::from(format!("[rbatis-codegen] SetNode attributes are empty in '{}'. 'collection' attribute is mandatory.", source_str)));
        }
        let mut collection_opt: Option<String> = None;
        let mut skip_null_val = false; // Default
        let mut skips_val: String = String::new(); // Default is now an empty String
        for part_str in actual_attrs_str.split(',') {
            let clean_part = part_str.trim();
            if clean_part.is_empty() {
                continue;
            }

            let kv: Vec<&str> = clean_part.splitn(2, '=').collect();
            if kv.len() != 2 {
                return Err(Error::from(format!("[rbatis-codegen] Malformed attribute in set node near '{}' in '{}'", clean_part, source_str)));
            }

            let key = kv[0].trim();
            let value_str_raw = kv[1].trim();

            match key {
                "collection" => {
                    collection_opt = Some(Self::strip_quotes_for_attr(value_str_raw));
                }
                "skip_null" => {
                    let val_bool_str = Self::strip_quotes_for_attr(value_str_raw);
                    if val_bool_str.eq_ignore_ascii_case("true") {
                        skip_null_val = true;
                    } else if val_bool_str.eq_ignore_ascii_case("false") {
                        skip_null_val = false;
                    } else {
                        return Err(Error::from(format!("[rbatis-codegen] Invalid boolean value for skip_null: '{}' in '{}'", value_str_raw, source_str)));
                    }
                }
                "skips" => {
                    let inner_skips_str = Self::strip_quotes_for_attr(value_str_raw);
                    skips_val = inner_skips_str;
                }
                _ => {
                    return Err(Error::from(format!("[rbatis-codegen] Unknown attribute '{}' for set node in '{}'", key, source_str)));
                }
            }
        }
        let collection_val = collection_opt.ok_or_else(|| Error::from(format!("[rbatis-codegen] Mandatory attribute 'collection' missing for set node in '{}'", source_str)))?;
        Ok(NodeType::NSet(SetNode {
            childs,
            collection: collection_val,
            skip_null: skip_null_val,
            skips: skips_val,
        }))
    }
}