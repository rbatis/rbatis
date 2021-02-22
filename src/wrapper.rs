use std::collections::HashMap;
use std::ops::Add;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;
use crate::core::Error;
use crate::sql::upper::SqlUpperCase;
use std::fmt;
use std::fmt::{Debug, Formatter};

/// The packing/Wrapper of the SQL
/// SQL passed into the Wrapper keep the keyword uppercase
///
/// for Example:
///         let w = Wrapper::new(&DriverType::Mysql)
///             .push_sql(“id == 1”)
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
///             ;
///
#[derive(Clone)]
pub struct Wrapper {
    pub driver_type: DriverType,
    pub sql: String,
    pub args: Vec<serde_json::Value>,
    pub formats: HashMap<String, fn(arg: &str) -> String>,
}

impl Debug for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formats = HashMap::new();
        for (k, v) in &self.formats {
            formats.insert(k.to_string(), v(k));
        }
        f.debug_struct("Wrapper")
            .field("driver_type", &self.driver_type)
            .field("sql", &self.sql)
            .field("args", &self.args)
            .field("formats", &formats)
            .finish()
    }
}

impl Wrapper {
    pub fn new(driver_type: &DriverType) -> Self {
        Self {
            driver_type: driver_type.clone(),
            sql: String::new(),
            args: vec![],
            formats: Default::default(),
        }
    }

    pub fn from(driver_type: &DriverType, sql: &str, args: Vec<serde_json::Value>) -> Self {
        Self {
            driver_type: driver_type.clone(),
            sql: sql.to_string(),
            args: args,
            formats: HashMap::new(),
        }
    }

    pub fn trim_value(mut self, from: &str, to: &str) -> Self {
        self.sql = self.sql.replace(from, to);
        self
    }

    pub fn set_formats(mut self, formats: HashMap<String, fn(arg: &str) -> String>) -> Self {
        self.formats = formats;
        self
    }

    /// link left Wrapper to this Wrapper
    /// for Example:
    /// let w = Wrapper::new(&DriverType::Postgres).push_sql("(").eq("a", "1").push_sql(")");
    /// let w2 = Wrapper::new(&DriverType::Postgres).eq("b", "2")
    /// .and()
    /// .push_wrapper(&w);
    /// println!("sql:{:?}", w2.sql.as_str());  // sql:"b = ? AND (a = ?)"
    /// println!("arg:{:?}", w2.args.clone()); // arg:[String("2"), String("1")]
    ///
    pub fn push_wrapper(self, arg: &Wrapper) -> Self {
        self.push(&arg.sql, &arg.args)
    }

