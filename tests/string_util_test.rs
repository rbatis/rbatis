#[cfg(test)]
mod test {
    use rbatis::utils::string_util::{find_convert_string, to_snake_name};

    #[test]
    fn test_to_snake_name() {
        assert_eq!("abc_def",to_snake_name("AbcDeF"));
    }

    #[test]
    fn test_find() {
        let sql = "update user set name=#{name}, password=#{password} ,sex=#{sex}, phone=#{phone}, delete_flag=#{flag}, #{name} #{ 1 + ";
        let finds = find_convert_string(sql);
        println!("{:?}", finds);
        assert_eq!(finds.len(), 5);
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
            index += 1;
        }
        println!("{:?}", finds);
    }

    #[test]
    fn test_find_fail() {
        let sql = "select #{column   #{  }";
        let finds = find_convert_string(sql);
        println!("{:?}", finds);
    }

    ///cargo test --release --package rbatis --test string_util_test test::bench_find --no-fail-fast -- --exact -Z unstable-options --show-output
    #[test]
    fn bench_find() {
        let sql = "update user set name=#{name}, password=#{password} ,sex=#{sex}, phone=#{phone}, delete_flag=#{flag}, #{name}";
        find_convert_string(sql);
        rbatis::bench!(100000, {
            find_convert_string(sql);
        });
    }
}
