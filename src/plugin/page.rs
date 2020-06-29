//TODO add page plugin
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

///分页 Page 抽象接口
pub trait IPage<T> {
    ///每页显示条数
    fn get_size(&self) -> i64;
    ///当前页
    fn get_current(&self) -> i64;
    ///总数
    fn get_total(&self) -> i64;
    ///查询数据列表
    fn get_records(&self) -> &Vec<T>;
    fn get_records_mut(&mut self) -> &mut Vec<T>;


    ///总数
    fn set_total(&mut self, arg: i64);
    ///每页显示条数
    fn set_size(&mut self, arg: i64);
    ///当前页
    fn set_current(&mut self, arg: i64);
    ///查询数据列表
    fn set_records(&mut self, arg: Vec<T>);

    ///当前分页总页数
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
    ///计算当前分页偏移量
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
    ///查询数据列表
    pub records: Vec<T>,
    ///总数
    pub total: i64,
    ///每页显示条数，默认 10
    pub size: i64,
    ///当前页
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