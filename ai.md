# Rbatis Framework User Guide

> This documentation is based on Rbatis 4.5+ and provides detailed instructions for using the Rbatis ORM framework. Rbatis is a high-performance Rust asynchronous ORM framework that supports multiple databases and provides compile-time dynamic SQL capabilities similar to MyBatis.

## Important Version Notes and Best Practices

Rbatis 4.5+ has significant improvements over previous versions. Here are the key changes and recommended best practices:

1. **Use macros instead of traits**: In current versions, use `crud!` and `impl_*` macros instead of implementing the `CRUDTable` trait (which was used in older 3.0 versions).

2. **Preferred pattern for defining models and operations**:
   ```rust
   // 1. Define your model
   #[derive(Clone, Debug, Serialize, Deserialize)]
   pub struct User {
       pub id: Option<String>,
       pub name: Option<String>,
       // other fields...
   }
   
   // 2. Generate basic CRUD operations
   crud!(User {});  // or crud!(User {}, "custom_table_name");
   
   // 3. Define custom methods using impl_* macros
   // Note: Doc comments must be placed ABOVE the impl_* macro, not inside it
   /// Select users by name
   impl_select!(User {select_by_name(name: &str) -> Vec => "` where name = #{name}`"});
   
   /// Get user by ID
   impl_select!(User {select_by_id(id: &str) -> Option => "` where id = #{id} limit 1`"});
   
   /// Update user status by ID
   impl_update!(User {update_status_by_id(id: &str, status: i32) => "` set status = #{status} where id = #{id}`"});
   
   /// Delete users by name
   impl_delete!(User {delete_by_name(name: &str) => "` where name = #{name}`"});
   ```

3. **Use lowercase SQL keywords**: Always use lowercase for SQL keywords like `select`, `where`, `and`, etc.

4. **Proper backtick usage**: Enclose dynamic SQL fragments in backticks (`) to preserve spaces.

5. **Async-first approach**: All database operations should be awaited with `.await`.

6. **Use SnowflakeId or ObjectId for IDs**: Rbatis provides built-in ID generation mechanisms that should be used for primary keys.

7. **Prefer select_in_column over JOIN**: For better performance and maintainability, avoid complex JOINs and use Rbatis' select_in_column to fetch related data, then combine them in your service layer.

Please refer to the examples below for the current recommended approaches.

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

// Note: In Rbatis 4.5+, using the crud! macro is the recommended approach
// rather than implementing the CRUDTable trait (which was used in older versions)
// Instead of implementing CRUDTable, use the following approach:

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

#### 6.1.1 Space Handling Mechanism

In HTML style dynamic SQL, **backticks (`) are the key to handling spaces**:

- **Default trims spaces**: Non-backtick-enclosed text nodes will automatically remove leading and trailing spaces
- **Backticks preserve original text**: Text enclosed in backticks(`) will preserve all spaces and newlines
- **Must use backticks**: Dynamic SQL fragments must be enclosed in backticks, otherwise leading spaces and newlines will be ignored
- **Complete enclosure**: Backticks should enclose the entire SQL fragment, not just the beginning part

Incorrect use of backticks example:
```rust
<if test="status != null">
    and status = #{status} <!-- No backticks, leading spaces will be lost -->
</if>

<if test="type != null">
    ` and type = #{type} ` <!-- Only beginning has backticks, not complete -->
</if>
```

Correct use of backticks example:
```rust
<if test="status != null">
    ` and status = #{status} ` <!-- Complete enclosure, all spaces preserved -->
</if>

<if test="items != null and items.len > 0">
    ` and item_id in `
    <foreach collection="items" item="item" open="(" close=")" separator=",">
        #{item}
    </foreach>
</if>
```

#### 6.1.2 Differences from MyBatis

Rbatis' HTML style has several key differences from MyBatis:

1. **No need for CDATA**: Rbatis does not need to use CDATA blocks to escape special characters
   ```rust
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
   ```rust
   <!-- MyBatis -->
   <if test="list != null and list.size() > 0">

   <!-- Rbatis -->
   <if test="list != null and list.len > 0">
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
   ```xml
   <if test="condition">
       SQL fragment
   </if>
   ```

2. **`<choose>/<when>/<otherwise>`**：Multi-condition selection (similar to switch statement)
   ```xml
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
   ```xml
   <trim prefixOverrides="AND|OR" suffixOverrides=",">
       SQL fragment
   </trim>
   ```

4. **`<foreach>`**：Loop processing
   ```xml
   <foreach collection="items" item="item" index="index" separator=",">
       #{item}
   </foreach>
   ```

5. **`<where>`**：Automatically handle WHERE clause (will smartly remove leading AND/OR)
   ```xml
   <where>
       <if test="id != null">
           and id = #{id}
       </if>
   </where>
   ```