    /// push sql,args into self
    pub fn push<T>(mut self, sql: &str, args: &[T]) -> Self
        where
            T: Serialize,
    {
        let mut new_sql = sql.to_string();
        if self.driver_type.is_number_type() {
            let self_arg_len = self.args.len();
            for index in 0..args.len() {
                let str = self.driver_type.stmt_convert(index);
                new_sql = new_sql.replace(
                    str.as_str(),
                    self.driver_type.stmt_convert(index + args.len()).as_str(),
                );
            }
            for index in args.len()..self_arg_len {
                let str = self.driver_type.stmt_convert(index);
                new_sql = new_sql.replace(
                    str.as_str(),
                    self.driver_type.stmt_convert(index + args.len()).as_str(),
                );
            }
        }
        self.sql.push_str(new_sql.as_str());

        let args = json!(args);
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
    ///  wrapper.do_if(true, |w| w.eq("id"))
    pub fn do_if<'s, F>(self, test: bool, method: F) -> Self
        where
            F: FnOnce(Self) -> Self,
    {
        if test {
            return method(self);
        }
        return self;
    }

    /// do method,if test is true
    /// for example:
    ///  let arg = 1;
    ///  wrapper.do_if(true, |w| w.eq("id"),|w|w)
    pub fn do_if_else<'s, F>(self, test: bool, method_if: F, default: fn(Self) -> Self) -> Self
        where
            F: FnOnce(Self) -> Self,
    {
        if test {
            return method_if(self);
        } else {
            return default(self);
        }
    }

    ///match cases
    /// for example:
    ///  let p = Option::<i32>::Some(1);
    ///         let w = Wrapper::new(&DriverType::Postgres)
    ///             .do_match(&[
    ///                 (p == 0, |w| w.eq("a", "some")),
    ///                 (p == 1, |w| w.eq("a", "some")),
    ///             ], |w| w.eq("a", "default"))
    ///             ;
    pub fn do_match<'s, F>(self, cases: &[(bool, fn(Wrapper) -> Wrapper)], default: F) -> Self
        where
            F: FnOnce(Self) -> Self,
    {
        for (test, case) in cases {
            if *test {
                return case(self);
            }
        }
        return default(self);
    }

    pub fn set_sql(mut self, sql: &str) -> Self {
        self.sql = self.driver_type.upper_case_sql(sql);
        self
    }

    pub fn push_sql(mut self, sql: &str) -> Self {
        self.sql.push_str(&self.driver_type.upper_case_sql(sql));
        self
    }

    pub fn set_args<T>(mut self, args: &[T]) -> Self
        where
            T: Serialize,
    {
        let v = json!(args);
        if v.is_null() {
            return self;
        }
        if v.is_array() {
            self.args = v.as_array().unwrap_or(&vec![]).to_owned();
        }
        self
    }

    pub fn push_arg<T>(mut self, arg: T) -> Self
        where
            T: Serialize,
    {
        let v = json!(arg);
        self.args.push(v);
        self
    }

    pub fn pop_arg(mut self) -> Self {
        self.args.pop();
        self
    }

    fn not_allow_and_or(&self) -> bool {
        let sql = self.sql.trim_end();
        if sql.is_empty() {
            return true;
        }
        sql.ends_with(" WHERE")
            || sql.ends_with(" AND")
            || sql.ends_with(" OR")
            || sql.ends_with("(")
            || sql.ends_with(",")
            || sql.ends_with("=")
            || sql.ends_with("+")
            || sql.ends_with("-")
            || sql.ends_with("*")
            || sql.ends_with("/")
            || sql.ends_with("%")
            || sql.ends_with("^")
            || sql.ends_with(">")
            || sql.ends_with("<")
            || sql.ends_with("&")
            || sql.ends_with("|")
    }

    /// link wrapper sql, if end with where , do nothing
    pub fn and(mut self) -> Self {
        if !self.not_allow_and_or() {
            self.sql.push_str(" AND ");
        }
        self
    }

    /// link wrapper sql, if end with where , do nothing
    pub fn or(mut self) -> Self {
        if !self.not_allow_and_or() {
            self.sql.push_str(" OR ");
        }
        self
    }

    pub fn having(mut self, sql_having: &str) -> Self {
        self = self.and();
        self.sql
            .push_str(format!(" HAVING {} ", sql_having).as_str());
        self
    }

    /// arg: JsonObject or struct{} or map[String,**]
    pub fn all_eq<T>(mut self, arg: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        let v = json!(arg);
        if v.is_null() {
            return self;
        }
        if !v.is_object() {
            return self;
        }
        let map = v.as_object().unwrap();
        if map.len() == 0 {
            return self;
        }
        let len = map.len();
        let mut index = 0;
        for (k, v) in map {
            self = self.eq(k.as_str(), v);
            if (index + 1) != len {
                self.sql.push_str(" , ");
                index += 1;
            }
        }
        self
    }

    ///format column
    fn do_format_column(&self, column: &str, data: String) -> String {
        let source = self.formats.get(column);
        match source {
            Some(s) => {
                return s(&data);
            }
            _ => {
                return data;
            }
        }
    }

    /// equal
    /// for example:
    ///  eq("a",1) " a = 1 "
    pub fn eq<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        self.sql.push_str(&format!(
            "{} = {}",
            column,
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len()))
        ));
        self.args.push(json!(obj));
        self
    }

    /// not equal
    pub fn ne<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        self.sql.push_str(&format!(
            "{} <> {}",
            column,
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len()))
        ));
        self.args.push(json!(obj));
        self
    }

    pub fn order_by(mut self, is_asc: bool, columns: &[&str]) -> Self {
        let len = columns.len();
        if len == 0 {
            return self;
        }
        let mut index = 0;
        self.sql = self
            .sql
            .trim_end()
            .trim_end_matches(" WHERE")
            .trim_end_matches(" AND")
            .trim_end_matches(" OR")
            .to_string();
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

    pub fn group_by(mut self, columns: &[&str]) -> Self {
        let len = columns.len();
        if len == 0 {
            return self;
        }
        let mut index = 0;
        self.sql = self
            .sql
            .trim()
            .trim_end_matches(" WHERE")
            .trim_end_matches(" AND")
            .trim_end_matches(" OR")
            .to_string();
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
    pub fn gt<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        self.sql.push_str(&format!(
            "{} > {}",
            column,
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len()))
        ));
        self.args.push(json!(obj));
        self
    }
    ///  sql:   column >= obj
    pub fn ge<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        self.sql.push_str(&format!(
            "{} >= {}",
            column,
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len()))
        ));
        self.args.push(json!(obj));
        self
    }

    ///  sql:   column < obj
    pub fn lt<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        self.sql.push_str(&format!(
            "{} < {}",
            column,
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len()))
        ));
        self.args.push(json!(obj));
        self
    }

    ///  sql:   column <= obj
    pub fn le<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        self.sql.push_str(&format!(
            "{} <= {}",
            column,
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len()))
        ));
        self.args.push(json!(obj));
        self
    }

    pub fn between<T>(mut self, column: &str, min: T, max: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        self.sql.push_str(&format!(
            "{} BETWEEN {} AND {}",
            column,
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len())),
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len() + 1))
        ));
        self.args.push(json!(min));
        self.args.push(json!(max));
        self
    }

    pub fn not_between<T>(mut self, column: &str, min: T, max: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        self.sql.push_str(&format!(
            "{} NOT BETWEEN {} AND {}",
            column,
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len())),
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len() + 1))
        ));
        self.args.push(json!(min));
        self.args.push(json!(max));
        self
    }

    pub fn like<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        let v = json!(obj);
        let mut v_str = String::new();
        if v.is_string() {
            v_str = format!("%{}%", v.as_str().unwrap());
        } else {
            v_str = format!("%{}%", v.to_string());
        }
        self.sql.push_str(&format!(
            "{} LIKE {}",
            column,
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len()))
        ));
        self.args.push(json!(v_str));
        self
    }

    pub fn like_left<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        let v = json!(obj);
        let mut v_str = String::new();
        if v.is_string() {
            v_str = format!("%{}", v.as_str().unwrap());
        } else {
            v_str = format!("%{}", v.to_string());
        }
        self.sql.push_str(&format!(
            "{} LIKE {}",
            column,
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len()))
        ));
        self.args.push(json!(v_str));
        self
    }

    pub fn like_right<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        let v = json!(obj);
        let mut v_str = String::new();
        if v.is_string() {
            v_str = format!("{}%", v.as_str().unwrap());
        } else {
            v_str = format!("{}%", v.to_string());
        }
        self.sql.push_str(&format!(
            "{} LIKE {}",
            column,
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len()))
        ));
        self.args.push(json!(v_str));
        self
    }

    pub fn not_like<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        self = self.and();
        let v = json!(obj);
        let mut v_str = String::new();
        if v.is_string() {
            v_str = format!("%{}%", v.as_str().unwrap());
        } else {
            v_str = format!("%{}%", v.to_string());
        }
        self.sql.push_str(&format!(
            "{} NOT LIKE {}",
            column,
            self.do_format_column(column, self.driver_type.stmt_convert(self.args.len()))
        ));
        self.args.push(json!(v_str));
        self
    }

    pub fn is_null(mut self, column: &str) -> Self {
        self = self.and();
        self.sql.push_str(column);
        self.sql.push_str(" IS NULL");
        self
    }

    pub fn is_not_null(mut self, column: &str) -> Self {
        self = self.and();
        self.sql.push_str(column);
        self.sql.push_str(" IS NOT NULL");
        self
    }

    /// gen sql: * in (*,*,*)
    pub fn in_array<T>(mut self, column: &str, obj: &[T]) -> Self
        where
            T: Serialize,
    {
        if obj.len() == 0 {
            return self;
        }
        let arr = json!(obj);
        match arr {
            serde_json::Value::Array(vec) => {
                self = self.and();
                let mut sqls = String::new();
                for x in vec {
                    sqls.push_str(&format!(
                        " {} ",
                        self.do_format_column(
                            column,
                            self.driver_type.stmt_convert(self.args.len()),
                        )
                    ));
                    sqls.push_str(",");
                    self.args.push(x);
                }
                sqls.pop();
                self.sql
                    .push_str(format!("{} IN ({})", column, sqls).as_str());
            }
            _ => {}
        }
        self
    }

    /// gen sql: * in (*,*,*)
    pub fn in_<T>(self, column: &str, obj: &[T]) -> Self
        where
            T: Serialize,
    {
        self.in_array(column, obj)
    }

    /// gen sql: * in (*,*,*)
    pub fn r#in<T>(self, column: &str, obj: &[T]) -> Self
        where
            T: Serialize,
    {
        self.in_array(column, obj)
    }

    pub fn not_in<T>(mut self, column: &str, obj: &[T]) -> Self
        where
            T: Serialize,
    {
        let arr = json!(obj);
        match arr {
            serde_json::Value::Array(vec) => {
                self = self.and();
                let mut sqls = String::new();
                for x in vec {
                    sqls.push_str(&format!(
                        " {} ",
                        self.do_format_column(
                            column,
                            self.driver_type.stmt_convert(self.args.len()),
                        )
                    ));
                    sqls.push_str(",");
                    self.args.push(x);
                }
                sqls.pop();
                self.sql
                    .push_str(format!("{} NOT IN ({})", column, sqls).as_str());
            }
            _ => {}
        }
        self
    }

    pub fn trim_space(mut self) -> Self {
        self.sql = self.sql.replace("  ", " ");
        return self;
    }

    pub fn trim_and(mut self) -> Self {
        self.sql = self
            .sql
            .trim()
            .trim_start_matches("AND ")
            .trim_end_matches(" AND")
            .to_string();
        self
    }

    pub fn trim_or(mut self) -> Self {
        self.sql = self
            .sql
            .trim()
            .trim_start_matches("OR ")
            .trim_end_matches(" OR")
            .to_owned();
        self
    }

    pub fn trim_and_or(mut self) -> Self {
        self.sql = self
            .sql
            .trim()
            .trim_start_matches("AND ")
            .trim_end_matches(" AND")
            .trim_start_matches("OR ")
            .trim_end_matches(" OR")
            .to_owned();
        self
    }

    pub fn insert_into(mut self, table_name: &str, columns: &str, values: &str) -> Self {
        if values.starts_with("(") && values.ends_with(")") {
            self.sql = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                table_name, columns, values
            );
        } else {
            self.sql = format!("INSERT INTO {} ({}) VALUES {}", table_name, columns, values);
        }
        self
    }

    /// limit
    /// for example:
    ///  limit(1) " LIMIT 1 "
    pub fn limit(mut self, limit: u64) -> Self {
        self.sql.push_str(&format!(" LIMIT {} ", limit));
        self
    }
}
