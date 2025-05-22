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
    fn operator_sql(&self) -> &str;
}

impl ValueOperatorSql for Value {
    fn operator_sql(&self) -> &str {
        match self {
            Value::Null => {"="}
            Value::Bool(_) => {"="}
            Value::I32(_) => {"="}
            Value::I64(_) => {"="}
            Value::U32(_) => {"="}
            Value::U64(_) => {"="}
            Value::F32(_) => {"="}
            Value::F64(_) => {"="}
            Value::String(v) => {
                if v.starts_with("%") || v.ends_with("%") {
                    "like"
                }else{
                    "="
                }
            }
            Value::Binary(_) => {"="}
            Value::Array(_) => {"="}
            Value::Map(_) => {"="}
            Value::Ext(_, v) => {v.operator_sql()}
        }
    }
}
