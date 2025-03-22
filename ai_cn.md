# Rbatis框架使用指南

> 本文档基于Rbatis 4.5+ 版本，提供了Rbatis ORM框架的详细使用说明。Rbatis是一个高性能的Rust异步ORM框架，支持多种数据库，提供了编译时动态SQL和类似MyBatis的功能。

## 重要版本说明和最佳实践

Rbatis 4.5+相比之前的版本有显著改进。以下是主要变化和推荐的最佳实践：

1. **使用宏替代特质**：在当前版本中，使用`crud!`和`impl_*`宏替代实现`CRUDTable`特质（这是旧版3.0中使用的方式）。

2. **定义模型和操作的首选模式**：
   ```rust
   // 1. 定义你的数据模型
   #[derive(Clone, Debug, Serialize, Deserialize)]
   pub struct User {
       pub id: Option<String>,
       pub name: Option<String>,
       // 其他字段...
   }
   
   // 2. 生成基本的CRUD操作
   crud!(User {});  // 或 crud!(User {}, "自定义表名");
   
   // 3. 使用impl_*宏定义自定义方法
   impl_select!(User {select_by_name(name: &str) -> Vec => "` where name = #{name}`"});
   impl_select!(User {select_by_id(id: &str) -> Option => "` where id = #{id} limit 1`"});
   impl_update!(User {update_status_by_id(id: &str, status: i32) => "` set status = #{status} where id = #{id}`"});
   impl_delete!(User {delete_by_name(name: &str) => "` where name = #{name}`"});
   ```

3. **使用小写SQL关键字**：SQL关键字始终使用小写，如`select`、`where`、`and`等。

4. **正确使用反引号**：用反引号(`)包裹动态SQL片段以保留空格。

5. **异步优先方法**：所有数据库操作都应使用`.await`等待完成。

请参考下面的示例了解当前推荐的使用方法。

## 1. Rbatis简介

Rbatis是一个Rust语言编写的ORM(对象关系映射)框架，提供了丰富的数据库操作功能。它支持多种数据库类型，包括但不限于MySQL、PostgreSQL、SQLite、MS SQL Server等。

Rbatis的设计灵感来源于Java的MyBatis框架，但针对Rust语言特性进行了优化和调整。作为一个现代ORM框架，它利用Rust的编译时特性，在编译阶段完成SQL解析和代码生成，提供零开销的动态SQL能力。

### 1.1 主要特性

Rbatis提供以下主要特性：

- **零运行时开销的动态SQL**：使用编译时技术（proc-macro、Cow）实现动态SQL，无需运行时解析引擎
- **类JDBC驱动设计**：驱动通过cargo依赖和`Box<dyn Driver>`实现分离
- **多数据库支持**：所有数据库驱动都支持`#{arg}`、`${arg}`、`?`占位符（pg/mssql自动将`?`转换为`$1`和`@P1`）
- **动态SQL语法**：支持py_sql查询语言和html_sql（受MyBatis启发）
- **动态连接池配置**：基于fast_pool实现高性能连接池
- **基于拦截器的日志支持**
- **100%纯Rust实现**：启用`#![forbid(unsafe_code)]`保证安全

### 1.2 支持的数据库驱动

Rbatis支持任何实现了`rbdc`接口的驱动程序。以下是官方支持的驱动：

| 数据库类型 | crates.io包 | 相关链接 |
|------------|-------------|----------|
| MySQL | rbdc-mysql | github.com/rbatis/rbatis/tree/master/rbdc-mysql |
| PostgreSQL | rbdc-pg | github.com/rbatis/rbatis/tree/master/rbdc-pg |
| SQLite | rbdc-sqlite | github.com/rbatis/rbatis/tree/master/rbdc-sqlite |
| MSSQL | rbdc-mssql | github.com/rbatis/rbatis/tree/master/rbdc-mssql |
| MariaDB | rbdc-mysql | github.com/rbatis/rbatis/tree/master/rbdc-mysql |
| TiDB | rbdc-mysql | github.com/rbatis/rbatis/tree/master/rbdc-mysql |
| CockroachDB | rbdc-pg | github.com/rbatis/rbatis/tree/master/rbdc-pg |
| Oracle | rbdc-oracle | github.com/chenpengfan/rbdc-oracle |
| TDengine | rbdc-tdengine | github.com/tdcare/rbdc-tdengine |

## 2. 核心概念

1. **RBatis结构体**：框架的主要入口，负责管理数据库连接池、拦截器等核心组件
2. **Executor**：执行SQL操作的接口，包括RBatisConnExecutor（连接执行器）和RBatisTxExecutor（事务执行器）
3. **CRUD操作**：提供了基本的增删改查操作宏和函数
4. **动态SQL**：支持HTML和Python风格的SQL模板，可根据条件动态构建SQL语句
5. **拦截器**：可以拦截和修改SQL执行过程，如日志记录、分页等

## 3. 安装和依赖配置

在Cargo.toml中添加以下依赖：

```toml
[dependencies]
rbatis = "4.5"
rbs = "4.5"
# 选择一个数据库驱动
rbdc-sqlite = "4.5" # SQLite驱动
# rbdc-mysql = "4.5" # MySQL驱动
# rbdc-pg = "4.5" # PostgreSQL驱动
# rbdc-mssql = "4.5" # MS SQL Server驱动

# 异步运行时
tokio = { version = "1", features = ["full"] }
# 序列化支持
serde = { version = "1", features = ["derive"] }
```

Rbatis是一个异步框架，需要配合tokio等异步运行时使用。它利用serde进行数据序列化和反序列化操作。

### 3.1 配置TLS支持

如果需要TLS支持，可以使用以下配置：

```toml
rbs = { version = "4.5" }
rbdc-sqlite = { version = "4.5", default-features = false, features = ["tls-native-tls"] }
# rbdc-mysql = { version = "4.5", default-features = false, features = ["tls-native-tls"] }
# rbdc-pg = { version = "4.5", default-features = false, features = ["tls-native-tls"] }
# rbdc-mssql = { version = "4.5", default-features = false, features = ["tls-native-tls"] }
rbatis = { version = "4.5" }
```

## 4. 基本使用流程

### 4.1 创建RBatis实例和初始化数据库连接

