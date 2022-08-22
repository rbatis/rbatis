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
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::page::{Page, PageRequest};
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
    pub create_time: Option<FastDateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}
#[html_sql("example/example.html")]
async fn select_by_condition(rb: &mut dyn Executor, page_req: &PageRequest, name: &str, dt: &FastDateTime) -> Vec<BizActivity> {
    impled!()
}
```

```log
2022-08-17 17:16:23.624803 INFO rbatis::plugin::log - [rbatis] [402390551883812864] Fetch  ==> select * from biz_activity where name like ? and create_time < ? and id != '-1' and  name != ''
                                                      [rbatis]                      Args   ==> ["test",DateTime("2022-08-17 17:16:23")]
```
