use rbatis_codegen::error::Error;
use std::error::Error as StdError;
use std::io::{self, ErrorKind};
use syn;
use proc_macro2;

#[test]
fn test_error_display() {
    let error = Error::from("测试错误");
    assert_eq!(error.to_string(), "测试错误");
}

#[test]
fn test_error_from_string() {
    let error = Error::from("测试错误".to_string());
    assert_eq!(error.to_string(), "测试错误");
}

#[test]
fn test_error_from_str() {
    let error = Error::from("测试错误");
    assert_eq!(error.to_string(), "测试错误");
}

#[test]
fn test_error_from_io_error() {
    let io_error = io::Error::new(ErrorKind::NotFound, "文件未找到");
    let error = Error::from(io_error);
    assert_eq!(error.to_string(), "文件未找到");
}

#[test]
fn test_error_from_dyn_error() {
    let io_error: Box<dyn StdError> = Box::new(io::Error::new(ErrorKind::Other, "其他错误"));
    let error = Error::from(io_error.as_ref());
    assert_eq!(error.to_string(), "其他错误");
}

#[test]
fn test_error_clone() {
    let error = Error::from("原始错误");
    let cloned_error = error.clone();
    assert_eq!(cloned_error.to_string(), "原始错误");
    
    let mut error1 = Error::from("错误1");
    let error2 = Error::from("错误2");
    error1.clone_from(&error2);
    assert_eq!(error1.to_string(), "错误2");
}

#[test]
fn test_error_from_syn_error() {
    let syn_error = syn::Error::new(proc_macro2::Span::call_site(), "语法错误");
    let error = Error::from(syn_error);
    assert_eq!(error.to_string(), "语法错误");
} 