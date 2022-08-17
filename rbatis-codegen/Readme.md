rbatis html query lang codegen

from html logic just like:
```html
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://github.com/rbatis/rbatis_codegen/raw/main/mybatis-3-mapper.dtd">
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
#[html_sql("example/example.html")]
async fn select_by_condition(
    rb: &mut dyn Executor,
    page_req: &PageRequest,
    name: &str,
    dt: &FastDateTime,
) -> Vec<BizActivity> {
    impled!()
}
```

```log
2022-08-17 17:16:23.624803 INFO rbatis::plugin::log - [rbatis] [402390551883812864] Fetch  ==> select * from biz_activity where name like ? and create_time < ? and id != '-1' and  name != ''
                                                      [rbatis]                      Args   ==> ["test",DateTime("2022-08-17 17:16:23")]
```
