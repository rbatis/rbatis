#[cfg(test)]
mod test {
    use std::{thread, time};

    use chrono::Local;
    use serde_json::{json, Value};

    use crate::interpreter::expr::{lexer, runtime};
    use crate::interpreter::expr::ast::Node;
    use crate::interpreter::expr::lexer::parse_tokens;
    use crate::interpreter::expr::token::TokenMap;
    use crate::utils::time_util;

    #[test]
    fn test_node_run() {
        let arg = json!({
        "a":1,
        "b":2,
        "c":"c",
        "d":null,
        "e":[1],
        "f":[{"field":1}]
         });
        let exec_expr = |arg: &serde_json::Value, expr: &str| -> serde_json::Value{
            println!("{}", expr.clone());
            let box_node = lexer::lexer_parse_node(expr, &TokenMap::new()).unwrap();
            //println!("{:#?}", &box_node);
            let v = box_node.eval(arg).unwrap();
            println!("'{}' -> {}", expr.clone(), &v);
            v
        };
        assert_eq!(exec_expr(&arg, "-1 == -a"), json!(true));
        assert_eq!(exec_expr(&arg, "d.a == null"), json!(true));
        assert_eq!(exec_expr(&arg, "1 == 1.0"), json!(true));
        assert_eq!(exec_expr(&arg, "'2019-02-26' == '2019-02-26'"), json!(true));
        assert_eq!(exec_expr(&arg, "`f`+`s`"), json!("fs"));
        assert_eq!(exec_expr(&arg, "a +1 > b * 8"), json!(false));
        assert_eq!(exec_expr(&arg, "a >= 0"), json!(true));
        assert_eq!(exec_expr(&arg, "'a'+c"), json!("ac"));
        assert_eq!(exec_expr(&arg, "b"), json!(2));
        assert_eq!(exec_expr(&arg, "a < 1"), json!(false));
        assert_eq!(exec_expr(&arg, "a +1 > b*8"), json!(false));
        assert_eq!(exec_expr(&arg, "a * b == 2"), json!(true));
        assert_eq!(exec_expr(&arg, "a - b == 0"), json!(false));
        assert_eq!(exec_expr(&arg, "a >= 0 && a != 0"), json!(true));
        assert_eq!(exec_expr(&arg, "a == 1 && a != 0"), json!(true));
        assert_eq!(exec_expr(&arg, "1 > 3 "), json!(false));
        assert_eq!(exec_expr(&arg, "1 + 2 != null"), json!(true));
        assert_eq!(exec_expr(&arg, "1 != null"), json!(true));
        assert_eq!(exec_expr(&arg, "1 + 2 != null && 1 > 0 "), json!(true));
        assert_eq!(exec_expr(&arg, "1 + 2 != null && 2 < b*8 "), json!(true));
        assert_eq!(exec_expr(&arg, "-1 != null"), json!(true));
        assert_eq!(exec_expr(&arg, "-1 != -2 && -1 == 2-3 "), json!(true));
        assert_eq!(exec_expr(&arg, "-3 == b*-1-1 "), json!(true));
        assert_eq!(exec_expr(&arg, "0-1 + a*0-1 "), json!(-2));
        assert_eq!(exec_expr(&arg, "2 ** 3"), json!(8.0));
        assert_eq!(exec_expr(&arg, "0-1 + -1*0-1 "), json!(-2));
        assert_eq!(exec_expr(&arg, "1-"), json!(1));
        assert_eq!(exec_expr(&arg, "-1"), json!(-1));
        assert_eq!(exec_expr(&arg, "1- -1"), json!(1--1));
        assert_eq!(exec_expr(&arg, "1-2 -1+"), json!(1-2-1));
        assert_eq!(exec_expr(&arg, "e[1]"), json!(null));
        assert_eq!(exec_expr(&arg, "e[0]"), json!(1));
        assert_eq!(exec_expr(&arg, "f[0].field"), json!(1));
        assert_eq!(exec_expr(&arg, "f.0.field"), json!(1));
        assert_eq!(exec_expr(&arg, "0.1"), json!(0.1));
        assert_eq!(exec_expr(&arg, "1"), json!(1));
        assert_eq!(exec_expr(&arg, "(1+1)"), json!(2));
        assert_eq!(exec_expr(&arg, "(1+5)>5"), json!((1+5)>5));
        assert_eq!(exec_expr(&arg, "(18*19)<19*19"), json!((18*19)<19*19));
        assert_eq!(exec_expr(&arg, "2*(1+1)"), json!(2*(1+1)));
        assert_eq!(exec_expr(&arg, "2*(1+(1+1)+1)"), json!(2*(1+(1+1)+1)));
        assert_eq!(exec_expr(&arg, "(((34 + 21) / 5) - 12) * 348"), json!((((34 + 21) / 5) - 12) * 348));
        assert_eq!(exec_expr(&arg, "11 ^ 1"), json!(11 ^ 1));
        assert_eq!(exec_expr(&arg, "null ^ null"), json!(0 ^ 0));
        assert_eq!(exec_expr(&arg, "null >= 0"), json!(true));
        assert_eq!(exec_expr(&arg, "null <= a"), json!(true));
    }

    #[test]
    fn test_eval_arg() {
        let box_node = lexer::lexer_parse_node("-1 == -a", &TokenMap::new()).unwrap();
        println!("{:#?}", box_node);
        let john = json!({
    });
        let v = box_node.eval(&john).unwrap();
        println!("{:?}", v);
    }

    #[test]
    fn test_mem_gc() {
        let box_node: Node = lexer::lexer_parse_node("'1'+'1'", &TokenMap::new()).unwrap();
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
        let numb = Node::new_f64(1.02 as f64);
        numb.eval(&john);
        // println!("{}", value);
    }

    #[test]
    fn test_gen() {
        let node = Node::new_binary(Node::new_i64(1), Node::new_i64(1), "+");
        println!("{}", node.eval(&serde_json::Value::Null).unwrap());
        let f = || {
            //ast lower
        };
    }
}