use crate::macros::py_sql_impl;
use crate::proc_macro::TokenStream;
use crate::util::{
    find_fn_body, find_return_type, get_fn_args, is_page_request, is_page_return_type, is_query,
    is_rb_ref, macro_span,
};
use crate::ParseArgs;
use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::spanned::Spanned;
use syn::{FnArg, ItemFn, ItemImpl};

pub(crate) fn impl_macro_html_sql(
    target_fn: &ItemFn,
    args: &ParseArgs,
    is_trait_impl: bool,
) -> syn::Result<TokenStream> {
    let span = macro_span(args, target_fn);
    let return_ty = find_return_type(target_fn);
    let return_ty_str = return_ty.to_string();

    // Check if return type is Page<T> - if so, generate pagination code
    if let Some(inner_type) = is_page_return_type(&return_ty_str) {
        return impl_macro_html_sql_with_page(target_fn, args, &inner_type, is_trait_impl);
    }

    // Original non-paginated implementation
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
    // the template exactly as it was written (LitStr::value() keeps `"` and newlines)
    let mut inline_html = String::new();
    if args.sqls.is_empty() {
        return Err(syn::Error::new(
            span,
            "[rb] Incorrect macro parameter length! The #[html_sql] attribute needs the HTML text or file name",
        ));
    }
    if rbatis_name.is_empty() {
        return Err(syn::Error::new(
            target_fn.sig.ident.span(),
            format!(
                "[rb] you should add rbatis ref param   `rb:&dyn Executor`  on '{}()'!",
                target_fn.sig.ident
            ),
        ));
    }
    for v in &args.sqls {
        inline_html.push_str(v.value().as_str());
    }
    let mut file_name = inline_html.trim().to_string();
    if file_name.ends_with(".html\"") {
        file_name = file_name
            .trim_start_matches("\"")
            .trim_end_matches("\"")
            .to_string();
    }
    let template_text = if file_name.ends_with(".html") {
        //relative path append realpath
        let file_path = PathBuf::from(file_name.clone());
        if file_path.is_relative() {
            let mut manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
                .map_err(|_| syn::Error::new(span, "Failed to read CARGO_MANIFEST_DIR"))?;
            manifest_dir.push('/');
            let mut current = PathBuf::from(manifest_dir);
            current.push(file_name.clone());
            if !current.exists() {
                current = current_dir().map_err(|e| {
                    syn::Error::new(span, format!("Failed to read the current directory: {}", e))
                })?;
                current.push(file_name.clone());
            }
            file_name = current.to_str().unwrap_or_default().to_string();
        }
        let mut html_data = String::new();
        let mut f = File::open(file_name.as_str()).map_err(|_| {
            syn::Error::new(span, format!("File Name = '{}' does not exist", file_name))
        })?;
        f.read_to_string(&mut html_data).map_err(|e| {
            syn::Error::new(span, format!("{} read_to_string fail: {}", file_name, e))
        })?;
        let htmls = rbatis_codegen::codegen::parser_html::load_mapper_map(&html_data)
            .map_err(|e| syn::Error::new(span, format!("[rb] load '{}' fail: {}", file_name, e)))?;
        if !htmls.contains_key(&func_name_ident.to_string()) {
            return Err(syn::Error::new(
                span,
                format!(
                    "[rb] '{}' has no element with id='{}'. An html_sql file needs <select id=\"fn_name\"> (or <update>/<insert>/<delete>) with the id of the function.",
                    file_name, func_name_ident
                ),
            ));
        }
        // pass the whole file: the generated helper has the id of the function, so
        // rbatis_codegen selects the right statement and keeps the original line numbers
        html_data
    } else {
        // inline html. rbatis_codegen reads the statement with the id of the generated
        // helper (named after the function), so a template with more than one statement
        // needs that id. Report the ambiguous template here instead of compiling the
        // alphabetically first statement.
        let fn_name = func_name_ident.to_string();
        // a template that does not load is left to rbatis_codegen, which reports it
        if let Ok(htmls) = rbatis_codegen::codegen::parser_html::load_mapper_map(&inline_html) {
            if htmls.len() > 1 && !htmls.contains_key(&fn_name) {
                // a `<sql id="..">` of the template is a fragment that another statement
                // includes, not a statement of its own
                let statement_count = htmls
                    .values()
                    .filter(|element| {
                        matches!(
                            element.tag.as_str(),
                            "select" | "insert" | "update" | "delete" | "set"
                        )
                    })
                    .count();
                // one single statement is unambiguous even without the id of the function
                if statement_count != 1 {
                    return Err(syn::Error::new(
                        span,
                        format!(
                            "[rb] the inline html of #[html_sql] on '{}()' has the statements [{}], but none with id='{}'. Give the statement you want to compile the id of the function (<select id=\"{}\">).",
                            fn_name,
                            htmls.keys().cloned().collect::<Vec<String>>().join(", "),
                            fn_name,
                            fn_name,
                        ),
                    ));
                }
            }
        }
        inline_html.clone()
    };
    let template_lit = syn::LitStr::new(&template_text, span);
    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let fn_body = find_fn_body(target_fn);
    let is_async = target_fn.sig.asyncness.is_some();
    if !is_async {
        return Err(syn::Error::new(
            target_fn.sig.ident.span(),
            format!(
                "[rbatis] fn {}({}) must be  async fn! ",
                func_name_ident, func_args_stream
            ),
        ));
    }
    if rbatis_ident.to_string().starts_with("mut ") {
        rbatis_ident = Ident::new(
            rbatis_ident.to_string().trim_start_matches("mut "),
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
    // the helper carries the name of the generated function: rbatis_codegen uses it
    // as the id of the statement to compile
    let helper_ident = &target_fn.sig.ident;
    let gen_target_method = quote! {
        #[rbatis::rb_html(#template_lit)]
        pub fn #helper_ident(arg: &rbs::Value, _tag: char) {}
    };
    let gen_target_macro_arg = quote! {
        #template_lit
    };
    let gen_func: proc_macro2::TokenStream =
        rbatis_codegen::rb_html(gen_target_macro_arg.into(), gen_target_method.into())?.into();

    // with the debug_mode feature the html file is added to the compile input so that a
    // change of the file re-runs the macro
    #[allow(unused_mut)]
    let mut include_data = quote!();
    #[cfg(feature = "debug_mode")]
    if cfg!(debug_assertions) && file_name.ends_with(".html") {
        let current_dir = current_dir().map_err(|e| {
            syn::Error::new(span, format!("Failed to read the current directory: {}", e))
        })?;
        let mut html_file_name = file_name.clone();
        if !PathBuf::from(&file_name).is_absolute() {
            html_file_name = format!("{}/{}", current_dir.to_str().unwrap_or_default(), file_name);
        }
        include_data = quote! {#include_data  let _ = include_bytes!(#html_file_name);};
    }
    let generic = target_fn.sig.generics.clone();
    //gen rust code
    let push_count = sql_args_gen
        .to_string()
        .matches("rb_arg_map.insert")
        .count();

    let visibility = if is_trait_impl {
        quote!()
    } else {
        target_fn.vis.to_token_stream()
    };
    Ok(quote! {
       #visibility async fn #func_name_ident #generic(#func_args_stream) -> #return_ty {
         #include_data
         let mut rb_arg_map = rbs::value::map::ValueMap::with_capacity(#push_count);
         #sql_args_gen
         #fn_body
         use rbatis::executor::{RBatisRef};
         let driver_type = #rbatis_ident.rb_ref().driver_type()?;
         use rbatis::rbatis_codegen;
         #gen_func
         let (mut sql,rb_args) = #helper_ident(rbs::Value::Map(rb_arg_map),'?');
         #call_method
       }
    }
    .into())
}

/// Generate paginated html_sql implementation
/// When Page<T> is detected, generate a call to htmlsql_select_page! macro
fn impl_macro_html_sql_with_page(
    target_fn: &ItemFn,
    args: &ParseArgs,
    inner_type: &str,
    _is_trait_impl: bool,
) -> syn::Result<TokenStream> {
    let func_name = target_fn.sig.ident.clone();
    // the `T` of the `Page<T>` return type
    let inner_type_ts: proc_macro2::TokenStream = inner_type.parse().map_err(|e| {
        syn::Error::new(
            target_fn.sig.ident.span(),
            format!(
                "[rb] parse the inner type '{}' of Page<{}> fail: {}",
                inner_type, inner_type, e
            ),
        )
    })?;

    // Extract business parameters (exclude rb/executor and page_request)
    // Use the types as-is (htmlsql_select_page! macro will handle adding & where needed)
    let mut business_params = Vec::new();
    for arg in &target_fn.sig.inputs {
        if let FnArg::Typed(t) = arg {
            let ty_str = t.ty.to_token_stream().to_string();
            // Skip executor and page_request parameters
            if !is_rb_ref(&ty_str) && !is_page_request(&ty_str) {
                business_params.push((t.pat.clone(), t.ty.as_ref().clone()));
            }
        }
    }

    // Build business parameters for htmlsql_select_page! macro
    let params_for_macro: Vec<_> = business_params
        .iter()
        .map(|(pat, ty)| {
            quote! { #pat: #ty }
        })
        .collect();

    // Build html file arguments
    let html_file_args: Vec<_> = args
        .sqls
        .iter()
        .map(|sql| {
            quote! { #sql }
        })
        .collect();

    // Generate: rbatis::htmlsql_select_page!(func_name(params) -> Type => "file.html")
    // Note: htmlsql_select_page! macro already handles pub visibility
    Ok(quote! {
        rbatis::htmlsql_select_page!(#func_name(#( #params_for_macro ),*) -> #inner_type_ts => #( #html_file_args ),*);
    }
    .into())
}

pub(crate) fn impl_macro_html_sql_impl(
    impl_block: &ItemImpl,
    args: &ParseArgs,
) -> syn::Result<TokenStream> {
    // Get all items in the impl block
    let items = &impl_block.items;

    // Check if impl block is empty
    if items.is_empty() {
        return Err(syn::Error::new(
            impl_block.self_ty.span(),
            "#[html_sql] applied to impl block requires at least one item",
        ));
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
                modifiers: syn::FnModifiers::default(),
                sig: func.sig.clone(),
                block: Box::new(func.block.clone()),
            };

            // Generate html sql implementation for each function. A broken template
            // keeps a stub with the original signature so that call sites do not add a
            // second "cannot find function" error.
            let is_trait_impl = impl_block.trait_.is_some();
            match impl_macro_html_sql(&item_fn, args, is_trait_impl) {
                Ok(func_stream) => processed_items.push(func_stream.into()),
                Err(e) => processed_items.push(crate::util::error_with_stub(e, &item_fn).into()),
            }
        } else {
            // Preserve other types of items
            processed_items.push(item.to_token_stream());
        }
    }

    // Check if impl block contains at least one function
    if !has_function {
        return Err(syn::Error::new(
            impl_block.self_ty.span(),
            "#[html_sql] applied to impl block requires at least one function",
        ));
    }

    // Reconstruct impl block
    let attrs = &impl_block.attrs;
    let defaultness = &impl_block.modifiers.defaultness;
    let polarity = &impl_block.modifiers.polarity;
    let unsafety = &impl_block.unsafety;
    let impl_token = &impl_block.impl_token;
    let generics = &impl_block.generics;
    let trait_ = &impl_block.trait_;
    let self_ty = &impl_block.self_ty;

    // Manually handle trait_ field because its type is complex
    let trait_tokens = match trait_ {
        Some((path, for_token)) => {
            quote! { #polarity #path #for_token }
        }
        None => quote! {},
    };

    // Generate final TokenStream
    Ok(quote! {
        #(#attrs)*
        #defaultness #unsafety #impl_token #generics #trait_tokens #self_ty {
            #(#processed_items)*
        }
    }
    .into())
}
