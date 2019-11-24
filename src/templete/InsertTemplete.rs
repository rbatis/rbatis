use serde_json::Value;
use serde_json::json;

pub struct InsertTemplete {}

impl InsertTemplete {
    pub fn doCreateSql(&self, table: &str, arg: Value) -> Option<String> {
        let mut sql = "insert into #{table} (#{fields}) VALUES (#{values})".to_string();
        sql = sql.replace("#{table}", table);
        let obj = arg.as_object();
        if obj.is_none() {
            return Option::None;
        }
        let obj_map = obj.unwrap();

        let mut fields = "".to_string();
        let mut values = "".to_string();

        let len = obj_map.len();
        let mut i = 0;
        for (x, v) in obj_map {
            let vstr: String;
            if v.is_null() {
                vstr = "null".to_string();
            } else if v.is_string() {
                vstr = "'".to_string() + v.as_str().unwrap_or("null") + "'";
            } else if v.is_number() {
                let number = v.as_f64().unwrap();
                vstr = number.to_string();
            } else {
                vstr = "null".to_string();
            }
            if i < (len - 1) {
                fields = fields + x.as_str() + ",";
                values = values + vstr.as_str() + ",";
            } else {
                fields = fields + x.as_str();
                values = values + vstr.as_str();
            }
            i = i + 1;
        }
        sql = sql.replace("#{fields}", fields.as_str());
        sql = sql.replace("#{values}", values.as_str());
        return Option::Some(sql);
    }
}

#[test]
fn test_insert_templete() {
    let t = InsertTemplete {};
    let arg = serde_json::from_str(r#"{  "a":"1","delete_flag":1}"#).unwrap();
    let sql = t.doCreateSql("activity", arg).unwrap();
    println!("{}", sql);
}