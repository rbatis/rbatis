//! Contains the `ColumnIndex`, `Row`, and `FromRow` traits.

use crate::database::Database;
use crate::decode::Decode;
use crate::types::{Type, TypeInfo};
use crate::value::{HasRawValue, RawValue};
use serde::de::DeserializeOwned;


/// A type that can be used to index into a [`Row`].
///
/// The [`get`] and [`try_get`] methods of [`Row`] accept any type that implements `ColumnIndex`.
/// This trait is implemented for strings which are used to look up a column by name, and for
/// `usize` which is used as a positional index into the row.
///
/// This trait is sealed and cannot be implemented for types outside of SQLx.
///
/// [`Row`]: trait.Row.html
/// [`get`]: trait.Row.html#method.get
/// [`try_get`]: trait.Row.html#method.try_get
pub trait ColumnIndex<'c, R>
where
    Self: private_column_index::Sealed,
    R: Row<'c> + ?Sized,
{
    /// Returns a valid positional index into the row, [`ColumnIndexOutOfBounds`], or,
    /// [`ColumnNotFound`].
    ///
    /// [`ColumnNotFound`]: ../enum.Error.html#variant.ColumnNotFound
    /// [`ColumnIndexOutOfBounds`]: ../enum.Error.html#variant.ColumnIndexOutOfBounds
    fn index(&self, row: &R) -> crate::Result<usize>;
}

impl<'c, R, I> ColumnIndex<'c, R> for &'_ I
where
    R: Row<'c>,
    I: ColumnIndex<'c, R> + ?Sized,
{
    #[inline]
    fn index(&self, row: &R) -> crate::Result<usize> {
        (**self).index(row)
    }
}

// Prevent users from implementing the `ColumnIndex` trait.
mod private_column_index {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl<T> Sealed for &'_ T where T: Sealed + ?Sized {}
}

