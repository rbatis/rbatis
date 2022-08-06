use crate::protocol::Capabilities;
use rbdc::io::Encode;

// https://dev.mysql.com/doc/internals/en/com-quit.html

#[derive(Debug)]
pub struct Quit;

impl Encode<'_, Capabilities> for Quit {
    fn encode_with(&self, buf: &mut Vec<u8>, _: Capabilities) {
        buf.push(0x01); // COM_QUIT
    }
}
