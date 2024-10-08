///PySql: gen select*,update*,insert*,delete* ... methods
///```rust
/// use rbexec::{Error, Executor};
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{
///    pub id: Option<String>
/// }
/// rbexec::crud!(rbexec,MockTable{}); //or crud!(MockTable{},"mock_table");
///
/// //use
/// async fn test_use(rb:&dyn Executor) -> Result<(),Error>{
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
    ($path:path,$table:ty{}) => {
        $crate::impl_insert!($path,$table {});
        $crate::impl_select!($path,$table {});
        $crate::impl_update!($path,$table {});
        $crate::impl_delete!($path,$table {});
    };
    ($path:path,$table:ty{},$table_name:expr) => {
        $crate::impl_insert!($path,$table {}, $table_name);
        $crate::impl_select!($path,$table {}, $table_name);
        $crate::impl_update!($path,$table {}, $table_name);
        $crate::impl_delete!($path,$table {}, $table_name);
    };
}

///PySql: gen sql => INSERT INTO table_name (column1,column2,column3,...) VALUES (value1,value2,value3,...);
///
/// example:
///```rust
/// use rbexec::{Error, Executor};
/// use rbexec::impl_insert;
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{
///   pub id: Option<String>
/// }
/// impl_insert!(rbexec,MockTable{});
///
/// //use
/// async fn test_use(rb:&dyn Executor) -> Result<(),Error>{
///  let table = MockTable{id: Some("1".to_string())};
///  let r = MockTable::insert(rb, &table).await;
///  let r = MockTable::insert_batch(rb, std::slice::from_ref(&table),10).await;
///  Ok(())
/// }
/// ```
///
#[macro_export]
macro_rules! impl_insert {
    ($path:path,$table:ty{}) => {
        $crate::impl_insert!($path,$table {}, "");
    };
    ($path:path,$table:ty{},$table_name:expr) => {
        impl $table {
            pub async fn insert_batch(
                executor: &dyn $crate::executor::Executor,
                tables: &[$table],
                batch_size: u64,
            ) -> std::result::Result<$crate::executor::ExecResult, $crate::executor::Error> {
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
                #[$crate::py_sql($path,
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
                ) -> std::result::Result<$crate::executor::ExecResult, $crate::executor::Error>
                {
                    impled!()
                }
                if tables.is_empty() {
                    return Err($crate::executor::Error::from(
                        "insert can not insert empty array tables!",
                    ));
                }
                #[$crate::snake_name($table)]
                fn snake_name() {}
                let mut table_name = $table_name.to_string();
                if table_name.is_empty() {
                    table_name = snake_name();
                }
                let mut result = $crate::executor::ExecResult {
                    rows_affected: 0,
                    last_insert_id: rbs::Value::Null,
                };
                let ranges = $crate::page::Page::<()>::make_ranges(tables.len() as u64, batch_size);
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
            ) -> std::result::Result<$crate::executor::ExecResult, $crate::executor::Error> {
                <$table>::insert_batch(executor, std::slice::from_ref(table), 1).await
            }
        }
    };
}

