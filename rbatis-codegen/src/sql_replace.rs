#[macro_export]
macro_rules! push_index {
    ($n:expr,$new_sql:ident,$index:expr) => {{
        let num = $index / $n;
        $new_sql.push((num + 48) as u8 as char);
        $index % $n
    }};
    ($index:ident,$new_sql:ident) => {
        if $index >= 0 && $index < 10 {
            $new_sql.push(($index + 48) as u8 as char);
        } else if $index >= 10 && $index < 100 {
            let $index = $crate::push_index!(10, $new_sql, $index);
            let $index = $crate::push_index!(1, $new_sql, $index);
        } else if $index >= 100 && $index < 1000 {
            let $index = $crate::push_index!(100, $new_sql, $index);
            let $index = $crate::push_index!(10, $new_sql, $index);
            let $index = $crate::push_index!(1, $new_sql, $index);
        } else if $index >= 1000 && $index < 10000 {
            let $index = $crate::push_index!(1000, $new_sql, $index);
            let $index = $crate::push_index!(100, $new_sql, $index);
            let $index = $crate::push_index!(10, $new_sql, $index);
            let $index = $crate::push_index!(1, $new_sql, $index);
        } else {
            use std::fmt::Write;
            $new_sql
                .write_fmt(format_args!("{}", $index))
                .expect("a Display implementation returned an error unexpectedly");
        }
    };
}

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
                    $crate::push_index!(index, new_sql);
                } else {
                    new_sql.push(x);
                }
            }
        }
        $sql = new_sql
    };
}
