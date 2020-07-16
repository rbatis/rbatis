use crate::io::BufMut;
use crate::mysql::protocol::{Capabilities, Encode};

// https://dev.mysql.com/doc/internals/en/com-quit.html

#[derive(Debug)]
pub struct Quit;

impl Encode for Quit {
    fn encode(&self, buf: &mut Vec<u8>, _: Capabilities) {
        buf.push(0x01); // COM_QUIT
    }
}
