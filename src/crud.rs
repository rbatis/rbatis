use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::export::fmt::Display;
use serde::Serialize;
use serde_json::{Map, Value};

use rbatis_core::convert::StmtConvert;
use rbatis_core::db::DriverType;
use rbatis_core::Error;
use rbatis_core::Result;

use crate::plugin::logic_delete::LogicAction;
use crate::plugin::page::{IPageRequest, Page};
use crate::rbatis::Rbatis;
use crate::sql::date::DateFormat;
use crate::utils::string_util::to_snake_name;
use crate::wrapper::Wrapper;
use rbatis_core::db_adapter::DBExecResult;

/// DB Table model trait
pub trait CRUDEnable: Send + Sync + Serialize + DeserializeOwned {
    /// your table id type,for example:
    /// IdType = String
    /// IdType = i32
    ///
    type IdType: Send + Sync + Serialize + Display;
    /// get table name,default is type name for snake name
    ///
    /// for Example:  struct  BizActivity{} =>  "biz_activity"
    /// also. you can overwrite this method return ture name
    ///
    ///  impl CRUDEnable for BizActivity{
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
        name = names.get(names.len() - 1).unwrap().to_string();
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
        let v = serde_json::to_value(&bean.unwrap()).unwrap_or(serde_json::Value::Null);
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
    ///TODO macro driver auto create this methods
    fn make_column_value_map(&self, db_type: &DriverType) -> Result<serde_json::Map<String, Value>> {
        let json = serde_json::to_value(self).unwrap_or(serde_json::Value::Null);
        if json.eq(&serde_json::Value::Null) {
            return Err(Error::from("[rbaits] to_value_map() fail!"));
        }
        if !json.is_object() {
            return Err(Error::from("[rbaits] to_value_map() fail,data is not an object!"));
        }
        return Ok(json.as_object().unwrap().to_owned());
    }

    ///return (value sql,args)
    fn make_value_sql_arg(&self, db_type: &DriverType, index: &mut usize) -> Result<(String, Vec<serde_json::Value>)> {
        let mut sql = String::new();
        let mut arr = vec![];
        let chains = Self::format_chain();

        let cols = Self::table_columns();
        let columns: Vec<&str> = cols.split(",").collect();
        let map = self.make_column_value_map(db_type)?;
        for column in &columns {
            let v = map.get(&column.to_string()).unwrap_or(&serde_json::Value::Null);
            //cast convert
            let mut column_sql = db_type.stmt_convert(*index);
            // cast column name
            for chain in &chains {
                chain.format(&db_type, column, &mut column_sql, v)?;
            }
            sql = sql + column_sql.as_str() + ",";
            arr.push(v.to_owned());
            *index += 1;
        }
        sql.pop();//remove ','
        return Ok((sql, arr));
    }

    /// return cast chain,
    /// you also can rewrite this method,
    /// but do not forget push DateFormat(if you need)
    fn format_chain() -> Vec<Box<dyn ColumnFormat>> {
        let chain: Vec<Box<dyn ColumnFormat>> = vec![Box::new(DateFormat {})];
        return chain;
    }
}

/// cast sql cloumn and return new sql
pub trait ColumnFormat: Send + Sync {
    ///column: table column
    ///value_sql: set column = value_sql
    fn format(&self, driver_type: &DriverType, column: &str, value_sql: &mut String, value: &serde_json::Value) -> rbatis_core::Result<()>;
}

impl<T> CRUDEnable for Option<T> where T: CRUDEnable {
    type IdType = T::IdType;

    fn table_name() -> String {
        T::table_name()
    }

    fn table_columns() -> String {
        T::table_columns()
    }

    fn format_chain() -> Vec<Box<dyn ColumnFormat>> {
        T::format_chain()
    }

    fn make_column_value_map(&self, db_type: &DriverType) -> Result<serde_json::Map<String, Value>> {
        if self.is_none() {
            return Err(rbatis_core::Error::from("[rbatis] can not make_column_value_map() for None value!"));
        }
        T::make_column_value_map(self.as_ref().unwrap(), db_type)
    }

    fn make_value_sql_arg(&self, db_type: &DriverType, index: &mut usize) -> Result<(String, Vec<serde_json::Value>)> {
        if self.is_none() {
            return Err(rbatis_core::Error::from("[rbatis] can not make_sql_arg() for None value!"));
        }
        T::make_value_sql_arg(self.as_ref().unwrap(), db_type, index)
    }
}

