use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::export::fmt::Display;
use serde::Serialize;
use serde_json::Value;

use rbatis_core::convert::StmtConvert;
use rbatis_core::db::DriverType;
use rbatis_core::Error;
use rbatis_core::Result;

use crate::plugin::page::{IPageRequest, Page};
use crate::rbatis::Rbatis;
use crate::utils::string_util::to_snake_name;
use crate::wrapper::Wrapper;

/// DB Table model trait
pub trait CRUDEnable: Send + Sync + Serialize + DeserializeOwned {
    /// your table id type,for example:
    /// IdType = String
    /// IdType = i32
    ///
    type IdType: Send + Sync + DeserializeOwned + Serialize + Display;

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
    #[inline]
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
    #[inline]
    fn table_fields() -> String {
        let a: Self = serde_json::from_str("{}").unwrap();
        let v = serde_json::to_value(&a).unwrap();
        if !v.is_object() {
            return " * ".to_string();
        }
        let m = v.as_object().unwrap();
        let mut fields = String::new();
        for (k, _) in m {
            fields.push_str(k);
            fields.push_str(",");
        }
        fields.pop();
        return format!(" {} ", fields);
    }

    fn to_value(&self) -> Result<serde_json::Value> {
        let json = serde_json::to_value(self).unwrap_or(serde_json::Value::Null);
        if json.eq(&serde_json::Value::Null) {
            return Err(Error::from("[rbaits] fields() fail!"));
        }
        return Ok(json);
    }

    fn to_value_map(&self) -> Result<serde_json::Map<String, Value>> {
        let json = serde_json::to_value(self).unwrap_or(serde_json::Value::Null);
        if json.eq(&serde_json::Value::Null) {
            return Err(Error::from("[rbaits] to_value_map() fail!"));
        }
        if !json.is_object() {
            return Err(Error::from("[rbaits] to_value_map() fail,data is not an object!"));
        }
        let m = json.as_object().unwrap().to_owned();
        let mut new_m = m.clone();
        for (k, v) in &m {
            if (k.contains("time") || k.contains("date")) && v.is_string() {
                let mut new_v = v.as_str().unwrap().to_string();
                new_v = new_v.replace("T", " ");
                let new_vs: Vec<&str> = new_v.split("+").collect();
                if new_vs.len() > 1 {
                    new_v = new_vs.get(0).unwrap().to_string();
                }
                new_m.insert(k.to_string(), serde_json::Value::String(new_v));
            }
        }
        return Ok(new_m);
    }

    fn fields(&self, map: &serde_json::Map<String, Value>) -> Result<String> {
        let mut sql = String::new();
        for (k, v) in map {
            sql = sql + k.as_str() + ",";
        }
        sql = sql.trim_end_matches(",").to_string();
        return Ok(sql);
    }

    fn values(&self, index: &mut usize, db_type: &DriverType, map: &serde_json::Map<String, serde_json::Value>) -> Result<(String, Vec<serde_json::Value>)> {
        let mut sql = String::new();
        let mut arr = vec![];
        for (k, v) in map {
            sql = sql + db_type.stmt_convert(*index).as_str() + ",";
            arr.push(v.to_owned());
            *index += 1;
        }
        sql = sql.trim_end_matches(",").to_string();
        return Ok((sql, arr));
    }
}


#[async_trait]
pub trait CRUD {
    async fn save<T>(&self, entity: &T) -> Result<u64> where T: CRUDEnable;
    async fn save_batch<T>(&self, entity: &[T]) -> Result<u64> where T: CRUDEnable;

    async fn remove_by_wrapper<T>(&self, w: &Wrapper) -> Result<u64> where T: CRUDEnable;
    async fn remove_by_id<T>(&self, id: &T::IdType) -> Result<u64> where T: CRUDEnable;
    async fn remove_batch_by_id<T>(&self, ids: &[T::IdType]) -> Result<u64> where T: CRUDEnable;

    async fn update_by_wrapper<T>(&self, arg: &T, w: &Wrapper) -> Result<u64> where T: CRUDEnable;
    async fn update_by_id<T>(&self, arg: &T) -> Result<u64> where T: CRUDEnable;
    async fn update_batch_by_id<T>(&self, ids: &[T]) -> Result<u64> where T: CRUDEnable;


    async fn fetch_by_wrapper<T>(&self, w: &Wrapper) -> Result<T> where T: CRUDEnable;
    async fn fetch_by_id<T>(&self, id: &T::IdType) -> Result<T> where T: CRUDEnable;

    ///fetch all record
    async fn list<T>(&self) -> Result<Vec<T>> where T: CRUDEnable;
    async fn list_by_wrapper<T>(&self, w: &Wrapper) -> Result<Vec<T>> where T: CRUDEnable;
    async fn list_by_ids<T>(&self, ids: &[T::IdType]) -> Result<Vec<T>> where T: CRUDEnable;

