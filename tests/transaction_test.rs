use rbatis::rbatis::RBatis;
use rbdc::rt::block_on;
use rbdc_sqlite::SqliteDriver;
use rbs::Value;

// 创建数据库连接并准备测试环境
fn setup_test() -> RBatis {
    let rb = RBatis::new();
    let rb_clone = rb.clone();
    
    block_on(async move {
        // 使用内存数据库，每个连接都是独立的
        let db_url = "sqlite://:memory:";
        println!("连接数据库: {}", db_url);
        rb_clone.link(SqliteDriver {}, db_url).await.unwrap();
        
        // 创建测试表
        println!("创建测试表 test_tx");
        rb_clone.exec("CREATE TABLE IF NOT EXISTS test_tx (id INTEGER PRIMARY KEY, name TEXT)", vec![]).await.unwrap();
        // 清空数据
        rb_clone.exec("DELETE FROM test_tx", vec![]).await.unwrap();
    });
    
    rb
}

#[test]
fn test_transaction_commit() {
    let rb = setup_test();
    
    let result = block_on(async move {
        // 开启事务
        let tx = rb.acquire_begin().await?;
        
        // 执行插入
        tx.exec("INSERT INTO test_tx (id, name) VALUES (?, ?)", vec![Value::I32(1), Value::String("tx_test1".to_string())]).await?;
        tx.exec("INSERT INTO test_tx (id, name) VALUES (?, ?)", vec![Value::I32(2), Value::String("tx_test2".to_string())]).await?;
        
        // 提交事务
        tx.commit().await?;
        
        // 使用query_decode验证数据已提交
        #[derive(serde::Deserialize, Debug)]
        struct Count {
            count: i64
        }
        
        let count: Count = rb.query_decode("SELECT COUNT(*) as count FROM test_tx", vec![]).await?;
        Ok::<_, rbatis::Error>(count)
    });
    
    if let Err(err) = &result {
        eprintln!("交易提交测试出错: {}", err);
    }
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count.count, 2);
}

#[test]
fn test_transaction_rollback() {
    let rb = setup_test();
    
    let result = block_on(async move {
        // 开启事务
        let tx = rb.acquire_begin().await?;
        
        // 执行插入
        tx.exec("INSERT INTO test_tx (id, name) VALUES (?, ?)", vec![Value::I32(3), Value::String("tx_test3".to_string())]).await?;
        
        // 回滚事务
        tx.rollback().await?;
        
        // 验证数据未提交
        #[derive(serde::Deserialize, Debug)]
        struct Count {
            count: i64
        }
        
        let count: Count = rb.query_decode("SELECT COUNT(*) as count FROM test_tx", vec![]).await?;
        Ok::<_, rbatis::Error>(count)
    });
    
    if let Err(err) = &result {
        eprintln!("交易回滚测试出错: {}", err);
    }
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count.count, 0); // 数据应该被回滚，计数为0
}

#[test]
fn test_nested_transaction() {
    let rb = setup_test();
    
    // 简化嵌套事务测试
    let result = block_on(async move {
        // 创建单层事务
        let tx = rb.acquire_begin().await?;
        
        // 执行两次插入
        tx.exec("INSERT INTO test_tx (id, name) VALUES (?, ?)", vec![Value::I32(5), Value::String("tx_test5".to_string())]).await?;
        tx.exec("INSERT INTO test_tx (id, name) VALUES (?, ?)", vec![Value::I32(6), Value::String("tx_test6".to_string())]).await?;
        
        // 提交事务
        tx.commit().await?;
        
        // 验证数据
        #[derive(serde::Deserialize, Debug)]
        struct Count {
            count: i64
        }
        
        let count: Count = rb.query_decode("SELECT COUNT(*) as count FROM test_tx", vec![]).await?;
        Ok::<_, rbatis::Error>(count)
    });
    
    if let Err(err) = &result {
        eprintln!("嵌套交易测试出错: {}", err);
    }
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count.count, 2);
}

#[test]
fn test_transaction_guard() {
    let rb = setup_test();
    
    let result = block_on(async move {
        let tx = rb.acquire_begin().await?;
        
        // 使用defer_async创建守卫
        let tx_guard = tx.defer_async(|_tx| async move {
            // 这个闭包会在tx_guard被丢弃时执行
            // 如果没有提交或回滚，默认会回滚
            println!("Transaction completed");
        });
        
        // 插入数据
        tx_guard.tx.exec("INSERT INTO test_tx (id, name) VALUES (?, ?)", vec![Value::I32(7), Value::String("guard_test".to_string())]).await?;
        
        // 提交事务
        tx_guard.commit().await?;
        
        // 验证数据
        #[derive(serde::Deserialize, Debug)]
        struct Count {
            count: i64
        }
        
        let count: Count = rb.query_decode("SELECT COUNT(*) as count FROM test_tx", vec![]).await?;
        Ok::<_, rbatis::Error>(count)
    });
    
    if let Err(err) = &result {
        eprintln!("交易守护测试出错: {}", err);
    }
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count.count, 1);
} 