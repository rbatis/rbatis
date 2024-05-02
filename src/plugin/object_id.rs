//! ObjectId
use hex::FromHexError;
use rand::{thread_rng, Rng};
use std::sync::OnceLock;
use std::{
    error, fmt, result,
    str::FromStr,
    sync::atomic::{AtomicUsize, Ordering},
    time::SystemTime,
};

const TIMESTAMP_SIZE: usize = 4;
const PROCESS_ID_SIZE: usize = 5;
const COUNTER_SIZE: usize = 3;

const TIMESTAMP_OFFSET: usize = 0;
const PROCESS_ID_OFFSET: usize = TIMESTAMP_OFFSET + TIMESTAMP_SIZE;
const COUNTER_OFFSET: usize = PROCESS_ID_OFFSET + PROCESS_ID_SIZE;

const MAX_U24: usize = 0xFF_FFFF;

pub static OID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Errors that can occur during OID construction and generation.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// An invalid argument was passed in.
    ArgumentError { message: String },

    /// An error occured parsing a hex string.
    FromHexError(FromHexError),
}

impl From<FromHexError> for Error {
    fn from(err: FromHexError) -> Error {
        Error::FromHexError(err)
    }
}

/// Alias for Result<T, oid::Error>.
pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ArgumentError { ref message } => message.fmt(fmt),
            Error::FromHexError(ref inner) => inner.fmt(fmt),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::ArgumentError { .. } => None,
            Error::FromHexError(ref inner) => Some(inner),
        }
    }
}

/// A wrapper around raw 12-byte ObjectId representations.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub struct ObjectId {
    pub id: [u8; 12],
}

impl Default for ObjectId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for ObjectId {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::with_string(s)
    }
}

impl ObjectId {
    /// Generates a new ObjectID, represented in bytes.
    /// See the [docs](http://docs.mongodb.org/manual/reference/object-id/)
    /// for more information.
    pub fn new() -> ObjectId {
        let timestamp = ObjectId::gen_timestamp();
        let process_id = ObjectId::gen_process_id();
        let counter = ObjectId::gen_count();

        let mut buf: [u8; 12] = [0; 12];
        buf[TIMESTAMP_OFFSET..(TIMESTAMP_SIZE + TIMESTAMP_OFFSET)]
            .clone_from_slice(&timestamp[..TIMESTAMP_SIZE]);
        buf[PROCESS_ID_OFFSET..(PROCESS_ID_SIZE + PROCESS_ID_OFFSET)]
            .clone_from_slice(&process_id[..PROCESS_ID_SIZE]);
        buf[COUNTER_OFFSET..(COUNTER_SIZE + COUNTER_OFFSET)]
            .clone_from_slice(&counter[..COUNTER_SIZE]);

        ObjectId::with_bytes(buf)
    }

    /// Constructs a new ObjectId wrapper around the raw byte representation.
    pub fn with_bytes(bytes: [u8; 12]) -> ObjectId {
        ObjectId { id: bytes }
    }

    /// Creates an ObjectID using a 12-byte (24-char) hexadecimal string.
    pub fn with_string(s: &str) -> Result<ObjectId> {
        let bytes: Vec<u8> = hex::decode(s.as_bytes())?;
        if bytes.len() != 12 {
            Err(Error::ArgumentError {
                message: "Provided string must be a 12-byte hexadecimal string.".to_owned(),
            })
        } else {
            let mut byte_array: [u8; 12] = [0; 12];
            byte_array[..].copy_from_slice(&bytes[..]);
            Ok(ObjectId::with_bytes(byte_array))
        }
    }

    /// Returns the raw byte representation of an ObjectId.
    pub fn bytes(&self) -> [u8; 12] {
        self.id
    }

    /// Convert the objectId to hex representation.
    pub fn to_hex(&self) -> String {
        hex::encode(self.id)
    }

    /// Returns u64
    pub fn u128(&self) -> u128 {
        //4-TIMESTAMP 10
        let b: [u8; 4] = [self.id[0], self.id[1], self.id[2], self.id[3]];
        let u4 = u32::from_be_bytes(b.into());

        //5-PROCESS_ID_OFFSET 10
        let b: [u8; 8] = [
            0u8, 0u8, 0u8, self.id[4], self.id[5], self.id[6], self.id[7], self.id[8],
        ];
        let u5 = u64::from_be_bytes(b.into());

        //3-COUNTER_OFFSET  8
        let b: [u8; 4] = [0u8, self.id[9], self.id[10], self.id[11]];
        let u3 = u32::from_be_bytes(b.into());
        let v = u5 as u128 * 1000000000000000000 + u4 as u128 * 100000000 + u3 as u128;
        v
    }