    async fn fetch_page_by_wrapper<T>(&self, w: &Wrapper, page: &dyn IPageRequest) -> Result<Page<T>> where T: CRUDEnable;
}

#[async_trait]
impl CRUD for Rbatis<'_> {
    /// save one entity to database
    async fn save<T>(&self, entity: &T) -> Result<u64>
        where T: CRUDEnable {
        let map = entity.to_value_map()?;
        let mut index = 0;
        let (values, args) = entity.values(&mut index, &self.driver_type()?, &map)?;
        let sql = format!("INSERT INTO {} ({}) VALUES ({})", T::table_name(), entity.fields(&map)?, values);
        return self.exec_prepare("", sql.as_str(), &args).await;
    }

    /// save batch makes many value into  only one sql. make sure your data not  to long!
    ///
    /// for Example:
    /// rb.save_batch(&vec![activity]);
    /// [rbatis] Exec ==>   INSERT INTO biz_activity (id,name,version) VALUES ( ? , ? , ?),( ? , ? , ?)
    ///
    ///
    async fn save_batch<T>(&self, args: &[T]) -> Result<u64> where T: CRUDEnable {
        if args.is_empty() {
            return Ok(0);
        }
        let mut value_arr = String::new();
        let mut arg_arr = vec![];
        let mut fields = "".to_string();
        let mut field_index = 0;
        for x in args {
            let map = x.to_value_map()?;
            if fields.is_empty() {
                fields = x.fields(&map)?;
            }
            let (values, args) = x.values(&mut field_index, &self.driver_type()?, &map)?;
            value_arr = value_arr + format!("({}),", values).as_str();
            for x in args {
                arg_arr.push(x);
            }
        }
        value_arr.pop();//pop ','
        let sql = format!("INSERT INTO {} ({}) VALUES {}", T::table_name(), fields, value_arr);
        return self.exec_prepare("", sql.as_str(), &arg_arr).await;
    }

    async fn remove_by_wrapper<T>(&self, arg: &Wrapper) -> Result<u64> where T: CRUDEnable {
        let mut where_sql = arg.sql.as_str();
        let mut sql = String::new();
        if self.logic_plugin.is_some() {
            sql = self.logic_plugin.as_ref().unwrap().create_sql(&self.driver_type()?, T::table_name().as_str(), make_where_sql(where_sql).as_str())?;
        } else {
            sql = format!("DELETE FROM {} {}", T::table_name(), make_where_sql(where_sql));
        }
        return self.exec_prepare("", sql.as_str(), &arg.args).await;
    }

    async fn remove_by_id<T>(&self, id: &T::IdType) -> Result<u64> where T: CRUDEnable {
        let mut sql = String::new();
        if self.logic_plugin.is_some() {
            sql = self.logic_plugin.as_ref().unwrap().create_sql(&self.driver_type()?, T::table_name().as_str(), format!(" WHERE id = {}", id).as_str())?;
        } else {
            sql = format!("DELETE FROM {} WHERE id = {}", T::table_name(), id);
        }
        return self.exec_prepare("", sql.as_str(), &vec![]).await;
    }

    ///remove batch id
    /// for Example :
    /// rb.remove_batch_by_id::<BizActivity>(&["1".to_string(),"2".to_string()]).await;
    /// [rbatis] Exec ==> DELETE FROM biz_activity WHERE id IN ( ? , ? )
    ///
    async fn remove_batch_by_id<T>(&self, ids: &[T::IdType]) -> Result<u64> where T: CRUDEnable {
        if ids.is_empty() {
            return Ok(0);
        }
        let mut w = Wrapper::new(&self.driver_type()?).and().in_array("id", &ids).check()?;
        return self.remove_by_wrapper::<T>(&w).await;
    }

    async fn update_by_wrapper<T>(&self, arg: &T, w: &Wrapper) -> Result<u64> where T: CRUDEnable {
        let mut index = 0;
        let mut args = vec![];

        let map = arg.to_value_map()?;
        let driver_type = &self.driver_type()?;
        let mut sets = String::new();
        for (k, v) in map {
            //filter null
            if v.is_null() {
                continue;
            }
            //filter id
            if k.eq("id") {
                continue;
            }
            sets.push_str(format!(" {} = {},", k, driver_type.stmt_convert(index)).as_str());
            args.push(v);
        }
        sets.pop();
        let mut wrapper = Wrapper::new(&self.driver_type()?);
        wrapper.sql = format!("UPDATE {} SET {}", T::table_name(), sets);
        wrapper.args = args;
        if !w.sql.is_empty() {
            wrapper.sql.push_str(" WHERE ");
            wrapper = wrapper.join_first_wrapper(w).check()?;
        }
        return self.exec_prepare("", wrapper.sql.as_str(), &wrapper.args).await;
    }

    async fn update_by_id<T>(&self, arg: &T) -> Result<u64> where T: CRUDEnable {
        let args = arg.to_value_map()?;
        let id_field = args.get("id");
        if id_field.is_none() {
            return Err(Error::from("[rbaits] arg not have \"id\" field! "));
        }
        self.update_by_wrapper(arg, Wrapper::new(&self.driver_type()?).eq("id", id_field.unwrap())).await
    }

    async fn update_batch_by_id<T>(&self, args: &[T]) -> Result<u64> where T: CRUDEnable {
        let mut updates = 0;
        for x in args {
            updates += self.update_by_id(x).await?
        }
        Ok(updates)
    }

    async fn fetch_by_wrapper<T>(&self, w: &Wrapper) -> Result<T> where T: CRUDEnable {
        let fields = T::table_fields();
        let mut where_sql = String::new();
        let mut sql = String::new();
        if self.logic_plugin.is_some() {
            sql = format!("SELECT {} FROM {} WHERE {} = {} AND {}", fields, T::table_name(), self.logic_plugin.as_ref().unwrap().column(), self.logic_plugin.as_ref().unwrap().un_deleted(), w.sql.as_str());
        } else {
            sql = format!("SELECT {} FROM {} WHERE {}", fields, T::table_name(), w.sql.as_str());
        }
        return self.fetch_prepare("", sql.as_str(), &w.args).await;
    }

    async fn fetch_by_id<T>(&self, id: &T::IdType) -> Result<T> where T: CRUDEnable {
        let w = Wrapper::new(&self.driver_type().unwrap()).eq("id", id).check()?;
        return self.fetch_by_wrapper(&w).await;
    }

    async fn list_by_wrapper<T>(&self, w: &Wrapper) -> Result<Vec<T>> where T: CRUDEnable {
        unimplemented!()
    }

    async fn list<T>(&self) -> Result<Vec<T>> where T: CRUDEnable {
        unimplemented!()
    }

    async fn list_by_ids<T>(&self, ids: &[T::IdType]) -> Result<Vec<T>> where T: CRUDEnable {
        unimplemented!()
    }

    async fn fetch_page_by_wrapper<T>(&self, w: &Wrapper, page: &dyn IPageRequest) -> Result<Page<T>> where T: CRUDEnable {
        unimplemented!()
    }
}

