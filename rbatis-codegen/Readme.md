rbatis html query lang codegen

from html logic just like:
```html
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
<mapper>
    <select id="select_by_condition">
        `select * from biz_activity`
        <where>
            <if test="name != ''">
                ` and name like #{name}`
            </if>
            <if test="dt >= '2009-12-12 00:00:00'">
                ` and create_time < #{dt}`
            </if>
            <choose>
                <when test="true">
                    ` and id != '-1'`
                </when>
                <otherwise>and id != -2</otherwise>
            </choose>
            ` and `
            <trim prefixOverrides=" and">
                ` and name != '' `
            </trim>
        </where>
    </select>
</mapper>
```

source code for example:
```rust
use rbatis::executor::Executor;
use rbatis::rbdc::datetime::DateTime;
use rbatis::plugin::page::{Page, PageRequest};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}
#[html_sql("example/example.html")]
async fn select_by_condition(rb: &dyn Executor, page_req: &PageRequest, name: &str, dt: &DateTime) -> Vec<BizActivity> {
    impled!()
}
```

```log
2022-08-17 17:16:23.624803 INFO rbatis::plugin::log - [rb] [402390551883812864] query  ==> select * from biz_activity where name like ? and create_time < ? and id != '-1' and  name != ''
                                                      [rb]                      Args   ==> ["test",DateTime("2022-08-17 17:16:23")]
```


# How it works

### 1. Whenever user define `html_sql` method(Of course, `py_sql` The implementation is also based on the `py_sql`  syntax tree  escaped to `html_sql`)

```rust
#[html_sql(r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
  <select id="select_by_condition">
        `select * from biz_activity where `
        <if test="name != null">
                ` name like #{name}`
        </if>
  </select>"#)]
async fn select_by_condition(
    rb: &dyn Executor,
    name: &str,
    a: bool,
) -> rbatis::Result<Vec<BizActivity>> {
    impled!()
}
```

### 2. RBatis expr

* RBatis expr is ```#{name}```,```#{age + 1}```,```${age + 1}``` and  code test:``` <if test="dt >= '2009-12-12 00:00:00'"></if> ```
* RBatis expr will be Convert to original rust code,if RBatis expression = ```#{age + 1}```,the code = ``` rb_arg_map["age"].op_add(1) ```
* RBatis expr directly use strings to compare and process date types,just like ``` <if test="dt >= '2009-12-12 00:00:00'"></if> ```,``` #{dt >= '2009-12-12 00:00:00'}```

### 3. The function body is generated through the process macro via rbatis-codegen

```rust
// pub trait Executor{ //this is rbatis's Executor
// fn exec(&mut self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>>;
//  fn query(&mut self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>>;
// }
pub async fn select_by_condition(
    rb: &dyn Executor,
    name: &str,
    a: bool,
) -> rbatis::Result<Vec<BizActivity>> {
    let mut rb_arg_map = rbs::value::map::ValueMap::new();
    rb_arg_map.insert(
        "name".to_string().into(),
        rbs::to_value(name).unwrap_or_default(),
    );
    rb_arg_map.insert("a".to_string().into(), rbs::to_value(a).unwrap_or_default());
    use rbatis::executor::RBatisRef;
    let driver_type = rb.driver_type()?;
    use rbatis::rbatis_codegen;
    pub fn impl_html_sql(arg: &rbs::Value, _tag: char) -> (String, Vec<rbs::Value>) {
        use rbatis_codegen::ops::*;
        let mut sql = String::with_capacity(55usize);
        let mut args = Vec::with_capacity(20);
        sql.push_str("select * from biz_activity where ");
        if { (&arg["name"]).op_ne(&rbs::Value::Null) }
            .to_owned()
            .into()
        {
            args.push(rbs::to_value({ &arg["name"] }).unwrap_or_default());
            sql.push_str(" name like ?");
        }
        return (sql, args);
    }
    let (mut sql, rb_args) = impl_html_sql(&rbs::Value::Map(rb_arg_map), '?');
    use rbatis::executor::Executor;
    let r = rb.query(&sql, rb_args).await?;
    rbatis::decode::decode(r)
}
```
