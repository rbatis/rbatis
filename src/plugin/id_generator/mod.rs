pub mod object_id;
pub mod snowflake;

// Re-export IdGenerator trait
pub use snowflake::IdGenerator;

// Re-export Snowflake and related items
pub use snowflake::{new_snowflake_id, Snowflake, SNOWFLAKE};

// Re-export ObjectId and related items (but not Error to avoid conflicts)
pub use object_id::{ObjectId, ObjectIdGenerator, OID_COUNTER};
