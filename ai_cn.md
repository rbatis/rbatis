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
   // 注意：文档注释必须放在impl_*宏的上面，而不是里面
   /// 按名称查询用户
   impl_select!(User {select_by_name(name: &str) -> Vec => "` where name = #{name}`"});
   
   /// 按ID获取用户
   impl_select!(User {select_by_id(id: &str) -> Option => "` where id = #{id} limit 1`"});
   
   /// 根据ID更新用户状态
   impl_update!(User {update_status_by_id(id: &str, status: i32) => "` set status = #{status} where id = #{id}`"});
   
   /// 按名称删除用户
   impl_delete!(User {delete_by_name(name: &str) => "` where name = #{name}`"});
   ```

3. **使用小写SQL关键字**：SQL关键字始终使用小写，如`select`、`where`、`and`等。

4. **正确使用反引号**：用反引号(`)包裹动态SQL片段以保留空格。

5. **异步优先方法**：所有数据库操作都应使用`.await`等待完成。

6. **使用雪花ID或ObjectId作为主键**：Rbatis提供了内置的ID生成机制，应该用于主键。

7. **优先使用select_in_column而非JOIN**：为了更好的性能和可维护性，避免复杂的JOIN查询，使用Rbatis的select_in_column获取关联数据，然后在服务层合并它们。

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

> **注意**：Rbatis处理机制要求SQL关键字使用小写形式（select、insert、update、delete等），这可能与某些SQL样式指南不同。使用Rbatis时，请始终使用小写SQL关键字以确保正确解析和执行。

### 5.1 使用CRUD宏

最简单的方式是使用`crud!`宏：

```rust
use rbatis::crud;

// 为User结构体自动生成CRUD方法
// 如果指定了表名，则使用指定的表名；否则，使用结构体名称的蛇形命名法作为表名
crud!(User {}); // 表名为user
// 或
crud!(User {}, "users"); // 表名为users
```

这将为User结构体生成以下方法：
- `User::insert`：插入单条记录
- `User::insert_batch`：批量插入记录
- `User::update_by_column`：基于指定列更新记录
- `User::update_by_column_batch`：批量更新记录
- `User::delete_by_column`：基于指定列删除记录
- `User::delete_in_column`：删除列值在指定集合中的记录
- `User::select_by_column`：基于指定列查询记录
- `User::select_in_column`：查询列值在指定集合中的记录
- `User::select_all`：查询所有记录
- `User::select_by_map`：基于映射条件查询记录

### 5.1.1 CRUD宏详细参考

`crud!`宏自动为您的数据模型生成一套完整的CRUD（创建、读取、更新、删除）操作。在底层，它展开为调用这四个实现宏：

```rust
// 等同于 
impl_insert!(User {});
impl_select!(User {});
impl_update!(User {});
impl_delete!(User {});
```

#### 生成的方法

当您使用`crud!(User {})`时，将生成以下方法：

##### 插入方法
- **`async fn insert(executor: &dyn Executor, table: &User) -> Result<ExecResult, Error>`**  
  插入单条记录。
  
- **`async fn insert_batch(executor: &dyn Executor, tables: &[User], batch_size: u64) -> Result<ExecResult, Error>`**  
  批量插入多条记录。`batch_size`参数控制每批操作插入的记录数。

##### 查询方法
- **`async fn select_all(executor: &dyn Executor) -> Result<Vec<User>, Error>`**  
  从表中检索所有记录。
  
- **`async fn select_by_column<V: Serialize>(executor: &dyn Executor, column: &str, column_value: V) -> Result<Vec<User>, Error>`**  
  检索指定列等于给定值的记录。
  
- **`async fn select_by_map(executor: &dyn Executor, condition: rbs::Value) -> Result<Vec<User>, Error>`**  
  检索匹配列值条件映射的记录（AND逻辑）。
  
- **`async fn select_in_column<V: Serialize>(executor: &dyn Executor, column: &str, column_values: &[V]) -> Result<Vec<User>, Error>`**  
  检索指定列的值在给定值列表中的记录（IN操作符）。

##### 更新方法
- **`async fn update_by_column(executor: &dyn Executor, table: &User, column: &str) -> Result<ExecResult, Error>`**  
  基于指定列（用作WHERE条件）更新记录。空值将被跳过。
  
- **`async fn update_by_column_batch(executor: &dyn Executor, tables: &[User], column: &str, batch_size: u64) -> Result<ExecResult, Error>`**  
  批量更新多条记录，使用指定列作为条件。
  
