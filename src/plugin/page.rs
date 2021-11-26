use std::fmt::{Debug, Display};
use std::future::Future;

use futures_core::future::BoxFuture;
use rbatis_core::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use bson2::Bson;

use crate::core::db::DriverType;
use crate::sql::PageLimit;

/// default 10
pub const DEFAULT_PAGE_SIZE: u64 = 10;

///default page plugin
pub trait PagePlugin: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    /// return 2 sql for select ,  (count_sql,select_sql)
    fn make_page_sql(
        &self,
        driver_type: &DriverType,
        sql: &str,
        args: &Vec<bson2::Bson>,
        page: &dyn IPageRequest,
    ) -> Result<(String, String), crate::core::Error>;
}

///Page interface, support get_pages() and offset()
pub trait IPageRequest: Send + Sync {
    fn get_page_size(&self) -> u64;
    fn get_page_no(&self) -> u64;
    fn get_total(&self) -> u64;
    fn is_search_count(&self) -> bool;

    fn set_total(&mut self, arg: u64);
    fn set_page_size(&mut self, arg: u64);
    fn set_page_no(&mut self, arg: u64);
    fn set_search_count(&mut self, arg: bool);

    ///sum pages
    fn get_pages(&self) -> u64 {
        if self.get_page_size() == 0 {
            return 0;
        }
        let mut pages = self.get_total() / self.get_page_size();
        if self.get_total() % self.get_page_size() != 0 {
            pages = pages + 1;
        }
        return pages;
    }
    ///sum offset
    fn offset(&self) -> u64 {
        if self.get_page_no() > 0 {
            (self.get_page_no() - 1) * self.get_page_size()
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

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Page<T> {
    /// data
    pub records: Vec<T>,
    /// total num
    pub total: u64,
    /// pages
    pub pages: u64,
    /// current page index
    pub page_no: u64,
    /// default 10
    pub page_size: u64,
    /// is search_count
    pub search_count: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct PageRequest {
    /// total num
    pub total: u64,
    /// current page index
    pub page_no: u64,
    /// page page_size default 10
    pub page_size: u64,
    pub search_count: bool,
}

impl PageRequest {
    pub fn new(page_no: u64, page_size: u64) -> Self {
        return PageRequest::new_total(page_no, page_size, DEFAULT_PAGE_SIZE);
    }

    pub fn new_option(page_no: &Option<u64>, page_size: &Option<u64>) -> Self {
        return PageRequest::new(page_no.unwrap_or(1), page_size.unwrap_or(DEFAULT_PAGE_SIZE));
    }

    pub fn new_total(page_no: u64, page_size: u64, total: u64) -> Self {
        return PageRequest::new_plugin(String::new(), page_no, page_size, total);
    }

    pub fn new_plugin(plugin: String, page_no: u64, page_size: u64, total: u64) -> Self {
        let mut page_no = page_no;
        if page_no < 1 {
            page_no = 1;
        }
        return Self {
            total,
            page_size,
            page_no: page_no,
            search_count: true,
        };
    }
}

impl Default for PageRequest {
    fn default() -> Self {
        return PageRequest {
            total: 0,
            page_size: DEFAULT_PAGE_SIZE,
            page_no: 1,
            search_count: true,
        };
    }
}

impl IPageRequest for PageRequest {
    fn get_page_size(&self) -> u64 {
        self.page_size
    }

    fn get_page_no(&self) -> u64 {
        self.page_no
    }

    fn get_total(&self) -> u64 {
        self.total
    }

    fn is_search_count(&self) -> bool {
        self.search_count
    }

    fn set_total(&mut self, total: u64) {
        self.total = total;
    }

    fn set_page_size(&mut self, arg: u64) {
        self.page_size = arg;
    }

    fn set_page_no(&mut self, arg: u64) {
        self.page_no = arg;
    }

    fn set_search_count(&mut self, arg: bool) {
        self.search_count = arg;
    }
}

impl ToString for PageRequest {
    fn to_string(&self) -> String {
        let result = serde_json::to_string(self);
        if result.is_err() {
            return String::new();
        } else {
            return result.unwrap();
        }
    }
}

impl<T> Page<T> {
    pub fn new(current: u64, page_size: u64) -> Self {
        return Page::new_total(current, page_size, 0);
    }

    pub fn new_option(current: &Option<u64>, page_size: &Option<u64>) -> Self {
        return Page::new(current.unwrap_or(1), page_size.unwrap_or(DEFAULT_PAGE_SIZE));
    }

    pub fn new_total(page_no: u64, page_size: u64, total: u64) -> Self {
        if page_no < 1 {
            return Self {
                total,
                pages: 0,
                page_size: page_size,
                page_no: 1 as u64,
                records: vec![],
                search_count: true,
            };
        }
        return Self {
            total,
            pages: 0,
            page_size: page_size,
            page_no,
            records: vec![],
            search_count: true,
        };
    }
}

impl<T> Default for Page<T> {
    fn default() -> Self {
        return Page {
            records: vec![],
            total: 0,
            pages: 0,
            page_size: DEFAULT_PAGE_SIZE,
            page_no: 1,
            search_count: true,
        };
    }
}

impl<T> IPageRequest for Page<T>
    where
        T: Send + Sync,
{
    fn get_page_size(&self) -> u64 {
        self.page_size
    }
    fn get_page_no(&self) -> u64 {
        self.page_no
    }

    fn get_total(&self) -> u64 {
        self.total
    }

    fn is_search_count(&self) -> bool {
        self.search_count
    }

    fn set_total(&mut self, total: u64) {
        self.total = total;
    }

    fn set_page_size(&mut self, arg: u64) {
        self.page_size = arg;
    }

    fn set_page_no(&mut self, arg: u64) {
        self.page_no = arg;
    }

    fn set_search_count(&mut self, arg: bool) {
        self.search_count = arg;
    }
}

impl<T> IPage<T> for Page<T>
    where
        T: Send + Sync,
{
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

impl<T> ToString for Page<T>
    where
        T: Send + Sync + Serialize,
{
    fn to_string(&self) -> String {
        let result = serde_json::to_string(self);
        if result.is_err() {
            return String::new();
        } else {
            return result.unwrap();
        }
    }
}

///use Replace page plugin
#[derive(Copy, Clone, Debug)]
pub struct RbatisReplacePagePlugin {}

impl RbatisReplacePagePlugin {
    pub fn make_count_sql(&self, sql: &str) -> String {
        // let sql = sql
        //     .replace(" from ", " from ")
        //     .replace(" order by ", " order by ")
        //     .replace(" limit ", " limit ");
        let mut from_index = sql.find(crate::sql::TEMPLATE.from.left_right_space);
        if from_index.is_some() {
            from_index = Option::Some(
                from_index.unwrap() + crate::sql::TEMPLATE.from.left_right_space.len(),
            );
        }
        let mut where_sql = sql[from_index.unwrap_or(0)..sql.len()].to_string();
        //remove order by
        if where_sql.contains(crate::sql::TEMPLATE.order_by.left_right_space) {
            where_sql = where_sql[0..where_sql
                .rfind(crate::sql::TEMPLATE.order_by.left_right_space)
                .unwrap_or(where_sql.len())]
                .to_string();
        }
        if where_sql.contains(crate::sql::TEMPLATE.limit.left_right_space) {
            where_sql = where_sql[0..where_sql
                .rfind(crate::sql::TEMPLATE.limit.left_right_space)
                .unwrap_or(where_sql.len())]
                .to_string();
        }
        format!(
            "{} count(1) {} {} ",
            crate::sql::TEMPLATE.select.value,
            crate::sql::TEMPLATE.from.value,
            where_sql
        )
    }
}

impl PagePlugin for RbatisReplacePagePlugin {
    fn make_page_sql(
        &self,
        driver_type: &DriverType,
        sql: &str,
        args: &Vec<Bson>,
        page: &dyn IPageRequest,
    ) -> Result<(String, String), crate::core::Error> {
        //default sql
        let mut sql = sql.trim().to_owned();
        if !sql.starts_with(crate::sql::TEMPLATE.select.right_space)
            && !sql.contains(crate::sql::TEMPLATE.from.left_right_space)
        {
            return Err(crate::core::Error::from(
                "[rbatis] make_page_sql() sql must contains 'select ' And ' from '",
            ));
        }
        //count sql
        let mut count_sql = sql.clone();
        if page.is_search_count() {
            //make count sql
            count_sql = self.make_count_sql(&count_sql);
        }
        //limit sql
        let limit_sql = driver_type.page_limit_sql(page.offset(), page.get_page_size())?;
        match driver_type {
            DriverType::Mssql => {
                sql = format!(
                    "{} RB_DATA.*, 0 {} RB_DATA_ORDER {} ({})RB_DATA {} RB_DATA_ORDER {}",
                    crate::sql::TEMPLATE.select.value,
                    crate::sql::TEMPLATE.r#as.value,
                    crate::sql::TEMPLATE.from.value,
                    sql,
                    crate::sql::TEMPLATE.order_by.value,
                    limit_sql
                );
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
        format!(
            "{} count(1) {} ({}) a",
            crate::sql::TEMPLATE.select.value,
            crate::sql::TEMPLATE.from.value,
            sql
        )
    }
}

impl PagePlugin for RbatisPackPagePlugin {
    fn make_page_sql(
        &self,
        driver_type: &DriverType,
        sql: &str,
        args: &Vec<Bson>,
        page: &dyn IPageRequest,
    ) -> Result<(String, String), crate::core::Error> {
        //default sql
        let mut sql = sql.trim().to_owned();
        if !sql.starts_with(crate::sql::TEMPLATE.select.right_space)
            && !sql.contains(crate::sql::TEMPLATE.from.left_right_space)
        {
            return Err(crate::core::Error::from(
                "[rbatis] make_page_sql() sql must contains 'select ' And ' from '",
            ));
        }
        //count sql
        let mut count_sql = sql.clone();
        if page.is_search_count() {
            //make count sql
            count_sql = self.make_count_sql(&count_sql);
        }
        //limit sql
        let limit_sql = driver_type.page_limit_sql(page.offset(), page.get_page_size())?;
        match driver_type {
            DriverType::Mssql => {
                sql = format!(
                    "{} RB_DATA.*, 0 {} RB_DATA_ORDER {} ({})RB_DATA {} RB_DATA_ORDER {}",
                    crate::sql::TEMPLATE.select.value,
                    crate::sql::TEMPLATE.r#as.value,
                    crate::sql::TEMPLATE.from.value,
                    sql,
                    crate::sql::TEMPLATE.order_by.value,
                    limit_sql
                );
            }
            _ => {
                sql = sql + limit_sql.as_str();
            }
        }
        return Ok((count_sql, sql));
    }
}

///mix page plugin
#[derive(Debug)]
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
    fn make_page_sql(
        &self,
        driver_type: &DriverType,
        sql: &str,
        args: &Vec<Bson>,
        page: &dyn IPageRequest,
    ) -> Result<(String, String), Error> {
        if sql.contains(crate::sql::TEMPLATE.group_by.value) {
            return self
                .pack
                .make_page_sql(driver_type, sql, args, page);
        } else {
            return self
                .replace
                .make_page_sql(driver_type, sql, args, page);
        }
    }
}
