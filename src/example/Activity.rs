use rbatis_macro_derive::RbatisMacro;
use rbatis_macro::RbatisMacro;
use serde::{Serialize, Deserialize};

/**
* 数据库表模型
*/
#[derive(Serialize, Deserialize, Debug, Clone,RbatisMacro)]
pub struct Activity {
    pub id: String,
    pub name: String,
    pub version: Option<i32>,
}