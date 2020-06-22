use std::path::Display;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Local, NaiveDateTime, NaiveDate};

use chrono::serde::ts_nanoseconds;

///数据库表模型
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,

    pub create_time: Option<DateTime<chrono::Utc>>,
    pub version: Option<i32>,
    pub delete_flag: Option<i32>,
}

impl Activity {
    pub fn new() -> Self {
        return Self {
            id: None,
            name: None,
            pc_link: None,
            h5_link: None,
            pc_banner_img: None,
            h5_banner_img: None,
            sort: None,
            status: None,
            remark: None,
            create_time: None,
            version: None,
            delete_flag: None,
        }
    }
}