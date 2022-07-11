pub trait Database: 'static + Send {
    fn name() -> &'static str
    where
        Self: Sized;
}

pub trait Connection: Send {}

pub trait Row: Send + Sync + 'static {
    /// Returns `true` if this row has no columns.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of columns in this row.
    #[inline]
    fn len(&self) -> usize {
        self.columns().len()
    }

    /// Gets all columns in this statement.
    fn columns(&self) -> &[Box<dyn Column>];

    /// Gets the column information at `index` or `None` if out of bounds.
    fn try_column(&self, index: rbs::Value) -> Option<&dyn Column>;

    /// Gets the column information at `index`.
    ///
    /// A string index can be used to access a column by name and a `usize` index
    /// can be used to access a column by position.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    /// See [`try_column`](Self::try_column) for a non-panicking version.
    fn column(&self, index: rbs::Value) -> &dyn Column {
        self.try_column(index).unwrap()
    }

    #[inline]
    fn get<'r>(&'r self, index: rbs::Value) -> rbs::ValueRef;
}

pub trait Column: 'static + Send + Sync {
    /// Gets the column ordinal.
    ///
    /// This can be used to unambiguously refer to this column within a row in case more than
    /// one column have the same name
    fn ordinal(&self) -> usize;

    /// Gets the column name or alias.
    ///
    /// The column name is unreliable (and can change between database minor versions) if this
    /// column is an expression that has not been aliased.
    fn name(&self) -> &str;

    /// Gets the type information for the column.
    fn type_info(&self) -> &dyn TypeInfo;
}

pub trait TypeInfo {}

#[cfg(test)]
mod test {
    use crate::db::Database;

    pub struct M {}

    impl Database for M {
        fn name() -> &'static str {
            "test"
        }
    }

    #[test]
    fn test_db() {
        let b: Box<dyn Database> = Box::new(M {});
    }
}
