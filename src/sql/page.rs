use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

/// default 10
pub const DEFAULT_PAGE_SIZE: u64 = 10;

///Page interface, support get_pages() and offset()
pub trait IPageRequest: Send+Sync {
    fn page_size(&self) -> u64;
    fn page_no(&self) -> u64;
    fn total(&self) -> u64;
    ///Control whether to execute count statements to count the total number
    fn search_count(&self) -> bool;

    ///sum pages
    fn pages(&self) -> u64 {
        if self.page_size() == 0 {
            return 0;
        }
        let mut pages = self.total() / self.page_size();
        if self.total() % self.page_size() != 0 {
            pages = pages + 1;
        }
        return pages;
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

    ///Control whether to execute count statements to count the total number
    fn set_search_count(&mut self, arg: bool);
}

///Page interface, support get_pages() and offset()
pub trait IPage<T>: IPageRequest {
    fn get_records(&self) -> &Vec<T>;
    fn get_records_mut(&mut self) -> &mut Vec<T>;
    fn set_records(self, arg: Vec<T>) -> Self;
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Page<T: Send+Sync> {
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
    /// Control whether to execute count statements to count the total number
    pub search_count: bool,
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
    pub search_count: bool,
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
            search_count: true,
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
    pub fn set_search_count(mut self, arg: bool) -> Self {
        self.search_count = arg;
        self
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
    fn page_size(&self) -> u64 {
        self.page_size
    }

    fn page_no(&self) -> u64 {
        self.page_no
    }

    fn total(&self) -> u64 {
        self.total
    }
    /// Control whether to execute count statements to count the total number
    fn search_count(&self) -> bool {
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
    /// Control whether to execute count statements to count the total number
    fn set_search_count(&mut self, arg: bool) {
        self.search_count = arg;
    }
}

impl<T: Send+Sync> Page<T> {
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
                pages: {
                    let mut pages = total / page_size;
                    if total % page_size != 0 {
                        pages += 1;
                    }
                    pages
                },
                page_size: page_size,
                page_no: 1 as u64,
                records: vec![],
                search_count: true,
            };
        }
        return Self {
            total,
            pages: {
                let mut pages = total / page_size;
                if total % page_size != 0 {
                    pages += 1;
                }
                pages
            },
            page_size,
            page_no,
            records: vec![],
            search_count: true,
        };
    }

    /// create Vec<Page> from data
    pub fn make_pages(mut data: Vec<T>, page_size: u64) -> Vec<Page<T>> {
        let total = data.len();
        let mut result = vec![];
        let page = Page::<T>::new_total(1, page_size, total as u64);
        for idx in 0..page.pages {
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
        let page = Page::<T>::new_total(1, page_size, total as u64);
        for idx in 0..page.pages {
            let current_page = Page::<T>::new_total(idx + 1, page_size, total as u64);
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
    pub fn set_search_count(mut self, arg: bool) -> Self {
        self.search_count = arg;
        self
    }
}

impl<T: Send+Sync> Default for Page<T> {
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

impl<T: Send+Sync> IPageRequest for Page<T> {
    fn page_size(&self) -> u64 {
        self.page_size
    }

    fn page_no(&self) -> u64 {
        self.page_no
    }

    fn total(&self) -> u64 {
        self.total
    }
    /// Control whether to execute count statements to count the total number
    fn search_count(&self) -> bool {
        self.search_count
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

    /// Control whether to execute count statements to count the total number
    fn set_search_count(&mut self, arg: bool) {
        self.search_count = arg;
    }
}

impl<T: Send+Sync> IPage<T> for Page<T> {
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

impl<T: Display + Debug + Send+Sync> Display for Page<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Page")
            .field("records", &self.records)
            .field("total", &self.total)
            .field("pages", &self.pages)
            .field("page_no", &self.page_no)
            .field("page_size", &self.page_size)
            .field("search_count", &self.search_count)
            .finish()
    }
}

impl<V: Send+Sync> Page<V> {
    pub fn from<T: Send+Sync>(arg: Page<T>) -> Self
        where
            V: From<T>,
    {
        let mut p = Page::<V>::new(arg.page_no, arg.page_size);
        p.pages = arg.pages;
        p.page_no = arg.page_no;
        p.page_size = arg.page_size;
        p.total = arg.total;
        p.search_count = arg.search_count;
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

#[cfg(test)]
mod test {
    use crate::sql::page::Page;


    #[test]
    fn test_page_zero() {
        let mut page = Page::<i32>::new_total(1, 10, 0);
        page.records = vec![];
        println!("{:?}", page);
        assert_eq!(page.pages, 0);
    }

    #[test]
    fn test_page() {
        let mut page = Page::<i32>::new_total(1, 10, 1);
        page.records = vec![];
        println!("{:?}", page);
        assert_eq!(page.pages, 1);
    }

    #[test]
    fn test_page_into_range() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let ranges = Page::<i32>::make_ranges(v.len() as u64, 3);
        let mut new_v = vec![];
        for (offset, limit) in ranges {
            for i in offset..limit {
                new_v.push(i + 1);
            }
        }
        assert_eq!(v, new_v);
    }

    #[test]
    fn test_page_into_pages() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let pages = Page::make_pages(v.clone(), 3);
        let mut new_v = vec![];
        for x in pages {
            for i in x.records {
                new_v.push(i);
            }
        }
        assert_eq!(v, new_v);
    }

    #[test]
    fn test_page_into_pages_more_than() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let pages = Page::make_pages(v.clone(), 18);
        let mut new_v = vec![];
        for x in pages {
            for i in x.records {
                new_v.push(i);
            }
        }
        assert_eq!(v, new_v);
    }

    #[test]
    fn test_page_into_pages_zero() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let pages = Page::make_pages(v.clone(), 0);
        let mut new_v = vec![];
        for x in pages {
            for i in x.records {
                new_v.push(i);
            }
        }
        assert_eq!(v, new_v);
    }

    #[test]
    fn test_page_into_pages_one() {
        let v = vec![1];
        let pages = Page::make_pages(v.clone(), 1);
        let mut new_v = vec![];
        for x in pages {
            for i in x.records {
                new_v.push(i);
            }
        }
        assert_eq!(v, new_v);
    }
}
