use binrw::binread;

use crate::types::McapString;

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct HeaderRecord {
    pub opcode: u8,

    #[br(assert(opcode == 0x01))]
    pub content_len: u64,

    #[br(count = content_len)]
    pub body: Vec<u8>,
}

#[binread]
#[br(little)]
pub struct Header {
    profile: McapString,
    library: McapString,
}
