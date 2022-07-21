use crate::protocol::text::{ColumnFlags, ColumnType};
use crate::protocol::Capabilities;
use crate::stmt::MySqlArguments;
use bytes::BufMut;
use rbdc::io::Encode;

// https://dev.mysql.com/doc/internals/en/com-stmt-execute.html

///  Execute
///  payload:
///     1              [17] COM_STMT_EXECUTE
///     4              stmt-id
///     1              flags
///     4              iteration-count
///       if num-params > 0:
///     n              NULL-bitmap, length: (num-params+7)/8
///     1              new-params-bound-flag
///       if new-params-bound-flag == 1:
///     n              type of each parameter, length: num-params * 2
///     n              value of each parameter
///
///   example:
///     12 00 00 00 17 01 00 00    00 00 01 00 00 00 00 01    ................
///     0f 00 03 66 6f 6f                                     ...foo
#[derive(Debug)]
pub struct Execute<'q> {
    pub statement_id: u32,
    pub arguments: &'q MySqlArguments,
}

impl<'q> Encode<'_, Capabilities> for Execute<'q> {
    fn encode_with(&self, buf: &mut Vec<u8>, _: Capabilities) {
        buf.push(0x17); // COM_STMT_EXECUTE
        buf.extend(&self.statement_id.to_le_bytes());
        buf.push(0); // NO_CURSOR
        buf.extend(&1_u32.to_le_bytes()); // iterations (always 1): int<4>

        if !self.arguments.types.is_empty() {
            buf.extend(&*self.arguments.null_bitmap);
            buf.push(1); // send type to server

            for ty in &self.arguments.types {
                buf.push(ty.r#type as u8);

                // buf.push(if ty.flags.contains(ColumnFlags::UNSIGNED) {
                //     0x80
                // } else {
                //     0
                // });

                //ColumnType::Tiny if is_unsigned => "TINYINT UNSIGNED",
                //         ColumnType::Short if is_unsigned => "SMALLINT UNSIGNED",
                //         ColumnType::Long if is_unsigned => "INT UNSIGNED",
                //         ColumnType::Int24 if is_unsigned => "MEDIUMINT UNSIGNED",
                //         ColumnType::LongLong if is_unsigned => "BIGINT UNSIGNED",

                buf.push(if false { 0x80 } else { 0 });
            }

            buf.extend(&*self.arguments.values);
        }
    }
}
