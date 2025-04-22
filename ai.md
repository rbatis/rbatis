# Rbatis Framework User Guide

> This documentation is based on Rbatis 4.5+ and provides detailed instructions for using the Rbatis ORM framework. Rbatis is a high-performance Rust asynchronous ORM framework that supports multiple databases and provides compile-time dynamic SQL capabilities similar to MyBatis.

## Important Version Notes and Best Practices

Rbatis 4.5+ has significant improvements over previous versions. Here are the key changes and recommended best practices:

1. **✅ Use macros instead of traits (v4.0+)**: 
   ```rust
   // ❌ Old approach (pre-v4.0):
   impl CRUDTable for User { ... }
   
   // ✅ New approach (v4.0+):
   crud!(User {});  // Generate all CRUD methods
   impl_select!(User {select_by_name(name: &str) -> Vec => "..."});  // Custom methods
   ```

2. **✅ Place documentation comments ABOVE macros (v4.0+)**:
   ```rust
   // ❌ INCORRECT - will cause compilation errors
   impl_select!(User {
       /// This comment inside causes errors
       find_by_name(name: &str) -> Vec => "..."
   });
   
   // ✅ CORRECT
   /// Find users by name
   impl_select!(User {find_by_name(name: &str) -> Vec => "..."});
   ```
   
3. **✅ Use Rust-style logical operators (v4.0+)**:
   ```html
   // ❌ INCORRECT - MyBatis style operators will fail
   <if test="name != null and name != ''">
   
   // ✅ CORRECT - Use Rust operators
   <if test="name != null && name != ''">
   ```

4. **✅ Use backticks for SQL fragments (v4.0+)**:
   ```html
   // ❌ INCORRECT - spaces might be lost
   <if test="status != null">
       and status = #{status}
   </if>
   
   // ✅ CORRECT - backticks preserve spaces
   <if test="status != null">
       ` and status = #{status} `
   </if>
   ```

5. **✅ Use lowercase SQL keywords (all versions)**:
   ```html
   // ❌ INCORRECT
   SELECT * FROM users
   
   // ✅ CORRECT
   select * from users
   ```

6. **✅ Prefer separate queries over complex JOINs (v4.0+)**:
   ```rust
   // ❌ DISCOURAGED - Complex JOIN
   let users_with_orders = rb.query("select u.*, o.* from users u join orders o on u.id = o.user_id", vec![]).await?;
   
   // ✅ RECOMMENDED - Separate efficient queries with in-memory joining
   let users = User::select_all(&rb).await?;
   let user_ids = table_field_vec!(users, id);
   let orders = Order::select_in_column(&rb, "user_id", &user_ids).await?;
   // Then combine in memory
   ```

## 1. Introduction to Rbatis

Rbatis is an ORM (Object-Relational Mapping) framework written in Rust that provides rich database operation functionality. It supports multiple database types, including but not limited to MySQL, PostgreSQL, SQLite, MS SQL Server, and more.

Rbatis draws inspiration from Java's MyBatis framework but has been optimized and adjusted for Rust language features. As a modern ORM framework, it leverages Rust's compile-time capabilities to complete SQL parsing and code generation at compile time, providing zero-cost dynamic SQL capabilities.

### 1.1 Key Features

Rbatis offers the following key features:

- **Zero runtime overhead dynamic SQL**: Implements dynamic SQL using compile-time techniques (proc-macro, Cow) without a runtime parsing engine
- **JDBC-like driver design**: Drivers are separated through cargo dependencies and `Box<dyn Driver>` implementation
- **Multiple database support**: All database drivers support `#{arg}`, `${arg}`, and `?` placeholders (pg/mssql automatically converts `?` to `$1` and `@P1`)
- **Dynamic SQL syntax**: Supports py_sql query language and html_sql (inspired by MyBatis)
- **Dynamic connection pool configuration**: Implements high-performance connection pools based on fast_pool
- **Log support based on interceptors**
- **100% pure Rust implementation**: Enables `#![forbid(unsafe_code)]` to ensure safety

### 1.2 Supported Database Drivers

Rbatis supports any driver that implements the `rbdc` interface. The following are officially supported drivers:

| Database Type | crates.io Package | Related Link |
|--------------|------------------|-------------|
| MySQL | rbdc-mysql | github.com/rbatis/rbatis/tree/master/rbdc-mysql |
| PostgreSQL | rbdc-pg | github.com/rbatis/rbatis/tree/master/rbdc-pg |
| SQLite | rbdc-sqlite | github.com/rbatis/rbatis/tree/master/rbdc-sqlite |
| MSSQL | rbdc-mssql | github.com/rbatis/rbatis/tree/master/rbdc-mssql |
| MariaDB | rbdc-mysql | github.com/rbatis/rbatis/tree/master/rbdc-mysql |
| TiDB | rbdc-mysql | github.com/rbatis/rbatis/tree/master/rbdc-mysql |
| CockroachDB | rbdc-pg | github.com/rbatis/rbatis/tree/master/rbdc-pg |
| Oracle | rbdc-oracle | github.com/chenpengfan/rbdc-oracle |
| TDengine | rbdc-tdengine | github.com/tdcare/rbdc-tdengine |

## 2. Core Concepts

1. **RBatis Structure**：The framework's main entry point, responsible for managing database connection pools, interceptors, etc.
2. **Executor**：The interface for executing SQL operations, including RBatisConnExecutor (connection executor) and RBatisTxExecutor (transaction executor)
3. **CRUD Operations**：Provides basic CRUD operation macros and functions
4. **Dynamic SQL**：Supports HTML and Python-style SQL templates, which can dynamically build SQL statements based on conditions
5. **Interceptors**：Can intercept and modify SQL execution process, such as logging, paging, etc.

## 3. Installation and Dependency Configuration

Add the following dependencies in Cargo.toml:

```toml
[dependencies]
rbatis = "4.5"
rbs = "4.5"
# Choose a database driver
rbdc-sqlite = "4.5" # SQLite driver
# rbdc-mysql = "4.5" # MySQL driver
# rbdc-pg = "4.5" # PostgreSQL driver
# rbdc-mssql = "4.5" # MS SQL Server driver

# Asynchronous runtime
tokio = { version = "1", features = ["full"] }
# Serialization support
serde = { version = "1", features = ["derive"] }
```

Rbatis is an asynchronous framework that needs to be used with tokio and other asynchronous runtimes. It uses serde for data serialization and deserialization operations.

### 3.1 Configuring TLS Support

If TLS support is needed, you can use the following configuration:

```toml
rbs = { version = "4.5" }
rbdc-sqlite = { version = "4.5", default-features = false, features = ["tls-native-tls"] }
# rbdc-mysql = { version = "4.5", default-features = false, features = ["tls-native-tls"] }
# rbdc-pg = { version = "4.5", default-features = false, features = ["tls-native-tls"] }
# rbdc-mssql = { version = "4.5", default-features = false, features = ["tls-native-tls"] }
rbatis = { version = "4.5" }
```

## 4. Basic Usage Flow

### 4.1 Creating RBatis Instance and Initializing Database Connection

```rust
use rbatis::RBatis;

#[tokio::main]
async fn main() {
    // Create RBatis instance
    let rb = RBatis::new();
    
    // Method 1: Initialize database driver but not establish connection (using init method)
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://database.db").unwrap();
    
    // Method 2: Initialize driver and attempt to establish connection (recommended, using link method)
    rb.link(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://database.db").await.unwrap();
    
    // Other database examples:
    // MySQL
    // rb.link(rbdc_mysql::driver::MysqlDriver{}, "mysql://root:123456@localhost:3306/test").await.unwrap();
    // PostgreSQL
    // rb.link(rbdc_pg::driver::PgDriver{}, "postgres://postgres:123456@localhost:5432/postgres").await.unwrap();
    // MSSQL/SQL Server
    // rb.link(rbdc_mssql::driver::MssqlDriver{}, "jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=test").await.unwrap();
    
    println!("Database connection successful!");
}
```

> **init method and link method differences**:
> - `init()`: Only sets the database driver, does not actually connect to the database
> - `link()`: Sets the driver and immediately attempts to connect to the database, recommended to use this method to ensure connection is available

### 4.2 Defining Data Model

Data model is a Rust structure mapped to a database table:

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

// Note: In Rbatis 4.5+, using the crud! macro is the standard approach
// The CRUDTable trait no longer exists in current versions.
// Use the following macros to generate CRUD methods:

// Generate CRUD methods for the User struct
crud!(User {});
// Or specify a custom table name
// crud!(User {}, "users");
```

### 4.3 Custom Table Name

Rbatis allows customizing table name in multiple ways:

```rust
// Method 1: Specify table name through crud macro parameters
rbatis::crud!(BizActivity {}, "biz_activity"); // Custom table name biz_activity

// Method 2: Specify table name through impl_* macro's last parameter
rbatis::impl_select!(BizActivity{select_by_id(id:String) -> Option => "` where id = #{id} limit 1 `"}, "biz_activity");

// Method 3: Specify table name dynamically through function parameters
rbatis::impl_select!(BizActivity{select_by_id2(table_name:&str,id:String) -> Option => "` where id = #{id} limit 1 `"});
```

Similarly, you can customize table column name:

```rust
// Specify table column dynamically through function parameters
rbatis::impl_select!(BizActivity{select_by_id(table_name:&str,table_column:&str,id:String) -> Option => "` where id = #{id} limit 1 `"});
```

## 5. CRUD Operations

Rbatis provides multiple ways to execute CRUD (Create, Read, Update, Delete) operations.

> **Note**: Rbatis processing requires SQL keywords to be in lowercase form (select, insert, update, delete, etc.), which may differ from some SQL style guidelines. When using Rbatis, always use lowercase SQL keywords to ensure correct parsing and execution.

### 5.1 Using CRUD Macro

The simplest way is to use `crud!` macro:

```rust
use rbatis::crud;

