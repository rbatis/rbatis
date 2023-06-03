use once_cell::sync::Lazy;

pub static TEMPLATE: Lazy<SqlTemplates> = Lazy::new(|| SqlTemplates::default());

#[derive(Clone, Debug)]
pub struct Keywords {
    pub value: &'static str,
    pub left_space: &'static str,
    pub right_space: &'static str,
    pub left_right_space: &'static str,
}

macro_rules! gen_template {
     ({ $($key:ident:$value:tt$(,)?)+ }) => {
           /// Most of the SQL keywords used by the rbatis
           #[derive(Clone,Debug)]
           pub struct SqlTemplates {
               $(pub $key:Keywords,
               )+
           }
           impl Default for SqlTemplates {
               fn default() -> Self {
               if cfg!(feature = "upper_case_sql_keyword") {
                 Self{
                  $(
                    $key:Keywords{
                        value:  Box::leak(($value.to_uppercase().into_boxed_str())),
                        left_space: Box::leak((" ".to_string()+$value.to_uppercase().as_str()).into_boxed_str()),
                        right_space: Box::leak(($value.to_uppercase()+" ").into_boxed_str()),
                        left_right_space: Box::leak((format!(" {} ",$value.to_uppercase()).into_boxed_str())),
                    },
                  )+
                 }
               }else{
                 Self{
                  $(
                    $key:Keywords{
                        value:concat!("",$value,""),
                        left_space:concat!(" ",$value,""),
                        right_space:concat!("",$value," "),
                        left_right_space:concat!(" ",$value," "),
                    },
                  )+
                  }
               }


               }
          }
     }
}

gen_template!({
            r#where: "where",
            and: "and",
            or: "or",
            r#in: "in",
            having: "having",
            order_by: "order by",
            group_by: "group by",
            asc: "asc",
            desc: "desc",
            between: "between",
            not: "not",
            like: "like",
            is: "is",
            null: "NULL",
            insert_into: "insert into",
            values: "values",
            limit: "limit",
            set: "set",
            update: "update",
            select: "select",
            delete_from: "delete from",
            from: "from",
            r#as: "as",
            offset: "offset",
            rows_query_next: "rows query next",
            rows_only: "rows only",
});

#[test]
fn test_gen() {
    let t = SqlTemplates::default();
    println!("{:?}", t);
}
