use std::future::Future;

use futures_core::future::BoxFuture;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde::export::fmt::Debug;
use serde_json::Value;

use rbatis_core::Error;

use crate::core::db::DriverType;
use crate::sql::PageLimit;

///default page plugin
pub trait PagePlugin: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    /// return 2 sql for select ,  (count_sql,select_sql)
    fn make_page_sql(&self, driver_type: &DriverType, context_id: &str, sql: &str, args: &Vec<serde_json::Value>, page: &dyn IPageRequest) -> Result<(String, String), crate::core::Error>;
}


///Page interface, support get_pages() and offset()
pub trait IPageRequest: Send + Sync {
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
    ///pages
    pub pages: u64,
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

impl PageRequest {
    pub fn new(current: u64, size: u64) -> Self {
        return PageRequest::new_total(current, size, 0);
    }

    pub fn new_option(current: &Option<u64>, size: &Option<u64>) -> Self {
        return PageRequest::new(current.unwrap_or(1), size.unwrap_or(10));
    }

    pub fn new_total(current: u64, size: u64, total: u64) -> Self {
        let mut current = current;
        if current < 1 {
            current = 1;
        }
        return Self {
            total,
            size,
            current,
            serch_count: true,
        };
    }
}

impl Default for PageRequest {
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

impl ToString for PageRequest {
    fn to_string(&self) -> String {
        let result = serde_json::to_string(self);
        if result.is_err() {
            return "".to_string();
        } else {
            return result.unwrap();
        }
    }
}


impl<T> Page<T> {
    pub fn new(current: u64, size: u64) -> Self {
        return Page::new_total(current, size, 0);
    }

    pub fn new_option(current: &Option<u64>, size: &Option<u64>) -> Self {
        return Page::new(current.unwrap_or(1), size.unwrap_or(10));
    }

