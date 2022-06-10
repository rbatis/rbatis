use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use rbson::Bson;
use rbatis_sql::ops::AsProxy;

use serde::{Deserialize, Serialize};

use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;
use crate::core::Error;

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
    pub dml: String,
    pub sql: String,
    pub args: Vec<Bson>,
    //formats map[String]String. for example: map["x"] "{}::uuid"
    pub formats: HashMap<String, String>,
}

macro_rules! push_sql {
    ($i:expr,$($v:expr$(,)?)*) => {
        $($i.push_str($v);)*
    };
}

impl Debug for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formats = HashMap::new();
        for (k, v) in &self.formats {
            formats.insert(k.to_string(), v.clone());
        }
        f.debug_struct("Wrapper")
            .field("driver_type", &self.driver_type)
            .field("sql", &self.sql)
            .field("args", &self.args)
            .field("dml", &self.dml)
            //.field("formats", &formats)
            .finish()
    }
}

impl Wrapper {
    pub fn new(driver_type: &DriverType) -> Self {
        Self {
            driver_type: driver_type.clone(),
            dml: "where".to_string(),
            sql: String::with_capacity(200),
            args: Vec::with_capacity(5),
            formats: Default::default(),
        }
    }

    pub fn trim_value(mut self, from: &str, to: &str) -> Self {
        self.sql = self.sql.replace(from, to);
        self
    }

    pub fn set_formats(mut self, formats: HashMap<String, String>) -> Self {
        self.formats = formats;
        self
    }

    pub fn set_dml(mut self, dml: &str) -> Self {
        self.dml = dml.to_string();
        self
    }

    /// link left Wrapper to this Wrapper
    /// for Example:
    /// let w = Wrapper::new(&DriverType::Postgres).push_sql("(").eq("a", "1").push_sql(")");
    /// let w2 = Wrapper::new(&DriverType::Postgres).eq("b", "2")
    /// .and()
    /// .push_wrapper(w);
    /// println!("sql:{:?}", w2.sql.as_str());  // sql:"b = ? and (a = ?)"
    /// println!("arg:{:?}", w2.args.clone()); // arg:[String("2"), String("1")]
    ///
    pub fn push_wrapper(self, arg: Wrapper) -> Self {
        self.push(&arg.sql, arg.args)
    }

