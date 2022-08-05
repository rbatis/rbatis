use std::collections::HashMap;
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


use crate::html_loader::{Element, load_html};
use crate::proc_macro::TokenStream;
use crate::py_sql::{NodeType, ParsePySql};
use crate::string_util::find_convert_string;

fn parse_html_str(html: &str, fn_name: &str, ignore: &mut Vec<String>) -> proc_macro2::TokenStream {
    let datas = load_html(html).expect("load_html() fail!");
    let mut sql_map = HashMap::new();
    let datas = include_replace(datas, &mut sql_map);
    drop(sql_map);
    for x in datas {
        if x.tag.eq("mapper") {
            for x in x.childs {
                match x.attrs.get("id") {
                    Some(id) => {
                        if id.eq(fn_name) {
                            let node= parse_html_node(vec![x], ignore);
                            return node;
                        }
                    }
                    _ => {}
                }
            }

            break;
        }
    }
    panic!("html not find fn:{}", fn_name);
}


fn find_element(id: &str, htmls: &Vec<Element>) -> Option<Element> {
    for x in htmls {
        if x.childs.len() != 0 {
            let find = find_element(id, &x.childs);
            if find.is_some() {
                return find;
            }
        }
        match x.attrs.get("id") {
            None => {}
            Some(id) => {
                if id.eq(id) {
                    return Some(x.clone());
                }
            }
        }
    }
    return None;
}

fn include_replace(htmls: Vec<Element>, sql_map: &mut HashMap<String, Vec<Element>>) -> Vec<Element> {
    let mut results = vec![];
    for mut x in htmls {
        match x.tag.as_str() {
            "sql" => {
                sql_map.insert(x.attrs.get("id").expect("[rbatis] <sql> element must have id!").clone(), x.childs.clone());
            }
            "include" => {
                let refid = x.attrs.get("refid").expect("[rbatis] <include> element must have refid!").clone();
                let data_url = Url::parse(format!("url://{}", refid).as_str()).expect("[rbatis] parse include url fail!");
                let mut find_file = false;
                for (k, v) in data_url.query_pairs() {
                    let v = v.to_string();
                    match k.to_string().as_str() {
                        "file" => {
                            if v.is_empty() {
                                panic!("[rbatis] <include> element file must be have an value!");
                            }
                            let mut html_string = String::new();
                            let mut f = File::open(v.as_str()).expect(&format!("File:\"{}\" does not exist", v));
                            f.read_to_string(&mut html_string);
                            let datas = load_html(html_string.as_str()).expect("load_html() fail!");
                            let find = find_element(refid.as_str(), &datas).expect(&format!("[rbatis] not find html:{} , element id={}", v, refid));
                            x.childs.push(find);
                            find_file = true;
                            break;
                        }
                        &_ => {}
                    }
                }
                if !find_file {
                    let refid_path = data_url.host().unwrap().to_string();
                    let element = sql_map.get(refid_path.as_str()).expect(&format!("[rbatis] can not find element {} <include refid='{}'> !", refid, refid_path)).clone();
                    for el in element {
                        x.childs.push(el.clone());
                    }
                }
            }
            _ => {
                match x.attrs.get("id") {
                    None => {}
                    Some(id) => {
                        if !id.is_empty() {
                            sql_map.insert(id.clone(), x.childs.clone());
                        }
                    }
                }
            }
        }
        if x.childs.len() != 0 {
            x.childs = include_replace(x.childs.clone(), sql_map);
        }

        match x.tag.as_str() {
            "include" | "sql" => {
                for c in x.childs {
                    results.push(c);
                }
            }
            _ => {
                results.push(x);
            }
        }
    }
    return results;
}

fn parse_html_node(htmls: Vec<Element>, ignore: &mut Vec<String>) -> proc_macro2::TokenStream {
    let mut methods = quote!();
    let fn_impl = parse(&htmls, &mut methods, "", ignore);
    let token = quote! {
        use rbatis_sql::ops::*;
        #methods
        #fn_impl
    };
    token
}

