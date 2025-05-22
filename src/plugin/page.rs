use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

/// default 10
pub const DEFAULT_PAGE_SIZE: u64 = 10;

///PageRequest trait
pub trait IPageRequest: Send + Sync {
    fn page_size(&self) -> u64;
    fn page_no(&self) -> u64;
    fn total(&self) -> u64;

    ///Control whether to execute count statements to count the total number
    fn do_count(&self) -> bool;

    ///sum pages
    fn pages(&self) -> u64 {
        if self.page_size() == 0 {
            return 0;
        }
        let mut pages = self.total() / self.page_size();
        if self.total() % self.page_size() != 0 {
            pages = pages + 1;
        }
        pages
    }

    ///sum offset
    fn offset(&self) -> u64 {
        if self.page_no() > 0 {
            (self.page_no() - 1) * self.page_size()
        } else {
            0
        }
    }

    ///sum offset_limit
    fn offset_limit(&self) -> u64 {
        let v = self.offset() + self.page_size();
        if v > self.total() {
            return self.total();
        }
        v
    }

    fn set_total(&mut self, arg: u64);
    fn set_page_size(&mut self, arg: u64);
    fn set_page_no(&mut self, arg: u64);

    ///Control execute select count(1) from table
    fn set_do_count(&mut self, arg: bool);
}

///Page trait
pub trait IPage<T>: IPageRequest {
    fn set_records(self, arg: Vec<T>) -> Self;
    fn records(&self) -> &Vec<T>;
    fn records_mut(&mut self) -> &mut Vec<T>;
    fn records_take(&mut self) -> Vec<T>;
}


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
        PageRequest::new_total(page_no, page_size, DEFAULT_PAGE_SIZE)
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
        PageRequest {
            total: 0,
            page_size: DEFAULT_PAGE_SIZE,
            page_no: 1,
            do_count: true,
        }
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

impl<T: Send + Sync> Page<T> {
    pub fn new(page_no: u64, mut page_size: u64, total: u64, records: Vec<T>) -> Self {
        if page_size == 0 {
            page_size = DEFAULT_PAGE_SIZE;
        }
        if page_no < 1 {
            return Self {
                total,
                page_size,
                page_no: 1u64,
                records,
                do_count: true,
            };
        }
        Self {
            total,
            page_size,
            page_no,
            records,
            do_count: true,
        }
    }

    pub fn new_total(page_no: u64, page_size: u64, total: u64) -> Self {
        Self::new(page_no, page_size, total, Vec::with_capacity(page_size as usize))
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

    pub fn set_records(mut self, arg: Vec<T>) -> Self {
        self.records = arg;
        self
    }

    /// create Vec<Page> from data
    pub fn make_pages(mut data: Vec<T>, page_size: u64) -> Vec<Page<T>> {
        let total = data.len() as u64;
        let mut result = vec![];
        let pages = PageRequest::new(1, page_size).set_total(total).pages();
        for idx in 0..pages {
            let mut current_page = Page::<T>::new(idx + 1, page_size, total, Vec::with_capacity(page_size as usize));
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
            let current_page = PageRequest::new(idx + 1, page_size).set_total(total);
            result.push((current_page.offset(), current_page.offset_limit()));
        }
        result
    }
}

impl<T: Send + Sync> Default for Page<T> {
    fn default() -> Self {
        Page {
            records: vec![],
            total: 0,
            page_size: DEFAULT_PAGE_SIZE,
            page_no: 1,
            do_count: true,
        }
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
    fn records(&self) -> &Vec<T> {
        self.records.as_ref()
    }

    fn records_mut(&mut self) -> &mut Vec<T> {
        self.records.as_mut()
    }

    fn set_records(mut self, arg: Vec<T>) -> Self {
        self.records = arg;
        self
    }

    fn records_take(&mut self) -> Vec<T> {
        let mut records = Vec::with_capacity(self.records.len());
        while let Some(v) = self.records.pop() {
            records.push(v);
        }
        records.reverse();
        records
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
        let mut page = Page::<V>::new(arg.page_no, arg.page_size, arg.total, Vec::with_capacity(arg.records.len()));
        page.page_no = arg.page_no;
        page.page_size = arg.page_size;
        page.total = arg.total;
        page.do_count = arg.do_count;
        for x in arg.records {
            page.records.push(V::from(x));
        }
        page
    }
}