```rust
use rbatis::RBatis;

#[tokio::main]
async fn main() {
    // 创建RBatis实例
    let rb = RBatis::new();
    
    // 方法1：仅初始化数据库驱动，但不建立连接（使用init方法）
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://database.db").unwrap();
    
    // 方法2：初始化驱动并尝试建立连接（推荐，使用link方法）
    rb.link(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://database.db").await.unwrap();
    
    // 其他数据库示例:
    // MySQL
    // rb.link(rbdc_mysql::driver::MysqlDriver{}, "mysql://root:123456@localhost:3306/test").await.unwrap();
    // PostgreSQL
    // rb.link(rbdc_pg::driver::PgDriver{}, "postgres://postgres:123456@localhost:5432/postgres").await.unwrap();
    // MSSQL/SQL Server
    // rb.link(rbdc_mssql::driver::MssqlDriver{}, "jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=test").await.unwrap();
    
    println!("数据库连接成功！");
}
```

> **init方法与link方法的区别**：
> - `init()`: 仅设置数据库驱动，不会实际连接数据库
> - `link()`: 设置驱动并立即尝试连接数据库，推荐使用此方法确保连接可用

### 4.2 定义数据模型

数据模型是映射到数据库表的Rust结构体：

```rust
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub create_time: Option<DateTime>,
    pub status: Option<i32>,
}

// 注意：在Rbatis 4.5+中，建议使用crud!宏
// 而不是实现CRUDTable特质（这是旧版本中的做法）
// 应该使用以下方式：

// 为User结构体生成CRUD方法
crud!(User {});
// 或指定自定义表名
// crud!(User {}, "users");
```

### 4.3 自定义表名

Rbatis允许通过多种方式自定义表名：

```rust
// 方式1: 通过crud宏参数指定表名
rbatis::crud!(BizActivity {}, "biz_activity"); // 自定义表名为biz_activity

// 方式2: 通过impl_*宏的最后一个参数指定表名
rbatis::impl_select!(BizActivity{select_by_id(id:String) -> Option => "` where id = #{id} limit 1 `"}, "biz_activity");

// 方式3: 通过函数参数动态指定表名
rbatis::impl_select!(BizActivity{select_by_id2(table_name:&str,id:String) -> Option => "` where id = #{id} limit 1 `"});
```

同样地，也可以自定义表列名：

```rust
// 通过函数参数动态指定表列
rbatis::impl_select!(BizActivity{select_by_id(table_name:&str,table_column:&str,id:String) -> Option => "` where id = #{id} limit 1 `"});
```

## 5. CRUD操作

Rbatis提供了多种方式执行CRUD（创建、读取、更新、删除）操作。

> **注意**：Rbatis处理时要求SQL关键字使用小写形式（select、insert、update、delete等），这与某些SQL样式指南可能不同。在使用Rbatis时，始终使用小写的SQL关键字，以确保正确解析和执行。

### 5.1 使用CRUD宏

最简单的方式是使用`crud!`宏：

```rust
use rbatis::crud;

// 为User结构体自动生成CRUD方法
// 如果指定了表名，就使用指定的表名，否则使用结构体名称的蛇形命名法作为表名
crud!(User {}); // 表名为user
// 或者
crud!(User {}, "users"); // 表名为users
```

这将为User结构体生成以下方法：
- `User::insert`：插入单条记录
- `User::insert_batch`：批量插入记录
- `User::update_by_column`：根据指定列更新记录
- `User::update_by_column_batch`：批量更新记录
- `User::delete_by_column`：根据指定列删除记录
- `User::delete_in_column`：删除列值在指定集合中的记录
- `User::select_by_column`：根据指定列查询记录
- `User::select_in_column`：查询列值在指定集合中的记录
- `User::select_all`：查询所有记录
- `User::select_by_map`：根据映射条件查询记录

### 5.2 CRUD操作示例

```rust
#[tokio::main]
async fn main() {
    // 初始化RBatis和数据库连接...
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://test.db").unwrap();
    
    // 创建用户实例
    let user = User {
        id: Some("1".to_string()),
        username: Some("test_user".to_string()),
        password: Some("password".to_string()),
        create_time: Some(DateTime::now()),
        status: Some(1),
    };
    
    // 插入数据
    let result = User::insert(&rb, &user).await.unwrap();
    println!("插入记录数: {}", result.rows_affected);
    
    // 查询数据
    let users: Vec<User> = User::select_by_column(&rb, "id", "1").await.unwrap();
    println!("查询到用户: {:?}", users);
    
    // 更新数据
    let mut user_to_update = users[0].clone();
    user_to_update.username = Some("updated_user".to_string());
    User::update_by_column(&rb, &user_to_update, "id").await.unwrap();
    
    // 删除数据
    User::delete_by_column(&rb, "id", "1").await.unwrap();
}
```

## 6. 动态SQL

Rbatis支持动态SQL，可以根据条件动态构建SQL语句。Rbatis提供了两种风格的动态SQL：HTML风格和Python风格。

### 6.1 HTML风格动态SQL

HTML风格的动态SQL使用类似XML的标签语法：

```rust
use rbatis::executor::Executor;
use rbatis::{html_sql, RBatis};

#[html_sql(
r#"
<select id="select_by_condition">
    select * from user
    <where>
        <if test="name != null">
            ` and name like #{name} `
        </if>
        <if test="age != null">
            ` and age > #{age} `
        </if>
        <choose>
            <when test="role == 'admin'">
                ` and role = 'admin' `
            </when>
            <otherwise>
                ` and role = 'user' `
            </otherwise>
        </choose>
    </where>
    order by id desc
</select>
"#
)]
async fn select_by_condition(
    rb: &dyn Executor,
    name: Option<&str>,
    age: Option<i32>,
    role: &str,
) -> rbatis::Result<Vec<User>> {
    impled!() // 特殊标记，会被rbatis宏处理器替换为实际实现
}
```

#### 6.1.1 空格处理机制

在HTML风格的动态SQL中，**反引号（`）是处理空格的关键**：

