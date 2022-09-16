/// impl exchange
pub fn impl_exchange(start_str: &str, start_num: usize, sql: &str) -> String {
    let mut last = ' ' as u8;
    let mut sql = sql.to_string();
    let mut sql_bytes = sql.as_bytes();
    let mut placeholder_idx = start_num;
    let mut index = 0;
    loop {
        if index == sql_bytes.len() {
            break;
        }
        let x = sql_bytes[index];
        if x == '?' as u8 && last != '\\' as u8 {
            sql.remove(index);
            let mut i = 0;
            for x in start_str.chars() {
                sql.insert(index + i, x);
                i += 1;
            }
            sql.insert_str(
                index + start_str.len(),
                itoa::Buffer::new().format(placeholder_idx),
            );
            placeholder_idx += 1;
            sql_bytes = sql.as_bytes();
        } else {
            last = x;
        }
        index += 1;
    }
    sql
}
