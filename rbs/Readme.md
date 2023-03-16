# rbs

* rbs is rbatis's impl serde serialize trait crates.
* The rbs serialization framework is used to serialize parameters and deserialize sql result sets, and provides the value structure as py_ Sql and html_ The intermediate object representation of the expression in sql.
* It conforms to the json specification and can be converted directly to a json string using to_string() for Value

## use example
```rust
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
    let json = v.to_string();
    println!("json:{}",json);
}
```