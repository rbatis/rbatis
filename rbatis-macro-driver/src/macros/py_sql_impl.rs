use proc_macro2::{Ident, Literal, Span};
use quote::quote;
use quote::ToTokens;
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::{AttributeArgs, FnArg, ItemFn, Lit, LitByteStr, LitStr, NestedMeta, Pat};

use crate::proc_macro::TokenStream;
use crate::util::{find_fn_body, find_return_type, get_fn_args, is_fetch, is_rbatis_ref};

///py_sql macro
///support args for RB:&Rbatis,page:&PageRequest
///support return for Page<*>
pub(crate) fn impl_macro_py_sql(target_fn: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let return_ty = find_return_type(target_fn);
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
                NestedMeta::Meta(m) => {}
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
    let func_name_ident = target_fn.sig.ident.to_token_stream();
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
    let sql_args_gen = filter_args_context_id(&rbatis_name, &get_fn_args(target_fn));
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
        #[rbatis::rb_py(#sql_ident)]
        pub fn #func_name_ident(arg: &rbs::Value, _tag: char) {}
    };
    let gen_target_macro_arg = quote! {
        #sql_ident
    };
    let gen_func: proc_macro2::TokenStream = rbatis_codegen::rb_py(gen_target_macro_arg.into(), gen_target_method.into()).into();
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
         let (mut sql,rb_args) = #func_name_ident(&rbs::Value::Map(rb_arg_map), '?');
         #call_method
       }
    }
        .into();
}

pub(crate) fn filter_args_context_id(
    rbatis_name: &str,
    fn_arg_name_vec: &Vec<Box<Pat>>,
) -> proc_macro2::TokenStream {
    let mut sql_args_gen = quote! {};
    for item in fn_arg_name_vec {
        // match **item{
        //     Pat::Box(_) => {println!("box");}
        //     Pat::Ident(_) => {println!("Ident");}
        //     Pat::Lit(_) => {println!("Lit");}
        //     Pat::Macro(_) => {println!("Macro");}
        //     Pat::Or(_) => {println!("Or");}
        //     Pat::Path(_) => {println!("Path");}
        //     Pat::Range(_) => {println!("Range");}
        //     Pat::Reference(_) => {println!("Reference");}
        //     Pat::Rest(_) => {println!("Rest");}
        //     Pat::Slice(_) => {println!("Slice");}
        //     Pat::Struct(_) => {println!("Struct");}
        //     Pat::Tuple(_) => {println!("Tuple");}
        //     Pat::TupleStruct(_) => {println!("TupleStruct");}
        //     Pat::Type(_) => {}
        //     Pat::Verbatim(_) => {}
        //     Pat::Wild(_) => {}
        //     _ => {}
        // }
        let item_name = item
            .to_token_stream()
            .to_string()
            .trim()
            .trim_start_matches("mut ")
            .to_string();
        if item_name.eq(rbatis_name) {
            continue;
        }
        let mut item = item.to_token_stream();
        if item.to_string().starts_with("mut ") {
            item = proc_macro2::Ident::new(
                item.to_string().trim_start_matches("mut "),
                Span::call_site(),
            )
                .to_token_stream();
        }
        sql_args_gen = quote! {
             #sql_args_gen
             rb_arg_map.insert(#item_name.to_string().into(),rbs::to_value(#item).unwrap_or_default());
        };
    }
    sql_args_gen
}
