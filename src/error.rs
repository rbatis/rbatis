use std::{error, fmt};
use std::error::Error;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RbatisError {
    E(String)
}


impl Display for RbatisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            RbatisError::E(ref err) => {
                write!(f, "IO error: {}", err)
            }
        };
    }
}

impl Error for RbatisError {
    fn description(&self) -> &str {
        return match self {
            RbatisError::E(data) => {
                return data.as_str();
            }
        };
    }
}

