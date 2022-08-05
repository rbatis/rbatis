use crate::{SqliteArguments, SqliteStatement};
use either::Either;

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
    pub fn take_arguments(mut self) -> Option<SqliteArguments> {
        if self.arguments.is_empty() {
            return None;
        }
        return Some(SqliteArguments::from(self.arguments));
    }

    #[inline]
    pub fn persistent(&self) -> bool {
        self.persistent
    }
}
