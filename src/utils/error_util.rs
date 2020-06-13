use std::collections::HashMap;

use rbatis_core::Error;

use crate::ast::node::node_type::NodeType;

/// Option to Result
fn to_result<'a,T,F>(arg: &Option<&'a T>, fail_method: F) -> Result<&'a T, Error>
    where F : Fn() -> String
{
    if arg.is_none() {
        return Err(Error::from(fail_method()));
    }
    return Ok(arg.unwrap());
}

pub trait ToResult<T>  {
    fn to_result<F>(&self, fail_method: F) -> Result<&T, Error>
        where F : Fn() -> String;
}

impl <T>ToResult<T> for Option<&T> {
    fn to_result<F>(&self, fail_method: F) -> Result<&T, Error> where F: Fn() -> String {
        to_result(self, fail_method)
    }
}

#[test]
fn test_to_result(){
    let i=1;
    let v= Option::Some(&i);
    let r= v.to_result(||"".to_string());
}