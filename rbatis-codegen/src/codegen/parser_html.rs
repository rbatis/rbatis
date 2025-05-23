use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::{ItemFn, LitStr};
use url::Url;

use crate::codegen::loader_html::{load_html, Element};
use crate::codegen::proc_macro::TokenStream;
use crate::codegen::string_util::find_convert_string;
use crate::codegen::ParseArgs;
use crate::error::Error;

// Constants for common strings
const SQL_TAG: &str = "sql";
const INCLUDE_TAG: &str = "include";
const MAPPER_TAG: &str = "mapper";
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

/// Parses HTML content into a function TokenStream
pub fn parse_html(html: &str, fn_name: &str, ignore: &mut Vec<String>) -> proc_macro2::TokenStream {
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
                element = handle_include_element(&element, sql_map);
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

/// Processes an include element by resolving its reference
fn handle_include_element(element: &Element, sql_map: &BTreeMap<String, Element>) -> Element {
    let ref_id = element.attrs.get("refid")
        .expect("[rbatis-codegen] <include> element must have attr <include refid=\"\">!");

    let url = if ref_id.contains("://") {
        Url::parse(ref_id).unwrap_or_else(|_| panic!(
            "[rbatis-codegen] parse <include refid=\"{}\"> fail!", ref_id
        ))
    } else {
        Url::parse(&format!("current://current?refid={}", ref_id)).unwrap_or_else(|_| panic!(
            "[rbatis-codegen] parse <include refid=\"{}\"> fail!", ref_id
        ))
    };

    match url.scheme() {
        "file" => handle_file_include(&url, ref_id),
        "current" => handle_current_include(&url, ref_id, sql_map),
        _ => panic!("Unimplemented scheme <include refid=\"{}\">", ref_id),
    }
}

/// Handles file-based includes
fn handle_file_include(url: &Url, ref_id: &str) -> Element {
    let mut manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to read CARGO_MANIFEST_DIR");
    manifest_dir.push('/');

    let path = url.host_str().unwrap_or_default().to_string() +
        url.path().trim_end_matches(&['/', '\\'][..]);
    let mut file_path = PathBuf::from(&path);

    if file_path.is_relative() {
        file_path = PathBuf::from(format!("{}{}", manifest_dir, path));
    }

    let ref_id = url.query_pairs()
        .find(|(k, _)| k == "refid")
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| {
            panic!("No ref_id found in URL {}", ref_id);
        });

    let mut file = File::open(&file_path).unwrap_or_else(|_| panic!(
        "[rbatis-codegen] can't find file='{}', url='{}'",
        file_path.to_str().unwrap_or_default(),
        url
    ));

    let mut html = String::new();
    file.read_to_string(&mut html).expect("Failed to read file");

    load_mapper_vec(&html).expect("Failed to parse HTML")
        .into_iter()
        .find(|e| e.tag == SQL_TAG && e.attrs.get("id") == Some(&ref_id))
        .unwrap_or_else(|| panic!(
            "No ref_id={} found in file={}",
            ref_id,
            file_path.to_str().unwrap_or_default()
        ))
}

/// Handles current document includes
fn handle_current_include(url: &Url, ref_id: &str, sql_map: &BTreeMap<String, Element>) -> Element {
    let ref_id = url.query_pairs()
        .find(|(k, _)| k == "refid")
        .map(|(_, v)| v.to_string())
        .unwrap_or(ref_id.to_string());

    sql_map.get(&ref_id).unwrap_or_else(|| panic!(
        "[rbatis-codegen] cannot find element <include refid=\"{}\">!",
        ref_id
    )).clone()
}

