use std::collections::LinkedList;

//find like #{*,*},${*,*} value *
pub fn find_convert_string(arg: &str) -> LinkedList<(String, String)> {
    let mut list = LinkedList::new();
    let chars: Vec<u8> = arg.bytes().collect();
    let mut item = String::with_capacity(arg.len());
    let mut index: i32 = -1;
    for v in &chars {
        index = index + 1;
        if !item.is_empty() {
            item.push(*v as char);
            if *v == '}' as u8 {
                let key = item[2..item.len() - 1].to_string();
                list.push_back((key, item.clone()));
                item.clear();
            }
            continue;
        }
        if (*v == '#' as u8 || *v == '$' as u8)
            && chars.get(index as usize + 1).eq(&Some(&('{' as u8)))
        {
            item.push(*v as char);
        }
    }
    return list;
}

pub fn count_string_num(s: &String, c: char) -> usize {
    let cs = s.chars();
    let mut num = 0;
    for x in cs {
        if x == c {
            num += 1;
        }
    }
    return num;
}

///input 'strings' => strings
pub fn un_packing_string(column: &str) -> &str {
    if column.len() >= 2 {
        if column.starts_with("'") && column.ends_with("'") {
            return &column[1..column.len() - 1];
        }
        if column.starts_with("`") && column.ends_with("`") {
            return &column[1..column.len() - 1];
        }
        if column.starts_with("\"") && column.ends_with("\"") {
            return &column[1..column.len() - 1];
        }
    }
    return column;
}

pub fn concat_str(text: &mut String, append_str: &str) {
    if append_str.is_empty() {
        return;
    }
    if !text.is_empty() 
        && !text.ends_with(' ')
        && !append_str.starts_with(' ')
        //以下两个判断条件内容是为了通过单元测试添加的内容
        //我觉得没必要所以更改了测试用例
        //&& !text.ends_with(',')
        //&& !append_str.starts_with(')') 
        {
        text.push(' ');
    }
    text.push_str(append_str);
}
