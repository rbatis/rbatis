use rbatis::executor::{RBatisRef, Executor};
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

#[test]
fn test_transaction_commit() {
    let rb = make_test_rbatis();
    
    let result = block_on(async move {
        // 创建测试表
        rb.exec("CREATE TABLE IF NOT EXISTS tx_test (id INTEGER PRIMARY KEY, name TEXT)", vec![]).await?;
        rb.exec("DELETE FROM tx_test", vec![]).await?;
        
        // 开始事务
        let tx = rb.acquire_begin().await?;
        
        // 在事务中执行插入
        tx.exec("INSERT INTO tx_test (id, name) VALUES (?, ?)", 
            vec![Value::I32(1), Value::String("tx_test".to_string())]).await?;
        
        // 提交事务
        tx.commit().await?;
        
        // 事务提交后验证数据是否存在
        let result = rb.query("SELECT * FROM tx_test WHERE id = ?", vec![Value::I32(1)]).await?;
        Ok::<_, rbatis::Error>(result)
    });
    
    assert!(result.is_ok());
    let result = result.unwrap();
    let arr = result.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0].as_map().unwrap()["name"].as_str().unwrap(), "tx_test");
}

#[test]
fn test_transaction_rollback() {
    let rb = make_test_rbatis();
    
    let result = block_on(async move {
        // 创建测试表
        rb.exec("CREATE TABLE IF NOT EXISTS tx_test (id INTEGER PRIMARY KEY, name TEXT)", vec![]).await?;
        rb.exec("DELETE FROM tx_test", vec![]).await?;
        
        // 开始事务
        let tx = rb.acquire_begin().await?;
        
        // 在事务中执行插入
        tx.exec("INSERT INTO tx_test (id, name) VALUES (?, ?)", 
            vec![Value::I32(2), Value::String("should_rollback".to_string())]).await?;
        
        // 回滚事务
        tx.rollback().await?;
        
        // 事务回滚后验证数据是否不存在
        let result = rb.query("SELECT * FROM tx_test WHERE id = ?", vec![Value::I32(2)]).await?;
        Ok::<_, rbatis::Error>(result)
    });
    
    assert!(result.is_ok());
    let result = result.unwrap();
    let arr = result.as_array().unwrap();
    // 应该没有数据（回滚了）
    assert_eq!(arr.len(), 0);
}

#[test]
fn test_transaction_query_decode() {
    let rb = make_test_rbatis();
    
    #[derive(serde::Deserialize, Debug)]
    struct TestRow {
        id: i32,
        name: String,
    }
    
    let result = block_on(async move {
        // 创建测试表
        rb.exec("CREATE TABLE IF NOT EXISTS tx_test (id INTEGER PRIMARY KEY, name TEXT)", vec![]).await?;
        rb.exec("DELETE FROM tx_test", vec![]).await?;
        rb.exec("INSERT INTO tx_test (id, name) VALUES (?, ?)", 
            vec![Value::I32(3), Value::String("decode_test".to_string())]).await?;
        
        // 开始事务
        let tx = rb.acquire_begin().await?;
        
        // 使用事务执行查询并解码
        let row: TestRow = tx.query_decode("SELECT * FROM tx_test WHERE id = ?", vec![Value::I32(3)]).await?;
        
        // 提交事务
        tx.commit().await?;
        
        Ok::<_, rbatis::Error>(row)
    });
    
    assert!(result.is_ok());
    let row = result.unwrap();
    assert_eq!(row.id, 3);
    assert_eq!(row.name, "decode_test");
}

#[test]
fn test_nested_transaction() {
    let rb = make_test_rbatis();
    
    // SQLite不支持真正的嵌套事务，但我们可以测试事务提交的基本功能
    let result = block_on(async move {
        // 创建测试表
        rb.exec("CREATE TABLE IF NOT EXISTS nested_tx_test (id INTEGER PRIMARY KEY, name TEXT)", vec![]).await?;
        rb.exec("DELETE FROM nested_tx_test", vec![]).await?;
        
        // 开始事务
        let tx = rb.acquire_begin().await?;
        
        // 在事务中插入数据
        tx.exec("INSERT INTO nested_tx_test (id, name) VALUES (?, ?)", 
            vec![Value::I32(1), Value::String("tx1".to_string())]).await?;
        
        // 另一个数据
        tx.exec("INSERT INTO nested_tx_test (id, name) VALUES (?, ?)", 
            vec![Value::I32(2), Value::String("tx2".to_string())]).await?;
        
        // 提交事务
        tx.commit().await?;
        
        // 验证插入的数据
        let result = rb.query("SELECT * FROM nested_tx_test ORDER BY id", vec![]).await?;
        Ok::<_, rbatis::Error>(result)
    });
    
    assert!(result.is_ok());
    let result = result.unwrap();
    let arr = result.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0].as_map().unwrap()["name"].as_str().unwrap(), "tx1");
    assert_eq!(arr[1].as_map().unwrap()["name"].as_str().unwrap(), "tx2");
}

#[test]
fn test_transaction_with_defer() {
    let rb = make_test_rbatis();
    
    let result = block_on(async move {
        // 创建测试表
        rb.exec("CREATE TABLE IF NOT EXISTS defer_tx_test (id INTEGER PRIMARY KEY, name TEXT)", vec![]).await?;
        rb.exec("DELETE FROM defer_tx_test", vec![]).await?;
        
        // 使用defer模式开始事务
        let tx = rb.acquire_begin().await?;
        
        // 注册defer回调，这将在tx被丢弃时自动提交事务
        let guard = tx.defer_async(|tx| async move {
            // 这里可以执行额外的清理工作
            let _ = tx.commit().await;
        });
        
        // 使用guard执行操作
        guard.tx.exec("INSERT INTO defer_tx_test (id, name) VALUES (?, ?)", 
            vec![Value::I32(1), Value::String("defer_test".to_string())]).await?;
        
        // 手动提交
        guard.commit().await?;
        
        // 验证数据
        let result = rb.query("SELECT * FROM defer_tx_test WHERE id = ?", vec![Value::I32(1)]).await?;
        Ok::<_, rbatis::Error>(result)
    });
    
    assert!(result.is_ok());
    let result = result.unwrap();
    let arr = result.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0].as_map().unwrap()["name"].as_str().unwrap(), "defer_test");
}

#[test]
fn test_executor_interface() {
    let rb = make_test_rbatis();
    
    let result = block_on(async move {
        // 测试Executor trait的方法
        
        // 创建测试表
        rb.exec("CREATE TABLE IF NOT EXISTS exec_test (id INTEGER PRIMARY KEY, name TEXT)", vec![]).await?;
        rb.exec("DELETE FROM exec_test", vec![]).await?;
        
        // 使用Executor trait的exec方法
        let exec_result = Executor::exec(&rb, 
            "INSERT INTO exec_test (id, name) VALUES (?, ?)", 
            vec![Value::I32(1), Value::String("exec_test".to_string())]).await?;
        
        // 验证执行结果
        assert_eq!(exec_result.rows_affected, 1);
        
        // 使用Executor trait的query方法
        let query_result = Executor::query(&rb, 
            "SELECT * FROM exec_test WHERE id = ?", 
            vec![Value::I32(1)]).await?;
        
        assert!(query_result.is_array());
        let arr = query_result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        
        Ok::<_, rbatis::Error>(())
    });
    
    assert!(result.is_ok());
}

fn make_test_rbatis() -> RBatis {
    let rb = RBatis::new();
    let rb_clone = rb.clone();
    block_on(async move {
        rb_clone.link(SqliteDriver {}, "sqlite://:memory:").await.unwrap();
    });
    rb
} 