- **默认会trim空格**：非反引号包裹的文本节点会自动去除前后空格
- **反引号保留原文**：用反引号(`)包裹的文本会完整保留所有空格和换行
- **必须使用反引号**：动态SQL片段必须用反引号包裹，否则前导空格和换行会被忽略
- **完整包裹**：反引号应包裹整个SQL片段，而不仅仅是开头部分

不正确使用反引号的示例：
```rust
<if test="status != null">
    and status = #{status} <!-- 没有反引号，前导空格会丢失 -->
</if>

<if test="type != null">
    ` and type = #{type} ` <!-- 只有开头有反引号，不完整 -->
</if>
```

正确使用反引号的示例：
```rust
<if test="status != null">
    ` and status = #{status} ` <!-- 完整包裹，所有空格保留 -->
</if>

<if test="items != null and items.len > 0">
    ` and item_id in `
    <foreach collection="items" item="item" open="(" close=")" separator=",">
        #{item}
    </foreach>
</if>
```

#### 6.1.2 与MyBatis的差异

Rbatis的HTML风格与MyBatis有几个关键差异：

1. **无需CDATA**：Rbatis不需要使用CDATA块来转义特殊字符
   ```rust
   <!-- MyBatis需要CDATA -->
   <if test="age > 18">
       <![CDATA[ AND age > 18 ]]>
   </if>

   <!-- Rbatis直接使用 -->
   <if test="age > 18">
       ` and age > 18 `
   </if>
   ```

2. **表达式语法**：Rbatis使用Rust风格的表达式语法
   ```rust
   <!-- MyBatis -->
   <if test="list != null and list.size() > 0">

   <!-- Rbatis -->
   <if test="list != null and list.len > 0">
   ```

3. **特殊标签属性**：Rbatis的foreach等标签属性名称与MyBatis略有不同

HTML风格支持的标签包括：
- `<if>`：条件判断
- `<choose>`、`<when>`、`<otherwise>`：多条件选择
- `<trim>`：去除前缀或后缀
- `<foreach>`：循环处理
- `<where>`：自动处理WHERE子句
- `<set>`：自动处理SET子句

### 6.2 Python风格动态SQL

Python风格的动态SQL使用类似Python的语法：

```rust
use rbatis::{py_sql, RBatis};

#[py_sql(
r#"
select * from user
where
    1 = 1
    if name != None:
        ` and name like #{name} `
    if age != None:
        ` and age > #{age} `
    if role == "admin":
        ` and role = "admin" `
    if role != "admin":
        ` and role = "user" `
"#
)]
async fn select_by_condition_py(
    rb: &dyn Executor,
    name: Option<&str>,
    age: Option<i32>,
    role: &str,
) -> rbatis::Result<Vec<User>> {
    impled!()
}
```

> **注意**：Rbatis要求SQL关键字使用小写形式。在以上示例中，使用了小写的`select`、`where`等关键字，这是推荐的做法。

#### 6.2.1 Python风格空格处理

Python风格动态SQL中的空格处理规则：

- **缩进敏感**：缩进用于识别代码块，必须保持一致
- **行首检测**：通过检测行首字符判断语句类型
- **反引号规则**：与HTML风格相同，用于保留空格
- **代码块约定**：每个控制语句后的代码块必须缩进

特别注意：
```rust
# 错误：缩进不一致
if name != None:
    ` and name = #{name}`
  ` and status = 1`  # 缩进错误，会导致语法错误

# 正确：一致的缩进
if name != None:
    ` and name = #{name} `
    ` and status = 1 `  # 与上一行缩进一致
```

#### 6.2.2 Python风格支持的语法

Python风格提供了以下语法结构：

1. **if 条件语句**：
   ```rust
   if condition:
       ` SQL片段 `
   ```
   注意：Python风格仅支持单一的`if`语句，不支持`elif`或`else`分支。

2. **for 循环**：
   ```rust
   for item in collection:
       ` SQL片段 `
   ```

3. **choose/when/otherwise**：使用特定的语法结构而不是`if/elif/else`
   ```rust
   choose:
       when condition1:
           ` SQL片段1 `
       when condition2:
           ` SQL片段2 `
       otherwise:
           ` 默认SQL片段 `
   ```

4. **trim, where, set**：特殊语法结构
   ```rust
   trim "AND|OR":
       ` and id = 1 `
       ` or id = 2 `
   ```

5. **break 和 continue**：可用于循环控制
   ```rust
   for item in items:
       if item.id == 0:
           continue
       if item.id > 10:
           break
       ` process item #{item.id} `
   ```

6. **bind 变量**：声明局部变量
   ```rust
   bind name = "John"
   ` WHERE name = #{name} `
   ```

#### 6.2.3 Python风格特有功能

Python风格提供了一些特有的便捷功能：

1. **内置函数**：如`len()`、`is_empty()`、`trim()`等
2. **集合操作**：通过`.sql()`和`.csv()`等方法简化IN子句
   ```rust
   if ids != None:
       ` and id in ${ids.sql()} `  #生成 in (1,2,3) 格式
   ```
3. **条件组合**：支持复杂表达式
   ```rust
   if (age > 18 and role == "vip") or level > 5:
       ` and is_adult = 1 `
   ```

### 6.3 HTML风格特有语法

HTML风格支持的标签包括：

1. **`<if>`**：条件判断
   ```xml
   <if test="condition">
       SQL片段
   </if>
   ```

2. **`<choose>/<when>/<otherwise>`**：多条件选择（类似switch语句）
   ```xml
   <choose>
       <when test="condition1">
           SQL片段1
       </when>
       <when test="condition2">
           SQL片段2
       </when>
       <otherwise>
           默认SQL片段
       </otherwise>
   </choose>
   ```

3. **`<trim>`**：去除前缀或后缀
   ```xml
   <trim prefixOverrides="AND|OR" suffixOverrides=",">
       SQL片段
   </trim>
   ```

4. **`<foreach>`**：循环处理
   ```xml
   <foreach collection="items" item="item" index="index" separator=",">
       #{item}
   </foreach>
   ```

5. **`<where>`**：自动处理WHERE子句（会智能去除前导AND/OR）
   ```xml
   <where>
       <if test="id != null">
           and id = #{id}
       </if>
   </where>
   ```

6. **`<set>`**：自动处理SET子句（会智能管理逗号）
   ```xml
   <set>
       <if test="name != null">
           name = #{name},
       </if>
       <if test="age != null">
           age = #{age},
       </if>
   </set>
   ```

7. **`<bind>`**：变量绑定
   ```xml
   <bind name="pattern" value="'%' + name + '%'" />
   ```

不支持传统MyBatis中的`<elseif>`标签，而是使用多个`<when>`来实现类似功能。

### 6.4 表达式引擎功能

Rbatis表达式引擎支持多种操作符和函数：

- **比较运算符**：`==`, `!=`, `>`, `<`, `>=`, `<=`
- **逻辑运算符**：`&&`, `||`, `!`
- **数学运算符**：`+`, `-`, `*`, `/`, `%`
- **集合操作**：`in`, `not in`
- **内置函数**：
  - `len(collection)`: 获取集合长度
  - `is_empty(collection)`: 检查集合是否为空
  - `trim(string)`: 去除字符串前后空格
  - `print(value)`: 打印值（调试用）
  - `to_string(value)`: 转换为字符串

表达式示例：
```rust
<if test="user.age >= 18 && (user.role == 'admin' || user.vip)">
    ` and is_adult = 1 `
