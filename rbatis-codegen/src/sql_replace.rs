#[macro_export]
macro_rules! sql_index {
    ($sql:ident,$format_char:expr) => {
        let mut new_sql = String::with_capacity($sql.len() + 20);
        let mut string_start = false;
        let mut index: i32 = 0;
        for x in $sql.chars() {
            if x == '\'' || x == '"' {
                if string_start == true {
                    string_start = false;
                    new_sql.push(x);
                    continue;
                }
                string_start = true;
                new_sql.push(x);
                continue;
            }
            if string_start {
                new_sql.push(x);
            } else {
                use std::fmt::Write;
                if x == '?' && $format_char != '?' {
                    index += 1;
                    new_sql.push($format_char);
                    new_sql.write_fmt(format_args!("{}", index)).expect("a Display implementation returned an error unexpectedly");
                }
            }
        }
        $sql = new_sql
    };
}

#[test]
fn test_id() {
    let mut sql = "select * from table ".to_string();
    for i in 0..100 {
        sql.push_str("?");
    }
    sql_index!( sql,'$');
    println!("{}", sql);
    assert_eq!(sql, "$1$2$3$4$5$6$7$8$9$10$11$12$13$14$15$16$17$18$19$20$21$22$23$24$25$26$27$28$29$30$31$32$33$34$35$36$37$38$39$40$41$42$43$44$45$46$47$48$49$50$51$52$53$54$55$56$57$58$59$60$61$62$63$64$65$66$67$68$69$70$71$72$73$74$75$76$77$78$79$80$81$82$83$84$85$86$87$88$89$90$91$92$93$94$95$96$97$98$99$100")
}