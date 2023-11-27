use crate::{SqliteArguments, SqliteStatement};
use either::Either;
use rbdc::Error;

pub struct SqliteQuery {
    pub statement: Either<String, SqliteStatement>,
    pub arguments: Vec<rbs::Value>,
    pub persistent: bool,
}

impl SqliteQuery {
    #[inline]
    pub fn sql(&self) -> &str {
        match self.statement {
            Either::Right(ref statement) => &statement.sql,
            Either::Left(ref sql) => sql,
        }
    }

    pub fn statement(&self) -> Option<&SqliteStatement> {
        match self.statement {
            Either::Right(ref statement) => Some(&statement),
            Either::Left(_) => None,
        }
    }

    #[inline]
    pub fn take_arguments(self) -> Result<Option<SqliteArguments>, Error> {
        if self.arguments.is_empty() {
            return Ok(None);
        }
        return Ok(Some(SqliteArguments::from_args(self.arguments)?));
    }

    #[inline]
    pub fn persistent(&self) -> bool {
        self.persistent
    }
}
