use serde_json::Value;
use crate::core::rbatis::Rbatis;
use std::borrow::BorrowMut;

impl Rbatis{
    pub fn delete(&mut self, mapper_name: String, id: &str, env: &mut Value){
        //TODO delete by id
        //TODO delete by map
        //TODO delete by ids
         println!("{}",env.is_string());
         println!("{}",env.as_str().unwrap());
    }
}

#[test]
fn test_delete() {
    let mut rbatis =Rbatis::new();
    rbatis.delete("".to_string(), "", serde_json::json!(r#"1234123123"#).borrow_mut());
}
