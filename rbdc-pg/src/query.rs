use crate::arguments::PgArguments;
use crate::statement::PgStatement;
use either::Either;
use rbdc::Error;

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
    pub fn take_arguments(self) -> Result<Option<PgArguments>, Error> {
        if self.arguments.is_empty() {
            return Ok(None);
        }
        return Ok(Some(PgArguments::from_args(self.arguments)?));
    }

    #[inline]
    pub fn persistent(&self) -> bool {
        self.persistent
    }
}
