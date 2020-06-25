use crate::database::Database;
use bitflags::_core::fmt::Debug;
use crate::Result;

/// Associate [`Database`] with a `RawValue` of a generic lifetime.
///
/// ---
///
/// The upcoming Rust feature, [Generic Associated Types], should obviate
/// the need for this trait.
///
/// [Generic Associated Types]: https://www.google.com/search?q=generic+associated+types+rust&oq=generic+associated+types+rust&aqs=chrome..69i57j0l5.3327j0j7&sourceid=chrome&ie=UTF-8
pub trait HasRawValue<'c> {
    type Database: Database;

    /// The Rust type used to hold a not-yet-decoded value that has just been
    /// received from the database.
    type RawValue: RawValue<'c, Database = Self::Database>+Debug;
}

pub trait RawValue<'c> {
    type Database: Database;

    fn type_info(&self) -> Option<<Self::Database as Database>::TypeInfo>;

    /// to an json value
    fn try_to_json(&self) -> Result<serde_json::Value>;
}
