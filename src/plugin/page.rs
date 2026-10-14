//! Pagination support: [`Page`], [`PageRequest`], [`IPage`] and [`IPageRequest`].
//!
//! # Automatic pagination (recommended)
//!
//! Declare a `py_sql` / `html_sql` function whose return type is [`Page<T>`](Page):
//! rbatis then runs the page query, adds the `limit`/`offset` for the current
//! database and (when the request asks for it) runs the matching `count(1)` query.
//!
//! ```rust,ignore
//! use rbatis::{impled, Page, PageRequest};
//!
//! #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
//! pub struct Activity {
//!     pub id: Option<String>,
//!     pub name: Option<String>,
//! }
//!
//! // the SQL lives in example/example.html, see <select id="select_by_page">
//! #[rbatis::html_sql("example/example.html")]
//! impl Activity {
//!     pub async fn select_by_page(
//!         rb: &dyn rbatis::Executor,
//!         page_req: &PageRequest,
//!         name: &str,
//!     ) -> rbatis::Result<Page<Activity>> {
//!         impled!()
//!     }
//! }
//!
//! // usage
//! let page: Page<Activity> = Activity::select_by_page(&rb, &PageRequest::new(1, 10), "a").await?;
//! println!("total={} rows={:?}", page.total, page.records);
//! ```
//!
//! The SQL of a `Page<T>` function must start with `select ` and contain ` from `;
//! [`PageIntercept`](crate::plugin::intercept_page::PageIntercept) rewrites it:
//!
//! | Request | SQL that is sent |
//! |---------|------------------|
//! | `do_count() == true` | `select count(1) as count from ...` (without `limit`/`order by`) |
//! | `do_count() == false` | the original `select` plus `limit`/`offset` |
//!
//! The limit syntax is driver specific (`limit ?,?` for mysql/sqlite,
//! `limit ? offset ?` for postgres, `offset ? rows fetch next ? rows only` for mssql).
//! If your SQL already contains ` limit `, rbatis leaves it untouched, which is what
//! the `htmlsql_select_page!`/`pysql_select_page!` macros rely on: there the query
//! itself receives `do_count`, `page_no` (= offset) and `page_size`.
//!
//! # Manual pagination
//!
//! [`Page`] and [`PageRequest`] are plain `serde` structs, so you can also fill them
//! yourself:
//!
//! ```rust
//! use rbatis::{IPageRequest, PageRequest};
//!
//! // page 2, 20 rows per page, 100 rows in total
//! let req = PageRequest::new(2, 20).set_total(100);
//! assert_eq!(req.offset(), 20);            // `offset` is 0 based: (page_no - 1) * page_size
//! assert_eq!(req.offset_limit(), 40);
//! assert_eq!(req.pages(), 5);              // builder style setters return `Self`
//! ```

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

/// default 10
pub const DEFAULT_PAGE_SIZE: u64 = 10;

/// Common behaviour of a page request. Implemented by [`PageRequest`] and [`Page`].
pub trait IPageRequest: Send + Sync {
    /// rows per page
    fn page_size(&self) -> u64;
    /// current page index, starting at 1
    fn page_no(&self) -> u64;
    /// total number of rows (only meaningful when `do_count` was used)
    fn total(&self) -> u64;

    ///Control whether to execute count statements to count the total number
    fn do_count(&self) -> bool;

    /// number of pages needed to hold `total` rows
    fn pages(&self) -> u64 {
        if self.page_size() == 0 {
            return 0;
        }
        let mut pages = self.total() / self.page_size();
        if !self.total().is_multiple_of(self.page_size()) {
            pages += 1;
        }
        pages
    }

    /// `(page_no - 1) * page_size`, the value passed to `limit`
    fn offset(&self) -> u64 {
        if self.page_no() > 0 {
            (self.page_no() - 1) * self.page_size()
        } else {
            0
        }
    }

    /// `offset() + page_size()`, capped at `total()`
    fn offset_limit(&self) -> u64 {
        let v = self.offset() + self.page_size();
        if v > self.total() {
            return self.total();
        }
        v
    }

    /// set the total number of rows
    fn set_total(&mut self, arg: u64);
    /// set the page size
    fn set_page_size(&mut self, arg: u64);
    /// set the current page index
    fn set_page_no(&mut self, arg: u64);

    ///Control execute select count(1) from table
    fn set_do_count(&mut self, arg: bool);
}

/// Common behaviour of a page result. Implemented by [`Page`].
pub trait IPage<T>: IPageRequest {
    /// replace the records of this page
    fn set_records(self, arg: Vec<T>) -> Self;
    /// the records of this page
    fn records(&self) -> &Vec<T>;
    /// mutable access to the records of this page
    fn records_mut(&mut self) -> &mut Vec<T>;
    /// take the records out of this page
    fn records_take(&mut self) -> Vec<T>;
}

