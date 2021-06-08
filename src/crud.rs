use std::collections::{BTreeMap, HashMap, HashSet, LinkedList, VecDeque};
use std::fmt::Display;
use std::hash::Hash;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::core::convert::{ResultCodec, StmtConvert};
use crate::core::db::DBExecResult;
use crate::core::db::DriverType;
use crate::core::Error;
use crate::core::Result;
use crate::plugin::page::{IPageRequest, Page};
use crate::plugin::version_lock::VersionLockPlugin;
use crate::rbatis::Rbatis;
use crate::sql::rule::SqlRule;
use crate::utils::string_util::to_snake_name;
use crate::wrapper::Wrapper;
use crate::executor::{RBatisConnExecutor, Executor, RBatisTxExecutor};
use crate::py::PySql;


/// DataBase Table Model trait
///
/// if use #[crud_table] impl Table struct,
/// for example:
///  #[crud_table(id_name:"id"|id_type:"String"|table_name:"biz_activity"|table_columns:"id,name,version,delete_flag"|formats_pg:"id:{}::uuid")]
///
/// if use impl CRUDTable for Table struct,
/// you must impl IdType and id_name() method!
///
pub trait CRUDTable: Send + Sync + Serialize + DeserializeOwned {
    /// get table name,default is type name for snake name
    ///
    /// for Example:  struct  BizActivity{} =>  "biz_activity"
    /// also. you can overwrite this method return ture name
    ///
    ///  impl CRUDTable for BizActivity{
    ///   table_name() -> String{
    ///     "biz_activity".to_string()
    ///   }
    /// }
    ///
    ///
    ///
    fn table_name() -> String {
        let type_name = std::any::type_name::<Self>();
        let mut name = type_name.to_string();
        let names: Vec<&str> = name.split("::").collect();
        name = names.get(names.len() - 1).unwrap_or(&"").to_string();
        return to_snake_name(&name);
    }

    /// get table fields string
    ///
    /// for Example:
    ///   "create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version"
    ///
    /// you also can impl this method for static string
    ///
    /// If a macro is used, the method is overridden by the macro
    fn table_columns() -> String {
        let bean: serde_json::Result<Self> = serde_json::from_str("{}");
        if bean.is_err() {
            //if json decode fail,return '*'
            return " * ".to_string();
        }
        let v = json!(&bean.unwrap());
        if !v.is_object() {
            //if json decode fail,return '*'
            return " * ".to_string();
        }
        let m = v.as_object().unwrap();
        let mut fields = String::new();
        for (k, _) in m {
            fields.push_str(k);
            fields.push_str(",");
        }
        fields.pop();
        return format!("{}", fields);
    }


    ///format column
    fn do_format_column(driver_type: &DriverType, column: &str, data: String) -> String {
        let m = Self::formats(driver_type);
        let source = m.get(column);
        match source {
            Some(s) => s(&data),
            _ => {
                return data.to_string();
            }
        }
    }

    ///return (columns_sql,columns_values_sql,args)
    fn make_value_sql_arg(
        &self,
        db_type: &DriverType,
        index: &mut usize,
    ) -> Result<(String, String, Vec<serde_json::Value>)> {
        let mut value_sql = String::new();
        let mut arr = vec![];
        let cols = Self::table_columns();
        let columns: Vec<&str> = cols.split(",").collect();
        let map;
        match serde_json::json!(self) {
            serde_json::Value::Object(m) => {
                map = m;
            }
            _ => {
                return Err(Error::from("[rbatis] arg not an json object!"));
            }
        }
        let mut column_sql = String::new();
        for column in columns {
            let column = crate::utils::string_util::un_packing_string(column);
            let v = map.get(column).unwrap_or(&serde_json::Value::Null);
            //cast convert
            column_sql = column_sql + column + ",";
            value_sql = value_sql
                + Self::do_format_column(db_type, &column, db_type.stmt_convert(*index)).as_str()
                + ",";
            arr.push(v.to_owned());
            *index += 1;
        }
        column_sql.pop(); //remove ','
        value_sql.pop(); //remove ','
        return Ok((column_sql, value_sql, arr));
    }

    /// return cast chain
    /// column:format_str
    /// for example: HashMap<"id",|arg|“{}::uuid”.to_string()>
    fn formats(
        driver_type: &crate::core::db::DriverType,
    ) -> HashMap<String, fn(arg: &str) -> String> {
        return HashMap::new();
    }


