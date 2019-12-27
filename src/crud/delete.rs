use serde_json::Value;
use crate::core::rbatis::Rbatis;
use std::borrow::BorrowMut;

impl Rbatis{
    pub fn delete_by_id(&mut self, mapper_name: String, id: &str, env: &mut Value){
         println!("{}",env.is_string());
         println!("{}",env.as_str().unwrap());
    }
}

#[test]
fn test_delete_by_id() {
    let mut rbatis =Rbatis::new();
    rbatis.delete_by_id("".to_string(), "", serde_json::json!(r#"1234123123"#).borrow_mut());
}
