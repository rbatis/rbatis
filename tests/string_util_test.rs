#[cfg(test)]
mod test {
    use rbatis::utils::string_util::to_snake_name;
    use rbatis_codegen::codegen::string_util::find_convert_string;

    #[test]
    fn test_to_snake_name() {
        assert_eq!("abc_def",to_snake_name("AbcDeF"));
    }

    #[test]
    fn test_find() {
        let sql = "update user set name=#{name}, password=#{password} ,sex=#{sex}, phone=#{phone}, delete_flag=#{flag}, #{name} #{ 1 + ";
        let finds = find_convert_string(sql);
        println!("{:?}", finds);
        assert_eq!(finds.len(), 6);
        let mut index = 0;
        for (k, _) in &finds {
            if index == 0 {
                assert_eq!(k, "name");
            }
            if index == 1 {
                assert_eq!(k, "password");
            }
            if index == 2 {
                assert_eq!(k, "sex");
            }
            if index == 3 {
                assert_eq!(k, "phone");
            }
            if index == 4 {
                assert_eq!(k, "flag");
            }
            if index == 5 {
                assert_eq!(k, "name");
            }
            index += 1;
        }
        println!("{:?}", finds);
    }

    #[test]
    fn test_find_fail() {
        let sql = "select #{column   #{  }";
        let finds = find_convert_string(sql);
        println!("{:?}", finds);
        assert_eq!("column   #{  ",finds.iter().next().unwrap().0);
    }

    #[test]
    fn test_have_two() {
        let sql = "select #{data.user_id})=#{data.user_id}";
        let finds = find_convert_string(sql);
        println!("{:?}", finds);
        assert_eq!("data.user_id",finds.iter().next().unwrap().0);
    }
}
