//TODO add page plugin


use serde::de::DeserializeOwned;
use serde::ser::Serialize;


pub trait IPage<T> where T: DeserializeOwned + Serialize {
    fn get_size(&self) -> i64;
    fn get_current(&self) -> i64;
    fn get_total(&self) -> i64;
    fn get_records(&self) -> &Vec<T>;
    fn get_records_mut(&mut self) -> &mut Vec<T>;

    fn set_total(&mut self, arg: i64);
    fn set_size(&mut self, arg: i64);
    fn set_current(&mut self, arg: i64);
    fn set_records(&mut self, arg: Vec<T>);
}


pub struct Page<T>
    where T: DeserializeOwned + Serialize {
    records: Vec<T>,
    total: i64,
    size: i64,
    current: i64,
}

impl <T>Page<T>
    where T: DeserializeOwned + Serialize{
    pub fn new()->Self{
        Self{
            records: vec![],
            total: 0,
            size: 0,
            current: 0
        }
    }
}

impl<T> IPage<T> for Page<T> where T: DeserializeOwned + Serialize {
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