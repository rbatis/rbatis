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


generated code for example:
```rust
py->sql: select * from biz_activity where delete_flag = 0 and name=$1 
py->args: ["asdf"]
sql: select * from table where id = 1 id != $1
        id in $2
        id in $3
        id in $4
        id not in $5a = $6name like $7
        or
        name not like $8
        create_time between $9 and $10
        group by$11$12$13order by$14$15
args: [1,[1,2,3],[1,2,3],[1,2,3],[1,2,3],1,"asdf","asdf","2020-23-23","2020-23-23",1,2,3,"id","name"]
```