    /// return table column value
    /// If a macro is used, the method is overridden by the macro
    fn get(&self, column: &str) -> serde_json::Value {
        let s = serde_json::json!(self);
        let value = s.get(column);
        match value {
            None => { return serde_json::Value::Null; }
            Some(v) => {
                return v.clone();
            }
        }
    }
}

impl<T> CRUDTable for Option<T>
    where
        T: CRUDTable,
{
    fn table_name() -> String {
        T::table_name()
    }

    fn table_columns() -> String {
        T::table_columns()
    }

    fn formats(driver_type: &DriverType) -> HashMap<String, fn(arg: &str) -> String> {
        T::formats(driver_type)
    }

    fn make_value_sql_arg(
        &self,
        db_type: &DriverType,
        index: &mut usize,
    ) -> Result<(String, String, Vec<serde_json::Value>)> {
        if self.is_none() {
            return Err(crate::core::Error::from(
                "[rbatis] can not make_sql_arg() for None value!",
            ));
        }
        T::make_value_sql_arg(self.as_ref().unwrap(), db_type, index)
    }
}

#[async_trait]
pub trait CRUD {
    async fn save_by_wrapper<T>(
        &self,
        table: &T,
        w: &Wrapper,
    ) -> Result<DBExecResult>
        where
            T: CRUDTable;
    async fn save<T>(&self, table: &T) -> Result<DBExecResult>
        where
            T: CRUDTable;
    async fn save_batch<T>(&self, tables: &[T]) -> Result<DBExecResult>
        where
            T: CRUDTable;

    async fn save_batch_slice<T>(
        &self,
        tables: &[T],
        slice_len: usize,
    ) -> Result<DBExecResult>
        where
            T: CRUDTable;

    async fn remove_by_wrapper<T>(&self, w: &Wrapper) -> Result<u64>
        where
            T: CRUDTable;
    async fn remove_by_column<T, C>(&self, column: &str, column_value: &C) -> Result<u64> where T: CRUDTable, C: Serialize + Send + Sync;

    async fn remove_batch_by_column<T, C>(&self, column: &str, column_values: &[C]) -> Result<u64>
        where
            T: CRUDTable, C: Serialize + Send + Sync;

    async fn update_by_wrapper<T>(
        &self,
        table: &mut T,
        w: &Wrapper,
        update_null_value: bool,
    ) -> Result<u64>
        where
            T: CRUDTable;
    /// update database record by id
    async fn update_by_column<T>(&self, column: &str, table: &mut T) -> Result<u64>
        where
            T: CRUDTable;

    /// remove batch database record by args
    async fn update_batch_by_column<T>(&self, column: &str, tables: &mut [T]) -> Result<u64>
        where
            T: CRUDTable;

    /// fetch database record by id
    async fn fetch_by_column<T, C>(&self, column: &str, value: &C) -> Result<T>
        where
            T: CRUDTable, C: Serialize + Send + Sync;

    /// fetch database record by a wrapper
    async fn fetch_by_wrapper<T>(&self, w: &Wrapper) -> Result<T>
        where
            T: CRUDTable;

    /// count database record by a wrapper
    async fn fetch_count_by_wrapper<T>(&self, w: &Wrapper) -> Result<u64>
        where
            T: CRUDTable;

    /// fetch page database record list by a wrapper
    async fn fetch_page_by_wrapper<T>(
        &self,
        w: &Wrapper,
        page: &dyn IPageRequest,
    ) -> Result<Page<T>>
        where
            T: CRUDTable;

    /// fetch database record list for all
    async fn fetch_list<T>(&self) -> Result<Vec<T>>
        where
            T: CRUDTable;

    /// fetch database record list by a id array
    async fn fetch_list_by_column<T, C>(&self, column: &str, column_values: &[C]) -> Result<Vec<T>>
        where
            T: CRUDTable, C: Serialize + Send + Sync;

    /// fetch database record list by a wrapper
    async fn fetch_list_by_wrapper<T>(&self, w: &Wrapper) -> Result<Vec<T>>
        where
            T: CRUDTable;
}

