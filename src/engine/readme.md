## express engine

> support opt

```
            Eq { express: "d.a == null", eq: json!(true) },
            Eq { express: "1 == 1.0", eq: json!(true) },
            Eq { express: "'2019-02-26' == '2019-02-26'", eq: json!(true) },
            Eq { express: "`f`+`s`", eq: json!("fs") },
            Eq { express: "a +1 > b * 8", eq: json!(false) },
            Eq { express: "a >= 0", eq: json!(true) },
            Eq { express: "'a'+c", eq: json!("ac") },
            Eq { express: "b", eq: json!(2) },
            Eq { express: "a < 1", eq: json!(false) },
            Eq { express: "a +1 > b*8", eq: json!(false) },
            Eq { express: "a * b == 2", eq: json!(true) },
            Eq { express: "a - b == 0", eq: json!(false) },
            Eq { express: "a >= 0 && a != 0", eq: json!(true) },
            Eq { express: "a == 1 && a != 0", eq: json!(true) },
            Eq { express: "1 > 3 ", eq: json!(false) },
            Eq { express: "1 + 2 != nil", eq: json!(true) },
            Eq { express: "1 != null", eq: json!(true) },
            Eq { express: "1 + 2 != nil && 1 > 0 ", eq: json!(true) },
            Eq { express: "1 + 2 != nil && 2 < b*8 ", eq: json!(true) },
            Eq { express: "-1 != nil", eq: json!(true) },
            Eq { express: "-1 != -2 && -1 == 2-3 ", eq: json!(true) },
            Eq { express: "-1 == a*-1 ", eq: json!(true) },
            Eq { express: "-1 + a*-1 ", eq: json!(-2.0) },
            Eq { express: "2 ** 3", eq: json!(8.0) },
```