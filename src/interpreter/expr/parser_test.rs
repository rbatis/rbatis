#[cfg(test)]
mod test {
    use core::time;
    use std::thread;
    use std::thread::Thread;
    use std::time::SystemTime;

    use chrono::Local;
    use serde_json::json;
    use serde_json::Value;

    use crate::interpreter::expr::{lexer, runtime};
    use crate::interpreter::expr::ast::{Node, NodeType};
    use crate::interpreter::expr::ast::NodeType::{NNumber, NOpt};
    use crate::interpreter::expr::parser::parse;
    use crate::interpreter::expr::token::TokenMap;
    use crate::utils;
    use crate::utils::bencher::QPS;
    use crate::utils::time_util;

    #[test]
    fn test_lexer() {
        let node = lexer::lexer_parse_node("-1 == -a", &TokenMap::new()).unwrap();
        println!("{:#?}", &node);
        let john = json!({
        "a":1,
        "name": "John Doe",
        "age": {
           "yes":"sadf"
        },
         "sex":{
            "a":"i'm a",
            "b":"i'm b",
         },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
        println!("result >>>>>>>>>>   =  {}", node.eval(&john).unwrap());
    }

    //cargo test --release --package rbatis --lib interpreter::json::lexer_test::test::test_benchmark_parse --no-fail-fast -- --exact -Z unstable-tokenions  --show-output
    #[test]
    fn test_benchmark_parse() {
        let total = 10000;
        let now = std::time::Instant::now();
        for _ in 0..total {
            let box_node = lexer::lexer("1+1", &TokenMap::new()).unwrap();
        }
        now.time(total)
    }

    #[test]
    fn test_benchmark() {
        let box_node = lexer::lexer_parse_node("1+1", &TokenMap::new()).unwrap();
        let john = json!({
        "name": "John Doe",
    });
        let total = 10000000;
        let now = std::time::Instant::now();
        for _ in 0..total {
            box_node.eval(&john);
        }
        now.time(total)
    }
}