use serde_json::Value;
use crate::core::rbatis::Rbatis;

pub struct Select {}

impl Select {
    pub fn select(&self, table: &str, arg: Value, engine: &Rbatis) -> Result<String, String> {
        //TODO select by id
        //TODO select by map
        //TODO select by ids
        //TODO select by page
        if arg.is_null() {
            return Result::Err("[rbatis] arg is null value".to_string());
        }
        //TODO select by id
        if arg.is_string() || arg.is_i64() {
            let select_by_id = do_select_by_id(table, arg)?;
            return Result::Ok(select_by_id);
        }
        //TODO select by id vec
        if arg.is_array() {}
        if arg.is_object() {
            //TODO select by map

            //TODO select by page
        }
        return Result::Err("[rbatis] eval select crud fail".to_string());
    }
}

fn do_select_by_id(table: &str, arg: Value) -> Result<String, String> {
    let mut sql = "select * from #{table}".to_string();
    sql = sql.replace("#{table}", table);

    return Result::Err("[rbatis] do_select_by_id fail!".to_string());
}