</if>

if (page_size * (page_no - 1)) <= total && !items.is_empty():
    ` limit #{page_size} offset #{page_size * (page_no - 1)} `
```

### 6.5 参数绑定机制

Rbatis提供两种参数绑定方式：

1. **命名参数**：使用`#{name}`格式，自动防SQL注入
   ```rust
   ` select * from user where username = #{username} `
   ```

2. **位置参数**：使用问号`?`占位符，按顺序绑定
   ```rust
   ` select * from user where username = ? and age > ? `
   ```

3. **原始插值**：使用`${expr}`格式，直接插入表达式结果（**谨慎使用**）
   ```rust
   ` select * from ${table_name} where id > 0 ` #用于动态表名
   ```

**安全提示**：
- `#{}`绑定会自动转义参数，防止SQL注入，推荐用于绑定值
- `${}`直接插入内容，存在SQL注入风险，仅用于表名、列名等结构部分
- 对于IN语句，使用`.sql()`方法生成安全的IN子句

核心区别：
- **`#{}`绑定**：
  - 将值转换为参数占位符，实际值放入参数数组
  - 自动处理类型转换和NULL值
  - 防止SQL注入

- **`${}`绑定**：
  - 直接将表达式结果转为字符串插入SQL
  - 用于动态表名、列名等结构元素
  - 不处理SQL注入风险

### 6.6 动态SQL实战技巧

#### 6.6.1 复杂条件构建

```rust
#[py_sql(r#"
select * from user
where 1=1
if name != None and name.trim() != '':  # 检查空字符串
    ` and name like #{name} `
if ids != None and !ids.is_empty():     # 使用内置函数
    ` and id in ${ids.sql()} `           # 使用.sql()方法生成in语句
if (age_min != None and age_max != None) and (age_min < age_max):
    ` and age between #{age_min} and #{age_max} `
if age_min != None:
    ` and age >= #{age_min} `
if age_max != None:
    ` and age <= #{age_max} `
"#)]
```

#### 6.6.2 动态排序和分组

```rust
#[py_sql(r#"
select * from user
where status = 1
if order_field != None:
    if order_field == "name":
        ` order by name `
    if order_field == "age":
        ` order by age `
    if order_field != "name" and order_field != "age":
        ` order by id `
    
    if desc == true:
        ` desc `
    if desc != true:
        ` asc `
"#)]
```

#### 6.6.3 动态表名与列名

```rust
#[py_sql(r#"
select ${select_fields} from ${table_name}
where ${where_condition}
"#)]
async fn dynamic_query(
    rb: &dyn Executor,
    select_fields: &str,  // 必须为安全值
    table_name: &str,     // 必须为安全值
    where_condition: &str, // 必须为安全值
) -> rbatis::Result<Vec<Value>> {
    impled!()
}
```

#### 6.6.4 通用模糊查询

```rust
#[html_sql(r#"
<select id="fuzzy_search">
    select * from user
    <where>
        <if test="search_text != null and search_text != ''">
            `(name like #{search_text_like} or 
             email like #{search_text_like} or 
             phone like #{search_text_like})`
        </if>
    </where>
    order by create_time desc
</select>
"#)]
async fn fuzzy_search(
    rb: &dyn Executor,
    search_text: Option<&str>,
    search_text_like: Option<&str>, // 预处理为 %text%
) -> rbatis::Result<Vec<User>> {
    impled!()
}

// 使用示例
let search = "test";
let result = fuzzy_search(&rb, Some(search), Some(&format!("%{}%", search))).await?;
```

### 6.7 动态SQL使用示例

```rust
#[tokio::main]
async fn main() {
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://test.db").unwrap();
    
    // 使用HTML风格的动态SQL
    let users = select_by_condition(&rb, Some("%test%"), Some(18), "admin").await.unwrap();
    println!("查询结果: {:?}", users);
    
    // 使用Python风格的动态SQL
    let users = select_by_condition_py(&rb, Some("%test%"), Some(18), "admin").await.unwrap();
    println!("查询结果: {:?}", users);
}
```

### 6.8 Rbatis表达式引擎详解

Rbatis的表达式引擎是动态SQL的核心，负责在编译时解析和处理表达式，并转换为Rust代码。通过深入了解表达式引擎的工作原理，可以更有效地利用Rbatis的动态SQL功能。

#### 6.8.1 表达式引擎架构

Rbatis表达式引擎由以下几个核心组件构成：

1. **词法分析器**：将表达式字符串分解为标记（tokens）
2. **语法分析器**：构建表达式的抽象语法树（AST）
3. **代码生成器**：将AST转换为Rust代码
4. **运行时支持**：提供类型转换和操作符重载等功能

在编译时，Rbatis处理器（如`html_sql`和`py_sql`宏）会调用表达式引擎解析条件表达式，并生成等效的Rust代码。

#### 6.8.2 表达式类型系统

Rbatis表达式引擎围绕`rbs::Value`类型构建，这是一个能表示多种数据类型的枚举。表达式引擎支持以下数据类型：

1. **标量类型**：
   - `Null`：空值
   - `Bool`：布尔值
   - `I32`/`I64`：有符号整数
   - `U32`/`U64`：无符号整数
   - `F32`/`F64`：浮点数
   - `String`：字符串

2. **复合类型**：
   - `Array`：数组/列表
   - `Map`：键值对映射
   - `Binary`：二进制数据
   - `Ext`：扩展类型

所有表达式最终都会被编译为操作`Value`类型的代码，表达式引擎会根据上下文自动进行类型转换。

#### 6.8.3 类型转换和运算符

Rbatis表达式引擎实现了强大的类型转换系统，允许不同类型间的操作：

```rust
// 源码中的AsProxy特质为各种类型提供转换功能
pub trait AsProxy {
    fn i32(&self) -> i32;
    fn i64(&self) -> i64;
    fn u32(&self) -> u32;
    fn u64(&self) -> u64;
    fn f64(&self) -> f64;
    fn usize(&self) -> usize;
    fn bool(&self) -> bool;
    fn string(&self) -> String;
    fn as_binary(&self) -> Vec<u8>;
}
```

表达式引擎重载了所有标准运算符，使它们能够应用于`Value`类型：

1. **比较运算符**：
   ```rust
   // 在表达式中
   user.age > 18
   
   // 编译为
   (user["age"]).op_gt(&Value::from(18))
   ```

2. **逻辑运算符**：
   ```rust
   // 在表达式中
   is_admin && is_active
   
   // 编译为
   bool::op_from(is_admin) && bool::op_from(is_active)
   ```

3. **数学运算符**：
   ```rust
   // 在表达式中
   price * quantity
   
   // 编译为
   (price).op_mul(&quantity)
   ```

不同类型之间的转换规则：
- 数值类型间自动转换（如i32到f64）
- 字符串与数值类型可互相转换（如"123"到123）
- 空值（null/None）与其他类型的比较遵循特定规则

#### 6.8.4 路径表达式与访问器

Rbatis支持通过点号和索引访问对象的嵌套属性：

```rust
// 点号访问对象属性
user.profile.age > 18

