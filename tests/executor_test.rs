use rbatis::executor::{RBatisRef};
use rbatis::rbatis::RBatis;
use rbdc::rt::block_on;
use rbdc_sqlite::SqliteDriver;
use rbs::Value;

#[test]
fn test_exec_query() {
    let rb = make_test_rbatis();
    
    // 创建测试表
    let result = block_on(async move {
        rb.exec("CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY, name TEXT)", vec![]).await?;
        
        // 插入一些测试数据
        rb.exec("INSERT INTO test_table (id, name) VALUES (?, ?)", vec![Value::I32(1), Value::String("test1".to_string())]).await?;
        rb.exec("INSERT INTO test_table (id, name) VALUES (?, ?)", vec![Value::I32(2), Value::String("test2".to_string())]).await?;
        
        // 查询数据
        let result = rb.query("SELECT * FROM test_table WHERE id = ?", vec![Value::I32(1)]).await?;
        Ok::<_, rbatis::Error>(result)
    });
    
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_array());
    let arr = result.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    
    let row = &arr[0];
    // 使用as_map()方法先将Value转换为map
    let row_map = row.as_map().unwrap();
    assert_eq!(row_map["id"].as_i64().unwrap(), 1);
    assert_eq!(row_map["name"].as_str().unwrap(), "test1");
}

#[test]
fn test_query_decode() {
    let rb = make_test_rbatis();
    
    #[derive(serde::Deserialize, Debug)]
    struct TestRow {
        id: i32,
        name: String,
    }
    
    let result = block_on(async move {
        // 确保表存在并有数据
        rb.exec("CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY, name TEXT)", vec![]).await?;
        rb.exec("DELETE FROM test_table", vec![]).await?;
        rb.exec("INSERT INTO test_table (id, name) VALUES (?, ?)", vec![Value::I32(3), Value::String("test3".to_string())]).await?;
        
        // 使用query_decode方法
        let result: TestRow = rb.query_decode("SELECT * FROM test_table WHERE id = ?", vec![Value::I32(3)]).await?;
        Ok::<_, rbatis::Error>(result)
    });
    
    assert!(result.is_ok());
    let row = result.unwrap();
    assert_eq!(row.id, 3);
    assert_eq!(row.name, "test3");
}

#[test]
fn test_rbatis_ref() {
    let rb = make_test_rbatis();
    assert_eq!(rb.rb_ref().driver_type().unwrap(), "sqlite");
}

fn make_test_rbatis() -> RBatis {
    let rb = RBatis::new();
    let rb_clone = rb.clone();
    block_on(async move {
        rb_clone.link(SqliteDriver {}, "sqlite://:memory:").await.unwrap();
    });
    rb
} 