#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum DriverType {
    None = 0,
    Mysql = 1,
    Postgres = 2,
    Sqlite = 3,
    Mssql = 4,
    Oracle = 5,
}

impl DriverType {
    pub fn is_number_type(&self) -> bool {
        return match self {
            DriverType::Postgres | DriverType::Mssql | DriverType::Oracle => {
                true
            }
            _ => {
                false
            }
        }
    }
}
