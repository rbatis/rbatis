use serde_json::Value;
use serde_json::json;

pub struct Insert {}

impl Insert {
    fn do_create_values_sql(&self, arg: &Value) -> String {
        let obj = arg.as_object();
        let obj_map = obj.unwrap();
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
                values = values + vstr.as_str() + ",";
            } else {
                values = values + vstr.as_str();
            }
            i = i + 1;
        }
        return values;
    }

    //todo replace to xml field
    fn do_create_field_sql(&self, arg: Value) -> String {
        let obj = arg.as_object();
        let obj_map = obj.unwrap();
        let mut fields = "".to_string();
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
            } else {
                fields = fields + x.as_str();
            }
            i = i + 1;
        }
        return fields;
    }

    fn do_create_obj_sql(&self, mut sql: String, arg: Value) -> String {
        let obj = arg.as_object();
        let obj_map = obj.unwrap();
        //todo replace to xml field
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
        sql = sql.replace("#{values}", ("(".to_string() + values.as_str() + ")").as_str());
        return sql;
    }


    pub fn eval(&self, table: &str, arg: Value) -> Option<String> {
        let mut sql = "insert into #{table} (#{fields}) VALUES #{values}".to_string();
        sql = sql.replace("#{table}", table);

        if arg.is_null() {
            return Option::None;
        }
        if arg.is_object() {
            return Option::Some(self.do_create_obj_sql(sql, arg));
        } else if arg.is_array() {
            let mut values = "".to_string();
            let arr = arg.as_array().unwrap();
            if arr.len() == 0 {
                return Option::None;
            }
            //todo replace to xml field
            let fields = self.do_create_field_sql(arr.get(0).unwrap().clone());
            sql = sql.replace("#{fields}", fields.as_str());

            for x in arr {
                let mut value_item_sql = self.do_create_values_sql(x);
                value_item_sql = "(".to_string() + value_item_sql.as_str() + "),";
                values.push_str(value_item_sql.as_str());
            }
            values.pop();
            sql = sql.replace("#{values}", values.as_str());
            return Option::Some(sql);
        } else {
            return Option::None;
        }
    }
}

#[test]
fn test_insert_templete_obj() {
    let t = Insert {};
    let arg = serde_json::from_str(r#"{  "a":"1","delete_flag":1}"#).unwrap();
    let sql = t.eval("activity", arg).unwrap();
    println!("{}", sql);
}

#[test]
fn test_insert_templete_array() {
    let t = Insert {};
    let arg = serde_json::from_str(r#"[{"a":"1","delete_flag":1}]"#).unwrap();
    let sql = t.eval("activity", arg).unwrap();
    println!("{}", sql);
}