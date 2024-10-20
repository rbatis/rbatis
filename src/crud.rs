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
        $crate::impl_insert!($table {});
        $crate::impl_select!($table {});
        $crate::impl_update!($table {});
        $crate::impl_delete!($table {});
    };
    ($table:ty{},$table_name:expr) => {
        $crate::impl_insert!($table {}, $table_name);
        $crate::impl_select!($table {}, $table_name);
        $crate::impl_update!($table {}, $table_name);
        $crate::impl_delete!($table {}, $table_name);
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
        $crate::impl_insert!($table {}, "");
    };
    ($table:ty{},$table_name:expr) => {
        impl $table {
            pub async fn insert_batch(
                executor: &dyn $crate::executor::Executor,
                tables: &[$table],
                batch_size: u64,
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                pub trait ColumnSet{
                    /// take `vec![Table{"id":1}]` columns
                    fn column_sets(&self)->rbs::Value;
                }
                impl ColumnSet for rbs::Value {
                    fn column_sets(&self) -> rbs::Value {
                        let len = self.len();
                        let mut column_set = std::collections::HashSet::with_capacity(len);
                        for item in self.as_array().unwrap() {
                            for (k,v) in &item {
                                if (*v) != rbs::Value::Null{
                                    column_set.insert(k);
                                }
                            }
                        }
                        let mut columns = rbs::Value::Array(vec![]);
                        if len > 0 {
                            let table = &self[0];
                            let mut column_datas = Vec::with_capacity(table.len());
                            for (column,_) in table {
                                if column_set.contains(&column){
                                    column_datas.push(column);
                                }
                            }
                            columns = rbs::Value::from(column_datas);
                        }
                        columns
                    }
                }
                #[$crate::py_sql(
                    "`insert into ${table_name} `
                    trim ',':
                     bind columns = tables.column_sets():
                     for idx,table in tables:
                      if idx == 0:
                         `(`
                         trim ',':
                           for _,v in columns:
                              ${v},
                         `) VALUES `
                      (
                      trim ',':
                       for _,v in columns:
                         #{table[v]},
                      ),
                    "
                )]
                async fn insert_batch(
                    executor: &dyn $crate::executor::Executor,
                    tables: &[$table],
                    table_name: &str,
                ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error>
                {
                    impled!()
                }
                if tables.is_empty() {
                    return Err($crate::rbdc::Error::from(
                        "insert can not insert empty array tables!",
                    ));
                }
                #[$crate::snake_name($table)]
                fn snake_name() {}
                let mut table_name = $table_name.to_string();
                if table_name.is_empty() {
                    table_name = snake_name();
                }
                let mut result = $crate::rbdc::db::ExecResult {
                    rows_affected: 0,
                    last_insert_id: rbs::Value::Null,
                };
                let ranges = $crate::plugin::Page::<()>::make_ranges(tables.len() as u64, batch_size);
                for (offset, limit) in ranges {
                    let exec_result = insert_batch(
                        executor,
                        &tables[offset as usize..limit as usize],
                        table_name.as_str(),
                    )
                    .await?;
                    result.rows_affected += exec_result.rows_affected;
                    result.last_insert_id = exec_result.last_insert_id;
                }
                Ok(result)
            }

            pub async fn insert(
                executor: &dyn $crate::executor::Executor,
                table: &$table,
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                <$table>::insert_batch(executor, std::slice::from_ref(table), 1).await
            }
        }
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
        $crate::impl_select!($table{},"");
    };
    ($table:ty{},$table_name:expr) => {
        $crate::impl_select!($table{select_all() => ""},$table_name);
        $crate::impl_select!($table{select_by_column<V:serde::Serialize>(column: &str,column_value: V) -> Vec => "` where ${column} = #{column_value}`"},$table_name);
        $crate::impl_select!($table{select_in_column<V:serde::Serialize>(column: &str,column_values: &[V]) -> Vec =>
         "` where ${column} in (`
          trim ',': for _,item in column_values:
             #{item},
          `)`"},$table_name);
    };
    ($table:ty{$fn_name:ident $(< $($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty $(,)?)*) => $sql:expr}$(,$table_name:expr)?) => {
        $crate::impl_select!($table{$fn_name$(<$($gkey:$gtype,)*>)?($($param_key:$param_type,)*) ->Vec => $sql}$(,$table_name)?);
    };
    ($table:ty{$fn_name:ident $(< $($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty $(,)?)*) -> $container:tt => $sql:expr}$(,$table_name:expr)?) => {
        impl $table{
            pub async fn $fn_name $(<$($gkey:$gtype,)*>)? (executor: &dyn  $crate::executor::Executor,$($param_key:$param_type,)*) -> std::result::Result<$container<$table>,$crate::rbdc::Error>
            {
                     #[$crate::py_sql("`select ${table_column} from ${table_name} `",$sql)]
                     async fn $fn_name$(<$($gkey: $gtype,)*>)?(executor: &dyn $crate::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> std::result::Result<$container<$table>,$crate::rbdc::Error> {impled!()}
                     let mut table_column = "*".to_string();
                     let mut table_name = String::new();
                     $(table_name = $table_name.to_string();)?
                     #[$crate::snake_name($table)]
                     fn snake_name(){}
                     if table_name.is_empty(){
                         table_name = snake_name();
                     }
                     $fn_name(executor,&table_column,&table_name,$($param_key ,)*).await
            }
        }
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
        $crate::impl_update!(
            $table{},
            ""
        );
    };
    ($table:ty{},$table_name:expr) => {
        $crate::impl_update!($table{update_by_column_value(column: &str, column_value: &rbs::Value, skip_null: bool) => "`where ${column} = #{column_value}`"},$table_name);
        impl $table {
            ///  will skip null column
            pub async fn update_by_column(
                executor: &dyn $crate::executor::Executor,
                table: &$table,
                column: &str) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error>{
                <$table>::update_by_column_skip(executor,table,column,true).await
            }

            ///will skip null column
            pub async fn update_by_column_batch(
                executor: &dyn $crate::executor::Executor,
                tables: &[$table],
                column: &str,
                batch_size: u64
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
              <$table>::update_by_column_batch_skip(executor,tables,column,batch_size,true).await
            }

            pub async fn update_by_column_skip(
                executor: &dyn $crate::executor::Executor,
                table: &$table,
                column: &str,
                skip_null: bool) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error>{
                let columns = rbs::to_value!(table);
                let column_value = &columns[column];
                <$table>::update_by_column_value(executor,table,column,column_value,skip_null).await
            }

            pub async fn update_by_column_batch_skip(
                executor: &dyn $crate::executor::Executor,
                tables: &[$table],
                column: &str,
                batch_size: u64,
                skip_null: bool
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                let mut rows_affected = 0;
                let ranges = $crate::plugin::Page::<()>::make_ranges(tables.len() as u64, batch_size);
                for (offset, limit) in ranges {
                    //todo better way impl batch?
                    for table in &tables[offset as usize..limit as usize]{
                       rows_affected += <$table>::update_by_column_skip(executor,table,column,skip_null).await?.rows_affected;
                    }
                }
                Ok($crate::rbdc::db::ExecResult{
                    rows_affected:rows_affected,
                    last_insert_id:rbs::Value::Null,
                })
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}$(,$table_name:expr)?) => {
        impl $table {
            pub async fn $fn_name(
                executor: &dyn $crate::executor::Executor,
                table: &$table,
                $($param_key:$param_type,)*
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                if $sql_where.is_empty(){
                    return Err($crate::rbdc::Error::from("sql_where can't be empty!"));
                }
                #[$crate::py_sql("`update ${table_name} set `
                                 trim ',':
                                   for k,v in table:
                                     if k == column:
                                        continue:
                                     if skip_null == true && v == null:
                                        continue:
                                     `${k}=#{v},`
                                 ` `",$sql_where)]
                  async fn $fn_name(
                      executor: &dyn $crate::executor::Executor,
                      table_name: String,
                      table: &rbs::Value,
                      skip_null:bool,
                      $($param_key:$param_type,)*
                  ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                      impled!()
                  }
                  let mut table_name = String::new();
                  $(table_name = $table_name.to_string();)?
                  #[$crate::snake_name($table)]
                  fn snake_name(){}
                  if table_name.is_empty(){
                         table_name = snake_name();
                  }
                  let table = rbs::to_value!(table);
                  $fn_name(executor, table_name, &table, true, $($param_key,)*).await
            }
        }
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
        $crate::impl_delete!(
            $table{},
            ""
        );
    };
    ($table:ty{},$table_name:expr) => {
        $crate::impl_delete!($table {delete_by_column<V:serde::Serialize>(column:&str,column_value: V) => "`where ${column} = #{column_value}`"},$table_name);
        $crate::impl_delete!($table {delete_in_column<V:serde::Serialize>(column:&str,column_values: &[V]) =>
        "`where ${column} in (`
          trim ',': for _,item in column_values:
             #{item},
          `)`"},$table_name);

        impl $table {
            pub async fn delete_by_column_batch<V:serde::Serialize>(
                executor: &dyn $crate::executor::Executor,
                column: &str,
                values: &[V],
                batch_size: u64,
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                let mut rows_affected = 0;
                let ranges = $crate::plugin::Page::<()>::make_ranges(values.len() as u64, batch_size);
                for (offset, limit) in ranges {
                    rows_affected += <$table>::delete_in_column(executor,column,&values[offset as usize..limit as usize]).await?.rows_affected;
                }
                Ok($crate::rbdc::db::ExecResult{
                    rows_affected: rows_affected,
                    last_insert_id: rbs::Value::Null
                })
            }
        }
    };
    ($table:ty{$fn_name:ident $(< $($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}$(,$table_name:expr)?) => {
        impl $table {
            pub async fn $fn_name$(<$($gkey:$gtype,)*>)?(
                executor: &dyn $crate::executor::Executor,
                $($param_key:$param_type,)*
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                if $sql_where.is_empty(){
                    return Err($crate::rbdc::Error::from("sql_where can't be empty!"));
                }
                #[$crate::py_sql("`delete from ${table_name} `",$sql_where)]
                async fn $fn_name$(<$($gkey: $gtype,)*>)?(
                    executor: &dyn $crate::executor::Executor,
                    table_name: String,
                    $($param_key:$param_type,)*
                ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                    impled!()
                }
                let mut table_name = String::new();
                $(table_name = $table_name.to_string();)?
                #[$crate::snake_name($table)]
                fn snake_name(){}
                if table_name.is_empty(){
                         table_name = snake_name();
                }
                $fn_name(executor, table_name, $($param_key,)*).await
            }
        }
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
        $crate::impl_select_page!(
            $table{$fn_name($($param_key:$param_type,)*)=> $where_sql},
            ""
        );
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $where_sql:expr}$(,$table_name:expr)?) => {
        impl $table {
            pub async fn $fn_name(
                executor: &dyn $crate::executor::Executor,
                page_request: &dyn $crate::plugin::IPageRequest,
                $($param_key:$param_type,)*
            ) -> std::result::Result<$crate::plugin::Page::<$table>, $crate::rbdc::Error> {
                let mut table_column = "*".to_string();
                let mut table_name = String::new();
                $(table_name = $table_name.to_string();)?
                #[$crate::snake_name($table)]
                fn snake_name(){}
                if table_name.is_empty(){
                    table_name = snake_name();
                }
                //pg,mssql can override this parameter to implement its own limit statement
                let mut limit_sql = " limit ${page_no},${page_size}".to_string();
                limit_sql=limit_sql.replace("${page_no}", &page_request.offset().to_string());
                limit_sql=limit_sql.replace("${page_size}", &page_request.page_size().to_string());
                let records:Vec<$table>;
                struct Inner{}
                impl Inner{
                 #[$crate::py_sql(
                    "`select `
                    if do_count == false:
                      `${table_column}`
                    if do_count == true:
                       `count(1) as count`
                    ` from ${table_name} `\n",$where_sql,"\n
                    if do_count == false:
                        `${limit_sql}`")]
                   async fn $fn_name(executor: &dyn $crate::executor::Executor,
                                     do_count:bool,
                                     table_column:&str,
                                     table_name: &str,
                                     page_no:u64,
                                     page_size:u64,
                                     page_offset:u64,
                                     limit_sql:&str,
                 $($param_key:&$param_type,)*) -> std::result::Result<rbs::Value, $crate::rbdc::Error> {impled!()}
                }
                let mut total = 0;
                if page_request.do_count() {
                    let total_value = Inner::$fn_name(executor,
                                                      true,
                                                      &table_column,
                                                      &table_name,
                                                      page_request.page_no(),
                                                      page_request.page_size(),
                                                      page_request.offset(),
                                                      "",
                                                      $(&$param_key,)*).await?;
                    total = $crate::decode(total_value).unwrap_or(0);
                }
                let mut page = $crate::plugin::Page::<$table>::new_total(page_request.page_no(), page_request.page_size(), total);
                let records_value = Inner::$fn_name(executor,
                                                    false,
                                                    &table_column,
                                                    &table_name,
                                                    page_request.page_no(),
                                                    page_request.page_size(),
                                                    page_request.offset(),
                                                    &limit_sql,
                                                    $(&$param_key,)*).await?;
                page.records = rbs::from_value(records_value)?;
                Ok(page)
            }
        }
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
            pub async fn $fn_name(executor: &dyn $crate::executor::Executor, page_request: &dyn $crate::plugin::IPageRequest, $($param_key:$param_type,)*) -> std::result::Result<$crate::plugin::Page<$table>, $crate::rbdc::Error> {
            struct Inner{}
            impl Inner{
              #[$crate::html_sql($html_file)]
              pub async fn $fn_name(executor: &dyn $crate::executor::Executor,do_count:bool,page_no:u64,page_size:u64,$($param_key: &$param_type,)*) -> std::result::Result<rbs::Value, $crate::rbdc::Error>{
                 $crate::impled!()
              }
            }
            let mut total = 0;
            if page_request.do_count() {
               let total_value = Inner::$fn_name(executor, true, page_request.offset(), page_request.page_size(), $(&$param_key,)*).await?;
               total = $crate::decode(total_value).unwrap_or(0);
            }
            let mut page = $crate::plugin::Page::<$table>::new_total(page_request.page_no(), page_request.page_size(), total);
            let records_value = Inner::$fn_name(executor, false, page_request.offset(), page_request.page_size(), $(&$param_key,)*).await?;
            page.records = rbs::from_value(records_value)?;
            Ok(page)
         }
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
            pub async fn $fn_name(executor: &dyn $crate::executor::Executor, page_request: &dyn $crate::plugin::IPageRequest, $($param_key:$param_type,)*) -> std::result::Result<$crate::plugin::Page<$table>, $crate::rbdc::Error> {
            struct Inner{}
            impl Inner{
              #[$crate::py_sql($py_file)]
              pub async fn $fn_name(executor: &dyn $crate::executor::Executor,do_count:bool,page_no:u64,page_size:u64,$($param_key: &$param_type,)*) -> std::result::Result<rbs::Value, $crate::rbdc::Error>{
                 $crate::impled!()
              }
            }
            let mut total = 0;
            if page_request.do_count() {
               let total_value = Inner::$fn_name(executor, true, page_request.offset(), page_request.page_size(), $(&$param_key,)*).await?;
               total = $crate::decode(total_value).unwrap_or(0);
            }
            let mut page = $crate::plugin::Page::<$table>::new_total(page_request.page_no(), page_request.page_size(), total);
            let records_value = Inner::$fn_name(executor, false, page_request.offset(), page_request.page_size(), $(&$param_key,)*).await?;
            page.records = rbs::from_value(records_value)?;
            Ok(page)
         }
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
       pub async fn $fn_name($($param_key: $param_type,)*) -> $return_type{
           pub struct Inner{};
           impl Inner{
               #[$crate::sql($sql_file)]
               pub async fn $fn_name($($param_key: $param_type,)*) -> $return_type{
                 impled!()
               }
           }
           Inner::$fn_name($($param_key,)*).await
       }
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
       pub async fn $fn_name($($param_key: $param_type,)*) -> $return_type{
           pub struct Inner{};
           impl Inner{
               #[$crate::py_sql($py_file)]
               pub async fn $fn_name($($param_key: $param_type,)*) -> $return_type{
                 impled!()
               }
           }
           Inner::$fn_name($($param_key,)*).await
       }
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
        pub async fn $fn_name($($param_key: $param_type,)*) -> $return_type{
            pub struct Inner{};
            impl Inner{
            #[$crate::html_sql($html_file)]
            pub async fn $fn_name($($param_key: $param_type,)*) -> $return_type{
              impled!()
             }
           }
           Inner::$fn_name($($param_key,)*).await
        }
    }
}