- **`async fn update_by_column_skip(executor: &dyn Executor, table: &User, column: &str, skip_null: bool) -> Result<ExecResult, Error>`**  
  更新记录，可控制是否跳过空值。
  
- **`async fn update_by_map(executor: &dyn Executor, table: &User, condition: rbs::Value, skip_null: bool) -> Result<ExecResult, Error>`**  
  更新匹配列值条件映射的记录。

##### 删除方法
- **`async fn delete_by_column<V: Serialize>(executor: &dyn Executor, column: &str, column_value: V) -> Result<ExecResult, Error>`**  
  删除指定列等于给定值的记录。
  
- **`async fn delete_by_map(executor: &dyn Executor, condition: rbs::Value) -> Result<ExecResult, Error>`**  
  删除匹配列值条件映射的记录。
  
- **`async fn delete_in_column<V: Serialize>(executor: &dyn Executor, column: &str, column_values: &[V]) -> Result<ExecResult, Error>`**  
  删除指定列的值在给定列表中的记录（IN操作符）。
  
- **`async fn delete_by_column_batch<V: Serialize>(executor: &dyn Executor, column: &str, values: &[V], batch_size: u64) -> Result<ExecResult, Error>`**  
  基于指定列值批量删除多条记录。

#### 使用示例

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化RBatis
    let rb = RBatis::new();
    rb.link(SqliteDriver {}, "sqlite://test.db").await?;
    
    // 插入单条记录
    let user = User {
        id: Some("1".to_string()),
        username: Some("john_doe".to_string()),
        // 其他字段...
    };
    User::insert(&rb, &user).await?;
    
    // 批量插入多条记录
    let users = vec![user1, user2, user3];
    User::insert_batch(&rb, &users, 100).await?;
    
    // 按列查询
    let active_users: Vec<User> = User::select_by_column(&rb, "status", 1).await?;
    
    // 使用IN子句查询
    let specific_users = User::select_in_column(&rb, "id", &["1", "2", "3"]).await?;
    
    // 更新记录
    let mut user_to_update = active_users[0].clone();
    user_to_update.status = Some(2);
    User::update_by_column(&rb, &user_to_update, "id").await?;
    
    // 删除记录
    User::delete_by_column(&rb, "id", "1").await?;
    
    // 使用IN子句删除多条记录
    User::delete_in_column(&rb, "status", &[0, -1]).await?;
    
    Ok(())
}
```

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

> ⚠️ **重要警告**
> 
> 在使用Rbatis XML格式时，请不要使用MyBatis风格的`BaseResultMap`或`Base_Column_List`！
> 
> 与MyBatis不同，Rbatis不需要也不支持：
> - `<result id="BaseResultMap" column="id,name,status"/>`
> - `<sql id="Base_Column_List">id,name,status</sql>`
> 
> Rbatis自动将数据库列映射到Rust结构体字段，因此这些结构是不必要的，并且可能导致错误。请始终编写完整的SQL语句，明确选择列或使用`SELECT *`。

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

#### 6.1.1 有效的XML结构

在Rbatis中使用HTML/XML风格时，必须遵循DTD中定义的正确结构：

```
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" 
"https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
```

**重要说明：**

1. **有效的顶级元素**：`<mapper>`元素只能包含：`<sql>`、`<insert>`、`<update>`、`<delete>`或`<select>`元素。

2. **不要使用BaseResultMap**：与MyBatis不同，Rbatis不使用`<resultMap>`或`BaseResultMap`。Rbatis自动将列映射到结构体字段。

3. **始终使用实际SQL查询**：不要使用列列表，直接编写SQL查询。

❌ **错误用法**（不要使用）：
```xml
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "...">
<mapper>
    <!-- 错误：result不是mapper的有效直接子元素 -->
    <result id="BaseResultMap" column="id,name,status"/>
    <!-- 错误：列列表应该在SQL中直接使用 -->
    <sql id="Base_Column_List">id,name,status</sql>
</mapper>
```

✅ **正确用法**（使用这种方式）：
```xml
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "...">
<mapper>
    <!-- 正确：使用select元素直接编写SQL -->
    <select id="select_by_id">
        select * from user where id = #{id}
    </select>
    
    <!-- 正确：sql元素用于SQL片段 -->
    <sql id="where_clause">
        <where>
            <if test="name != null">
                ` and name like #{name} `
            </if>
        </where>
    </sql>
    
    <select id="select_with_where">
        select * from user
        <include refid="where_clause"/>
    </select>
