use std::mem;

/// Encode a single value to be sent to the database.
pub trait Encode{

    /// Writes the value of `self` into `buf` without moving `self`.
    ///
    /// Where possible, make use of `encode` instead as it can take advantage of re-using
    /// memory.
    #[must_use]
    fn encode(&self, buf: &mut rbson::Bson);



    #[inline]
    fn size_hint(&self) -> usize {
        mem::size_of_val(self)
    }
}