///PySql: gen select*,update*,insert*,delete* ... methods
///```rust
/// use rbatis::{Error, RBatis};
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{
///    pub id: Option<String>
/// }
/// rbatis::crud!(MockTable{}); //or crud!(MockTable{},"mock_table");
///
/// //use
/// async fn test_use(rb:&RBatis) -> Result<(),Error>{
///  let table = MockTable{id: Some("1".to_string())};
///  let r = MockTable::insert(rb, &table).await;
///  let r = MockTable::insert_batch(rb, std::slice::from_ref(&table),10).await;
///
///  let tables = MockTable::select_by_column(rb,"id","1").await;
///  let tables = MockTable::select_all(rb).await;
///  let tables = MockTable::select_in_column(rb,"id", &vec!["1","2","3"]).await;
///
///  let r = MockTable::update_by_column(rb, &table,"id").await;
///
///  let r = MockTable::delete_by_column(rb, "id","1").await;
///  //... and more
///  Ok(())
/// }
///
///
/// ```
#[macro_export]
macro_rules! crud {
    ($table:ty{}) => {
        $crate::rbexec::crud!($crate::rbexec,$table {});
    };
    ($table:ty{},$table_name:expr) => {
        $crate::rbexec::crud!($crate::rbexec,$table {}, $table_name);
    };
}

///PySql: gen sql => INSERT INTO table_name (column1,column2,column3,...) VALUES (value1,value2,value3,...);
///
/// example:
///```rust
/// use rbatis::{Error, RBatis};
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{
///   pub id: Option<String>
/// }
/// rbatis::impl_insert!(MockTable{});
///
/// //use
/// async fn test_use(rb:&RBatis) -> Result<(),Error>{
///  let table = MockTable{id: Some("1".to_string())};
///  let r = MockTable::insert(rb, &table).await;
///  let r = MockTable::insert_batch(rb, std::slice::from_ref(&table),10).await;
///  Ok(())
/// }
/// ```
///
#[macro_export]
macro_rules! impl_insert {
    ($table:ty{}) => {
        $crate::rbexec::impl_insert!($crate::rbexec,$table {}, "");
    };
    ($table:ty{},$table_name:expr) => {
        $crate::rbexec::impl_insert!($crate::rbexec,$table {}, $table_name);
    };
}

///PySql: gen sql => SELECT (column1,column2,column3,...) FROM table_name (column1,column2,column3,...)  *** WHERE ***
///
/// example:
///```rust
/// use rbatis::{Error, RBatis};
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{
///   pub id: Option<String>
/// }
/// /// default
///rbatis::impl_select!(MockTable{});
///rbatis::impl_select!(MockTable{select_all_by_id(id:&str,name:&str) => "`where id = #{id} and name = #{name}`"});
/// /// container result
///rbatis::impl_select!(MockTable{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});
///rbatis::impl_select!(MockTable{select_by_id2(id:String) -> Vec => "`where id = #{id} limit 1`"});
///
/// //usage
/// async fn test_select(rb:&RBatis) -> Result<(),Error>{
///    let r = MockTable::select_by_column(rb,"id","1").await?;
///    let r = MockTable::select_all_by_id(rb,"1","xxx").await?;
///    let r:Option<MockTable> = MockTable::select_by_id(rb,"1".to_string()).await?;
///    let r:Vec<MockTable> = MockTable::select_by_id2(rb,"1".to_string()).await?;
///    Ok(())
/// }
/// ```
///
#[macro_export]
macro_rules! impl_select {
    ($table:ty{}) => {
         $crate::rbexec::impl_select!($crate::rbexec,$table {}, "");
    };
    ($table:ty{},$table_name:expr) => {
        $crate::rbexec::impl_select!($crate::rbexec,$table {}, $table_name);
    };
    ($table:ty{$fn_name:ident $(< $($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty $(,)?)*) => $sql:expr}$(,$table_name:expr)?) => {
        $crate::rbexec::impl_select!($crate::rbexec,$table{$fn_name$(<$($gkey:$gtype,)*>)?($($param_key:$param_type,)*) ->Vec => $sql}$(,$table_name)?);
    };
    ($table:ty{$fn_name:ident $(< $($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty $(,)?)*) -> $container:tt => $sql:expr}$(,$table_name:expr)?) => {
        $crate::rbexec::impl_select!($crate::rbexec,$table{$fn_name$(<$($gkey:$gtype,)*>)?($($param_key:$param_type,)*) -> $container => $sql}$(,$table_name)?);
    };
}

/// PySql: gen sql = UPDATE table_name SET column1=value1,column2=value2,... WHERE some_column=some_value;
/// ```rust
/// use rbatis::{Error, RBatis};
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{
///   pub id: Option<String>
/// }
/// rbatis::impl_update!(MockTable{});
/// //use
/// async fn test_use(rb:&RBatis) -> Result<(),Error>{
///  let table = MockTable{id: Some("1".to_string())};
///  let r = MockTable::update_by_column(rb, &table,"id").await;
///  Ok(())
/// }
/// ```
#[macro_export]
macro_rules! impl_update {
    ($table:ty{}) => {
        $crate::rbexec::impl_update!($crate::rbexec,$table{});
    };
    ($table:ty{},$table_name:expr) => {
        $crate::rbexec::impl_update!($crate::rbexec,$table{},$table_name);
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}$(,$table_name:expr)?) => {
        $crate::rbexec::impl_update!($crate::rbexec,$table{$fn_name($($param_key:$param_type,)*) => $sql_where}$(,$table_name:expr)?);
    };
}

/// PySql: gen sql = DELETE FROM table_name WHERE some_column=some_value;
///
/// ```rust
/// use rbatis::{Error, RBatis};
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{}
/// rbatis::impl_delete!(MockTable{});
///
/// //use
/// async fn test_use(rb:&RBatis) -> Result<(),Error>{
///  let r = MockTable::delete_by_column(rb, "id","1").await;
///  //... and more
///  Ok(())
/// }
/// ```
#[macro_export]
macro_rules! impl_delete {
    ($table:ty{}) => {
        $crate::rbexec::impl_delete!($crate::rbexec,$table{});
    };
    ($table:ty{},$table_name:expr) => {
        $crate::rbexec::impl_delete!($crate::rbexec,$table{},$table_name);
    };
    ($table:ty{$fn_name:ident $(< $($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}$(,$table_name:expr)?) => {
        $crate::rbexec::impl_delete!($crate::rbexec,$table{$fn_name $(< $($gkey:$gtype,)* >)? ($($param_key:$param_type,)*) => $sql_where}$(,$table_name)?);
    };
}

/// pysql impl_select_page
///
/// do_count: default do_count is a bool param value to determine the statement type
///
/// ```rust
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{}
/// rbatis::impl_select_page!(MockTable{select_page() =>"
///      if do_count == false:
///        `order by create_time desc`"});
/// ```
///
/// limit_sql: If the database does not support the statement `limit ${page_no},${page_size}`,You should add param 'limit_sql:&str'
/// ```rust
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{}
/// rbatis::impl_select_page!(MockTable{select_page(limit_sql:&str) =>"
///      if do_count == false:
///        `order by create_time desc`"});
/// ```
/// you can see ${page_no} = (page_no -1) * page_size;
/// you can see ${page_size} = page_size;
#[macro_export]
macro_rules! impl_select_page {
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $where_sql:expr}) => {
        $crate::rbexec::impl_select_page!($crate::rbexec,$table{$fn_name($($param_key:$param_type,)*)=> $where_sql});
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $where_sql:expr}$(,$table_name:expr)?) => {
        $crate::rbexec::impl_select_page!($crate::rbexec,$table{$fn_name($($param_key:$param_type,)*) => $where_sql}$(,$table_name)?);
    };
}

/// impl html_sql select page.
///
/// you must deal with 3 param:
/// (do_count:bool,page_no:u64,page_size:u64)
///
/// you must deal with sql:
/// return Vec<Record>（if param do_count = false）
/// return u64（if param do_count = true）
///
/// you can see ${page_no} = (page_no -1) * page_size;
/// you can see ${page_size} = page_size;
///
/// just like this example:
/// ```html
/// <select id="select_page_data">
///         `select `
///         <if test="do_count == true">
///             `count(1) from table`
///         </if>
///         <if test="do_count == false">
///             `* from table limit ${page_no},${page_size}`
///         </if>
///   </select>
/// ```
/// ```
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{}
/// //rbatis::htmlsql_select_page!(select_page_data(name: &str) -> MockTable => "example.html");
/// rbatis::htmlsql_select_page!(select_page_data(name: &str) -> MockTable => r#"
/// <select id="select_page_data">
///   `select `
///  <if test="do_count == true">
///   `count(1) from table`
///  </if>
///  <if test="do_count == false">
///  `* from table limit ${page_no},${page_size}`
///  </if>
/// </select>"#);
/// ```
#[macro_export]
macro_rules! htmlsql_select_page {
    ($fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $table:ty => $html_file:expr) => {
          $crate::rbexec::htmlsql_select_page!($crate::rbexec,$fn_name($($param_key:$param_type,)*) -> $table => $html_file);
    }
}

/// impl py_sql select page.
///
/// you must deal with 3 param:
/// (do_count:bool,page_no:u64,page_size:u64)
///
/// you must deal with sql:
/// return Vec<Record>（if param do_count = false）
/// return u64（if param do_count = true）·
///
/// you can see ${page_no} = (page_no -1) * page_size;
/// you can see ${page_size} = page_size;
///
/// just like this example:
/// ```py
/// `select * from activity where delete_flag = 0`
///                   if name != '':
///                     ` and name=#{name}`
///                   if !ids.is_empty():
///                     ` and id in `
///                     ${ids.sql()}
/// ```
/// ```
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{}
/// rbatis::pysql_select_page!(pysql_select_page(name:&str) -> MockTable =>
///     r#"`select `
///       if do_count == true:
///         ` count(1) as count `
///       if do_count == false:
///          ` * `
///       `from activity where delete_flag = 0`
///         if name != '':
///            ` and name=#{name}`
///       ` limit ${page_no},${page_size}`
/// "#);
/// ```
#[macro_export]
macro_rules! pysql_select_page {
    ($fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $table:ty => $py_file:expr) => {
          $crate::rbexec::pysql_select_page!($crate::rbexec,$fn_name($($param_key:$param_type,)*) -> $table => $py_file);
    }
}

/// use macro wrapper #[sql]
/// for example:
/// ```rust
/// use rbatis::executor::Executor;
/// rbatis::raw_sql!(test_same_id(rb: &dyn Executor, id: &u64)  -> Result<rbs::Value, rbatis::Error> =>
/// "select * from table where id = ?"
/// );
/// ```
#[macro_export]
macro_rules! raw_sql {
    ($fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $return_type:ty => $sql_file:expr) => {
       $crate::rbexec::raw_sql!($crate::rbexec,$fn_name($($param_key:$param_type,)*) -> $return_type => $sql_file);
    }
}

/// use macro wrapper #[py_sql]
/// for query example:
/// ```rust
/// use rbatis::executor::Executor;
/// rbatis::pysql!(test_same_id(rb: &dyn Executor, id: &u64)  -> Result<rbs::Value, rbatis::Error> =>
/// "select * from table where ${id} = 1
///  if id != 0:
///    `id = #{id}`"
/// );
/// ```
/// for exec example:
/// ```rust
/// use rbatis::executor::Executor;
/// use rbdc::db::ExecResult;
/// rbatis::pysql!(test_same_id(rb: &dyn Executor, id: &u64)  -> Result<ExecResult, rbatis::Error> =>
/// "`update activity set name = '1' where id = #{id}`"
/// );
/// ```
#[macro_export]
macro_rules! pysql {
    ($fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $return_type:ty => $py_file:expr) => {
       $crate::rbexec::pysql!($crate::rbexec,$fn_name($($param_key:$param_type,)*) -> $return_type => $py_file);
    }
}

/// use macro wrapper #[html_sql]
/// for example query rbs::Value:
/// ```rust
/// use rbatis::executor::Executor;
/// rbatis::htmlsql!(test_select_column(rb: &dyn Executor, id: &u64)  -> Result<rbs::Value, rbatis::Error> => r#"
///             <mapper>
///             <select id="test_same_id">
///               `select ${id} from my_table`
///             </select>
///             </mapper>"#);
/// ```
/// exec (from file)
/// ```rust
/// use rbatis::executor::Executor;
/// use rbdc::db::ExecResult;
/// rbatis::htmlsql!(update_by_id(rb: &dyn Executor, id: &u64)  -> Result<ExecResult, rbatis::Error> => "example/example.html");
/// ```
/// query
/// ```rust
/// use rbatis::executor::Executor;
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MyTable{
///      pub id:Option<u64>,
///      pub name:Option<String>,
/// }
/// rbatis::htmlsql!(test_select_table(rb: &dyn Executor, id: &u64)  -> Result<Vec<MyTable>, rbatis::Error> => r#"
///             <mapper>
///               <select id="test_same_id">
///                 `select * from my_table`
///               </select>
///             </mapper>"#);
/// ```
#[macro_export]
macro_rules! htmlsql {
    ($fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $return_type:ty => $html_file:expr) => {
        $crate::rbexec::htmlsql!($crate::rbexec,$fn_name($($param_key:$param_type,)*) -> $return_type => $html_file);
    }
}