6. **`<set>`**：Automatically handle SET clause (will smartly manage commas)
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

7. **`<bind>`**：Variable binding
   ```xml
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

Expression example:
```rust
<if test="user.age >= 18 && (user.role == 'admin' || user.vip)">
    ` and is_adult = 1 `
</if>

if (page_size * (page_no - 1)) <= total && !items.is_empty():
    ` limit #{page_size} offset #{page_size * (page_no - 1)} `
```

### 6.5 Parameter Binding Mechanism

Rbatis provides two parameter binding methods:

1. **Named Parameters**: Use `#{name}` format, automatically prevent SQL injection
   ```rust
   ` select * from user where username = #{username} `
   ```

2. **Position Parameters**: Use `?` placeholder, bind in order
   ```rust
   ` select * from user where username = ? and age > ? `
   ```

3. **Raw Interpolation**: Use `${expr}` format, directly insert expression result (**Use with caution**)
   ```rust
   ` select * from ${table_name} where id > 0 ` # Used for dynamic table name
   ```

**Safety Tips**:
- `#{}` binding will automatically escape parameters, prevent SQL injection, recommended for binding values
- `${}` directly inserts content, exists SQL injection risk, only used for table name, column name, etc. structure elements
- For IN statements, use `.sql()` method to generate safe IN clause

Core difference:
- **`#{}` binding**:
  - Converts value to parameter placeholder, actual value placed in parameter array
  - Automatically handles type conversion and NULL values
  - Prevent SQL injection

- **`${}` binding**:
  - Directly converts expression result to string inserted into SQL
  - Used for dynamic table name, column name, etc. structure elements
  - Does not handle SQL injection risk

### 6.6 Dynamic SQL Practical Tips

#### 6.6.1 Complex Condition Construction

```rust
#[py_sql(r#"
select * from user
where 1=1
if name != None and name.trim() != '':  # Check empty string
    ` and name like #{name} `
if ids != None and !ids.is_empty():     # Use built-in function
    ` and id in ${ids.sql()} `           # Use .sql() method to generate in statement
if (age_min != None and age_max != None) and (age_min < age_max):
    ` and age between #{age_min} and #{age_max} `
if age_min != None:
    ` and age >= #{age_min} `
if age_max != None:
    ` and age <= #{age_max} `
"#)]
```

#### 6.6.2 Dynamic Sorting and Grouping

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

#### 6.6.3 Dynamic Table Name and Column Name

```rust
#[py_sql(r#"
select ${select_fields} from ${table_name}
where ${where_condition}
"#)]
async fn dynamic_query(
    rb: &dyn Executor,
    select_fields: &str,  // Must be safe value
    table_name: &str,     // Must be safe value
    where_condition: &str, // Must be safe value
) -> rbatis::Result<Vec<Value>> {
    impled!()
}
```

#### 6.6.4 General Fuzzy Query

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
    search_text_like: Option<&str>, // Preprocess as %text%
) -> rbatis::Result<Vec<User>> {
    impled!()
}

// Usage example
let search = "test";
let result = fuzzy_search(&rb, Some(search), Some(&format!("%{}%", search))).await?;
```

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
```

### 10.3 Test Strategy

- Unit Test: Use Mock database for business logic testing
- Integration Test: Use test container (e.g., Docker) to create temporary database environment
- Performance Test: Simulate high concurrency scenario to test system performance and stability

## 11. Complete Example

The following is a complete Web application example that uses Rbatis to build, showing how to organize code and use various Rbatis features.

### 11.1 Project Structure

```
src/
├── main.rs                 # Application entry
├── config.rs               # Configuration management
├── error.rs                # Error definition
├── models/                 # Data model
│   ├── mod.rs
│   ├── user.rs
│   └── order.rs
├── repositories/           # Data access layer
│   ├── mod.rs
│   ├── user_repository.rs
│   └── order_repository.rs
├── services/               # Business logic layer
│   ├── mod.rs
│   ├── user_service.rs
│   └── order_service.rs
└── api/                    # API interface layer
    ├── mod.rs
    ├── user_controller.rs
    └── order_controller.rs
