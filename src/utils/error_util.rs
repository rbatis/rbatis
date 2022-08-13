use crate::Error;
use std::collections::HashMap;

pub trait ToResult<T> {
    fn to_result<F>(&self, fail_method: F) -> Result<&T, Error>
    where
        F: Fn() -> String;
}

impl<T> ToResult<T> for Option<&T> {
    fn to_result<F>(&self, fail_method: F) -> Result<&T, Error>
    where
        F: Fn() -> String,
    {
        if self.is_none() {
            return Err(Error::from(fail_method()));
        }
        return Ok(self.unwrap());
    }
}

#[test]
fn test_to_result() {
    let i = 1;
    let v = Option::Some(&i);
    let r = v.to_result(|| String::new());
}