</mapper>
```

#### 6.1.2 空格处理机制

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

// 使用HTML风格SQL进行复杂查询
#[html_sql(r#"
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" 
"https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
<mapper>
    <select id="find_users_by_criteria">
        select * from user
        <where>
            <if test="username != null">
                ` and username like #{username} `
            </if>
            <if test="email != null">
                ` and email like #{email} `
            </if>
            <if test="status_list != null and status_list.len > 0">
                ` and status in `
                <foreach collection="status_list" item="item" open="(" close=")" separator=",">
                    #{item}
                </foreach>
            </if>
            <choose>
                <when test="sort_by == 'name'">
                    ` order by username `
                </when>
                <when test="sort_by == 'date'">
                    ` order by create_time `
                </when>
                <otherwise>
                    ` order by id `
                </otherwise>
            </choose>
        </where>
    </select>
</mapper>
"#)]
async fn find_users_by_criteria(
    rb: &dyn rbatis::executor::Executor,
    username: Option<&str>,
    email: Option<&str>,
    status_list: Option<&[i32]>,
    sort_by: &str
) -> rbatis::Result<Vec<User>> {
    impled!()
}

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
    
    // 使用HTML SQL进行复杂查询
    let status_list = vec![1, 2, 3];
    let users = find_users_by_criteria(&rb, Some("test%"), None, Some(&status_list), "name").await?;
    println!("符合条件的用户数: {}", users.len());
    
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
6. 对于复杂查询，使用格式正确的HTML SQL，遵循正确的mapper结构

# 12. 总结

Rbatis是一个功能强大且灵活的ORM框架，适用于多种数据库类型。它提供了丰富的动态SQL功能，支持多种参数绑定方式，并提供了插件和拦截器机制。Rbatis的表达式引擎是其动态SQL的核心，负责在编译时解析和处理表达式，并转换为Rust代码。通过深入理解Rbatis的工作原理，开发者可以更有效地编写动态SQL，充分利用Rust的类型安全性和编译时检查，同时保持SQL的灵活性和表达力。

遵循最佳实践，可以充分发挥Rbatis框架的优势，构建高效、可靠的数据库应用。

### 重要编码规范

1. **使用小写SQL关键字**：Rbatis处理机制基于小写SQL关键字，所有SQL语句必须使用小写形式的`select`、`insert`、`update`、`delete`、`where`、`from`、`order by`等关键字，不要使用大写形式。
2. **正确处理空格**：使用反引号（`）包裹SQL片段以保留前导空格。
3. **类型安全**：充分利用Rust的类型系统，使用`Option<T>`处理可空字段。
4. **遵循异步编程模型**：Rbatis是异步ORM，所有数据库操作都应使用`.await`等待完成。 

# 3.5 ID生成

Rbatis提供了内置的ID生成机制，推荐用于数据库表的主键。使用这些机制可以确保全局唯一的ID，并为分布式系统提供更好的性能。

## 3.5.1 雪花ID (SnowflakeId)

雪花ID是由Twitter最初开发的分布式ID生成算法。它生成由以下部分组成的64位ID：
- 时间戳
- 机器ID
- 序列号

```rust
use rbatis::snowflake::new_snowflake_id;

// 在模型定义中
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    // 使用i64存储雪花ID
    pub id: Option<i64>,
    pub username: Option<String>,
    // 其他字段...
}

// 创建新记录时
async fn create_user(rb: &RBatis, username: &str) -> rbatis::Result<User> {
    let mut user = User {
        id: Some(new_snowflake_id()), // 生成新的雪花ID
        username: Some(username.to_string()),
        // 初始化其他字段...
    };
    
    User::insert(rb, &user).await?;
    Ok(user)
}
```

## 3.5.2 ObjectId

ObjectId受MongoDB的ObjectId启发，提供了由以下部分组成的12字节标识符：
- 4字节时间戳
- 3字节机器标识符
- 2字节进程ID
- 3字节计数器

```rust
use rbatis::object_id::ObjectId;

// 在模型定义中
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    // 可以使用String存储ObjectId
    pub id: Option<String>,
    pub title: Option<String>,
    // 其他字段...
}

// 创建新记录时
async fn create_document(rb: &RBatis, title: &str) -> rbatis::Result<Document> {
    let mut doc = Document {
        id: Some(ObjectId::new().to_string()), // 生成新的ObjectId作为字符串
        title: Some(title.to_string()),
        // 初始化其他字段...
    };
    
    Document::insert(rb, &doc).await?;
    Ok(doc)
}