    /// push sql,args into self
    pub fn push(mut self, sql: &str, args: Vec<rbson::Bson>) -> Self
    {
        let mut new_sql = sql.to_string();
        match self.driver_type {
            DriverType::None => {}
            DriverType::Mysql => {}
            DriverType::Postgres | DriverType::Mssql => {
                let self_arg_len = self.args.len();
                for index in 0..args.len() {
                    let mut convert_column = String::new();
                    self.driver_type.stmt_convert(index, &mut convert_column);

                    let mut convert_column_new = String::new();
                    self.driver_type.stmt_convert(index + args.len(), &mut convert_column_new);
                    new_sql = new_sql.replace(
                        convert_column.as_str(),
                        convert_column_new.as_str(),
                    );
                }
                for index in args.len()..self_arg_len {
                    let mut convert_column = String::new();
                    self.driver_type.stmt_convert(index, &mut convert_column);

                    let mut convert_column_new = String::new();
                    self.driver_type.stmt_convert(index + args.len(), &mut convert_column_new);

                    println!("{},{}", convert_column, convert_column_new);
                    new_sql = new_sql.replace(
                        convert_column.as_str(),
                        convert_column_new.as_str(),
                    );
                }
            }
            DriverType::Sqlite => {}
        }
        self.sql.push_str(new_sql.as_str());
        for x in args {
            self.args.push(x);
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
    ///  wrapper.r#if(true, |w| w.eq("id"))
    pub fn r#if<'s, F>(self, test: bool, method: F) -> Self
        where
            F: FnOnce(Self) -> Self,
    {
        self.do_if(test, method)
    }

    /// do method,if test is true
    /// for example:
    ///  let arg = 1;
    ///  wrapper.do_if(true, |w| w.eq("id"),|w|w)
    pub fn do_if_else<'s, F>(self, test: bool, method_if: F, method_else: fn(Self) -> Self) -> Self
        where
            F: FnOnce(Self) -> Self,
    {
        if test {
            return method_if(self);
        } else {
            return method_else(self);
        }
    }

    /// do method,if test is true
    /// for example:
    ///  let arg = 1;
    ///  wrapper.do_if(true, |w| w.eq("id"),|w|w)
    pub fn r#if_else<'s, F>(self, test: bool, method_if: F, method_else: fn(Self) -> Self) -> Self
        where
            F: FnOnce(Self) -> Self,
    {
        self.do_if_else(test, method_if, method_else)
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
        self.sql = sql.to_string();
        self
    }

    pub fn push_sql(mut self, sql: &str) -> Self {
        self.sql.push_str(sql);
        self
    }

    pub fn set_args<T>(mut self, args: &[T]) -> Self
        where
            T: Serialize,
    {
        let v = crate::as_bson!(args);
        match v {
            Bson::Null => {
                return self;
            }
            Bson::Array(ref arr) => {
                self.args = v.as_array().unwrap_or(&vec![]).to_owned();
            }
            _ => {}
        }
        self
    }

    pub fn push_arg<T>(mut self, arg: T) -> Self
        where
            T: Serialize,
    {
        let v = crate::as_bson!(&arg);
        self.args.push(v);
        self
    }

    pub fn pop_arg(mut self) -> Self {
        self.args.pop();
        self
    }

    pub fn not_allow_add_and_on_end(&self) -> bool {
        let sql = self.sql.trim_end();
        if sql.is_empty() {
            return true;
        }
        sql.ends_with(crate::sql::TEMPLATE.r#where.left_space)
            || sql.ends_with(crate::sql::TEMPLATE.and.left_space)
            || sql.ends_with(crate::sql::TEMPLATE.or.left_space)
    }

    /// link wrapper sql, if end with where , do nothing
    pub fn and(mut self) -> Self {
        if !self.not_allow_add_and_on_end() {
            self.sql
                .push_str(&crate::sql::TEMPLATE.and.left_right_space);
        }
        self
    }

    /// link wrapper sql, if end with where , do nothing
    pub fn or(mut self) -> Self {
        if !self.not_allow_add_and_on_end() {
            self.sql.push_str(&crate::sql::TEMPLATE.or.left_right_space);
        }
        self
    }

    pub fn having(mut self, sql_having: &str) -> Self {
        push_sql!(self.sql,crate::sql::TEMPLATE.having.value," ",sql_having);
        self
    }

    /// arg: JsonObject or struct{} or map[String,**]
    pub fn eq_all<T>(mut self, arg: T) -> Self
        where
            T: Serialize,
    {
        let v = crate::as_bson!(&arg);
        if v.is_null() {
            return self;
        }
        if !v.is_object() {
            return self;
        }
        let map = v.as_document().unwrap();
        if map.len() == 0 {
            return self;
        }
        self.sql.push_str("(");
        let len = map.len();
        let mut index = 0;
        for (k, v) in map {
            self = self.eq(k.as_str(), v);
            if (index + 1) != len {
                self.sql.push_str(" and ");
                index += 1;
            }
        }
        self.sql.push_str(")");
        self
    }

    ///format column
    pub fn do_format_column(&self, column: &str, data: &mut String) {
        let source = self.formats.get(column);
        match source {
            Some(source) => {
                *data = source.replace("{}", data);
            }
            _ => {}
        }
    }

    /// equal
    /// for example:
    ///  eq("a",1) " a = 1 "
    pub fn eq<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql,column," = ",convert_column.as_str());
        self.args.push(crate::as_bson!(&obj));
        self
    }

    /// not equal
    pub fn ne<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql,column," <> ",convert_column.as_str());
        self.args.push(crate::as_bson!(&obj));
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
            .trim_end_matches(crate::sql::TEMPLATE.r#where.left_space)
            .trim_end_matches(crate::sql::TEMPLATE.and.left_space)
            .trim_end_matches(crate::sql::TEMPLATE.or.left_space)
            .to_string();
        self.sql.push_str(&crate::sql::TEMPLATE.order_by.left_right_space);
        for x in columns {
            if is_asc {
                push_sql!(self.sql,x," ",crate::sql::TEMPLATE.asc.value);
            } else {
                push_sql!(self.sql,x," ",crate::sql::TEMPLATE.desc.value);
            }
            if (index + 1) != len {
                self.sql.push_str(",");
                index += 1;
            }
        }
        self.sql.push_str(" ");
        self
    }

