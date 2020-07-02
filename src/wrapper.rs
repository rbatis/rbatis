use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use crate::crud::CRUDEntity;
use std::ops::Add;

//TODO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wrapper {
    pub sql: String,
    pub args: Vec<serde_json::Value>,
}

impl Wrapper {
    pub fn new() -> Self {
        Self {
            sql: "".to_string(),
            args: vec![],
        }
    }

    pub fn and(&mut self) -> &mut Self {
        self.sql.push_str(" AND ");
        self
    }

    pub fn or(&mut self) -> &mut Self {
        self.sql.push_str(" OR ");
        self
    }

    pub fn having(&mut self, sql_having: &str) -> &mut Self {
        self.sql.push_str(format!(" HAVING {} ", sql_having).as_str());

        self
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
        self
    }

    pub fn eq<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" = ?");
        self.args.push(v);


        self
    }

    /// not equal
    pub fn ne<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" <> ?");
        self.args.push(v);


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

    pub fn group_by(&mut self, columns: &[&str]) -> &mut Self {
        let len = columns.len();
        let mut index = 0;
        if len == 0 {
            return self;
        }
        self.sql = self.sql.trim_end_matches("WHERE ").to_string();
        self.sql.push_str(" GROUP BY ");
        for x in columns {
            self.sql.push_str(x);
            if (index + 1) != len {
                self.sql.push_str(" , ");
                index += 1;
            }
        }
        self
    }

    ///  sql:   column > obj
    pub fn gt<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" > ?");
        self.args.push(v);
        self
    }
    ///  sql:   column >= obj
    pub fn ge<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" >= ?");
        self.args.push(v);


        self
    }

    ///  sql:   column < obj
    pub fn lt<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" < ?");
        self.args.push(v);

        self
    }

    ///  sql:   column <= obj
    pub fn le<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" <= ?");
        self.args.push(v);


        self
    }

    pub fn between<T>(&mut self, column: &str, min: T, max: T) -> &mut Self
        where T: Serialize {
        let min_v = serde_json::to_value(min).unwrap();
        let max_v = serde_json::to_value(max).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" BETWEEN ? AND ?");
        self.args.push(min_v);
        self.args.push(max_v);


        self
    }

    pub fn not_between<T>(&mut self, column: &str, min: T, max: T) -> &mut Self
        where T: Serialize {
        let min_v = serde_json::to_value(min).unwrap();
        let max_v = serde_json::to_value(max).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" NOT BETWEEN ? AND ?");
        self.args.push(min_v);
        self.args.push(max_v);

        self
    }

    pub fn like<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" LIKE '%?%'");
        self.args.push(v);

        self
    }
    pub fn like_left<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" LIKE '%?'");
        self.args.push(v);

        self
    }

    pub fn like_right<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" LIKE '?%'");
        self.args.push(v);
        self
    }

    pub fn not_like<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);
        self.sql.push_str(" NOT LIKE '%?%'");
        self.args.push(v);
        self
    }

    pub fn is_null(&mut self, column: &str) -> &mut Self {
        self.sql.push_str(column);
        self.sql.push_str(" is null");
        self
    }

    pub fn is_not_null(&mut self, column: &str) -> &mut Self {
        self.sql.push_str(column);
        self.sql.push_str(" is not null");
        self
    }

    pub fn in_<T>(&mut self, column: &str, obj: &[T]) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);

        let vec = v.as_array().unwrap();
        let mut sqls = String::new();
        for x in vec {
            sqls.push_str(" ? ");
            sqls.push_str(",");
            self.args.push(x.clone());
        }
        sqls.pop();
        self.sql.push_str(format!(" IN ({})", sqls).as_str());


        self
    }

    pub fn not_in<T>(&mut self, column: &str, obj: &[T]) -> &mut Self
        where T: Serialize {
        let v = serde_json::to_value(obj).unwrap();
        self.sql.push_str(column);

        let vec = v.as_array().unwrap();
        let mut sqls = String::new();
        for x in vec {
            sqls.push_str(" ? ");
            sqls.push_str(",");
            self.args.push(x.clone());
        }
        sqls.pop();
        self.sql.push_str(format!(" NOT IN ({})", sqls).as_str());
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
        w.eq("id", 1)
            .in_("id", &[1, 2, 3])
            .all_eq(&m)
            .like("name", 1)
            .not_like("name", "asdf")
            .between("create_time", "2020-01-01 00:00:00", "2020-12-12 00:00:00")
            .order_by(true, &["id", "name"]);
        println!("{:?}", w.clone());

        let ms:Vec<&str>= w.sql.matches("?").collect();
        assert_eq!(ms.len(),w.args.len());
    }
}