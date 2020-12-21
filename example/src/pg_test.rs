use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Po{
   pub x:i32,
   pub y:i32
}

#[crud_enable]
#[derive(Clone,Debug)]
pub struct P{
    pub id:i32,
    pub po: Vec<u8>
}

#[async_std::test]
pub async fn test_select_point() {
    let rb=Rbatis::new();
    rb.link("postgres://postgres:123456@localhost:5432/postgres").await.unwrap();

    let v:P=rb.fetch_by_id("",&1).await.unwrap();
    println!("{:?}",v);
}