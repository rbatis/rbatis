///PySql: gen select*,update*,insert*,delete* ... methods
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
/// ```rust
/// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// pub struct BizActivity{ pub id: Option<String> }
/// rbatis::impl_insert!(BizActivity{});
/// ```
///
#[macro_export]
macro_rules! impl_insert {
    ($table:ty{}) => {
        $crate::impl_insert!(
            $table {},
            $crate::utils::string_util::to_snake_name(stringify!($table))
        );
    };
    ($table:ty{},$table_name:expr) => {
        impl $table {
            pub async fn insert_batch(
                executor: &mut dyn $crate::executor::Executor,
                tables: &[$table],
                batch_size: u64,
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                #[$crate::py_sql(
                    "`insert into ${table_name} `
                    trim ',':
                     for idx,table in tables:
                      if idx == 0:
                         `(`
                         trim ',':
                           for k,v in table:
                              if k == 'id' && v== null:
                                 continue:
                              ${k},
                         `) VALUES `
                      (
                      trim ',':
                       for k,v in table:
                         if k == 'id' && v== null:
                            continue:
                         #{v},
                      ),
                    "
                )]
                async fn insert_batch(
                    executor: &mut dyn $crate::executor::Executor,
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
                let table_name = $table_name.to_string();
                let mut result = $crate::rbdc::db::ExecResult {
                    rows_affected: 0,
                    last_insert_id: rbs::Value::Null,
                };
                let ranges = $crate::sql::Page::<()>::make_ranges(tables.len() as u64, batch_size);
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
                executor: &mut dyn $crate::executor::Executor,
                table: &$table,
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                <$table>::insert_batch(executor, &[table.clone()], 1).await
            }
        }
    };
}

///PySql: gen sql => SELECT (column1,column2,column3,...) FROM table_name (column1,column2,column3,...)  *** WHERE ***
///
/// example:
///```rust
/// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// pub struct BizActivity{ pub id: Option<String> }
///rbatis::impl_select!(BizActivity{});
///rbatis::impl_select!(BizActivity{select_all_by_id(id:&str,name:&str) => "where id = #{id} and name = #{name}"});
///rbatis::impl_select!(BizActivity{select_by_id(id:String) -> Option => "where id = #{id} limit 1"});
///
/// //use
/// //BizActivity::select**()
/// ```
///
#[macro_export]
macro_rules! impl_select {
    ($table:ty{}) => {
        $crate::impl_select!($table{},$crate::utils::string_util::to_snake_name(stringify!($table)));
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
            pub async fn $fn_name $(<$($gkey:$gtype,)*>)? (executor: &mut dyn  $crate::executor::Executor,$($param_key:$param_type,)*) -> std::result::Result<$container<$table>,$crate::rbdc::Error>
            {
                     #[$crate::py_sql("`select ${table_column} from ${table_name} `",$sql)]
                     async fn $fn_name$(<$($gkey: $gtype,)*>)?(executor: &mut dyn $crate::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> std::result::Result<$container<$table>,$crate::rbdc::Error> {impled!()}
                     let mut table_column = "*".to_string();
                     let mut table_name = String::new();
                     $(table_name = $table_name.to_string();)?
                     if table_name.is_empty(){
                         table_name = $crate::utils::string_util::to_snake_name(stringify!($table));
                     }
                     $fn_name(executor,&table_column,&table_name,$($param_key ,)*).await
            }
        }
    };
}