// 或者，你可以直接在模型中使用ObjectId
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentWithObjectId {
    pub id: Option<ObjectId>,
    pub title: Option<String>,
    // 其他字段...
}

async fn create_document_with_object_id(rb: &RBatis, title: &str) -> rbatis::Result<DocumentWithObjectId> {
    let mut doc = DocumentWithObjectId {
        id: Some(ObjectId::new()), // 生成新的ObjectId
        title: Some(title.to_string()),
        // 初始化其他字段...
    };
    
    DocumentWithObjectId::insert(rb, &doc).await?;
    Ok(doc)
}
```

## 6.5 文档和注释

使用Rbatis宏时，遵循一定的文档和注释约定很重要。

### 6.5.1 为impl_*宏添加文档

为`impl_*`宏生成的方法添加文档注释时，注释**必须**放在宏的**上面**，而不是里面：

```rust
// 正确：文档注释在宏的上面
/// 根据状态查找用户
/// @param status: 要搜索的用户状态
impl_select!(User {find_by_status(status: i32) -> Vec => 
    "` where status = #{status}`"});

// 错误：会导致编译错误
impl_select!(User {
    /// 宏内的这个注释会导致错误
    find_by_name(name: &str) -> Vec => 
        "` where name = #{name}`"
});
```

### 6.5.2 注释的常见错误

一个常见的错误是在宏内部放置文档注释：

```rust
// 这会导致编译错误
impl_select!(DiscountTask {
    /// 按类型查询折扣任务
    find_by_type(task_type: &str) -> Vec => 
        "` where task_type = #{task_type} and state = 'published' and deleted = 0 and end_time > now() order by discount_percent desc`"
});
```

正确的方法是：

```rust
// 这样可以正常工作
/// 按类型查询折扣任务
impl_select!(DiscountTask {find_by_type(task_type: &str) -> Vec => 
    "` where task_type = #{task_type} and state = 'published' and deleted = 0 and end_time > now() order by discount_percent desc`"});
```

### 6.5.3 为什么这很重要

Rbatis过程宏系统在编译时解析宏内容。当文档注释放在宏内部时，它们会干扰解析过程，导致编译错误。通过将文档注释放在宏外部，它们会正确地附加到生成的方法上，同时避免解析器问题。 

## 12. 处理关联数据

在处理表之间的关联数据（如一对多或多对多关系）时，Rbatis建议使用`select_in_column`而不是复杂的JOIN查询。这种方法在大多数情况下更高效且更易于维护。

### 12.1 JOIN查询的问题

虽然SQL JOIN功能强大，但它们可能会导致几个问题：
- 难以维护的复杂查询
- 大数据集的性能问题
- 处理嵌套关系的困难
- 将平面结果集映射到对象层次结构的挑战

### 12.2 Rbatis方法：select_in_column

Rbatis建议，不要使用JOIN，而是：
1. 先查询主实体
2. 从主实体中提取ID
3. 使用`select_in_column`批量获取关联实体
4. 在服务层合并数据

这种方法有几个优点：
- 大数据集的性能更好
- 代码更清晰，更易于维护
- 更好地控制获取的数据
- 避免N+1查询问题

### 12.3 示例：一对多关系

```rust
// 实体
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub total: Option<f64>,
    // 其他字段...
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Option<String>,
    pub order_id: Option<String>,
    pub product_id: Option<String>,
    pub quantity: Option<i32>,
    pub price: Option<f64>,
    // 其他字段...
}

// 生成CRUD操作
crud!(Order {});
crud!(OrderItem {});

// OrderItem的自定义方法
impl_select!(OrderItem {
    select_by_order_ids(order_ids: &[String]) -> Vec =>
        "` where order_id in ${order_ids.sql()} order by id asc`"
});

// 服务层
pub struct OrderService {
    rb: RBatis,
}

