use crate::io::PgBufMutExt;
use crate::types::Oid;
use rbdc::io::Encode;

const CLOSE_PORTAL: u8 = b'P';
const CLOSE_STATEMENT: u8 = b'S';

#[derive(Debug)]
#[allow(dead_code)]
pub enum Close {
    Statement(Oid),
    Portal(Oid),
}

impl Encode<'_> for Close {
    fn encode_with(&self, buf: &mut Vec<u8>, _: ()) {
        // 15 bytes for 1-digit statement/portal IDs
        buf.reserve(20);
        buf.push(b'C');

        buf.put_length_prefixed(|buf| match self {
            Close::Statement(id) => {
                buf.push(CLOSE_STATEMENT);
                buf.put_statement_name(*id);
            }

            Close::Portal(id) => {
                buf.push(CLOSE_PORTAL);
                buf.put_portal_name(Some(*id));
            }
        })
    }
}
