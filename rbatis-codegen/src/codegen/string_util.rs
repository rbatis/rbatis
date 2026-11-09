use std::collections::LinkedList;

//find like #{*,*},${*,*} value *
pub fn find_convert_string(arg: &str) -> LinkedList<(String, String)> {
    let mut list = LinkedList::new();
    let chars: Vec<u8> = arg.bytes().collect();
    let mut item = String::with_capacity(arg.len());
    let mut index: i32 = -1;
    for v in &chars {
        index += 1;
        if !item.is_empty() {
            item.push(*v as char);
            if *v == b'}' {
                let key = item[2..item.len() - 1].to_string();
                list.push_back((key, item.clone()));
                item.clear();
            }
            continue;
        }
        if (*v == b'#' || *v == b'$') && chars.get(index as usize + 1).eq(&Some(&b'{')) {
            item.push(*v as char);
        }
    }
    list
}

pub fn count_string_num(s: &str, c: char) -> usize {
    let cs = s.chars();
    let mut num = 0;
    for x in cs {
        if x == c {
            num += 1;
        }
    }
    num
}

/// Escape a value before writing it into a `name="value"` attribute of the HTML
/// the code generators build for each other
/// ([`crate::codegen::syntax_tree_pysql`] writes it,
/// [`crate::codegen::loader_html`] reads it).
///
/// HTML entities are decoded by the parser, so the attribute value survives a
/// write/read round trip unchanged. Replacing `"` with `'` would not be lossless:
/// it changes the expression source, and a value containing both quote kinds
/// could not be represented at all.
pub fn escape_attr_value(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            _ => escaped.push(ch),
        }
    }
    escaped
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
    column
}

/// Efficiently concatenates SQL fragments with automatic space insertion.
/// Optimized for hot path in SQL code generation.
#[inline(always)]
pub fn concat_str(text: &mut String, append_str: &str) {
    // Fast path: empty text - just push (no space needed)
    if text.is_empty() {
        text.push_str(append_str);
        return;
    }

    // Check if we need to add a space between fragments
    // Space is added when: text doesn't end with space AND append_str doesn't start with space
    let need_space = !text.ends_with(' ') && !append_str.starts_with(' ');

    // Reserve capacity to reduce reallocations
    if need_space {
        text.reserve(1 + append_str.len());
        text.push(' ');
        text.push_str(append_str);
    } else {
        text.push_str(append_str);
    }
}