// Automatically generate CRUD methods for User structure
// If a table name is specified, it uses the specified table name; otherwise, it uses the snake case naming method of the structure name as the table name
crud!(User {}); // Table name user
// Or
crud!(User {}, "users"); // Table name users
```

This will generate the following methods for the User structure:
- `User::insert`: Insert single record
- `User::insert_batch`: Batch insert records
- `User::update_by_column`: Update record based on specified column
- `User::update_by_column_batch`: Batch update records
- `User::delete_by_column`: Delete record based on specified column
- `User::delete_in_column`: Delete record where column value is in specified collection
- `User::select_by_column`: Query records based on specified column
- `User::select_in_column`: Query records where column value is in specified collection
- `User::select_all`: Query all records
- `User::select_by_map`: Query records based on mapping conditions

### 5.1.1 Detailed CRUD Macro Reference

The `crud!` macro automatically generates a complete set of CRUD (Create, Read, Update, Delete) operations for your data model. Under the hood, it expands to call these four implementation macros:

```rust
// Equivalent to 
impl_insert!(User {});
impl_select!(User {});
impl_update!(User {});
impl_delete!(User {});
```

#### Generated Methods

When you use `crud!(User {})`, the following methods are generated:

##### Insert Methods
- **`async fn insert(executor: &dyn Executor, table: &User) -> Result<ExecResult, Error>`**  
  Inserts a single record.
  
- **`async fn insert_batch(executor: &dyn Executor, tables: &[User], batch_size: u64) -> Result<ExecResult, Error>`**  
  Inserts multiple records with batch processing. The `batch_size` parameter controls how many records are inserted in each batch operation.

##### Select Methods
- **`async fn select_all(executor: &dyn Executor) -> Result<Vec<User>, Error>`**  
  Retrieves all records from the table.
  
- **`async fn select_by_column<V: Serialize>(executor: &dyn Executor, column: &str, column_value: V) -> Result<Vec<User>, Error>`**  
  Retrieves records where the specified column equals the given value.
  
- **`async fn select_by_map(executor: &dyn Executor, condition: rbs::Value) -> Result<Vec<User>, Error>`**  
  Retrieves records matching a map of column-value conditions (AND logic).
  
- **`async fn select_in_column<V: Serialize>(executor: &dyn Executor, column: &str, column_values: &[V]) -> Result<Vec<User>, Error>`**  
  Retrieves records where the specified column's value is in the given list of values (IN operator).

##### Update Methods
- **`async fn update_by_column(executor: &dyn Executor, table: &User, column: &str) -> Result<ExecResult, Error>`**  
  Updates a record based on the specified column (used as a WHERE condition). Null values are skipped.
  
- **`async fn update_by_column_batch(executor: &dyn Executor, tables: &[User], column: &str, batch_size: u64) -> Result<ExecResult, Error>`**  
  Updates multiple records in batches, using the specified column as the condition.
  
- **`async fn update_by_column_skip(executor: &dyn Executor, table: &User, column: &str, skip_null: bool) -> Result<ExecResult, Error>`**  
  Updates a record with control over whether null values should be skipped.
  
- **`async fn update_by_map(executor: &dyn Executor, table: &User, condition: rbs::Value, skip_null: bool) -> Result<ExecResult, Error>`**  
  Updates records matching a map of column-value conditions.

##### Delete Methods
- **`async fn delete_by_column<V: Serialize>(executor: &dyn Executor, column: &str, column_value: V) -> Result<ExecResult, Error>`**  
  Deletes records where the specified column equals the given value.
  
- **`async fn delete_by_map(executor: &dyn Executor, condition: rbs::Value) -> Result<ExecResult, Error>`**  
  Deletes records matching a map of column-value conditions.
  
- **`async fn delete_in_column<V: Serialize>(executor: &dyn Executor, column: &str, column_values: &[V]) -> Result<ExecResult, Error>`**  
  Deletes records where the specified column's value is in the given list (IN operator).
  
- **`async fn delete_by_column_batch<V: Serialize>(executor: &dyn Executor, column: &str, values: &[V], batch_size: u64) -> Result<ExecResult, Error>`**  
  Deletes multiple records in batches, based on specified column values.

#### Example Usage

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize RBatis
    let rb = RBatis::new();
    rb.link(SqliteDriver {}, "sqlite://test.db").await?;
    
    // Insert a single record
    let user = User {
        id: Some("1".to_string()),
        username: Some("john_doe".to_string()),
        // other fields...
    };
    User::insert(&rb, &user).await?;
    
    // Batch insert multiple records
    let users = vec![user1, user2, user3];
    User::insert_batch(&rb, &users, 100).await?;
    
    // Select by column
    let active_users: Vec<User> = User::select_by_column(&rb, "status", 1).await?;
    
    // Select with IN clause
    let specific_users = User::select_in_column(&rb, "id", &["1", "2", "3"]).await?;
    
    // Update a record
    let mut user_to_update = active_users[0].clone();
    user_to_update.status = Some(2);
    User::update_by_column(&rb, &user_to_update, "id").await?;
    
    // Delete a record
    User::delete_by_column(&rb, "id", "1").await?;
    
    // Delete multiple records with IN clause
    User::delete_in_column(&rb, "status", &[0, -1]).await?;
    
    Ok(())
}
```

### 5.2 CRUD Operation Example

```rust
#[tokio::main]
async fn main() {
    // Initialize RBatis and database connection...
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://test.db").unwrap();
    
    // Create user instance
    let user = User {
        id: Some("1".to_string()),
        username: Some("test_user".to_string()),
        password: Some("password".to_string()),
        create_time: Some(DateTime::now()),
        status: Some(1),
    };
    
    // Insert data
    let result = User::insert(&rb, &user).await.unwrap();
    println!("Inserted record count: {}", result.rows_affected);
    
    // Query data
    let users: Vec<User> = User::select_by_column(&rb, "id", "1").await.unwrap();
    println!("Query user: {:?}", users);
    
    // Update data
    let mut user_to_update = users[0].clone();
    user_to_update.username = Some("updated_user".to_string());
    User::update_by_column(&rb, &user_to_update, "id").await.unwrap();
    
    // Delete data
    User::delete_by_column(&rb, "id", "1").await.unwrap();
}
```

## 6. Dynamic SQL

Rbatis supports dynamic SQL, which can dynamically build SQL statements based on conditions. Rbatis provides two styles of dynamic SQL: HTML style and Python style.

> ⚠️ **IMPORTANT WARNING**
> 
> When using Rbatis XML format, do NOT use MyBatis-style `BaseResultMap` or `Base_Column_List`!
> 
> Unlike MyBatis, Rbatis does not require or support:
> - `<result id="BaseResultMap" column="id,name,status"/>`
> - `<sql id="Base_Column_List">id,name,status</sql>`
> 
> Rbatis automatically maps database columns to Rust struct fields, so these constructs are unnecessary and may cause errors. Always write complete SQL statements with explicit column selections or use `SELECT *`.

### 6.1 HTML Style Dynamic SQL

HTML style dynamic SQL uses similar XML tag syntax:

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
    impled!() // Special marker, will be replaced by actual implementation by rbatis macro processor
}
```

#### 6.1.1 Valid XML Structure

When using HTML/XML style in Rbatis, it's important to follow the correct structure defined in the DTD:

```
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" 
"https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
```

**Important Notes:**

1. **Valid top-level elements**: The `<mapper>` element can only contain: `<sql>`, `<insert>`, `<update>`, `<delete>`, or `<select>` elements.

2. **Do not use BaseResultMap**: Unlike MyBatis, Rbatis doesn't use `<resultMap>` or `BaseResultMap`. Rbatis automatically maps columns to struct fields.

3. **Always use actual SQL queries**: Instead of using column lists, directly write SQL queries.

❌ **INCORRECT** (Do not use):
```html
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "...">
<mapper>
    <!-- Incorrect: result is not a valid direct child of mapper -->
    <result id="BaseResultMap" column="id,name,status"/>
    <!-- Incorrect: column lists should be in SQL -->
    <sql id="Base_Column_List">id,name,status</sql>
</mapper>
```

✅ **CORRECT** (Use this approach):
```html
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "...">
<mapper>
    <!-- Correct: use select with direct SQL -->
    <select id="select_by_id">
        select * from user where id = #{id}
    </select>
    
    <!-- Correct: sql can be used for SQL fragments -->
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

#### 6.1.2 Space Handling Mechanism

In HTML style dynamic SQL, **backticks (`) are the key to handling spaces**:

- **Default trims spaces**: Non-backtick-enclosed text nodes will automatically remove leading and trailing spaces
- **Backticks preserve original text**: Text enclosed in backticks(`) will preserve all spaces and newlines
- **Must use backticks**: Dynamic SQL fragments must be enclosed in backticks, otherwise leading spaces and newlines will be ignored
- **Complete enclosure**: Backticks should enclose the entire SQL fragment, not just the beginning part

Incorrect use of backticks example:
```html
<if test="status != null">
    and status = #{status} <!-- No backticks, leading spaces will be lost -->
</if>

<if test="type != null">
    ` and type = #{type} ` <!-- Only beginning has backticks, not complete -->
</if>
```

Correct use of backticks example:
```html
<if test="status != null">
    ` and status = #{status} ` <!-- Complete enclosure, all spaces preserved -->
</if>

<if test="items != null && items.len > 0">
    ` and item_id in `
    <foreach collection="items" item="item" open="(" close=")" separator=",">
        #{item}
    </foreach>
</if>
```

#### 6.1.2 Differences from MyBatis

Rbatis' HTML style has several key differences from MyBatis:

1. **No need for CDATA**: Rbatis does not need to use CDATA blocks to escape special characters
   ```html
   <!-- MyBatis needs CDATA -->
   <if test="age > 18">
       <![CDATA[ AND age > 18 ]]>
   </if>

   <!-- Rbatis directly uses -->
   <if test="age > 18">
       ` and age > 18 `
   </if>
   ```

2. **Expression Syntax**: Rbatis uses Rust style expression syntax
   ```html
   <!-- MyBatis -->
   <if test="list != null and list.size() > 0">

   <!-- Rbatis -->
   <if test="list != null && list.len > 0">
   ```

3. **Special Tag Attributes**: Rbatis' foreach tag attributes are slightly different from MyBatis

HTML style supports the following tags:
- `<if>`: Conditional judgment
- `<choose>`, `<when>`, `<otherwise>`: Multi-condition selection
- `<trim>`: Remove prefix or suffix
- `<foreach>`: Loop processing
- `<where>`: Automatically handle WHERE clause
- `<set>`: Automatically handle SET clause

### 6.2 Python Style Dynamic SQL

Python style dynamic SQL uses similar Python syntax:

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

> **Note**: Rbatis requires SQL keywords to be in lowercase form. In the above example, lowercase `select`, `where`, etc. keywords are used, which is the recommended practice.

#### 6.2.1 Python Style Space Handling

Python style dynamic SQL space handling rules:

- **Indentation sensitive**: Indentation is used to identify code blocks, must be consistent
- **Line head detection**: Line head character detection is used to determine statement type
- **Backtick rules**: Same as HTML style, used to preserve spaces
- **Code block convention**: Each control statement code block must be indented

Special note:
```rust
# Incorrect: inconsistent indentation
if name != None:
    ` and name = #{name}`
  ` and status = 1`  # Incorrect indentation, will cause syntax error

# Correct: consistent indentation
if name != None:
    ` and name = #{name} `
    ` and status = 1 `  # Same indentation as previous line
