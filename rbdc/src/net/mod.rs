mod socket;
mod tls;

pub use socket::Socket;
pub use tls::{CertificateInput, MaybeTlsStream};

type PollReadBuf<'a> = crate::rt::ReadBuf<'a>;

type PollReadOut = ();