/// fetch id value
///
/// for example:
///     impl Id for BizActivity {
///         type IdType = String;
///
///         fn get_id(&self) -> Option<Self::IdType> {
///             self.id.clone()
///         }
///     }
/// let vec = vec![BizActivity {
///             id: Some("12312".to_string())
///         }];
///         let ids = vec.to_ids();
///         println!("{:?}", ids);
///
pub trait Id {
    type IdType: Send + Sync + DeserializeOwned + Serialize + Display;
    fn get_id(&self) -> Option<Self::IdType>;
}

/// fetch ids, must use Id trait  together
pub trait Ids<C> where C: Id {
    ///get ids
    fn to_ids(&self) -> Vec<C::IdType>;
}

impl<C> Ids<C> for Vec<C> where C: Id {
    fn to_ids(&self) -> Vec<C::IdType> {
        let mut vec = vec![];
        for item in self {
            let id = item.get_id();
            if id.is_some() {
                vec.push(id.unwrap());
            }
        }
        vec
    }
}

#[async_trait]
pub trait CRUD {
    /// tx_id: Transaction id,default ""
    async fn save<T>(&self, tx_id: &str, entity: &T) -> Result<DBExecResult> where T: CRUDEnable;
    async fn save_batch<T>(&self, tx_id: &str, entity: &[T]) -> Result<DBExecResult> where T: CRUDEnable;


    async fn remove_by_wrapper<T>(&self, tx_id: &str, w: &Wrapper) -> Result<u64> where T: CRUDEnable;
    async fn remove_by_id<T>(&self, tx_id: &str, id: &T::IdType) -> Result<u64> where T: CRUDEnable;
    async fn remove_batch_by_id<T>(&self, tx_id: &str, ids: &[T::IdType]) -> Result<u64> where T: CRUDEnable;

    async fn update_by_wrapper<T>(&self, tx_id: &str, arg: &T, w: &Wrapper, update_null_value: bool) -> Result<u64> where T: CRUDEnable;
    async fn update_by_id<T>(&self, tx_id: &str, arg: &T) -> Result<u64> where T: CRUDEnable;
    async fn update_batch_by_id<T>(&self, tx_id: &str, ids: &[T]) -> Result<u64> where T: CRUDEnable;

    async fn fetch_by_wrapper<T>(&self, tx_id: &str, w: &Wrapper) -> Result<T> where T: CRUDEnable;
    async fn fetch_by_id<T>(&self, tx_id: &str, id: &T::IdType) -> Result<T> where T: CRUDEnable;
    async fn fetch_page_by_wrapper<T>(&self, tx_id: &str, w: &Wrapper, page: &dyn IPageRequest) -> Result<Page<T>> where T: CRUDEnable;

    ///fetch all record
    async fn list<T>(&self, tx_id: &str) -> Result<Vec<T>> where T: CRUDEnable;
    async fn list_by_wrapper<T>(&self, tx_id: &str, w: &Wrapper) -> Result<Vec<T>> where T: CRUDEnable;
    async fn list_by_ids<T>(&self, tx_id: &str, ids: &[T::IdType]) -> Result<Vec<T>> where T: CRUDEnable;
}

#[async_trait]
impl CRUD for Rbatis {
    /// save one entity to database
    async fn save<T>(&self, tx_id: &str, entity: &T) -> Result<DBExecResult>
        where T: CRUDEnable {
        let mut index = 0;
        let (values, args) = entity.make_value_sql_arg(&self.driver_type()?, &mut index)?;
        let sql = format!("INSERT INTO {} ({}) VALUES ({})", T::table_name(), T::table_columns(), values);
        return self.exec_prepare(tx_id, sql.as_str(), &args).await;
    }

    /// save batch makes many value into  only one sql. make sure your data not  to long!
    ///
    /// for Example:
    /// rb.save_batch(&vec![activity]);
    /// [rbatis] Exec ==>   INSERT INTO biz_activity (id,name,version) VALUES ( ? , ? , ?),( ? , ? , ?)
    ///
    ///
    async fn save_batch<T>(&self, tx_id: &str, args: &[T]) -> Result<DBExecResult> where T: CRUDEnable {
        if args.is_empty() {
            return Ok(DBExecResult{
                rows_affected: 0,
                last_insert_id: None
            });
        }
        let mut value_arr = String::new();
        let mut arg_arr = vec![];
        let mut columns = "".to_string();
        let mut field_index = 0;
        for x in args {
            if columns.is_empty() {
                columns = T::table_columns();
            }
            let (values, args) = x.make_value_sql_arg(&self.driver_type()?, &mut field_index)?;
            value_arr = value_arr + format!("({}),", values).as_str();
            for x in args {
                arg_arr.push(x);
            }
        }
        value_arr.pop();//pop ','
        let sql = format!("INSERT INTO {} ({}) VALUES {}", T::table_name(), columns, value_arr);
        return self.exec_prepare(tx_id, sql.as_str(), &arg_arr).await;
    }