impl OrderService {
    // 获取订单及其项目
    pub async fn get_orders_with_items(&self, user_id: &str) -> rbatis::Result<Vec<OrderWithItems>> {
        // 步骤1：获取用户的所有订单
        let orders = Order::select_by_column(&self.rb, "user_id", user_id).await?;
        if orders.is_empty() {
            return Ok(vec![]);
        }
        
        // 步骤2：提取订单ID
        let order_ids: Vec<String> = orders
            .iter()
            .filter_map(|order| order.id.clone())
            .collect();
            
        // 步骤3：在单个查询中获取所有订单项
        let items = OrderItem::select_by_order_ids(&self.rb, &order_ids).await?;
        
        // 步骤4：按order_id分组项目以便快速查找
        let mut items_map: HashMap<String, Vec<OrderItem>> = HashMap::new();
        for item in items {
            if let Some(order_id) = &item.order_id {
                items_map
                    .entry(order_id.clone())
                    .or_insert_with(Vec::new)
                    .push(item);
            }
        }
        
        // 步骤5：将订单与其项目组合
        let result = orders
            .into_iter()
            .map(|order| {
                let order_id = order.id.clone().unwrap_or_default();
                let order_items = items_map.get(&order_id).cloned().unwrap_or_default();
                
                OrderWithItems {
                    order,
                    items: order_items,
                }
            })
            .collect();
            
        Ok(result)
    }
}

// 组合数据结构
pub struct OrderWithItems {
    pub order: Order,
    pub items: Vec<OrderItem>,
}
```

### 12.4 示例：多对多关系

```rust
// 实体
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: Option<String>,
    pub name: Option<String>,
    // 其他字段...
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<String>,
    pub title: Option<String>,
    // 其他字段...
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentCourse {
    pub id: Option<String>,
    pub student_id: Option<String>,
    pub course_id: Option<String>,
    pub enrollment_date: Option<DateTime>,
}

// 生成CRUD操作
crud!(Student {});
crud!(Course {});
crud!(StudentCourse {});

// 自定义方法
impl_select!(StudentCourse {
    select_by_student_ids(student_ids: &[String]) -> Vec =>
        "` where student_id in ${student_ids.sql()}`"
});

impl_select!(Course {
    select_by_ids(ids: &[String]) -> Vec =>
        "` where id in ${ids.sql()}`"
});

// 服务层函数，获取学生及其课程
async fn get_students_with_courses(rb: &RBatis) -> rbatis::Result<Vec<StudentWithCourses>> {
    // 步骤1：获取所有学生
    let students = Student::select_all(rb).await?;
    
    // 步骤2：提取学生ID
    let student_ids: Vec<String> = students
        .iter()
        .filter_map(|s| s.id.clone())
        .collect();
        
    // 步骤3：获取这些学生的所有选课记录
    let enrollments = StudentCourse::select_by_student_ids(rb, &student_ids).await?;
    
    // 步骤4：从选课记录中提取课程ID
    let course_ids: Vec<String> = enrollments
        .iter()
        .filter_map(|e| e.course_id.clone())
        .collect();
        
    // 步骤5：在一个查询中获取所有课程
    let courses = Course::select_by_ids(rb, &course_ids).await?;
    
    // 步骤6：创建查找映射
    let mut enrollment_map: HashMap<String, Vec<StudentCourse>> = HashMap::new();
    for enrollment in enrollments {
        if let Some(student_id) = &enrollment.student_id {
            enrollment_map
                .entry(student_id.clone())
                .or_insert_with(Vec::new)
                .push(enrollment);
        }
    }
    
    let course_map: HashMap<String, Course> = courses
        .into_iter()
        .filter_map(|course| {
            course.id.clone().map(|id| (id, course))
        })
        .collect();
    
    // 步骤7：组合所有内容
    let result = students
        .into_iter()
        .map(|student| {
            let student_id = student.id.clone().unwrap_or_default();
            let student_enrollments = enrollment_map.get(&student_id).cloned().unwrap_or_default();
            
            let student_courses = student_enrollments
                .iter()
                .filter_map(|enrollment| {
                    enrollment.course_id.clone().and_then(|course_id| {
                        course_map.get(&course_id).cloned()
                    })
                })
                .collect();
                
            StudentWithCourses {
                student,
                courses: student_courses,
            }
        })
        .collect();
        
    Ok(result)
}