/// The request part of a page: which page to read and how many rows it holds.
///
/// ```rust
/// use rbatis::{IPageRequest, PageRequest};
///
/// // 10 rows per page (the default), page 3, counting the total by default
/// let req = PageRequest::new(3, 10);
/// assert_eq!(req.offset(), 20);
/// assert!(req.do_count());
/// ```
///
/// Setters take and return `self`, so they can be chained:
/// `PageRequest::new(1, 20).set_do_count(false)`.
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
    /// create a request for `page_no` with `page_size` rows, see also [`PageRequest::new_total`]
    pub fn new(page_no: u64, page_size: u64) -> Self {
        PageRequest::new_total(page_no, page_size, DEFAULT_PAGE_SIZE)
    }

    /// create a request and set the total number of rows at the same time
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

    /// set the total number of rows
    pub fn set_total(mut self, total: u64) -> Self {
        self.total = total;
        self
    }

    /// set the page size
    pub fn set_page_size(mut self, arg: u64) -> Self {
        self.page_size = arg;
        self
    }

    /// set the current page index
    pub fn set_page_no(mut self, arg: u64) -> Self {
        self.page_no = arg;
        self
    }

    /// Control whether to execute count statements to count the total number
    ///
    /// When set to `false`, no `count(1)` query is sent and [`Page::total`] stays
    /// at its previous value (which saves one query per page).
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

/// One page of rows plus the metadata needed to render a pager.
///
/// `Page<T>` is what a `#[py_sql]` / `#[html_sql]` function returns when it should be
/// paginated, see [the module documentation](self) for an example.
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
    /// create a page from its metadata. `page_size == 0` falls back to
    /// [`DEFAULT_PAGE_SIZE`] and `page_no < 1` falls back to 1.
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

    /// create an empty page with capacity for `page_size` records
    pub fn new_total(page_no: u64, page_size: u64, total: u64) -> Self {
        Self::new(
            page_no,
            page_size,
            total,
            Vec::with_capacity(page_size as usize),
        )
    }

    /// set the total number of rows
    pub fn set_total(mut self, total: u64) -> Self {
        self.total = total;
        self
    }

    /// set the page size
    pub fn set_page_size(mut self, arg: u64) -> Self {
        self.page_size = arg;
        self
    }

    /// set the current page index
    pub fn set_page_no(mut self, arg: u64) -> Self {
        self.page_no = arg;
        self
    }
    /// Control whether to execute count statements to count the total number
    pub fn set_do_count(mut self, arg: bool) -> Self {
        self.do_count = arg;
        self
    }

    /// set the records of this page
    pub fn set_records(mut self, arg: Vec<T>) -> Self {
        self.records = arg;
        self
    }

    /// split an in-memory `data` vector into pages of `page_size` records
    pub fn make_pages(mut data: Vec<T>, page_size: u64) -> Vec<Page<T>> {
        let total = data.len() as u64;
        let mut result = vec![];
        let pages = PageRequest::new(1, page_size).set_total(total).pages();
        for idx in 0..pages {
            let mut current_page = Page::<T>::new(
                idx + 1,
                page_size,
                total,
                Vec::with_capacity(page_size as usize),
            );
            for _ in current_page.offset()..current_page.offset_limit() {
                current_page.records.push(data.remove(0));
            }
            result.push(current_page);
        }
        result
    }

    /// build the `(offset, limit)` ranges of `total` rows with `page_size` rows
    /// per page, used by `insert_batch` to slice its input
    pub fn make_ranges(total: u64, page_size: u64) -> Vec<(u64, u64)> {
        let mut result = vec![];
        let pages = PageRequest::new(1, page_size).set_total(total).pages();
        for idx in 0..pages {
            let current_page = PageRequest::new(idx + 1, page_size).set_total(total);
            result.push((current_page.offset(), current_page.offset_limit()));
        }
        result
    }

    /// build one [`PageRequest`] per page for `total` rows with `page_size` rows per page
    pub fn make_page_requests(total: u64, page_size: u64) -> Vec<PageRequest> {
        let mut result = vec![];
        let pages = PageRequest::new(1, page_size).set_total(total).pages();
        for idx in 0..pages {
            let current_page = PageRequest::new(idx + 1, page_size).set_total(total);
            result.push(current_page);
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
    /// convert a `Page<T>` into a `Page<V>` when `V: From<T>`
    pub fn from<T: Send + Sync>(arg: Page<T>) -> Self
    where
        V: From<T>,
    {
        let mut page = Page::<V>::new(
            arg.page_no,
            arg.page_size,
            arg.total,
            Vec::with_capacity(arg.records.len()),
        );
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