```

### 11.2 Data Model Layer

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

### 11.3 Data Access Layer

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
    
    // Use HTML style dynamic SQL for advanced query
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

### 11.4 Business Logic Layer

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
        // Add business logic, such as password encryption, data validation, etc.
        if user.status.is_none() {
            user.status = Some(1); // Default status
        }
        user.create_time = Some(rbatis::rbdc::datetime::DateTime::now());
        
        // Start transaction processing
        let tx = self.rb.acquire_begin().await?;
        
        // Check if username already exists
        let exist_users = UserRepository::search(
            &tx, 
            Some(user.username.clone()), 
            None
        ).await?;
        
        if !exist_users.is_empty() {
            tx.rollback().await?;
            return Err(Error::from("Username already exists"));
        }
        
        // Save user
        UserRepository::save(&tx, user).await?;
        
        // Commit transaction
        tx.commit().await?;
        
        Ok(())
    }
    
    pub async fn update_user(&self, user: &User) -> Result<(), Error> {
        if user.id.is_none() {
            return Err(Error::from("User ID cannot be empty"));
        }
        
        // Check if user exists
        let exist = UserRepository::find_by_id(&self.rb, user.id.as_ref().unwrap()).await?;
        if exist.is_none() {
            return Err(Error::from("User does not exist"));
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
            // Fuzzy query processing
            let like_username = format!("%{}%", username_str);
            UserRepository::search(&self.rb, Some(like_username), status).await
                .map(|users| {
                    // Manual paging processing
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
            // Use built-in paging query
            let page_req = PageRequest::new(page, page_size);
            UserRepository::find_by_status(&self.rb, status.unwrap_or(1), &page_req).await
        }
    }
}
```

### 11.5 API Interface Layer

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
            ApiResponse::<()>::error(404, "User does not exist".to_string())
        ),
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(500, format!("Server error: {}", e))
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
            ApiResponse::<()>::error(500, format!("Server error: {}", e))
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
            if e.to_string().contains("Username already exists") {
                HttpResponse::BadRequest().json(
                    ApiResponse::<()>::error(400, e.to_string())
                )
            } else {
                HttpResponse::InternalServerError().json(
                    ApiResponse::<()>::error(500, format!("Server error: {}", e))
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
            if e.to_string().contains("User does not exist") {
                HttpResponse::NotFound().json(
                    ApiResponse::<()>::error(404, e.to_string())
                )
            } else {
                HttpResponse::InternalServerError().json(
                    ApiResponse::<()>::error(500, format!("Server error: {}", e))
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
            ApiResponse::<()>::error(500, format!("Server error: {}", e))
        ),
    }
}
```

### 11.6 Application Configuration and Startup

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
    // Initialize log
    env_logger::init();
    
    // Initialize database connection
    let rb = RBatis::new();
    rb.init(
        rbdc_mysql::driver::MysqlDriver{}, 
        &config::get_database_url()
    ).unwrap();
    
    // Run table synchronization (Optional)
    rb.sync(models::user::User {
        id: None,
        username: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
        create_time: None,
        status: None,
    }).await.unwrap();
    
    // Create service
    let user_service = UserService::new(rb.clone());
    
    // Start HTTP server
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

### 11.7 Client Call Example

```rust
// Use reqwest client to call API
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
    
    // Create user
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
    
    println!("Create user response: {:?}", resp);
    
    // Query user list
    let resp = client.get("http://localhost:8080/api/users")
        .query(&[("page", "1"), ("page_size", "10")])
        .send()
        .await?
        .json::<ApiResponse<Vec<User>>>()
        .await?;
    
    println!("User list: {:?}", resp);
    
    Ok(())
}
```

This complete example shows how to use Rbatis to build a Web application containing data model, data access layer, business logic layer, and API interface layer, covering various Rbatis features, including basic CRUD operations, dynamic SQL, transaction management, paging query, etc. Through this example, developers can quickly understand how to effectively use Rbatis in actual projects.

## 11.8 Modern Rbatis 4.5+ Example

Here's a concise example that shows the recommended way to use Rbatis 4.5+:

```rust
use rbatis::{crud, impl_select, impl_update, impl_delete, RBatis};
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize, Serialize};
use rbatis::rbdc::datetime::DateTime;

// Define your data model
#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
    id: Option<String>,
    username: Option<String>,
    email: Option<String>,
    status: Option<i32>,
    create_time: Option<DateTime>,
}

// Generate basic CRUD methods
crud!(User {});

// Define custom query methods
impl_select!(User{find_by_username(username: &str) -> Option => 
    "` where username = #{username} limit 1`"});

impl_select!(User{find_active_users() -> Vec => 
    "` where status = 1 order by create_time desc`"});

impl_update!(User{update_status(id: &str, status: i32) =>
    "` set status = #{status} where id = #{id}`"});

impl_delete!(User{remove_inactive() =>
    "` where status = 0`"});

// Define a page query
impl_select_page!(User{find_by_email_page(email: &str) =>
    "` where email like #{email}`"});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    fast_log::init(fast_log::Config::new().console()).unwrap();
    
    // Create RBatis instance and connect to database
    let rb = RBatis::new();
    rb.link(SqliteDriver {}, "sqlite://test.db").await?;
    
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