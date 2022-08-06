# rbs

* rbs is rbatis impl message pack serde crates.

##  Unlike the Message Pack changes
* change struct -> Map
* change Integer ->  i32,i64,u32,u64




## use example
```rust
use std::collections::HashMap;
fn main(){
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct A {
        pub name: String,
    }
    let a = A {
        name: "sss".to_string(),
    };
    let v = rbs::to_value_ref(&a).unwrap();
    println!("{:?}", v);

    let mut m = HashMap::new();
    m.insert(1, 2);
    
    let v = rbs::to_value_ref(&m).unwrap();
    println!("{:?}", v);

    let v = rbs::to_value(a).unwrap();
    println!("v: {}",v);
    let s: A = rbs::from_value(v).unwrap();
    println!("s:{:?}", s);
}
```