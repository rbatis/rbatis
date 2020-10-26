use std::ops::Add;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use rbatis_core::convert::StmtConvert;
use rbatis_core::db::DriverType;
use rbatis_core::Error;

/// you can serialize to JSON, and Clone, Debug
/// use json rpc send this Wrapper to server
///
/// for Example:
///         let w = Wrapper::new(&DriverType::Mysql)
///             .eq("id", 1)
///             .and()
///             .ne("id", 1)
///             .and()
///             .in_array("id", &[1, 2, 3])
///             .in_("id", &[1, 2, 3])
///             .r#in("id", &[1, 2, 3])
///             .and()
///             .not_in("id", &[1, 2, 3])
///             .and()
///             .like("name", 1)
///             .or()
///             .not_like("name", "asdf")
///             .and()
///             .between("create_time", "2020-01-01 00:00:00", "2020-12-12 00:00:00")
///             .group_by(&["id"])
///             .order_by(true, &["id", "name"])
///             .check().unwrap();
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wrapper {
    pub driver_type: DriverType,
    pub sql: String,
    pub args: Vec<serde_json::Value>,
    pub error: Option<Error>,
    pub checked: bool,
}

impl Wrapper {
    pub fn new(driver_type: &DriverType) -> Self {
        Self {
            driver_type: driver_type.clone(),
            sql: "".to_string(),
            args: vec![],
            error: None,
            checked: false,
        }
    }

    pub fn from(driver_type: &DriverType, sql: &str, args: &Vec<serde_json::Value>) -> Self {
        Self {
            driver_type: driver_type.clone(),
            sql: sql.to_string(),
            args: args.clone(),
            error: None,
            checked: false,
        }
    }

    //check is done？and return cloned Wrapper
    pub fn check(&mut self) -> Result<Wrapper, Error> {
        if self.error.is_some() {
            return Err(self.error.take().unwrap());
        }
        //remove and ,or
        self.trim_and();
        self.trim_or();
        let clone = Wrapper {
            driver_type: self.driver_type.clone(),
            sql: self.sql.clone(),
            args: self.args.clone(),
            error: self.error.clone(),
            checked: true,
        };
        return Ok(clone);
    }

    /// link left Wrapper to this Wrapper
    /// for Example:
    /// let w = Wrapper::new(&DriverType::Postgres).push_sql("(").eq("a", "1").push_sql(")").check().unwrap();
    /// let w2 = Wrapper::new(&DriverType::Postgres).eq("b", "2")
    /// .and()
    /// .push_wrapper(&w)
    /// .check().unwrap();
    /// println!("sql:{:?}", w2.sql.as_str());  // sql:"b = ? AND (a = ?)"
    /// println!("arg:{:?}", w2.args.clone()); // arg:[String("2"), String("1")]
    ///
    pub fn push_wrapper(&mut self, arg: &Wrapper) -> &mut Self {
        self.push(&arg.sql, &arg.args)
    }

    /// push sql,args into self
    pub fn push<T>(&mut self, sql: &str, args: &[T]) -> &mut Self
        where T: Serialize {
        let mut new_sql = sql.to_string();
        if self.driver_type.eq(&DriverType::Postgres) {
            for index in 0..args.len() {
                let str = self.driver_type.stmt_convert(index);
                new_sql = new_sql.replace(str.as_str(), self.driver_type.stmt_convert(index + args.len()).as_str());
            }
        }
        self.sql.push_str(new_sql.as_str());

        let args = serde_json::to_value(args).unwrap_or(serde_json::Value::Null);
        if args.is_null() {
            return self;
        }
        let args = args.as_array().unwrap();
        for x in args {
            self.args.push(x.to_owned());
        }
        self
    }