///PySql: gen sql => SELECT (column1,column2,column3,...) FROM table_name (column1,column2,column3,...)  *** WHERE ***
///
/// example:
///```rust
/// use rbexec::{Error, Executor};
/// use rbexec::impl_select;
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{
///   pub id: Option<String>
/// }
/// /// default
///impl_select!(rbexec,MockTable{});
///impl_select!(rbexec,MockTable{select_all_by_id(id:&str,name:&str) => "`where id = #{id} and name = #{name}`"});
/// /// container result
///impl_select!(rbexec,MockTable{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});
///impl_select!(rbexec,MockTable{select_by_id2(id:String) -> Vec => "`where id = #{id} limit 1`"});
///
/// //usage
/// async fn test_select(rb:&dyn Executor) -> Result<(),Error>{
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
    ($path:path,$table:ty{}) => {
        $crate::impl_select!($path,$table{},"");
    };
    ($path:path,$table:ty{},$table_name:expr) => {
        $crate::impl_select!($path,$table{select_all() => ""},$table_name);
        $crate::impl_select!($path,$table{select_by_column<V:serde::Serialize>(column: &str,column_value: V) -> Vec => "` where ${column} = #{column_value}`"},$table_name);
        $crate::impl_select!($path,$table{select_in_column<V:serde::Serialize>(column: &str,column_values: &[V]) -> Vec =>
         "` where ${column} in (`
          trim ',': for _,item in column_values:
             #{item},
          `)`"},$table_name);
    };
    ($path:path,$table:ty{$fn_name:ident $(< $($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty $(,)?)*) => $sql:expr}$(,$table_name:expr)?) => {
        $crate::impl_select!($path,$table{$fn_name$(<$($gkey:$gtype,)*>)?($($param_key:$param_type,)*) ->Vec => $sql}$(,$table_name)?);
    };
    ($path:path,$table:ty{$fn_name:ident $(< $($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty $(,)?)*) -> $container:tt => $sql:expr}$(,$table_name:expr)?) => {
        impl $table{
            pub async fn $fn_name $(<$($gkey:$gtype,)*>)? (executor: &dyn  $crate::executor::Executor,$($param_key:$param_type,)*) -> std::result::Result<$container<$table>,$crate::executor::Error>
            {
                     #[$crate::py_sql($path,"`select ${table_column} from ${table_name} `",$sql)]
                     async fn $fn_name$(<$($gkey: $gtype,)*>)?(executor: &dyn $crate::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> std::result::Result<$container<$table>,$crate::executor::Error> {impled!()}
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
/// use rbexec::{Error, Executor};
/// use rbexec::impl_update;
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{
///   pub id: Option<String>
/// }
/// impl_update!(rbexec,MockTable{});
/// //use
/// async fn test_use(rb:&dyn Executor) -> Result<(),Error>{
///  let table = MockTable{id: Some("1".to_string())};
///  let r = MockTable::update_by_column(rb, &table,"id").await;
///  Ok(())
/// }
/// ```
#[macro_export]
macro_rules! impl_update {
    ($path:path,$table:ty{}) => {
        $crate::impl_update!(
            $path,
            $table{},
            ""
        );
    };
    ($path:path,$table:ty{},$table_name:expr) => {
        $crate::impl_update!($path,$table{update_by_column_value(column: &str, column_value: &rbs::Value, skip_null: bool) => "`where ${column} = #{column_value}`"},$table_name);
        impl $table {
            ///  will skip null column
            pub async fn update_by_column(
                executor: &dyn $crate::executor::Executor,
                table: &$table,
                column: &str) -> std::result::Result<$crate::executor::ExecResult, $crate::executor::Error>{
                <$table>::update_by_column_skip(executor,table,column,true).await
            }

            ///will skip null column
            pub async fn update_by_column_batch(
                executor: &dyn $crate::executor::Executor,
                tables: &[$table],
                column: &str,
                batch_size: u64
            ) -> std::result::Result<$crate::executor::ExecResult, $crate::executor::Error> {
              <$table>::update_by_column_batch_skip(executor,tables,column,batch_size,true).await
            }

            pub async fn update_by_column_skip(
                executor: &dyn $crate::executor::Executor,
                table: &$table,
                column: &str,
                skip_null: bool) -> std::result::Result<$crate::executor::ExecResult, $crate::executor::Error>{
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
            ) -> std::result::Result<$crate::executor::ExecResult, $crate::executor::Error> {
                let mut rows_affected = 0;
                let ranges = $crate::page::Page::<()>::make_ranges(tables.len() as u64, batch_size);
                for (offset, limit) in ranges {
                    //todo better way impl batch?
                    for table in &tables[offset as usize..limit as usize]{
                       rows_affected += <$table>::update_by_column_skip(executor,table,column,skip_null).await?.rows_affected;
                    }
                }
                Ok($crate::executor::ExecResult{
                    rows_affected:rows_affected,
                    last_insert_id:rbs::Value::Null,
                })
            }
        }
    };
    ($path:path,$table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}$(,$table_name:expr)?) => {
        impl $table {
            pub async fn $fn_name(
                executor: &dyn $crate::executor::Executor,
                table: &$table,
                $($param_key:$param_type,)*
            ) -> std::result::Result<$crate::executor::ExecResult, $crate::executor::Error> {
                if $sql_where.is_empty(){
                    return Err($crate::executor::Error::from("sql_where can't be empty!"));
                }
                #[$crate::py_sql($path,
                            "`update ${table_name} set `
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
                  ) -> std::result::Result<$crate::executor::ExecResult, $crate::executor::Error> {
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
/// use rbexec::{Error, Executor};
/// use rbexec::impl_delete;
///
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{}
/// impl_delete!(rbexec,MockTable{});
/// //use
/// async fn test_use(rb:&dyn Executor) -> Result<(),Error>{
///  let r = MockTable::delete_by_column(rb, "id","1").await;
///  //... and more
///  Ok(())
/// }
/// ```
#[macro_export]
macro_rules! impl_delete {
    ($path:path,$table:ty{}) => {
        $crate::impl_delete!(
            $path,
            $table{},
            ""
        );
    };
    ($path:path,$table:ty{},$table_name:expr) => {
        $crate::impl_delete!($path,$table {delete_by_column<V:serde::Serialize>(column:&str,column_value: V) => "`where ${column} = #{column_value}`"},$table_name);
        $crate::impl_delete!($path,$table {delete_in_column<V:serde::Serialize>(column:&str,column_values: &[V]) =>
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
            ) -> std::result::Result<$crate::executor::ExecResult, $crate::executor::Error> {
                let mut rows_affected = 0;
                let ranges = $crate::page::Page::<()>::make_ranges(values.len() as u64, batch_size);
                for (offset, limit) in ranges {
                    rows_affected += <$table>::delete_in_column(executor,column,&values[offset as usize..limit as usize]).await?.rows_affected;
                }
                Ok($crate::executor::ExecResult{
                    rows_affected: rows_affected,
                    last_insert_id: rbs::Value::Null
                })
            }
        }
    };
    ($path:path,$table:ty{$fn_name:ident $(< $($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}$(,$table_name:expr)?) => {
        impl $table {
            pub async fn $fn_name$(<$($gkey:$gtype,)*>)?(
                executor: &dyn $crate::executor::Executor,
                $($param_key:$param_type,)*
            ) -> std::result::Result<$crate::executor::ExecResult, $crate::executor::Error> {
                if $sql_where.is_empty(){
                    return Err($crate::executor::Error::from("sql_where can't be empty!"));
                }
                #[$crate::py_sql($path,"`delete from ${table_name} `",$sql_where)]
                async fn $fn_name$(<$($gkey: $gtype,)*>)?(
                    executor: &dyn $crate::executor::Executor,
                    table_name: String,
                    $($param_key:$param_type,)*
                ) -> std::result::Result<$crate::executor::ExecResult, $crate::executor::Error> {
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
/// use rbexec::impl_select_page;
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{}
/// impl_select_page!(rbexec,MockTable{select_page() =>"
///      if do_count == false:
///        `order by create_time desc`"});
/// ```
///
/// limit_sql: If the database does not support the statement `limit ${page_no},${page_size}`,You should add param 'limit_sql:&str'
/// ```rust
/// use rbexec::impl_select_page;
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{}
/// impl_select_page!(rbexec,MockTable{select_page(limit_sql:&str) =>"
///      if do_count == false:
///        `order by create_time desc`"});
/// ```
/// you can see ${page_no} = (page_no -1) * page_size;
/// you can see ${page_size} = page_size;
#[macro_export]
macro_rules! impl_select_page {
    ($path:path,$table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $where_sql:expr}) => {
        $crate::impl_select_page!(
            $path,
            $table{$fn_name($($param_key:$param_type,)*)=> $where_sql},
            ""
        );
    };
    ($path:path,$table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $where_sql:expr}$(,$table_name:expr)?) => {
        impl $table {
            pub async fn $fn_name(
                executor: &dyn $crate::executor::Executor,
                page_request: &dyn $crate::page::IPageRequest,
                $($param_key:$param_type,)*
            ) -> std::result::Result<$crate::page::Page::<$table>, $crate::executor::Error> {
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
                 #[$crate::py_sql($path,
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
                 $($param_key:&$param_type,)*) -> std::result::Result<rbs::Value, $crate::executor::Error> {impled!()}
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
                let mut page = $crate::page::Page::<$table>::new_total(page_request.page_no(), page_request.page_size(), total);
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
/// use rbexec::htmlsql_select_page;
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{}
/// //rbexec::htmlsql_select_page!(select_page_data(name: &str) -> MockTable => "example.html");
/// htmlsql_select_page!(rbexec,select_page_data(name: &str) -> MockTable => r#"
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
    ($path:path,$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $table:ty => $html_file:expr) => {
            pub async fn $fn_name(executor: &dyn $crate::executor::Executor, page_request: &dyn $crate::page::IPageRequest, $($param_key:$param_type,)*) -> std::result::Result<$crate::page::Page<$table>, $crate::executor::Error> {
            struct Inner{}
            impl Inner{
              #[$crate::html_sql($path,$html_file)]
              pub async fn $fn_name(executor: &dyn $crate::executor::Executor,do_count:bool,page_no:u64,page_size:u64,$($param_key: &$param_type,)*) -> std::result::Result<rbs::Value, $crate::executor::Error>{
                 $crate::impled!()
              }
            }
            let mut total = 0;
            if page_request.do_count() {
               let total_value = Inner::$fn_name(executor, true, page_request.offset(), page_request.page_size(), $(&$param_key,)*).await?;
               total = $crate::decode(total_value).unwrap_or(0);
            }
            let mut page = $crate::page::Page::<$table>::new_total(page_request.page_no(), page_request.page_size(), total);
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
/// use rbexec::pysql_select_page;
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{}
/// pysql_select_page!(rbexec,pysql_select_page(name:&str) -> MockTable =>
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
    ($path:path,$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $table:ty => $py_file:expr) => {
            pub async fn $fn_name(executor: &dyn $crate::executor::Executor, page_request: &dyn $crate::page::IPageRequest, $($param_key:$param_type,)*) -> std::result::Result<$crate::page::Page<$table>, $crate::executor::Error> {
            struct Inner{}
            impl Inner{
              #[$crate::py_sql($path,$py_file)]
              pub async fn $fn_name(executor: &dyn $crate::executor::Executor,do_count:bool,page_no:u64,page_size:u64,$($param_key: &$param_type,)*) -> std::result::Result<rbs::Value, $crate::executor::Error>{
                 $crate::impled!()
              }
            }
            let mut total = 0;
            if page_request.do_count() {
               let total_value = Inner::$fn_name(executor, true, page_request.offset(), page_request.page_size(), $(&$param_key,)*).await?;
               total = $crate::decode(total_value).unwrap_or(0);
            }
            let mut page = $crate::page::Page::<$table>::new_total(page_request.page_no(), page_request.page_size(), total);
            let records_value = Inner::$fn_name(executor, false, page_request.offset(), page_request.page_size(), $(&$param_key,)*).await?;
            page.records = rbs::from_value(records_value)?;
            Ok(page)
         }
    }
}

/// use macro wrapper #[sql]
/// for example:
/// ```rust
/// use rbexec::executor::Executor;
/// use rbexec::raw_sql;
/// raw_sql!(rbexec,test_same_id(rb: &dyn Executor, id: &u64)  -> Result<rbs::Value, rbexec::Error> =>
/// "select * from table where id = ?"
/// );
/// ```
#[macro_export]
macro_rules! raw_sql {
    ($path:path,$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $return_type:ty => $sql_file:expr) => {
       pub async fn $fn_name($($param_key: $param_type,)*) -> $return_type{
           pub struct Inner{};
           impl Inner{
               #[$crate::sql($path,$sql_file)]
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
/// use rbexec::executor::Executor;
/// use rbexec::pysql;
/// pysql!(rbexec,test_same_id(rb: &dyn Executor, id: &u64)  -> Result<rbs::Value, rbexec::Error> =>
/// "select * from table where ${id} = 1
///  if id != 0:
///    `id = #{id}`"
/// );
/// ```
/// for exec example:
/// ```rust
/// use rbexec::executor::Executor;
/// use rbexec::pysql;
/// use rbdc::db::ExecResult;
/// pysql!(rbexec,test_same_id(rb: &dyn Executor, id: &u64)  -> Result<ExecResult, rbexec::Error> =>
/// "`update activity set name = '1' where id = #{id}`"
/// );
/// ```
#[macro_export]
macro_rules! pysql {
    ($path:path,$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $return_type:ty => $py_file:expr) => {
       pub async fn $fn_name($($param_key: $param_type,)*) -> $return_type{
           pub struct Inner{};
           impl Inner{
               #[$crate::py_sql($path,$py_file)]
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
/// use rbexec::executor::Executor;
/// use rbexec::htmlsql;
/// htmlsql!(rbexec,test_select_column(rb: &dyn Executor, id: &u64)  -> Result<rbs::Value, rbexec::Error> => r#"
///             <mapper>
///             <select id="test_same_id">
///               `select ${id} from my_table`
///             </select>
///             </mapper>"#);
/// ```
/// exec (from file)
/// ```rust
/// use rbexec::executor::Executor;
/// use rbexec::htmlsql;
/// use rbdc::db::ExecResult;
/// htmlsql!(rbexec,update_by_id(rb: &dyn Executor, id: &u64)  -> Result<ExecResult, rbexec::Error> => "example/example.html");
/// ```
/// query
/// ```rust
/// use rbexec::executor::Executor;
/// use rbexec::htmlsql;
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct MyTable{
///      pub id:Option<u64>,
///      pub name:Option<String>,
/// }
/// htmlsql!(rbexec,test_select_table(rb: &dyn Executor, id: &u64)  -> Result<Vec<MyTable>, rbexec::Error> => r#"
///             <mapper>
///               <select id="test_same_id">
///                 `select * from my_table`
///               </select>
///             </mapper>"#);
/// ```
#[macro_export]
macro_rules! htmlsql {
    ($path:path,$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $return_type:ty => $html_file:expr) => {
        pub async fn $fn_name($($param_key: $param_type,)*) -> $return_type{
            pub struct Inner{};
            impl Inner{
            #[$crate::html_sql($path,$html_file)]
            pub async fn $fn_name($($param_key: $param_type,)*) -> $return_type{
              impled!()
             }
           }
           Inner::$fn_name($($param_key,)*).await
        }
    }
}
