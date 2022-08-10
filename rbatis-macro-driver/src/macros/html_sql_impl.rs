use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn::{AttributeArgs, FnArg, ItemFn, Lit, NestedMeta};

use crate::macros::py_sql_impl;
use crate::proc_macro::TokenStream;
use crate::util::{find_fn_body, find_return_type, get_fn_args, is_fetch, is_rbatis_ref};

pub(crate) fn impl_macro_html_sql(target_fn: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let return_ty = find_return_type(target_fn);
    let func_name_ident = target_fn.sig.ident.to_token_stream();

    let mut rbatis_ident = "".to_token_stream();
    let mut rbatis_name = String::new();
    for x in &target_fn.sig.inputs {
        match x {
            FnArg::Receiver(_) => {}
            FnArg::Typed(t) => {
                let ty_stream = t.ty.to_token_stream().to_string();
                if is_rbatis_ref(&ty_stream) {
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
    if args.len() >= 1 {
        if rbatis_name.is_empty() {
            panic!("[rbatis] you should add rbatis ref param  rb:&Rbatis  or rb: &mut Executor<'_,'_>  on '{}()'!", target_fn.sig.ident);
        }
        let mut s = "".to_string();
        for ele in args {
            match ele {
                NestedMeta::Meta(_) => {}
                NestedMeta::Lit(l) => match l {
                    Lit::Str(v) => {
                        s = s + v.value().as_str();
                    }
                    Lit::ByteStr(_) => {}
                    Lit::Byte(_) => {}
                    Lit::Char(_) => {}
                    Lit::Int(_) => {}
                    Lit::Float(_) => {}
                    Lit::Bool(_) => {}
                    Lit::Verbatim(_) => {}
                },
            }
        }
        sql_ident = quote!(#s);
    } else {
        panic!("[rbatis] Incorrect macro parameter length!");
    }
    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let fn_body = find_fn_body(target_fn);
    let is_async = target_fn.sig.asyncness.is_some();
    if !is_async {
        panic!(
            "[rbaits] #[crud_table] 'fn {}({})' must be  async fn! ",
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
    let is_fetch = is_fetch(&return_ty.to_string());
    let mut call_method = quote! {};
    if is_fetch {
        call_method = quote! {
             use rbatis::executor::{Executor};
             let r=#rbatis_ident.fetch(&sql,rb_args).await?;
             Ok(rbatis::decode::decode(r)?)
        };
    } else {
        call_method = quote! {
             use rbatis::executor::{Executor};
             let r=#rbatis_ident.exec(&sql,rb_args).await?;
             Ok(r)
        };
    }
    let gen_target_method = quote! {
        #[rbatis::rb_html(#sql_ident)]
        pub fn #func_name_ident(arg: &rbs::Value, _tag: char) {}
    };
    let gen_target_macro_arg = quote! {
        #sql_ident
    };
    let gen_func: proc_macro2::TokenStream = rbatis_codegen::rb_html(gen_target_macro_arg.into(), gen_target_method.into()).into();
    //gen rust code templete
    return quote! {
       pub async fn #func_name_ident(#func_args_stream) -> #return_ty {
         let mut rb_arg_map = rbs::value::map::ValueMap::new();
         #sql_args_gen
         #fn_body
         use rbatis::executor::{RbatisRef};
         let driver_type = #rbatis_ident.get_rbatis().driver_type()?;
         use rbatis::rbatis_codegen;
         #gen_func
         let (mut sql,rb_args) = #func_name_ident(&rbs::Value::Map(rb_arg_map),'?');
         #call_method
       }
    }
    .into();
}
