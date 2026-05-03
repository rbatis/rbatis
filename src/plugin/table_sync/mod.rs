pub mod deprecated;
pub mod rbdc_mapper;

pub use deprecated::*;

use crate::executor::Executor;
use crate::Error;
use futures_core::future::BoxFuture;
use log::debug;
use rbs::Value;

const PRIMARY_KEY: &str = " PRIMARY KEY ";

/// create table if not exists, add column if not exists
/// ```rust
/// use rbatis::executor::{Executor, RBatisRef};
/// use rbatis::RBatis;
/// use rbatis::table_sync::sync;
///
/// /// let rb = RBatis::new();
/// /// let conn = rb.acquire().await;
/// pub async fn do_sync_table(conn: &dyn Executor){
///     let map = rbs::value!{
///             "id":"TEXT",
///             "name":"TEXT",
///      };
///      let _ = sync(conn, conn.rb_ref(),map,"user").await;
/// }
///
/// ```
///
/// sync table struct
/// ```rust
/// use rbatis::executor::{Executor, RBatisRef};
/// use rbatis::RBatis;
/// use rbatis::table_sync::sync;
/// use rbs::value;
///
/// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// pub struct User{
///   pub id:String,
///   pub name: Option<String>
/// }
///
/// /// let rb = RBatis::new();
/// /// let conn = rb.acquire().await;
/// pub async fn do_sync_table(conn: &dyn Executor){
///      let table = User{id: "".to_string(), name: Some("".to_string())};
///      let _ = sync(conn, conn.rb_ref(),value!(table),"user").await;
/// }
///
/// ```
///
/// sync table struct (custom string column type)
/// ```rust
/// use rbatis::executor::Executor;
/// use rbatis::RBatis;
/// use rbatis::table_sync::sync;
/// use rbs::value;
///
/// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// pub struct User{
///   pub id:String,
///   pub name: Option<String>
/// }
///
/// pub async fn do_sync_table_mysql(conn: &dyn Executor){
///      let table = User{id: "".to_string(), name: Some("VARCHAR(50)".to_string())};
///      let _ = sync(conn, conn.rb_ref(),value!(table),"user").await;
/// }
/// ```
pub fn sync<'a>(
    executor: &'a dyn Executor,
    mapper: &'a dyn ColumnMapper,
    table: Value,
    table_name: &str,
) -> BoxFuture<'a, Result<(), Error>> {
    let name = table_name.to_owned();
    Box::pin(async move {
        match table {
            Value::Map(m) => {
                let db_driver_type = executor.driver_type()?;
                if db_driver_type != mapper.driver_type() {
                    return Err(Error::from(format!(
                        "table sync mapper driver='{}',db driver='{}'",
                        mapper.driver_type(),
                        db_driver_type
                    )));
                }
                let mut sql_create = format!("CREATE TABLE {} ", name);
                let mut sql_column = String::new();
                for (k, v) in &m {
                    let k = k.as_str().unwrap_or_default();
                    let column_type_value = mapper.get_column_type(k, v);
                    sql_column.push_str(k);
                    sql_column.push(' ');
                    sql_column.push_str(column_type_value.as_str());
                    if column_type_value.is_empty() && k.eq("id")
                        || v.as_str().unwrap_or_default() == "id"
                    {
                        sql_column.push_str(PRIMARY_KEY);
                    }
                    sql_column.push(',');
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
                                    id_key = PRIMARY_KEY;
                                }
                                let column_type = mapper.get_column_type(k, v);
                                match executor
                                    .exec(
                                        &format!(
                                            "alter table {} add {} {} {};",
                                            name, k, column_type, id_key
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

/// Mapper Column and ColumnType
pub trait ColumnMapper: Sync + Send {
    fn driver_type(&self) -> String;

    /// for example input `"id":i32` -> id:INT
    fn get_column_type(&self, field: &str, v: &Value) -> String;
}