/// Parses HTML nodes into Rust code
fn parse_html_node(
    elements: Vec<Element>,
    ignore: &mut Vec<String>,
    fn_name: &str,
) -> proc_macro2::TokenStream {
    let mut methods = quote!();
    let fn_impl = parse_elements(&elements, &mut methods, ignore, fn_name);
    quote! { #methods #fn_impl }
}

/// Main parsing function that converts elements to Rust code
fn parse_elements(
    elements: &[Element],
    methods: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
) -> proc_macro2::TokenStream {
    let mut body = quote! {};

    for element in elements {
        match element.tag.as_str() {
            MAPPER_TAG => {
                return parse_elements(&element.childs, methods, ignore, fn_name);
            }
            SQL_TAG | INCLUDE_TAG => {
                let code = parse_elements(&element.childs, methods, ignore, fn_name);
                body = quote! { #body #code };
            }
            CONTINUE_TAG => impl_continue(&mut body),
            BREAK_TAG => impl_break(&mut body),
            "" => handle_text_element(element, &mut body, ignore),
            IF_TAG => handle_if_element(element, &mut body, methods, ignore, fn_name),
            TRIM_TAG => handle_trim_element(element, &mut body, methods, ignore, fn_name),
            BIND_TAG => handle_bind_element(element, &mut body, ignore),
            WHERE_TAG => handle_where_element(element, &mut body, methods, ignore, fn_name),
            CHOOSE_TAG => handle_choose_element(element, &mut body, methods, ignore, fn_name),
            FOREACH_TAG => handle_foreach_element(element, &mut body, methods, ignore, fn_name),
            SET_TAG => handle_set_element(element, &mut body, methods, ignore, fn_name),
            SELECT_TAG | UPDATE_TAG | INSERT_TAG | DELETE_TAG => {
                handle_crud_element(element, &mut body, methods, ignore, fn_name)
            }
            _ => {}
        }
    }

    body
}

/// Handles plain text elements
fn handle_text_element(
    element: &Element,
    body: &mut proc_macro2::TokenStream,
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
            quote! { #body sql.push_str(#string_data); }
        } else {
            quote! { #body sql.push_str(&format!(#string_data #formats_value)); }
        };
    }
}

/// Handles if elements
fn handle_if_element(
    element: &Element,
    body: &mut proc_macro2::TokenStream,
    methods: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
) {
    let test_value = element.attrs.get("test")
        .unwrap_or_else(|| panic!("{} element must have test field!", element.tag));

    let if_tag_body = if !element.childs.is_empty() {
        parse_elements(&element.childs, methods, ignore, fn_name)
    } else {
        quote! {}
    };

    impl_condition(test_value, if_tag_body, body, methods, quote! {}, ignore);
}

/// Handles trim elements
fn handle_trim_element(
    element: &Element,
    body: &mut proc_macro2::TokenStream,
    methods: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
) {
    let empty = String::new();
    let prefix = element.attrs.get("prefix").unwrap_or(&empty);
    let suffix = element.attrs.get("suffix").unwrap_or(&empty);
    let prefix_overrides = element.attrs.get("start")
        .or_else(|| element.attrs.get("prefixOverrides"))
        .unwrap_or(&empty);
    let suffix_overrides = element.attrs.get("end")
        .or_else(|| element.attrs.get("suffixOverrides"))
        .unwrap_or(&empty);

    impl_trim(
        prefix,
        suffix,
        prefix_overrides,
        suffix_overrides,
        element,
        body,
        methods,
        ignore,
        fn_name,
    );
}

/// Handles bind elements
fn handle_bind_element(
    element: &Element,
    body: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
) {
    let name = element.attrs.get("name")
        .expect("<bind> must have name!");
    let value = element.attrs.get("value")
        .expect("<bind> element must have value!");

    let method_impl = crate::codegen::func::impl_fn(
        &body.to_string(),
        "",
        &format!("\"{}\"", value),
        false,
        ignore,
    );

    let lit_str = LitStr::new(name, Span::call_site());

    *body = quote! {
        #body
        if arg[#lit_str] == rbs::Value::Null {
            arg.insert(rbs::Value::String(#lit_str.to_string()), rbs::Value::Null);
        }
        arg[#lit_str] = rbs::value(#method_impl).unwrap_or_default();
    };
}

/// Handles where elements
fn handle_where_element(
    element: &Element,
    body: &mut proc_macro2::TokenStream,
    methods: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
) {
    impl_trim(
        " where ",
        " ",
        " |and |or ",
        " | and| or",
        element,
        body,
        methods,
        ignore,
        fn_name,
    );

    *body = quote! {
        #body
        sql = sql.trim_end_matches(" where  ").to_string();
    };
}

/// Handles choose elements
fn handle_choose_element(
    element: &Element,
    body: &mut proc_macro2::TokenStream,
    methods: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
) {
    let mut inner_body = quote! {};

    for child in &element.childs {
        match child.tag.as_str() {
            WHEN_TAG => {
                let test_value = child.attrs.get("test")
                    .unwrap_or_else(|| panic!("{} element must have test field!", child.tag));

                let if_tag_body = if !child.childs.is_empty() {
                    parse_elements(&child.childs, methods, ignore, fn_name)
                } else {
                    quote! {}
                };

                impl_condition(
                    test_value,
                    if_tag_body,
                    &mut inner_body,
                    methods,
                    quote! { return sql; },
                    ignore,
                );
            }
            OTHERWISE_TAG => {
                let child_body = parse_elements(&child.childs, methods, ignore, fn_name);
                impl_otherwise(child_body, &mut inner_body);
            }
            _ => panic!("choose node's children must be when or otherwise nodes!"),
        }
    }

    let capacity = element.child_string_cup() + 1000;
    *body = quote! {
        #body
        sql.push_str(&|| -> String {
            let mut sql = String::with_capacity(#capacity);
            #inner_body
            return sql;
        }());
    };
}

/// Handles foreach elements
fn handle_foreach_element(
    element: &Element,
    body: &mut proc_macro2::TokenStream,
    methods: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
) {
    let empty = String::new();
    let def_item = "item".to_string();
    let def_index = "index".to_string();

    let collection = element.attrs.get("collection").unwrap_or(&empty);
    let mut item = element.attrs.get("item").unwrap_or(&def_item);
    let mut index = element.attrs.get("index").unwrap_or(&def_index);
    let open = element.attrs.get("open").unwrap_or(&empty);
    let close = element.attrs.get("close").unwrap_or(&empty);
    let separator = element.attrs.get("separator").unwrap_or(&empty);

    if item.is_empty() || item == "_" {
        item = &def_item;
    }
    if index.is_empty() || index == "_" {
        index = &def_index;
    }

    let mut ignores = ignore.clone();
    ignores.push(index.to_string());
    ignores.push(item.to_string());

    let impl_body = parse_elements(&element.childs, methods, &mut ignores, fn_name);
    let method_impl = crate::codegen::func::impl_fn(
        &body.to_string(),
        "",
        &format!("\"{}\"", collection),
        false,
        ignore,
    );

    let open_impl = if !open.is_empty() {
        quote! { sql.push_str(#open); }
    } else {
        quote! {}
    };

    let close_impl = if !close.is_empty() {
        quote! { sql.push_str(#close); }
    } else {
        quote! {}
    };

    let item_ident = Ident::new(item, Span::call_site());
    let index_ident = Ident::new(index, Span::call_site());

    let (split_code, split_code_trim) = if !separator.is_empty() {
        (
            quote! { sql.push_str(#separator); },
            quote! { sql = sql.trim_end_matches(#separator).to_string(); }
        )
    } else {
        (quote! {}, quote! {})
    };

    *body = quote! {
        #body
        #open_impl
        for (ref #index_ident, #item_ident) in #method_impl {
            #impl_body
            #split_code
        }
        #split_code_trim
        #close_impl
    };
}

/// Handles set elements
fn handle_set_element(
    element: &Element,
    body: &mut proc_macro2::TokenStream,
    methods: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
) {
    if let Some(collection) = element.attrs.get("collection") {
        let skip_null = element.attrs.get("skip_null");
        let skips = element.attrs.get("skips").unwrap_or(&"id".to_string()).to_string();
        let elements = make_sets(collection, skip_null, &skips);
        let code = parse_elements(&elements, methods, ignore, fn_name);
        *body = quote! { #body #code };
    } else {
        impl_trim(
            " set ", " ", " |,", " |,", element, body, methods, ignore, fn_name,
        );
    }
}

/// Handles CRUD elements (select, update, insert, delete)
fn handle_crud_element(
    element: &Element,
    body: &mut proc_macro2::TokenStream,
    methods: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
) {
    let method_name = Ident::new(fn_name, Span::call_site());
    let child_body = parse_elements(&element.childs, methods, ignore, fn_name);
    let capacity = element.child_string_cup() + 1000;
    let push_count = child_body.to_string().matches("args.push").count();

    let function = quote! {
        pub fn #method_name(mut arg: rbs::Value, _tag: char) -> (String, Vec<rbs::Value>) {
            use rbatis_codegen::ops::*;
            let mut sql = String::with_capacity(#capacity);
            let mut args = Vec::with_capacity(#push_count);
            #child_body
            (sql, args)
        }
    };

    *body = quote! { #body #function };
}

/// Creates set elements for SQL updates
fn make_sets(collection: &str, skip_null: Option<&String>, skips: &str) -> Vec<Element> {
    let is_skip_null = skip_null.map_or(true, |v| v != "false");
    let skip_strs: Vec<&str> = skips.split(',').collect();

    let skip_elements = skip_strs.iter().map(|x| Element {
        tag: IF_TAG.to_string(),
        data: String::new(),
        attrs: {
            let mut attr = HashMap::new();
            attr.insert("test".to_string(), format!("k == '{}'", x));
            attr
        },
        childs: vec![Element {
            tag: CONTINUE_TAG.to_string(),
            data: String::new(),
            attrs: HashMap::new(),
            childs: vec![],
        }],
    }).collect::<Vec<_>>();

    let mut for_each_body = skip_elements;

    if is_skip_null {
        for_each_body.push(Element {
            tag: IF_TAG.to_string(),
            data: String::new(),
            attrs: {
                let mut attr = HashMap::new();
                attr.insert("test".to_string(), "v == null".to_string());
                attr
            },
            childs: vec![Element {
                tag: CONTINUE_TAG.to_string(),
                data: String::new(),
                attrs: HashMap::new(),
                childs: vec![],
            }],
        });
    }

    for_each_body.push(Element {
        tag: "".to_string(),
        data: "${k}=#{v},".to_string(),
        attrs: HashMap::new(),
        childs: vec![],
    });

    vec![Element {
        tag: TRIM_TAG.to_string(),
        data: String::new(),
        attrs: {
            let mut attr = HashMap::new();
            attr.insert("prefix".to_string(), " set ".to_string());
            attr.insert("suffix".to_string(), " ".to_string());
            attr.insert("start".to_string(), " ".to_string());
            attr.insert("end".to_string(), " ".to_string());
            attr
        },
        childs: vec![Element {
            tag: TRIM_TAG.to_string(),
            data: String::new(),
            attrs: {
                let mut attr = HashMap::new();
                attr.insert("prefix".to_string(), "".to_string());
                attr.insert("suffix".to_string(), "".to_string());
                attr.insert("start".to_string(), ",".to_string());
                attr.insert("end".to_string(), ",".to_string());
                attr
            },
            childs: vec![Element {
                tag: FOREACH_TAG.to_string(),
                data: String::new(),
                attrs: {
                    let mut attr = HashMap::new();
                    attr.insert("collection".to_string(), collection.to_string());
                    attr.insert("index".to_string(), "k".to_string());
                    attr.insert("item".to_string(), "v".to_string());
                    attr
                },
                childs: for_each_body,
            }],
        }],
    }]
}

/// Cleans up text content by removing extra characters
fn remove_extra(text: &str) -> String {
    let text = text.trim().replace("\\r", "");
    let lines: Vec<&str> = text.split('\n').collect();

    let mut data = String::with_capacity(text.len());
    for (i, line) in lines.iter().enumerate() {
        let mut line = line.trim();
        line = line.trim_start_matches('`').trim_end_matches('`');
        data.push_str(line);
        if i + 1 < lines.len() {
            data.push('\n');
        }
    }

    data.trim_matches('`').replace("``", "")
}

/// Implements continue statement
fn impl_continue(body: &mut proc_macro2::TokenStream) {
    *body = quote! { #body continue; };
}

/// Implements break statement
fn impl_break(body: &mut proc_macro2::TokenStream) {
    *body = quote! { #body break; };
}

/// Implements conditional logic
fn impl_condition(
    test_value: &str,
    condition_body: proc_macro2::TokenStream,
    body: &mut proc_macro2::TokenStream,
    _methods: &mut proc_macro2::TokenStream,
    appends: proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
) {
    let method_impl = crate::codegen::func::impl_fn(
        &body.to_string(),
        "",
        &format!("\"{}\"", test_value),
        false,
        ignore,
    );

    *body = quote! {
        #body
        if #method_impl.to_owned().into() {
            #condition_body
            #appends
        }
    };
}

/// Implements otherwise clause
fn impl_otherwise(
    child_body: proc_macro2::TokenStream,
    body: &mut proc_macro2::TokenStream,
) {
    *body = quote! { #body #child_body };
}

/// Implements trim logic
fn impl_trim(
    prefix: &str,
    suffix: &str,
    start: &str,
    end: &str,
    element: &Element,
    body: &mut proc_macro2::TokenStream,
    methods: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
) {
    let trim_body = parse_elements(&element.childs, methods, ignore, fn_name);
    let prefixes: Vec<&str> = start.split('|').collect();
    let suffixes: Vec<&str> = end.split('|').collect();
    let has_trim = !prefixes.is_empty() && !suffixes.is_empty();
    let capacity = element.child_string_cup();

    let mut trims = quote! {
        let mut sql = String::with_capacity(#capacity);
        #trim_body
        sql = sql
    };

    for prefix in prefixes {
        trims = quote! { #trims .trim_start_matches(#prefix) };
    }

    for suffix in suffixes {
        trims = quote! { #trims .trim_end_matches(#suffix) };
    }

    if !prefix.is_empty() {
        *body = quote! { #body sql.push_str(#prefix); };
    }

    if has_trim {
        *body = quote! { #body sql.push_str(&{#trims.to_string(); sql}); };
    }

    if !suffix.is_empty() {
        *body = quote! { #body sql.push_str(#suffix); };
    }
}

/// Implements HTML SQL function
pub fn impl_fn_html(m: &ItemFn, args: &ParseArgs) -> TokenStream {
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