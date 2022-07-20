use crate::CommonType;

impl CommonType for &str {
    fn common_type(&self) -> &'static str {
        ///Date RFC3339 = "2006-01-02"
        ///DateTime RFC3339 = "2006-01-02 15:04:05.999999"
        ///Time RFC3339 = "15:04:05.999999"
        ///TimeStamp = 9999999999999Z
        ///Decimal   = 12345678D
        todo!()
    }
}
