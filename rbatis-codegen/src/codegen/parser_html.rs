use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::BTreeMap;
use syn::ItemFn;

use crate::codegen::loader_html::{self, Element};
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
    load_mapper_map_with_span(html, Span::call_site())
}

/// Loads HTML content into a map of elements keyed by their ID, using `span` for the
/// elements created while resolving `<include>` directives.
pub fn load_mapper_map_with_span(
    html: &str,
    span: Span,
) -> Result<BTreeMap<String, Element>, Error> {
    let elements = load_mapper_vec(html)?;
    let mut sql_map = BTreeMap::new();
    let processed_elements = include_replace(elements, &mut sql_map, span)?;

    let mut m = BTreeMap::new();
    for x in processed_elements {
        if let Some(v) = x.attrs.get("id") {
            m.insert(v.to_string(), x);
        }
    }
    Ok(m)
}

/// Loads HTML content into a vector of elements
///
/// kept as an alias of [`loader_html::load_mapper_vec`] because it is part of the
/// public surface of this module
pub fn load_mapper_vec(html: &str) -> Result<Vec<Element>, Error> {
    loader_html::load_mapper_vec(html)
}

/// Handles include directives and replaces them with referenced content
fn include_replace(
    elements: Vec<Element>,
    sql_map: &mut BTreeMap<String, Element>,
    span: Span,
) -> Result<Vec<Element>, Error> {
    let mut result = Vec::with_capacity(elements.len());
    for mut element in elements {
        match element.tag.as_str() {
            SQL_TAG => {
                let id = element.attrs.get("id").ok_or_else(|| {
                    Error::from(format!(
                        "[rbatis-codegen] <sql> is missing the required \"id\" attribute{}",
                        element.code_frame("add id=\"...\"")
                    ))
                })?;
                if id.trim().is_empty() {
                    return Err(Error::from(format!(
                        "[rbatis-codegen] <sql> has an empty \"id\" attribute{}",
                        element.code_frame("id must not be empty")
                    )));
                }
                sql_map.insert(id.clone(), element.clone());
            }
            INCLUDE_TAG => {
                let node = IncludeTagNode::try_from_element(&element, span)?;
                element = node.try_process_include(sql_map)?;
            }
            _ => {
                if let Some(id) = element.attrs.get("id").filter(|id| !id.is_empty()) {
                    sql_map.insert(id.clone(), element.clone());
                }
            }
        }

        if !element.childs.is_empty() {
            element.childs = include_replace(element.childs, sql_map, span)?;
        }

        result.push(element);
    }
    Ok(result)
}

/// Parses HTML content into a function TokenStream.
///
/// This is the infallible compatibility wrapper: it panics on a bad template. The
/// proc-macro entry points use [`try_parse_html`] to turn the error into a spanned
/// `compile_error!` instead.
pub fn parse_html(html: &str, fn_name: &str, ignore: &mut Vec<String>) -> TokenStream {
    try_parse_html(html, fn_name, ignore, Span::call_site()).unwrap_or_else(|e| panic!("{}", e))
}

/// Parses HTML content into a function TokenStream, reporting a bad template as a
/// spanned [`syn::Error`] (the span of the `#[html_sql]` / `#[py_sql]` literal).
pub fn try_parse_html(
    html: &str,
    fn_name: &str,
    ignore: &mut Vec<String>,
    span: Span,
) -> syn::Result<TokenStream> {
    let processed_html = html
        .replace("\\\"", "\"")
        .replace("\\n", "\n")
        .trim_matches('"')
        .to_string();

    let mut elements = load_mapper_map_with_span(&processed_html, span)
        .map_err(|e| syn::Error::new(span, format!("Failed to load html: {}", e)))?;

    // the map keeps every `<sql>`/`<select>`/... entry, so the id of the function must
    // be looked up. Taking just the first entry compiled the wrong statement whenever
    // an inline #[html_sql] had more than one (BTreeMap sorts by id, not by source
    // order). If no id matches, fall back to the single statement of the template
    let element = match elements.remove(fn_name) {
        Some(element) => element,
        None => {
            // Prefer the single statement of the template. A `<sql id="...">` fragment
            // is not a statement, and BTreeMap order can put the fragment first.
            let statements: Vec<Element> = elements
                .values()
                .filter(|element| {
                    matches!(
                        element.tag.as_str(),
                        SELECT_TAG | INSERT_TAG | UPDATE_TAG | DELETE_TAG | SET_TAG
                    )
                })
                .cloned()
                .collect();
            if statements.len() == 1 {
                statements.into_iter().next().expect("one statement")
            } else {
                elements
                    .into_iter()
                    .next()
                    .ok_or_else(|| {
                        syn::Error::new(span, format!("HTML not found for function: {}", fn_name))
                    })?
                    .1
            }
        }
    };

    parse_html_node(vec![element], ignore, fn_name, span)
}

