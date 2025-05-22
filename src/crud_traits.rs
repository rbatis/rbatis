use rbs::Value;

/// take `vec![Table{"id":1}]` columns
pub trait ColumnSet {
    fn column_sets(&self) -> Value;
}
impl ColumnSet for Value {
    fn column_sets(&self) -> Value {
        let len = self.len();
        let mut column_set = std::collections::HashSet::with_capacity(len);
        if let Some(array) = self.as_array(){
            for item in array {
                for (k,v) in &item {
                    if (*v) != rbs::Value::Null{
                        column_set.insert(k);
                    }
                }
            }
        }
        let mut columns = rbs::Value::Array(vec![]);
        if len > 0 {
            let table = &self[0];
            let mut column_datas = Vec::with_capacity(table.len());
            for (column, _) in table {
                if column_set.contains(&column) {
                    column_datas.push(column);
                }
            }
            columns = rbs::Value::from(column_datas);
        }
        columns
    }
}



/// create sql opt from rbs::Value
pub trait ValueOperatorSql {
    fn operator_sql(&self) -> String;
}

impl ValueOperatorSql for Value {
    fn operator_sql(&self) -> String {
        match self {
            Value::String(v) => {
                if v.contains(" ") {
                    v.to_string()
                }else{
                    format!("{}{}",v," = ")
                }
            }
            Value::Ext(_, v) => {v.operator_sql()}
            _=>{"".to_string()}
        }
    }
}
