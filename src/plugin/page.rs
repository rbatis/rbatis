//TODO add page plugin
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use async_std::process::Output;
use std::future::Future;
use futures_core::future::BoxFuture;
use serde_json::Value;
use rbatis_core::db::DriverType;
use crate::sql::PageLimit;

///Page interface, support get_pages() and offset()
pub trait IPageRequest {
    fn get_size(&self) -> u64;
    fn get_current(&self) -> u64;
    fn get_total(&self) -> u64;
    fn is_serch_count(&self) -> bool;

    fn set_total(&mut self, arg: u64);
    fn set_size(&mut self, arg: u64);
    fn set_current(&mut self, arg: u64);
    fn set_serch_count(&mut self, arg: bool);

    ///sum pages
    fn get_pages(&self) -> u64 {
        if self.get_size() == 0 {
            return 0;
        }
        let mut pages = self.get_total() / self.get_size();
        if self.get_total() % self.get_size() != 0 {
            pages = pages + 1;
        }
        return pages;
    }
    ///sum offset
    fn offset(&self) -> u64 {
        if self.get_current() > 0 {
            (self.get_current() - 1) * self.get_size()
        } else {
            0
        }
    }
}


///Page interface, support get_pages() and offset()
pub trait IPage<T>: IPageRequest {
    fn get_records(&self) -> &Vec<T>;
    fn get_records_mut(&mut self) -> &mut Vec<T>;
    fn set_records(&mut self, arg: Vec<T>);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Page<T> {
    ///data
    pub records: Vec<T>,
    ///total num
    pub total: u64,
    ///default 10
    pub size: u64,
    ///current index
    pub current: u64,

    pub serch_count: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PageRequest {
    ///total num
    pub total: u64,
    ///default 10
    pub size: u64,
    ///current index
    pub current: u64,
    pub serch_count: bool,
}

impl PageRequest{
    pub fn new(current: u64, size: u64) -> Self {
        return PageRequest::new_total(current, size, 0);
    }
    pub fn new_total(current: u64, size: u64, total: u64) -> Self {
        if current < 1 {
            return Self {
                total,
                size,
                current: 1 as u64,
                serch_count: true,
            };
        }
        return Self {
            total,
            size,
            current,
            serch_count: true,
        };
    }
}

impl Default for PageRequest{
    fn default() -> Self {
        return PageRequest {
            total: 0,
            size: 10,
            current: 1,
            serch_count: true,
        };
    }
}

impl IPageRequest for PageRequest {
    fn get_size(&self) -> u64 {
        self.size
    }
    fn get_current(&self) -> u64 {
        self.current
    }

    fn get_total(&self) -> u64 {
        self.total
    }

    fn is_serch_count(&self) -> bool {
        self.serch_count
    }

    fn set_total(&mut self, total: u64) {
        self.total = total;
    }

    fn set_size(&mut self, arg: u64) {
        self.size = arg;
    }

    fn set_current(&mut self, arg: u64) {
        self.current = arg;
    }

    fn set_serch_count(&mut self, arg: bool) {
        self.serch_count = arg;
    }
}


impl<T> Page<T> {
    pub fn new(current: u64, size: u64) -> Self {
        return Page::new_total(current, size, 0);
    }
    pub fn new_total(current: u64, size: u64, total: u64) -> Self {
        if current < 1 {
            return Self {
                total,
                size,
                current: 1 as u64,
                records: vec![],
                serch_count: true,
            };
        }
        return Self {
            total,
            size,
            current,
            records: vec![],
            serch_count: true,
        };
    }
}

impl<T> Default for Page<T> {
    fn default() -> Self {
        return Page {
            records: vec![],
            total: 0,
            size: 10,
            current: 1,
            serch_count: true,
        };
    }
}

impl<T> IPageRequest for Page<T> {
    fn get_size(&self) -> u64 {
        self.size
    }
    fn get_current(&self) -> u64 {
        self.current
    }

    fn get_total(&self) -> u64 {
        self.total
    }

    fn is_serch_count(&self) -> bool {
        self.serch_count
    }

    fn set_total(&mut self, total: u64) {
        self.total = total;
    }

    fn set_size(&mut self, arg: u64) {
        self.size = arg;
    }

    fn set_current(&mut self, arg: u64) {
        self.current = arg;
    }

    fn set_serch_count(&mut self, arg: bool) {
        self.serch_count = arg;
    }
}

impl<T> IPage<T> for Page<T> {
    fn get_records(&self) -> &Vec<T> {
        self.records.as_ref()
    }

    fn get_records_mut(&mut self) -> &mut Vec<T> {
        self.records.as_mut()
    }

    fn set_records(&mut self, arg: Vec<T>) {
        self.records = arg;
    }
}

use async_trait::async_trait;

///default page plugin
pub trait PagePlugin: Send + Sync {
    /// return 2 sql for select ,  (count_sql,select_sql)
    fn create_page_sql(&self, driver_type: &DriverType, tx_id: &str, sql: &str, args: &Vec<serde_json::Value>, page: &dyn IPageRequest) -> Result<(String, String), rbatis_core::Error>;
}

#[derive(Copy, Clone, Debug)]
pub struct RbatisPagePlugin {}


impl PagePlugin for RbatisPagePlugin {
    fn create_page_sql<>(&self, driver_type: &DriverType, tx_id: &str, sql: &str, args: &Vec<Value>, page: &dyn IPageRequest) -> Result<(String, String), rbatis_core::Error> {
        let mut sql = sql.to_owned();
        sql = sql.replace("select ", "SELECT ");
        sql = sql.replace("from ", "FROM ");
        sql = sql.trim().to_string();
        let limit_sql = driver_type.page_limit_sql(page.offset(), page.get_size())?;
        sql = sql + limit_sql.as_str();
        if !sql.starts_with("SELECT ") && !sql.contains("FROM ") {
            return Err(rbatis_core::Error::from("[rbatis] xml_fetch_page() sql must contains 'select ' And 'from '"));
        }
        let mut count_sql = sql.clone();
        if page.is_serch_count() {
            //make count sql
            let sql_vec: Vec<&str> = count_sql.split("FROM ").collect();
            count_sql = "SELECT count(1) FROM ".to_string() + sql_vec[1];
        }
        return Ok((count_sql, sql));
    }
}


mod test {
    use crate::plugin::page::{Page, IPage, IPageRequest};

    #[test]
    pub fn test_page() {
        let page: Page<i32> = Page::new(2, 10);
        let s = serde_json::to_string(&page).unwrap();
        println!("{}", s.clone());
        let r: Page<i32> = serde_json::from_str(s.as_str()).unwrap();
        println!("{:?}", r);

        println!("offset:{}", page.offset());
        println!("get_pages:{}", page.get_pages());
        assert_eq!(page.offset(), 10);
    }
}