use crate::arguments::PgArguments;
use crate::statement::PgStatement;
use either::Either;

/// Raw SQL query with bind parameters. Returned by [`query`][crate::query::query].
#[must_use = "query must be executed to affect database"]
pub struct PgQuery {
    pub statement: Either<String, PgStatement>,
    pub arguments: Vec<rbs::Value>,
    pub persistent: bool,
}

impl PgQuery {
    #[inline]
    pub fn sql(&self) -> &str {
        match self.statement {
            Either::Right(ref statement) => &statement.sql,
            Either::Left(ref sql) => sql,
        }
    }

    pub fn statement(&self) -> Option<&PgStatement> {
        match self.statement {
            Either::Right(ref statement) => Some(&statement),
            Either::Left(_) => None,
        }
    }

    #[inline]
    pub fn take_arguments(mut self) -> Option<PgArguments> {
        if self.arguments.is_empty() {
            return None;
        }
        //Value to MysqlArguments
        return Some({ PgArguments::from(self.arguments) });
    }

    #[inline]
    pub fn persistent(&self) -> bool {
        self.persistent
    }
}
