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

fn has_single_line_comment(text: &str) -> bool {
    if text.trim_ascii().is_empty() {
        return false;
    }
    let text = text
        .rsplit_once('\n')
        .map(|(_, after)| after)
        .unwrap_or(text);

    // `--` single-line comment style
    if text.contains("--") {
        return true;
    }

    // MySQL `#` single-line comment style
    let bytes = text.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b'#' {
            // not match `#{...}`
            if i + 1 >= bytes.len() || bytes[i + 1] != b'{' {
                return true;
            }
        }
    }
    false
}

/// Efficiently concatenates SQL fragments with automatic space insertion.
/// Optimized for hot path in SQL code generation.
#[inline(always)]
pub fn concat_str(text: &mut String, append_str: &str) {
    if append_str.is_empty() {
        return;
    }
    // Append a linefeed at the end of a single-line comment
    let need_ln = !append_str.ends_with('\n') && has_single_line_comment(append_str);

    // Fast path: empty text - just push (no space needed)
    if text.is_empty() {
        let len = append_str.len() + if need_ln { 1 } else { 0 };
        text.reserve(len);
        text.push_str(append_str);
        if need_ln {
            text.push('\n');
        }
        return;
    }

    // Check if we need to add a space between fragments
    // Space is added when: text doesn't end with space AND append_str doesn't start with space
    let pat = &[' ', '\t', '\n'];
    let need_space = !text.ends_with(pat) && !append_str.starts_with(pat);

    // Reserve capacity to reduce reallocations
    let len = append_str.len() + if need_space { 1 } else { 0 } + if need_ln { 1 } else { 0 };
    text.reserve(len);
    if need_space {
        text.push(' ');
    }
    text.push_str(append_str);
    if need_ln {
        text.push('\n');
    }
}
