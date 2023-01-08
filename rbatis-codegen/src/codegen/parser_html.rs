use std::collections::BTreeMap;
use std::env::current_dir;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

use base64::{decode, encode};
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{AttributeArgs, Expr, ItemFn, ItemMod, ItemStruct, Path};
use url::Url;

use crate::codegen::loader_html::{load_html, Element};
use crate::codegen::proc_macro::TokenStream;
use crate::codegen::string_util::find_convert_string;
use crate::codegen::syntax_tree::NodeType;
use crate::error::Error;

/// load a Map<id,Element>
pub fn load_mapper_map(html: &str) -> Result<BTreeMap<String, Element>, Error> {
    let mut datas = load_mapper_vec(html)?;
    let mut sql_map = BTreeMap::new();
    let mut datas = include_replace(datas, &mut sql_map);
    let mut m = BTreeMap::new();
    for x in datas {
        if let Some(v) = x.attrs.get("id") {
            m.insert(v.to_string(), x);
        }
    }
    Ok(m)
}

/// load a Vec<Element>
pub fn load_mapper_vec(html: &str) -> Result<Vec<Element>, Error> {
    let mut datas = load_html(html).map_err(|e| Error::from(e.to_string()))?;
    let mut mappers = vec![];
    for x in datas {
        if x.tag.eq("mapper") {
            for x in x.childs {
                mappers.push(x);
            }
        } else {
            mappers.push(x);
        }
    }
    Ok(mappers)
}

/// parse html to function TokenStream
pub fn parse_html(html: &str, fn_name: &str, ignore: &mut Vec<String>) -> proc_macro2::TokenStream {
    let html = html
        .replace("\\\"", "\"")
        .replace("\\n", "\n")
        .trim_start_matches("\"")
        .trim_end_matches("\"")
        .to_string();
    let mut datas = load_mapper_map(&html).expect(&format!("laod html={} fail", html));
    match datas.into_iter().next() {
        None => {
            panic!("html not find fn:{}", fn_name);
        }
        Some((_, v)) => {
            let node = parse_html_node(vec![v], ignore, fn_name);
            return node;
        }
    }
}

fn include_replace(htmls: Vec<Element>, sql_map: &mut BTreeMap<String, Element>) -> Vec<Element> {
    let mut results = vec![];
    for mut x in htmls {
        match x.tag.as_str() {
            "sql" => {
                sql_map.insert(
                    x.attrs
                        .get("id")
                        .expect("[rbatis] <sql> element must have id!")
                        .clone(),
                    x.clone(),
                );
            }
            "include" => {
                let ref_id = x
                    .attrs
                    .get("refid")
                    .expect("[rbatis] <include> element must have attr <include refid=\"\">!")
                    .clone();
                let mut url;
                if ref_id.contains("://") {
                    url = Url::parse(&ref_id).expect(&format!(
                        "[rbatis] parse <include refid=\"{}\"> fail!",
                        ref_id
                    ));
                } else {
                    url = Url::parse(&format!("current://current?refid={}", ref_id)).expect(
                        &format!("[rbatis] parse <include refid=\"{}\"> fail!", ref_id),
                    );
                }
                let path = url.host_str().unwrap_or_default().to_string()
                    + url.path().trim_end_matches("/").trim_end_matches("\\");
                match url.scheme() {
                    "file" => {
                        let mut ref_id = ref_id.clone();
                        let mut have_ref_id = false;
                        for (k, v) in url.query_pairs() {
                            if k.eq("refid") {
                                ref_id = v.to_string();
                                have_ref_id = true;
                            }
                        }
                        if !have_ref_id {
                            panic!("not find ref_id on url {}", ref_id);
                        }
                        let mut f = File::open(&path).expect(&format!(
                            "[rbatis] can't find file={}",
                            url.host_str().unwrap_or_default()
                        ));
                        let mut html = String::new();
                        f.read_to_string(&mut html).expect("read fail");
                        let datas = load_mapper_vec(&html).expect("read fail");
                        let mut not_find = true;
                        for element in datas {
                            if element.tag.eq("sql") && element.attrs.get("id").eq(&Some(&ref_id)) {
                                x = element.clone();
                                not_find = false;
                            }
                        }
                        if not_find {
                            panic!("not find ref_id={} on file={}", ref_id, path);
                        }
                    }
                    "current" => {
                        let mut ref_id_pair = ref_id.to_string();
                        for (k, v) in url.query_pairs() {
                            if k.eq("refid") {
                                ref_id_pair = v.to_string();
                            }
                        }
                        let mut element = sql_map
                            .get(ref_id_pair.as_str())
                            .expect(&format!(
                                "[rbatis] can not find element <include refid=\"{}\"> !",
                                ref_id
                            ))
                            .clone();
                        x = element;
                    }
                    scheme => {
                        panic!("unimplemented scheme <include refid=\"{}\">", ref_id)
                    }
                }
            }
            _ => match x.attrs.get("id") {
                None => {}
                Some(id) => {
                    if !id.is_empty() {
                        sql_map.insert(id.clone(), x.clone());
                    }
                }
            },
        }
        if x.childs.len() != 0 {
            x.childs = include_replace(x.childs.clone(), sql_map);
        }
        results.push(x);
    }
    return results;
}