    pub fn order_bys(mut self, column_asc: &[(&str, bool)]) -> Self {
        let len = column_asc.len();
        if len == 0 {
            return self;
        }
        let mut index = 0;
        self.sql = self
            .sql
            .trim_end()
            .trim_end_matches(crate::sql::TEMPLATE.r#where.left_space)
            .trim_end_matches(crate::sql::TEMPLATE.and.left_space)
            .trim_end_matches(crate::sql::TEMPLATE.or.left_space)
            .to_string();
        self.sql.push_str(&crate::sql::TEMPLATE.order_by.left_right_space);
        for (x, is_asc) in column_asc {
            if *is_asc {
                push_sql!(self.sql,x," ",crate::sql::TEMPLATE.asc.value);
            } else {
                push_sql!(self.sql,x," ",crate::sql::TEMPLATE.desc.value);
            }
            if (index + 1) != len {
                self.sql.push_str(",");
                index += 1;
            }
        }
        self.sql.push_str(" ");
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
            .trim_end_matches(crate::sql::TEMPLATE.r#where.left_space)
            .trim_end_matches(crate::sql::TEMPLATE.and.left_space)
            .trim_end_matches(crate::sql::TEMPLATE.or.left_space)
            .to_string();
        self.sql
            .push_str(&crate::sql::TEMPLATE.group_by.left_right_space);
        for x in columns {
            self.sql.push_str(x);
            if (index + 1) != len {
                self.sql.push_str(",");
                index += 1;
            }
        }
        self.sql.push_str(" ");
        self
    }

    ///  sql:   column > obj
    pub fn gt<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql,column," > ", &convert_column.as_str());
        self.args.push(crate::as_bson!(&obj));
        self
    }
    ///  sql:   column >= obj
    pub fn ge<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql,column," >= ", &convert_column.as_str());
        self.args.push(crate::as_bson!(&obj));
        self
    }

    ///  sql:   column < obj
    pub fn lt<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql,column," < ", &convert_column.as_str());
        self.args.push(crate::as_bson!(&obj));
        self
    }

    ///  sql:   column <= obj
    pub fn le<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql,column," <= ", &convert_column.as_str());
        self.args.push(crate::as_bson!(&obj));
        self
    }

    pub fn between<T>(mut self, column: &str, min: T, max: T) -> Self
        where
            T: Serialize,
    {
        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql,column," ",crate::sql::TEMPLATE.between.value," ", &convert_column.as_str());

        self.args.push(crate::as_bson!(&min));

        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql," ",crate::sql::TEMPLATE.and.value," ", &convert_column.as_str());

        self.args.push(crate::as_bson!(&max));
        self
    }

    pub fn not_between<T>(mut self, column: &str, min: T, max: T) -> Self
        where
            T: Serialize,
    {
        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql,column," ",crate::sql::TEMPLATE.not.value," ",crate::sql::TEMPLATE.between.value," ", &convert_column.as_str());

        self.args.push(crate::as_bson!(&min));

        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql," ",crate::sql::TEMPLATE.and.value," ", &convert_column.as_str());

        self.args.push(crate::as_bson!(&max));
        self
    }

    pub fn like<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        let v = crate::as_bson!(&obj);
        let mut v_str = String::new();
        if v.as_str().is_some() {
            v_str = format!("%{}%", v.as_str().unwrap());
        } else {
            v_str = format!("%{}%", v.to_string());
        }

        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql,column," ",crate::sql::TEMPLATE.like.value," ", &convert_column.as_str());

        self.args.push(crate::as_bson!(&v_str));
        self
    }

    pub fn like_left<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        let v = crate::as_bson!(&obj);
        let mut v_str = String::new();
        if v.as_str().is_some() {
            v_str = format!("%{}", v.as_str().unwrap());
        } else {
            v_str = format!("%{}", v.to_string());
        }

        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql,column," ",crate::sql::TEMPLATE.like.value," ", &convert_column.as_str());

        self.args.push(crate::as_bson!(&v_str));
        self
    }

    pub fn like_right<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        let v = crate::as_bson!(&obj);
        let mut v_str = String::new();
        if v.as_str().is_some() {
            v_str = format!("{}%", v.as_str().unwrap());
        } else {
            v_str = format!("{}%", v.to_string());
        }

        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql,column," ",crate::sql::TEMPLATE.like.value," ", &convert_column.as_str());

        self.args.push(crate::as_bson!(&v_str));
        self
    }

    pub fn not_like<T>(mut self, column: &str, obj: T) -> Self
        where
            T: Serialize,
    {
        let v = crate::as_bson!(&obj);
        let mut v_str = String::new();
        if v.as_str().is_some() {
            v_str = format!("%{}%", v.as_str().unwrap());
        } else {
            v_str = format!("%{}%", v.to_string());
        }
        let mut convert_column = String::new();
        self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
        self.do_format_column(column, &mut convert_column);
        push_sql!(self.sql,column," ",crate::sql::TEMPLATE.not.value," ",crate::sql::TEMPLATE.like.value," ", &convert_column.as_str());
        self.args.push(crate::as_bson!(&v_str));
        self
    }

    pub fn is_null(mut self, column: &str) -> Self {
        self.sql.push_str(column);
        self.sql.push_str(crate::sql::TEMPLATE.is.left_space);
        self.sql.push_str(crate::sql::TEMPLATE.null.left_space);
        self
    }

    pub fn is_not_null(mut self, column: &str) -> Self {
        self.sql.push_str(column);
        self.sql.push_str(crate::sql::TEMPLATE.is.left_space);
        self.sql.push_str(crate::sql::TEMPLATE.not.left_space);
        self.sql.push_str(crate::sql::TEMPLATE.null.left_space);
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
        push_sql!(self.sql,column," ",crate::sql::TEMPLATE.r#in.value," (");
        for x in obj {
            let mut convert_column = String::new();
            self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
            self.do_format_column(column, &mut convert_column);
            push_sql!(self.sql," ",&convert_column.as_str()," ");
            self.sql.push_str(",");
            self.args.push(crate::as_bson!(x));
        }
        self.sql.pop();
        push_sql!(self.sql,")");
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
        if obj.len() == 0 {
            return self;
        }
        push_sql!(self.sql,column," ",crate::sql::TEMPLATE.not.value," ",crate::sql::TEMPLATE.r#in.value," (");
        for x in obj {
            let mut convert_column = String::new();
            self.driver_type.stmt_convert(self.args.len(), &mut convert_column);
            self.do_format_column(column, &mut convert_column);
            push_sql!(self.sql," ",&convert_column.as_str()," ");
            self.sql.push_str(",");
            self.args.push(crate::as_bson!(x));
        }
        self.sql.pop();
        push_sql!(self.sql,")");
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
            .trim_start_matches(crate::sql::TEMPLATE.and.right_space)
            .trim_end_matches(crate::sql::TEMPLATE.and.left_space)
            .to_string();
        self
    }

    pub fn trim_or(mut self) -> Self {
        self.sql = self
            .sql
            .trim()
            .trim_start_matches(crate::sql::TEMPLATE.or.right_space)
            .trim_end_matches(crate::sql::TEMPLATE.or.left_space)
            .to_owned();
        self
    }

    pub fn trim_and_or(mut self) -> Self {
        self.sql = self
            .sql
            .trim()
            .trim_start_matches(crate::sql::TEMPLATE.and.right_space)
            .trim_end_matches(crate::sql::TEMPLATE.and.left_space)
            .trim_start_matches(crate::sql::TEMPLATE.and.right_space)
            .trim_end_matches(crate::sql::TEMPLATE.and.left_space)
            .to_owned();
        self
    }

    pub fn insert_into(mut self, table_name: &str, columns: &str, values: &str) -> Self {
        if values.starts_with("(") && values.ends_with(")") {
            self.sql = format!(
                "{} {} ({}) {} ({})",
                crate::sql::TEMPLATE.insert_into.value,
                table_name,
                columns,
                crate::sql::TEMPLATE.values.value,
                values
            );
        } else {
            self.sql = format!(
                "{} {} ({}) {} ({})",
                crate::sql::TEMPLATE.insert_into.value,
                table_name,
                columns,
                crate::sql::TEMPLATE.values.value,
                values
            );
        }
        self
    }

    /// limit
    /// for example:
    ///  limit(1) " limit 1 "
    pub fn limit(mut self, limit: u64) -> Self {
        push_sql!(self.sql," ",crate::sql::TEMPLATE.limit.value," ");
        std::fmt::Write::write_fmt(&mut self.sql, format_args!("{}", limit));
        self.sql.push_str(" ");
        self
    }
}


impl Add<&str> for Wrapper {
    type Output = Self;

    fn add(mut self, rhs: &str) -> Self::Output {
        self.sql.push_str(rhs);
        self
    }
}

impl Add<(&str, Bson)> for Wrapper {
    type Output = Self;

    fn add(mut self, rhs: (&str, Bson)) -> Self::Output {
        self.sql.push_str(rhs.0);
        self.args.push(rhs.1);
        self
    }
}

impl Add<(&str, Vec<Bson>)> for Wrapper {
    type Output = Self;

    fn add(mut self, rhs: (&str, Vec<Bson>)) -> Self::Output {
        self.sql.push_str(rhs.0);
        for x in rhs.1 {
            self.args.push(x);
        }
        self
    }
}