#[async_trait]
pub trait CRUDMut {
    async fn save_by_wrapper<T>(
        &mut self,
        table: &T,
        w: &Wrapper,
    ) -> Result<DBExecResult>
        where
            T: CRUDTable;
    async fn save<T>(&mut self, table: &T) -> Result<DBExecResult>
        where
            T: CRUDTable;
    async fn save_batch<T>(&mut self, tables: &[T]) -> Result<DBExecResult>
        where
            T: CRUDTable;

    async fn save_batch_slice<T>(
        &mut self,
        tables: &[T],
        slice_len: usize,
    ) -> Result<DBExecResult>
        where
            T: CRUDTable;

    async fn remove_by_wrapper<T>(&mut self, w: &Wrapper) -> Result<u64>
        where
            T: CRUDTable;
    async fn remove_by_column<T, C>(&mut self, column: &str, column_value: &C) -> Result<u64> where T: CRUDTable, C: Serialize + Send + Sync;

    async fn remove_batch_by_column<T, C>(&mut self, column: &str, column_values: &[C]) -> Result<u64>
        where
            T: CRUDTable, C: Serialize + Send + Sync;

    async fn update_by_wrapper<T>(
        &mut self,
        table: &mut T,
        w: &Wrapper,
        update_null_value: bool,
    ) -> Result<u64>
        where
            T: CRUDTable;
    /// update database record by id
    async fn update_by_column<T>(&mut self, column: &str, table: &mut T) -> Result<u64>
        where
            T: CRUDTable;

    /// remove batch database record by tables
    async fn update_batch_by_column<T>(&mut self, column: &str, tables: &mut [T]) -> Result<u64>
        where
            T: CRUDTable;

    /// fetch database record by id
    async fn fetch_by_column<T, C>(&mut self, column: &str, value: &C) -> Result<T>
        where
            T: CRUDTable, C: Serialize + Send + Sync;

    /// fetch database record by a wrapper
    async fn fetch_by_wrapper<T>(&mut self, w: &Wrapper) -> Result<T>
        where
            T: CRUDTable;

    /// count database record by a wrapper
    async fn fetch_count_by_wrapper<T>(&mut self, w: &Wrapper) -> Result<u64>
        where
            T: CRUDTable;

    /// fetch page database record list by a wrapper
    async fn fetch_page_by_wrapper<T>(
        &mut self,
        w: &Wrapper,
        page: &dyn IPageRequest,
    ) -> Result<Page<T>>
        where
            T: CRUDTable;

    /// fetch database record list for all
    async fn fetch_list<T>(&mut self) -> Result<Vec<T>>
        where
            T: CRUDTable;

    /// fetch database record list by a id array
    async fn fetch_list_by_column<T, C>(&mut self, column: &str, column_values: &[C]) -> Result<Vec<T>>
        where
            T: CRUDTable, C: Serialize + Send + Sync;

    /// fetch database record list by a wrapper
    async fn fetch_list_by_wrapper<T>(&mut self, w: &Wrapper) -> Result<Vec<T>>
        where
            T: CRUDTable;
}

#[async_trait]
pub trait ImplCRUD: PySql {
    /// save by wrapper
    async fn save_by_wrapper<T>(
        &mut self,
        table: &T,
        w: &Wrapper,
    ) -> Result<DBExecResult>
        where
            T: CRUDTable,
    {
        if w.sql.starts_with(crate::sql::TEMPLATE.insert_into.value) {
            return self.execute(&w.sql, &w.args).await;
        } else {
            let mut w = w.clone();
            let mut index = 0;
            let (columns, column_values, args) = table.make_value_sql_arg(&self.driver_type()?, &mut index)?;
            let table_name = choose_dyn_table_name::<T>(&w);
            w = w.insert_into(&table_name, &columns, &column_values);
            for x in args {
                w.args.push(x);
            }
            return self.execute(&w.sql, &w.args).await;
        }
    }

    /// save one entity to database
    async fn save<T>(&mut self, table: &T) -> Result<DBExecResult>
        where
            T: CRUDTable,
    {
        let mut index = 0;
        let (columns, values, args) =
            table.make_value_sql_arg(&self.driver_type()?, &mut index)?;
        let sql = format!(
            "{} {} ({}) {} ({})",
            crate::sql::TEMPLATE.insert_into.value,
            T::table_name(),
            columns,
            crate::sql::TEMPLATE.values.value,
            values
        );
        return self.execute(sql.as_str(), &args).await;
    }

