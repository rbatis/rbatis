use proc_macro2::{TokenStream};
use quote::{quote, ToTokens};
use std::collections::{BTreeMap};
use syn::{ItemFn};


use crate::codegen::loader_html::{load_html, Element};
use crate::codegen::proc_macro::TokenStream as MacroTokenStream;
use crate::codegen::string_util::{concat_str, find_convert_string};
use crate::codegen::syntax_tree_html::*;
use crate::codegen::ParseArgs;
use crate::error::Error;

// Constants for common strings
const SQL_TAG: &str = "sql";
const INCLUDE_TAG: &str = "include";
pub(crate) const MAPPER_TAG: &str = "mapper";
const IF_TAG: &str = "if";
const TRIM_TAG: &str = "trim";
const BIND_TAG: &str = "bind";
const WHERE_TAG: &str = "where";
const CHOOSE_TAG: &str = "choose";
const WHEN_TAG: &str = "when";
const OTHERWISE_TAG: &str = "otherwise";
const FOREACH_TAG: &str = "foreach";
const SET_TAG: &str = "set";
const CONTINUE_TAG: &str = "continue";
const BREAK_TAG: &str = "break";
const SELECT_TAG: &str = "select";
const UPDATE_TAG: &str = "update";
const INSERT_TAG: &str = "insert";
const DELETE_TAG: &str = "delete";

/// Loads HTML content into a map of elements keyed by their ID
pub fn load_mapper_map(html: &str) -> Result<BTreeMap<String, Element>, Error> {
    let elements = load_mapper_vec(html)?;
    let mut sql_map = BTreeMap::new();
    let processed_elements = include_replace(elements, &mut sql_map);

    let mut m = BTreeMap::new();
    for x in processed_elements {
        if let Some(v) = x.attrs.get("id") {
            m.insert(v.to_string(), x);
        }
    }
    Ok(m)
}

/// Loads HTML content into a vector of elements
pub fn load_mapper_vec(html: &str) -> Result<Vec<Element>, Error> {
    let elements = load_html(html).map_err(|e| Error::from(e.to_string()))?;

    let mut mappers = Vec::new();
    for element in elements {
        if element.tag == MAPPER_TAG {
            mappers.extend(element.childs);
        } else {
            mappers.push(element);
        }
    }

    Ok(mappers)
}

/// Handles include directives and replaces them with referenced content
fn include_replace(elements: Vec<Element>, sql_map: &mut BTreeMap<String, Element>) -> Vec<Element> {
    elements.into_iter().map(|mut element| {
        match element.tag.as_str() {
            SQL_TAG => {
                let id = element.attrs.get("id")
                    .expect("[rbatis-codegen] <sql> element must have id!");
                sql_map.insert(id.clone(), element.clone());
            }
            INCLUDE_TAG => {
                element = IncludeTagNode::from_element(&element).process_include(sql_map);
            }
            _ => {
                if let Some(id) = element.attrs.get("id").filter(|id| !id.is_empty()) {
                    sql_map.insert(id.clone(), element.clone());
                }
            }
        }

        if !element.childs.is_empty() {
            element.childs = include_replace(element.childs, sql_map);
        }

        element
    }).collect()
}

/// Parses HTML content into a function TokenStream
pub fn parse_html(html: &str, fn_name: &str, ignore: &mut Vec<String>) -> TokenStream {
    let processed_html = html
        .replace("\\\"", "\"")
        .replace("\\n", "\n")
        .trim_matches('"')
        .to_string();

    let elements = load_mapper_map(&processed_html)
        .unwrap_or_else(|_| panic!("Failed to load html: {}", processed_html));

    let (_, element) = elements.into_iter().next()
        .unwrap_or_else(|| panic!("HTML not found for function: {}", fn_name));

    parse_html_node(vec![element], ignore, fn_name)
}