    pub fn new_total(current: u64, size: u64, total: u64) -> Self {
        if current < 1 {
            return Self {
                total,
                pages: 0,
                size,
                current: 1 as u64,
                records: vec![],
                serch_count: true,
            };
        }
        return Self {
            total,
            pages: 0,
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
            pages: 0,
            size: 10,
            current: 1,
            serch_count: true,
        };
    }
}

impl<T> IPageRequest for Page<T>
    where T: Send + Sync {
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

impl<T> IPage<T> for Page<T>
    where T: Send + Sync {
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

impl<T> ToString for Page<T> where T: Send + Sync + Serialize {
    fn to_string(&self) -> String {
        let result = serde_json::to_string(self);
        if result.is_err() {
            return "".to_string();
        } else {
            return result.unwrap();
        }
    }
}

///use Replace page plugin
#[derive(Copy, Clone, Debug)]
pub struct RbatisReplacePagePlugin {}


impl RbatisReplacePagePlugin {
    fn make_count_sql(&self, sql: &str) -> String {
        let mut from_index = sql.find(" FROM ");
        if from_index.is_some() {
            from_index = Option::Some(from_index.unwrap() + " FROM ".len());
        }
        let mut where_sql = sql[from_index.unwrap_or(0)..sql.len()].to_string();
        where_sql = where_sql.replace(" order by ", " ORDER BY ");
        where_sql = where_sql.replace(" limit ", " LIMIT ");
        //remove order by
        if where_sql.contains(" ORDER BY ") {
            where_sql = where_sql[0..where_sql.rfind(" ORDER BY ").unwrap_or(where_sql.len())].to_string();
        }
        if where_sql.contains(" LIMIT ") {
            where_sql = where_sql[0..where_sql.rfind(" LIMIT ").unwrap_or(where_sql.len())].to_string();
        }
        format!("SELECT count(1) FROM {}", where_sql)
    }
}


impl PagePlugin for RbatisReplacePagePlugin {
    fn make_page_sql<>(&self, driver_type: &DriverType, context_id: &str, sql: &str, args: &Vec<Value>, page: &dyn IPageRequest) -> Result<(String, String), crate::core::Error> {
        //default sql
        let mut sql = sql.to_owned();
        sql = sql.replace("select ", "SELECT ");
        sql = sql.replace(" from ", " FROM ");
        sql = sql.trim().to_string();
        if !sql.starts_with("SELECT ") && !sql.contains("FROM ") {
            return Err(crate::core::Error::from("[rbatis] make_page_sql() sql must contains 'SELECT ' And 'FROM '"));
        }
        //count sql
        let mut count_sql = sql.clone();
        if page.is_serch_count() {
            //make count sql
            count_sql = self.make_count_sql(&count_sql);
        }
        //limit sql
        let limit_sql = driver_type.page_limit_sql(page.offset(), page.get_size())?;
        match driver_type {
            DriverType::Mssql => {
                sql = format!("SELECT RB_DATA.*, 0 AS RB_DATA_ORDER FROM ({})RB_DATA ORDER BY RB_DATA_ORDER {}", sql, limit_sql);
            }
            _ => {
                sql = sql + limit_sql.as_str();
            }
        }
        return Ok((count_sql, sql));
    }
}

///use pack sql page plugin
#[derive(Copy, Clone, Debug)]
pub struct RbatisPackPagePlugin {}

impl RbatisPackPagePlugin {
    fn make_count_sql(&self, sql: &str) -> String {
        format!("SELECT count(1) FROM ({}) a", sql)
    }
}


impl PagePlugin for RbatisPackPagePlugin {
    fn make_page_sql<>(&self, driver_type: &DriverType, context_id: &str, sql: &str, args: &Vec<Value>, page: &dyn IPageRequest) -> Result<(String, String), crate::core::Error> {
        //default sql
        let mut sql = sql.to_owned();
        sql = sql.replace("select ", "SELECT ");
        sql = sql.replace(" from ", " FROM ");
        sql = sql.trim().to_string();
        if !sql.starts_with("SELECT ") && !sql.contains("FROM ") {
            return Err(crate::core::Error::from("[rbatis] make_page_sql() sql must contains 'SELECT ' And 'FROM '"));
        }
        //count sql
        let mut count_sql = sql.clone();
        if page.is_serch_count() {
            //make count sql
            count_sql = self.make_count_sql(&count_sql);
        }
        //limit sql
        let limit_sql = driver_type.page_limit_sql(page.offset(), page.get_size())?;
        match driver_type {
            DriverType::Mssql => {
                sql = format!("SELECT RB_DATA.*, 0 AS RB_DATA_ORDER FROM ({})RB_DATA ORDER BY RB_DATA_ORDER {}", sql, limit_sql);
            }
            _ => {
                sql = sql + limit_sql.as_str();
            }
        }
        return Ok((count_sql, sql));
    }
}


///mix page plugin
#[derive(Copy, Clone, Debug)]
pub struct RbatisPagePlugin {
    pub pack: RbatisPackPagePlugin,
    pub replace: RbatisReplacePagePlugin,
}

impl RbatisPagePlugin {
    pub fn new() -> Self {
        return Self::default();
    }
}

impl Default for RbatisPagePlugin {
    fn default() -> Self {
        Self {
            pack: RbatisPackPagePlugin {},
            replace: RbatisReplacePagePlugin {},
        }
    }
}

impl PagePlugin for RbatisPagePlugin {
    fn make_page_sql(&self, driver_type: &DriverType, context_id: &str, sql: &str, args: &Vec<Value>, page: &dyn IPageRequest) -> Result<(String, String), Error> {
        if sql.contains("GROUP BY") || sql.contains("group by") {
            return self.pack.make_page_sql(driver_type, context_id, sql, args, page);
        } else {
            return self.replace.make_page_sql(driver_type, context_id, sql, args, page);
        }
    }
}


#[cfg(test)]
mod test {
    use crate::plugin::page::{IPage, IPageRequest, Page, RbatisReplacePagePlugin};

    #[test]
    pub fn test_page() {
        let mut page: Page<i32> = Page::new(2, 10);
        page.records.push(12);
        let s = serde_json::to_string(&page).unwrap();
        println!("{}", s.clone());
        let r: Page<i32> = serde_json::from_str(s.as_str()).unwrap();
        println!("{:?}", r);

        println!("offset:{}", page.offset());
        println!("get_pages:{}", page.get_pages());
        println!("page_string:{}", page.to_string());
        assert_eq!(page.offset(), 10);
    }

    #[test]
    fn test_make_count() {
        let plugin = RbatisReplacePagePlugin {};
        let sql = plugin.make_count_sql("biz_activity where id = 1 and ORDER BY id DESC and ORDER BY id DESC");
        println!("sql:{}", sql);
    }
}