fn parse_html_node(
    htmls: Vec<Element>,
    ignore: &mut Vec<String>,
    fn_name: &str,
) -> proc_macro2::TokenStream {
    let mut methods = quote!();
    let fn_impl = parse(&htmls, &mut methods, ignore, fn_name);
    let token = quote! {
        #methods
        #fn_impl
    };
    token
}

/// gen rust code
fn parse(
    arg: &Vec<Element>,
    methods: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
) -> proc_macro2::TokenStream {
    let mut body = quote! {};
    let fix_sql = quote! {};
    for x in arg {
        match x.tag.as_str() {
            "mapper" => {
                return parse(&x.childs, methods, ignore, fn_name);
            }
            "sql" => {
                let code_sql = parse(&x.childs, methods, ignore, fn_name);
                body = quote! {
                            #body
                            #code_sql
                };
            }
            "include" => {
                return parse(&x.childs, methods, ignore, fn_name);
            }
            "continue" => {
                impl_continue(x, &mut body, ignore);
            }
            "" => {
                println!("sql raw=>{:?}",x.data);
                let mut string_data = remove_extra(&x.data);
                println!("sql=>{:?}",string_data);
                let convert_list = find_convert_string(&string_data);
                let mut replaces = quote! {};
                for (k, v) in convert_list {
                    let method_impl = crate::codegen::func::impl_fn(
                        &body.to_string(),
                        "",
                        &format!("\"{}\"", k),
                        false,
                        ignore,
                    );
                    if v.starts_with("#") {
                        string_data = string_data.replacen(&v, &"?", 1);
                        body = quote! {
                            #body
                            args.push(rbs::to_value(#method_impl).unwrap_or_default());
                        };
                    } else {
                        replaces = quote! {#replaces.replacen(#v, &#method_impl.as_sql(), 1)};
                    }
                }
                if !replaces.is_empty() {
                    replaces = quote! {
                        #replaces.as_str()
                    }
                }
                if !string_data.is_empty() {
                    body = quote!(
                     #body
                      sql.push_str(#string_data #replaces);
                    );
                }
            }
            "if" => {
                let test_value = x
                    .attrs
                    .get("test")
                    .expect(&format!("{} element must be have test field!", x.tag));
                let mut if_tag_body = quote! {};
                if x.childs.len() != 0 {
                    if_tag_body = parse(&x.childs, methods, ignore, fn_name);
                }
                impl_if(
                    test_value,
                    if_tag_body,
                    &mut body,
                    methods,
                    quote! {},
                    ignore,
                );
            }
            "trim" => {
                let empty_string = String::new();
                let prefix = x.attrs.get("prefix").unwrap_or(&empty_string).to_string();
                let suffix = x.attrs.get("suffix").unwrap_or(&empty_string).to_string();
                let prefixOverrides = x
                    .attrs
                    .get("prefixOverrides")
                    .unwrap_or(&empty_string)
                    .to_string();
                let suffixOverrides = x
                    .attrs
                    .get("suffixOverrides")
                    .unwrap_or(&empty_string)
                    .to_string();
                impl_trim(
                    &prefix,
                    &suffix,
                    &prefixOverrides,
                    &suffixOverrides,
                    x,
                    &mut body,
                    arg,
                    methods,
                    ignore,
                    fn_name,
                );
            }
            "bind" => {
                let name = x
                    .attrs
                    .get("name")
                    .expect("<bind> must be have name!")
                    .to_string();
                let value = x
                    .attrs
                    .get("value")
                    .expect("<bind> element must be have value!")
                    .to_string();

                let name_expr = parse_expr(&name);
                let method_impl = crate::codegen::func::impl_fn(
                    &body.to_string(),
                    "",
                    &format!("\"{}\"", value),
                    false,
                    ignore,
                );
                body = quote! {
                    #body
                    //bind
                    let #name_expr = rbs::to_value(#method_impl).unwrap_or_default();
                };
                ignore.push(name);
            }

            "where" => {
                impl_trim(
                    " where ",
                    " ",
                    " |and |or ",
                    " | and| or",
                    x,
                    &mut body,
                    arg,
                    methods,
                    ignore,
                    fn_name,
                );
                body = quote! {
                    #body
                    //check ends with where
                    sql = sql.trim_end().to_string();
                    sql = sql.trim_end_matches(" where").to_string();
                };
            }

            "choose" => {
                let mut inner_body = quote! {};
                for x in &x.childs {
                    if x.tag.ne("when") && x.tag.ne("otherwise") {
                        panic!("choose node's childs must be when node and otherwise node!");
                    }
                    if x.tag.eq("when") {
                        let test_value = x
                            .attrs
                            .get("test")
                            .expect(&format!("{} element must be have test field!", x.tag));
                        let mut if_tag_body = quote! {};
                        if x.childs.len() != 0 {
                            if_tag_body = parse(&x.childs, methods, ignore, fn_name);
                        }
                        impl_if(
                            test_value,
                            if_tag_body,
                            &mut inner_body,
                            methods,
                            quote! {return sql;},
                            ignore,
                        );
                    }
                    if x.tag.eq("otherwise") {
                        let child_body = parse(&x.childs, methods, ignore, fn_name);
                        impl_otherwise(child_body, &mut inner_body, methods, ignore);
                    }
                }
                let cup = x.child_string_cup();
                body = quote! {
                  #body
                  sql.push_str(&|| -> String {
                           let mut sql = String::with_capacity(#cup);
                           #inner_body
                           return sql;
                  }());
                }
            }

            "foreach" => {
                let empty_string = String::new();

                let def_item = "item".to_string();
                let def_index = "index".to_string();

                let collection = x
                    .attrs
                    .get("collection")
                    .unwrap_or(&empty_string)
                    .to_string();
                let mut item = x.attrs.get("item").unwrap_or(&def_item).to_string();
                let mut findex = x.attrs.get("index").unwrap_or(&def_index).to_string();
                let open = x.attrs.get("open").unwrap_or(&empty_string).to_string();
                let close = x.attrs.get("close").unwrap_or(&empty_string).to_string();
                let separator = x
                    .attrs
                    .get("separator")
                    .unwrap_or(&empty_string)
                    .to_string();

                if item.is_empty() {
                    item = def_item;
                }
                if findex.is_empty() {
                    findex = def_index;
                }
                let mut ignores = ignore.clone();
                ignores.push(findex.to_string());
                ignores.push(item.to_string());

                let impl_body = parse(&x.childs, methods, &mut ignores, fn_name);
                let method_impl = crate::codegen::func::impl_fn(
                    &body.to_string(),
                    "",
                    &format!("\"{}\"", collection),
                    false,
                    ignore,
                );
                body = quote! {
                    #body
                };
                let mut open_impl = quote! {};
                if !open.is_empty() {
                    open_impl = quote! {
                    sql.push_str(#open);
                    };
                }
                let mut close_impl = quote! {};
                if !close.is_empty() {
                    close_impl = quote! {sql.push_str(#close);};
                }
                let item_ident = Ident::new(&item, Span::call_site());
                let index_ident = Ident::new(&findex, Span::call_site());
                let mut split_code = quote! {};
                let mut split_code_trim = quote! {};
                if !separator.is_empty() {
                    split_code = quote! {
                        sql.push_str(#separator);
                    };
                    split_code_trim = quote! {
                       sql = sql.trim_end_matches(#separator).to_string();
                    };
                }
                body = quote! {
                    #body
                    #open_impl
                    for (#index_ident,#item_ident) in #method_impl {
                        #impl_body
                        #split_code
                    }
                    #split_code_trim
                    #close_impl
                };
                body = quote! {
                    #body
                };
            }

            "set" => {
                impl_trim(
                    " set ", " ", " |,", " |,", x, &mut body, arg, methods, ignore, fn_name,
                );
            }

            "select" => {
                let id = x
                    .attrs
                    .get("id")
                    .expect("<select> element must be have id!");
                let method_name = Ident::new(fn_name, Span::call_site());
                let child_body = parse(&x.childs, methods, ignore, fn_name);
                let cup = x.child_string_cup();
                let select = quote! {
                    pub fn #method_name (arg:&rbs::Value, _tag: char) -> (String,Vec<rbs::Value>) {
                       use rbatis_codegen::ops::*;
                       let mut sql = String::with_capacity(#cup);
                       let mut args = Vec::with_capacity(20);
                       #child_body
                       #fix_sql
                       return (sql,args);
                    }
                };
                body = quote! {
                    #body
                    #select
                };
            }
            "update" => {
                let id = x
                    .attrs
                    .get("id")
                    .expect("<update> element must be have id!");
                let method_name = Ident::new(fn_name, Span::call_site());
                let child_body = parse(&x.childs, methods, ignore, fn_name);
                let cup = x.child_string_cup();
                let select = quote! {
                    pub fn #method_name (arg:&rbs::Value, _tag: char) -> (String,Vec<rbs::Value>) {
                       use rbatis_codegen::ops::*;
                       let mut sql = String::with_capacity(#cup);
                       let mut args = Vec::with_capacity(20);
                       #child_body
                       #fix_sql
                       return (sql,args);
                    }
                };
                body = quote! {
                    #body
                    #select
                };
            }
            "insert" => {
                let id = x
                    .attrs
                    .get("id")
                    .expect("<insert> element must be have id!");
                let method_name = Ident::new(fn_name, Span::call_site());
                let child_body = parse(&x.childs, methods, ignore, fn_name);
                let cup = x.child_string_cup();
                let select = quote! {
                    pub fn #method_name (arg:&rbs::Value, _tag: char) -> (String,Vec<rbs::Value>) {
                       use rbatis_codegen::ops::*;
                       let mut sql = String::with_capacity(#cup);
                       let mut args = Vec::with_capacity(20);
                       #child_body
                       #fix_sql
                       return (sql,args);
                    }
                };
                body = quote! {
                    #body
                    #select
                };
            }
            "delete" => {
                let id = x
                    .attrs
                    .get("id")
                    .expect("<delete> element must be have id!");
                let method_name = Ident::new(fn_name, Span::call_site());
                let child_body = parse(&x.childs, methods, ignore, fn_name);
                let cup = x.child_string_cup();
                let select = quote! {
                    pub fn #method_name (arg:&rbs::Value, _tag: char) -> (String,Vec<rbs::Value>) {
                       use rbatis_codegen::ops::*;
                       let mut sql = String::with_capacity(#cup);
                       let mut args = Vec::with_capacity(20);
                       #child_body
                       #fix_sql
                       return (sql,args);
                    }
                };
                body = quote! {
                    #body
                    #select
                };
            }
            _ => {}
        }
    }

    return body.into();
}

fn remove_extra(txt: &str) -> String {
    let txt = txt.trim().replace("\\r", "");
    let lines: Vec<&str> = txt.split("\n").collect();
    let mut data = String::with_capacity(txt.len());
    let mut index = 0;
    for line in &lines {
        let mut line = line.trim_start().trim_end();
        if line.starts_with("`") && line.ends_with("`") {
            line = line.trim_start_matches("`").trim_end_matches("`");
        }
        data.push_str(line);
        if index + 1 < lines.len() {
            data.push_str("\n");
        }
        index += 1;
    }
    if data.starts_with("`") && data.ends_with("`") {
        data.trim_start_matches("`")
            .trim_end_matches("`")
            .to_string();
    }
    data = data.replace("``", "").to_string();
    data
}

fn impl_continue(x: &Element, body: &mut proc_macro2::TokenStream, ignore: &mut Vec<String>) {
    *body = quote! {
         #body
         continue
    };
}

fn gen_method_name(test_value: &str) -> (String, Ident) {
    let method_name_string = encode(&test_value)
        .replace("_", "__")
        .replace("=", "_")
        .replace("+", "add");
    (
        method_name_string.clone(),
        Ident::new(&method_name_string, Span::call_site()),
    )
}

fn impl_if(
    test_value: &str,
    if_tag_body: proc_macro2::TokenStream,
    body: &mut proc_macro2::TokenStream,
    methods: &mut proc_macro2::TokenStream,
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
             #if_tag_body
             #appends
          }
    };
}

fn impl_otherwise(
    child_body: proc_macro2::TokenStream,
    body: &mut proc_macro2::TokenStream,
    methods: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
) {
    *body = quote!(
           #body
           #child_body
    );
}

fn impl_trim(
    prefix: &str,
    suffix: &str,
    prefixOverrides: &str,
    suffixOverrides: &str,
    x: &Element,
    body: &mut proc_macro2::TokenStream,
    arg: &Vec<Element>,
    methods: &mut proc_macro2::TokenStream,
    ignore: &mut Vec<String>,
    fn_name: &str,
) {
    let trim_body = parse(&x.childs, methods, ignore, fn_name);
    let prefixs: Vec<&str> = prefixOverrides.split("|").collect();
    let suffixs: Vec<&str> = suffixOverrides.split("|").collect();
    let have_trim = prefixs.len() != 0 && suffixs.len() != 0;
    let cup = x.child_string_cup();
    let mut trims = quote! {
         let mut sql= String::with_capacity(#cup);
         #trim_body
         sql=sql
    };
    for x in prefixs {
        trims = quote! {
            #trims
            .trim_start_matches(#x)
        }
    }
    for x in suffixs {
        trims = quote! {
            #trims
            .trim_end_matches(#x)
        }
    }
    if !prefix.is_empty() {
        *body = quote! {
           #body
           sql.push_str(#prefix);
        };
    }
    if have_trim {
        *body = quote! {
           #body
           sql.push_str(&{#trims.to_string(); sql });
        };
    }
    if !suffix.is_empty() {
        *body = quote! {
           #body
           sql.push_str(#suffix);
        };
    }
}

pub fn impl_fn_html(m: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let current_dir = current_dir().unwrap();
    let fn_name = m.sig.ident.to_string();
    let mut html_data = args.get(0).to_token_stream().to_string();
    let mut t;
    let mut format_char = '?';
    if args.len() > 1 {
        for x in args.get(1).to_token_stream().to_string().chars() {
            if x != '\'' && x != '"' {
                format_char = x;
                break;
            }
        }
    }
    t = parse_html(&html_data, &fn_name, &mut vec![]);
    return t.into();
}

/// parse to expr
fn parse_expr(lit_str: &str) -> Expr {
    let s = syn::parse::<syn::LitStr>(lit_str.to_token_stream().into())
        .expect(&format!("parse::<syn::LitStr> fail: {}", lit_str));
    return syn::parse_str::<Expr>(&s.value())
        .expect(&format!("parse_str::<Expr> fail: {}", lit_str));
}

/// parse to expr
fn parse_path(lit_str: &str) -> Path {
    let s = syn::parse::<syn::LitStr>(lit_str.to_token_stream().into())
        .expect(&format!("parse::<syn::LitStr> fail: {}", lit_str));
    return syn::parse_str::<Path>(&s.value())
        .expect(&format!("parse_str::<Path> fail: {}", lit_str));
}
