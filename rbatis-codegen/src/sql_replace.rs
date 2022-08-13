

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
                if x == '?' && $format_char != '?' {
                    index += 1;
                    new_sql.push($format_char);
                    new_sql.push((index +48 ) as u8 as char);
                } else {
                    new_sql.push(x);
                }
            }
        }
        $sql = new_sql
    };
}

#[test]
fn test_id(){
    let mut sql="select * from table ? ? ?".to_string();
    sql_index!( sql,'$');
    println!("{}",sql);
    assert_eq!(sql,"select * from table $1 $2 $3")
}