fn to_mod(m: &ItemMod, t: &proc_macro2::TokenStream) -> TokenStream {
    let ident = &m.ident;
    let mod_token = quote! {
        pub mod #ident{
            #t
        }
    };
    mod_token.into()
}


/// gen rust code
fn parse(arg: &Vec<Element>, methods: &mut proc_macro2::TokenStream, block_name: &str, ignore: &mut Vec<String>) -> proc_macro2::TokenStream {
    let mut body = quote! {};
    let fix_sql = quote! {
        rbatis_sql::sql_index!(sql,_tag);
    };
    for x in arg {
        match x.tag.as_str() {
            "mapper" => {
                return parse(&x.childs, methods, "mapper", ignore);
            }
            "sql" => {
                return parse(&x.childs, methods, "sql", ignore);
            }
            "include" => {
                return parse(&x.childs, methods, "include", ignore);
            }
            "println" => {
                impl_println(x, &mut body, ignore);
            }
            "" => {
                let mut string_data = x.data.to_string();
                let convert_list = find_convert_string(&string_data);
                let mut replaces = quote! {};

                let mut replaced = HashMap::<String, bool>::new();
                for (k, v) in convert_list {
                    let (method_name_string, method_name) = gen_method_name(&format!("{}:{}", block_name, k));
                    let method_impl = crate::func::impl_fn(&body.to_string(), "", &format!("\"{}\"", k), false, true, ignore);
                    //check append value
                    body = quote! {
                              #body
                              let #method_name = #method_impl;
                          };
                    if v.starts_with("#") {
                        string_data = string_data.replacen(&v, &"?", 1);
                        body = quote! {
                              #body
                              args.push(rbs::to_value!(#method_name));
                          };
                    } else {
                        if replaced.get(&v).is_none() {
                            replaces = quote! {#replaces.replacen(#v, &#method_name.as_sql(), 1)};
                            replaced.insert(v.to_string(), true);
                        }
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
                         sql.push_str(#string_data#replaces);
                       );
                }
            }
            "if" => {
                let test_value = x.attrs.get("test").expect(&format!("{} element must be have test field!", x.tag));
                let mut if_tag_body = quote! {};
                if x.childs.len() != 0 {
                    if_tag_body = parse(&x.childs, methods, block_name, ignore);
                }
                impl_if(test_value, if_tag_body, &mut body, methods, quote! {}, &format!("{}:{}", block_name, "if"), ignore);
            }
            "trim" => {
                let empty_string = String::new();
                let prefix = x.attrs.get("prefix").unwrap_or(&empty_string).to_string();
                let suffix = x.attrs.get("suffix").unwrap_or(&empty_string).to_string();
                let prefixOverrides = x.attrs.get("prefixOverrides").unwrap_or(&empty_string).to_string();
                let suffixOverrides = x.attrs.get("suffixOverrides").unwrap_or(&empty_string).to_string();
                impl_trim(&prefix, &suffix, &prefixOverrides, &suffixOverrides, x, &mut body, arg, methods, &format!("{}:{}", block_name, "trim"), ignore);
            }
            "bind" => {
                let name = x.attrs.get("name").expect("<bind> must be have name!").to_string();
                let value = x.attrs.get("value").expect("<bind> element must be have value!").to_string();

                let name_expr = parse_expr(&name);
                let method_impl = crate::func::impl_fn(&body.to_string(), "", &format!("\"{}\"", value), false, true, ignore);
                body = quote! {
                            #body
                            //bind
                            let #name_expr = rbs::to_value!(#method_impl);
                        };
                ignore.push(name);
            }

            "where" => {
                impl_trim(" where ", " ", " |and |or ", " | and| or", x, &mut body, arg, methods, &format!("{}:{}", block_name, "where:trim"), ignore);
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
                        let test_value = x.attrs.get("test").expect(&format!("{} element must be have test field!", x.tag));
                        let mut if_tag_body = quote! {};
                        if x.childs.len() != 0 {
                            if_tag_body = parse(&x.childs, methods, block_name, ignore);
                        }
                        impl_if(test_value, if_tag_body, &mut inner_body, methods, quote! {return sql;}, &format!("{}:{}", block_name, "choose:when:if"), ignore);
                    }
                    if x.tag.eq("otherwise") {
                        let child_body = parse(&x.childs, methods, block_name, ignore);
                        impl_otherwise(child_body, &mut inner_body, methods, &format!("{}:{}", block_name, "choose:otherwise"), ignore);
                    }
                }
                body = quote! {
                #body
                let mut do_choose = || -> String {
                         let mut sql = String::new();
                         #inner_body
                         return sql;
                };
                sql.push_str(&do_choose());
              }
            }

            "foreach" => {
                let empty_string = String::new();

                let def_item = "item".to_string();
                let def_index = "index".to_string();

                let collection = x.attrs.get("collection").unwrap_or(&empty_string).to_string();
                let mut item = x.attrs.get("item").unwrap_or(&def_item).to_string();
                let mut findex = x.attrs.get("index").unwrap_or(&def_index).to_string();
                let open = x.attrs.get("open").unwrap_or(&empty_string).to_string();
                let close = x.attrs.get("close").unwrap_or(&empty_string).to_string();
                let separator = x.attrs.get("separator").unwrap_or(&empty_string).to_string();

                if item.is_empty() {
                    item = def_item;
                }
                if findex.is_empty() {
                    findex = def_index;
                }
                let mut ignores = ignore.clone();
                ignores.push(findex.to_string());
                ignores.push(item.to_string());

                let impl_body = parse(&x.childs, methods, "foreach", &mut ignores);
                let (method_name_string, method_name) = gen_method_name(&format!("{}:{}", block_name, collection));
                let method_impl = crate::func::impl_fn(&body.to_string(), "", &format!("\"{}\"", collection), false, false, ignore);
                //check append value
                body = quote! {
                              #body
                              let #method_name = #method_impl;
                          };
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
                    close_impl = quote! {
                    sql.push_str(#close);
                    };
                }
                let item_ident = Ident::new(&item, Span::call_site());
                let index_ident = Ident::new(&findex, Span::call_site());

                let mut index_create = quote! {};
                let mut index_add = quote! {};
                let mut split_code = quote! {};
                let mut split_code_end = quote! {};
                index_create = quote! {
                        let mut #index_ident=0;
                    };
                index_add = quote! {
                        #index_ident=#index_ident+1;
                    };

                if !separator.is_empty() {
                    split_code = quote! {    sql.push_str(#separator);  };
                    split_code_end = quote! {
                        if foreach_arr.len() != 0 {
                             sql.pop();
                        }
                    };
                }

                body = quote! {
                    #body
                    if #method_name.is_array(){
                        #open_impl
                        #index_create
                        {
                          let foreach_arr = #method_name.array().unwrap();
                          for #item_ident in foreach_arr {
                            #impl_body
                            #split_code
                            #index_add
                          }
                          #split_code_end
                        }
                        #close_impl
                    }else if #method_name.is_object(){
                        #open_impl
                        {
                          let foreach_arr = #method_name.object().unwrap();
                          for (#index_ident,#item_ident) in foreach_arr {
                              #impl_body
                              #split_code
                          }
                          #split_code_end
                        }
                        #close_impl
                    }
                };
                body = quote! {
                    #body
                };
            }

            "set" => {
                impl_trim(" set ", " ", " |,", " |,", x, &mut body, arg, methods, &format!("{}:{}", block_name, "set:trim"), ignore);
            }

            "select" => {
                let id = x.attrs.get("id").expect("<select> element must be have id!");
                let method_name = Ident::new(id, Span::call_site());
                let child_body = parse(&x.childs, methods, "select", ignore);
                let select = quote! {
                            pub fn #method_name (arg:&rbs::Value, _tag: char) -> (String,Vec<rbs::Value>) {
                               use rbatis_sql::ops::*;
                               let mut sql = String::with_capacity(1000);
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
                let id = x.attrs.get("id").expect("<update> element must be have id!");
                let method_name = Ident::new(id, Span::call_site());
                let child_body = parse(&x.childs, methods, "select", ignore);
                let select = quote! {
                            pub fn #method_name (arg:&rbs::Value, _tag: char) -> (String,Vec<rbs::Value>) {
                               use rbatis_sql::ops::*;

                               let mut sql = String::with_capacity(1000);
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
                let id = x.attrs.get("id").expect("<insert> element must be have id!");
                let method_name = Ident::new(id, Span::call_site());
                let child_body = parse(&x.childs, methods, "select", ignore);
                let select = quote! {
                            pub fn #method_name (arg:&rbs::Value, _tag: char) -> (String,Vec<rbs::Value>) {
                               use rbatis_sql::ops::*;

                               let mut sql = String::with_capacity(1000);
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
                let id = x.attrs.get("id").expect("<delete> element must be have id!");
                let method_name = Ident::new(id, Span::call_site());
                let child_body = parse(&x.childs, methods, "select", ignore);
                let select = quote! {
                            pub fn #method_name (arg:&rbs::Value, _tag: char) -> (String,Vec<rbs::Value>) {
                               use rbatis_sql::ops::*;

                               let mut sql = String::with_capacity(1000);
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


fn impl_println(x: &Element, body: &mut proc_macro2::TokenStream, ignore: &mut Vec<String>) {
    let value = x.attrs.get("value").expect(&format!("{} element must be have value field!", x.tag));
    let method_name = impl_method(value, body, ignore);
    let mut format = String::new();
    if let Some(s) = x.attrs.get("format") {
        format = s.to_string();
    }
    if format.is_empty() {
        *body = quote! {
                   #body
                   println!("{}",#method_name);
                  };
    } else {
        let format_expr = syn::parse_str::<syn::Lit>(&format!("\"{}\"", format)).expect(&format!("[rexpr]syn::parse_str: {}", format));
        *body = quote! {
                   #body
                   println!(#format_expr,#method_name);
                  };
    }
}

fn gen_method_name(test_value: &str) -> (String, Ident) {
    let method_name_string = encode(&test_value)
        .replace("_", "__")
        .replace("=", "_")
        .replace("+", "add");
    (method_name_string.clone(), Ident::new(&method_name_string, Span::call_site()))
}


fn impl_method(test_value: &str, body: &mut proc_macro2::TokenStream, ignore: &mut Vec<String>) -> Ident {
    let (_, method_name) = gen_method_name(&test_value);
    let method_impl = crate::func::impl_fn(&body.to_string(), "", &format!("\"{}\"", test_value), false, true, ignore);
    //check append value
    *body = quote! {
                 #body
               let #method_name = #method_impl;
    };
    return method_name;
}

fn impl_if(test_value: &str, if_tag_body: proc_macro2::TokenStream, body: &mut proc_macro2::TokenStream, methods: &mut proc_macro2::TokenStream, appends: proc_macro2::TokenStream, block_name: &str, ignore: &mut Vec<String>) {
    let method_name = impl_method(test_value, body, ignore);
    *body = quote! {
                  #body
                  if #method_name {
                     #if_tag_body
                     #appends
                  }
            };
}

fn impl_otherwise(child_body: proc_macro2::TokenStream, body: &mut proc_macro2::TokenStream, methods: &mut proc_macro2::TokenStream, block_name: &str, ignore: &mut Vec<String>) {
    *body = quote!(
                   #body
                   #child_body
            );
}


fn impl_trim(prefix: &str, suffix: &str, prefixOverrides: &str, suffixOverrides: &str, x: &Element, body: &mut proc_macro2::TokenStream, arg: &Vec<Element>, methods: &mut proc_macro2::TokenStream, block_name: &str, ignore: &mut Vec<String>) {
    let trim_body = parse(&x.childs, methods, block_name, ignore);
    let prefixs: Vec<&str> = prefixOverrides.split("|").collect();
    let suffixs: Vec<&str> = suffixOverrides.split("|").collect();
    let have_trim = prefixs.len() != 0 && suffixs.len() != 0;
    let mut trims = quote! {
                     let mut sql=String::new();
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

    *body = quote! {
                   #body
                    sql.push_str(#prefix);
                };
    if have_trim {
        *body = quote! {
                   #body
                   sql.push_str(&{#trims.to_string(); sql });
                };
    }
    *body = quote! {
                   #body
                   sql.push_str(#suffix);
                };
}

pub fn impl_fn_html(m: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let current_dir = current_dir().unwrap();
    let fn_name = m.sig.ident.to_string();
    let mut file_name = args.get(0).to_token_stream().to_string();
    if file_name.ne("\"\"") && file_name.starts_with("\"") && file_name.ends_with("\"") {
        file_name = file_name[1..file_name.len() - 1].to_string();
    }
    let mut t;
    #[cfg(feature = "debug_mode")]
    {
        println!("try open file:{}", file_name);
    }
    let file_name_path_buf = PathBuf::from_str(&file_name).unwrap();

    let mut data = String::new();
    let mut f = File::open(file_name.as_str()).expect(&format!("File Name = '{}' does not exist", file_name));
    f.read_to_string(&mut data);


    let mut format_char = '?';
    if args.len() > 1 {
        for x in args.get(1).to_token_stream().to_string().chars() {
            if x != '\'' && x != '"' {
                format_char = x;
                break;
            }
        };
    }
    t = parse_html_str(&data, &fn_name, &mut vec![]);

    #[cfg(feature = "debug_mode")]
    {
        let id = Ident::new(&format!("_include_{}", fn_name), Span::call_site());
        let mut html_file_name = file_name.clone();
        if !file_name_path_buf.is_absolute() {
            html_file_name = format!("{}/{}", current_dir.to_str().unwrap_or_default(), file_name);
        }
        t = quote! {#t fn #id() {let _ = include_str!(#html_file_name);}};
    }
    return t.into();
}

pub fn impl_fn_py(m: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let fn_name = m.sig.ident.to_string();
    let mut data = args.get(0).to_token_stream().to_string();
    if data.ne("\"\"") && data.starts_with("\"") && data.ends_with("\"") {
        data = data[1..data.len() - 1].to_string();
    }
    let t;
    #[cfg(feature = "debug_mode")]
    {
        println!("py_sql:{}", data);
    }
    let mut format_char = '?';
    if args.len() > 1 {
        for x in args.get(1).to_token_stream().to_string().chars() {
            if x != '\'' && x != '"' {
                format_char = x;
                break;
            }
        };
    }
    let nodes = NodeType::parse(&data).expect("[rbatis] parse py_sql fail!");
    let htmls = crate::py_sql::to_html(&nodes, data.starts_with("select") || data.starts_with(" select"), &fn_name);
    #[cfg(feature = "debug_mode")]
    {
        println!("html:{}", htmls);
    }
    t = parse_html_str(&htmls, &fn_name, &mut vec![]);
    return t.into();
}

/// parse to expr
fn parse_expr(lit_str: &str) -> Expr {
    let s = syn::parse::<syn::LitStr>(lit_str.to_token_stream().into()).expect(&format!("parse::<syn::LitStr> fail: {}", lit_str));
    return syn::parse_str::<Expr>(&s.value()).expect(&format!("parse_str::<Expr> fail: {}", lit_str));
}

/// parse to expr
fn parse_path(lit_str: &str) -> Path {
    let s = syn::parse::<syn::LitStr>(lit_str.to_token_stream().into()).expect(&format!("parse::<syn::LitStr> fail: {}", lit_str));
    return syn::parse_str::<Path>(&s.value()).expect(&format!("parse_str::<Path> fail: {}", lit_str));
}