// 数组索引访问
items[0].price > 100

// 多级路径
order.customer.address.city == "Beijing"
```

这些表达式会被转换为对`Value`的索引操作：

```rust
// user.profile.age > 18 转换为
(&arg["user"]["profile"]["age"]).op_gt(&Value::from(18))
```

#### 6.8.5 内置函数与方法

Rbatis表达式引擎提供了许多内置函数和方法：

1. **集合函数**：
   - `len(collection)`：获取集合长度
   - `is_empty(collection)`：检查集合是否为空
   - `contains(collection, item)`：检查集合是否包含某项

2. **字符串函数**：
   - `trim(string)`：去除字符串两端空格
   - `starts_with(string, prefix)`：检查字符串前缀
   - `ends_with(string, suffix)`：检查字符串后缀
   - `to_string(value)`：转换为字符串

3. **SQL生成方法**：
   - `value.sql()`：生成SQL片段，特别适用于IN子句
   - `value.csv()`：生成逗号分隔值列表

```rust
// 表达式中使用函数
if !ids.is_empty() && len(names) > 0:
    ` AND id IN ${ids.sql()} `
```

#### 6.8.6 表达式调试技巧

调试复杂表达式时，可以使用以下技巧：

1. **Print函数**：
   ```rust
   // 在表达式中添加print函数（仅在Python风格中有效）
   if print(user) && user.age > 18:
       ` and is_adult = 1 `
   ```

2. **启用详细日志**：
   ```rust
   fast_log::init(fast_log::Config::new().console().level(LevelFilter::Debug)).unwrap();
   ```

3. **表达式分解**：将复杂表达式分解为多个简单表达式，逐步验证

#### 6.8.7 表达式性能注意事项

1. **编译时评估**：Rbatis的表达式解析在编译时进行，不会影响运行时性能
2. **避免复杂表达式**：过于复杂的表达式可能导致生成的代码膨胀
3. **使用适当的类型**：尽量使用匹配的数据类型，减少运行时类型转换
4. **缓存计算结果**：对于重复使用的表达式结果，考虑预先计算并传递给SQL函数

通过深入理解Rbatis表达式引擎的工作原理，开发者可以更有效地编写动态SQL，充分利用Rust的类型安全性和编译时检查，同时保持SQL的灵活性和表达力。

## 7. 事务管理

Rbatis支持事务管理，可以在一个事务中执行多个SQL操作，要么全部成功，要么全部失败。

### 7.1 使用事务执行器

```rust
use rbatis::RBatis;

#[tokio::main]
async fn main() {
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://test.db").unwrap();
    
    // 获取事务执行器
    let mut tx = rb.acquire_begin().await.unwrap();
    
    // 在事务中执行多个操作
    let user1 = User {
        id: Some("1".to_string()),
        username: Some("user1".to_string()),
        password: Some("password1".to_string()),
        create_time: Some(DateTime::now()),
        status: Some(1),
    };
    
    let user2 = User {
        id: Some("2".to_string()),
        username: Some("user2".to_string()),
        password: Some("password2".to_string()),
        create_time: Some(DateTime::now()),
        status: Some(1),
    };
    
    // 插入第一个用户
    let result1 = User::insert(&mut tx, &user1).await;
    if result1.is_err() {
        // 如果出错，回滚事务
        tx.rollback().await.unwrap();
        println!("事务回滚: {:?}", result1.err());
        return;
    }
    
    // 插入第二个用户
    let result2 = User::insert(&mut tx, &user2).await;
    if result2.is_err() {
        // 如果出错，回滚事务
        tx.rollback().await.unwrap();
        println!("事务回滚: {:?}", result2.err());
        return;
    }
    
    // 提交事务
    tx.commit().await.unwrap();
    println!("事务提交成功");
}
```

## 8. 插件和拦截器

Rbatis提供了插件和拦截器机制，可以在SQL执行过程中进行拦截和处理。

### 8.1 日志拦截器

Rbatis默认内置了日志拦截器，可以记录SQL执行的详细信息：

```rust
use log::LevelFilter;
use rbatis::RBatis;
use rbatis::intercept_log::LogInterceptor;

fn main() {
    // 初始化日志系统
    fast_log::init(fast_log::Config::new().console().level(LevelFilter::Debug)).unwrap();
    
    // 创建RBatis实例
    let rb = RBatis::new();
    
    // 添加自定义日志拦截器
    rb.intercepts.clear(); // 清除默认拦截器
    rb.intercepts.push(Arc::new(LogInterceptor::new(LevelFilter::Debug)));
    
    // 后续操作...
}
```

### 8.2 自定义拦截器

可以实现`Intercept`特质来创建自定义拦截器：

```rust
use std::sync::Arc;
use async_trait::async_trait;
use rbatis::plugin::intercept::{Intercept, InterceptContext, InterceptResult};
use rbatis::RBatis;

// 定义自定义拦截器
#[derive(Debug)]
struct MyInterceptor;

#[async_trait]
impl Intercept for MyInterceptor {
    async fn before(&self, ctx: &mut InterceptContext) -> Result<bool, rbatis::Error> {
        println!("执行SQL前: {}", ctx.sql);
        // 返回true表示继续执行，false表示中断执行
        Ok(true)
    }
    
