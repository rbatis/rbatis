//! Distributed id generators.
//!
//! Both generators implement [`IdGenerator`] and can be plugged into
//! [`RBatis::task_id_generator`](crate::RBatis::task_id_generator), which is what
//! provides the `task_id` seen by interceptors:
//!
//! * [`Snowflake`] - 64 bit ids made of timestamp + machine id + node id + counter
//! * [`ObjectId`] - MongoDB style 12 byte ids (timestamp + process id + counter)
//!
//! ```rust
//! use rbatis::id_generator::{ObjectId, Snowflake};
//!
//! let snowflake = Snowflake::new(1, 2, 0);
//! assert!(snowflake.generate_id() > 0);
//!
//! let oid = ObjectId::new();
//! println!("{}", oid.to_hex());
//! ```

/// MongoDB style `ObjectId` generator, see [`ObjectId`]
pub mod object_id;
/// Snowflake id generator, see [`Snowflake`]
pub mod snowflake;

// Re-export IdGenerator trait
pub use snowflake::IdGenerator;

// Re-export Snowflake and related items
pub use snowflake::{new_snowflake_id, Snowflake, SNOWFLAKE};

// Re-export ObjectId and related items (but not Error to avoid conflicts)
pub use object_id::{ObjectId, ObjectIdGenerator, OID_COUNTER};
