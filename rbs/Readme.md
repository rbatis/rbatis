# rbs

* rbs is rbatis's impl serde serialize trait crates.
* The rbs serialization framework is used to serialize parameters and deserialize sql result sets, and provides the value structure as py_ Sql and html_ The intermediate object representation of the expression in sql.

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
    let v = rbs::to_value(a).unwrap();
    println!("v: {}",v);
    let s: A = rbs::from_value(v).unwrap();
    println!("s:{:?}", s);
}
```