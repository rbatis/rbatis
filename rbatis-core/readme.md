## rbatis-core driver

rbatis 核心驱动程序

#### support bigdecimal,json decode,async_std,tokio

#### 支持serde_json解码,支持BigDecimal，支持序列化，反序列化，支持async_std和tokio,支持自动判断数据库类型(支持mysql，pg，sqlite，tidb等等相关驱动)

```rust

fn main()  -> Result<u64, rbatis_core::Error>{

 async_std::task::block_on(
        async move {
           //Automatic judgment of database type
           let pool = DBPool::new("mysql://root:123456@localhost:3306/test").await?;
           let mut conn = pool.acquire().await?;
           let count:u64 = conn.execute("SELECT count(1) FROM biz_activity;").await?;
           println("count: {}",count);
           return count;
        }
    );

}


```