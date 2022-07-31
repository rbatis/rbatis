pub mod driver;
pub mod decode;
pub mod encode;

pub struct OracleConnection {

}


#[cfg(test)]
mod tests {
    use rbdc::block_on;
    use rbdc::rt::tokio;

    #[test]
    fn test_oracle_pool() {
        let f=async move{
            use oracle::{Connection, Error};
            // Connect to a database.
            let conn = Connection::connect("scott", "tiger", "//localhost/XE").unwrap();
            let sql = "select ename, sal, comm from emp where deptno = :1";
        };
        block_on!(f);
    }
}