// 组合数据结构
pub struct StudentWithCourses {
    pub student: Student,
    pub courses: Vec<Course>,
}
```

通过使用这种方法，你可以：
1. 避免复杂的JOIN查询
2. 最小化数据库查询次数（避免N+1问题）
3. 保持数据访问和业务逻辑之间的清晰分离
4. 更好地控制数据获取和转换
5. 轻松处理更复杂的嵌套关系

### 12.5 使用Rbatis表工具宏进行数据关联

Rbatis在`table_util.rs`中提供了几个强大的工具宏，可以在合并相关实体数据时显著简化数据处理。这些宏是SQL JOIN的更高效替代方案：

#### 12.5.1 可用的表工具宏

1. **`table_field_vec!`** - 将集合中的特定字段提取到新的Vec中：
   ```rust
   // 从用户角色集合中提取所有角色ID
   let role_ids: Vec<String> = table_field_vec!(user_roles, role_id);
   // 使用引用（不克隆）
   let role_ids_ref: Vec<&String> = table_field_vec!(&user_roles, role_id);
   ```

2. **`table_field_set!`** - 将特定字段提取到HashSet中（适用于唯一值）：
   ```rust
   // 提取唯一的角色ID
   let role_ids: HashSet<String> = table_field_set!(user_roles, role_id);
   // 使用引用
   let role_ids_ref: HashSet<&String> = table_field_set!(&user_roles, role_id);
   ```

3. **`table_field_map!`** - 创建以特定字段为键的HashMap：
   ```rust
   // 创建以role_id为键、UserRole为值的HashMap
   let role_map: HashMap<String, SysUserRole> = table_field_map!(user_roles, role_id);
   ```

4. **`table_field_btree!`** - 创建以特定字段为键的BTreeMap（有序映射）：
   ```rust
   // 创建以role_id为键的BTreeMap
   let role_btree: BTreeMap<String, SysUserRole> = table_field_btree!(user_roles, role_id);
   ```

5. **`table!`** - 通过使用Default特性简化表构造：
   ```rust
   // 创建一个特定字段已初始化的新实例
   let user = table!(User { id: new_snowflake_id(), name: "张三".to_string() });
   ```

#### 12.5.2 改进示例：一对多关系

以下是如何使用这些工具简化一对多示例：

```rust
// 导入
use std::collections::HashMap;
use rbatis::{table_field_vec, table_field_map};

// 服务方法
pub async fn get_orders_with_items(&self, user_id: &str) -> rbatis::Result<Vec<OrderWithItems>> {
    // 获取用户的所有订单
    let orders = Order::select_by_column(&self.rb, "user_id", user_id).await?;
    if orders.is_empty() {
        return Ok(vec![]);
    }
    
    // 使用table_field_vec!宏提取订单ID - 更简洁！
    let order_ids = table_field_vec!(orders, id);
    
    // 在单个查询中获取所有订单项
    let items = OrderItem::select_by_order_ids(&self.rb, &order_ids).await?;
    
    // 使用table_field_map!按order_id分组项目 - 自动分组！
    let mut items_map: HashMap<String, Vec<OrderItem>> = HashMap::new();
    for (order_id, item) in table_field_map!(items, order_id) {
        items_map.entry(order_id).or_insert_with(Vec::new).push(item);
    }
    
    // 将订单映射到结果
    let result = orders
        .into_iter()
        .map(|order| {
            let order_id = order.id.clone().unwrap_or_default();
            let order_items = items_map.get(&order_id).cloned().unwrap_or_default();
            
            OrderWithItems {
                order,
                items: order_items,
            }
        })
        .collect();
        
    Ok(result)
}
```

#### 12.5.3 简化的多对多示例

对于多对多关系，这些工具也能简化代码：

```rust
// 导入
use std::collections::{HashMap, HashSet};
use rbatis::{table_field_vec, table_field_set, table_field_map};

// 多对多的服务函数
async fn get_students_with_courses(rb: &RBatis) -> rbatis::Result<Vec<StudentWithCourses>> {
    // 获取所有学生
    let students = Student::select_all(rb).await?;
    
    // 使用工具宏提取学生ID
    let student_ids = table_field_vec!(students, id);
    
    // 获取这些学生的选课记录
    let enrollments = StudentCourse::select_by_student_ids(rb, &student_ids).await?;
    
    // 使用set提取唯一课程ID（自动去除重复）
    let course_ids = table_field_set!(enrollments, course_id);
    
    // 在一个查询中获取所有课程
    let courses = Course::select_by_ids(rb, &course_ids.into_iter().collect::<Vec<_>>()).await?;
    
    // 使用工具宏创建查找映射
    let course_map = table_field_map!(courses, id);
    
    // 创建学生->选课记录的映射
    let mut student_enrollments: HashMap<String, Vec<StudentCourse>> = HashMap::new();
    for enrollment in enrollments {
        if let Some(student_id) = &enrollment.student_id {
            student_enrollments
                .entry(student_id.clone())
                .or_insert_with(Vec::new)
                .push(enrollment);
        }
    }
    
    // 构建结果
    let result = students
        .into_iter()
        .map(|student| {
            let student_id = student.id.clone().unwrap_or_default();
            let enrollments = student_enrollments.get(&student_id).cloned().unwrap_or_default();
            
            // 将选课记录映射到课程
            let student_courses = enrollments
                .iter()
                .filter_map(|enrollment| {
                    enrollment.course_id.as_ref().and_then(|course_id| {
                        course_map.get(course_id).cloned()
                    })
                })
                .collect();
                
            StudentWithCourses {
                student,
                courses: student_courses,
            }
        })
        .collect();
        
    Ok(result)
}
```

使用这些工具宏提供了几个优势：
1. **更简洁的代码** - 减少提取和映射数据的模板代码
2. **类型安全** - 保持Rust的强类型特性
3. **高效性** - 预分配集合的优化操作
4. **可读性** - 使数据转换的意图更清晰
5. **更符合惯用法** - 利用Rbatis的内置工具进行常见操作

## 8.5 Rbatis数据类型 (rbatis::rbdc::types)

Rbatis在`rbatis::rbdc::types`模块中提供了一系列专用数据类型，用于更好地实现数据库集成和互操作性。这些类型处理Rust原生类型与数据库特定数据格式之间的转换。正确理解和使用这些类型对于正确处理数据至关重要，特别是在处理所有权和转换方法方面。

### 8.5.1 Decimal类型

`Decimal`类型表示任意精度的十进制数，特别适用于金融应用。

```rust
use rbatis::rbdc::types::Decimal;
use std::str::FromStr;