/// Represents a single row from the database.
///
/// Applications should not generally need to use this trait. Values of this trait are only
/// encountered when manually implementing [`FromRow`] (as opposed to deriving) or iterating
/// a [`Cursor`] (returned from [`Query::fetch`]).
///
/// This trait is sealed and cannot be implemented for types outside of SQLx.
///
/// [`FromRow`]: crate::row::FromRow
/// [`Cursor`]: crate::cursor::Cursor
/// [`Query::fetch`]: crate::query::Query::fetch
pub trait Row<'c>
where
    Self: private_row::Sealed + Unpin + Send + Sync,
{
    /// The `Database` this `Row` is implemented for.
    type Database: Database;

    /// Returns `true` if this row has no columns.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of columns in this row.
    fn len(&self) -> usize;

    /// Index into the database row and decode a single value.
    ///
    /// A string index can be used to access a column by name and a `usize` index
    /// can be used to access a column by position.
    ///
    /// ```rust,ignore
    /// # let mut cursor = sqlx::query("SELECT id, name FROM users")
    /// #     .fetch(&mut conn);
    /// #
    /// # let row = cursor.next().await?.unwrap();
    /// #
    /// let id: i32 = row.get("id"); // a column named "id"
    /// let name: &str = row.get(1); // the second column in the result
    /// ```
    ///
    /// # Panics
    /// Panics if the column does not exist or its value cannot be decoded into the requested type.
    /// See [`try_get`](#method.try_get) for a non-panicking version.
    #[inline]
    fn get<T, I>(&self, index: I) -> T
    where
        T: Type<Self::Database>,
        I: ColumnIndex<'c, Self>,
        T: Decode<'c, Self::Database>,
    {
        self.try_get::<T, I>(index).unwrap()
    }

    /// Index into the database row and decode a single value.
    ///
    /// See [`try_get_unchecked`](#method.try_get_unchecked).
    #[inline]
    fn get_unchecked<T, I>(&self, index: I) -> T
    where
        T: Type<Self::Database>,
        I: ColumnIndex<'c, Self>,
        T: Decode<'c, Self::Database>,
    {
        self.try_get_unchecked::<T, I>(index).unwrap()
    }

    /// Index into the database row and decode a single value.
    ///
    /// A string index can be used to access a column by name and a `usize` index
    /// can be used to access a column by position.
    ///
    /// ```rust,ignore
    /// # let mut cursor = sqlx::query("SELECT id, name FROM users")
    /// #     .fetch(&mut conn);
    /// #
    /// # let row = cursor.next().await?.unwrap();
    /// #
    /// let id: i32 = row.try_get("id")?; // a column named "id"
    /// let name: &str = row.try_get(1)?; // the second column in the result
    /// ```
    ///
    /// # Errors
    ///  * [`ColumnNotFound`] if the column by the given name was not found.
    ///  * [`ColumnIndexOutOfBounds`] if the `usize` index was greater than the number of columns in the row.
    ///  * [`Decode`] if the value could not be decoded into the requested type.
    ///
    /// [`Decode`]: crate::Error::Decode
    /// [`ColumnNotFound`]: crate::Error::ColumnNotFound
    /// [`ColumnIndexOutOfBounds`]: crate::Error::ColumnIndexOutOfBounds
    fn try_get<T, I>(&self, index: I) -> crate::Result<T>
    where
        T: Type<Self::Database>,
        I: ColumnIndex<'c, Self>,
        T: Decode<'c, Self::Database>,
    {
        let value = self.try_get_raw(index)?;

        if let Some(expected_ty) = value.type_info() {
            // NOTE: If there is no type, the value is NULL. This is fine. If the user tries
            //       to get this into a non-Option we catch that elsewhere and report as
            //       UnexpectedNullError.
            if !expected_ty.compatible(&T::type_info()) {
                return Err(crate::Error::mismatched_types::<Self::Database, T>(
                    expected_ty,
                ));
            }
        }

        T::decode(value)
    }


    ///json decode
    fn json_decode<T, I>(&self, index: I) -> crate::Result<T>
        where
            I: ColumnIndex<'c, Self>,
            T: DeserializeOwned
    {
        let value = self.try_get_raw(index)?;
        let v = value.try_to_json();
        if (v.is_err()){
            return Err(decode_err!("unexpected value {:?} for serde_json::Value", v.err().unwrap()));
        }
        let t:Result<T,serde_json::Error> = serde_json::from_value(v.unwrap());
        match t {
            Ok(r)=>{
                return Ok(r);
            }
            Err(e)=>{
                return Err(decode_err!("unexpected value {:?} for serde_json::from_value", e.to_string()));
            }
        }
    }

    /// Index into the database row and decode a single value.
    ///
    /// Unlike [`try_get`](#method.try_get), this method does not check that the type
    /// being returned from the database is compatible with the Rust type and just blindly tries
    /// to decode the value. An example of where this could be useful is decoding a Postgres
    /// enumeration as a Rust string (instead of deriving a new Rust enum).
    #[inline]
    fn try_get_unchecked<T, I>(&self, index: I) -> crate::Result<T>
    where
        T: Type<Self::Database>,
        I: ColumnIndex<'c, Self>,
        T: Decode<'c, Self::Database>,
    {
        self.try_get_raw(index).and_then(T::decode)
    }

    #[doc(hidden)]
    fn try_get_raw<I>(
        &self,
        index: I,
    ) -> crate::Result<<Self::Database as HasRawValue<'c>>::RawValue>
    where
        I: ColumnIndex<'c, Self>;
}

// Prevent users from implementing the `Row` trait.
pub(crate) mod private_row {
    pub trait Sealed {}
}

/// Associate [`Database`] with a [`Row`] of a generic lifetime.
///
/// ---
///
/// The upcoming Rust feature, [Generic Associated Types], should obviate
/// the need for this trait.
///
/// [Generic Associated Types]: https://www.google.com/search?q=generic+associated+types+rust&oq=generic+associated+types+rust&aqs=chrome..69i57j0l5.3327j0j7&sourceid=chrome&ie=UTF-8
pub trait HasRow<'c> {
    type Database: Database;

    /// The concrete `Row` implementation for this database.
    type Row: Row<'c, Database = Self::Database>;
}