/// PySql: gen sql = UPDATE table_name SET column1=value1,column2=value2,... WHERE some_column=some_value;
/// ```rust
/// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// pub struct BizActivity{ pub id: Option<String> }
/// rbatis::impl_update!(BizActivity{});
/// ```
#[macro_export]
macro_rules! impl_update {
    ($table:ty{}) => {
        $crate::impl_update!(
            $table{},
            $crate::utils::string_util::to_snake_name(stringify!($table))
        );
    };
    ($table:ty{},$table_name:expr) => {
        $crate::impl_update!($table{update_by_column_value(column: &str,column_value: &rbs::Value) => "`where ${column} = #{column_value}`"},$table_name);
        impl $table {
            pub async fn update_by_column(
                executor: &mut dyn $crate::executor::Executor,
                table: &$table,
                column: &str) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error>{
                let columns = rbs::to_value!(table);
                let column_value = &columns[column];
                <$table>::update_by_column_value(executor,table,column,column_value).await
            }

            pub async fn update_by_column_batch(
                executor: &mut dyn $crate::executor::Executor,
                tables: &[$table],
                column: &str,
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                let mut rows_affected = 0;
                for item in tables{
                    rows_affected += <$table>::update_by_column(executor,item,column).await?.rows_affected;
                }
                Ok($crate::rbdc::db::ExecResult{
                    rows_affected:rows_affected,
                    last_insert_id:rbs::Value::Null
                })
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}$(,$table_name:expr)?) => {
        impl $table {
            pub async fn $fn_name(
                executor: &mut dyn $crate::executor::Executor,
                table: &$table,
                $($param_key:$param_type,)*
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                if $sql_where.is_empty(){
                    return Err($crate::rbdc::Error::from("sql_where can't be empty!"));
                }
                #[$crate::py_sql("`update ${table_name} set `
                                 trim ',':
                                   for k,v in table:
                                     if k == column || v== null:
                                        continue:
                                     `${k}=#{v},`
                                 ` `",$sql_where)]
                  async fn $fn_name(
                      executor: &mut dyn $crate::executor::Executor,
                      table_name: String,
                      table: &rbs::Value,
                      $($param_key:$param_type,)*
                  ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                      impled!()
                  }
                  let mut table_name = String::new();
                  $(table_name = $table_name.to_string();)?
                  if table_name.is_empty(){
                      table_name = $crate::utils::string_util::to_snake_name(stringify!($table));
                  }
                  let table = rbs::to_value!(table);
                  $fn_name(executor, table_name, &table, $($param_key,)*).await
            }
        }
    };
}

/// PySql: gen sql = DELETE FROM table_name WHERE some_column=some_value;
///
/// ```rust
/// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// pub struct BizActivity{}
/// rbatis::impl_delete!(BizActivity{});
/// ```
#[macro_export]
macro_rules! impl_delete {
    ($table:ty{}) => {
        $crate::impl_delete!(
            $table{},
            $crate::utils::string_util::to_snake_name(stringify!($table))
        );
    };
    ($table:ty{},$table_name:expr) => {
        $crate::impl_delete!($table {delete_by_column<V:serde::Serialize>(column:&str,column_value: V) => "`where ${column} = #{column_value}`"},$table_name);
        $crate::impl_delete!($table {delete_in_column<V:serde::Serialize>(column:&str,column_values: &[V]) =>
        "`where ${column} in (`
          trim ',': for _,item in column_values:
             #{item},
          `)`"},$table_name);
        $crate::impl_delete!($table {delete_by_column_batch<V:serde::Serialize>(column:&str,column_values: &[V]) => "`where ${column} in (`
                                       trim ',':
                                         for _,v in column_values:
                                            #{v},
                                       `)`"},$table_name);
    };
    ($table:ty{$fn_name:ident $(< $($gkey:ident:$gtype:path $(,)?)* >)? ($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}$(,$table_name:expr)?) => {
        impl $table {
            pub async fn $fn_name$(<$($gkey:$gtype,)*>)?(
                executor: &mut dyn $crate::executor::Executor,
                $($param_key:$param_type,)*
            ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                if $sql_where.is_empty(){
                    return Err($crate::rbdc::Error::from("sql_where can't be empty!"));
                }
                #[$crate::py_sql("`delete from ${table_name} `",$sql_where)]
                async fn $fn_name$(<$($gkey: $gtype,)*>)?(
                    executor: &mut dyn $crate::executor::Executor,
                    table_name: String,
                    $($param_key:$param_type,)*
                ) -> std::result::Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                    impled!()
                }
                let mut table_name = String::new();
                $(table_name = $table_name.to_string();)?
                if table_name.is_empty(){
                  table_name = $crate::utils::string_util::to_snake_name(stringify!($table));
                }
                $fn_name(executor, table_name, $($param_key,)*).await
            }
        }
    };
}

/// pysql impl_select_page
/// If the database does not support the statement `limit ${page_no},${page_size}`,You should include ${page_no} and ${page_size} in SQL
///
/// ```rust
/// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// pub struct BizActivity{}
/// rbatis::impl_select_page!(BizActivity{select_page() =>"
///      if !sql.contains('count'):
///        `order by create_time desc`"});
/// ```
#[macro_export]
macro_rules! impl_select_page {
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $where_sql:expr}) => {
        $crate::impl_select_page!(
            $table{$fn_name($($param_key:$param_type)*)=> $where_sql},
            $crate::utils::string_util::to_snake_name(stringify!($table))
        );
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $where_sql:expr},$table_name:expr) => {
        impl $table {
            pub async fn $fn_name(
                executor: &mut dyn $crate::executor::Executor,
                page_req: &$crate::sql::PageRequest,
                $($param_key:$param_type,)*
            ) -> std::result::Result<$crate::sql::Page::<$table>, $crate::rbdc::Error> {
                use $crate::sql::IPageRequest;
                let mut table_column = "*".to_string();
                let mut table_name = $table_name.to_string();
                //pg,mssql can override this parameter to implement its own limit statement
                let mut limit_sql = " limit ${page_no},${page_size}".to_string();
                limit_sql=limit_sql.replace("${page_no}",&page_req.offset().to_string());
                limit_sql=limit_sql.replace("${page_size}",&page_req.page_size.to_string());
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
                    if !sql.contains('page_no') && !sql.contains('page_size'):
                        `${limit_sql}`")]
                   async fn $fn_name(executor: &mut dyn $crate::executor::Executor,do_count:bool,table_column:&str,table_name: &str,page_no:u64,page_size:u64,page_offset:u64,limit_sql:&str,$($param_key:$param_type,)*) -> std::result::Result<rbs::Value, $crate::rbdc::Error> {impled!()}
                }
                let totalValue = Inner::$fn_name(executor,true,&table_column,&table_name,page_req.page_no, page_req.page_size,page_req.offset(),"",$($param_key,)*).await?;
                let recordsValue = Inner::$fn_name(executor,false,&table_column,&table_name,page_req.page_no, page_req.page_size,page_req.offset(),&limit_sql,$($param_key,)*).await?;
                let total =  $crate::decode(totalValue)?;
                let records = rbs::from_value(recordsValue)?;
                let mut page = $crate::sql::Page::<$table>::new_total(page_req.offset(), page_req.page_size, total);
                let mut page = $crate::sql::Page::<$table>::new_total(page_req.page_no, page_req.page_size, total);
                page.records = records;
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
/// just like this exmaple:
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
/// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// pub struct BizActivity{}
/// //rbatis::htmlsql_select_page!(select_page_data(name: &str) -> BizActivity => "example.html");
/// rbatis::htmlsql_select_page!(select_page_data(name: &str) -> BizActivity => r#"<select id="select_page_data">`select `<if test="do_count == true">`count(1) from table`</if><if test="do_count == false">`* from table limit ${page_no},${page_size}`</if></select>"#);
/// ```
#[macro_export]
macro_rules! htmlsql_select_page {
    ($fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $table:ty => $html_file:expr) => {
            pub async fn $fn_name(executor: &mut dyn $crate::executor::Executor, page_req: &$crate::sql::PageRequest, $($param_key:$param_type,)*) -> std::result::Result<$crate::sql::Page<$table>, $crate::rbdc::Error> {
            use $crate::sql::IPageRequest;
            struct Inner{}
            impl Inner{
              #[$crate::html_sql($html_file)]
              pub async fn $fn_name(executor: &mut dyn $crate::executor::Executor,do_count:bool,page_no:u64,page_size:u64,$($param_key:$param_type,)*) -> std::result::Result<rbs::Value, $crate::rbdc::Error>{
                 $crate::impled!()
              }
            }
            let totalValue = Inner::$fn_name(executor, true, page_req.offset(), page_req.page_size, $($param_key,)*).await?;
            let recordsValue = Inner::$fn_name(executor, false, page_req.offset(), page_req.page_size, $($param_key,)*).await?;
            let total =  $crate::decode(totalValue)?;
            let records = rbs::from_value(recordsValue)?;
            let mut page = $crate::sql::Page::<$table>::new_total(page_req.offset(), page_req.page_size, total);
            page.records = records;
            Ok(page)
         }
    }
}
