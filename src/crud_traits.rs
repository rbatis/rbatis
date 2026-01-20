use rbs::Value;

/// take `vec![Table{"id":1}]` columns
pub trait ColumnSet {
    fn column_sets(&self) -> Value;
}

impl ColumnSet for Value {
    fn column_sets(&self) -> Value {
        // Fast path: empty array
        let array = match self.as_array() {
            Some(arr) if !arr.is_empty() => arr,
            _ => return rbs::Value::Array(vec![]),
        };

        // Fast path: single element - directly extract non-null columns
        // This avoids HashSet allocation for the common case (single row insert)
        if array.len() == 1 {
            if let Some(table) = array.first() {
                let mut column_datas = Vec::with_capacity(table.len());
                for (column, value) in table {
                    if *value != rbs::Value::Null {
                        column_datas.push(column.clone());
                    }
                }
                return rbs::Value::from(column_datas);
            }
        }

        // Slow path: multiple elements - collect non-null columns across all rows
        let mut column_set = std::collections::HashSet::with_capacity(array.len() * 8);
        for item in array {
            for (k, v) in item {
                if *v != rbs::Value::Null {
                    column_set.insert(k.clone());
                }
            }
        }

        if let Some(table) = array.first() {
            let mut column_datas = Vec::with_capacity(table.len());
            for (column, _) in table {
                if column_set.contains(&column) {
                    column_datas.push(column.clone());
                }
            }
            rbs::Value::from(column_datas)
        } else {
            rbs::Value::Array(vec![])
        }
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

/// Filter value by specified columns
pub trait FilterByColumns {
    fn filter_by_columns(&self, columns: &Value) -> Value;
}

impl FilterByColumns for Value {
    fn filter_by_columns(&self, columns: &Value) -> Value {
        match self {
            Value::Map(map) => {
                // Extract column names from the columns Value
                let column_names = match columns {
                    Value::Array(arr) => {
                        if arr.is_empty() {
                            // Empty array means no columns to filter, return empty map
                            return Value::Map(rbs::value::map::ValueMap::new());
                        }
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .collect::<std::collections::HashSet<&str>>()
                    }
                    _ => {
                        // If columns is not an array, return original value
                        return self.clone();
                    }
                };

                if column_names.is_empty() {
                    // No valid column names, return empty map
                    return Value::Map(rbs::value::map::ValueMap::new());
                }

                let mut filtered_map = rbs::value::map::ValueMap::new();
                for (key, value) in map {
                    if let Some(key_str) = key.as_str() {
                        if column_names.contains(key_str) {
                            filtered_map.insert(key.clone(), value.clone());
                        }
                    }
                }

                Value::Map(filtered_map)
            }
            _ => self.clone()
        }
    }
}