    /// save batch makes many value into  only one sql. make sure your data do not too long!
    ///
    /// for Example:
    /// rb.save_batch("",&Cec![activity]);
    /// [rbatis] Exec ==>   insert into biz_activity (id,name,version) values ( ? , ? , ?),( ? , ? , ?)
    ///
    ///
    async fn save_batch<T>(&mut self, tables: &[T]) -> Result<DBExecResult>
        where
            T: CRUDTable,
    {
        if tables.is_empty() {
            return Ok(DBExecResult {
                rows_affected: 0,
                last_insert_id: None,
            });
        }
        let mut value_arr = String::new();
        let mut arg_arr = vec![];
        let mut column_sql = String::new();
        let mut field_index = 0;
        for x in tables {
            let (columns, values, args) =
                x.make_value_sql_arg(&self.driver_type()?, &mut field_index)?;
            if column_sql.is_empty() {
                column_sql = columns;
            }
            value_arr = value_arr + format!("({}),", values).as_str();
            for x in args {
                arg_arr.push(x);
            }
        }
        value_arr.pop(); //pop ','
        let sql = format!(
            "{} {} ({}) {} {}",
            crate::sql::TEMPLATE.insert_into.value,
            T::table_name(),
            column_sql,
            crate::sql::TEMPLATE.values.value,
            value_arr
        );
        return self.execute(sql.as_str(), &arg_arr).await;
    }

    /// save batch slice makes many value into  many sql. make sure your slice_len do not too long!
    /// slice_len = 0 : save all data
    /// slice_len != 0 : save data with slice_len everytime until save all data
    ///
    /// for Example:
    /// rb.save_batch_slice("",&Cec![activity],0);
    /// [rbatis] Exec ==>   insert into biz_activity (id,name,version) values ( ? , ? , ?),( ? , ? , ?)
    ///
    async fn save_batch_slice<T>(
        &mut self,
        tables: &[T],
        slice_len: usize,
    ) -> Result<DBExecResult>
        where
            T: CRUDTable,
    {
        if slice_len == 0 || tables.len() <= slice_len {
            return self.save_batch(tables).await;
        } else {
            let mut temp_result = DBExecResult {
                rows_affected: 0,
                last_insert_id: None,
            };
            let total = tables.len();
            let mut pages = tables.len() / slice_len;
            if total % slice_len != 0 {
                pages = pages + 1;
            }
            for page in 0..pages {
                let mut temp_len = slice_len * (1 + page);
                if temp_len > total {
                    temp_len = total;
                }
                let temp = &tables[page * slice_len..temp_len];
                let result = self.save_batch(temp).await?;
                temp_result.last_insert_id = result.last_insert_id;
                temp_result.rows_affected = result.rows_affected + temp_result.rows_affected;
            }
            return Ok(temp_result);
        }
    }

    /// remove database record by a wrapper
    async fn remove_by_wrapper<T>(&mut self, w: &Wrapper) -> Result<u64>
        where
            T: CRUDTable,
    {
        let table_name = choose_dyn_table_name::<T>(w);
        let where_sql = self.driver_type()?.make_where(&w.sql);
        let mut sql = String::new();
        if self.get_rbatis().logic_plugin.is_some() {
            sql = self.get_rbatis().logic_plugin.as_ref().unwrap().create_remove_sql(
                "",
                &self.driver_type()?,
                &table_name,
                &T::table_columns(),
                &where_sql,
            )?;
        } else {
            sql = format!(
                "{} {} {}",
                crate::sql::TEMPLATE.delete_from.value,
                table_name,
                &where_sql
            );
        }
        return Ok(self
            .execute(sql.as_str(), &w.args)
            .await?
            .rows_affected);
    }

