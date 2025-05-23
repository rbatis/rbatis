pub mod into_sql;
pub use into_sql::IntoSql;
pub mod codegen;
pub mod error;
pub mod from_bool;
pub mod ops;
pub mod ops_add;
pub mod ops_bit_and;
pub mod ops_bit_or;
pub mod ops_cmp;
pub mod ops_div;
pub mod ops_eq;
pub mod ops_mul;
pub mod ops_neg;
pub mod ops_not;
pub mod ops_rem;
pub mod ops_sub;
pub mod ops_xor;
pub mod ops_string;

pub use codegen::{rb_html, rb_py};

#[cfg(test)]
mod tests {
    #[cfg(feature = "use_pest")]
    #[test]
    fn test_pest_parser() {
        use crate::codegen::parser_pysql_pest::parse_pysql;
        
        // 测试最简单的SQL语句
        let sql = "SELECT * FROM users";
        let nodes = parse_pysql(sql).unwrap();
        assert_eq!(1, nodes.len());
        
        // 测试带有if语句的SQL
        let sql = "SELECT * FROM users\nif name != null:\n  WHERE name = #{name}";
        let nodes = parse_pysql(sql).unwrap();
        assert!(nodes.len() > 1);
        
        // 测试带有括号的SQL
        let sql = "SELECT * FROM users WHERE (id > 10)";
        let nodes = parse_pysql(sql).unwrap();
        assert_eq!(1, nodes.len());
        
        // 测试带有反引号的SQL
        let sql = "`SELECT * FROM users`";
        let nodes = parse_pysql(sql).unwrap();
        assert_eq!(1, nodes.len());
    }
}
