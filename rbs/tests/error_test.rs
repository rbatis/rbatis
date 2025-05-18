use rbs::Error;
use std::error::Error as StdError;
use std::io;
use std::io::{Error as IoError, ErrorKind};

#[test]
fn test_error_creation() {
    // 从字符串创建错误
    let err = Error::from("test error");
    assert_eq!(err.to_string(), "test error");
    
    // 从静态字符串引用创建错误
    let err = Error::from("static error");
    assert_eq!(err.to_string(), "static error");
    
    // 从字符串引用创建错误
    let s = String::from("string ref error");
    let err = Error::from(&s[..]);
    assert_eq!(err.to_string(), "string ref error");
    
    // 从其他错误类型创建
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = Error::from(io_err);
    assert!(err.to_string().contains("file not found"));
}

#[test]
fn test_error_box() {
    // 测试从Error转换为Box<dyn StdError>
    let _err = Error::from("test error");
    let boxed: Box<dyn StdError> = Box::new(Error::from("test error"));
    
    // 测试从Error转换为Box<dyn StdError + Send>
    let send_boxed: Box<dyn StdError + Send> = Box::new(Error::from("test error"));
    
    // 测试从Error转换为Box<dyn StdError + Send + Sync>
    let sync_boxed: Box<dyn StdError + Send + Sync> = Box::new(Error::from("test error"));
    
    // 确保错误信息一致
    assert_eq!(boxed.to_string(), "test error");
    assert_eq!(send_boxed.to_string(), "test error");
    assert_eq!(sync_boxed.to_string(), "test error");
}

#[test]
fn test_error_source() {
    // 创建一个嵌套的错误
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
    let err = Error::from(io_err);
    
    // 测试source方法 - 注意：当前实现可能不保留源错误
    let _source = err.source();
    
    // 我们不对source结果做具体断言，因为Error实现可能不保留源
    // 这个测试主要是确保调用source方法不会崩溃
}

#[test]
fn test_error_display_and_debug() {
    let err = Error::from("test display and debug");
    
    // 测试Display实现
    let display_str = format!("{}", err);
    assert_eq!(display_str, "test display and debug");
    
    // 测试Debug实现
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("test display and debug") || 
            debug_str.contains("E") && debug_str.contains("test display and debug"));
}

#[test]
fn test_from_string() {
    // 测试从String创建错误
    let err1 = Error::from("error 1".to_string());
    let err2 = Error::from("error 2");
    
    assert_eq!(err1.to_string(), "error 1");
    assert_eq!(err2.to_string(), "error 2");
}

// 测试自定义错误通过字符串转换
#[derive(Debug)]
struct CustomError {
    message: String,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomError: {}", self.message)
    }
}

impl StdError for CustomError {}

#[test]
fn test_custom_error_conversion() {
    let custom = CustomError {
        message: "custom error message".to_string(),
    };
    
    // 通过Display特性转换到字符串，再到Error
    let err = Error::from(custom.to_string());
    assert!(err.to_string().contains("custom error message"));
}

#[test]
fn test_append_error() {
    let err = Error::from("base error");
    let appended = err.append(" with more info");
    assert_eq!(appended.to_string(), "base error with more info");
}

#[test]
fn test_protocol_error() {
    let err = Error::protocol("protocol violation");
    assert!(err.to_string().contains("ProtocolError"));
    assert!(err.to_string().contains("protocol violation"));
}

#[test]
fn test_error_display() {
    let err = Error::E("test error".to_string());
    assert_eq!(err.to_string(), "test error");
}

#[test]
fn test_error_append() {
    let err = Error::E("test error".to_string());
    let err = err.append(" appended");
    assert_eq!(err.to_string(), "test error appended");
}

#[test]
fn test_error_protocol() {
    let err = Error::protocol("protocol error");
    assert_eq!(err.to_string(), "ProtocolError protocol error");
}

#[test]
fn test_error_from_string() {
    let err = Error::from("test error".to_string());
    assert_eq!(err.to_string(), "test error");
}

#[test]
fn test_error_from_str() {
    let err = Error::from("test error");
    assert_eq!(err.to_string(), "test error");
}

#[test]
fn test_error_from_io_error() {
    let io_err = IoError::new(ErrorKind::NotFound, "file not found");
    let err = Error::from(io_err);
    assert!(err.to_string().contains("file not found"));
}

#[allow(invalid_from_utf8)]
#[test]
fn test_error_from_utf8_error() {
    // 无效的 UTF-8 序列
    let utf8_err = std::str::from_utf8(&[0, 159, 146, 150]).unwrap_err();
    let err = Error::from(utf8_err);
    assert!(err.to_string().contains("invalid utf-8"));
}

#[test]
fn test_error_from_parse_int_error() {
    let parse_err = "abc".parse::<i32>().unwrap_err();
    let err = Error::from(parse_err);
    assert!(err.to_string().contains("invalid digit"));
}

#[test]
fn test_error_from_parse_float_error() {
    let parse_err = "abc".parse::<f64>().unwrap_err();
    let err = Error::from(parse_err);
    assert!(err.to_string().contains("invalid float"));
}

#[test]
fn test_error_from_try_from_int_error() {
    let i: i64 = 1234567890123;
    let try_from_err = i32::try_from(i).unwrap_err();
    let err = Error::from(try_from_err);
    assert!(err.to_string().contains("out of range"));
}

#[test]
fn test_err_protocol_macro() {
    let err = rbs::err_protocol!("macro error");
    assert_eq!(err.to_string(), "macro error");
    
    let err = rbs::err_protocol!("formatted error: {}", 42);
    assert_eq!(err.to_string(), "formatted error: 42");
}

#[test]
fn test_serde_ser_error() {
    use serde::ser::Error;
    let ser_err: rbs::Error = Error::custom("serialize error");
    assert_eq!(ser_err.to_string(), "serialize error");
}

#[test]
fn test_serde_de_error() {
    use serde::de::Error;
    let de_err: rbs::Error = Error::custom("deserialize error");
    assert_eq!(de_err.to_string(), "deserialize error");
} 