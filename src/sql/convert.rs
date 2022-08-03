pub trait StmtConvert{
    fn stmt_convert(&self,index:usize,data:&mut String);
}


impl StmtConvert for String {
    fn stmt_convert(&self, index: usize, data: &mut String) {
        self.as_str().stmt_convert(index,data)
    }
}

impl StmtConvert for &str {
    fn stmt_convert(&self, index: usize, data: &mut String) {
        todo!()
    }
}