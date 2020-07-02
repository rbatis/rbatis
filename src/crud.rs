use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use async_trait::async_trait;
use rbatis_core::db::DriverType;
use rbatis_core::Error;
use rbatis_core::Result;

use crate::convert::stmt_convert::StmtConvert;
use crate::rbatis::Rbatis;

/// DB Table model trait
pub trait CRUDEntity: Send + Sync + DeserializeOwned + Serialize {
    /// your table id type,for example:
    /// IdType = String
    /// IdType = i32
    ///
    type IdType: Send + Sync + DeserializeOwned + Serialize;
    /// your table name
    fn table_name() -> String{
        let type_name = std::any::type_name::<Self>();
        return type_name.to_string();
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
        return Ok(m);
    }

    fn fields(&self, map: &serde_json::Map<String, Value>) -> Result<String> {
        let mut sql = String::new();
        for (k, v) in map {
            sql = sql + k.as_str() + ",";
        }
        sql = sql.trim_end_matches(",").to_string();
        return Ok(sql);
    }

    fn values(&self, db_type: &DriverType, map: &serde_json::Map<String, serde_json::Value>) -> Result<(String, Vec<serde_json::Value>)> {
        let mut sql = String::new();
        let mut index = 0;
        let mut arr = vec![];
        for (k, v) in map {
            sql = sql + db_type.stmt_convert(index).as_str() + ",";
            arr.push(v.to_owned());
            index += 1;
        }
        sql = sql.trim_end_matches(",").to_string();
        return Ok((sql, arr));
    }
}


#[async_trait]
pub trait CRUD {
    async fn save<T>(&self, entity: &T) -> Result<u64> where T: CRUDEntity;
    async fn save_batch<T>(&self, entity: &Vec<T>) -> Result<u64> where T: CRUDEntity;
    async fn remove_by_id<T>(&self, id: &T::IdType) -> Result<u64> where T: CRUDEntity;
    async fn remove_batch_by_id<T>(&self, ids: &Vec<T::IdType>) -> Result<u64> where T: CRUDEntity;
    async fn update_by_id<T>(&self, id: &T::IdType) -> Result<u64> where T: CRUDEntity;
    async fn update_batch_by_id<T>(&self, ids: &Vec<T::IdType>) -> Result<u64> where T: CRUDEntity;
    async fn get_by_id<T>(&self, id: &T::IdType) -> Result<T> where T: CRUDEntity;
    ///all record
    async fn list<T>(&self) -> Result<Vec<T>> where T: CRUDEntity;
    async fn list_by_ids<T>(&self, ids: &Vec<T::IdType>) -> Result<Vec<T>> where T: CRUDEntity;
}

#[async_trait]
impl CRUD for Rbatis<'_> {
    async fn save<T>(&self, entity: &T) -> Result<u64>
        where T: CRUDEntity {
        let map = entity.to_value_map()?;
        let (values, args) = entity.values(&self.driver_type()?, &map)?;
        let sql = format!("INSERT INTO {} ({}) VALUES ({})", T::table_name(), entity.fields(&map)?, values);
        return self.exec_prepare("", sql.as_str(), &args).await;
    }

    async fn save_batch<T>(&self, entity: &Vec<T>) -> Result<u64> where T: CRUDEntity {
        let mut r = 0;
        for x in entity {
            let v = self.save(x).await?;
            r = r + v;
        }
        return Ok(r);
    }

    async fn remove_by_id<T>(&self, id: &T::IdType) -> Result<u64> where T: CRUDEntity {
        unimplemented!()
    }

    async fn remove_batch_by_id<T>(&self, ids: &Vec<T::IdType>) -> Result<u64> where T: CRUDEntity {
        unimplemented!()
    }

    async fn update_by_id<T>(&self, id: &T::IdType) -> Result<u64> where T: CRUDEntity {
        unimplemented!()
    }

    async fn update_batch_by_id<T>(&self, ids: &Vec<T::IdType>) -> Result<u64> where T: CRUDEntity {
        unimplemented!()
    }

    async fn get_by_id<T>(&self, id: &T::IdType) -> Result<T> where T: CRUDEntity {
        unimplemented!()
    }

    async fn list<T>(&self) -> Result<Vec<T>> where T: CRUDEntity {
        unimplemented!()
    }

    async fn list_by_ids<T>(&self, ids: &Vec<T::IdType>) -> Result<Vec<T>> where T: CRUDEntity {
        unimplemented!()
    }
}

mod test {
    use chrono::{DateTime, Utc};
    use serde::de::DeserializeOwned;
    use serde::Deserialize;
    use serde::Serialize;

    use crate::crud::{CRUD, CRUDEntity};
    use crate::rbatis::Rbatis;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Activity {
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

    impl CRUDEntity for Activity {
        type IdType = String;
    }

    #[test]
    pub fn test_save() {
        async_std::task::block_on(async {
            let activity = Activity {
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

            let rb = Rbatis::new();
            rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
            let r = rb.save(&activity).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        })
    }
}