use rbatis::executor::{Executor, RBatisRef};
use rbatis::rbatis::RBatis;
use rbdc::rt::block_on;
use rbdc_sqlite::SqliteDriver;
use rbs::Value;

#[test]
fn test_exec_query() {
    let rb = make_test_rbatis();

    #[derive(serde::Deserialize, Debug)]
    struct TestRow {
        id: i32,
        name: String,
    }

    let result = block_on(async move {
        rb.exec(
            "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY, name TEXT)",
            vec![],
        )
        .await?;

        rb.exec(
            "INSERT INTO test_table (id, name) VALUES (?, ?)",
            vec![Value::I32(1), Value::String("test1".to_string())],
        )
        .await?;
        rb.exec(
            "INSERT INTO test_table (id, name) VALUES (?, ?)",
            vec![Value::I32(2), Value::String("test2".to_string())],
        )
        .await?;

        let result: Vec<TestRow> = rb
            .exec_decode("SELECT * FROM test_table WHERE id = ?", vec![Value::I32(1)])
            .await?;
        Ok::<_, rbatis::Error>(result)
    });

    assert!(result.is_ok());
    let rows = result.unwrap();
    let row = rows.first().unwrap();
    assert_eq!(row.id, 1);
    assert_eq!(row.name, "test1");
}

#[test]
fn test_exec_decode() {
    let rb = make_test_rbatis();

    #[derive(serde::Deserialize, Debug)]
    struct TestRow {
        id: i32,
        name: String,
    }

    let result = block_on(async move {
        rb.exec(
            "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY, name TEXT)",
            vec![],
        )
        .await?;
        rb.exec("DELETE FROM test_table", vec![]).await?;
        rb.exec(
            "INSERT INTO test_table (id, name) VALUES (?, ?)",
            vec![Value::I32(3), Value::String("test3".to_string())],
        )
        .await?;
        let result: Vec<TestRow> = rb
            .exec_decode("SELECT * FROM test_table WHERE id = ?", vec![Value::I32(3)])
            .await?;
        Ok::<_, rbatis::Error>(result)
    });

    assert!(result.is_ok());
    let rows = result.unwrap();
    let row = rows.first().unwrap();
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

    #[derive(serde::Deserialize, Debug)]
    struct TestRow {
        name: String,
    }

    let result = block_on(async move {
        rb.exec(
            "CREATE TABLE IF NOT EXISTS tx_test (id INTEGER PRIMARY KEY, name TEXT)",
            vec![],
        )
        .await?;
        rb.exec("DELETE FROM tx_test", vec![]).await?;

        let tx = rb.acquire_begin().await?;

        tx.exec(
            "INSERT INTO tx_test (id, name) VALUES (?, ?)",
            vec![Value::I32(1), Value::String("tx_test".to_string())],
        )
        .await?;

        tx.commit().await?;

        let result: Vec<TestRow> = rb
            .exec_decode("SELECT * FROM tx_test WHERE id = ?", vec![Value::I32(1)])
            .await?;
        Ok::<_, rbatis::Error>(result)
    });

    assert!(result.is_ok());
    let rows = result.unwrap();
    let row = rows.first().unwrap();
    assert_eq!(row.name, "tx_test");
}

#[test]
fn test_transaction_rollback() {
    let rb = make_test_rbatis();

    let result = block_on(async move {
        rb.exec(
            "CREATE TABLE IF NOT EXISTS tx_test (id INTEGER PRIMARY KEY, name TEXT)",
            vec![],
        )
        .await?;
        rb.exec("DELETE FROM tx_test", vec![]).await?;

        let tx = rb.acquire_begin().await?;

        tx.exec(
            "INSERT INTO tx_test (id, name) VALUES (?, ?)",
            vec![Value::I32(2), Value::String("should_rollback".to_string())],
        )
        .await?;

        tx.rollback().await?;

        let result = rb
            .query("SELECT * FROM tx_test WHERE id = ?", vec![Value::I32(2)])
            .await?;
        Ok::<_, rbatis::Error>(result)
    });

    assert!(result.is_ok());
    let result = result.unwrap();
    let arr = result.as_array().unwrap();
    assert_eq!(arr.len(), 0);
}

