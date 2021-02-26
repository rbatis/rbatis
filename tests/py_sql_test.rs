#[cfg(test)]
mod test {
    use py_sql::py_sql::PyRuntime;
    use rbatis_core::db::DriverType;
    use rexpr::runtime::RExprRuntime;

    //cargo test --release --package rbatis --test py_sql_test test::test_bench_py_sqsl --no-fail-fast -- --exact -Z unstable-options --show-output
    #[test]
    pub fn test_bench_py_sqsl() {
        let py_runtime = PyRuntime::new(vec![]);
        let engine = RExprRuntime::new();
        let (sql, arg) = py_runtime.eval(&DriverType::Mysql, "select * from table where
                                                                                                if 1 == 1:
                                                                                                   column = 1
                                                                                                and age in (1,2,3)
                                                                                                if 2==2:
                                                                                                and age = 2",
                                         &mut serde_json::json!({}), &engine).unwrap();
        println!("sql:{},arg:{:?}", sql, arg);
        rbatis::bench!(10000, {
            py_runtime.eval(&DriverType::Mysql, "select * from table where
                                                                                                if 1 == 1:
                                                                                                   column = 1
                                                                                                and age in (1,2,3)
                                                                                                if 2==2:
                                                                                                and age = 2",
                                          &mut serde_json::json!({}), &engine).unwrap();
        });
    }

    //cargo test --release --package rbatis --test py_sql_test test::test_bench_py_sqsl_select --no-fail-fast -- --exact -Z unstable-options --show-output
    #[test]
    pub fn test_bench_py_sqsl_select() {
        let py_runtime = PyRuntime::new(vec![]);
        let engine = RExprRuntime::new();
        let (sql, arg) = py_runtime.eval(&DriverType::Mysql, "select * from table where
                                                                                                if 1 == 1:
                                                                                                   column = 1
                                                                                                and age in (1,2,3)
                                                                                                if 2==2:
                                                                                                and age = 2",
                                         &mut serde_json::json!({}), &engine).unwrap();
        println!("sql:{},arg:{:?}", sql, arg);
        rbatis::bench!(10000, {
            {
                py_runtime.eval(&DriverType::Mysql, "select * from table where
                                                                  id = #{id}
                                                                  id != #{id}
                                                                  id in #{ids}
                                                                  id in #{ids}
                                                                  id in #{ids}
                                                                  id not in #{ids}
                                                                  for k,v in map:
                                                                     #{k}=#{v}
                                                                  name like #{name}
                                                                  or
                                                                  name not like #{name}
                                                                  create_time between #{create_time} and #{create_time}
                                                                  group by
                                                                  for item in ids:
                                                                     #{item}
                                                                  order by
                                                                  for item in order_by:
                                                                     #{item}",
                            &mut serde_json::json!({"id":1,"order_by":["id","name"],"ids":[1,2,3],"name":"asdf","map":{"a":1},"create_time":"2020-23-23"}), &engine).unwrap();
            }
        });
    }
}
