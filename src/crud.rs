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

/// DataBase Table Model trait
///
/// if use #[crud_enable] impl Table struct,
/// for example:
///  #[crud_enable(id_name:"id"|id_type:"String"|table_name:"biz_activity"|table_columns:"id,name,version,delete_flag"|formats_pg:"id:{}::uuid")]
///
/// if use impl CRUDTable for Table struct,
/// you must impl IdType and id_name() method!
///
pub trait CRUDTable: Send + Sync + Serialize + DeserializeOwned {
    /// your table id type,for example:
    /// IdType = String
    /// IdType = i32
    ///
    type IdType: Send + Sync + Clone + Serialize + Display + Eq + PartialEq;

    ///table id column
    fn id_name() -> String {
        "id".to_string()
    }

    /// get struct id field
    fn get_id(&self) -> Option<&Self::IdType>;

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

    /// make an Map<table_column,value>
    fn make_column_value_map(
        &self,
        db_type: &DriverType,
    ) -> Result<serde_json::Map<String, Value>> {
        let json = json!(self);
        if json.eq(&serde_json::Value::Null) {
            return Err(Error::from("[rbaits] to_value_map() fail!"));
        }
        if !json.is_object() {
            return Err(Error::from(
                "[rbaits] to_value_map() fail,data is not an object!",
            ));
        }
        return Ok(json.as_object().unwrap().to_owned());
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

    ///return (columns_sql,values_sql,args)
    fn make_value_sql_arg(
        &self,
        db_type: &DriverType,
        index: &mut usize,
    ) -> Result<(String, String, Vec<serde_json::Value>)> {
        let mut value_sql = String::new();
        let mut arr = vec![];
        let cols = Self::table_columns();
        let columns: Vec<&str> = cols.split(",").collect();
        let map = self.make_column_value_map(db_type)?;
        let mut column_sql = String::new();
        for column in &columns {
            let column = crate::utils::string_util::un_packing_string(column);
            let v = map.get(column).unwrap_or(&serde_json::Value::Null);
            if Self::id_name().eq(column) && v.eq(&serde_json::Value::Null) {
                continue;
            }
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
}

impl<T> CRUDTable for Option<T>
    where
        T: CRUDTable,
{
    type IdType = T::IdType;

    fn id_name() -> String {
        T::id_name()
    }

    fn get_id(&self) -> Option<&Self::IdType> {
        match self {
            Some(s) => {
                return s.get_id();
            }
            None => {
                return None;
            }
        }
    }

    fn table_name() -> String {
        T::table_name()
    }

    fn table_columns() -> String {
        T::table_columns()
    }

    fn formats(driver_type: &DriverType) -> HashMap<String, fn(arg: &str) -> String> {
        T::formats(driver_type)
    }
    fn make_column_value_map(
        &self,
        db_type: &DriverType,
    ) -> Result<serde_json::Map<String, Value>> {
        if self.is_none() {
            return Err(crate::core::Error::from(
                "[rbatis] can not make_column_value_map() for None value!",
            ));
        }
        T::make_column_value_map(self.as_ref().unwrap(), db_type)
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

/// fetch ids, must use Id trait  together
pub trait Ids<C>
    where
        C: CRUDTable,
{
    ///get ids
    fn to_ids(&self) -> Vec<C::IdType>;
}

impl<C> Ids<C> for [C]
    where
        C: CRUDTable,
{
    fn to_ids(&self) -> Vec<C::IdType> {
        let mut vec = vec![];
        for item in self {
            let id = item.get_id();
            if id.is_some() {
                match id {
                    Some(id) => {
                        vec.push(id.clone());
                    }
                    _ => {}
                }
            }
        }
        vec
    }
}

impl<C> Ids<C> for HashSet<C>
    where
        C: CRUDTable,
{
    fn to_ids(&self) -> Vec<C::IdType> {
        let mut vec = vec![];
        for item in self {
            let id = item.get_id();
            if id.is_some() {
                match id {
                    Some(id) => {
                        vec.push(id.clone());
                    }
                    _ => {}
                }
            }
        }
        vec
    }
}

impl<C> Ids<C> for VecDeque<C>
    where
        C: CRUDTable,
{
    fn to_ids(&self) -> Vec<C::IdType> {
        let mut vec = vec![];
        for item in self {
            let id = item.get_id();
            if id.is_some() {
                match id {
                    Some(id) => {
                        vec.push(id.clone());
                    }
                    _ => {}
                }
            }
        }
        vec
    }
}

impl<C> Ids<C> for LinkedList<C>
    where
        C: CRUDTable,
{
    fn to_ids(&self) -> Vec<C::IdType> {
        let mut vec = vec![];
        for item in self {
            let id = item.get_id();
            if id.is_some() {
                match id {
                    Some(id) => {
                        vec.push(id.clone());
                    }
                    _ => {}
                }
            }
        }
        vec
    }
}

impl<K, C> Ids<C> for HashMap<K, C>
    where
        C: CRUDTable,
{
    fn to_ids(&self) -> Vec<C::IdType> {
        let mut vec = vec![];
        for (_, item) in self {
            let id = item.get_id();
            if id.is_some() {
                match id {
                    Some(id) => {
                        vec.push(id.clone());
                    }
                    _ => {}
                }
            }
        }
        vec
    }
}

impl<K, C> Ids<C> for BTreeMap<K, C>
    where
        C: CRUDTable,
{
    fn to_ids(&self) -> Vec<C::IdType> {
        let mut vec = vec![];
        for (_, item) in self {
            let id = item.get_id();
            if id.is_some() {
                match id {
                    Some(id) => {
                        vec.push(id.clone());
                    }
                    _ => {}
                }
            }
        }
        vec
    }
}

#[async_trait]
pub trait CRUD {
    async fn save_by_wrapper<T>(
        &self,
        context_id: &str,
        entity: &T,
        w: &Wrapper,
    ) -> Result<DBExecResult>
        where
            T: CRUDTable;
    async fn save<T>(&self, context_id: &str, entity: &T) -> Result<DBExecResult>
        where
            T: CRUDTable;
    async fn save_batch<T>(&self, context_id: &str, entity: &[T]) -> Result<DBExecResult>
        where
            T: CRUDTable;

    async fn save_batch_slice<T>(&self, context_id: &str, entity: &[T], slice_len: usize) -> Result<DBExecResult>
        where
            T: CRUDTable;

    async fn remove_by_wrapper<T>(&self, context_id: &str, w: &Wrapper) -> Result<u64>
        where
            T: CRUDTable;
    async fn remove_by_id<T>(&self, context_id: &str, id: &T::IdType) -> Result<u64>
        where
            T: CRUDTable;
    async fn remove_batch_by_id<T>(&self, context_id: &str, ids: &[T::IdType]) -> Result<u64>
        where
            T: CRUDTable;

    async fn update_by_wrapper<T>(
        &self,
        context_id: &str,
        arg: &mut T,
        w: &Wrapper,
        update_null_value: bool,
    ) -> Result<u64>
        where
            T: CRUDTable;
    /// update database record by id
    async fn update_by_id<T>(&self, context_id: &str, arg: &mut T) -> Result<u64>
        where
            T: CRUDTable;

    /// remove batch database record by args
    async fn update_batch_by_id<T>(&self, context_id: &str, ids: &mut [T]) -> Result<u64>
        where
            T: CRUDTable;

    /// fetch database record by id
    async fn fetch_by_id<T>(&self, context_id: &str, id: &T::IdType) -> Result<T>
        where
            T: CRUDTable;

    /// fetch database record by a wrapper
    async fn fetch_by_wrapper<T>(&self, context_id: &str, w: &Wrapper) -> Result<T>
        where
            T: CRUDTable;

    /// count database record by a wrapper
    async fn fetch_count_by_wrapper<T>(&self, context_id: &str, w: &Wrapper) -> Result<u64>
        where
            T: CRUDTable;

    /// fetch page database record list by a wrapper
    async fn fetch_page_by_wrapper<T>(
        &self,
        context_id: &str,
        w: &Wrapper,
        page: &dyn IPageRequest,
    ) -> Result<Page<T>>
        where
            T: CRUDTable;

    /// fetch database record list for all
    async fn fetch_list<T>(&self, context_id: &str) -> Result<Vec<T>>
        where
            T: CRUDTable;

    /// fetch database record list by a id array
    async fn fetch_list_by_ids<T>(&self, context_id: &str, ids: &[T::IdType]) -> Result<Vec<T>>
        where
            T: CRUDTable;

    /// fetch database record list by a wrapper
    async fn fetch_list_by_wrapper<T>(&self, context_id: &str, w: &Wrapper) -> Result<Vec<T>>
        where
            T: CRUDTable;
}

#[async_trait]
impl CRUD for Rbatis {
    /// save by wrapper
    async fn save_by_wrapper<T>(
        &self,
        context_id: &str,
        entity: &T,
        w: &Wrapper,
    ) -> Result<DBExecResult>
        where
            T: CRUDTable,
    {
        if w.sql.starts_with(crate::sql::TEMPLATE.insert_into) {
            return self.exec_prepare(context_id, &w.sql, &w.args).await;
        } else {
            let mut w = w.clone();
            let mut index = 0;
            let (columns, values, args) =
                entity.make_value_sql_arg(&self.driver_type()?, &mut index)?;
            let table_name = choose_dyn_table_name::<T>(&w);
            w = w.insert_into(&table_name, &columns, &values);
            for x in args {
                w.args.push(x);
            }
            return self.exec_prepare(context_id, &w.sql, &w.args).await;
        }
    }

    /// save one entity to database
    async fn save<T>(&self, context_id: &str, entity: &T) -> Result<DBExecResult>
        where
            T: CRUDTable,
    {
        let mut index = 0;
        let (columns, values, args) =
            entity.make_value_sql_arg(&self.driver_type()?, &mut index)?;
        let sql = format!(
            "{} {} ({}){}({})",
            crate::sql::TEMPLATE.insert_into,
            T::table_name(),
            columns,
            crate::sql::TEMPLATE.values,
            values
        );
        return self.exec_prepare(context_id, sql.as_str(), &args).await;
    }

    /// save batch makes many value into  only one sql. make sure your data do not too long!
    ///
    /// for Example:
    /// rb.save_batch("",&vec![activity]);
    /// [rbatis] Exec ==>   insert into biz_activity (id,name,version) values ( ? , ? , ?),( ? , ? , ?)
    ///
    ///
    async fn save_batch<T>(&self, context_id: &str, args: &[T]) -> Result<DBExecResult>
        where
            T: CRUDTable,
    {
        if args.is_empty() {
            return Ok(DBExecResult {
                rows_affected: 0,
                last_insert_id: None,
            });
        }
        let mut value_arr = String::new();
        let mut arg_arr = vec![];
        let mut column_sql = String::new();
        let mut field_index = 0;
        for x in args {
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
            "{}{} ({}){}{}",
            crate::sql::TEMPLATE.insert_into,
            T::table_name(),
            column_sql,
            crate::sql::TEMPLATE.values,
            value_arr
        );
        return self.exec_prepare(context_id, sql.as_str(), &arg_arr).await;
    }

    /// save batch slice makes many value into  many sql. make sure your slice_len do not too long!
    /// slice_len = 0 : save all data
    /// slice_len != 0 : save data with slice_len everytime until save all data
    ///
    /// for Example:
    /// rb.save_batch_slice("",&vec![activity],0);
    /// [rbatis] Exec ==>   insert into biz_activity (id,name,version) values ( ? , ? , ?),( ? , ? , ?)
    ///
    async fn save_batch_slice<T>(&self, context_id: &str, args: &[T], slice_len: usize) -> Result<DBExecResult>
        where
            T: CRUDTable,
    {
        if slice_len == 0 || args.len() <= slice_len {
            return self.save_batch(context_id, args).await;
        } else {
            let mut temp_result = DBExecResult { rows_affected: 0, last_insert_id: None };
            let total = args.len();
            let mut pages = args.len() / slice_len;
            if total % slice_len != 0 {
                pages = pages + 1;
            }
            for page in 0..pages {
                let mut temp_len = slice_len * (1 + page);
                if temp_len > total {
                    temp_len = total;
                }
                let temp = &args[page * slice_len..temp_len];
                let result = self.save_batch(context_id, temp).await?;
                temp_result.last_insert_id = result.last_insert_id;
                temp_result.rows_affected = result.rows_affected + temp_result.rows_affected;
            }
            return Ok(temp_result);
        }
    }

    /// remove database record by a wrapper
    async fn remove_by_wrapper<T>(&self, context_id: &str, w: &Wrapper) -> Result<u64>
        where
            T: CRUDTable,
    {
        let table_name = choose_dyn_table_name::<T>(w);
        let where_sql = self.driver_type()?.make_where(&w.sql);
        let mut sql = String::new();
        if self.logic_plugin.is_some() {
            sql = self.logic_plugin.as_ref().unwrap().create_remove_sql(
                context_id,
                &self.driver_type()?,
                &table_name,
                &T::table_columns(),
                &where_sql,
            )?;
        } else {
            sql = format!("{}{} {}", crate::sql::TEMPLATE.delete_from, table_name, &where_sql);
        }
        return Ok(self
            .exec_prepare(context_id, sql.as_str(), &w.args)
            .await?
            .rows_affected);
    }

    /// remove database record by id
    async fn remove_by_id<T>(&self, context_id: &str, id: &T::IdType) -> Result<u64>
        where
            T: CRUDTable,
    {
        let mut sql = String::new();
        let driver_type = &self.driver_type()?;
        let id_str = T::do_format_column(&driver_type, &T::id_name(), driver_type.stmt_convert(0));
        if self.logic_plugin.is_some() {
            sql = self.logic_plugin.as_ref().unwrap().create_remove_sql(
                context_id,
                &driver_type,
                T::table_name().as_str(),
                &T::table_columns(),
                format!("{}{} = {}", crate::sql::TEMPLATE.r#where, T::id_name(), id_str).as_str(),
            )?;
        } else {
            sql = format!(
                "{} {} {} {} = {}",
                crate::sql::TEMPLATE.delete_from,
                T::table_name(),
                crate::sql::TEMPLATE.r#where,
                T::id_name(),
                id_str
            );
        }
        return Ok(self
            .exec_prepare(context_id, sql.as_str(), &vec![json!(id)])
            .await?
            .rows_affected);
    }

    ///remove batch id
    /// for Example :
    /// rb.remove_batch_by_id::<BizActivity>(&["1".to_string(),"2".to_string()]).await;
    /// [rbatis] Exec ==> delete from biz_activity where id IN ( ? , ? )
    ///
    async fn remove_batch_by_id<T>(&self, context_id: &str, ids: &[T::IdType]) -> Result<u64>
        where
            T: CRUDTable,
    {
        if ids.is_empty() {
            return Ok(0);
        }
        let w = self
            .new_wrapper_table::<T>()
            .and()
            .in_array(&T::id_name(), &ids);
        return self.remove_by_wrapper::<T>(context_id, &w).await;
    }

    /// update arg by wrapper
    async fn update_by_wrapper<T>(
        &self,
        context_id: &str,
        arg: &mut T,
        w: &Wrapper,
        update_null_value: bool,
    ) -> Result<u64>
        where
            T: CRUDTable,
    {
        let table_name = choose_dyn_table_name::<T>(w);
        let mut args = vec![];
        let mut old_version = serde_json::Value::Null;
        let driver_type = &self.driver_type()?;
        let mut map = arg.make_column_value_map(&driver_type)?;
        let mut sets = String::new();
        for (column, v) in &mut map {
            //filter id
            if column.eq(&T::id_name()) {
                continue;
            }
            //filter null
            if !update_null_value && v.is_null() {
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
            match &self.version_lock_plugin {
                Some(version_lock_plugin) => {
                    old_version = v.clone();
                    *v = version_lock_plugin.try_add_one(context_id, &old_version, column);
                }
                _ => {}
            }
            args.push(v.clone());
        }
        sets.pop();
        let mut wrapper = self.new_wrapper_table::<T>();
        wrapper.sql = format!("{}{}{}{}", crate::sql::TEMPLATE.update, table_name, crate::sql::TEMPLATE.set, sets);
        wrapper.args = args;
        //version lock
        match self.version_lock_plugin.as_ref() {
            Some(version_lock_plugin) => {
                wrapper.sql.push_str(
                    &version_lock_plugin
                        .as_ref()
                        .try_make_where_sql(context_id, &old_version),
                );
            }
            _ => {}
        }
        if !w.sql.is_empty() {
            if !wrapper.sql.contains(crate::sql::TEMPLATE.r#where) {
                wrapper.sql.push_str(crate::sql::TEMPLATE.r#where);
            }
            wrapper = wrapper.push_wrapper(&w);
        }
        let rows_affected = self
            .exec_prepare(context_id, wrapper.sql.as_str(), &wrapper.args)
            .await?
            .rows_affected;
        if rows_affected > 0 {
            *arg = serde_json::from_value(serde_json::Value::Object(map)).into_result()?;
        }
        return Ok(rows_affected);
    }

    /// update database record by id
    async fn update_by_id<T>(&self, context_id: &str, arg: &mut T) -> Result<u64>
        where
            T: CRUDTable,
    {
        let id = arg.get_id();
        if id.is_none() {
            let id_name = T::id_name();
            return Err(crate::core::Error::from(format!(
                "[rbatis] update_by_id() arg.{} can no be none!",
                id_name
            )));
        }
        self.update_by_wrapper(
            context_id,
            arg,
            &self
                .new_wrapper_table::<T>()
                .eq(&T::id_name(), arg.get_id()),
            false,
        )
            .await
    }

    /// remove batch database record by args
    async fn update_batch_by_id<T>(&self, context_id: &str, args: &mut [T]) -> Result<u64>
        where
            T: CRUDTable,
    {
        let mut updates = 0;
        for x in args {
            updates += self.update_by_id(context_id, x).await?
        }
        Ok(updates)
    }

    /// fetch database record by a wrapper
    async fn fetch_by_wrapper<T>(&self, context_id: &str, w: &Wrapper) -> Result<T>
        where
            T: CRUDTable,
    {
        let sql = make_select_sql::<T>(context_id, &self, &T::table_columns(), &w)?;
        return self.fetch_prepare(context_id, sql.as_str(), &w.args).await;
    }

    /// count database record by a wrapper
    async fn fetch_count_by_wrapper<T>(&self, context_id: &str, w: &Wrapper) -> Result<u64>
        where
            T: CRUDTable,
    {
        let sql = make_select_sql::<T>(context_id, &self, "count(1)", &w)?;
        return self.fetch_prepare(context_id, sql.as_str(), &w.args).await;
    }

    /// fetch database record by id
    async fn fetch_by_id<T>(&self, context_id: &str, id: &T::IdType) -> Result<T>
        where
            T: CRUDTable,
    {
        let w = self.new_wrapper_table::<T>().eq(&T::id_name(), id);
        return self.fetch_by_wrapper(context_id, &w).await;
    }

    /// fetch database record list by a wrapper
    async fn fetch_list_by_wrapper<T>(&self, context_id: &str, w: &Wrapper) -> Result<Vec<T>>
        where
            T: CRUDTable,
    {
        let sql = make_select_sql::<T>(context_id, &self, &T::table_columns(), &w)?;
        return self.fetch_prepare(context_id, sql.as_str(), &w.args).await;
    }

    /// fetch database record list for all
    async fn fetch_list<T>(&self, context_id: &str) -> Result<Vec<T>>
        where
            T: CRUDTable,
    {
        return self
            .fetch_list_by_wrapper(context_id, &self.new_wrapper_table::<T>())
            .await;
    }

    /// fetch database record list by a id array
    async fn fetch_list_by_ids<T>(&self, context_id: &str, ids: &[T::IdType]) -> Result<Vec<T>>
        where
            T: CRUDTable,
    {
        let w = self.new_wrapper_table::<T>().in_array(&T::id_name(), ids);
        return self.fetch_list_by_wrapper(context_id, &w).await;
    }

    /// fetch page database record list by a wrapper
    async fn fetch_page_by_wrapper<T>(
        &self,
        context_id: &str,
        w: &Wrapper,
        page: &dyn IPageRequest,
    ) -> Result<Page<T>>
        where
            T: CRUDTable,
    {
        let sql = make_select_sql::<T>(context_id, &self, &T::table_columns(), &w)?;
        self.fetch_page(context_id, sql.as_str(), &w.args, page)
            .await
    }
}

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

fn make_select_sql<T>(context_id: &str, rb: &Rbatis, column: &str, w: &Wrapper) -> Result<String>
    where
        T: CRUDTable,
{
    let driver_type = rb.driver_type()?;
    let table_name = choose_dyn_table_name::<T>(w);
    if rb.logic_plugin.is_some() {
        let logic_ref = rb.logic_plugin.as_ref().unwrap();
        return logic_ref.create_select_sql(
            context_id,
            &driver_type,
            &table_name,
            column,
            &T::table_columns(),
            &w.sql,
        );
    }
    Ok(format!(
        "{} {} {} {} {}",
        crate::sql::TEMPLATE.select,
        column,
        crate::sql::TEMPLATE.from,
        table_name,
        driver_type.make_where(&w.sql)
    ))
}
