use crate::CommonType;

impl CommonType for &str {
    fn common_type(&self) -> &'static str {
        let bytes = self.as_bytes();
        ///Date RFC3339 = "2006-01-02"
        if self.len() == 10 && (bytes[4] == '-' as u8 && bytes[7] == '-' as u8) {
            return "date";
        }
        ///Time RFC3339 = "15:04:05.999999"
        if self.len() >= 8 && (bytes[2] == ':' as u8 && bytes[5] == ':' as u8) {
            return "time";
        }
        ///DateTime RFC3339 = "2006-01-02 15:04:05.999999"
        if self.len() == 19
            && (bytes[4] == '-' as u8
                && bytes[7] == '-' as u8
                && bytes[13] == ':' as u8
                && bytes[16] == ':' as u8)
        {
            return "datetime";
        }
        ///TimeStamp = 9999999999999Z
        if self.ends_with("Z") {
            return "timestamp";
        }
        ///Decimal   = 12345678D
        if self.ends_with("D") {
            return "decimal";
        }
        if bytes[0] == '{' as u8 && bytes[bytes.len() - 1] == '}' as u8 {
            return "json";
        }
        return "string";
    }
}

impl CommonType for String {
    fn common_type(&self) -> &'static str {
        self.as_str().common_type()
    }
}