    /// remove database record by id
    async fn remove_by_column<T, C>(&mut self, column: &str, value: &C) -> Result<u64>
        where
            T: CRUDTable, C: Serialize + Send + Sync,
    {
        let mut sql = String::new();
        let driver_type = &self.driver_type()?;
        let id_str = T::do_format_column(&driver_type, column, driver_type.stmt_convert(0));
        if self.get_rbatis().logic_plugin.is_some() {
            sql = self.get_rbatis().logic_plugin.as_ref().unwrap().create_remove_sql(
                "",
                &driver_type,
                T::table_name().as_str(),
                &T::table_columns(),
                format!(
                    "{} {} = {}",
                    crate::sql::TEMPLATE.r#where.value,
                    column,
                    id_str
                )
                    .as_str(),
            )?;
        } else {
            sql = format!(
                "{} {} {} {} = {}",
                crate::sql::TEMPLATE.delete_from.value,
                T::table_name(),
                crate::sql::TEMPLATE.r#where.value,
                column,
                id_str
            );
        }
        return Ok(self
            .execute(&sql, &vec![json!(value)])
            .await?
            .rows_affected);
    }

    ///remove batch id
    /// for Example :
    /// rb.remove_batch_by_column::<BizActivity>(&["1".to_string(),"2".to_string()]).await;
    /// [rbatis] Exec ==> delete from biz_activity where id IN ( ? , ? )
    ///
    async fn remove_batch_by_column<T, C>(&mut self, column: &str, values: &[C]) -> Result<u64>
        where
            T: CRUDTable, C: Serialize + Send + Sync
    {
        if values.is_empty() {
            return Ok(0);
        }
        let w = self
            .get_rbatis()
            .new_wrapper_table::<T>()
            .and()
            .in_array(column, values);
        return self.remove_by_wrapper::<T>(&w).await;
    }

