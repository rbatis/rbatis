use std::convert::identity;
lazy_static!(
  pub static ref TEMPLATE:SqlTemplate = SqlTemplate::default();
);

/// Most of the SQL keywords used by the rbatis
pub struct SqlTemplate {
    pub r#where: &'static str,
    pub and: &'static str,
    pub or: &'static str,
    pub r#in: &'static str,
    pub having: &'static str,
    pub order_by: &'static str,
    pub group_by: &'static str,
    pub asc: &'static str,
    pub desc: &'static str,
    pub between: &'static str,
    pub not: &'static str,
    pub like: &'static str,
    pub is: &'static str,
    pub null: &'static str,
    pub insert_into: &'static str,
    pub values: &'static str,
    pub limit: &'static str,
    pub set: &'static str,
    pub update: &'static str,
    pub select: &'static str,
    pub delete_from: &'static str,
    pub from: &'static str,
    pub r#as: &'static str,
    pub offset: &'static str,

    //mssql
    pub rows_fetch_next: &'static str,
    pub rows_only: &'static str,
}

impl Default for SqlTemplate {
    fn default() -> Self {
        if cfg!(feature = "upper_case_sql") {
            return Self {
                r#where: " WHERE ",
                and: " AND ",
                or: " OR ",
                r#in: " IN ",
                having: " HAVING ",
                order_by: " ORDER BY ",
                group_by: " GROUP BY ",
                asc: " ASC ",
                desc: " DESC ",
                between: " BETWEEN ",
                not: " NOT ",
                like: " LIKE ",
                is: " IS ",
                null: " NULL ",
                insert_into: "INSERT INTO ",
                values: " VALUES ",
                limit: " LIMIT ",
                set: " SET ",
                update: "UPDATE ",
                select: "SELECT ",
                delete_from: "DELETE FROM ",
                from: " FROM ",
                r#as: " AS ",
                offset: " OFFSET ",
                rows_fetch_next: " ROWS FETCH NEXT ",
                rows_only: " ROWS ONLY ",
            };
        }
        Self {
            r#where: " where ",
            and: " and ",
            or: " or ",
            r#in: " in ",
            having: " having ",
            order_by: " order by ",
            group_by: " group by ",
            asc: " asc ",
            desc: " desc ",
            between: " between ",
            not: " not ",
            like: " like ",
            is: " is ",
            null: " NULL ",
            insert_into: "insert into ",
            values: " values ",
            limit: " limit ",
            set: " set ",
            update: "update ",
            select: "select ",
            delete_from: "delete from ",
            from: " from ",
            r#as: " as ",
            offset: " offset ",
            rows_fetch_next: " rows fetch next ",
            rows_only: " rows only ",
        }
    }
}


fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

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
               if cfg!(feature = "upper_case_sql") {
               Self{
                  $(
                    $key:Keywords{
                        value: string_to_static_str($value.to_string()),
                        left_space:string_to_static_str(" ".to_string()+$value),
                        right_space:string_to_static_str($value.to_string()+" "),
                        left_right_space:string_to_static_str(format!(" {} ",$value.to_string())),
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
            rows_fetch_next: "rows fetch next",
            rows_only: "rows only",
});

#[test]
fn test_gen() {
    let t = SqlTemplates::default();
    println!("{:?}", t);
}