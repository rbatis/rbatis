use crate::stmt::{MySqlArguments, MySqlStatement};
use either::Either;
use rbdc::Error;

#[must_use = "query must be executed to affect database"]
pub struct MysqlQuery {
    pub(crate) statement: Either<String, MySqlStatement>,
    pub(crate) arguments: Vec<rbs::Value>,
    pub(crate) persistent: bool,
}
impl MysqlQuery {
    #[inline]
    pub fn sql(&self) -> &str {
        match self.statement {
            Either::Right(ref statement) => &statement.sql,
            Either::Left(ref sql) => sql,
        }
    }

    pub fn statement(&self) -> Option<&MySqlStatement> {
        match self.statement {
            Either::Right(ref statement) => Some(&statement),
            Either::Left(_) => None,
        }
    }

    #[inline]
    pub fn take_arguments(self) -> Result<Option<MySqlArguments>, Error> {
        if self.arguments.is_empty() {
            return Ok(None);
        }
        //Value to MysqlArguments
        return Ok(Some(MySqlArguments::from_args(self.arguments)?));
    }

    #[inline]
    pub fn persistent(&self) -> bool {
        self.persistent
    }
}
