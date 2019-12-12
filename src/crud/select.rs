use serde_json::Value;

pub struct Select {}

impl Select {
    pub fn eval(&self, table: &str, arg: Value) -> Result<String, String> {
        unimplemented!();
        //TODO select by id
        if arg.is_string() || arg.is_i64() {
            let select_by_id = do_select_by_id(table, arg)?;
            return Result::Ok(select_by_id);
        }
        //TODO select by id vec
        if arg.is_array(){

        }
        if arg.is_object(){
            //TODO select by map

            //TODO select by page

        }
        return Result::Err("[rbatis] eval select crud fail".to_string());
    }
}

fn do_select_by_id(table: &str, arg: Value) -> Result<String, String> {
    let mut sql = "insert into #{table} (#{fields}) VALUES #{values}".to_string();
    sql = sql.replace("#{table}", table);

    if arg.is_null() {
        return Result::Err("[rbatis] arg is null value".to_string());
    }
    if arg.is_object() {}
    return Result::Err("[rbatis] do_select_by_id fail!".to_string());
}