#[test]
fn test_transaction_exec_decode() {
    let rb = make_test_rbatis();

    #[derive(serde::Deserialize, Debug)]
    struct TestRow {
        id: i32,
        name: String,
    }

    let result = block_on(async move {
        rb.exec(
            "CREATE TABLE IF NOT EXISTS tx_test (id INTEGER PRIMARY KEY, name TEXT)",
            vec![],
        )
        .await?;
        rb.exec("DELETE FROM tx_test", vec![]).await?;
        rb.exec(
            "INSERT INTO tx_test (id, name) VALUES (?, ?)",
            vec![Value::I32(3), Value::String("decode_test".to_string())],
        )
        .await?;

        let tx = rb.acquire_begin().await?;

        let rows: Vec<TestRow> = tx
            .exec_decode("SELECT * FROM tx_test WHERE id = ?", vec![Value::I32(3)])
            .await?;

        tx.commit().await?;

        Ok::<_, rbatis::Error>(rows)
    });

    assert!(result.is_ok());
    let rows = result.unwrap();
    let row = rows.first().unwrap();
    assert_eq!(row.id, 3);
    assert_eq!(row.name, "decode_test");
}

#[test]
fn test_nested_transaction() {
    let rb = make_test_rbatis();

    #[derive(serde::Deserialize, Debug)]
    struct TestRow {
        name: String,
    }

    let result = block_on(async move {
        rb.exec(
            "CREATE TABLE IF NOT EXISTS nested_tx_test (id INTEGER PRIMARY KEY, name TEXT)",
            vec![],
        )
        .await?;
        rb.exec("DELETE FROM nested_tx_test", vec![]).await?;

        let tx = rb.acquire_begin().await?;

        tx.exec(
            "INSERT INTO nested_tx_test (id, name) VALUES (?, ?)",
            vec![Value::I32(1), Value::String("tx1".to_string())],
        )
        .await?;

        tx.exec(
            "INSERT INTO nested_tx_test (id, name) VALUES (?, ?)",
            vec![Value::I32(2), Value::String("tx2".to_string())],
        )
        .await?;

        tx.commit().await?;

        let result: Vec<TestRow> = rb
            .exec_decode("SELECT * FROM nested_tx_test ORDER BY id", vec![])
            .await?;
        Ok::<_, rbatis::Error>(result)
    });

    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name, "tx1");
    assert_eq!(rows[1].name, "tx2");
}

#[test]
fn test_transaction_with_defer() {
    let rb = make_test_rbatis();

    #[derive(serde::Deserialize, Debug)]
    struct TestRow {
        name: String,
    }

    let result = block_on(async move {
        rb.exec(
            "CREATE TABLE IF NOT EXISTS defer_tx_test (id INTEGER PRIMARY KEY, name TEXT)",
            vec![],
        )
        .await?;
        rb.exec("DELETE FROM defer_tx_test", vec![]).await?;

        let tx = rb.acquire_begin().await?;

        let guard = tx.defer_async(|tx| async move {
            let _ = tx.commit().await;
        });

        guard
            .tx
            .exec(
                "INSERT INTO defer_tx_test (id, name) VALUES (?, ?)",
                vec![Value::I32(1), Value::String("defer_test".to_string())],
            )
            .await?;

        guard.commit().await?;

        let result: Vec<TestRow> = rb
            .exec_decode(
                "SELECT * FROM defer_tx_test WHERE id = ?",
                vec![Value::I32(1)],
            )
            .await?;
        Ok::<_, rbatis::Error>(result)
    });

    assert!(result.is_ok());
    let rows = result.unwrap();
    let row = rows.first().unwrap();
    assert_eq!(row.name, "defer_test");
}

#[test]
fn test_executor_interface() {
    let rb = make_test_rbatis();

    #[derive(serde::Deserialize, Debug)]
    struct TestRow {
        id: i32,
        name: String,
    }

    let result = block_on(async move {
        rb.exec(
            "CREATE TABLE IF NOT EXISTS exec_test (id INTEGER PRIMARY KEY, name TEXT)",
            vec![],
        )
        .await?;
        rb.exec("DELETE FROM exec_test", vec![]).await?;

        let exec_result = Executor::exec(
            &rb,
            "INSERT INTO exec_test (id, name) VALUES (?, ?)",
            vec![Value::I32(1), Value::String("exec_test".to_string())],
        )
        .await?;

        assert_eq!(exec_result.rows_affected, 1);

        let rows: Vec<TestRow> = rb
            .exec_decode("SELECT * FROM exec_test WHERE id = ?", vec![Value::I32(1)])
            .await?;

        let row = rows.first().unwrap();
        assert_eq!(row.id, 1);
        assert_eq!(row.name, "exec_test");

        Ok::<_, rbatis::Error>(())
    });

    assert!(result.is_ok());
}

fn make_test_rbatis() -> RBatis {
    let rb = RBatis::new();
    block_on(async move {
        rb
            .link(SqliteDriver {}, "sqlite://:memory:")
            .await
            .unwrap();
        rb
    })
}
