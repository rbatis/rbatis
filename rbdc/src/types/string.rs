use crate::Type;

/// string subtype date,time,datetime,timestamp,decimal,uuid,json
/// you can see test mod
impl Type for &str {
    fn type_name(&self) -> &'static str {
        let bytes = self.as_bytes();
        //Date RFC3339 = "2006-01-02"
        if self.len() == 10 && (bytes[4] == '-' as u8 && bytes[7] == '-' as u8) {
            return "date";
        }
        //Time RFC3339 = "15:04:05.999999"
        if self.len() >= 8 && (bytes[2] == ':' as u8 && bytes[5] == ':' as u8) {
            return "time";
        }
        //DateTime RFC3339 = "2006-01-02 15:04:05.999999"
        if bytes[4] == '-' as u8
            && bytes[7] == '-' as u8
            && bytes[13] == ':' as u8
            && bytes[16] == ':' as u8
        {
            return "datetime";
        }
        //TimeStamp = 9999999999999Z
        if bytes.len() == 14
            && bytes[bytes.len() - 1].eq(&('Z' as u8))
            && is_uint(&bytes[..(bytes.len() - 2)])
        {
            return "timestamp";
        }
        //Decimal   = 12345678D
        if bytes.len() >= 2
            && bytes[bytes.len() - 1].eq(&('D' as u8))
            && is_decimal(&bytes[..(bytes.len() - 2)])
        {
            return "decimal";
        }
        //UUID-V4 = 4b3f82bc-fa70-48e5-914c-17f0c8d246e2
        if self.len() == 36
            && bytes[8] == '-' as u8
            && bytes[13] == '-' as u8
            && bytes[18] == '-' as u8
            && bytes[23] == '-' as u8
        {
            return "uuid";
        };
        // json = {"abc":"efg"}
        if bytes[0] == '{' as u8 && bytes[bytes.len() - 1] == '}' as u8 {
            return "json";
        }
        // json array json = [{"abc":"efg"}]
        if bytes.len() >= 4 && bytes[0] == '[' as u8 && bytes[bytes.len() - 1] == ']' as u8 {
            if bytes[1] == '{' as u8 && bytes[bytes.len() - 2] == '}' as u8 {
                return "json";
            }
        }
        return "string";
    }
}

impl Type for String {
    fn type_name(&self) -> &'static str {
        self.as_str().type_name()
    }
}

fn is_uint(arg: &[u8]) -> bool {
    for x in arg {
        if !(('0' as u8).eq(x)
            || ('1' as u8).eq(x)
            || ('2' as u8).eq(x)
            || ('3' as u8).eq(x)
            || ('4' as u8).eq(x)
            || ('5' as u8).eq(x)
            || ('6' as u8).eq(x)
            || ('7' as u8).eq(x)
            || ('8' as u8).eq(x)
            || ('9' as u8).eq(x))
        {
            return false;
        }
    }
    return true;
}

fn is_decimal(arg: &[u8]) -> bool {
    for x in arg {
        if !(('0' as u8).eq(x)
            || ('1' as u8).eq(x)
            || ('2' as u8).eq(x)
            || ('3' as u8).eq(x)
            || ('4' as u8).eq(x)
            || ('5' as u8).eq(x)
            || ('6' as u8).eq(x)
            || ('7' as u8).eq(x)
            || ('8' as u8).eq(x)
            || ('9' as u8).eq(x)
            || ('.' as u8).eq(x))
        {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use crate::Type;

    #[test]
    fn test_date() {
        assert_eq!("2006-01-02".type_name(), "date")
    }
    #[test]
    fn test_time() {
        assert_eq!("15:04:05.999999".type_name(), "time")
    }
    #[test]
    fn test_datetime() {
        assert_eq!("2006-01-02T15:04:05.999999".type_name(), "datetime")
    }
    #[test]
    fn test_timestamp() {
        assert_eq!("9999999999999Z".type_name(), "timestamp")
    }
    #[test]
    fn test_deciaml() {
        assert_eq!("9999999999999.99999999D".type_name(), "decimal")
    }
    #[test]
    fn test_uuid() {
        assert_eq!("4b3f82bc-fa70-48e5-914c-17f0c8d246e2".type_name(), "uuid")
    }
    #[test]
    fn test_json() {
        assert_eq!(r#"{"abc":null}"#.type_name(), "json");
        assert_eq!(r#"[{"abc":null}]"#.type_name(), "json");
    }
}