/// Parses HTML nodes into Rust code
fn parse_html_node(
    elements: Vec<Element>,
    ignore: &mut Vec<String>,
    fn_name: &str,
    span: Span,
) -> syn::Result<TokenStream> {
    let mut methods = quote!();
    let fn_impl = parse_elements(&elements, &mut methods, ignore, fn_name, span)?;
    Ok(quote! { #methods #fn_impl })
}

/// Main parsing function that converts elements to Rust code using AST nodes
fn parse_elements(
    elements: &[Element],
    methods: &mut TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
    span: Span,
) -> syn::Result<TokenStream> {
    let mut body = quote! {};

    // Create a context object that will be passed to node generators
    let mut context = NodeContext {
        methods,
        fn_name,
        span,
        child_parser: |child_elements: &[Element],
                       child_methods: &mut TokenStream,
                       child_ignore: &mut Vec<String>,
                       child_fn_name: &str| {
            parse_elements(
                child_elements,
                child_methods,
                child_ignore,
                child_fn_name,
                span,
            )
        },
    };

    for element in elements {
        match element.tag.as_str() {
            "" => {
                // Text node, handle directly here
                handle_text_element(element, &mut body, ignore, span)?;
            }
            MAPPER_TAG => {
                let node = MapperTagNode::try_from_element(element, span)?;
                body = node.try_generate_tokens(&mut context, ignore)?;
            }
            SQL_TAG => {
                let node = SqlTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            INCLUDE_TAG => {
                // Include tags are normally resolved earlier in `include_replace`
                // (inside `load_mapper_map`). If one reaches this point, parse its children.
                let node = IncludeTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            CONTINUE_TAG => {
                let node = ContinueTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            BREAK_TAG => {
                let node = BreakTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            IF_TAG => {
                let node = IfTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            TRIM_TAG => {
                let node = TrimTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            BIND_TAG => {
                let node = BindTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            WHERE_TAG => {
                let node = WhereTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            CHOOSE_TAG => {
                let node = ChooseTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            FOREACH_TAG => {
                let node = ForeachTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            SET_TAG => {
                let node = SetTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            SELECT_TAG => {
                let node = SelectTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            UPDATE_TAG => {
                let node = UpdateTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            INSERT_TAG => {
                let node = InsertTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            DELETE_TAG => {
                let node = DeleteTagNode::try_from_element(element, span)?;
                let code = node.try_generate_tokens(&mut context, ignore)?;
                body = quote! { #body #code };
            }
            WHEN_TAG => {}
            OTHERWISE_TAG => {}
            _ => {
                return Err(syn::Error::new(
                    span,
                    format!(
                    "[rbatis-codegen] unsupported html tag: <{}>{}",
                    element.tag,
                    element.code_frame("not a supported html_sql tag")
                ),
                ))
            }
        }
    }

    Ok(body)
}

/// Handles plain text elements
#[allow(clippy::ptr_arg)]
fn handle_text_element(
    element: &Element,
    body: &mut TokenStream,
    ignore: &mut Vec<String>,
    span: Span,
) -> syn::Result<()> {
    let mut string_data = remove_extra(&element.data);
    let convert_list = find_convert_string(&string_data);

    let mut formats_value = quote! {};
    let mut replace_num = 0;

    for (k, v) in convert_list {
        let method_impl = crate::codegen::func::try_impl_fn_with_span(
            &body.to_string(),
            "",
            &format!("\"{}\"", k),
            false,
            ignore,
            span,
        )
        .map_err(|e| {
            syn::Error::new(
                e.span(),
                format!("{}{}", e, element.code_frame("check the expression")),
            )
        })?;

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
    Ok(())
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

/// Implements HTML SQL function.
///
/// This is the infallible compatibility wrapper: it panics on a bad template. The
/// proc-macro entry points use [`try_impl_fn_html`].
pub fn impl_fn_html(m: &ItemFn, args: &ParseArgs) -> MacroTokenStream {
    try_impl_fn_html(m, args).unwrap_or_else(|e| panic!("{}", e))
}

/// Implements HTML SQL function, reporting template errors as a spanned
/// [`syn::Error`] that points at the `#[html_sql(...)]` literal.
pub fn try_impl_fn_html(m: &ItemFn, args: &ParseArgs) -> syn::Result<MacroTokenStream> {
    let fn_name = m.sig.ident.to_string();

    if args.sqls.is_empty() {
        return Err(syn::Error::new(
            m.sig.ident.span(),
            format!(
                "[rbatis-codegen] #[html_sql()] must have html_data, for example: {}",
                stringify!(#[html_sql(r#"<select id="select_by_condition">`select * from biz_activity</select>"#)])
            ),
        ));
    }

    // every argument is part of the template, taking just the first one would drop the
    // rest of them
    let html_data = args
        .sqls
        .iter()
        .map(|sql| sql.value())
        .collect::<Vec<String>>()
        .join("");
    let span = args
        .sqls
        .first()
        .map(|sql| sql.span())
        .unwrap_or_else(|| m.sig.ident.span());
    Ok(try_parse_html(&html_data, &fn_name, &mut vec![], span)?.into())
}
