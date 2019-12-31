
use serde::{Serialize, Deserialize};

/**
* 数据库表模型
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<i32>,
}