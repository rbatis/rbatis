use rbatis_core::db::DBPool;
use rbatis_core::Error;
use rbson::{bson, Bson};
use rbatis_core::types::{Format, Json, Uuid};
use rbatis_core::types::Timestamp;

#[macro_use]
pub mod bencher;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //Automatic judgment of database type or  postgres://postgres:123456@localhost:5432/postgres
    let pool = DBPool::new("postgres://postgres:123456@localhost:5432/postgres").await?;
    let mut conn = pool.acquire().await?;

    let mut q = conn.make_query("insert into biz_activity (id,name,create_time,js) values ($1,$2,$3,$4)").unwrap();
    let id = Uuid::new();
    println!("{}", id);
    q.bind_value(rbson::to_bson(&id).unwrap());
    q.bind_value(rbson::to_bson(&"xxxx").unwrap());
    q.bind_value(rbson::to_bson(&Timestamp::now()).unwrap());
    q.bind_value(rbson::to_bson(&Json {
        inner: 1
    }).unwrap());
    conn.exec_prepare(q).await.unwrap();

    let mut q = pool.make_query("SELECT * FROM biz_activity").unwrap();

    let data: (Bson, usize) = conn
        .fetch_parperd(q)
        .await.unwrap();
    println!("count: {}", data.0.do_format());
    return Ok(());
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use rbatis_core::convert::StmtConvert;
    use rbatis_core::db::{DriverType, DBPool};
    use rbatis_core::{Error, Json, Timestamp, Uuid};

    #[test]
    pub fn test_type_name() {
        let mut type_name = std::any::type_name::<[i32]>();
        println!("type_name:{}", type_name);
    }

    #[test]
    pub fn test_convert() {
        let mut s = String::new();
        DriverType::Postgres.stmt_convert(1000, &mut s);
        println!("stmt:{}", s);
    }

    #[test]
    pub fn bench_convert() {
        let mut s = String::with_capacity(200000);
        DriverType::Postgres.stmt_convert(0, &mut s);
        println!("stmt:{}", s);
        bench!(100000,{
            DriverType::Postgres.stmt_convert(1,&mut s);
        });
    }

    #[test]
    pub fn bench_do_format() {
        use rbatis_core::Format;
        let mut m = HashMap::new();
        m.insert("1", rbson::to_bson(&Uuid::new()).unwrap());
        m.insert("2", rbson::to_bson(&Timestamp::now()).unwrap());
        m.insert("3", rbson::to_bson(&Json::<serde_json::Value>::from(serde_json::json!(1))).unwrap());

        let mut b = rbson::to_bson(&m).unwrap();
        bench!(100000,{
            //use Time: 372.422236ms ,each:3724 ns/op use QPS: 268501 QPS/s
            b.do_format();
            //use Time: 233.170734ms ,each:2331 ns/op  use QPS: 428807 QPS/s
            //b.to_string();
        });
    }

    #[tokio::test]
    async fn test_tx() -> Result<(), Error> {
        //Automatic judgment of database type or  postgres://postgres:123456@localhost:5432/postgres
        let pool = DBPool::new("mysql://root:123456@localhost:3306/test").await?;
        let mut tx = pool.begin().await?;
        let data = tx
            .exec_sql("UPDATE `biz_activity` SET `name` = 'test2' WHERE (`id` = '222');")
            .await.unwrap();
        println!("count: {:?}", data);
        tx.commit().await.unwrap();
        return Ok(());
    }

    #[tokio::test]
    async fn test_conn_tx() -> Result<(), Error> {
        //Automatic judgment of database type or  postgres://postgres:123456@localhost:5432/postgres
        let pool = DBPool::new("mysql://root:123456@localhost:3306/test").await?;
        let mut conn = pool.acquire().await?;
        let mut tx = conn.begin().await?;
        let data = tx
            .exec_sql("UPDATE `biz_activity` SET `name` = 'test2' WHERE (`id` = '222');")
            .await.unwrap();
        println!("count: {:?}", data);
        tx.commit().await.unwrap();
        return Ok(());
    }

    #[tokio::test]
    async fn test_conn_tx_rollback() -> Result<(), Error> {
        //Automatic judgment of database type or  postgres://postgres:123456@localhost:5432/postgres
        let pool = DBPool::new("mysql://root:123456@localhost:3306/test").await?;
        let mut conn = pool.acquire().await?;
        let mut tx = conn.begin().await?;
        let data = tx
            .exec_sql("UPDATE `biz_activity` SET `name` = 'test2' WHERE (`id` = '222');")
            .await.unwrap();
        println!("count: {:?}", data);
        tx.rollback().await.unwrap();
        return Ok(());
    }
}
