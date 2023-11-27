pub mod mssql_mapper;
pub mod mysql_mapper;
pub mod pg_mapper;
pub mod sqlite_mapper;

use crate::executor::Executor;
use crate::Error;
use futures_core::future::BoxFuture;
use log::debug;
pub use mssql_mapper::*;
pub use mysql_mapper::*;
pub use pg_mapper::*;
use rbs::Value;
pub use sqlite_mapper::*;

const PRIMARY_KEY: &'static str = " PRIMARY KEY ";

/// create table if not exists, add column if not exists
/// ```rust
/// use rbatis::executor::RBatisConnExecutor;
/// use rbatis::RBatis;
/// use rbatis::table_sync::{MysqlTableMapper, SqliteTableMapper, sync};
/// use rbs::to_value;
///
/// /// let rb = RBatis::new();
/// /// let conn = rb.acquire().await;
/// pub async fn do_sync_table(conn: &RBatisConnExecutor){
///     let map = rbs::to_value!{
///             "id":"TEXT",
///             "name":"TEXT",
///      };
///      let _ = sync(conn, &SqliteTableMapper{},map,"user").await;
/// }
///
/// ```
/// ```rust
/// use rbatis::executor::RBatisConnExecutor;
/// use rbatis::RBatis;
/// use rbatis::table_sync::{MysqlTableMapper, SqliteTableMapper, sync};
/// use rbs::to_value;
///
/// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// pub struct User{
///   pub id:String,
///   pub name: Option<String>
/// }
///
/// /// let rb = RBatis::new();
/// /// let conn = rb.acquire().await;
/// pub async fn do_sync_table(conn: &RBatisConnExecutor){
///      let table = User{id: "".to_string(), name: Some("".to_string())};
///      let _ = sync(conn, &SqliteTableMapper{},to_value!(table),"user").await;
/// }
///
/// ```
/// ```rust
/// use rbatis::executor::RBatisConnExecutor;
/// use rbatis::RBatis;
/// use rbatis::table_sync::{MysqlTableMapper, sync};
/// use rbs::to_value;
///
/// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// pub struct User{
///   pub id:String,
///   pub name: Option<String>
/// }
///
/// pub async fn do_sync_table_mysql(conn: &RBatisConnExecutor){
///      let table = User{id: "".to_string(), name: Some("VARCHAR(50)".to_string())};
///      let _ = sync(conn, &MysqlTableMapper{},to_value!(table),"user").await;
/// }
/// ```
pub fn sync<'a>(
    executor: &'a dyn Executor,
    mapper: &'a dyn ColumMapper,
    table: Value,
    table_name: &str,
) -> BoxFuture<'a, Result<(), Error>> {
    let name = table_name.to_owned();
    Box::pin(async move {
        match table {
            Value::Map(m) => {
                let mut sql_create = format!("CREATE TABLE {} ", name);
                let mut sql_column = format!("");
                for (k, v) in &m {
                    let k = k.as_str().unwrap_or_default();
                    sql_column.push_str(k);
                    sql_column.push_str(" ");
                    sql_column.push_str(&mapper.get_column(k, &v));
                    if k.eq("id") || v.as_str().unwrap_or_default() == "id" {
                        sql_column.push_str(&PRIMARY_KEY);
                    }
                    sql_column.push_str(",");
                }
                if sql_column.ends_with(",") {
                    sql_column = sql_column.trim_end_matches(",").to_string();
                }
                sql_create = sql_create + &format!("({});", sql_column);
                let result_create = executor.exec(&sql_create, vec![]).await;
                match result_create {
                    Ok(_) => {}
                    Err(e) => {
                        if e.to_string().to_lowercase().contains("already") {
                            //TODO have any better way do not Repeated add and compatibility with most databases
                            for (k, v) in &m {
                                let k = k.as_str().unwrap_or_default();
                                let mut id_key = "";
                                if k.eq("id") || v.as_str().unwrap_or_default() == "id" {
                                    id_key = &PRIMARY_KEY;
                                }
                                match executor
                                    .exec(
                                        &format!(
                                            "alter table {} add {} {} {};",
                                            name,
                                            k,
                                            mapper.get_column(k, &v),
                                            id_key
                                        ),
                                        vec![],
                                    )
                                    .await
                                {
                                    Ok(_) => {}
                                    Err(e) => {
                                        debug!("ADD COLUMN fail={}", e);
                                        continue;
                                    }
                                }
                            }
                            return Ok(());
                        }
                        return Err(e);
                    }
                }
                Ok(())
            }
            _ => Err(Error::from("table not is an struct or map!")),
        }
    })
}

pub trait ColumMapper: Sync + Send {
    fn get_column(&self, column: &str, v: &Value) -> String;
}
