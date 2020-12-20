use serde_json::Value;

use crate::db::DriverType;
use crate::Result;

///the stmt replace str convert
pub trait StmtConvert {
    fn stmt_convert(&self, index: usize) -> String;
}

impl StmtConvert for DriverType {
    fn stmt_convert(&self, index: usize) -> String {
        match &self {
            DriverType::Postgres => {
                format!("${}", index + 1)
            }
            DriverType::Mysql => {
                "?".to_string()
            }
            DriverType::Sqlite => {
                "?".to_string()
            }
            DriverType::Mssql => {
                format!("@p{}", index + 1)
            }
            DriverType::None => {
                panic!("[rbatis] un support none for driver type!")
            }
        }
    }
}

///json convert
pub trait JsonCodec {
    /// to an json value
    fn try_to_json(self) -> Result<Value>;
}

///json convert
pub trait RefJsonCodec {
    /// to an json value
    fn try_to_json(&self) -> Result<Value>;
}

///result convert
pub trait ResultCodec<T> {
    fn into_result(self) -> Result<T>;
}