    async fn remove_by_wrapper<T>(&self, tx_id: &str, w: &Wrapper) -> Result<u64> where T: CRUDEnable {
        let w = match w.checked {
            false => w.clone().check()?,
            _ => w.clone()
        };
        let where_sql = w.sql.as_str();
        let mut sql = String::new();
        if self.logic_plugin.is_some() {
            sql = self.logic_plugin.as_ref().unwrap().create_remove_sql(&self.driver_type()?, T::table_name().as_str(), &T::table_columns(), make_where_sql(where_sql).as_str())?;
        } else {
            sql = format!("DELETE FROM {} {}", T::table_name(), make_where_sql(where_sql));
        }
        return Ok(self.exec_prepare(tx_id, sql.as_str(), &w.args).await?.rows_affected);
    }

    async fn remove_by_id<T>(&self, tx_id: &str, id: &T::IdType) -> Result<u64> where T: CRUDEnable {
        let mut sql = String::new();
        if self.logic_plugin.is_some() {
            sql = self.logic_plugin.as_ref().unwrap().create_remove_sql(&self.driver_type()?, T::table_name().as_str(), &T::table_columns(), format!(" WHERE id = {}", id).as_str())?;
        } else {
            sql = format!("DELETE FROM {} WHERE id = {}", T::table_name(), id);
        }
        return Ok(self.exec_prepare(tx_id, sql.as_str(), &vec![]).await?.rows_affected);
    }

    ///remove batch id
    /// for Example :
    /// rb.remove_batch_by_id::<BizActivity>(&["1".to_string(),"2".to_string()]).await;
    /// [rbatis] Exec ==> DELETE FROM biz_activity WHERE id IN ( ? , ? )
    ///
    async fn remove_batch_by_id<T>(&self, tx_id: &str, ids: &[T::IdType]) -> Result<u64> where T: CRUDEnable {
        if ids.is_empty() {
            return Ok(0);
        }
        let w = Wrapper::new(&self.driver_type()?).and().in_array("id", &ids).check()?;
        return self.remove_by_wrapper::<T>(tx_id, &w).await;
    }

    /// update arg by wrapper
    async fn update_by_wrapper<T>(&self, tx_id: &str, arg: &T, w: &Wrapper, update_null_value: bool) -> Result<u64> where T: CRUDEnable {
        let w = match w.checked {
            false => w.clone().check()?,
            _ => w.clone()
        };
        let mut args = vec![];
        let map = arg.make_column_value_map(&self.driver_type()?)?;
        let driver_type = &self.driver_type()?;

        let chain = T::format_chain();
        let mut sets = String::new();
        for (column, v) in map {
            //filter id
            if column.eq("id") {
                continue;
            }
            //filter null
            if !update_null_value && v.is_null() {
                continue;
            }
            let mut value_column = driver_type.stmt_convert(args.len());
            for item in &chain {
                item.format(driver_type, &column, &mut value_column, &v)?;
            }
            sets.push_str(format!(" {} = {},", column, value_column).as_str());
            args.push(v);
        }
        sets.pop();
        let mut wrapper = Wrapper::new(&self.driver_type()?);
        wrapper.sql = format!("UPDATE {} SET {}", T::table_name(), sets);
        wrapper.args = args;
        if !w.sql.is_empty() {
            wrapper.sql.push_str(" WHERE ");
            wrapper = wrapper.push_wrapper(&w).check()?;
        }
        return Ok(self.exec_prepare(tx_id, wrapper.sql.as_str(), &wrapper.args).await?.rows_affected);
    }

    async fn update_by_id<T>(&self, tx_id: &str, arg: &T) -> Result<u64> where T: CRUDEnable {
        let map = serde_json::to_value(arg).unwrap();
        if !map.is_object() {
            return Err(rbatis_core::Error::from("[rbatis] update_by_id() arg must be an object/struct!"));
        }
        let map = map.as_object().unwrap();
        let id = map.get("id");
        if id.is_none() {
            return Err(rbatis_core::Error::from("[rbatis] update_by_id() arg's id can no be none!"));
        }
        self.update_by_wrapper(tx_id, arg, Wrapper::new(&self.driver_type()?).eq("id", id), false).await
    }

