#[cfg(test)]
mod test {
    use rbs::value::map::ValueMap;
    use rbs::Value;
    use rbatis_macro_driver::rb_py;

    #[rb_py(
        "
    SELECT * FROM biz_activity
    if  name != null:
      AND delete_flag = #{del}
      AND version = 1
      if  age!=1:
        AND version = 1
      AND version = 1
    AND a = 0
      yes
    and id in (
    trim ',': for item in ids:
      #{item},
    )
    and id in (
    trim ',': for index,item in ids:
      #{item},
    )
    trim 'AND':
      AND delete_flag = #{map.k}
    choose:
        when age==27:
          AND age = 27
        otherwise:
          AND age = 0
    WHERE id  = '2';"
    )]
    pub fn py_select_by_condition(arg: &mut rbs::Value, _tag: char) {}

    #[test]
    fn test_rbatis_codegen() {
        let mut arg = ValueMap::new();
        arg.insert("name".into(), "ss".into());
        arg.insert("del".into(), 1.into());
        arg.insert("del2".into(), 2.into());
        arg.insert(
            "ids".into(),
            vec![Value::I32(1), Value::I32(2), Value::I32(3)].into(),
        );
        arg.insert("map".into(), {
            let mut m=ValueMap::new();
            m.insert("k".into(),1.into());
            Value::Map(m)
        });
        let (sql, args) = py_select_by_condition(&rbs::Value::Map(arg), '$');
        println!("py->sql: {}", sql);
        println!("py->args: {}", serde_json::to_string(&args).unwrap());
    }

    #[rb_py(
        "insert into ${table_name} (
             trim ',':
               for k,v2 in table:
                 ${k},
             ) VALUES (
             trim ',':
               for k,v1 in table:
                 #{v1},
             )"
    )]
    pub fn save(arg: &mut rbs::Value, _tag: char) {}

    #[test]
    fn test_save() {
        let mut arg = ValueMap::new();
        arg.insert("table".into(), {
            let mut arg = ValueMap::new();
            arg.insert("table_name".into(), "a".into());
            arg.insert("table".into(), "a".into());
            Value::Map(arg)
        });
        let (sql, args) = save(&rbs::Value::Map(arg), '$');
        println!("py->sql: {}", sql);
        println!("py->args: {}", serde_json::to_string(&args).unwrap());
    }
}
