#[cfg(test)]
mod test {
    use rbatis::interpreter::sql::py_sql::PyRuntime;
    use rbatis_core::db::DriverType;
    use rexpr::runtime::RExprRuntime;
    use rbatis::utils::bencher::QPS;

    #[test]
    pub fn test_bench_py_sqsl(){
        let py_runtime=PyRuntime::new(vec![]);
        let engine=RExprRuntime::new();
        let (sql,arg)=py_runtime.eval(&DriverType::Mysql, "select * from table where
                                                                                                if 1 == 1:
                                                                                                   column = 1",
                                       &mut serde_json::json!({}), &engine).unwrap();
        println!("sql:{},arg:{:?}",sql,arg);
        let total=10000;
        let now=std::time::Instant::now();
        for _ in 0..total{
            let (sql,arg) = py_runtime.eval(&DriverType::Mysql, "select * from table where
                                                                                                if 1 == 1:
                                                                                                   column = 1",
                                          &mut serde_json::json!({}), &engine).unwrap();
        }
        now.time(total);
        now.qps(total);
    }
}
