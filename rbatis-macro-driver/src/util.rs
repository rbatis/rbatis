use quote::quote;
use quote::ToTokens;
use syn::{FnArg, ItemFn, Pat, ReturnType};

//find and check method return type
pub(crate) fn find_return_type(target_fn: &ItemFn) -> proc_macro2::TokenStream {
    let mut return_ty = target_fn.sig.output.to_token_stream();
    if let ReturnType::Type(_, b) = &target_fn.sig.output {
        return_ty = b.to_token_stream();
    }
    let s = format!("{}", return_ty);
    if !s.contains(":: Result") && !s.starts_with("Result") {
        return_ty = quote! {
             rbatis::Result <#return_ty>
        };
    }
    return_ty
}

#[allow(clippy::vec_box)]
pub(crate) fn get_fn_args(target_fn: &ItemFn) -> Vec<Box<Pat>> {
    let mut fn_arg_name_vec = vec![];
    for arg in &target_fn.sig.inputs {
        if let FnArg::Typed(t) = arg {
            fn_arg_name_vec.push(t.pat.clone());
            //println!("arg_name {}", arg_name);
        }
    }
    fn_arg_name_vec
}

//find and check method return type
pub(crate) fn find_fn_body(target_fn: &ItemFn) -> proc_macro2::TokenStream {
    //del todos
    let mut target_fn = target_fn.clone();
    let mut new_stmts = vec![];
    for x in &target_fn.block.stmts {
        let token = x
            .to_token_stream()
            .to_string()
            .replace("\n", "")
            .replace(" ", "");
        if token.eq("todo!()") || token.eq("unimplemented!()") || token.contains("impled!()") {
            //nothing to do
        } else {
            new_stmts.push(x.to_owned());
        }
    }
    target_fn.block.stmts = new_stmts;
    target_fn.block.to_token_stream()
}

pub(crate) fn is_query(return_source: &str) -> bool {
    !return_source.contains("ExecResult")
}

pub(crate) fn is_rb_ref(ty_stream: &str) -> bool {
    // RBatis, RBatisConnExecutor, RBatisTxExecutor, RBatisTxExecutorGuard, ...
    ty_stream.contains("Executor") || ty_stream.contains("RBatis")
}

/// Check if the return type is Page<T> or Result<Page<T>>
/// Returns Some(inner_type) if it is, None otherwise
pub(crate) fn is_page_return_type(return_str: &str) -> Option<String> {
    let s = return_str.replace(" ", "");

    // Handle Page<T>, rbatis::plugin::Page<T>, Result<Page<T>> and rbatis::Result<Page<T>>.
    // Every "Result<...>" form also contains "Page<", so testing "Page<" is enough.
    if s.contains("Page<") {
        if let Some(start) = s.find("Page<") {
            return extract_type_after(&s, start + 5);
        }
    }

    None
}

/// Extract type content between angle brackets
/// e.g. "Activity>" -> "Activity", "Activity," -> "Activity"
fn extract_type_after(s: &str, start: usize) -> Option<String> {
    let mut depth = 0;
    let mut end = start;

    for (i, c) in s[start..].chars().enumerate() {
        match c {
            '<' => depth += 1,
            '>' => {
                if depth == 0 {
                    end = start + i;
                    break;
                }
                depth -= 1;
            }
            ',' if depth == 0 => {
                end = start + i;
                break;
            }
            _ => {}
        }
    }

    if end > start {
        // nested generics like Vec<Activity> keep their angle brackets
        Some(s[start..end].trim().to_string())
    } else {
        None
    }
}

/// Check if a type string is IPageRequest, PageRequest, or a reference to them
pub(crate) fn is_page_request(ty_stream: &str) -> bool {
    let s = ty_stream.replace(" ", "");
    s.contains("IPageRequest") || s.contains("PageRequest")
}
