use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use crate::crud::CRUDEntity;
use std::ops::Add;

//TODO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wrapper {
    pub sql: String,
    pub args: Vec<serde_json::Value>,
    pub where_num: i32,
}

impl Wrapper {
    pub fn new() -> Self {
        Self {
            sql: format!("WHERE "),
            args: vec![],
            where_num: 0
        }
    }

    pub fn all_eq<T>(&mut self, arg: &T) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(arg).unwrap();
        if !v.is_object() {
            panic!("[rbatis] wrapper all_eq only support object struct!")
        }
        let map = v.as_object().unwrap();
        let len = map.len();
        let mut index = 0;
        for (k, v) in map {
            self.eq(k.as_str(), v);
            if (index + 1) != len {
                self.sql.push_str(" , ");
                index += 1;
            }
        }
        self.where_num += 1;

        self
    }

    pub fn eq<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        if self.where_num != 0 {
            self.sql.push_str(" AND ");
        }

        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" = ?");
        self.args.push(v);
        self.where_num += 1;

        self
    }

    pub fn order_by(&mut self, is_asc: bool, columns: &[&str]) -> &mut Self {
        let len = columns.len();
        let mut index = 0;
        if len == 0 {
            return self;
        }
        self.sql = self.sql.trim_end_matches("WHERE ").to_string();
        self.sql.push_str(" ORDER BY ");
        for x in columns {
            if is_asc {
                self.sql.push_str(format!("{} ASC", x).as_str());
            } else {
                self.sql.push_str(format!("{} DESC", x, ).as_str());
            }
            if (index + 1) != len {
                self.sql.push_str(" , ");
                index += 1;
            }
        }
        self
    }
}

mod test {
    use crate::wrapper::Wrapper;
    use serde_json::Map;
    use serde_json::json;

    #[test]
    fn test_select() {
        let mut w = Wrapper::new();
        let mut m = Map::new();
        m.insert("a".to_string(), json!("1"));
        w.eq("id", 1).all_eq(&m).order_by(true, &["id", "name"]);
        println!("{:?}", w);
    }
}