```

#### 6.2.2 Python Style Supported Syntax

Python style provides the following syntax structures:

1. **if condition statement**:
   ```rust
   if condition:
       ` SQL fragment `
   ```
   Note: Python style only supports a single `if` statement, no `elif` or `else` branches.

2. **for loop**:
   ```rust
   for item in collection:
       ` SQL fragment `
   ```

3. **choose/when/otherwise**: Use specific syntax structures instead of `if/elif/else`
   ```rust
   choose:
       when condition1:
           ` SQL fragment1 `
       when condition2:
           ` SQL fragment2 `
       otherwise:
           ` Default SQL fragment `
   ```

4. **trim, where, set**: Special syntax structures
   ```rust
   trim "AND|OR":
       ` and id = 1 `
       ` or id = 2 `
   ```

5. **break and continue**: Can be used for loop control
   ```rust
   for item in items:
       if item.id == 0:
           continue
       if item.id > 10:
           break
       ` process item #{item.id} `
   ```

6. **bind variable**: Declare local variable
   ```rust
   bind name = "John"
   ` WHERE name = #{name} `
   ```

#### 6.2.3 Python Style Specific Features

Python style provides some specific convenient features:

1. **Built-in Functions**: Such as `len()`, `is_empty()`, `trim()`
2. **Collection Operations**: Simplify IN clause through `.sql()` and `.csv()` methods
   ```rust
   if ids != None:
       ` and id in ${ids.sql()} `  # Generate in (1,2,3) format
   ```
3. **Condition Combination**: Support complex expressions
   ```rust
   if (age > 18 and role == "vip") or level > 5:
       ` and is_adult = 1 `
   ```

### 6.3 HTML Style Specific Syntax

HTML style supports the following tags:

1. **`<if>`**：Conditional judgment
   ```html
   <if test="condition">
       SQL fragment
   </if>
   ```

2. **`<choose>/<when>/<otherwise>`**：Multi-condition selection (similar to switch statement)
   ```html
   <choose>
       <when test="condition1">
           SQL fragment1
       </when>
       <when test="condition2">
           SQL fragment2
       </when>
       <otherwise>
            Default SQL fragment
       </otherwise>
   </choose>
   ```

3. **`<trim>`**：Remove prefix or suffix
   ```html
   <trim prefixOverrides="AND|OR" suffixOverrides=",">
       SQL fragment
   </trim>
   ```

4. **`<foreach>`**：Loop processing
   ```html
   <foreach collection="items" item="item" index="index" separator=",">
       #{item}
   </foreach>
   ```

5. **`<where>`**：Automatically handle WHERE clause (will smartly remove leading AND/OR)
   ```html
   <where>
       <if test="id != null">
           and id = #{id}
       </if>
   </where>
   ```

6. **`<set>`**：Automatically handle SET clause (will smartly manage commas)
   ```html
   <set>
       <if test="name != null">
           name = #{name},
       </if>
       <if test="age != null">
           age = #{age},
       </if>
   </set>
   ```

7. **`<bind>`**：Variable binding
   ```html
   <bind name="pattern" value="'%' + name + '%'" />
   ```

Traditional MyBatis' `<elseif>` tag is not supported, instead multiple `<when>` are used to implement similar functionality.

### 6.4 Expression Engine Function

Rbatis expression engine supports multiple operators and functions:

- **Comparison Operators**: `==`, `!=`, `>`, `<`, `>=`, `<=`
- **Logical Operators**: `&&`, `||`, `!`
- **Mathematical Operators**: `+`, `-`, `*`, `/`, `%`
- **Collection Operations**: `in`, `not in`
- **Built-in Functions**:
  - `len(collection)`: Get collection length
  - `is_empty(collection)`: Check if collection is empty
  - `trim(string)`: Remove string leading and trailing spaces
  - `print(value)`: Print value (for debugging)
  - `to_string(value)`: Convert to string

Expression examples:
```rust
<if test="user.age >= 18 && (user.role == 'admin' || user.vip)">
    ` and is_adult = 1 `
</if>

<if test="!is_empty(ids)">
    ` and id in ${ids.sql()} `
</if>
```

### 6.5 Important Differences Between Rbatis and MyBatis Expression Syntax

While Rbatis draws inspiration from MyBatis, there are **critical differences** in the expression syntax that must be understood to avoid compilation errors:

#### 1. Logical Operators: Use `&&` and `||`, NOT `and` and `or`

```html
<!-- ✅ CORRECT: Using Rbatis operators -->
<if test="name != null && name != ''">
    ` and name = #{name}`
</if>

<!-- ❌ WRONG: Using MyBatis style operators -->
<if test="name != null and name != ''">  <!-- This will cause compilation errors! -->
    ` and name = #{name}`
</if>
```

Rbatis directly translates expressions to Rust code, where logical operators are `&&` and `||`. The keywords `and` and `or` are not recognized by the Rbatis expression engine.

#### 2. Null Comparison: Use `== null` or `!= null`

```html
<!-- ✅ CORRECT -->
<if test="user != null && user.name != null">
    ` and user_name = #{user.name}`
</if>

<!-- Another correct approach for null checks -->
<if test="user == null || user.name == null">
    ` and user_name = 'Guest'`
</if>
```

#### 3. String Comparison: Use `==` and `!=` with quotes

```html
<!-- ✅ CORRECT -->
<if test="status == 'active'">
    ` and status = 1`
</if>

<!-- ✅ CORRECT: Empty string check -->
<if test="name != ''">
    ` and name like #{name}`
</if>
```

#### 4. Expression Grouping: Use parentheses for complex conditions

```html
<!-- ✅ CORRECT: Using parentheses for clarity and proper precedence -->
<if test="(age > 18 || vip == true) && status != 'banned'">
    ` and can_access = true`
</if>
```

#### 5. Collection Operations: Use appropriate functions

```html
<!-- ✅ CORRECT: Checking if a collection is empty -->
<if test="!is_empty(permissions)">
    ` and permission in ${permissions.sql()}`
</if>

<!-- ✅ CORRECT: Using collection length -->
<if test="len(items) > 0">
    ` and has_items = true`
</if>
```

### 6.5.1 Rbatis Expression Engine Internals

Understanding how Rbatis parses and processes expressions is crucial for writing correct dynamic SQL. The following details explain the internal workings of the Rbatis expression system:

#### Expression Processing Mechanism

Rbatis processes expressions in a fundamentally different way than MyBatis:

1. **Direct Rust Code Generation**: Expressions in `test` attributes are directly translated to Rust code at compile time. For example, `name != null && name != ''` is converted to Rust code that operates on the `Value` type.

2. **No Runtime OGNL Parsing**: Unlike MyBatis which uses OGNL to interpret expressions at runtime, Rbatis performs all expression parsing during compilation.

3. **Type Conversion System**: Expressions are evaluated through a system of operator overloading and type conversions implemented for the `Value` type.

```rust
// Internal conversion in test="user.age >= 18 && user.role == 'admin'"
(&arg["user"]["age"]).op_gt(&Value::from(18)) && (&arg["user"]["role"]).op_eq(&Value::from("admin"))
```

#### Expression Syntax Rules

The expression syntax in Rbatis follows these strict rules:

1. **Strict Rust-like Operators**:
   - Logical operators: `&&` (AND), `||` (OR), `!` (NOT)
   - Comparison operators: `==`, `!=`, `>`, `<`, `>=`, `<=`
   - Mathematical operators: `+`, `-`, `*`, `/`, `%`
   
2. **Path Navigation**:
   - Access object properties using dot notation: `user.name`
   - Access array elements using brackets: `items[0]`
   - Internally converted to: `arg["user"]["name"]` and `arg["items"][0]`

3. **Null Handling**:
   - Use `null` keyword for null checks: `item == null` or `item != null`
   - Empty string checks: `str == ''` or `str != ''`

4. **String Literals**:
   - Must be enclosed in single quotes: `name == 'John'`
   - Internally converted to: `arg["name"].op_eq(&Value::from("John"))`

5. **Function Calls**:
   - Functions are translated into method calls on the `Value` type
   - Example: `len(items) > 0` becomes `arg["items"].len() > 0`

#### Available Expression Functions

Rbatis provides a set of built-in functions that can be used in expressions:

```rust
// Collection functions
len(collection)           // Get length of collection
is_empty(collection)      // Check if collection is empty
contains(collection, item) // Check if collection contains item

// String functions
trim(string)              // Remove leading/trailing spaces
starts_with(string, prefix) // Check if string starts with prefix
ends_with(string, suffix)  // Check if string ends with suffix
to_string(value)          // Convert value to string

// SQL generation
value.sql()               // Generate SQL fragment (for IN clauses)
value.csv()               // Generate comma-separated value list
```

#### Expression Context and Variable Scope

In Rbatis expressions:

1. **Root Context**: All variables are accessed from the root argument context.
   
2. **Variable Binding**: The `<bind>` tag creates new variables in the context.
   ```html
   <bind name="pattern" value="'%' + name + '%'" />
   <!-- Creates a new variable 'pattern' in the context -->
   ```

3. **Loop Variables**: In `<foreach>` loops, the `item` and `index` variables are available only within the loop scope.
   ```html
   <foreach collection="items" item="item" index="index">
     <!-- 'item' and 'index' are available only here -->
   </foreach>
   ```

#### Common Expression Patterns

Here are some patterns for common expression needs in Rbatis:

```html
<!-- Null or empty string check -->
<if test="name != null && name.trim() != ''">
  ` and name = #{name}`
</if>

<!-- Collection emptiness check -->
<if test="!is_empty(ids)">
  ` and id in ${ids.sql()}`
</if>

<!-- Complex condition with grouping -->
<if test="(status == 'active' || status == 'pending') && create_time != null">
  ` and (status = #{status} or status = 'review') and create_time > #{create_time}`
</if>

<!-- Numeric comparisons -->
<if test="min_age != null && max_age != null && min_age < max_age">
  ` and age between #{min_age} and #{max_age}`
</if>

<!-- String operations -->
<if test="name != null && name.trim() != '' && starts_with(name, 'A')">
  ` and name like #{name}%`
</if>
```

#### Error Handling in Expressions

Common expression errors and how to avoid them:

1. **Type Mismatch Errors**:
   ```html
   <!-- ❌ WRONG: Type mismatch -->
   <if test="count + 'string'">
     <!-- Will cause compilation error -->
   </if>
   
   <!-- ✅ CORRECT: Consistent types -->
   <if test="to_string(count) + 'string' != ''">
     <!-- Properly converts types -->
   </if>
   ```

2. **Operator Precedence Issues**:
   ```html
   <!-- ❌ WRONG: Ambiguous precedence -->
   <if test="a && b || c">
     <!-- Might not behave as expected -->
   </if>
   
   <!-- ✅ CORRECT: Clear precedence with parentheses -->
   <if test="(a && b) || c">
     <!-- Explicit grouping -->
   </if>
   ```

3. **Null Safety**:
   ```html
   <!-- ❌ WRONG: Potential null reference -->
   <if test="user.address.city == 'New York'">
     <!-- Crashes if user or address is null -->
   </if>
   
   <!-- ✅ CORRECT: Null-safe navigation -->
   <if test="user != null && user.address != null && user.address.city == 'New York'">
     <!-- Properly checks for null before accessing properties -->
   </if>
   ```

### 6.6 Common Mistakes to Avoid

1. **Using MyBatis keywords**: Rbatis doesn't support MyBatis OGNL expressions like `and`, `or`, etc.

2. **Ignoring operator case sensitivity**: Operators in Rbatis are case-sensitive; use `&&` not `AND` or `And`.

3. **Omitting spaces**: Ensure proper spacing around operators: `a&&b` should be `a && b`.

4. **Forgetting backticks**: Wrap SQL fragments in backticks to ensure proper space handling.

5. **Using non-existent functions**: Only use functions that are explicitly supported by Rbatis.

### 6.7 Dynamic SQL Usage Example

```rust
#[tokio::main]
async fn main() {
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://test.db").unwrap();
    
    // Use HTML style dynamic SQL
    let users = select_by_condition(&rb, Some("%test%"), Some(18), "admin").await.unwrap();
    println!("Query result: {:?}", users);
    
    // Use Python style dynamic SQL
    let users = select_by_condition_py(&rb, Some("%test%"), Some(18), "admin").await.unwrap();
    println!("Query result: {:?}", users);
}
```

### 6.8 Rbatis Expression Engine Detailed Explanation

Rbatis' expression engine is the core of dynamic SQL, responsible for parsing and processing expressions at compile time, and converting to Rust code. Through in-depth understanding of the expression engine's working principles, you can more effectively utilize Rbatis' dynamic SQL capabilities.

#### 6.8.1 Expression Engine Architecture

Rbatis expression engine consists of several core components:

1. **Lexical Analyzer**: Decompose expression string into tokens
2. **Syntax Analyzer**: Build expression abstract syntax tree (AST)
3. **Code Generator**: Convert AST to Rust code
4. **Runtime Support**: Provide type conversion and operator overloading features

At compile time, Rbatis processor (such as `html_sql` and `py_sql` macros) calls expression engine to parse condition expressions and generate equivalent Rust code.

#### 6.8.2 Expression Type System

Rbatis expression engine is built around `rbs::Value` type, which is an enumeration that can represent multiple data types. Expression engine supports the following data types:

1. **Scalar Types**:
   - `Null`: Null value
   - `Bool`: Boolean value
   - `I32`/`I64`: Signed integers
   - `U32`/`U64`: Unsigned integers
   - `F32`/`F64`: Floating point numbers
   - `String`: String

2. **Composite Types**:
   - `Array`: Array/List
   - `Map`: Key-Value Mapping
   - `Binary`: Binary Data
   - `Ext`: Extended Type

All expressions ultimately compile to code operating on `Value` type, expression engine automatically performs type conversion based on context.

#### 6.8.3 Type Conversion and Operators

Rbatis expression engine implements a powerful type conversion system, allowing operations between different types:

```rust
// Source code AsProxy trait provides conversion functionality for various types
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

