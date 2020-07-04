use rbatis_core::db::DriverType;

///the stmt replace str convert
pub trait StmtConvert {
    fn stmt_convert(&self, index: usize) -> String;
}

impl StmtConvert for DriverType {
    fn stmt_convert(&self, index: usize) -> String {
        match &self {
            DriverType::Postgres => {
                format!(" ${} ", index + 1)
            }
            DriverType::Mysql => {
                " ? ".to_string()
            }
            DriverType::Sqlite => {
                " ? ".to_string()
            }
            DriverType::None => {
                panic!("un support none for driver type!")
            }
        }
    }
}