    /// do method,if test is true
    /// for example:
    ///  let arg = 1;
    ///  wrapper.if_do(true, |w,arg| w.eq("id", arg))
    pub fn do_if<'s, F>(&'s mut self, test: bool, method: F) -> &'s mut Self
        where F: FnOnce(&'s mut Self) -> &'s mut Self {
        if test {
            return method(self);
        }
        return self;
    }


    ///match cases
    /// for example:
    ///  let p = Option::<i32>::Some(1);
    ///         let w = Wrapper::new(&DriverType::Postgres)
    ///             .do_match(&[
    ///                 Case::new(p.is_some(),|w| w),
    ///                 Case::new(p.is_none(),|w| w),
    ///             ], |w| w)
    ///             .check().unwrap();
    pub fn do_match<'s, F>(&'s mut self, cases: &'s [Case], default: F) -> &'s mut Self
        where F: FnOnce(&'s mut Self) -> &'s mut Self {
        for x in cases {
            if x.test {
                return x.call_func(self);
            }
        }
        return default(self);
    }


    pub fn set_sql(&mut self, sql: &str) -> &mut Self {
        self.sql = sql.replace(" and ", " AND ").replace(" or ", " OR ").replace(" where ", " WHERE ");
        self
    }

    pub fn push_sql(&mut self, sql: &str) -> &mut Self {
        let s = sql.replace(" and ", " AND ").replace(" or ", " OR ").replace(" where ", " WHERE ");
        self.sql.push_str(s.as_str());
        self
    }

    pub fn set_args<T>(&mut self, args: &[T]) -> &mut Self where T: Serialize {
        let v = serde_json::to_value(args).unwrap_or(serde_json::Value::Null);
        if v.is_null() {
            return self;
        }
        if v.is_array() {
            self.args = v.as_array().unwrap_or(&vec![]).to_owned();
        }
        self
    }

    pub fn push_arg<T>(&mut self, arg: T) -> &mut Self where T: Serialize {
        let v = serde_json::to_value(arg).unwrap_or(serde_json::Value::Null);
        self.args.push(v);
        self
    }

    pub fn pop_arg(&mut self) -> &mut Self {
        self.args.pop();
        self
    }

    fn is_start_opt(&self) -> bool {
        let sql = self.sql.trim_end();
        sql.ends_with("WHERE")
            || sql.ends_with("AND")
            || sql.ends_with("OR")
            || sql.ends_with("(")
            || sql.ends_with(",")
            || sql.ends_with("=")
            || sql.ends_with("+")
            || sql.ends_with("-")
            || sql.ends_with("*")
            || sql.ends_with("/")
    }

    /// link wrapper sql, if end with where , do nothing
    pub fn and(&mut self) -> &mut Self {
        if !self.is_start_opt() {
            self.sql.push_str(" AND ");
        }
        self
    }

    /// link wrapper sql, if end with where , do nothing
    pub fn or(&mut self) -> &mut Self {
        if !self.is_start_opt() {
            self.sql.push_str(" OR ");
        }
        self
    }

    pub fn having(&mut self, sql_having: &str) -> &mut Self {
        self.and();
        self.sql.push_str(format!(" HAVING {} ", sql_having).as_str());
        self
    }

    /// arg: JsonObject or struct{} or map[String,**]
    pub fn all_eq<T>(&mut self, arg: T) -> &mut Self
        where T: Serialize {
        self.and();
        let v = serde_json::to_value(arg).unwrap_or(serde_json::Value::Null);
        if v.is_null() {
            self.error = Some(Error::from("[rbatis] wrapper all_eq only support object/map struct!"));
            return self;
        }
        if !v.is_object() {
            self.error = Some(Error::from("[rbatis] wrapper all_eq only support object/map struct!"));
            return self;
        }
        let map = v.as_object().unwrap();
        if map.len() == 0 {
            return self;
        }
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

    /// equal
    /// for example:
    ///  eq("a",1) " a = 1 "
    pub fn eq<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        self.and();
        let v = serde_json::to_value(obj).unwrap_or(serde_json::Value::Null);
        self.sql.push_str(column);
        self.sql.push_str(format!(" = {}", self.driver_type.stmt_convert(self.args.len())).as_str());
        self.args.push(v);


        self
    }

    /// not equal
    pub fn ne<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        self.and();
        let v = serde_json::to_value(obj).unwrap_or(serde_json::Value::Null);
        self.sql.push_str(column);
        self.sql.push_str(format!(" <> {}", self.driver_type.stmt_convert(self.args.len())).as_str());
        self.args.push(v);
        self
    }

    pub fn order_by(&mut self, is_asc: bool, columns: &[&str]) -> &mut Self {
        let len = columns.len();
        if len == 0 {
            return self;
        }
        let mut index = 0;
        self.sql = self.sql.trim_end_matches(" WHERE ")
            .trim_end_matches(" AND ")
            .trim_end_matches(" OR ").to_string();
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
        if len == 0 {
            return self;
        }
        let mut index = 0;
        self.sql = self.sql.trim_end_matches(" WHERE ")
            .trim_end_matches(" AND ")
            .trim_end_matches(" OR ").to_string();
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
        self.and();
        let v = serde_json::to_value(obj).unwrap_or(serde_json::Value::Null);
        self.sql.push_str(column);
        self.sql.push_str(format!(" > {}", self.driver_type.stmt_convert(self.args.len())).as_str());
        self.args.push(v);
        self
    }
    ///  sql:   column >= obj
    pub fn ge<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        self.and();
        let v = serde_json::to_value(obj).unwrap_or(serde_json::Value::Null);
        self.sql.push_str(column);
        self.sql.push_str(format!(" >= {}", self.driver_type.stmt_convert(self.args.len())).as_str());
        self.args.push(v);
        self
    }

    ///  sql:   column < obj
    pub fn lt<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        self.and();
        let v = serde_json::to_value(obj).unwrap_or(serde_json::Value::Null);
        self.sql.push_str(column);
        self.sql.push_str(format!(" < {}", self.driver_type.stmt_convert(self.args.len())).as_str());
        self.args.push(v);

        self
    }

    ///  sql:   column <= obj
    pub fn le<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        self.and();
        let v = serde_json::to_value(obj).unwrap_or(serde_json::Value::Null);
        self.sql.push_str(column);
        self.sql.push_str(format!(" <= {}", self.driver_type.stmt_convert(self.args.len())).as_str());
        self.args.push(v);
        self
    }

    pub fn between<T>(&mut self, column: &str, min: T, max: T) -> &mut Self
        where T: Serialize {
        self.and();
        let min_v = serde_json::to_value(min).unwrap_or(serde_json::Value::Null);
        let max_v = serde_json::to_value(max).unwrap_or(serde_json::Value::Null);
        self.sql.push_str(column);
        self.sql.push_str(format!(" BETWEEN {} AND {}", self.driver_type.stmt_convert(self.args.len()), self.driver_type.stmt_convert(self.args.len() + 1)).as_str());
        self.args.push(min_v);
        self.args.push(max_v);
        self
    }

    pub fn not_between<T>(&mut self, column: &str, min: T, max: T) -> &mut Self
        where T: Serialize {
        self.and();
        let min_v = serde_json::to_value(min).unwrap_or(serde_json::Value::Null);
        let max_v = serde_json::to_value(max).unwrap_or(serde_json::Value::Null);
        self.sql.push_str(column);
        self.sql.push_str(format!(" NOT BETWEEN {} AND {}", self.driver_type.stmt_convert(self.args.len()), self.driver_type.stmt_convert(self.args.len() + 1)).as_str());
        self.args.push(min_v);
        self.args.push(max_v);
        self
    }

    pub fn like<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        self.and();
        let v = serde_json::to_value(obj).unwrap_or(serde_json::Value::Null);
        let mut v_str = String::new();
        if v.is_string() {
            v_str = format!("%{}%", v.as_str().unwrap());
        } else {
            v_str = format!("%{}%", v.to_string());
        }
        self.sql.push_str(column);
        self.sql.push_str(format!(" LIKE {}", self.driver_type.stmt_convert(self.args.len())).as_str());
        self.args.push(json!(v_str));
        self
    }

    pub fn like_left<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        self.and();
        let v = serde_json::to_value(obj).unwrap_or(serde_json::Value::Null);
        let mut v_str = String::new();
        if v.is_string() {
            v_str = format!("%{}", v.as_str().unwrap());
        } else {
            v_str = format!("%{}", v.to_string());
        }
        self.sql.push_str(column);
        self.sql.push_str(format!(" LIKE {}", self.driver_type.stmt_convert(self.args.len())).as_str());
        self.args.push(json!(v_str));
        self
    }

    pub fn like_right<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        self.and();
        let v = serde_json::to_value(obj).unwrap_or(serde_json::Value::Null);
        let mut v_str = String::new();
        if v.is_string() {
            v_str = format!("{}%", v.as_str().unwrap());
        } else {
            v_str = format!("{}%", v.to_string());
        }
        self.sql.push_str(column);
        self.sql.push_str(format!(" LIKE {}", self.driver_type.stmt_convert(self.args.len())).as_str());
        self.args.push(json!(v_str));
        self
    }

    pub fn not_like<T>(&mut self, column: &str, obj: T) -> &mut Self
        where T: Serialize {
        self.and();
        let v = serde_json::to_value(obj).unwrap_or(serde_json::Value::Null);
        let mut v_str = String::new();
        if v.is_string() {
            v_str = format!("%{}%", v.as_str().unwrap());
        } else {
            v_str = format!("%{}%", v.to_string());
        }
        self.sql.push_str(column);
        self.sql.push_str(format!(" NOT LIKE {}", self.driver_type.stmt_convert(self.args.len())).as_str());
        self.args.push(json!(v_str));
        self
    }

    pub fn is_null(&mut self, column: &str) -> &mut Self {
        self.and();
        self.sql.push_str(column);
        self.sql.push_str(" IS NULL");
        self
    }

    pub fn is_not_null(&mut self, column: &str) -> &mut Self {
        self.and();
        self.sql.push_str(column);
        self.sql.push_str(" IS NOT NULL");
        self
    }

    /// gen sql: * in (*,*,*)
    pub fn in_array<T>(&mut self, column: &str, obj: &[T]) -> &mut Self
        where T: Serialize {
        self.and();
        if obj.len() == 0 {
            return self;
        }
        let v = serde_json::to_value(obj).unwrap_or(serde_json::Value::Null);
        self.sql.push_str(column);
        let vec = v.as_array().unwrap();
        let mut sqls = String::new();
        for x in vec {
            sqls.push_str(format!(" {} ", self.driver_type.stmt_convert(self.args.len())).as_str());
            sqls.push_str(",");
            self.args.push(x.clone());
        }
        sqls.pop();
        self.sql.push_str(format!(" IN ({})", sqls).as_str());
        self
    }

    /// gen sql: * in (*,*,*)
    pub fn in_<T>(&mut self, column: &str, obj: &[T]) -> &mut Self
        where T: Serialize {
        self.in_array(column,obj)
    }

    /// gen sql: * in (*,*,*)
    pub fn r#in<T>(&mut self, column: &str, obj: &[T]) -> &mut Self
        where T: Serialize {
        self.in_array(column,obj)
    }


    pub fn not_in<T>(&mut self, column: &str, obj: &[T]) -> &mut Self
        where T: Serialize {
        self.and();
        let v = serde_json::to_value(obj).unwrap_or(serde_json::Value::Null);
        self.sql.push_str(column);
        let vec = v.as_array().unwrap();
        let mut sqls = String::new();
        for x in vec {
            sqls.push_str(format!(" {} ", self.driver_type.stmt_convert(self.args.len())).as_str());
            sqls.push_str(",");
            self.args.push(x.clone());
        }
        sqls.pop();
        self.sql.push_str(format!(" NOT IN ({})", sqls).as_str());
        self
    }

    pub fn trim_and(&mut self) -> &mut Self {
        self.sql = self.sql
            .trim_start_matches(" AND ")
            .trim_end_matches(" AND ")
            .trim_start_matches("AND ")
            .trim_end_matches(" AND")
            .to_string();
        self
    }

    pub fn trim_or(&mut self) -> &mut Self {
        self.sql = self.sql.
            trim_start_matches(" OR ")
            .trim_end_matches(" OR ")
            .trim_start_matches("OR ")
            .trim_end_matches(" OR")
            .to_string();
        self
    }
}

