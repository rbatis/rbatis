## json express interpreter engine

#### support token

| token    | doc  |
| ------ | ------ |
|   (*)    |    brackets    | 
|   %     |        | 
|   ^     |   xor     | 
|   *     |        | 
|   **     |   square     | 
|   /     |        | 
|   +     |        | 
|   -     |        | 
|   <=     |        | 
|   <     |        | 
|   >     |        | 
|   >=     |        | 
|   !=     |        | 
|   ==     |        | 
|   &&     |        | 
|   &#124;&#124;     |        | 

#### for example:

```
    #[test]
    fn test_node_run() {
        let arg = json!({"a":1,"b":2,"c":"c", "d":null,});
        let exec_expr = |arg: &serde_json::Value, expr: &str| -> serde_json::Value{
            println!("{}", expr.clone());
            let box_node = lexer::parse(expr, &OptMap::new()).unwrap();
            box_node.eval(arg).unwrap()
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
        assert_eq!(exec_expr(&arg, "null ^ null"), json!(0 ^ 0));
        assert_eq!(exec_expr(&arg, "null >= 0"), json!(true));
        assert_eq!(exec_expr(&arg, "null <= a"), json!(true));
    }
```