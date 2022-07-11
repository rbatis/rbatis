use rbdc::db::MetaData;

pub struct MysqlMetaData {}

impl MetaData for MysqlMetaData {
    fn column_len(&self) -> u64 {
        todo!()
    }

    fn column_name(&self, i: usize) -> String {
        todo!()
    }

    fn column_type(&self, i: usize) -> String {
        todo!()
    }
}
