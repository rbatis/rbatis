
use serde::{Serialize, Deserialize};
use std::path::Display;

/**
* 数据库表模型
*/
#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<i32>,
}