//s.len()==n,time = O(n)
pub fn to_number(s: &str) -> f64 {
    let iter = s.bytes().rev();
    let mut idx_end = s.len();
    for x in iter {
        if matches!(x, b'A'..=b'Z' | b'a'..=b'z') {
            idx_end -= 1;
        } else {
            break;
        }
    }
    let iter = s.bytes();
    let mut idx = 0;
    for x in iter {
        if matches!(x, b'A'..=b'Z' | b'a'..=b'z') {
            idx += 1;
        } else {
            break;
        }
    }
    let inner = &s[idx..idx_end];
    if inner.is_empty() {
        0.0
    } else {
        inner.parse().unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use crate::value::util::to_number;

    #[test]
    fn test() {
        assert_eq!(to_number("ESF1.2332"), 1.2332);
        assert_eq!(to_number("1.2332ESF"), 1.2332);
        assert_eq!(to_number("3.324324D"), 3.324324);
        assert_eq!(
            to_number("123456789012345678901234567890"),
            123456789012345678901234567890.0
        );
    }
}