    async fn after(&self, ctx: &mut InterceptContext, res: &mut InterceptResult) -> Result<bool, rbatis::Error> {
        println!("执行SQL后: {}, 结果: {:?}", ctx.sql, res.return_value);
        // 返回true表示继续执行，false表示中断执行
        Ok(true)
    }
}

#[tokio::main]
async fn main() {
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://test.db").unwrap();
    
    // 添加自定义拦截器
    rb.intercepts.push(Arc::new(MyInterceptor {}));
    
    // 后续操作...
}
```

### 8.3 分页插件

Rbatis内置了分页拦截器，可以自动处理分页查询：

```rust
use rbatis::executor::Executor;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::{html_sql, RBatis};

#[html_sql(
r#"
select * from user
<where>
    <if test="name != null">
        and name like #{name}
    </if>
</where>
order by id desc
"#
)]
async fn select_page(
    rb: &dyn Executor,
    page_req: &PageRequest,
    name: Option<&str>,
) -> rbatis::Result<Page<User>> {
    impled!()
}

#[tokio::main]
async fn main() {
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://test.db").unwrap();
    
    // 创建分页请求
    let page_req = PageRequest::new(1, 10); // 第1页，每页10条
    
    // 执行分页查询
    let page_result = select_page(&rb, &page_req, Some("%test%")).await.unwrap();
    
    println!("总记录数: {}", page_result.total);
    println!("总页数: {}", page_result.pages);
    println!("当前页: {}", page_result.page_no);
    println!("每页大小: {}", page_result.page_size);
    println!("查询结果: {:?}", page_result.records);
}
```

## 9. 表同步和数据库管理

Rbatis提供了表同步功能，可以根据结构体定义自动创建或更新数据库表结构。

### 9.1 表同步

```rust
use rbatis::table_sync::{SqliteTableMapper, TableSync};
use rbatis::RBatis;

#[tokio::main]
async fn main() {
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://test.db").unwrap();
    
    // 获取数据库连接
    let conn = rb.acquire().await.unwrap();
    
    // 根据User结构体同步表结构
    // 第一个参数是连接，第二个参数是数据库特定的映射器，第三个参数是结构体实例，第四个参数是表名
    RBatis::sync(
        &conn,
        &SqliteTableMapper {},
        &User {
            id: Some(String::new()),
            username: Some(String::new()),
            password: Some(String::new()),
            create_time: Some(DateTime::now()),
            status: Some(0),
        },
        "user",
    )
    .await
    .unwrap();
    
    println!("表同步完成");
}
```

不同的数据库需要使用不同的表映射器：
- SQLite：`SqliteTableMapper`
- MySQL：`MysqlTableMapper`
- PostgreSQL：`PgTableMapper`
- SQL Server：`MssqlTableMapper`

### 9.2 表字段映射

可以使用`table_column`和`table_id`属性自定义字段映射：

```rust
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    #[table_id]
    pub id: Option<String>, // 主键字段
    
    #[serde(rename = "user_name")]
    #[table_column(rename = "user_name")]
    pub username: Option<String>, // 自定义列名
    
    pub password: Option<String>,
    
    #[table_column(default = "CURRENT_TIMESTAMP")] // 设置默认值
    pub create_time: Option<DateTime>,
    
    #[table_column(comment = "用户状态: 1=启用, 0=禁用")] // 添加列注释
    pub status: Option<i32>,
    
    #[table_column(ignore)] // 忽略此字段，不映射到表中
    pub temp_data: Option<String>,
}
```

## 10. 最佳实践

### 10.1 优化性能

- 使用连接池优化：合理配置连接池大小和超时设置，避免频繁创建和销毁连接
- 批量处理：使用批量插入、更新替代循环单条操作
- 懒加载：只在需要时加载相关数据，避免过度查询
- 适当索引：为常用查询字段建立合适的索引
- 避免N+1问题：使用联合查询替代多次单独查询

### 10.2 错误处理最佳实践

```rust
async fn handle_user_operation() -> Result<User, Error> {
    let rb = init_rbatis().await?;
    
    // 使用?操作符传播错误
    let user = rb.query_by_column("id", "1").await?;
    
    // 使用Result的组合器方法处理错误
    rb.update_by_column("id", &user).await
        .map_err(|e| {
            error!("更新用户信息失败: {}", e);
            Error::from(e)
        })?;
    
    Ok(user)
}
```

### 10.3 测试策略

- 单元测试：使用Mock数据库进行业务逻辑测试
- 集成测试：使用测试容器（如Docker）创建临时数据库环境
- 性能测试：模拟高并发场景测试系统性能和稳定性

## 11. 完整示例

以下是一个使用Rbatis构建的完整Web应用示例，展示了如何组织代码和使用Rbatis的各种功能。

### 11.1 项目结构

```
src/
├── main.rs                 # 应用入口
├── config.rs               # 配置管理
├── error.rs                # 错误定义
├── models/                 # 数据模型
│   ├── mod.rs
│   ├── user.rs
│   └── order.rs
├── repositories/           # 数据访问层
│   ├── mod.rs
│   ├── user_repository.rs
│   └── order_repository.rs
├── services/               # 业务逻辑层
│   ├── mod.rs
│   ├── user_service.rs
│   └── order_service.rs
└── api/                    # API接口层
    ├── mod.rs
    ├── user_controller.rs
    └── order_controller.rs
```

### 11.2 数据模型层

```rust
// models/user.rs
use rbatis::crud::CRUDTable;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub create_time: Option<DateTime>,
    pub status: Option<i32>,
}

impl CRUDTable for User {
    fn table_name() -> String {
        "user".to_string()
    }
    
