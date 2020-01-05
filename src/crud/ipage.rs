use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IPage<T: Serialize + Clone> {
    pub total: i64,
    pub size: i64,
    pub current: i64,
    pub records: Option<Vec<T>>,
}

impl <T> IPage<T>where T:Serialize + Clone {
    pub fn new(current:i64,size:i64)-> Self{
        return IPage::new_total(current, size, 0);
    }
    pub fn new_total(current:i64,size:i64,total:i64)-> Self{
        return Self{
            total: total,
            size,
            current,
            records: None
        }
    }
    pub fn set_records(&mut self,data:Vec<T>){
        self.records=Some(data);
    }
}