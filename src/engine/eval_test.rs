#[cfg(test)]
mod test {
    use std::{thread, time};

    use chrono::Local;
    use serde_json::{json, Value};

    use crate::engine::{parser, runtime};
    use crate::engine::node::Node;
    use crate::engine::runtime::OptMap;
    use crate::utils::time_util;

    #[test]
    fn test_eval_arg() {
        let box_node = parser::parse("-1 == -1", &OptMap::new()).unwrap();
        println!("{:#?}", box_node);
        let john = json!({
    });
        let v = box_node.eval(&john).unwrap();
        println!("{:?}", v);
    }

    #[test]
    fn test_mem_gc() {
        let box_node: Node = parser::parse("'1'+'1'", &OptMap::new()).unwrap();
        let john = json!({
        "n":1,
        "name": "John Doe",
         "age": {
           "yes":"sadf"
        }
    });

        let total = 10000000;
        println!("start");

        for _loop in 0..3 {
            for i in 0..total {
                box_node.eval(&john);
                if i == (total - 1) {
                    println!("done:{}", _loop);
                    let ten_millis = time::Duration::from_secs(5);
                    thread::sleep(ten_millis);
                }
                if i % 1000000 == 0 {
                    println!("number:{}", i)
                }
            }
        }
    }

    #[test]
    fn test_node_run() {
        let arg = json!({"a":1,"b":2,"c":"c", "d":null,});
        let parse_func = |arg: &serde_json::Value, expr: &str| -> serde_json::Value{
            println!("{}", expr.clone());
            let box_node = parser::parse(expr, &OptMap::new()).unwrap();
            box_node.eval(arg).unwrap()
        };
        assert_eq!(parse_func(&arg, "d.a == null"), json!(true));
        assert_eq!(parse_func(&arg, "1 == 1.0"), json!(true));
        assert_eq!(parse_func(&arg, "'2019-02-26' == '2019-02-26'"), json!(true));
        assert_eq!(parse_func(&arg, "`f`+`s`"), json!("fs"));
        assert_eq!(parse_func(&arg, "a +1 > b * 8"), json!(false));
        assert_eq!(parse_func(&arg, "a >= 0"), json!(true));
        assert_eq!(parse_func(&arg, "'a'+c"), json!("ac"));
        assert_eq!(parse_func(&arg, "b"), json!(2));
        assert_eq!(parse_func(&arg, "a < 1"), json!(false));
        assert_eq!(parse_func(&arg, "a +1 > b*8"), json!(false));
        assert_eq!(parse_func(&arg, "a * b == 2"), json!(true));
        assert_eq!(parse_func(&arg, "a - b == 0"), json!(false));
        assert_eq!(parse_func(&arg, "a >= 0 && a != 0"), json!(true));
        assert_eq!(parse_func(&arg, "a == 1 && a != 0"), json!(true));
        assert_eq!(parse_func(&arg, "1 > 3 "), json!(false));
        assert_eq!(parse_func(&arg, "1 + 2 != nil"), json!(true));
        assert_eq!(parse_func(&arg, "1 != null"), json!(true));
        assert_eq!(parse_func(&arg, "1 + 2 != nil && 1 > 0 "), json!(true));
        assert_eq!(parse_func(&arg, "1 + 2 != nil && 2 < b*8 "), json!(true));
        assert_eq!(parse_func(&arg, "-1 != nil"), json!(true));
        assert_eq!(parse_func(&arg, "-1 != -2 && -1 == 2-3 "), json!(true));
        assert_eq!(parse_func(&arg, "-1 == a*-1 "), json!(true));
        assert_eq!(parse_func(&arg, "-1 + a*-1 "), json!(-2.0));
        assert_eq!(parse_func(&arg, "2 ** 3"), json!(8.0));
    }


    #[test]
    fn test_string_node() {
        let str_node = Node::new_string("sadf");
        str_node.eval(&Value::Null {});
        //println!("value:{}", result);
    }

    #[test]
    fn test_arg_node() {
        let john = json!({
        "name": "John Doe",
        "age": Value::Null,
         "sex":{
            "a":"i'm a",
            "b":"i'm b",
         },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

        let arg_node = Node::new_arg("sex.a");
        arg_node.eval(&john);
        //println!("value:{},error:{}", result, Error);
    }

    #[test]
    fn benchmark_arg_node() {
        let john = json!({
        "name": "John Doe",
        "age": Value::Null,
         "sex":{
            "a":"i'm a",
            "b":"i'm b",
         },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

        let arg_node = Node::new_arg("sex.a");

        let total = 100000;
        let now = std::time::Instant::now();
        for i in 0..total {
            arg_node.eval(&john);
        }
        time_util::count_time_qps("benchmark_arg_node", total, now);
    }

    #[test]
    fn test_number_node() {
        let john = json!({
        "name": "John Doe",
        "age": 1,
         "sex":{
            "a":"i'm a",
            "b":"i'm b",
         },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
        let numb = Node::new_number_f64(1.02 as f64);
        numb.eval(&john);
        // println!("{}", value);
    }

    #[test]
    fn benchmark_parser_token() {
        let s = "'2019-02-26' == '2019-02-26'".to_string();
        let opt_map = OptMap::new();

        let total = 100000;
        let now = std::time::Instant::now();
        for i in 0..total {
            runtime::parse_tokens(&s, &opt_map);
        }
        time_util::count_time_qps("benchmark_parser_token", total, now);
    }
}