Expression engine overloads all standard operators, allowing them to be applied to `Value` type:

1. **Comparison Operators**:
   ```rust
   // In expression
   user.age > 18
   
   // Compile to
   (user["age"]).op_gt(&Value::from(18))
   ```

2. **Logical Operators**:
   ```rust
   // In expression
   is_admin && is_active
   
   // Compile to
   bool::op_from(is_admin) && bool::op_from(is_active)
   ```

3. **Mathematical Operators**:
   ```rust
   // In expression
   price * quantity
   
   // Compile to
   (price).op_mul(&quantity)
   ```

Different type conversions rules:
- Automatic type conversion between numerical types (e.g., i32 to f64)
- String and numerical type can be converted to each other (e.g., "123" to 123)
- NULL value comparison rules with other types

#### 6.8.4 Path Expression and Accessor

Rbatis supports accessing nested attributes of objects through dot and index:

```rust
// Dot access object attributes
user.profile.age > 18

// Array index access
items[0].price > 100

// Multi-level path
order.customer.address.city == "Beijing"
```

These expressions are converted to `Value` index operations:

```rust
// user.profile.age > 18 converted to
(&arg["user"]["profile"]["age"]).op_gt(&Value::from(18))
```

#### 6.8.5 Built-in Functions and Methods

Rbatis expression engine provides many built-in functions and methods:

1. **Collection Functions**:
   - `len(collection)`: Get collection length
   - `is_empty(collection)`: Check if collection is empty
   - `contains(collection, item)`: Check if collection contains an item

2. **String Functions**:
   - `trim(string)`: Remove string leading and trailing spaces
   - `starts_with(string, prefix)`: Check string prefix
   - `ends_with(string, suffix)`: Check string suffix
   - `to_string(value)`: Convert to string

3. **SQL Generation Methods**:
   - `value.sql()`: Generate SQL fragment, especially useful for IN clause
   - `value.csv()`: Generate comma-separated value list

```rust
// Expression uses function
if !ids.is_empty() && len(names) > 0:
    ` AND id IN ${ids.sql()} `
```

#### 6.8.6 Expression Debugging Tips

When debugging complex expressions, you can use the following tips:

1. **Print Function**:
   ```rust
   // Add print function to expression (Only valid in Python style)
   if print(user) && user.age > 18:
       ` and is_adult = 1 `
   ```

2. **Enable Detailed Logging**:
   ```rust
   fast_log::init(fast_log::Config::new().console()).unwrap();
   ```

3. **Expression Decomposition**: Decompose complex expressions into multiple simple expressions, gradually verify

#### 6.8.7 Expression Performance Considerations

1. **Compile Time Evaluation**: Rbatis expression parsing is done at compile time, does not affect runtime performance
2. **Avoid Complex Expressions**: Too complex expressions may lead to generated code bloating
3. **Use Appropriate Types**: Try to use matching data types to reduce runtime type conversion
4. **Cache Calculated Results**: For repeated expression results used, consider pre-calculating and passing to SQL function

Through in-depth understanding of Rbatis expression engine's working principles, developers can more effectively write dynamic SQL, fully utilize Rust's type safety and compile-time checks, while maintaining SQL's flexibility and expressiveness.

## 7. Transaction Management

Rbatis supports transaction management, which can execute multiple SQL operations in a transaction, either all succeed or all fail.

### 7.1 Using Transaction Executor

```rust
use rbatis::RBatis;

#[tokio::main]
async fn main() {
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://test.db").unwrap();
    
    // Get transaction executor
    let mut tx = rb.acquire_begin().await.unwrap();
    
    // Execute multiple operations in transaction
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
    
    // Insert first user
    let result1 = User::insert(&mut tx, &user1).await;
    if result1.is_err() {
        // If error, roll back transaction
        tx.rollback().await.unwrap();
        println!("Transaction rolled back: {:?}", result1.err());
        return;
    }
    
    // Insert second user
    let result2 = User::insert(&mut tx, &user2).await;
    if result2.is_err() {
        // If error, roll back transaction
        tx.rollback().await.unwrap();
        println!("Transaction rolled back: {:?}", result2.err());
        return;
    }
    
    // Commit transaction
    tx.commit().await.unwrap();
    println!("Transaction committed successfully");
}
```

## 8. Plugin and Interceptor

Rbatis provides plugin and interceptor mechanisms, which can intercept and process SQL execution process.

### 8.1 Log Interceptor

Rbatis has a built-in log interceptor by default, which can record detailed SQL execution information:

```rust
use log::LevelFilter;
use rbatis::RBatis;
use rbatis::intercept_log::LogInterceptor;

fn main() {
    // Initialize log system
    fast_log::init(fast_log::Config::new().console().level(LevelFilter::Debug)).unwrap();
    
    // Create RBatis instance
    let rb = RBatis::new();
    
    // Add custom log interceptor
    rb.intercepts.clear(); // Clear default interceptors
    rb.intercepts.push(Arc::new(LogInterceptor::new(LevelFilter::Debug)));
    
    // Subsequent operations...
}
```

### 8.2 Custom Interceptor

You can implement `Intercept` trait to create custom interceptors:

```rust
use std::sync::Arc;
use async_trait::async_trait;
use rbatis::plugin::intercept::{Intercept, InterceptContext, InterceptResult};
use rbatis::RBatis;

// Define custom interceptor
#[derive(Debug)]
struct MyInterceptor;

#[async_trait]
impl Intercept for MyInterceptor {
    async fn before(&self, ctx: &mut InterceptContext) -> Result<bool, rbatis::Error> {
        println!("Before executing SQL: {}", ctx.sql);
        // Return true to continue execution, false to interrupt execution
        Ok(true)
    }
    
    async fn after(&self, ctx: &mut InterceptContext, res: &mut InterceptResult) -> Result<bool, rbatis::Error> {
        println!("After executing SQL: {}, Result: {:?}", ctx.sql, res.return_value);
        // Return true to continue execution, false to interrupt execution
        Ok(true)
    }
}

#[tokio::main]
async fn main() {
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://test.db").unwrap();
    
    // Add custom interceptor
    rb.intercepts.push(Arc::new(MyInterceptor {}));
    
    // Subsequent operations...
}
```

### 8.3 Paging Plugin

Rbatis has a built-in paging interceptor that can automatically handle paging queries:

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
    
    // Create paging request
    let page_req = PageRequest::new(1, 10); // Page 1, 10 per page
    
    // Execute paging query
    let page_result = select_page(&rb, &page_req, Some("%test%")).await.unwrap();
    
    println!("Total record count: {}", page_result.total);
    println!("Total page count: {}", page_result.pages);
    println!("Current page: {}", page_result.page_no);
    println!("Page size: {}", page_result.page_size);
    println!("Query result: {:?}", page_result.records);
}
```

## 9. Table Synchronization and Database Management

Rbatis provides table synchronization functionality, which can automatically create or update database table structure based on structure definition.

### 9.1 Table Synchronization

```rust
use rbatis::table_sync::{SqliteTableMapper, TableSync};
use rbatis::RBatis;

#[tokio::main]
async fn main() {
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://test.db").unwrap();
    
    // Get database connection
    let conn = rb.acquire().await.unwrap();
    
    // Synchronize table structure based on User structure
    // First parameter is connection, second parameter is database specific mapper, third parameter is structure instance, fourth parameter is table name
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
    
    println!("Table synchronization completed");
}

// 替换为：

use rbatis::table_sync::SqliteTableMapper;
use rbatis::RBatis;

#[tokio::main]
async fn main() {
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://test.db").unwrap();
    
    // Get database connection
    let conn = rb.acquire().await.unwrap();
    
    // Synchronize table structure based on User structure
    // Parameters: connection, database mapper, entity instance, table name
    RBatis::sync(
        &conn,
        &SqliteTableMapper {},
        &User::default(),  // Use default() instead of creating instance with empty values
        "user",
    )
    .await
    .unwrap();
    
    println!("Table synchronization completed");
}
```

Different databases need to use different table mappers:
- SQLite: `SqliteTableMapper`
- MySQL: `MysqlTableMapper`
- PostgreSQL: `PgTableMapper`
- SQL Server: `MssqlTableMapper`

### 9.2 Table Field Mapping

You can use `table_column` and `table_id` attributes to customize field mapping:

```rust
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    #[table_id]
    pub id: Option<String>, // Primary key field
    
    #[serde(rename = "user_name")]
    #[table_column(rename = "user_name")]
    pub username: Option<String>, // Custom column name
    
    pub password: Option<String>,
    
    #[table_column(default = "CURRENT_TIMESTAMP")] // Set default value
    pub create_time: Option<DateTime>,
    
    #[table_column(comment = "User status: 1=Enabled, 0=Disabled")] // Add column comment
    pub status: Option<i32>,
    
    #[table_column(ignore)] // Ignore this field, not mapped to table
    pub temp_data: Option<String>,
}
```

## 10. Best Practices

### 10.1 Optimize Performance

- Use connection pool optimization: Reasonable configure connection pool size and timeout settings, avoid frequent creation and destruction of connections
- Batch processing: Use batch insert, update instead of loop single operation
- Lazy loading: Load related data only when needed, avoid excessive queries
- Appropriate indexing: Establish appropriate index for commonly queried fields
- Avoid N+1 problem: Use joint query instead of multiple separate queries

### 10.2 Best Practices for Error Handling

```rust
async fn handle_user_operation() -> Result<User, Error> {
    let rb = init_rbatis().await?;
    
    // Use ? operator to propagate errors
    let user = rb.query_by_column("id", "1").await?;
    
    // Use Result's combinator method to handle errors
    rb.update_by_column("id", &user).await
        .map_err(|e| {
            error!("Failed to update user information: {}", e);
            Error::from(e)
        })?;
    
    Ok(user)
}

