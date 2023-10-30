pub mod sqlite_mapper;
pub mod mysql_mapper;
pub mod pg_mapper;
pub mod mssql_mapper;

use crate::executor::RBatisConnExecutor;
use crate::Error;
use futures_core::future::BoxFuture;
use log::debug;
use rbs::Value;
pub use sqlite_mapper::*;
pub use mysql_mapper::*;
pub use pg_mapper::*;
pub use mssql_mapper::*;


const PRIMARY_KEY: &'static str = " PRIMARY KEY ";


/// create table if not exists, add column if not exists
/// ```rust
/// use rbatis::Error;
/// use rbatis::executor::RBatisConnExecutor;
/// use rbatis::RBatis;
/// use rbatis::table_sync::{SqliteTableMapper, sync};
/// use rbs::to_value;
///
/// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// pub struct User{
///   pub id:String,
///   pub name: Option<String>
/// }
///
/// /// use rbs::to_value;
/// /// let rb = RBatis::new();
/// /// let conn = rb.acquire().await;
/// pub async fn do_sync_table(conn: &RBatisConnExecutor){
///      let table = User{id: "1".to_string(), name: Some("".to_string())};
///      sync(conn, &SqliteTableMapper{},to_value!(table),"user").await;
/// }
/// ```
pub fn sync<'a>(
    rb: &'a RBatisConnExecutor,
    mapper: &'a dyn ColumMapper,
    table: Value,
    name: &str,
) -> BoxFuture<'a, Result<(), Error>> {
    let name = name.to_owned();
    Box::pin(async move {
        match table {
            Value::Map(m) => {
                let mut sql_create = format!("CREATE TABLE {} ", name);
                let mut sql_column = format!("");
                for (k, v) in &m {
                    let k = k.as_str().unwrap_or_default();
                    sql_column.push_str(k);
                    sql_column.push_str(" ");
                    sql_column.push_str(mapper.get_column(&v));
                    if k.eq("id") || v.as_str().unwrap_or_default() == "id" {
                        sql_column.push_str(&PRIMARY_KEY);
                    }
                    sql_column.push_str(",");
                }
                if sql_column.ends_with(",") {
                    sql_column = sql_column.trim_end_matches(",").to_string();
                }
                sql_create = sql_create + &format!("({});", sql_column);
                let result_create = rb.exec(&sql_create, vec![]).await;
                match result_create {
                    Ok(_) => {}
                    Err(e) => {
                        if e.to_string().to_lowercase().contains("already") {
                            for (k, v) in &m {
                                let k = k.as_str().unwrap_or_default();
                                let mut id_key = "";
                                if k.eq("id") || v.as_str().unwrap_or_default() == "id" {
                                    id_key = &PRIMARY_KEY;
                                }
                                match rb
                                    .exec(
                                        &format!(
                                            "alter table {} add {} {} {};",
                                            name,
                                            k,
                                            mapper.get_column(&v),
                                            id_key
                                        ),
                                        vec![],
                                    )
                                    .await
                                {
                                    Ok(_) => {}
                                    Err(e) => {
                                        debug!("ADD COLUMN fail={}",e);
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
    fn get_column(&self, v: &Value) -> &'static str;
}