    /// update arg by wrapper
    /// skips:  vec!["null"] ->  will skip  null fields
    /// skips:  vec!["column"] ->  will skip  column fields
    async fn update_by_wrapper<T>(
        &mut self,
        table: &mut T,
        w: &Wrapper,
        skips: Vec<&str>,
    ) -> Result<u64>
        where
            T: CRUDTable,
    {
        let table_name = choose_dyn_table_name::<T>(w);
        let mut args = vec![];
        let mut old_version = serde_json::Value::Null;
        let driver_type = &self.driver_type()?;
        let columns = T::table_columns();
        let columns_vec: Vec<&str> = columns.split(",").collect();
        let map;
        match serde_json::json!(table) {
            serde_json::Value::Object(m) => {
                map = m;
            }
            _ => {
                return Err(Error::from("[rbatis] arg not an json object!"));
            }
        }
        let null = serde_json::Value::Null;
        let mut sets = String::new();
        let mut skip_null_value = false;
        for x in &skips {
            if "null".eq(*x) {
                skip_null_value = true;
            }
        }

        for column in columns_vec {
            //filter
            for x in &skips {
                if column.eq(*x) {
                    continue;
                }
            }
            let mut v = map.get(column).unwrap_or_else(|| &null).clone();
            //filter null
            if skip_null_value && v.is_null() {
                continue;
            }
            sets.push_str(
                format!(
                    " {} = {},",
                    column,
                    T::do_format_column(
                        &driver_type,
                        &column,
                        driver_type.stmt_convert(args.len()),
                    )
                )
                    .as_str(),
            );
            match &self.get_rbatis().version_lock_plugin {
                Some(version_lock_plugin) => {
                    old_version = v.clone();
                    v = version_lock_plugin.try_add_one("", &old_version, column);
                }
                _ => {}
            }
            args.push(v.clone());
        }
        sets.pop();
        let mut wrapper = self.get_rbatis().new_wrapper_table::<T>();
        wrapper.sql = format!(
            "{} {} {} {} ",
            crate::sql::TEMPLATE.update.value,
            table_name,
            crate::sql::TEMPLATE.set.value,
            sets
        );
        wrapper.args = args;

        //version lock
        match self.get_rbatis().version_lock_plugin.as_ref() {
            Some(version_lock_plugin) => {
                let version_sql = version_lock_plugin
                    .as_ref()
                    .try_make_where_sql("", &old_version);
                if !version_sql.is_empty() {
                    if !wrapper
                        .sql
                        .contains(crate::sql::TEMPLATE.r#where.left_right_space)
                    {
                        wrapper
                            .sql
                            .push_str(crate::sql::TEMPLATE.r#where.left_right_space);
                    }
                    wrapper.sql.push_str(&version_sql);
                }
            }
            _ => {}
        }
        if !w.sql.is_empty() {
            if !wrapper
                .sql
                .contains(crate::sql::TEMPLATE.r#where.left_right_space)
            {
                wrapper
                    .sql
                    .push_str(crate::sql::TEMPLATE.r#where.left_right_space);
            }
            wrapper = wrapper.and();
            wrapper = wrapper.push_wrapper(&w);
        }

        let rows_affected = self
            .execute(wrapper.sql.as_str(), &wrapper.args)
            .await?
            .rows_affected;
        if rows_affected > 0 {
            *table = serde_json::from_value(serde_json::Value::Object(map)).into_result()?;
        }
        return Ok(rows_affected);
    }

    /// update database record by id
    async fn update_by_column<T>(&mut self, column: &str, table: &mut T) -> Result<u64>
        where
            T: CRUDTable
    {
        let rb = self
            .get_rbatis();
        let value = table.get(column);
        self.update_by_wrapper(
            table,
            &rb
                .new_wrapper_table::<T>()
                .eq(column, value),
            vec![],
        )
            .await
    }

    /// remove batch database record by args
    async fn update_batch_by_column<T>(&mut self, column: &str, args: &mut [T]) -> Result<u64>
        where
            T: CRUDTable
    {
        let mut updates = 0;
        for x in args {
            updates += self.update_by_column(column, x).await?
        }
        Ok(updates)
    }

    /// fetch database record by a wrapper
    async fn fetch_by_wrapper<T>(&mut self, w: &Wrapper) -> Result<T>
        where
            T: CRUDTable,
    {
        let sql = make_select_sql::<T>(self.get_rbatis(), &T::table_columns(), &w)?;
        return self.fetch(sql.as_str(), &w.args).await;
    }

    /// count database record by a wrapper
    async fn fetch_count_by_wrapper<T>(&mut self, w: &Wrapper) -> Result<u64>
        where
            T: CRUDTable,
    {
        let sql = make_select_sql::<T>(self.get_rbatis(), "count(1)", &w)?;
        return self.fetch(sql.as_str(), &w.args).await;
    }

    /// fetch database record by value
    async fn fetch_by_column<T, C>(&mut self, column: &str, value: &C) -> Result<T>
        where
            T: CRUDTable, C: Serialize + Send + Sync,
    {
        let w = self.get_rbatis().new_wrapper_table::<T>().eq(&column, value);
        return self.fetch_by_wrapper(&w).await;
    }

    /// fetch database record list by a wrapper
    async fn fetch_list_by_wrapper<T>(&mut self, w: &Wrapper) -> Result<Vec<T>>
        where
            T: CRUDTable,
    {
        let sql = make_select_sql::<T>(self.get_rbatis(), &T::table_columns(), &w)?;
        return self.fetch(sql.as_str(), &w.args).await;
    }

    /// fetch database record list for all
    async fn fetch_list<T>(&mut self) -> Result<Vec<T>>
        where
            T: CRUDTable,
    {
        let rb = self.get_rbatis();
        return self.fetch_list_by_wrapper(&rb.new_wrapper_table::<T>())
            .await;
    }

    /// fetch database record list by a id array
    async fn fetch_list_by_column<T, C>(&mut self, column: &str, column_values: &[C]) -> Result<Vec<T>>
        where
            T: CRUDTable, C: Serialize + Send + Sync,
    {
        let w = self.get_rbatis().new_wrapper_table::<T>().in_array(&column, column_values);
        return self.fetch_list_by_wrapper(&w).await;
    }

    /// fetch page database record list by a wrapper
    async fn fetch_page_by_wrapper<T>(
        &mut self,
        w: &Wrapper,
        page: &dyn IPageRequest,
    ) -> Result<Page<T>>
        where
            T: CRUDTable,
    {
        let sql = make_select_sql::<T>(self.get_rbatis(), &T::table_columns(), &w)?;
        self.fetch_page(sql.as_str(), &w.args, page).await
    }
}

impl ImplCRUD for RBatisConnExecutor<'_> {}

impl ImplCRUD for RBatisTxExecutor<'_> {}

/// choose table name
fn choose_dyn_table_name<T>(w: &Wrapper) -> String
    where
        T: CRUDTable,
{
    let mut table_name = T::table_name();
    let table_name_format = w.formats.get("table_name");
    if table_name_format.is_some() {
        match table_name_format {
            Some(table_name_format) => {
                table_name = table_name_format(&table_name);
            }
            _ => {}
        }
    }
    return table_name;
}

fn make_select_sql<T>(rb: &Rbatis, column: &str, w: &Wrapper) -> Result<String>
    where
        T: CRUDTable,
{
    let driver_type = rb.driver_type()?;
    let table_name = choose_dyn_table_name::<T>(w);
    if rb.logic_plugin.is_some() {
        let logic_ref = rb.logic_plugin.as_ref().unwrap();
        return logic_ref.create_select_sql(
            "",
            &driver_type,
            &table_name,
            column,
            &T::table_columns(),
            &w.sql,
        );
    }
    Ok(format!(
        "{} {} {} {} {}",
        crate::sql::TEMPLATE.select.value,
        column,
        crate::sql::TEMPLATE.from.value,
        table_name,
        driver_type.make_where(&w.sql)
    ))
}

#[async_trait]
impl CRUD for Rbatis {
    async fn save_by_wrapper<T>(&self, table: &T, w: &Wrapper) -> Result<DBExecResult> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        conn.save_by_wrapper(table, w).await
    }

    async fn save<T>(&self, table: &T) -> Result<DBExecResult> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        conn.save(table).await
    }

    async fn save_batch<T>(&self, tables: &[T]) -> Result<DBExecResult> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        conn.save_batch(tables).await
    }

    async fn save_batch_slice<T>(&self, tables: &[T], slice_len: usize) -> Result<DBExecResult> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        conn.save_batch_slice(tables, slice_len).await
    }

