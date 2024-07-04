pub trait ColumnSet{
    fn column_sets(&self)->rbs::Value;
}
impl ColumnSet for rbs::Value {
    fn column_sets(&self) -> rbs::Value {
        let len = self.len();
        let mut column_set = std::collections::HashSet::with_capacity(len);
        for item in self.as_array().unwrap() {
            for (k,v) in &item {
                if (*v) != rbs::Value::Null{
                    column_set.insert(k);
                }
            }
        }
        let mut columns = rbs::Value::Array(vec![]);
        if len > 0 {
            let table = &self[0];
            let mut column_datas = Vec::with_capacity(table.len());
            for (column,_) in table {
                if column_set.contains(&column){
                    column_datas.push(column);
                }
            }
            columns = rbs::Value::from(column_datas);
        }
        columns
    }
}