### rbatis macro code gen project

#### use way [see](https://github.com/rbatis/rbatis)


#### How it works
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
