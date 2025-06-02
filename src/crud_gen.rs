use crate::crud_traits::ColumnSet;
use crate::Error;
use rbatis_codegen::ops::AsProxy;
use rbs::Value;

pub async fn gen_insert_batch(table_name: &str, mut tables: Value) -> Result<(String, Vec<Value>), Error> {
    let mut sql = format!("insert into {table_name}");
    if tables.is_map() {
        tables = Value::Array(vec![tables]);
    }
    let mut args = {
        if tables.is_empty() {
            vec![]
        } else {
            let item_len = tables[0].len() * tables.len();
            Vec::with_capacity(item_len)
        }
    };
    let columns = &tables.column_sets();
    let columns_len = columns.len();
    let tables_len = tables.len();
    let mut idx = 0;
    for (_, table) in tables {
        if idx == 0 {
            sql.push_str("(");
            let mut idx1 = 0;
            for (_, v) in columns {
                sql.push_str(v.string().as_str());
                if (idx1 + 1) < columns_len {
                    sql.push_str(",");
                }
                idx1 += 1;
            }
            sql.push_str(") VALUES ");
        }
        sql.push_str("(");
        let mut idx1 = 0;
        for (_, v) in columns {
            let value = &table[v];
            sql.push_str("?");
            if (idx1 + 1) < columns_len {
                sql.push_str(",");
            }
            args.push(value.clone());
            idx1 += 1;
        }
        sql.push_str(")");
        if (idx + 1) < tables_len {
            sql.push_str(",");
        }
        idx += 1;
    }
    Ok((sql, args))
}
