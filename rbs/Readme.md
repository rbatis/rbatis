# rbs

* rbs is rbatis's impl serde serialize trait crates.
* The rbs serialization framework is used to serialize parameters and deserialize sql result sets, and provides the value structure as py_ Sql and html_ The intermediate object representation of the expression in sql.


# How it works
* 1 first define html_sql
```rust
#[html_sql(r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
  <select id="select_by_condition">
        `select * from biz_activity where `
        <if test="name != null">
                ` and name like #{name}`
        </if>
  </select>"#)]
async fn select_by_condition(
    rb: &mut dyn Executor,
    name: &str,
    a: bool,
) -> rbatis::Result<Vec<BizActivity>> {
    impled!()
}
```
* 2 The function body is generated through the process macro via rbatis-codegen
```rust 
 async fn select_by_condition(
    rb: &mut dyn Executor,
    name: &str,
    a: bool,
) -> rbatis::Result<Vec<BizActivity>> {
    let mut map = rbs::ValueMap::new();
    let mut args = vec![];
    map.insert("name",rbs::Value::String(name.to_string()));
    let mut sql = "select * from biz_activity where ".to_string();
    if map["name"]!=rbs::Value::Null{
        sql.push_str(" and name like #{name}");
        sql=sql.replace("#{name}", "?");
        args.push(map["name"].clone());
    }
    todo!("impl exec sql and return Result")
}
```


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