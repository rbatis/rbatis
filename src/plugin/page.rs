//TODO add page plugin
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

///Page interface, support get_pages() and offset()
pub trait IPage<T> {
    fn get_size(&self) -> i64;
    fn get_current(&self) -> i64;
    fn get_total(&self) -> i64;
    fn get_records(&self) -> &Vec<T>;
    fn get_records_mut(&mut self) -> &mut Vec<T>;

    fn set_total(&mut self, arg: i64);
    fn set_size(&mut self, arg: i64);
    fn set_current(&mut self, arg: i64);
    fn set_records(&mut self, arg: Vec<T>);

    ///sum pages
    fn get_pages(&self) -> i64 {
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
    fn offset(&self) -> i64 {
        if self.get_current() > 0 {
            (self.get_current() - 1) * self.get_size()
        } else {
            0
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Page<T> {
    ///data
    pub records: Vec<T>,
    ///total num
    pub total: i64,
    ///default 10
    pub size: i64,
    ///current index
    pub current: i64,
}


impl<T> Page<T> {
    pub fn new(current: i64, size: i64) -> Self {
        return Page::new_total(current, size, 0);
    }
    pub fn new_total(current: i64, size: i64, total: i64) -> Self {
        if current < 1 {
            return Self {
                total,
                size,
                current: 1 as i64,
                records: vec![],
            };
        }
        return Self {
            total,
            size,
            current,
            records: vec![],
        };
    }
}

impl <T>Default for Page<T>{
    fn default() -> Self {
        return Page {
            records: vec![],
            total: 0,
            size: 10,
            current: 1,
        };
    }
}

impl<T> IPage<T> for Page<T> {
    fn get_size(&self) -> i64 {
        self.size
    }

    fn get_current(&self) -> i64 {
        self.current
    }

    fn get_total(&self) -> i64 {
        self.total
    }

    fn get_records(&self) -> &Vec<T> {
        self.records.as_ref()
    }

    fn get_records_mut(&mut self) -> &mut Vec<T> {
        self.records.as_mut()
    }

    fn set_total(&mut self, total: i64) {
        self.total = total;
    }

    fn set_size(&mut self, arg: i64) {
        self.size = arg;
    }

    fn set_current(&mut self, arg: i64) {
        self.current = arg;
    }

    fn set_records(&mut self, arg: Vec<T>) {
        self.records = arg;
    }
}

mod test {
    use crate::plugin::page::{Page, IPage};

    #[test]
    pub fn test_page() {
        let page: Page<i32> = Page::new(2, 10);
        let s = serde_json::to_string(&page).unwrap();
        println!("{}", s.clone());
        let r: Page<i32> = serde_json::from_str(s.as_str()).unwrap();
        println!("{:?}", r);

        println!("offset:{}",page.offset());
        println!("get_pages:{}",page.get_pages());
        assert_eq!(page.offset(),10);
    }
}