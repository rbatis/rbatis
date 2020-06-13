use std::collections::HashMap;

use rbatis_core::Error;

use crate::ast::node::node_type::NodeType;

/// Option to Result
fn to_result<'a,T>(arg: &Option<&'a T>, fail_str: String) -> Result<&'a T, Error> {
    if arg.is_none() {
        return Err(Error::from(fail_str));
    }
    return Ok(arg.unwrap());
}

pub trait ToResult<T>  {
    fn to_result(&self, fail_str: String) -> Result<&T, Error>;
}

impl <T>ToResult<T> for Option<&T> {
    fn to_result(&self, fail_str: String) -> Result<&T, Error> {
        to_result(self, fail_str)
    }
}