    fn table_columns() -> String {
        "id,username,email,password,create_time,status".to_string()
    }
}
```

### 11.3 数据访问层

```rust
// repositories/user_repository.rs
use crate::models::user::User;
use rbatis::executor::Executor;
use rbatis::rbdc::Error;
use rbatis::rbdc::db::ExecResult;
use rbatis::plugin::page::{Page, PageRequest};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(rb: &dyn Executor, id: &str) -> Result<Option<User>, Error> {
        rb.query_by_column("id", id).await
    }
    
    pub async fn find_all(rb: &dyn Executor) -> Result<Vec<User>, Error> {
        rb.query("select * from user").await
    }
    
    pub async fn find_by_status(
        rb: &dyn Executor, 
        status: i32,
        page_req: &PageRequest
    ) -> Result<Page<User>, Error> {
        let wrapper = rb.new_wrapper()
            .eq("status", status);
        rb.fetch_page_by_wrapper(wrapper, page_req).await
    }
    
    pub async fn save(rb: &dyn Executor, user: &User) -> Result<ExecResult, Error> {
        rb.save(user).await
    }
    
    pub async fn update(rb: &dyn Executor, user: &User) -> Result<ExecResult, Error> {
        rb.update_by_column("id", user).await
    }
    
    pub async fn delete(rb: &dyn Executor, id: &str) -> Result<ExecResult, Error> {
        rb.remove_by_column::<User, _>("id", id).await
    }
    
    // 使用HTML风格动态SQL的高级查询
    #[html_sql(r#"
        select * from user
        where 1=1
        <if test="username != null and username != ''">
          and username like #{username}
        </if>
        <if test="status != null">
          and status = #{status}
        </if>
        order by create_time desc
    "#)]
    pub async fn search(
        rb: &dyn Executor,
        username: Option<String>,
        status: Option<i32>,
    ) -> Result<Vec<User>, Error> {
        todo!()
    }
}
```

### 11.4 业务逻辑层

```rust
// services/user_service.rs
use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;
use rbatis::rbatis::RBatis;
use rbatis::rbdc::Error;
use rbatis::plugin::page::{Page, PageRequest};

pub struct UserService {
    rb: RBatis,
}

impl UserService {
    pub fn new(rb: RBatis) -> Self {
        Self { rb }
    }
    
    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, Error> {
        UserRepository::find_by_id(&self.rb, id).await
    }
    
    pub async fn list_users(&self) -> Result<Vec<User>, Error> {
        UserRepository::find_all(&self.rb).await
    }
    
    pub async fn create_user(&self, user: &mut User) -> Result<(), Error> {
        // 添加业务逻辑，如密码加密、数据验证等
        if user.status.is_none() {
            user.status = Some(1); // 默认状态
        }
        user.create_time = Some(rbatis::rbdc::datetime::DateTime::now());
        
        // 开启事务处理
        let tx = self.rb.acquire_begin().await?;
        
        // 检查用户名是否已存在
        let exist_users = UserRepository::search(
            &tx, 
            Some(user.username.clone()), 
            None
        ).await?;
        
        if !exist_users.is_empty() {
            tx.rollback().await?;
            return Err(Error::from("用户名已存在"));
        }
        
        // 保存用户
        UserRepository::save(&tx, user).await?;
        
        // 提交事务
        tx.commit().await?;
        
        Ok(())
    }
    
    pub async fn update_user(&self, user: &User) -> Result<(), Error> {
        if user.id.is_none() {
            return Err(Error::from("用户ID不能为空"));
        }
        
        // 检查用户是否存在
        let exist = UserRepository::find_by_id(&self.rb, user.id.as_ref().unwrap()).await?;
        if exist.is_none() {
            return Err(Error::from("用户不存在"));
        }
        
        UserRepository::update(&self.rb, user).await?;
        Ok(())
    }
    
    pub async fn delete_user(&self, id: &str) -> Result<(), Error> {
        UserRepository::delete(&self.rb, id).await?;
        Ok(())
    }
    
    pub async fn search_users(
        &self,
        username: Option<String>,
        status: Option<i32>,
        page: u64,
        page_size: u64
    ) -> Result<Page<User>, Error> {
        if let Some(username_str) = &username {
            // 模糊查询处理
            let like_username = format!("%{}%", username_str);
            UserRepository::search(&self.rb, Some(like_username), status).await
                .map(|users| {
                    // 手动分页处理
                    let total = users.len() as u64;
                    let start = (page - 1) * page_size;
                    let end = std::cmp::min(start + page_size, total);
                    
                    let records = if start < total {
                        users[start as usize..end as usize].to_vec()
                    } else {
                        vec![]
                    };
                    
                    Page {
                        records,
                        page_no: page,
                        page_size,
                        total,
                    }
                })
        } else {
            // 使用内置分页查询
            let page_req = PageRequest::new(page, page_size);
            UserRepository::find_by_status(&self.rb, status.unwrap_or(1), &page_req).await
        }
    }
}
```

### 11.5 API接口层

```rust
// api/user_controller.rs
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::models::user::User;
use crate::services::user_service::UserService;

#[derive(Deserialize)]
pub struct UserQuery {
    username: Option<String>,
    status: Option<i32>,
    page: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }
    
    pub fn error(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}

pub async fn get_user(
    path: web::Path<String>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    let id = path.into_inner();
    
    match user_service.get_user_by_id(&id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse::success(user)),
        Ok(None) => HttpResponse::NotFound().json(
            ApiResponse::<()>::error(404, "用户不存在".to_string())
        ),
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(500, format!("服务器错误: {}", e))
        ),
    }
}

pub async fn list_users(
    query: web::Query<UserQuery>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    
    match user_service.search_users(
        query.username.clone(),
        query.status,
        page,
        page_size
    ).await {
        Ok(users) => HttpResponse::Ok().json(ApiResponse::success(users)),
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(500, format!("服务器错误: {}", e))
        ),
    }
}

pub async fn create_user(
    user: web::Json<User>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    let mut new_user = user.into_inner();
    
    match user_service.create_user(&mut new_user).await {
        Ok(_) => HttpResponse::Created().json(ApiResponse::success(new_user)),
        Err(e) => {
            if e.to_string().contains("用户名已存在") {
                HttpResponse::BadRequest().json(
                    ApiResponse::<()>::error(400, e.to_string())
                )
            } else {
                HttpResponse::InternalServerError().json(
                    ApiResponse::<()>::error(500, format!("服务器错误: {}", e))
                )
            }
        }
    }
}

pub async fn update_user(
    user: web::Json<User>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    match user_service.update_user(&user).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success(())),
        Err(e) => {
            if e.to_string().contains("用户不存在") {
                HttpResponse::NotFound().json(
                    ApiResponse::<()>::error(404, e.to_string())
                )
            } else {
                HttpResponse::InternalServerError().json(
                    ApiResponse::<()>::error(500, format!("服务器错误: {}", e))
                )
            }
        }
    }
}