    async fn update_batch_by_id<T>(&self, tx_id: &str, args: &[T]) -> Result<u64> where T: CRUDEnable {
        let mut updates = 0;
        for x in args {
            updates += self.update_by_id(tx_id, x).await?
        }
        Ok(updates)
    }

    async fn fetch_by_wrapper<T>(&self, tx_id: &str, w: &Wrapper) -> Result<T> where T: CRUDEnable {
        let w = match w.checked {
            false => w.clone().check()?,
            _ => w.clone()
        };
        let sql = make_select_sql::<T>(&self, &w)?;
        return self.fetch_prepare(tx_id, sql.as_str(), &w.args).await;
    }

    async fn fetch_by_id<T>(&self, tx_id: &str, id: &T::IdType) -> Result<T> where T: CRUDEnable {
        let w = Wrapper::new(&self.driver_type()?).eq("id", id).check()?;
        return self.fetch_by_wrapper(tx_id, &w).await;
    }

    async fn list_by_wrapper<T>(&self, tx_id: &str, w: &Wrapper) -> Result<Vec<T>> where T: CRUDEnable {
        let w = match w.checked {
            false => w.clone().check()?,
            _ => w.clone()
        };
        let sql = make_select_sql::<T>(&self, &w)?;
        return self.fetch_prepare(tx_id, sql.as_str(), &w.args).await;
    }

    async fn list<T>(&self, tx_id: &str) -> Result<Vec<T>> where T: CRUDEnable {
        return self.list_by_wrapper(tx_id, &Wrapper::new(&self.driver_type()?)).await;
    }

    async fn list_by_ids<T>(&self, tx_id: &str, ids: &[T::IdType]) -> Result<Vec<T>> where T: CRUDEnable {
        let w = Wrapper::new(&self.driver_type()?).in_array("id", ids).check()?;
        return self.list_by_wrapper(tx_id, &w).await;
    }

    async fn fetch_page_by_wrapper<T>(&self, tx_id: &str, w: &Wrapper, page: &dyn IPageRequest) -> Result<Page<T>> where T: CRUDEnable {
        let w = match w.checked {
            false => w.clone().check()?,
            _ => w.clone()
        };
        let sql = make_select_sql::<T>(&self, &w)?;
        self.fetch_page(tx_id, sql.as_str(), &w.args, page).await
    }
}

fn make_where_sql(arg: &str) -> String {
    let mut where_sql = arg.to_string();
    where_sql = where_sql.trim_start().trim_start_matches("AND ").trim_start_matches("OR ").to_string();
    format!(" WHERE {} ", where_sql)
}

