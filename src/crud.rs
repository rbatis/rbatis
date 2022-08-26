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
                rb: &mut dyn $crate::executor::Executor,
                tables: &[$table],
                batch_size: u64,
            ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                #[$crate::py_sql(
                    "`insert into ${table_name} (`
             trim ',':
               for k,v in tables[0]:
                  if k == 'id' && v== null:
                    continue:
                 ${k},
             `) VALUES `
             trim ',':
              for _,table in tables:
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
                    rb: &mut dyn $crate::executor::Executor,
                    tables: &[$table],
                    table_name: &str,
                ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                    impled!()
                }
                if tables.is_empty() {
                    return Err($crate::rbdc::Error::from(
                        "insert can not insert empty array tables!",
                    ));
                }
                let table_name = $table_name.to_string();
                let mut result = $crate::rbdc::db::ExecResult{
                     rows_affected: 0,
                     last_insert_id: rbs::Value::Null,
                };
                let ranges = $crate::sql::Page::<()>::into_ranges(tables.len() as u64, batch_size);
                for (offset,limit) in ranges {
                    let exec_result = insert_batch(rb, &tables[offset as usize..limit as usize], table_name.as_str()).await?;
                    result.rows_affected += exec_result.rows_affected;
                    result.last_insert_id = exec_result.last_insert_id;
                }
                Ok(result)
            }

            pub async fn insert(
                rb: &mut dyn $crate::executor::Executor,
                table: &$table,
            ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                <$table>::insert_batch(rb, &[table.clone()], 1).await
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
        impl $table{
            pub async fn select_all(rb: &mut dyn  $crate::executor::Executor)->Result<Vec<$table>,$crate::rbdc::Error>{
                #[$crate::py_sql("select * from ${table_name}")]
                async fn select_all(rb: &mut dyn $crate::executor::Executor,table_name:String) -> Result<Vec<$table>,$crate::rbdc::Error> {impled!()}
                let table_name = $table_name.to_string();
                select_all(rb,table_name).await
            }

            pub async fn select_by_column<V:serde::Serialize>(rb: &mut dyn  $crate::executor::Executor, column: &str,column_value:V)->Result<Vec<$table>,$crate::rbdc::Error>{
                #[$crate::py_sql("select * from ${table_name} where ${column} = #{column_value}")]
                async fn select_by_column(rb: &mut dyn $crate::executor::Executor,table_name:String, column:&str, column_value: &rbs::Value) -> Result<Vec<$table>,$crate::rbdc::Error> {impled!()}
                let table_name = $table_name.to_string();
                let column_value = rbs::to_value!(column_value);
                select_by_column(rb,table_name,column,&column_value).await
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql:expr}) => {
        impl $table{
            pub async fn $fn_name(rb: &mut dyn  $crate::executor::Executor,$($param_key:$param_type,)*)->Result<Vec<$table>,$crate::rbdc::Error>{
                   #[$crate::py_sql("`select ${table_column} from ${table_name} `",$sql)]
                   async fn $fn_name(rb: &mut dyn $crate::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<Vec<$table>,$crate::rbdc::Error> {impled!()}
                   let mut table_column = "*".to_string();
                   let mut table_name = $crate::utils::string_util::to_snake_name(stringify!($table));
                   $fn_name(rb,&table_column,&table_name,$($param_key ,)*).await
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $container:tt => $sql:expr}) => {
        impl $table{
            pub async fn $fn_name(rb: &mut dyn  $crate::executor::Executor,$($param_key:$param_type,)*)->Result<$container<$table>,$crate::rbdc::Error>{
                     #[$crate::py_sql("`select ${table_column} from ${table_name} `",$sql)]
                     async fn $fn_name(rb: &mut dyn $crate::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<$container<$table>,$crate::rbdc::Error> {impled!()}
                     let mut table_column = "*".to_string();
                     let mut table_name = $crate::utils::string_util::to_snake_name(stringify!($table));
                     $fn_name(rb,&table_column,&table_name,$($param_key ,)*).await
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
        impl $table {
            pub async fn update_by_column(
                rb: &mut dyn $crate::executor::Executor,
                table: &$table,
                column: &str,
            ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                #[$crate::py_sql(
                    "`update ${table_name} set `
             trim ',':
               for k,v in table:
                  if k == column || v== null:
                    continue:
                 `${k}=#{v},`
             ` where  ${column} = #{column_value}`"
                )]
                async fn update_by_column(
                    rb: &mut dyn $crate::executor::Executor,
                    table_name: String,
                    table: &rbs::Value,
                    column_value: &rbs::Value,
                    column: &str,
                ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                    impled!()
                }
                let table_name = $table_name.to_string();
                let table = rbs::to_value!(table);
                let column_value = &table[column];
                update_by_column(rb, table_name, &table, column_value, column).await
            }
            pub async fn update_by_column_batch(
                rb: &mut dyn $crate::executor::Executor,
                tables: &[$table],
                column: &str,
            ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                let mut rows_affected = 0;
                for item in tables{
                    rows_affected += <$table>::update_by_column(rb,item,column).await?.rows_affected;
                }
                Ok($crate::rbdc::db::ExecResult{
                    rows_affected:rows_affected,
                    last_insert_id:rbs::Value::Null
                })
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}) => {
        impl $table {
            pub async fn $fn_name(
                rb: &mut dyn $crate::executor::Executor,
                table: &$table,
                $($param_key:$param_type,)*
            ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                if $sql_where.is_empty(){
                    return Err($crate::rbdc::Error::from("sql_where can't be empty!"));
                }
                #[$crate::py_sql("`update ${table_name} set  `
                                 trim ',':
                                   for k,v in table:
                                     if k == column || v== null:
                                        continue:
                                     `${k}=#{v},`
                                 ` `",$sql_where)]
                  async fn $fn_name(
                      rb: &mut dyn $crate::executor::Executor,
                      table_name: String,
                      table: &rbs::Value,
                      $($param_key:$param_type,)*
                  ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                      impled!()
                  }
                  let mut table_name = $crate::utils::string_util::to_snake_name(stringify!($table));
                  let table = rbs::to_value!(table);
                  $fn_name(rb, table_name, &table, $($param_key,)*).await
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
        impl $table {
            pub async fn delete_by_column<V:serde::Serialize>(
                rb: &mut dyn $crate::executor::Executor,
                column: &str,
                column_value: V,
            ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                #[$crate::py_sql("`delete from ${table_name} where  ${column} = #{column_value}`")]
                async fn delete_by_column(
                    rb: &mut dyn $crate::executor::Executor,
                    table_name: String,
                    column_value: &rbs::Value,
                    column: &str,
                ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                    impled!()
                }
                let column_value = rbs::to_value!(column_value);
                let table_name = $table_name.to_string();
                delete_by_column(rb, table_name, &column_value, column).await
            }
            pub async fn delete_by_column_batch<V:serde::Serialize>(
                rb: &mut dyn $crate::executor::Executor,
                column: &str,
                column_values: &[V],
            ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                 #[$crate::py_sql("`delete from ${table_name} where  ${column} in (`
                                       trim ',':
                                         for _,v in column_values:
                                            #{v},
                                       `)`")]
                async fn delete_by_column_batch(
                    rb: &mut dyn $crate::executor::Executor,
                    table_name: String,
                    column_values: rbs::Value,
                    column: &str,
                ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                    impled!()
                }
                let column_values = rbs::to_value!(column_values);
                let table_name = $table_name.to_string();
                delete_by_column_batch(rb, table_name, column_values, column).await
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}) => {
        impl $table {
            pub async fn $fn_name(
                rb: &mut dyn $crate::executor::Executor,
                $($param_key:$param_type,)*
            ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                if $sql_where.is_empty(){
                    return Err($crate::rbdc::Error::from("sql_where can't be empty!"));
                }
                #[$crate::py_sql("`delete from ${table_name} `",$sql_where)]
                async fn $fn_name(
                    rb: &mut dyn $crate::executor::Executor,
                    table_name: String,
                    $($param_key:$param_type,)*
                ) -> Result<$crate::rbdc::db::ExecResult, $crate::rbdc::Error> {
                    impled!()
                }
                let mut table_name = $crate::utils::string_util::to_snake_name(stringify!($table));
                $fn_name(rb, table_name, $($param_key,)*).await
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
                rb: &mut dyn $crate::executor::Executor,
                page_req: &$crate::sql::PageRequest,
                $($param_key:$param_type,)*
            ) -> Result<$crate::sql::Page::<$table>, $crate::rbdc::Error> {
                use $crate::sql::IPageRequest;
                let mut table_column = "*".to_string();
                let mut table_name = $table_name.to_string();
                let mut total = 0;
                {
                   #[$crate::py_sql("`select count(1) as count from ${table_name} `",$where_sql)]
                   async fn $fn_name(rb: &mut dyn $crate::executor::Executor,table_column:&str,table_name: &str,$($param_key:$param_type,)*) -> Result<u64, $crate::rbdc::Error> {impled!()}
                   total = $fn_name(rb, &table_column,&table_name, $($param_key,)*).await?;
                }
                let records:Vec<$table>;
                #[$crate::py_sql("`select ${table_column} from ${table_name} `",$where_sql,"
                              if !sql.contains('page_no') && !sql.contains('page_size'):
                                ` limit ${page_no},${page_size}`")]
                async fn $fn_name(rb: &mut dyn $crate::executor::Executor,table_column:&str,table_name: &str,page_no:u64,page_size:u64,$($param_key:$param_type,)*) -> Result<Vec<$table>, $crate::rbdc::Error> {impled!()}
                records = $fn_name(rb,&table_column,&table_name,page_req.offset(), page_req.page_size,$($param_key,)*).await?;

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
/// rbatis::htmlsql_select_page!(select_page_data(name: &str) -> BizActivity => "example/example.html");
/// ```
#[macro_export]
macro_rules! htmlsql_select_page {
    ($fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $table:ty => $html_file:expr) => {
            pub async fn $fn_name(rb: &mut dyn $crate::executor::Executor, page_req: &$crate::sql::PageRequest, $($param_key:$param_type,)*) -> Result<$crate::sql::Page<$table>, $crate::rbdc::Error> {
            use $crate::sql::IPageRequest;
            let mut total = 0;
            {
              #[$crate::html_sql($html_file)]
              pub async fn $fn_name(rb: &mut dyn $crate::executor::Executor,do_count:bool,page_no:u64,page_size:u64,$($param_key:$param_type,)*) -> Result<u64, $crate::rbdc::Error>{
                 $crate::impled!()
              }
              total = $fn_name(rb, true, page_req.offset(), page_req.page_size, $($param_key,)*).await?;
            }

            #[$crate::html_sql($html_file)]
            pub async fn $fn_name(rb: &mut dyn $crate::executor::Executor,do_count:bool,page_no:u64,page_size:u64,$($param_key:$param_type,)*) -> Result<Vec<$table>, $crate::rbdc::Error>{
                $crate::impled!()
            }
            let records = $fn_name(rb, false, page_req.offset(), page_req.page_size, $($param_key,)*).await?;
            let mut page = $crate::sql::Page::<$table>::new_total(page_req.offset(), page_req.page_size, total);
            page.records = records;
            Ok(page)
         }
    }
}