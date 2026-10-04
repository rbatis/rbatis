use rbatis::rbatis::RBatis;
use rbdc::rt::block_on;
use rbdc_sqlite::SqliteDriver;
use rbs::Value;

// Create the database connection and prepare the test environment.
fn setup_test() -> RBatis {
    let rb = RBatis::new();
    let rb_clone = rb.clone();
    block_on(async move {
        let db_url = "sqlite://:memory:";
        println!("link database: {}", db_url);
        rb_clone.link(SqliteDriver {}, db_url).await.unwrap();

        println!("create test table test_tx");
        rb_clone
            .exec(
                "CREATE TABLE IF NOT EXISTS test_tx (id INTEGER PRIMARY KEY, name TEXT)",
                vec![],
            )
            .await
            .unwrap();
        rb_clone.exec("DELETE FROM test_tx", vec![]).await.unwrap();
    });

    rb
}

#[test]
fn test_transaction_commit() {
    let rb = setup_test();

    let result = block_on(async move {
        let tx = rb.acquire_begin().await?;

        tx.exec(
            "INSERT INTO test_tx (id, name) VALUES (?, ?)",
            vec![Value::I32(1), Value::String("tx_test1".to_string())],
        )
        .await?;
        tx.exec(
            "INSERT INTO test_tx (id, name) VALUES (?, ?)",
            vec![Value::I32(2), Value::String("tx_test2".to_string())],
        )
        .await?;

        tx.commit().await?;

        let count: i64 = rb
            .exec_decode("SELECT COUNT(*) as count FROM test_tx", vec![])
            .await?;
        Ok::<_, rbatis::Error>(count)
    });

    if let Err(err) = &result {
        eprintln!("transaction commit test error: {}", err);
    }
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, 2);
}

#[test]
fn test_transaction_rollback() {
    let rb = setup_test();

    let result = block_on(async move {
        let tx = rb.acquire_begin().await?;

        tx.exec(
            "INSERT INTO test_tx (id, name) VALUES (?, ?)",
            vec![Value::I32(3), Value::String("tx_test3".to_string())],
        )
        .await?;

        tx.rollback().await?;

        let count: i64 = rb
            .exec_decode("SELECT COUNT(*) as count FROM test_tx", vec![])
            .await?;
        Ok::<_, rbatis::Error>(count)
    });

    if let Err(err) = &result {
        eprintln!("transaction rollback test error: {}", err);
    }
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_nested_transaction() {
    let rb = setup_test();

    let result = block_on(async move {
        let tx = rb.acquire_begin().await?;

        tx.exec(
            "INSERT INTO test_tx (id, name) VALUES (?, ?)",
            vec![Value::I32(5), Value::String("tx_test5".to_string())],
        )
        .await?;
        tx.exec(
            "INSERT INTO test_tx (id, name) VALUES (?, ?)",
            vec![Value::I32(6), Value::String("tx_test6".to_string())],
        )
        .await?;

        tx.commit().await?;

        let count: i64 = rb
            .exec_decode("SELECT COUNT(*) as count FROM test_tx", vec![])
            .await?;
        Ok::<_, rbatis::Error>(count)
    });

    if let Err(err) = &result {
        eprintln!("nested transaction test error: {}", err);
    }
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, 2);
}

#[test]
fn test_transaction_guard() {
    let rb = setup_test();

    let result = block_on(async move {
        let tx = rb.acquire_begin().await?;

        let tx_guard = tx.defer_async(|_tx| async move {
            println!("Transaction completed");
        });

        tx_guard
            .tx
            .exec(
                "INSERT INTO test_tx (id, name) VALUES (?, ?)",
                vec![Value::I32(7), Value::String("guard_test".to_string())],
            )
            .await?;

        tx_guard.commit().await?;

        let count: i64 = rb
            .exec_decode("SELECT COUNT(*) as count FROM test_tx", vec![])
            .await?;
        Ok::<_, rbatis::Error>(count)
    });

    if let Err(err) = &result {
        eprintln!("transaction guard test error: {}", err);
    }
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, 1);
}