    pub fn with_u128(arg: u128) -> Self {
        let u5 = (arg / 1000000000000000000) as u64;
        let u4 = ((arg - u5 as u128 * 1000000000000000000) / 100000000) as u32;
        let u3 = (arg - u5 as u128 * 1000000000000000000 - u4 as u128 * 100000000) as u32;

        let u5 = u5.to_be_bytes();
        let u3 = u3.to_be_bytes();
        let u4 = u4.to_be_bytes();
        //timestamp(4), PROCESS_ID(5),COUNTER(3)
        ObjectId {
            id: [
                u4[0], u4[1], u4[2], u4[3], u5[3], u5[4], u5[5], u5[6], u5[7], u3[1], u3[2], u3[3],
            ],
        }
    }

    // Generates a new timestamp representing the current seconds since epoch.
    // Represented in Big Endian.
    fn gen_timestamp() -> [u8; 4] {
        let timestamp: u32 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system clock is before 1970")
            .as_secs()
            .try_into()
            .unwrap(); // will succeed until 2106 since timestamp is unsigned
        timestamp.to_be_bytes()
    }

    // Generate a random 5-byte array.
    fn gen_process_id() -> [u8; 5] {
        pub static BUF: OnceLock<[u8; 5]> = OnceLock::new();
        let r = BUF.get_or_init(|| {
            let rng = thread_rng().gen_range(0..MAX_U24) as u32;
            let mut buf: [u8; 5] = [0; 5];
            buf[0..4].copy_from_slice(&rng.to_be_bytes());
            buf
        });
        *r
    }

    // Gets an incremental 3-byte count.
    // Represented in Big Endian.
    fn gen_count() -> [u8; 3] {
        let u_counter = OID_COUNTER.fetch_add(1, Ordering::SeqCst);

        // Mod result instead of OID_COUNTER to prevent threading issues.
        let u = u_counter % (MAX_U24 + 1);

        // Convert usize to writable u64, then extract the first three bytes.
        let u_int = u as u64;

        let buf = u_int.to_be_bytes();
        let buf_u24: [u8; 3] = [buf[5], buf[6], buf[7]];
        buf_u24
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("ObjectId({})", self.to_hex()))
    }
}

#[cfg(test)]
mod test {
    use crate::object_id::ObjectId;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_new() {
        println!("objectId:{}", ObjectId::new().to_string());
        println!("objectId:{}", ObjectId::new().to_string());
        println!("objectId:{}", ObjectId::new().to_string());
        println!("objectId:{}", ObjectId::new().to_string());
    }

    #[test]
    fn test_new_u128() {
        println!("objectId:{}", ObjectId::new().u128());
        println!("objectId:{}", ObjectId::new().u128());
        println!("objectId:{}", ObjectId::new().u128());
        println!("objectId:{}", ObjectId::new().u128());
    }

    #[test]
    fn test_display() {
        let id = super::ObjectId::with_string("53e37d08776f724e42000000").unwrap();

        assert_eq!(format!("{}", id), "53e37d08776f724e42000000")
    }

    #[test]
    fn test_debug() {
        let id = super::ObjectId::with_string("53e37d08776f724e42000000").unwrap();

        assert_eq!(format!("{:?}", id), "ObjectId(53e37d08776f724e42000000)")
    }

    #[test]
    fn test_u128() {
        let oid = ObjectId::new();
        println!("oid={}", oid);
        println!("oid-u128={}", oid.u128());
        println!("oid-from={}", ObjectId::with_u128(oid.u128()));
        assert_eq!(oid, ObjectId::with_u128(oid.u128()));
    }

    #[test]
    fn test_u128_parse() {
        for _ in 0..1000 {
            sleep(Duration::from_nanos(500));
            let oid = ObjectId::new();
            assert_eq!(oid, ObjectId::with_u128(oid.u128()));
        }
    }
}
