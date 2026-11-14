use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::BTreeMap;
use syn::ItemFn;

use crate::codegen::diagnostic::{Diagnostic, SourceSpan};
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
    load_mapper_map_with_span_and_source(html, span, true)
}

/// [`load_mapper_map_with_span`] with an explicit source-tracking switch.
///
/// Generated templates (`py_sql` -> html) pass `false`: their line numbers do not
/// map back to the py_sql source, so no code frame should pretend they do.
pub(crate) fn load_mapper_map_with_span_and_source(
    html: &str,
    span: Span,
    track_source: bool,
) -> Result<BTreeMap<String, Element>, Error> {
    let elements = loader_html::load_mapper_vec_with_source(html, track_source)?;
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
                    Diagnostic::new(
                        Span::call_site(),
                        "[rbatis-codegen] <sql> is missing the required \"id\" attribute",
                        "add id=\"...\"",
                    )
                    .with_source(element.source.clone())
                    .to_syn()
                })?;
                if id.trim().is_empty() {
                    return Err(Diagnostic::new(
                        Span::call_site(),
                        "[rbatis-codegen] <sql> has an empty \"id\" attribute",
                        "id must not be empty",
                    )
                    .with_source(element.source.clone())
                    .to_syn()
                    .into());
                }
                sql_map.insert(id.clone(), element.clone());
            }
            INCLUDE_TAG => {
                let node = IncludeTagNode::try_from_element(&element, span)
                    .map_err(|d| d.with_source(element.source.clone()).to_syn())?;
                element = node
                    .try_process_include(sql_map)
                    .map_err(|d| d.with_source(element.source.clone()).to_syn())?;
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
    try_parse_html_inner(html, fn_name, ignore, span, true)
}

/// Parse a generated template (py_sql -> html) without attaching source lines.
pub fn try_parse_html_without_source(
    html: &str,
    fn_name: &str,
    ignore: &mut Vec<String>,
    span: Span,
) -> syn::Result<TokenStream> {
    try_parse_html_inner(html, fn_name, ignore, span, false)
}

fn try_parse_html_inner(
    html: &str,
    fn_name: &str,
    ignore: &mut Vec<String>,
    span: Span,
    track_source: bool,
) -> syn::Result<TokenStream> {
    let processed_html = html
        .replace("\\\"", "\"")
        .replace("\\n", "\n")
        .trim_matches('"')
        .to_string();

    let mut elements =
        load_mapper_map_with_span_and_source(&processed_html, span, track_source)
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
    let fn_impl = parse_elements(&elements, &mut methods, ignore, fn_name, span)
        .map_err(|d| d.to_syn())?;
    Ok(quote! { #methods #fn_impl })
}

/// Create an AST node and attach the source of the element it came from.
///
/// This is the single place where node diagnostics become [`syn::Error`]s, so every
/// node (including nodes that currently have no errors) shares the same code-frame
/// rendering without storing source fields on the node structs.
fn node_from<N: HtmlAstNode>(element: &Element, span: Span) -> Result<N, Diagnostic> {
    N::try_from_element(element, span).map_err(|d| d.with_source(element.source.clone()))
}

/// Generate a node's tokens and attach the source of the element it came from.
fn node_tokens<N, F>(
    node: &N,
    context: &mut NodeContext<F>,
    ignore: &mut Vec<String>,
    source: &SourceSpan,
) -> Result<TokenStream, Diagnostic>
where
    N: HtmlAstNode,
    F: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> Result<TokenStream, Diagnostic>,
{
    node.try_generate_tokens(context, ignore)
        .map_err(|d| d.with_source(source.clone()))
}

/// Main parsing function that converts elements to Rust code using AST nodes
fn parse_elements(
    elements: &[Element],
    methods: &mut TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
    span: Span,
) -> Result<TokenStream, Diagnostic> {
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
        let source = element.source.clone();
        match element.tag.as_str() {
            "" => {
                // Text node, handle directly here
                handle_text_element(element, &mut body, ignore, span)?;
            }
            MAPPER_TAG => {
                let node = node_from::<MapperTagNode>(element, span)?;
                body = node_tokens(&node, &mut context, ignore, &source)?;
            }
            SQL_TAG => {
                let node = node_from::<SqlTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            INCLUDE_TAG => {
                // Include tags are normally resolved earlier in `include_replace`
                // (inside `load_mapper_map`). If one reaches this point, parse its children.
                let node = node_from::<IncludeTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            CONTINUE_TAG => {
                let node = node_from::<ContinueTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            BREAK_TAG => {
                let node = node_from::<BreakTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            IF_TAG => {
                let node = node_from::<IfTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            TRIM_TAG => {
                let node = node_from::<TrimTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            BIND_TAG => {
                let node = node_from::<BindTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            WHERE_TAG => {
                let node = node_from::<WhereTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            CHOOSE_TAG => {
                let node = node_from::<ChooseTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            FOREACH_TAG => {
                let node = node_from::<ForeachTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            SET_TAG => {
                let node = node_from::<SetTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            SELECT_TAG => {
                let node = node_from::<SelectTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            UPDATE_TAG => {
                let node = node_from::<UpdateTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            INSERT_TAG => {
                let node = node_from::<InsertTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            DELETE_TAG => {
                let node = node_from::<DeleteTagNode>(element, span)?;
                let code = node_tokens(&node, &mut context, ignore, &source)?;
                body = quote! { #body #code };
            }
            WHEN_TAG => {}
            OTHERWISE_TAG => {}
            _ => {
                return Err(Diagnostic::new(
                    span,
                    format!("[rbatis-codegen] unsupported html tag: <{}>", element.tag),
                    "not a supported html_sql tag",
                )
                .with_source(source))
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
) -> Result<(), Diagnostic> {
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
            Diagnostic::from_error(e, "check the expression").with_source(
                element
                    .source
                    .clone()
                    .highlighting_in_text(&element.data, &k),
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