/// Parses HTML nodes into Rust code
fn parse_html_node(
    elements: Vec<Element>,
    ignore: &mut Vec<String>,
    fn_name: &str,
) -> TokenStream {
    let mut methods = quote!();
    let fn_impl = parse_elements(&elements, &mut methods, ignore, fn_name);
    quote! { #methods #fn_impl }
}

/// Main parsing function that converts elements to Rust code using AST nodes
fn parse_elements(
    elements: &[Element],
    methods: &mut TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
) -> TokenStream {
    let mut body = quote! {};

    // Create a context object that will be passed to node generators
    let mut context = NodeContext {
        methods,
        fn_name,
        child_parser: parse_elements,
    };

    for element in elements {
        match element.tag.as_str() {
            "" => {
                // Text node, handle directly here
                handle_text_element(element, &mut body, ignore);
            }
            MAPPER_TAG => {
                let node = MapperTagNode::from_element(element);
                body = node.generate_tokens(&mut context, ignore);
            }
            SQL_TAG => {
                let node = SqlTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            INCLUDE_TAG => {
                // Include tags should be processed earlier in include_replace
                // If we encounter one here, just parse its children
                let node = IncludeTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            CONTINUE_TAG => {
                let node = ContinueTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            BREAK_TAG => {
                let node = BreakTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            IF_TAG => {
                let node = IfTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            TRIM_TAG => {
                let node = TrimTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            BIND_TAG => {
                let node = BindTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            WHERE_TAG => {
                let node = WhereTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            CHOOSE_TAG => {
                let node = ChooseTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            FOREACH_TAG => {
                let node = ForeachTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            SET_TAG => {
                let node = SetTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            SELECT_TAG => {
                let node = SelectTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            UPDATE_TAG => {
                let node = UpdateTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            INSERT_TAG => {
                let node = InsertTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            DELETE_TAG => {
                let node = DeleteTagNode::from_element(element);
                let code = node.generate_tokens(&mut context, ignore);
                body = quote! { #body #code };
            }
            WHEN_TAG => {
               
            }
            OTHERWISE_TAG => {

            }
            _ => {}
        }
    }

    body
}

/// Handles plain text elements
fn handle_text_element(
    element: &Element,
    body: &mut TokenStream,
    ignore: &mut Vec<String>,
) {
    let mut string_data = remove_extra(&element.data);
    let convert_list = find_convert_string(&string_data);

    let mut formats_value = quote! {};
    let mut replace_num = 0;

    for (k, v) in convert_list {
        let method_impl = crate::codegen::func::impl_fn(
            &body.to_string(),
            "",
            &format!("\"{}\"", k),
            false,
            ignore,
        );

        if v.starts_with('#') {
            string_data = string_data.replacen(&v, "?", 1);
            *body = quote! {
                #body
                args.push(rbs::value(#method_impl).unwrap_or_default());
            };
        } else {
            string_data = string_data.replacen(&v, "{}", 1);
            if !formats_value.to_string().trim().ends_with(',') {
                formats_value = quote!(#formats_value,);
            }
            formats_value = quote!(#formats_value &#method_impl.string());
            replace_num += 1;
        }
    }

    if !string_data.is_empty() {
        *body = if replace_num == 0 {
            quote! { #body rbatis_codegen::codegen::string_util::concat_str(&mut sql, #string_data); }
        } else {
            quote! { #body rbatis_codegen::codegen::string_util::concat_str(&mut sql, &format!(#string_data #formats_value)); }
        };
    }
}

/// Cleans up text content by removing extra characters
fn remove_extra(text: &str) -> String {
    let text = text.trim().replace("\\r", "");
    let lines: Vec<&str> = text.split('\n').collect();

    let mut data = String::with_capacity(text.len());
    for (i, line) in lines.iter().enumerate() {
        let mut line = line.trim();
        line = line.trim_start_matches('`').trim_end_matches('`');

        let list: Vec<&str> = line.split("``").collect();
        let mut text = String::with_capacity(line.len());
        for s in list {
            concat_str(&mut text, s);
        }
        data.push_str(&text);
        if i + 1 < lines.len() {
            data.push('\n');
        }
    }

    data
}

/// Implements HTML SQL function
pub fn impl_fn_html(m: &ItemFn, args: &ParseArgs) -> MacroTokenStream {
    let fn_name = m.sig.ident.to_string();

    if args.sqls.is_empty() {
        panic!(
            "[rbatis-codegen] #[html_sql()] must have html_data, for example: {}",
            stringify!(#[html_sql(r#"<select id="select_by_condition">`select * from biz_activity</select>"#)])
        );
    }

    let html_data = args.sqls[0].to_token_stream().to_string();
    parse_html(&html_data, &fn_name, &mut vec![]).into()
}