fn make_select_sql<T>(rb: &Rbatis, w: &Wrapper) -> Result<String> where T: CRUDEnable {
    let where_sql = w.sql.clone();
    let mut sql = String::new();
    if rb.logic_plugin.is_some() {
        let logic_ref = rb.logic_plugin.as_ref().unwrap();
        return logic_ref.create_select_sql(&rb.driver_type()?, &T::table_name(), &T::table_columns(), &where_sql);
    }
    if !where_sql.is_empty() {
        sql = format!("SELECT {} FROM {} WHERE {}", T::table_columns(), T::table_name(), where_sql);
    } else {
        sql = format!("SELECT {} FROM {}", T::table_columns(), T::table_name());
    }
    Ok(sql)
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc};

    use serde::de::DeserializeOwned;
    use serde::Deserialize;
    use serde::Serialize;

    use rbatis_core::Error;

    use crate::crud::{CRUD, CRUDEnable, Id, Ids};
    use crate::plugin::logic_delete::RbatisLogicDeletePlugin;
    use crate::plugin::page::{Page, PageRequest};
    use crate::rbatis::Rbatis;
    use crate::wrapper::Wrapper;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct BizActivity {
        pub id: Option<String>,
        pub name: Option<String>,
        pub pc_link: Option<String>,
        pub h5_link: Option<String>,
        pub pc_banner_img: Option<String>,
        pub h5_banner_img: Option<String>,
        pub sort: Option<String>,
        pub status: Option<i32>,
        pub remark: Option<String>,
        pub create_time: Option<String>,
        pub version: Option<i32>,
        pub delete_flag: Option<i32>,
    }

    /// 必须实现 CRUDEntity接口，如果表名 不正确，可以重写 fn table_name() -> String 方法！
    impl CRUDEnable for BizActivity {
        type IdType = String;
    }

    impl Id for BizActivity {
        type IdType = String;

        fn get_id(&self) -> Option<Self::IdType> {
            self.id.clone()
        }
    }


    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct BizActivityNoDel {
        pub id: Option<String>,
        pub name: Option<String>,
    }

    impl CRUDEnable for BizActivityNoDel {
        type IdType = String;
        fn table_name() -> String {
            "biz_activity".to_string()
        }
    }


    #[test]
    pub fn test_ids() {
        let vec = vec![BizActivity {
            id: Some("12312".to_string()),
            name: None,
            pc_link: None,
            h5_link: None,
            pc_banner_img: None,
            h5_banner_img: None,
            sort: None,
            status: Some(1),
            remark: None,
            create_time: Some("2020-02-09 00:00:00".to_string()),
            version: Some(1),
            delete_flag: Some(1),
        }];
        let ids = vec.to_ids();
        println!("{:?}", ids);
    }

    #[test]
    pub fn test_save() {
        async_std::task::block_on(async {
            let activity = BizActivity {
                id: Some("12312".to_string()),
                name: None,
                pc_link: None,
                h5_link: None,
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(1),
                remark: None,
                create_time: Some("2020-02-09 00:00:00".to_string()),
                version: Some(1),
                delete_flag: Some(1),
            };

            fast_log::init_log("requests.log", 1000,log::Level::Info, true);
            let rb = Rbatis::new();
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
            let r = rb.save("", &activity).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_save_batch() {
        async_std::task::block_on(async {
            let activity = BizActivity {
                id: Some("12312".to_string()),
                name: None,
                pc_link: None,
                h5_link: None,
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(1),
                remark: None,
                create_time: Some("2020-02-09 00:00:00".to_string()),
                version: Some(1),
                delete_flag: Some(1),
            };
            let args = vec![activity.clone(), activity];

            fast_log::init_log("requests.log", 1000,log::Level::Info, true);
            let rb = Rbatis::new();
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
            let r = rb.save_batch("", &args).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }


    #[test]
    pub fn test_remove_batch_by_id() {
        async_std::task::block_on(async {
            fast_log::init_log("requests.log", 1000,log::Level::Info, true);
            let mut rb = Rbatis::new();
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
            let r = rb.remove_batch_by_id::<BizActivity>("", &["1".to_string(), "2".to_string()]).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }


    #[test]
    pub fn test_remove_by_id() {
        async_std::task::block_on(async {
            fast_log::init_log("requests.log", 1000,log::Level::Info, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
            let r = rb.remove_by_id::<BizActivity>("", &"1".to_string()).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_update_by_wrapper() {
        async_std::task::block_on(async {
            fast_log::init_log("requests.log", 1000,log::Level::Info, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();

            let activity = BizActivity {
                id: Some("12312".to_string()),
                name: None,
                pc_link: None,
                h5_link: None,
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(1),
                remark: None,
                create_time: Some("2020-02-09 00:00:00".to_string()),
                version: Some(1),
                delete_flag: Some(1),
            };

            let w = Wrapper::new(&rb.driver_type().unwrap()).eq("id", "12312").check().unwrap();
            let r = rb.update_by_wrapper("", &activity, &w, false).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }


    #[test]
    pub fn test_update_by_id() {
        async_std::task::block_on(async {
            fast_log::init_log("requests.log", 1000,log::Level::Info, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();

            let activity = BizActivity {
                id: Some("12312".to_string()),
                name: None,
                pc_link: None,
                h5_link: None,
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(1),
                remark: None,
                create_time: Some("2020-02-09 00:00:00".to_string()),
                version: Some(1),
                delete_flag: Some(1),
            };
            let r = rb.update_by_id("", &activity).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_fetch_by_wrapper() {
        async_std::task::block_on(async {
            fast_log::init_log("requests.log", 1000,log::Level::Info, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();

            let w = Wrapper::new(&rb.driver_type().unwrap()).eq("id", "12312").check().unwrap();
            let r: Result<BizActivity, Error> = rb.fetch_by_wrapper("", &w).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_fetch_no_del() {
        async_std::task::block_on(async {
            fast_log::init_log("requests.log", 1000,log::Level::Info, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();

            let w = Wrapper::new(&rb.driver_type().unwrap()).eq("id", "12312").check().unwrap();
            let r: Result<BizActivityNoDel, Error> = rb.fetch_by_wrapper("", &w).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_fetch_page_by_wrapper() {
        async_std::task::block_on(async {
            fast_log::init_log("requests.log", 1000,log::Level::Info, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();

            let w = Wrapper::new(&rb.driver_type().unwrap()).check().unwrap();
            let r: Page<BizActivity> = rb.fetch_page_by_wrapper("", &w, &PageRequest::new(1, 20)).await.unwrap();
            println!("{}", serde_json::to_string(&r).unwrap());
        });
    }
}