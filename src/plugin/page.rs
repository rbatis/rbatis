//TODO add page plugin




use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub trait IPage<'de,T> where T:Deserialize<'de>+Serialize {
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

#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct Page<T> {
    records: Vec<T>,
    total: i64,
    size: i64,
    current: i64,
}


impl <T>Page<T>{
    pub fn new()->Self{
        Self{
            records: vec![],
            total: 0,
            size: 0,
            current: 0
        }
    }
}

impl<'de,T> IPage<'de,T> for Page<T> where T:Deserialize<'de>+Serialize {
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


#[test]
pub fn test_page() {
    let p: Page<i32> = Page::new();
    let v = serde_json::to_string(&p).unwrap();
    println!("{}", v.clone());

    let d: Page<i32> = serde_json::from_str(v.as_str()).unwrap();
    println!("{:?}", d);
}