fn make_where_sql(arg: &str) -> String {
    let mut where_sql = arg.to_string();
    where_sql = where_sql.trim_start().trim_start_matches("AND ").trim_start_matches("OR ").to_string();
    format!(" WHERE {} ", where_sql)
}


mod test {
    use chrono::{DateTime, Utc};
    use fast_log::log::RuntimeType;
    use serde::de::DeserializeOwned;
    use serde::Deserialize;
    use serde::Serialize;

    use rbatis_core::Error;

    use crate::crud::{CRUD, CRUDEnable};
    use crate::plugin::logic_delete::RbatisLogicDeletePlugin;
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

            fast_log::log::init_log("requests.log", &RuntimeType::Std);
            let rb = Rbatis::new();
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
            let r = rb.save(&activity).await;
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

            fast_log::log::init_log("requests.log", &RuntimeType::Std);
            let rb = Rbatis::new();
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
            let r = rb.save_batch(&args).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }


    #[test]
    pub fn test_remove_batch_by_id() {
        async_std::task::block_on(async {
            fast_log::log::init_log("requests.log", &RuntimeType::Std);
            let mut rb = Rbatis::new();
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
            let r = rb.remove_batch_by_id::<BizActivity>(&["1".to_string(), "2".to_string()]).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }


    #[test]
    pub fn test_remove_by_id() {
        async_std::task::block_on(async {
            fast_log::log::init_log("requests.log", &RuntimeType::Std);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
            let r = rb.remove_by_id::<BizActivity>(&"1".to_string()).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_update_by_wrapper() {
        async_std::task::block_on(async {
            fast_log::log::init_log("requests.log", &RuntimeType::Std);
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
            let r = rb.update_by_wrapper(&activity, &w).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }


    #[test]
    pub fn test_update_by_id() {
        async_std::task::block_on(async {
            fast_log::log::init_log("requests.log", &RuntimeType::Std);
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
            let r = rb.update_by_id(&activity).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_fetch_by_wrapper() {
        async_std::task::block_on(async {
            fast_log::log::init_log("requests.log", &RuntimeType::Std);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();

            let w = Wrapper::new(&rb.driver_type().unwrap()).eq("id", "12312").check().unwrap();
            let r: Result<BizActivity, Error> = rb.fetch_by_wrapper(&w).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }
}