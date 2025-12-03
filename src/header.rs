use binrw::binread;

use crate::types::McapString;

#[binread]
#[br(little)]
pub struct Header {
    profile: McapString,
    library: McapString,
}
