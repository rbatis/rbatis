use std::{error, fmt};
use std::error::Error;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RbatisError {
    E(String)
}

impl From<&str> for RbatisError{
    fn from(arg: &str) -> Self {
        return RbatisError::E(arg.to_string());
    }
}

impl From<std::string::String> for RbatisError{
    fn from(arg: String) -> Self {
        return RbatisError::E(arg);
    }
}

impl Display for RbatisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            RbatisError::E(ref err) => {
                write!(f, "Rbatis Error: {}", err)
            }
        };
    }
}

impl Error for RbatisError {
    fn description(&self) -> &str {
        return match self {
            RbatisError::E(data) => {
                data.as_str()
            }
        };
    }
}


#[test]
pub fn test_error() {
    let e = e().err().unwrap();
    println!("{}", e);
}

fn e() -> Result<String, RbatisError> {
    return Err(RbatisError::from("e"));
}