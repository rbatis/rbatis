use crate::macros::py_sql_impl;
use crate::proc_macro::TokenStream;
use crate::util::{find_fn_body, find_return_type, get_fn_args, is_query, is_rb_ref};
use crate::ParseArgs;
use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::{FnArg, Item, ItemFn, ItemImpl, ItemMod};

pub(crate) fn impl_macro_html_sql(target_fn: &ItemFn, args: &ParseArgs, is_trait_impl: bool) -> TokenStream {
    let return_ty = find_return_type(target_fn);
    let func_name_ident = target_fn.sig.ident.to_token_stream();

    let mut rbatis_ident = "".to_token_stream();
    let mut rbatis_name = String::new();
    for x in &target_fn.sig.inputs {
        match x {
            FnArg::Receiver(_) => {}
            FnArg::Typed(t) => {
                let ty_stream = t.ty.to_token_stream().to_string();
                if is_rb_ref(&ty_stream) {
                    rbatis_ident = t.pat.to_token_stream();
                    rbatis_name = rbatis_ident
                        .to_string()
                        .trim_start_matches("mut ")
                        .to_string();
                    break;
                }
            }
        }
    }
    let mut sql_ident = quote!();
    if args.sqls.len() >= 1 {
        if rbatis_name.is_empty() {
            panic!(
                "[rb] you should add rbatis ref param   `rb:&dyn Executor`  on '{}()'!",
                target_fn.sig.ident
            );
        }
        let mut s = "".to_string();
        for v in &args.sqls {
            s = s + v.value().as_str();
        }
        sql_ident = quote!(#s);
    } else {
        panic!("[rb] Incorrect macro parameter length!");
    }
    // sql_ident is html or file?
    let mut file_name = sql_ident.to_string().trim().to_string();
    if file_name.ends_with(".html\"") {
        file_name = file_name
            .trim_start_matches("\"")
            .trim_end_matches("\"")
            .to_string();
    }
    if file_name.ends_with(".html") {
        //relative path append realpath
        let file_path = PathBuf::from(file_name.clone());
        if file_path.is_relative() {
            let mut manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
                .expect("Failed to read CARGO_MANIFEST_DIR");
            manifest_dir.push_str("/");
            let mut current = PathBuf::from(manifest_dir);
            current.push(file_name.clone());
            if !current.exists() {
                current = current_dir().unwrap_or_default();
                current.push(file_name.clone());
            }
            file_name = current.to_str().unwrap_or_default().to_string();
        }
        let mut html_data = String::new();
        let mut f = File::open(file_name.as_str())
            .expect(&format!("File Name = '{}' does not exist", file_name));
        f.read_to_string(&mut html_data)
            .expect(&format!("{} read_to_string fail", file_name));
        let htmls = rbatis_codegen::codegen::parser_html::load_mapper_map(&html_data)
            .expect("load html content fail");
        let token = htmls.get(&func_name_ident.to_string()).expect("");
        let token = format!("{}", token);
        sql_ident = token.to_token_stream();
    }
    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let fn_body = find_fn_body(target_fn);
    let is_async = target_fn.sig.asyncness.is_some();
    if !is_async {
        panic!(
            "[rbaits] fn {}({}) must be  async fn! ",
            func_name_ident, func_args_stream
        );
    }
    if rbatis_ident.to_string().starts_with("mut ") {
        rbatis_ident = Ident::new(
            &rbatis_ident.to_string().trim_start_matches("mut "),
            Span::call_site(),
        )
        .to_token_stream();
    }
    //append all args
    let sql_args_gen = py_sql_impl::filter_args_context_id(&rbatis_name, &get_fn_args(target_fn));
    let is_query = is_query(&return_ty.to_string());
    let mut call_method = quote! {};
    if is_query {
        call_method = quote! {
             use rbatis::executor::{Executor};
             let r=#rbatis_ident.query(&sql,rb_args).await?;
             rbatis::decode::decode(r)
        };
    } else {
        call_method = quote! {
             use rbatis::executor::{Executor};
             #rbatis_ident.exec(&sql,rb_args).await
        };
    }
    let gen_target_method = quote! {
        #[rbatis::rb_html(#sql_ident)]
        pub fn impl_html_sql(arg: &rbs::Value, _tag: char) {}
    };
    let gen_target_macro_arg = quote! {
        #sql_ident
    };
    let gen_func: proc_macro2::TokenStream =
        rbatis_codegen::rb_html(gen_target_macro_arg.into(), gen_target_method.into()).into();

    let mut include_data = quote!();
    include_data = quote! {
        //no-debug_mode
    };
    #[cfg(feature = "debug_mode")]
    if cfg!(debug_assertions) {
        if file_name.ends_with(".html") {
            use std::env::current_dir;
            use std::path::PathBuf;
            let current_dir = current_dir().unwrap();
            let mut html_file_name = file_name.clone();
            if !PathBuf::from(&file_name).is_absolute() {
                html_file_name =
                    format!("{}/{}", current_dir.to_str().unwrap_or_default(), file_name);
            }
            include_data = quote! {#include_data  let _ = include_bytes!(#html_file_name);};
        }
    }
    let generic = target_fn.sig.generics.clone();
    //gen rust code
    let push_count = sql_args_gen
        .to_string()
        .matches("rb_arg_map.insert")
        .count();

    let visibility = if is_trait_impl { quote!() } else { target_fn.vis.to_token_stream() };
    return quote! {
       #visibility async fn #func_name_ident #generic(#func_args_stream) -> #return_ty {
         #include_data
         let mut rb_arg_map = rbs::value::map::ValueMap::with_capacity(#push_count);
         #sql_args_gen
         #fn_body
         use rbatis::executor::{RBatisRef};
         let driver_type = #rbatis_ident.rb_ref().driver_type()?;
         use rbatis::rbatis_codegen;
         #gen_func
         let (mut sql,rb_args) = impl_html_sql(rbs::Value::Map(rb_arg_map),'?');
         #call_method
       }
    }
    .into();
}

pub(crate) fn impl_macro_html_sql_module(module: &ItemMod, args: &ParseArgs) -> TokenStream {
    // Check if module has content
    let content = match &module.content {
        Some((_, items)) => items,
        None => {
            panic!("#[html_sql] applied to module requires inline content");
        }
    };
    
    // Generate code for each item in the module
    let mut processed_items = Vec::new();
    let mut has_function = false;
    
    for item in content {
        if let Item::Fn(func) = item {
            // Mark that a function was found
            has_function = true;
            
            // Generate html sql implementation for each function
            let func_stream = impl_macro_html_sql(func, args, false);
            processed_items.push(func_stream.into());
        } else {
            // Preserve other types of items
            processed_items.push(item.to_token_stream());
        }
    }
    
    // Check if module contains at least one function
    if !has_function {
        panic!("#[html_sql] applied to module requires at least one function");
    }
    
    // Reconstruct module content
    let module_ident = &module.ident;
    let module_vis = &module.vis;
    
    return quote! {
        #module_vis mod #module_ident {
            #(#processed_items)*
        }
    }
    .into();
}

pub(crate) fn impl_macro_html_sql_impl(impl_block: &ItemImpl, args: &ParseArgs) -> TokenStream {
    // Get all items in the impl block
    let items = &impl_block.items;
    
    // Check if impl block is empty
    if items.is_empty() {
        panic!("#[html_sql] applied to impl block requires at least one item");
    }
    
    // Generate code for each item
    let mut processed_items = Vec::new();
    let mut has_function = false;
    
    for item in items {
        if let syn::ImplItem::Fn(func) = item {
            // Mark that a function was found
            has_function = true;
            
            // Convert ImplItemFn to ItemFn for processing
            let item_fn = ItemFn {
                attrs: func.attrs.clone(),
                vis: func.vis.clone(),
                sig: func.sig.clone(),
                block: Box::new(func.block.clone()),
            };
            
            // Generate html sql implementation for each function
            let is_trait_impl = impl_block.trait_.is_some();
            let func_stream = impl_macro_html_sql(&item_fn, args, is_trait_impl);
            processed_items.push(func_stream.into());
        } else {
            // Preserve other types of items
            processed_items.push(item.to_token_stream());
        }
    }
    
    // Check if impl block contains at least one function
    if !has_function {
        panic!("#[html_sql] applied to impl block requires at least one function");
    }
    
    // Reconstruct impl block
    let attrs = &impl_block.attrs;
    let defaultness = &impl_block.defaultness;
    let unsafety = &impl_block.unsafety;
    let impl_token = &impl_block.impl_token;
    let generics = &impl_block.generics;
    let trait_ = &impl_block.trait_;
    let self_ty = &impl_block.self_ty;
    
    // Manually handle trait_ field because its type is complex
    let trait_tokens = match trait_ {
        Some((not, path, for_token)) => {
            quote! { #not #path #for_token }
        }
        None => quote! {}
    };
    
    // Generate final TokenStream
    return quote! {
        #(#attrs)*
        #defaultness #unsafety #impl_token #generics #trait_tokens #self_ty {
            #(#processed_items)*
        }
    }
    .into();
}