pub async fn delete_user(
    path: web::Path<String>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    let id = path.into_inner();
    
    match user_service.delete_user(&id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success(())),
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(500, format!("服务器错误: {}", e))
        ),
    }
}
```

### 11.6 应用配置和启动

```rust
// main.rs
use actix_web::{web, App, HttpServer};
use rbatis::rbatis::RBatis;

mod api;
mod models;
mod repositories;
mod services;
mod config;
mod error;

use crate::api::user_controller;
use crate::services::user_service::UserService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init();
    
    // 初始化数据库连接
    let rb = RBatis::new();
    rb.init(
        rbdc_mysql::driver::MysqlDriver{}, 
        &config::get_database_url()
    ).unwrap();
    
    // 运行表同步（可选）
    rb.sync(models::user::User {
        id: None,
        username: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
        create_time: None,
        status: None,
    }).await.unwrap();
    
    // 创建服务
    let user_service = UserService::new(rb.clone());
    
    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/users")
                            .route("", web::get().to(user_controller::list_users))
                            .route("", web::post().to(user_controller::create_user))
                            .route("", web::put().to(user_controller::update_user))
                            .route("/{id}", web::get().to(user_controller::get_user))
                            .route("/{id}", web::delete().to(user_controller::delete_user))
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### 11.7 客户端调用示例

```rust
// 使用reqwest客户端调用API
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<String>,
    username: String,
    email: String,
    password: String,
    status: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    code: i32,
    message: String,
    data: Option<T>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 创建用户
    let new_user = User {
        id: None,
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        status: Some(1),
    };
    
    let resp = client.post("http://localhost:8080/api/users")
        .json(&new_user)
        .send()
        .await?
        .json::<ApiResponse<User>>()
        .await?;
    
    println!("创建用户响应: {:?}", resp);
    
    // 查询用户列表
    let resp = client.get("http://localhost:8080/api/users")
        .query(&[("page", "1"), ("page_size", "10")])
        .send()
        .await?
        .json::<ApiResponse<Vec<User>>>()
        .await?;
    
    println!("用户列表: {:?}", resp);
    
    Ok(())
}
```

这个完整示例展示了如何使用Rbatis构建一个包含数据模型、数据访问层、业务逻辑层和API接口层的Web应用，覆盖了Rbatis的各种特性，包括基本CRUD操作、动态SQL、事务管理、分页查询等。通过这个示例，开发者可以快速理解如何在实际项目中有效使用Rbatis。

## 11.8 现代Rbatis 4.5+示例

以下是一个简洁的示例，展示了Rbatis 4.5+的推荐使用方法：

```rust
use rbatis::{crud, impl_select, impl_update, impl_delete, RBatis};
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize, Serialize};
use rbatis::rbdc::datetime::DateTime;

// 定义数据模型
#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
    id: Option<String>,
    username: Option<String>,
    email: Option<String>,
    status: Option<i32>,
    create_time: Option<DateTime>,
}

// 生成基本CRUD方法
crud!(User {});

// 定义自定义查询方法
impl_select!(User{find_by_username(username: &str) -> Option => 
    "` where username = #{username} limit 1`"});

impl_select!(User{find_active_users() -> Vec => 
    "` where status = 1 order by create_time desc`"});

impl_update!(User{update_status(id: &str, status: i32) =>
    "` set status = #{status} where id = #{id}`"});

impl_delete!(User{remove_inactive() =>
    "` where status = 0`"});

// 定义分页查询
impl_select_page!(User{find_by_email_page(email: &str) =>
    "` where email like #{email}`"});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    fast_log::init(fast_log::Config::new().console()).unwrap();
    
    // 创建RBatis实例并连接数据库
    let rb = RBatis::new();
    rb.link(SqliteDriver {}, "sqlite://test.db").await?;
    
    // 创建新用户
    let user = User {
        id: Some("1".to_string()),
        username: Some("test_user".to_string()),
        email: Some("test@example.com".to_string()),
        status: Some(1),
        create_time: Some(DateTime::now()),
    };
    
    // 插入用户
    User::insert(&rb, &user).await?;
    
    // 通过用户名查找用户（返回Option<User>）
    let found_user = User::find_by_username(&rb, "test_user").await?;
    println!("查找到的用户: {:?}", found_user);
    
    // 查找所有活跃用户（返回Vec<User>）
    let active_users = User::find_active_users(&rb).await?;
    println!("活跃用户数量: {}", active_users.len());
    
    // 更新用户状态
    User::update_status(&rb, "1", 2).await?;
    
    // 分页查询（返回Page<User>）
    use rbatis::plugin::page::PageRequest;
    let page_req = PageRequest::new(1, 10);
    let user_page = User::find_by_email_page(&rb, &page_req, "%example%").await?;
    println!("总用户数: {}, 当前页: {}", user_page.total, user_page.page_no);
    
    // 按列删除
    User::delete_by_column(&rb, "id", "1").await?;
    
    // 使用自定义方法删除非活跃用户
    User::remove_inactive(&rb).await?;
    
    Ok(())
}
```

这个示例展示了使用Rbatis 4.5+的现代方法：
1. 使用`#[derive]`属性定义数据模型
2. 使用`crud!`宏生成基本CRUD方法
3. 使用适当的`impl_*`宏定义自定义查询
4. 为方法返回使用强类型（Option、Vec、Page等）
5. 对所有数据库操作使用async/await

# 12. 总结

Rbatis是一个功能强大且灵活的ORM框架，适用于多种数据库类型。它提供了丰富的动态SQL功能，支持多种参数绑定方式，并提供了插件和拦截器机制。Rbatis的表达式引擎是其动态SQL的核心，负责在编译时解析和处理表达式，并转换为Rust代码。通过深入理解Rbatis的工作原理，开发者可以更有效地编写动态SQL，充分利用Rust的类型安全性和编译时检查，同时保持SQL的灵活性和表达力。

遵循最佳实践，可以充分发挥Rbatis框架的优势，构建高效、可靠的数据库应用。

### 重要编码规范

1. **使用小写SQL关键字**：Rbatis处理机制基于小写SQL关键字，所有SQL语句必须使用小写形式的`select`、`insert`、`update`、`delete`、`where`、`from`、`order by`等关键字，不要使用大写形式。
2. **正确处理空格**：使用反引号（`）包裹SQL片段以保留前导空格。
3. **类型安全**：充分利用Rust的类型系统，使用`Option<T>`处理可空字段。
4. **遵循异步编程模型**：Rbatis是异步ORM，所有数据库操作都应使用`.await`等待完成。 