    async fn remove_by_wrapper<T>(&self, w: &Wrapper) -> Result<u64> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        conn.remove_by_wrapper::<T>(w).await
    }

    async fn remove_by_column<T, C>(&self, column: &str, value: &C) -> Result<u64> where
        T: CRUDTable, C: Serialize + Send + Sync {
        let mut conn = self.acquire().await?;
        conn.remove_by_column::<T, C>(column, value).await
    }

    async fn remove_batch_by_column<T, C>(&self, column: &str, values: &[C]) -> Result<u64> where
        T: CRUDTable, C: Serialize + Send + Sync {
        let mut conn = self.acquire().await?;
        conn.remove_batch_by_column::<T, C>(column, values).await
    }

    async fn update_by_wrapper<T>(&self, table: &mut T, w: &Wrapper, update_null_value: bool) -> Result<u64> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        if update_null_value {
            conn.update_by_wrapper(table, w, vec!["null"]).await
        } else {
            conn.update_by_wrapper(table, w, vec![]).await
        }
    }

    async fn update_by_column<T>(&self, column: &str, table: &mut T) -> Result<u64> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        conn.update_by_column(column, table).await
    }

    async fn update_batch_by_column<T>(&self, column: &str, args: &mut [T]) -> Result<u64> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        conn.update_batch_by_column::<T>(column, args).await
    }

    async fn fetch_by_column<T, C>(&self, column: &str, value: &C) -> Result<T> where
        T: CRUDTable, C: Serialize + Send + Sync {
        let mut conn = self.acquire().await?;
        conn.fetch_by_column::<T, C>(column, value).await
    }

    async fn fetch_by_wrapper<T>(&self, w: &Wrapper) -> Result<T> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        conn.fetch_by_wrapper(w).await
    }

    async fn fetch_count_by_wrapper<T>(&self, w: &Wrapper) -> Result<u64> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        conn.fetch_count_by_wrapper::<T>(w).await
    }

    async fn fetch_page_by_wrapper<T>(&self, w: &Wrapper, page: &dyn IPageRequest) -> Result<Page<T>> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        conn.fetch_page_by_wrapper::<T>(w, page).await
    }

    async fn fetch_list<T>(&self) -> Result<Vec<T>> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        conn.fetch_list().await
    }

    async fn fetch_list_by_column<T, C>(&self, column: &str, column_values: &[C]) -> Result<Vec<T>> where
        T: CRUDTable, C: Serialize + Send + Sync {
        let mut conn = self.acquire().await?;
        conn.fetch_list_by_column::<T, C>(column, column_values).await
    }

    async fn fetch_list_by_wrapper<T>(&self, w: &Wrapper) -> Result<Vec<T>> where
        T: CRUDTable {
        let mut conn = self.acquire().await?;
        conn.fetch_list_by_wrapper(w).await
    }
}