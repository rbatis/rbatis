use serde::export::fmt::Debug;
use serde_json::Value;

use rbatis_core::Error;

use crate::crud::CRUDEnable;
use crate::rbatis::Rbatis;
use crate::core::convert::StmtConvert;

/// sql intercept
pub trait SqlIntercept: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    /// do intercept sql/args
    /// is_prepared_sql: if is run in prepared_sql=ture
    fn do_intercept(&self, rb: &Rbatis, sql: &mut String, args: &mut Vec<serde_json::Value>, is_prepared_sql: bool) -> Result<(), crate::core::Error>;
}


///dyn_table(old_table,new_table)
#[derive(Debug, Clone)]
pub struct RbatisDynTableNameIntercept {}

impl RbatisDynTableNameIntercept {
    pub fn dyn_table_arg_str(old_table: &str, new_table: &str) -> String {
        return format!("dyn_table({},{})", old_table, new_table);
    }
    pub fn dyn_table_arg<T>(new_table: &str) -> String where T: CRUDEnable {
        return format!("dyn_table({},{})", T::table_name(), new_table);
    }
}

impl SqlIntercept for RbatisDynTableNameIntercept {
    fn do_intercept(&self, rb: &Rbatis, sql: &mut String, args: &mut Vec<Value>, is_prepared_sql: bool) -> Result<(), Error> {
        let mut index=-1;
        for x in args {
            index+=1;
            if x.is_string() {
                let x = x.as_str().unwrap();
                if x.starts_with("dyn_table(") && x.ends_with(")") && x.contains(",") {
                    let replace = x["dyn_table(".len()..(x.len() - 1) as usize].to_string();
                    let sp: Vec<&str> = replace.split(",").collect();
                    if sp.len() != 2 {
                        continue;
                    }
                    *sql = sql.replace(sp[0], sp[1]);
                    //TODO delete '?','$1'
                    //sql.rfind(rb.driver_type()?.stmt_convert(index))
                }
            }
        }
        return Ok(());
    }
}
