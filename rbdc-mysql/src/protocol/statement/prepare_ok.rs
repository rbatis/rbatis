use bytes::{Buf, Bytes};

use crate::protocol::Capabilities;
use rbdc::io::Decode;
use rbdc::{err_protocol, Error};

// https://dev.mysql.com/doc/internals/en/com-stmt-prepare-response.html#packet-COM_STMT_PREPARE_OK

#[derive(Debug)]
pub struct PrepareOk {
    pub statement_id: u32,
    pub columns: u16,
    pub params: u16,
    #[allow(unused)]
    pub warnings: u16,
}

impl Decode<'_, Capabilities> for PrepareOk {
    fn decode_with(mut buf: Bytes, _: Capabilities) -> Result<Self, Error> {
        let status = buf.get_u8();
        if status != 0x00 {
            return Err(err_protocol!(
                "expected 0x00 (COM_STMT_PREPARE_OK) but found 0x{:02x}",
                status
            ));
        }

        let statement_id = buf.get_u32_le();
        let columns = buf.get_u16_le();
        let params = buf.get_u16_le();

        buf.advance(1); // reserved: string<1>

        let warnings = buf.get_u16_le();

        Ok(Self {
            statement_id,
            columns,
            params,
            warnings,
        })
    }
}