pub struct Case {
    test: bool,
    func: Box<dyn Fn(&mut Wrapper) -> &mut Wrapper>,
}

impl Case {
    pub fn new<F>(test: bool, f: F) -> Self
        where F: 'static + Fn(&mut Wrapper) -> &mut Wrapper {
        Self {
            test,
            func: Box::new(f),
        }
    }

    pub fn call_func<'s, 'a>(&'s self, w: &'a mut Wrapper) -> &'a mut Wrapper {
        (self.func)(w)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use serde_json::Map;

    use rbatis_core::db::DriverType;

    use crate::utils::bencher::Bencher;
    use crate::wrapper::{Case, Wrapper};

    #[test]
    fn test_select() {
        let mut m = Map::new();
        m.insert("a".to_string(), json!("1"));
        let w = Wrapper::new(&DriverType::Mysql).eq("id", 1)
            .ne("id", 1)
            .in_array("id", &[1, 2, 3])
            .not_in("id", &[1, 2, 3])
            .all_eq(&m)
            .like("name", 1)
            .or()
            .not_like("name", "asdf")
            .between("create_time", "2020-01-01 00:00:00", "2020-12-12 00:00:00")
            .group_by(&["id"])
            .order_by(true, &["id", "name"])
            .check().unwrap();
        println!("sql:{:?}", w.sql.as_str());
        println!("arg:{:?}", w.args.clone());

        let ms: Vec<&str> = w.sql.matches("?").collect();
        assert_eq!(ms.len(), w.args.len());
    }


    ///cargo test --release --color=always --package rbatis --lib wrapper::test::bench_select --no-fail-fast -- --exact -Z unstable-options --show-output
    ///run with windows10:
    ///  use Time: 0.51 s,each:5100 nano/op  use TPS: 196078.431372549 TPS/s
    #[test]
    fn bench_select() {
        let mut map = Map::new();
        map.insert("a".to_string(), json!("1"));
        let mut b = Bencher::new(100000);
        b.iter_mut(&mut map, |m| {
            let w = Wrapper::new(&DriverType::Mysql).eq("id", 1)
                .ne("id", 1)
                .in_array("id", &[1, 2, 3])
                .r#in("id",&[1,2,3])
                .in_("id",&[1,2,3])
                .not_in("id", &[1, 2, 3])
                .all_eq(&m)
                .like("name", 1)
                .or()
                .not_like("name", "asdf")
                .between("create_time", "2020-01-01 00:00:00", "2020-12-12 00:00:00")
                .group_by(&["id"])
                .order_by(true, &["id", "name"])
                .check().unwrap();
        });
    }

    #[test]
    fn test_link() {
        let w = Wrapper::new(&DriverType::Postgres).eq("a", "1").check().unwrap();
        let w2 = Wrapper::new(&DriverType::Postgres).eq("b", "2")
            .and()
            .push_wrapper(&w)
            .check().unwrap();

        println!("sql:{:?}", w2.sql.as_str());
        println!("arg:{:?}", w2.args.clone());

        let ms: Vec<&str> = w.sql.matches("$").collect();
        assert_eq!(ms.len(), w.args.len());
    }

    #[test]
    fn test_do_if() {
        let p = Option::<i32>::Some(1);
        let w = Wrapper::new(&DriverType::Postgres)
            .do_if(p.is_some(), |w| w.eq("a", p))
            .check().unwrap();
        println!("sql:{:?}", w.sql.as_str());
        println!("arg:{:?}", w.args.clone());
    }


    #[test]
    fn test_do_match() {
        let p = 1;
        let w = Wrapper::new(&DriverType::Postgres)
            .do_match(&[
                Case::new(p == 0, |w| w.eq("a", "some")),
                Case::new(p == 2, |w| w.eq("a", "none")),
            ], |w| w.eq("a", "default"))
            .check().unwrap();
        println!("sql:{:?}", w.sql.as_str());
        println!("arg:{:?}", w.args.clone());
    }

    #[test]
    fn test_wp() {
        let w = Wrapper::new(&DriverType::Postgres)
            .eq("1", "1")
            .or()
            .like("TITLE", "title")
            .or()
            .like("ORIGINAL_NAME", "saf")
            .check().unwrap();
        println!("sql:{:?}", w.sql.as_str());
        println!("arg:{:?}", w.args.clone());
    }

    #[test]
    fn test_push_arg() {
        let w = Wrapper::new(&DriverType::Mysql)
            .push_sql("?,?")
            .push_arg(1)
            .push_arg("asdfasdfa")
            .check().unwrap();
        println!("sql:{:?}", w.sql.as_str());
        println!("arg:{:?}", w.args.clone());
    }

    #[test]
    fn test_push_wrapper() {
        let mut w1 = Wrapper::new(&DriverType::Mysql);
        let mut w2 = w1.clone();

        let w2 = w1
            .eq("b", "2")
            .and()
            .push_wrapper(&w2.push_sql("(").eq("a", "1").push_sql(")").check().unwrap())
            .check().unwrap();
        println!("sql:{:?}", w2.sql.as_str());
        println!("arg:{:?}", w2.args.clone());
    }
}