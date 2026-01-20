pub mod snowflake;
pub mod object_id;

// Re-export IdGenerator trait
pub use snowflake::IdGenerator;

// Re-export Snowflake and related items
pub use snowflake::{Snowflake, SNOWFLAKE, new_snowflake_id};

// Re-export ObjectId and related items (but not Error to avoid conflicts)
pub use object_id::{ObjectId, ObjectIdGenerator, OID_COUNTER};