// 替换为：

/// ✅ RECOMMENDED: Error handling patterns for Rbatis (v4.5+)
async fn handle_user_operation() -> Result<User, Error> {
    // Initialize connection
    let rb = RBatis::new();
    rb.link(SqliteDriver {}, "sqlite://test.db").await?;
    
    // Option 1: Simple propagation with ?
    let user = User::select_by_id(&rb, "1").await?;
    
    // Option 2: Custom error mapping with map_err
    User::update_by_column(&rb, &user, "id").await
        .map_err(|e| {
            log::error!("Failed to update user: {}", e);
            Error::from(format!("Database error: {}", e))
        })?;
    
    // Option 3: Transaction with error handling
    let mut tx = rb.acquire_begin().await?;
    
    let result = (|| async {
        // Multiple operations in transaction
        User::insert(&mut tx, &user).await?;
    // Create a new user
    let user = User {
        id: Some("1".to_string()),
        username: Some("test_user".to_string()),
        email: Some("test@example.com".to_string()),
        status: Some(1),
        create_time: Some(DateTime::now()),
    };
    
    // Insert the user
    User::insert(&rb, &user).await?;
    
    // Find user by username (returns Option<User>)
    let found_user = User::find_by_username(&rb, "test_user").await?;
    println!("Found user: {:?}", found_user);
    
    // Find all active users (returns Vec<User>)
    let active_users = User::find_active_users(&rb).await?;
    println!("Active users count: {}", active_users.len());
    
    // Update user status
    User::update_status(&rb, "1", 2).await?;
    
    // Paginated query (returns Page<User>)
    use rbatis::plugin::page::PageRequest;
    let page_req = PageRequest::new(1, 10);
    let user_page = User::find_by_email_page(&rb, &page_req, "%example%").await?;
    println!("Total users: {}, Current page: {}", user_page.total, user_page.page_no);
    
    // Complex query using HTML SQL
    let status_list = vec![1, 2, 3];
    let users = find_users_by_criteria(&rb, Some("test%"), None, Some(&status_list), "name").await?;
    println!("Found {} users matching criteria", users.len());
    
    // Delete by column
    User::delete_by_column(&rb, "id", "1").await?;
    
    // Delete inactive users using custom method
    User::remove_inactive(&rb).await?;
    
    Ok(())
}
```

This example shows the modern approach to using Rbatis 4.5+:
1. Define your model using `#[derive]` attributes
2. Generate basic CRUD methods using the `crud!` macro
3. Define custom queries using the appropriate `impl_*` macros
4. Use strong typing for method returns (Option, Vec, Page, etc.)
5. Use async/await for all database operations
6. For complex queries, use properly formatted HTML SQL with correct mapper structure

## 12. Handling Related Data (Associations)

When dealing with related data between tables (like one-to-many or many-to-many relationships), Rbatis recommends using `select_in_column` rather than complex JOIN queries. This approach is more efficient and maintainable in most cases.

### 12.1 The Problem with JOINs

While SQL JOINs are powerful, they can lead to several issues:
- Complex queries that are hard to maintain
- Performance problems with large datasets
- Difficulty handling nested relationships
- Mapping challenges from flat result sets to object hierarchies

### 12.2 Rbatis Approach: select_in_column

Instead of JOINs, Rbatis recommends:
1. Query the main entity first
2. Extract IDs from the main entities
3. Use `select_in_column` to fetch related entities in a batch
4. Combine the data in your service layer

This approach has several advantages:
- Better performance for large datasets
- Cleaner, more maintainable code
- Better control over exactly what data is fetched
- Avoids N+1 query problems

### 12.3 Example: One-to-Many Relationship

```rust
// Entities
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub total: Option<f64>,
    // Other fields...
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Option<String>,
    pub order_id: Option<String>,
    pub product_id: Option<String>,
    pub quantity: Option<i32>,
    pub price: Option<f64>,
    // Other fields...
}

// Generate CRUD operations
crud!(Order {});
crud!(OrderItem {});

// Custom methods for OrderItem
impl_select!(OrderItem {
    select_by_order_ids(order_ids: &[String]) -> Vec =>
        "` where order_id in ${order_ids.sql()} order by id asc`"
});

// Service layer
pub struct OrderService {
    rb: RBatis,
}

impl OrderService {
    // Get orders with their items
    pub async fn get_orders_with_items(&self, user_id: &str) -> rbatis::Result<Vec<OrderWithItems>> {
        // Step 1: Get all orders for the user
        let orders = Order::select_by_column(&self.rb, "user_id", user_id).await?;
        if orders.is_empty() {
            return Ok(vec![]);
        }
        
        // Step 2: Extract order IDs
        let order_ids: Vec<String> = orders
            .iter()
            .filter_map(|order| order.id.clone())
            .collect();
            
        // Step 3: Fetch all order items in a single query
        let items = OrderItem::select_by_order_ids(&self.rb, &order_ids).await?;
        
        // Step 4: Group items by order_id for quick lookup
        let mut items_map: HashMap<String, Vec<OrderItem>> = HashMap::new();
        for item in items {
            if let Some(order_id) = &item.order_id {
                items_map
                    .entry(order_id.clone())
                    .or_insert_with(Vec::new)
                    .push(item);
            }
        }
        
        // Step 5: Combine orders with their items
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

// Combined data structure
pub struct OrderWithItems {
    pub order: Order,
    pub items: Vec<OrderItem>,
}
```

### 12.4 Example: Many-to-Many Relationship

```rust
// Entities
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: Option<String>,
    pub name: Option<String>,
    // Other fields...
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<String>,
    pub title: Option<String>,
    // Other fields...
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentCourse {
    pub id: Option<String>,
    pub student_id: Option<String>,
    pub course_id: Option<String>,
    pub enrollment_date: Option<DateTime>,
}

// Generate CRUD operations
crud!(Student {});
crud!(Course {});
crud!(StudentCourse {});

// Custom methods
impl_select!(StudentCourse {
    select_by_student_ids(student_ids: &[String]) -> Vec =>
        "` where student_id in ${student_ids.sql()}`"
});

impl_select!(Course {
    select_by_ids(ids: &[String]) -> Vec =>
        "` where id in ${ids.sql()}`"
});

// Service layer function to get students with their courses
async fn get_students_with_courses(rb: &RBatis) -> rbatis::Result<Vec<StudentWithCourses>> {
    // Step 1: Get all students
    let students = Student::select_all(rb).await?;
    
    // Step 2: Extract student IDs
    let student_ids: Vec<String> = students
        .iter()
        .filter_map(|s| s.id.clone())
        .collect();
        
    // Step 3: Get all enrollments for these students
    let enrollments = StudentCourse::select_by_student_ids(rb, &student_ids).await?;
    
    // Step 4: Extract course IDs from enrollments
    let course_ids: Vec<String> = enrollments
        .iter()
        .filter_map(|e| e.course_id.clone())
        .collect();
        
    // Step 5: Get all courses in one query
    let courses = Course::select_by_ids(rb, &course_ids).await?;
    
    // Step 6: Create lookup maps
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
    
    // Step 7: Combine everything
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

// Combined data structure
pub struct StudentWithCourses {
    pub student: Student,
    pub courses: Vec<Course>,
}
```

By using this approach, you:
1. Avoid complex JOIN queries
2. Minimize the number of database queries (avoiding N+1 issues)
3. Keep clear separation between data access and business logic
4. Have more control over data fetching and transformation
5. Can easily handle more complex nested relationships

### 12.5 Using Rbatis Table Utility Macros for Data Joining

Rbatis provides several powerful utility macros in `table_util.rs` that can significantly simplify data processing when combining related entities. These macros are more efficient alternatives to SQL JOINs:

#### 12.5.1 Available Table Utility Macros

1. **`table_field_vec!`** - Extract a specific field from a collection into a new Vec:
   ```rust
   // Extract all role_ids from a collection of user roles
   let role_ids: Vec<String> = table_field_vec!(user_roles, role_id);
   // Using references (no cloning)
   let role_ids_ref: Vec<&String> = table_field_vec!(&user_roles, role_id);
   ```

2. **`table_field_set!`** - Extract a specific field into a HashSet (useful for unique values):
   ```rust
   // Extract unique role_ids
   let role_ids: HashSet<String> = table_field_set!(user_roles, role_id);
   // Using references
   let role_ids_ref: HashSet<&String> = table_field_set!(&user_roles, role_id);
   ```

3. **`table_field_map!`** - Create a HashMap with a specific field as the key:
   ```rust
   // Create a HashMap with role_id as key and UserRole as value
   let role_map: HashMap<String, SysUserRole> = table_field_map!(user_roles, role_id);
   ```

4. **`table_field_btree!`** - Create a BTreeMap (ordered map) with a specific field as the key:
   ```rust
   // Create a BTreeMap with role_id as key
   let role_btree: BTreeMap<String, SysUserRole> = table_field_btree!(user_roles, role_id);
   ```

5. **`table!`** - Simplify table construction by using Default trait:
   ```rust
   // Create a new instance with specific fields initialized
   let user = table!(User { id: new_snowflake_id(), name: "John".to_string() });
   ```

#### 12.5.2 Improved Example: One-to-Many Relationship

Here's how to use these utilities to simplify the one-to-many example:

```rust
// Imports
use std::collections::HashMap;
use rbatis::{table_field_vec, table_field_map};

// Service method
pub async fn get_orders_with_items(&self, user_id: &str) -> rbatis::Result<Vec<OrderWithItems>> {
    // Get all orders for the user
    let orders = Order::select_by_column(&self.rb, "user_id", user_id).await?;
    if orders.is_empty() {
        return Ok(vec![]);
    }
    
    // Extract order IDs using table_field_vec! macro - much cleaner!
    let order_ids = table_field_vec!(orders, id);
    
    // Fetch all order items in a single query
    let items = OrderItem::select_by_order_ids(&self.rb, &order_ids).await?;
    
    // Group items by order_id using table_field_map! - automatic grouping!
    let mut items_map: HashMap<String, Vec<OrderItem>> = HashMap::new();
    for (order_id, item) in table_field_map!(items, order_id) {
        items_map.entry(order_id).or_insert_with(Vec::new).push(item);
    }
    
    // Map orders to result
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

#### 12.5.3 Simplified Many-to-Many Example

For many-to-many relationships, these utilities also simplify the code:

```rust
// Imports
use std::collections::{HashMap, HashSet};
use rbatis::{table_field_vec, table_field_set, table_field_map};

// Service function for many-to-many
async fn get_students_with_courses(rb: &RBatis) -> rbatis::Result<Vec<StudentWithCourses>> {
    // Get all students
    let students = Student::select_all(rb).await?;
    
    // Extract student IDs using the utility macro
    let student_ids = table_field_vec!(students, id);
    
    // Get enrollments for these students
    let enrollments = StudentCourse::select_by_student_ids(rb, &student_ids).await?;
    
    // Extract unique course IDs using set (removes duplicates automatically)
    let course_ids = table_field_set!(enrollments, course_id);
    
    // Get all courses in one query
    let courses = Course::select_by_ids(rb, &course_ids.into_iter().collect::<Vec<_>>()).await?;
    
    // Create lookup maps using utility macros
    let course_map = table_field_map!(courses, id);
    
    // Create a student -> enrollments map
    let mut student_enrollments: HashMap<String, Vec<StudentCourse>> = HashMap::new();
    for enrollment in enrollments {
        if let Some(student_id) = &enrollment.student_id {
            student_enrollments
                .entry(student_id.clone())
                .or_insert_with(Vec::new)
                .push(enrollment);
        }
    }
    
    // Build the result
    let result = students
        .into_iter()
        .map(|student| {
            let student_id = student.id.clone().unwrap_or_default();
            let enrollments = student_enrollments.get(&student_id).cloned().unwrap_or_default();
            
            // Map enrollments to courses
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

Using these utility macros provides several advantages:
1. **Cleaner code** - Reduces boilerplate for extracting and mapping data
2. **Type safety** - Maintains Rust's strong typing
3. **Efficiency** - Optimized operations with pre-allocated collections
4. **Readability** - Makes the intent of data transformations clear
5. **More idiomatic** - Leverages Rbatis' built-in tools for common operations

# 12. Summary

Rbatis is a powerful and flexible ORM framework that is suitable for multiple database types. It provides rich dynamic SQL capabilities, supports multiple parameter binding methods, and provides plugin and interceptor mechanisms. Rbatis' expression engine is the core of dynamic SQL, responsible for parsing and processing expressions at compile time, and converting to Rust code. Through in-depth understanding of Rbatis' working principles, developers can more effectively write dynamic SQL, fully utilize Rust's type safety and compile-time checks, while maintaining SQL's flexibility and expressiveness.

Following best practices can fully leverage Rbatis framework advantages to build efficient, reliable database applications.

### Important Coding Specifications

1. **Use lowercase SQL keywords**: Rbatis processing mechanism is based on lowercase SQL keywords, all SQL statements must use lowercase form of `select`, `insert`, `update`, `delete`, `where`, `from`, `order by`, etc., do not use uppercase form.
2. **Correct space handling**: Use backticks (`) to enclose SQL fragments to preserve leading spaces.
3. **Type safety**: Fully utilize Rust's type system, use `Option<T>` to handle nullable fields.
4. **Follow asynchronous programming model**: Rbatis is asynchronous ORM, all database operations should use `.await` to wait for completion.

# 3.5 ID Generation

Rbatis provides built-in ID generation mechanisms that are recommended for primary keys in your database tables. Using these mechanisms ensures globally unique IDs and better performance for distributed systems.

## 3.5.1 SnowflakeId

SnowflakeId is a distributed ID generation algorithm originally developed by Twitter. It generates 64-bit IDs that are composed of:
- Timestamp
- Machine ID
- Sequence number

```rust
use rbatis::snowflake::new_snowflake_id;

// In your model definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    // Use i64 for snowflake IDs
    pub id: Option<i64>,
    pub username: Option<String>,
    // other fields...
}

// When creating a new record
async fn create_user(rb: &RBatis, username: &str) -> rbatis::Result<User> {
    let mut user = User {
        id: Some(new_snowflake_id()), // Generate a new snowflake ID
        username: Some(username.to_string()),
        // initialize other fields...
    };
    
    User::insert(rb, &user).await?;
    Ok(user)
}
```

## 3.5.2 ObjectId

ObjectId is inspired by MongoDB's ObjectId, providing a 12-byte identifier that consists of:
- 4-byte timestamp
- 3-byte machine identifier
- 2-byte process ID
- 3-byte counter

```rust
use rbatis::object_id::ObjectId;

// In your model definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    // Can use String for ObjectId
    pub id: Option<String>,
    pub title: Option<String>,
    // other fields...
}

// When creating a new record
async fn create_document(rb: &RBatis, title: &str) -> rbatis::Result<Document> {
    let mut doc = Document {
        id: Some(ObjectId::new().to_string()), // Generate a new ObjectId as string
        title: Some(title.to_string()),
        // initialize other fields...
    };
    
    Document::insert(rb, &doc).await?;
    Ok(doc)
}

// Alternatively, you can use ObjectId directly in your model
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentWithObjectId {
    pub id: Option<ObjectId>,
    pub title: Option<String>,
    // other fields...
}

async fn create_document_with_object_id(rb: &RBatis, title: &str) -> rbatis::Result<DocumentWithObjectId> {
    let mut doc = DocumentWithObjectId {
        id: Some(ObjectId::new()), // Generate a new ObjectId
        title: Some(title.to_string()),
        // initialize other fields...
    };
    
    DocumentWithObjectId::insert(rb, &doc).await?;
    Ok(doc)
}
```

## 6.5 Documentation and Comments

When working with Rbatis macros, it's important to follow certain conventions for documentation and comments.

### 6.5.1 Documenting impl_* Macros

When adding documentation comments to methods generated by `impl_*` macros, the comments **must** be placed **above** the macro, not inside it:

```rust
// CORRECT: Documentation comment above the macro
/// Find users by status
/// @param status: User status to search for
impl_select!(User {find_by_status(status: i32) -> Vec => 
    "` where status = #{status}`"});

// INCORRECT: Will cause compilation errors
impl_select!(User {
    /// This comment inside the macro will cause errors
    find_by_name(name: &str) -> Vec => 
        "` where name = #{name}`"
});
```

### 6.5.2 Common Error with Comments

One common error is placing documentation comments inside the macro:

```rust
// This will cause compilation errors
impl_select!(DiscountTask {
    /// Query discount tasks by type
    find_by_type(task_type: &str) -> Vec => 
        "` where task_type = #{task_type} and state = 'published' and deleted = 0 and end_time > now() order by discount_percent desc`"
});
```

Instead, the correct approach is:

```rust
// This will work correctly
/// Query discount tasks by type
impl_select!(DiscountTask {find_by_type(task_type: &str) -> Vec => 
    "` where task_type = #{task_type} and state = 'published' and deleted = 0 and end_time > now() order by discount_percent desc`"});
```

### 6.5.3 Why This Matters

The Rbatis proc-macro system parses the macro content at compile time. When documentation comments are placed inside the macro, they interfere with the parsing process, leading to compilation errors. By placing documentation comments outside the macro, they're properly attached to the generated method while avoiding parser issues.

## 8.5 Rbatis Data Types (rbatis::rbdc::types)

Rbatis provides a set of specialized data types in the `rbatis::rbdc::types` module for better database integration and interoperability. These types handle the conversion between Rust native types and database-specific data formats. Proper understanding and usage of these types is essential for correct data handling, especially regarding ownership and conversion methods.

### 8.5.1 Decimal Type

The `Decimal` type represents arbitrary precision decimal numbers, particularly useful for financial applications.

```rust
use rbatis::rbdc::types::Decimal;
use std::str::FromStr;

// Creating Decimal instances
let d1 = Decimal::from(100i32); // From integer (Note: Use `from` not `from_i32`)
let d2 = Decimal::from_str("123.45").unwrap(); // From string
let d3 = Decimal::new("67.89").unwrap(); // Another way from string
let d4 = Decimal::from_f64(12.34).unwrap(); // From f64 (returns Option<Decimal>)

// ❌ INCORRECT - These will not work
// let wrong1 = Decimal::from_i32(100); // Error: method doesn't exist
// let mut wrong2 = Decimal::from(0); wrong2 = wrong2 + 1; // Error: using moved value

// ✅ CORRECT - Ownership handling
let decimal1 = Decimal::from(10i32);
let decimal2 = Decimal::from(20i32);
let sum = decimal1.clone() + decimal2; // Need to clone() as operations consume the value

// Rounding and scale operations
let amount = Decimal::from_str("123.456789").unwrap();
let rounded = amount.clone().round(2); // Rounds to 2 decimal places: 123.46
let scaled = amount.with_scale(3); // Sets scale to 3 decimal places: 123.457

// Conversion to primitive types
let as_f64 = amount.to_f64().unwrap_or(0.0);
let as_i64 = amount.to_i64().unwrap_or(0);
```

**Important Notes about Decimal:**
- `Decimal` wraps the `BigDecimal` type from the `bigdecimal` crate
- It doesn't implement `Copy` trait, only `Clone`
- Most operations consume the value, so you may need to use `clone()`
- Use `Decimal::from(i32)` instead of non-existent `from_i32` methods
- Always handle the `Option` or `Result` returned by conversion functions

### 8.5.2 DateTime Type

The `DateTime` type handles date and time values with timezone information.

```rust
use rbatis::rbdc::types::DateTime;
use std::str::FromStr;
use std::time::Duration;

// Creating DateTime instances
let now = DateTime::now(); // Current local time
let utc = DateTime::utc(); // Current UTC time
let dt1 = DateTime::from_str("2023-12-25 13:45:30").unwrap(); // From string
let dt2 = DateTime::from_timestamp(1640430000); // From Unix timestamp (seconds)
let dt3 = DateTime::from_timestamp_millis(1640430000000); // From milliseconds

// Formatting
let formatted = dt1.format("%Y-%m-%d %H:%M:%S"); // "2023-12-25 13:45:30"
let iso_format = dt1.to_string(); // ISO 8601 format

// Date/time components
let year = dt1.year();
let month = dt1.mon();
let day = dt1.day();
let hour = dt1.hour();
let minute = dt1.minute();
let second = dt1.sec();

// Manipulating DateTime
let tomorrow = now.clone().add(Duration::from_secs(86400));
let yesterday = now.clone().sub(Duration::from_secs(86400));
let later = now.clone().add_sub_sec(3600); // Add 1 hour

// Comparison
if dt1.before(&dt2) {
    println!("dt1 is earlier than dt2");
}

// Converting to timestamp
let ts_secs = dt1.unix_timestamp(); // Seconds since Unix epoch
let ts_millis = dt1.unix_timestamp_millis(); // Milliseconds
let ts_micros = dt1.unix_timestamp_micros(); // Microseconds
```

### 8.5.3 Json Type

When working with JSON data in Rbatis, the preferred approach is to use native Rust structs and collections directly. Rbatis is smart enough to automatically detect and properly handle `struct` or `Vec<struct>` types as JSON when serializing to the database.

```rust
use serde::{Deserialize, Serialize};

// Define your data structure
#[derive(Clone, Debug, Serialize, Deserialize)]
struct UserSettings {
    theme: String,
    notifications_enabled: bool,
    preferences: HashMap<String, String>,
}

// In your entity definition
#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
    id: Option<String>,
    username: String,
    // ✅ RECOMMENDED: Use the struct directly for JSON columns
    // Rbatis will automatically handle serialization/deserialization
    settings: Option<UserSettings>,
}

// For collections, use Vec<T> directly
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Product {
    id: Option<String>,
    name: String,
    // ✅ RECOMMENDED: Use Vec<T> directly for JSON array columns
    tags: Vec<String>,
    // Array of objects
    variants: Vec<ProductVariant>,
}

// Working with the data is natural and type-safe
let user = User {
    id: None,
    username: "alice".to_string(),
    settings: Some(UserSettings {
        theme: "dark".to_string(),
        notifications_enabled: true,
        preferences: HashMap::new(),
    }),
};

// Insert/update operations will automatically handle JSON serialization
user.insert(rb).await?;
```

While Rbatis provides specialized JSON types (`Json` and `JsonV`), they are mainly useful for specific cases:

```rust
use rbatis::rbdc::types::{Json, JsonV};
use std::str::FromStr;

// For dynamic or unstructured JSON content
let json_str = r#"{"name":"John","age":30}"#;
let json = Json::from_str(json_str).unwrap();

// JsonV is a thin wrapper around any serializable type
// Useful for explicit typing but generally not necessary
let user_settings = UserSettings { 
    theme: "light".to_string(),
    notifications_enabled: false,
    preferences: HashMap::new(),
};
let json_v = JsonV(user_settings);

// For deserializing mixed JSON content (string or object)
// This helper is useful when the database might contain either
// JSON strings or native JSON objects
#[derive(Clone, Debug, Serialize, Deserialize)]
struct UserProfile {
    id: Option<String>,
    #[serde(deserialize_with = "rbatis::rbdc::types::deserialize_maybe_str")]
    settings: UserSettings,
}
```

**Best Practices for JSON Handling in Rbatis:**

1. **Use native Rust types directly** - Let Rbatis handle the serialization/deserialization.
2. **Define proper struct types** - Create proper structs with appropriate types rather than using generic JSON objects.
3. **Use `Option<T>` for nullable JSON fields**.
4. **Only use `deserialize_maybe_str` when needed** - Use this for columns that might contain either JSON strings or native JSON objects.
5. **Avoid unnecessary wrappers** - The `JsonV` wrapper is rarely needed as Rbatis can work directly with your types.

### 8.5.4 Date, Time, and Timestamp Types

Rbatis provides specialized types for working with date, time, and timestamp data.

```rust
use rbatis::rbdc::types::{Date, Time, Timestamp};
use std::str::FromStr;

// Date type (date only)
let today = Date::now();
let christmas = Date::from_str("2023-12-25").unwrap();
println!("{}", christmas); // "2023-12-25"

// Time type (time only)
let current_time = Time::now();
let noon = Time::from_str("12:00:00").unwrap();
println!("{}", noon); // "12:00:00"

// Timestamp type (Unix timestamp)
let ts = Timestamp::now();
let custom_ts = Timestamp::from(1640430000);
println!("{}", custom_ts); // Unix timestamp in seconds
```

### 8.5.5 Bytes and UUID Types

For binary data and UUIDs, Rbatis provides the following types:

```rust
use rbatis::rbdc::types::{Bytes, Uuid};
use std::str::FromStr;

// Bytes for binary data
let data = vec![1, 2, 3, 4, 5];
let bytes = Bytes::from(data.clone());
let bytes2 = Bytes::new(data);
println!("Length: {}", bytes.len());

// UUID
let uuid = Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
let new_uuid = Uuid::random(); // Generate a new random UUID
println!("{}", uuid); // "550e8400-e29b-41d4-a716-446655440000"
```

### 8.5.6 Best Practices for Working with Rbatis Data Types

1. **Handle Ownership Correctly**: Most of the Rbatis types don't implement `Copy`, so be mindful of ownership and use `clone()` when needed.

2. **Use the Correct Creation Methods**: Pay attention to the available constructor methods. For example, use `Decimal::from(123)` instead of non-existent `Decimal::from_i32(123)`.

3. **Error Handling**: Most conversion and parsing methods return `Result` or `Option`, always handle these properly.

4. **Data Persistence**: When defining structs for database tables, use `Option<T>` for nullable fields.

5. **Type Conversion**: Be aware of the automatic type conversions that happen when reading from databases. Use the appropriate Rbatis types for your database schema.

6. **Test Boundary Cases**: Test your code with edge cases like very large numbers for `Decimal` or extreme dates for `DateTime`.

```rust
// Example of a well-designed entity using Rbatis types
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

// Proper usage in a function
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

## 7. HTML Template SQL

// ... existing code ...

### 7.3 Handle Space in SQL 

// ... existing code ...

### 7.4 XML Mapper DTD Structure

When using HTML/XML style mappings in Rbatis, it's important to understand the correct structure. Unlike MyBatis, Rbatis has specific rules for XML elements and their arrangement.

#### Valid Element Structure

The XML mapper structure in Rbatis follows these rules:

1. **Root Element**: The root element must be `<mapper>`.

2. **Valid Top-Level Elements**: The following elements are valid direct children of the `<mapper>` element:
   - `<select>`
   - `<update>`
   - `<insert>`
   - `<delete>`
   - `<sql>`
   
3. **Invalid Elements**: Unlike MyBatis, Rbatis does not support:
   - `<resultMap>` or `<BaseResultMap>` as top-level elements

#### XML Structure Example

```html
<!-- ❌ INCORRECT: Using BaseResultMap -->
<mapper>
    <BaseResultMap id="BaseResultMap" type="User">
        <id column="id" property="id"/>
        <result column="name" property="name"/>
        <result column="age" property="age"/>
    </BaseResultMap>
    
    <select id="selectById">
        select * from user where id = #{id}
    </select>
</mapper>

<!-- ✅ CORRECT: Valid structure -->
<mapper>
    <sql id="userColumns">id, name, age, email</sql>
    
    <select id="selectById">
        select * from user where id = #{id}
    </select>
    
    <insert id="insert">
        insert into user (name, age) values (#{name}, #{age})
    </insert>
    
    <update id="update">
        update user set name = #{name}, age = #{age} where id = #{id}
    </update>
    
    <delete id="delete">
        delete from user where id = #{id}
    </delete>
</mapper>
```

#### Include Tag Usage

The `<include>` tag is supported for reusing SQL fragments:

```html
<mapper>
    <sql id="userColumns">id, name, age, email</sql>
    
    <select id="selectById">
        select 
        <include refid="userColumns"/> 
        from user 
        where id = #{id}
    </select>
</mapper>
```

#### External References

You can include SQL fragments from external files:

```html
<mapper>
    <select id="selectUsers">
        select 
        <include refid="file://src/mapper/common.xml?refid=userColumns"/>
        from user
        where status = #{status}
    </select>
</mapper>
```

#### Important Notes About XML Mappers

1. **SQL Queries, Not Column Lists**: In Rbatis, you should include the actual SQL query in your elements, not just column lists.

2. **No ResultMap-Based Mappings**: Unlike MyBatis, Rbatis doesn't support result mappings through XML. It uses Rust struct definitions instead.

3. **Case Sensitivity**: Element and attribute names are case-sensitive.

4. **Document Validation**: Rbatis performs validation during compilation, not at runtime.

5. **Processing Flow**: XML mappers are parsed at compile time and converted to Rust code that generates SQL at runtime.

#### Parsing Process

The XML mapping follows this internal process:

1. Parse XML structure at compile time
2. Generate Rust functions that build dynamic SQL
3. Convert expressions in `test` attributes to Rust code
4. Handle parameter binding for `#{}` and `${}` placeholders
5. Apply whitespace handling for elements like `<trim>` and `<where>`

Understanding this structure helps avoid common errors when writing XML mappings for Rbatis.

### 7.5 Advanced Dynamic Elements Usage

Rbatis provides several powerful dynamic elements that can greatly simplify SQL generation. Understanding their full capabilities and attributes is essential for effective use.

#### The `<foreach>` Element in Detail

The `<foreach>` element is used to iterate over collections and generate repeated SQL fragments. It supports the following attributes:

```html
<foreach 
  collection="collection_expression" 
  item="item_name" 
  index="index_name" 
  open="opening_string" 
  close="closing_string" 
  separator="separator_string">
  <!-- Content to repeat -->
</foreach>
```

| Attribute | Description | Default | Required |
|-----------|-------------|---------|----------|
| `collection` | Expression pointing to the collection to iterate | - | Yes |
| `item` | Name of variable for current item | "item" | No |
| `index` | Name of variable for current index | "index" | No |
| `open` | String to prepend to the entire result | "" | No |
| `close` | String to append to the entire result | "" | No |
| `separator` | String to insert between items | "" | No |

**Examples:**

```html
<!-- Basic IN clause -->
<select id="selectByIds">
  select * from user where id in 
  <foreach collection="ids" item="id" open="(" close=")" separator=",">
    #{id}
  </foreach>
</select>

<!-- Multi-value insert -->
<insert id="batchInsert">
  insert into user (name, age) values 
  <foreach collection="users" item="user" separator=",">
    (#{user.name}, #{user.age})
  </foreach>
</insert>

<!-- Complex case with index -->
<select id="complexQuery">
  select * from user where 
  <foreach collection="conditions" item="condition" index="field" separator=" AND ">
    ${field} = #{condition}
  </foreach>
</select>
```

#### The `<set>` Element in Detail

The `<set>` element is primarily used in UPDATE statements to handle dynamic column updates. It offers both a simple and an advanced collection-based form:

**Simple form:**

```html
<update id="updateUser">
  update user 
  <set>
    <if test="name != null">name = #{name},</if>
    <if test="age != null">age = #{age},</if>
    <if test="email != null">email = #{email},</if>
  </set>
  where id = #{id}
</update>
```

**Advanced collection-based form:**

```html
<update id="updateDynamic">
  update user 
  <set collection="updates" skip_null="true" skips="id,created_at">
  </set>
  where id = #{id}
</update>
```

| Attribute | Description | Default | Required for Advanced Form |
|-----------|-------------|---------|----------|
| `collection` | Map or object to generate SET clause from | - | Yes |
| `skip_null` | Whether to skip null values | "true" | No |
| `skips` | Comma-separated list of fields to skip | "id" | No |

The collection-based form iterates through all properties of the given object or map, generating `key=value` pairs for the SET clause automatically. This is extremely useful for handling complex or unpredictable update structures.

**Example with a dynamic map:**

```rust
// In your Rust service code
let mut updates = HashMap::new();
updates.insert("name".to_string(), "John Doe".to_string());
updates.insert("age".to_string(), 30);
// Only fields present in the map will be updated
rb.exec("updateDynamic", rbs::to_value!({"updates": updates, "id": 1})).await?;
```

#### The `<trim>` Element in Detail

The `<trim>` element provides fine-grained control over whitespace and delimiters:

```html
<trim prefix="WHERE" prefixOverrides="AND |OR " suffix="" suffixOverrides=",">
  <!-- Content -->
</trim>
```

| Attribute | Description |
|-----------|-------------|
| `prefix` | String to prepend if the content is not empty |
| `suffix` | String to append if the content is not empty |
| `prefixOverrides` | Pipe-separated list of strings to remove from the beginning |
| `suffixOverrides` | Pipe-separated list of strings to remove from the end |

Aliases for compatibility:
- `start` is an alias for `prefixOverrides`
- `end` is an alias for `suffixOverrides`

**Example:**

```html
<select id="customSearch">
  select * from user
  <trim prefix="WHERE" prefixOverrides="AND |OR ">
    <if test="name != null">
      AND name like #{name}
    </if>
    <if test="age != null">
      AND age >= #{age}
    </if>
  </trim>
</select>
```

#### The `<bind>` Element for Variable Creation

The `<bind>` element creates new variables that can be used in the SQL:

```html
<select id="searchByPattern">
  <bind name="pattern" value="'%' + name + '%'" />
  select * from user where name like #{pattern}
</select>
```

This is particularly useful for preparing values before using them in SQL statements.

## 8. Json Template SQL

// ... existing code ...

## 12. Additional Resources

// ... existing code ...

### 12.6 Example Code Patterns

Below are real-world examples extracted from the Rbatis example directory. These demonstrate recommended patterns and anti-patterns for common Rbatis operations.

#### 12.6.1 CRUD Operations with `crud!` Macro

The preferred way to implement CRUD operations is using the `crud!` macro, as shown in this example from `crud.rs`:

```rust
use rbatis::crud;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<rbatis::rbdc::datetime::DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

// Choose one of these approaches:
crud!(Activity {}); // Uses the struct name 'activity' as table name
// crud!(Activity {}, "activity"); // Explicitly specify table name

// Example usage:
async fn crud_examples(rb: &RBatis) -> Result<(), rbatis::Error> {
    let table = Activity {
        id: Some("1".into()),
        name: Some("Test Activity".into()),
        status: Some(1),
        // ... other fields
        create_time: Some(rbatis::rbdc::datetime::DateTime::now()),
        version: Some(1),
        delete_flag: Some(0),
    };
    
    // Insert a single record
    let insert_result = Activity::insert(rb, &table).await?;
    
    // Batch insert
    let tables = vec![table.clone(), Activity { 
        id: Some("2".to_string()), 
        ..table.clone() 
    }];
    let batch_result = Activity::insert_batch(rb, &tables, 10).await?;
    
    // Update by column
    let update_result = Activity::update_by_column(rb, &table, "id").await?;
    
    // Update by column with skip null fields
    let update_skip_result = Activity::update_by_column_skip(rb, &table, "id", false).await?;
    
    // Select by map (multiple conditions)
    let select_result: Vec<Activity> = Activity::select_by_map(rb, rbs::to_value!{
        "id": "1",
        "status": 1,
    }).await?;
    
    // Select with IN clause
    let in_result = Activity::select_in_column(rb, "id", &["1", "2", "3"]).await?;
    
    // Delete by column
    let delete_result = Activity::delete_by_column(rb, "id", "1").await?;
    
    // Delete with IN clause
    let delete_in_result = Activity::delete_in_column(rb, "id", &["1", "2", "3"]).await?;
    
    Ok(())
}
```

**Key Benefits:**
- Automatically generates all CRUD methods
- Type-safe operations
- No SQL required for basic operations
- Handles null/Some values correctly
- Support for bulk operations

**Anti-patterns to Avoid:**
```rust
// ❌ AVOID: Directly implementing CRUDTable trait (use crud! macro instead)
impl CRUDTable for Activity {
    // ...
}

// ❌ AVOID: Raw SQL for simple operations that crud! can handle
let result = rb.exec("INSERT INTO activity (id, name) VALUES (?, ?)", 
                    vec![to_value!("1"), to_value!("name")]).await?;
```

#### 12.6.2 Table Utility Macros

The `table_util.rs` example demonstrates how to use Rbatis' table utility macros for data transformation:

```rust
use rbatis::{table, table_field_btree, table_field_map, table_field_vec};

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct Activity {
    pub id: Option<i64>,
    pub name: Option<String>,
    // ... other fields
}

fn process_activities() {
    // Create a table with only needed fields initialized
    let tables: Vec<Activity> = vec![
        table!(Activity {
            id: Some(1),
            name: Some("Activity 1".to_string()),
        }),
        table!(Activity {
            id: Some(2),
            name: Some("Activity 2".to_string()),
        }),
        table!(Activity {
            id: Some(3),
            name: Some("Activity 3".to_string()),
        })
    ];
    
    // Create a HashMap with ID as key (references)
    let id_map = table_field_map!(&tables, id);
    // Usage: id_map.get(&1) returns reference to first Activity
    
    // Create a HashMap with ID as key (owned)
    let id_map_owned = table_field_map!(tables.clone(), id);
    
    // Create a BTreeMap with ID as key (ordered, references)
    let id_btree = table_field_btree!(&tables, id);
    
    // Create a BTreeMap with ID as key (ordered, owned)
    let id_btree_owned = table_field_btree!(tables.clone(), id);
    
    // Extract a vector of IDs from tables (references)
    let ids_refs = table_field_vec!(&tables, id);
    
    // Extract a vector of IDs from tables (owned)
    let ids = table_field_vec!(tables, id);
    // ids contains [Some(1), Some(2), Some(3)]
}
```

**Key Benefits:**
- Simplifies data transformation
- Creates lookup maps efficiently 
- Works with both references and owned values
- Better than manual loops for common operations

**Anti-patterns to Avoid:**
```rust
// ❌ AVOID: Manual loops for data transformation
let mut id_map = HashMap::new();
for table in &tables {
    if let Some(id) = table.id {
        id_map.insert(id, table);
    }
}

// ❌ AVOID: Manual extraction of fields
let mut ids = Vec::new();
for table in tables {
    ids.push(table.id);
}
```

#### 12.6.3 XML Mapper Examples

The `example.html` file demonstrates proper XML mapper structure and dynamic SQL generation:

```html
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN"
        "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
<mapper>
    <!-- Reusable SQL fragment -->
    <sql id="whereClause">
        ` and id != '' `
    </sql>
    
    <!-- INSERT with dynamic columns -->
    <insert id="insert">
        `insert into activity`
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            <if test="key == 'id'">
                <continue></continue>
            </if>
            ${key}
        </foreach>
        ` values `
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            <if test="key == 'id'">
                <continue></continue>
            </if>
            ${item.sql()}
        </foreach>
    </insert>
    
    <!-- SELECT with WHERE, IF, CHOOSE conditions -->
    <select id="select_by_condition">
        `select * from activity`
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
    
    <!-- UPDATE with dynamic SET -->
    <update id="update_by_id">
        ` update activity `
        <set collection="arg"></set>
        ` where id = #{id} `
    </update>
</mapper>
```

**Usage in Rust:**
```rust
use rbatis::RBatis;

async fn use_xml_mapper(rb: &RBatis) -> Result<(), rbatis::Error> {
    // First, load the HTML file
    rb.load_html("example/example.html").await?;
    
    // Then use the XML mapper methods
    let params = rbs::to_value!({
        "name": "test%",
        "dt": "2023-01-01 00:00:00"
    });
    
    let results: Vec<Activity> = rb.fetch("select_by_condition", &params).await?;
    
    // For the dynamic update
    let update_params = rbs::to_value!({
        "id": 1,
        "name": "Updated Name",
        "status": 2
    });
    
    let update_result = rb.exec("update_by_id", &update_params).await?;
    
    Ok(())
}
```

**Key Benefits:**
- Clean separation of SQL from Rust code
- Dynamic SQL generation
- Support for complex queries
- Reusable SQL fragments

**Anti-patterns to Avoid:**
```html
<!-- ❌ AVOID: Using ResultMap in Rbatis -->
<mapper>
    <resultMap id="BaseResultMap" type="Activity">
        <!-- This is NOT supported in Rbatis -->
        <id column="id" property="id"/>
        <result column="name" property="name"/>
    </resultMap>
</mapper>

<!-- ❌ AVOID: Missing backticks for SQL keywords -->
<select id="badSql">
    SELECT * FROM activity WHERE id = #{id}
</select>
```

#### 12.6.4 Raw SQL Operations

When the CRUD macros and HTML mappers aren't sufficient, you can use raw SQL as shown in `raw_sql.rs`:

```rust
use rbatis::RBatis;
use rbs::to_value;

async fn raw_sql_examples(rb: &RBatis) -> Result<(), rbatis::Error> {
    // Query with parameters and decode to struct
    let activity: Option<Activity> = rb
        .query_decode("select * from activity where id = ? limit 1", 
                     vec![to_value!("1")])
        .await?;
    
    // Query multiple rows
    let activities: Vec<Activity> = rb
        .query_decode("select * from activity where status = ?", 
                     vec![to_value!(1)])
        .await?;
    
    // Execute statement without returning results
    let affected_rows = rb
        .exec("update activity set status = ? where id = ?", 
             vec![to_value!(0), to_value!("1")])
        .await?;
    
    // Execute insert
    let insert_result = rb
        .exec("insert into activity (id, name, status) values (?, ?, ?)", 
             vec![to_value!("3"), to_value!("New Activity"), to_value!(1)])
        .await?;
    
    // Execute delete
    let delete_result = rb
        .exec("delete from activity where id = ?", 
             vec![to_value!("3")])
        .await?;
    
    Ok(())
}
```

**Key Benefits:**
- Full control over SQL
- Useful for complex queries
- Good for migrations and schema changes
- Handles any SQL the driver supports

**Anti-patterns to Avoid:**
```rust
// ❌ AVOID: String concatenation for SQL (SQL injection risk)
let id = "1";
let unsafe_sql = format!("select * from activity where id = '{}'", id);
let result = rb.query_decode(unsafe_sql, vec![]).await?;

// ❌ AVOID: Raw SQL for standard CRUD operations
// Use Activity::insert(rb, &activity) instead of:
rb.exec("insert into activity (id, name) values (?, ?)", 
       vec![to_value!(activity.id), to_value!(activity.name)]).await?;
```

#### 12.6.5 Common Mistakes and Best Practices

Based on the example code, here are some general best practices and common mistakes:

**Best Practices:**
1. **Use the `crud!` macro** for standard CRUD operations
2. **Leverage table utilities** for data transformation
3. **Use HTML mappers** for complex queries
4. **Place SQL keywords in backticks** when using HTML mappers
5. **Use parameter binding** (`#{param}` or `?`) rather than string concatenation
6. **Always check for nulls** in dynamic SQL
7. **Prefer `select_in_column`** over complex JOIN operations
8. **Use `table!` macro** to create partially initialized structs

**Common Mistakes:**
1. **❌ Implementing `CRUDTable` manually** instead of using `crud!`
2. **❌ Using `ResultMap` elements** which aren't supported in Rbatis XML
3. **❌ Forgetting backticks** around SQL keywords in HTML mappers
4. **❌ String concatenation for SQL parameters** (SQL injection risk)
5. **❌ Complex JOINs** instead of using `select_in_column` and merging in Rust
6. **❌ Inefficient loops** instead of using table utility macros
7. **❌ Missing DOCTYPE declaration** in HTML mapper files
8. **❌ Unnecessary raw SQL** for operations supported by macros

Remember that Rbatis is designed to be Rust-idiomatic, and it often differs from other ORMs like MyBatis. Following these patterns will help you use Rbatis effectively and avoid common pitfalls.

## 13. Conclusion

// ... existing code ...