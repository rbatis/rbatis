//s.len()==n,time = O(n)
pub fn extract_number(s: &str) -> f64 {
    let iter = s.bytes().rev();
    let mut idx = s.len();
    for x in iter {
        if x.is_ascii_alphabetic() {
            idx -= 1;
        }
    }
    let inner = &s[0..idx];
    if inner.is_empty() {
        0.0
    } else {
        inner.parse().unwrap_or_default()
    }
}


#[cfg(test)]
mod test {
    use crate::value::util::extract_number;

    #[test]
    fn test() {
        assert_eq!(extract_number("1.2332ESF"), 1.2332);
        assert_eq!(extract_number("3.324324D"), 3.324324);
        assert_eq!(extract_number("123456789012345678901234567890"), 123456789012345678901234567890.0);
    }
}