/// A record that can be built from a row returned by the database.
///
/// In order to use [`query_as`] the output type must implement `FromRow`.
///
/// # Deriving
/// This trait can be automatically derived by SQLx for any struct. The generated implementation
/// will consist of a sequence of calls to [`Row::try_get`] using the name from each
/// struct field.
///
/// ```rust,ignore
/// #[derive(sqlx::FromRow)]
/// struct User {
///     id: i32,
///     name: String,
/// }
/// ```
///
/// [`query_as`]: crate::query_as
/// [`Row::try_get`]: crate::row::Row::try_get
pub trait FromRow<'c, R>
where
    Self: Sized,
    R: Row<'c>,
{
    #[allow(missing_docs)]
    fn from_row(row: &R) -> crate::Result<Self>;
}

// Macros to help unify the internal implementations as a good chunk
// is very similar

#[allow(unused_macros)]
macro_rules! impl_from_row_for_tuple {
    ($db:ident, $r:ident; $( ($idx:tt) -> $T:ident );+;) => {
        impl<'c, $($T,)+> crate::row::FromRow<'c, $r<'c>> for ($($T,)+)
        where
            $($T: 'c,)+
            $($T: crate::types::Type<$db>,)+
            $($T: crate::decode::Decode<'c, $db>,)+
        {
            #[inline]
            fn from_row(row: &$r<'c>) -> crate::Result<Self> {
                use crate::row::Row;

                Ok(($(row.try_get($idx as usize)?,)+))
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_from_row_for_tuples {
    ($db:ident, $r:ident) => {
        impl_from_row_for_tuple!($db, $r;
            (0) -> T1;
        );

        impl_from_row_for_tuple!($db, $r;
            (0) -> T1;
            (1) -> T2;
        );

        impl_from_row_for_tuple!($db, $r;
            (0) -> T1;
            (1) -> T2;
            (2) -> T3;
        );

        impl_from_row_for_tuple!($db, $r;
            (0) -> T1;
            (1) -> T2;
            (2) -> T3;
            (3) -> T4;
        );

        impl_from_row_for_tuple!($db, $r;
            (0) -> T1;
            (1) -> T2;
            (2) -> T3;
            (3) -> T4;
            (4) -> T5;
        );

        impl_from_row_for_tuple!($db, $r;
            (0) -> T1;
            (1) -> T2;
            (2) -> T3;
            (3) -> T4;
            (4) -> T5;
            (5) -> T6;
        );

        impl_from_row_for_tuple!($db, $r;
            (0) -> T1;
            (1) -> T2;
            (2) -> T3;
            (3) -> T4;
            (4) -> T5;
            (5) -> T6;
            (6) -> T7;
        );

        impl_from_row_for_tuple!($db, $r;
            (0) -> T1;
            (1) -> T2;
            (2) -> T3;
            (3) -> T4;
            (4) -> T5;
            (5) -> T6;
            (6) -> T7;
            (7) -> T8;
        );

        impl_from_row_for_tuple!($db, $r;
            (0) -> T1;
            (1) -> T2;
            (2) -> T3;
            (3) -> T4;
            (4) -> T5;
            (5) -> T6;
            (6) -> T7;
            (7) -> T8;
            (8) -> T9;
        );
    };
}

#[allow(unused_macros)]
macro_rules! impl_map_row_for_row {
    ($DB:ident, $R:ident) => {
        impl<O: Unpin, F> crate::query::MapRow<$DB> for F
        where
            F: for<'c> FnMut($R<'c>) -> O,
        {
            type Output = O;

            fn map_row(&mut self, row: $R) -> O {
                (self)(row)
            }
        }

        impl<O: Unpin, F> crate::query::TryMapRow<$DB> for F
        where
            F: for<'c> FnMut($R<'c>) -> crate::Result<O>,
        {
            type Output = O;

            fn try_map_row(&mut self, row: $R) -> crate::Result<O> {
                (self)(row)
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_from_row_for_row {
    ($R:ident) => {
        impl<'c> crate::row::FromRow<'c, $R<'c>> for $R<'c> {
            #[inline]
            fn from_row(row: $R<'c>) -> crate::Result<Self> {
                Ok(row)
            }
        }
    };
}
