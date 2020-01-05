
use serde::{Serialize, Deserialize};
use std::path::Display;

/**
* 数据库表模型
*/
#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link:Option<String>,
    pub h5_link:Option<String>,
    pub remark:Option<String>,
    pub create_time:Option<String>,
    pub version: Option<i32>,
    pub delete_flag: Option<i32>,
}