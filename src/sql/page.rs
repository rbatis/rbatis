#![allow(deprecated)]
use crate::{IPage, IPageRequest, DEFAULT_PAGE_SIZE};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[deprecated(note = "please use `rbatis::plugin::page::Page`")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Page<T: Send + Sync> {
    /// data
    pub records: Vec<T>,
    /// total num
    pub total: u64,
    /// current page index
    pub page_no: u64,
    /// default 10
    pub page_size: u64,
    /// Control whether to execute count statements to count the total number
    pub do_count: bool,
}

#[deprecated(note = "please use `rbatis::plugin::page::PageRequest`")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct PageRequest {
    /// total num
    pub total: u64,
    /// current page index default 1,range=1...MAX
    pub page_no: u64,
    /// page page_size default 10
    pub page_size: u64,
    /// Control whether to execute count statements to count the total number
    pub do_count: bool,
}

impl PageRequest {
    pub fn new(page_no: u64, page_size: u64) -> Self {
        return PageRequest::new_total(page_no, page_size, DEFAULT_PAGE_SIZE);
    }

    pub fn new_option(page_no: Option<u64>, page_size: Option<u64>) -> Self {
        return PageRequest::new(page_no.unwrap_or(1), page_size.unwrap_or(DEFAULT_PAGE_SIZE));
    }

    pub fn new_total(page_no: u64, page_size: u64, total: u64) -> Self {
        let mut page_no = page_no;
        if page_no < 1 {
            page_no = 1;
        }
        Self {
            total,
            page_size,
            page_no,
            do_count: true,
        }
    }

    pub fn set_total(mut self, total: u64) -> Self {
        self.total = total;
        self
    }

    pub fn set_page_size(mut self, arg: u64) -> Self {
        self.page_size = arg;
        self
    }

    pub fn set_page_no(mut self, arg: u64) -> Self {
        self.page_no = arg;
        self
    }

    /// Control whether to execute count statements to count the total number
    pub fn set_do_count(mut self, arg: bool) -> Self {
        self.do_count = arg;
        self
    }
}

impl Default for PageRequest {
    fn default() -> Self {
        return PageRequest {
            total: 0,
            page_size: DEFAULT_PAGE_SIZE,
            page_no: 1,
            do_count: true,
        };
    }
}

impl IPageRequest for PageRequest {
    fn page_size(&self) -> u64 {
        self.page_size
    }

    fn page_no(&self) -> u64 {
        self.page_no
    }

    fn total(&self) -> u64 {
        self.total
    }

    fn do_count(&self) -> bool {
        self.do_count
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

    fn set_do_count(&mut self, arg: bool) {
        self.do_count = arg;
    }
}

impl<T: Send + Sync> Page<T> {
    pub fn new(current: u64, page_size: u64) -> Self {
        return Page::new_total(current, page_size, 0);
    }

    pub fn new_option(current: &Option<u64>, page_size: &Option<u64>) -> Self {
        return Page::new(current.unwrap_or(1), page_size.unwrap_or(DEFAULT_PAGE_SIZE));
    }

    pub fn new_total(page_no: u64, mut page_size: u64, total: u64) -> Self {
        if page_size == 0 {
            page_size = DEFAULT_PAGE_SIZE;
        }
        if page_no < 1 {
            return Self {
                total,
                page_size: page_size,
                page_no: 1 as u64,
                records: vec![],
                do_count: true,
            };
        }
        return Self {
            total,
            page_size,
            page_no,
            records: vec![],
            do_count: true,
        };
    }

    /// create Vec<Page> from data
    pub fn make_pages(mut data: Vec<T>, page_size: u64) -> Vec<Page<T>> {
        let total = data.len() as u64;
        let mut result = vec![];
        let pages = PageRequest::new(1, page_size).set_total(total).pages();
        for idx in 0..pages {
            let mut current_page = Page::<T>::new_total(idx + 1, page_size, total as u64);
            for _ in current_page.offset()..current_page.offset_limit() {
                current_page.records.push(data.remove(0));
            }
            result.push(current_page);
        }
        result
    }

    /// create (Vec<offset,limit>) from total,page_size
    pub fn make_ranges(total: u64, page_size: u64) -> Vec<(u64, u64)> {
        let mut result = vec![];
        let pages = PageRequest::new(1, page_size).set_total(total).pages();
        for idx in 0..pages {
            let current_page = Page::<T>::new_total(idx + 1, page_size, total);
            result.push((current_page.offset(), current_page.offset_limit()));
        }
        result
    }

    pub fn set_total(mut self, total: u64) -> Self {
        self.total = total;
        self
    }

    pub fn set_page_size(mut self, arg: u64) -> Self {
        self.page_size = arg;
        self
    }

    pub fn set_page_no(mut self, arg: u64) -> Self {
        self.page_no = arg;
        self
    }
    /// Control whether to execute count statements to count the total number
    pub fn set_do_count(mut self, arg: bool) -> Self {
        self.do_count = arg;
        self
    }
}

impl<T: Send + Sync> Default for Page<T> {
    fn default() -> Self {
        return Page {
            records: vec![],
            total: 0,
            page_size: DEFAULT_PAGE_SIZE,
            page_no: 1,
            do_count: true,
        };
    }
}

impl<T: Send + Sync> IPageRequest for Page<T> {
    fn page_size(&self) -> u64 {
        self.page_size
    }

    fn page_no(&self) -> u64 {
        self.page_no
    }

    fn total(&self) -> u64 {
        self.total
    }

    fn do_count(&self) -> bool {
        self.do_count
    }

    fn set_total(&mut self, arg: u64) {
        self.total = arg;
    }

    fn set_page_size(&mut self, arg: u64) {
        self.page_size = arg;
    }

    fn set_page_no(&mut self, arg: u64) {
        self.page_no = arg;
    }

    fn set_do_count(&mut self, arg: bool) {
        self.do_count = arg;
    }
}

impl<T: Send + Sync> IPage<T> for Page<T> {
    fn get_records(&self) -> &Vec<T> {
        self.records.as_ref()
    }

    fn get_records_mut(&mut self) -> &mut Vec<T> {
        self.records.as_mut()
    }

    fn set_records(mut self, arg: Vec<T>) -> Self {
        self.records = arg;
        self
    }
}

impl<T: Display + Debug + Send + Sync> Display for Page<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Page")
            .field("records", &self.records)
            .field("total", &self.total)
            .field("page_no", &self.page_no)
            .field("page_size", &self.page_size)
            .field("do_count", &self.do_count)
            .finish()
    }
}

impl<V: Send + Sync> Page<V> {
    pub fn from<T: Send + Sync>(arg: Page<T>) -> Self
    where
        V: From<T>,
    {
        let mut p = Page::<V>::new(arg.page_no, arg.page_size);
        p.page_no = arg.page_no;
        p.page_size = arg.page_size;
        p.total = arg.total;
        p.do_count = arg.do_count;
        p.records = {
            let mut records = Vec::with_capacity(arg.records.len());
            for x in arg.records {
                records.push(V::from(x));
            }
            records
        };
        p
    }
}