// 创建Decimal实例
let d1 = Decimal::from(100i32); // 从整数创建（注意：使用`from`而不是`from_i32`）
let d2 = Decimal::from_str("123.45").unwrap(); // 从字符串创建
let d3 = Decimal::new("67.89").unwrap(); // 另一种从字符串创建的方式
let d4 = Decimal::from_f64(12.34).unwrap(); // 从f64创建（返回Option<Decimal>）

// ❌ 错误用法 - 这些将不会工作
// let wrong1 = Decimal::from_i32(100); // 错误：方法不存在
// let mut wrong2 = Decimal::from(0); wrong2 = wrong2 + 1; // 错误：使用了已移动的值

// ✅ 正确的所有权处理
let decimal1 = Decimal::from(10i32);
let decimal2 = Decimal::from(20i32);
let sum = decimal1.clone() + decimal2; // 需要clone()，因为操作会消耗值

// 四舍五入和小数位操作
let amount = Decimal::from_str("123.456789").unwrap();
let rounded = amount.clone().round(2); // 四舍五入到2位小数：123.46
let scaled = amount.with_scale(3); // 设置3位小数：123.457

// 转换为原始类型
let as_f64 = amount.to_f64().unwrap_or(0.0);
let as_i64 = amount.to_i64().unwrap_or(0);
```

**关于Decimal的重要说明：**
- `Decimal`是对`bigdecimal`包中`BigDecimal`类型的封装
- 它没有实现`Copy`特性，只实现了`Clone`
- 大多数操作会消耗值，所以你可能需要使用`clone()`
- 使用`Decimal::from(i32)`而不是不存在的`from_i32`方法
- 始终处理转换函数返回的`Option`或`Result`

### 8.5.2 DateTime类型

`DateTime`类型处理带有时区信息的日期和时间值。

```rust
use rbatis::rbdc::types::DateTime;
use std::str::FromStr;
use std::time::Duration;

// 创建DateTime实例
let now = DateTime::now(); // 当前本地时间
let utc = DateTime::utc(); // 当前UTC时间
let dt1 = DateTime::from_str("2023-12-25 13:45:30").unwrap(); // 从字符串创建
let dt2 = DateTime::from_timestamp(1640430000); // 从Unix时间戳（秒）创建
let dt3 = DateTime::from_timestamp_millis(1640430000000); // 从毫秒创建

// 格式化
let formatted = dt1.format("%Y-%m-%d %H:%M:%S"); // "2023-12-25 13:45:30"
let iso_format = dt1.to_string(); // ISO 8601格式

// 日期/时间组件
let year = dt1.year();
let month = dt1.mon();
let day = dt1.day();
let hour = dt1.hour();
let minute = dt1.minute();
let second = dt1.sec();

// 操作DateTime
let tomorrow = now.clone().add(Duration::from_secs(86400));
let yesterday = now.clone().sub(Duration::from_secs(86400));
let later = now.clone().add_sub_sec(3600); // 增加1小时

// 比较
if dt1.before(&dt2) {
    println!("dt1比dt2早");
}

// 转换为时间戳
let ts_secs = dt1.unix_timestamp(); // 自Unix纪元以来的秒数
let ts_millis = dt1.unix_timestamp_millis(); // 毫秒
let ts_micros = dt1.unix_timestamp_micros(); // 微秒
```

### 8.5.3 Json类型

`Json`类型帮助管理数据库中的JSON数据，特别是对于具有JSON类型的列。

```rust
use rbatis::rbdc::types::{Json, JsonV};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// 基本JSON字符串处理
let json_str = r#"{"name":"张三","age":30}"#;
let json = Json::from_str(json_str).unwrap();
println!("{}", json); // 打印JSON字符串

// 从serde_json值创建
let serde_value = serde_json::json!({"status": "success", "code": 200});
let json2 = Json::from(serde_value);

// 对于结构化数据，使用JsonV
#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
    id: Option<String>,
    name: String,
    age: i32,
}

// 使用结构化数据创建JsonV
let user = User {
    id: Some("1".to_string()),
    name: "张三".to_string(),
    age: 25,
};
let json_v = JsonV(user);

// 在实体定义中使用JSON列
#[derive(Clone, Debug, Serialize, Deserialize)]
struct UserProfile {
    id: Option<String>,
    // 对JSON字段使用deserialize_with
    #[serde(deserialize_with = "rbatis::rbdc::types::deserialize_maybe_str")]
    settings: User,
}
```

### 8.5.4 Date、Time和Timestamp类型

Rbatis提供了专门的类型用于处理日期、时间和时间戳数据。

```rust
use rbatis::rbdc::types::{Date, Time, Timestamp};
use std::str::FromStr;

// Date类型（仅日期）
let today = Date::now();
let christmas = Date::from_str("2023-12-25").unwrap();
println!("{}", christmas); // "2023-12-25"

// Time类型（仅时间）
let current_time = Time::now();
let noon = Time::from_str("12:00:00").unwrap();
println!("{}", noon); // "12:00:00"

// Timestamp类型（Unix时间戳）
let ts = Timestamp::now();
let custom_ts = Timestamp::from(1640430000);
println!("{}", custom_ts); // 以秒为单位的Unix时间戳
```

### 8.5.5 Bytes和UUID类型

对于二进制数据和UUID，Rbatis提供了以下类型：

```rust
use rbatis::rbdc::types::{Bytes, Uuid};
use std::str::FromStr;

// 用于二进制数据的Bytes
let data = vec![1, 2, 3, 4, 5];
let bytes = Bytes::from(data.clone());
let bytes2 = Bytes::new(data);
println!("长度: {}", bytes.len());

// UUID
let uuid = Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
let new_uuid = Uuid::random(); // 生成新的随机UUID
println!("{}", uuid); // "550e8400-e29b-41d4-a716-446655440000"
```

### 8.5.6 使用Rbatis数据类型的最佳实践

1. **正确处理所有权**：大多数Rbatis类型没有实现`Copy`，所以要注意所有权并在需要时使用`clone()`。

2. **使用正确的创建方法**：注意可用的构造方法。例如，使用`Decimal::from(123)`而不是不存在的`Decimal::from_i32(123)`。

3. **错误处理**：大多数转换和解析方法返回`Result`或`Option`，始终正确处理这些结果。

4. **数据持久化**：为数据库表定义结构体时，对可空字段使用`Option<T>`。

5. **类型转换**：了解从数据库读取时发生的自动类型转换。为你的数据库模式使用适当的Rbatis类型。

6. **测试边界情况**：使用边界情况测试你的代码，例如`Decimal`的极大数字或`DateTime`的极端日期。

```rust
// 使用Rbatis类型的设计良好的实体示例
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<String>,
    pub user_id: String,
    pub amount: rbatis::rbdc::types::Decimal,
    pub timestamp: rbatis::rbdc::types::DateTime,
    pub notes: Option<String>,
    #[serde(deserialize_with = "rbatis::rbdc::types::deserialize_maybe_str")]
    pub metadata: UserMetadata,
}

// 在函数中正确使用
async fn record_transaction(rb: &dyn rbatis::executor::Executor, user_id: &str, amount_str: &str) -> Result<(), Error> {
    let transaction = Transaction {
        id: None,
        user_id: user_id.to_string(),
        amount: rbatis::rbdc::types::Decimal::from_str(amount_str)?,
        timestamp: rbatis::rbdc::types::DateTime::now(),
        notes: None,
        metadata: UserMetadata::default(),
    };
    
    transaction.insert(rb).await?;
    Ok(())
}
```
