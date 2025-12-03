use binrw::{BinRead, BinResult, Endian};
use std::{
    fmt,
    io::{Read, Seek},
};

use binrw::binread;

#[binread]
#[derive(Debug)]
pub struct McapString(#[br(parse_with = read_mcap_string)] pub String);

fn read_mcap_string<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    _args: (),
) -> BinResult<String> {
    // Read the length prefix with the given endian
    let len = u32::read_options(reader, endian, ())? as usize;

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;

    String::from_utf8(buf).map_err(|_| McapStringError::InvalidUtf8.into())
}

#[derive(Debug)]
pub enum McapStringError {
    InvalidUtf8,
}

impl std::fmt::Display for McapStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            McapStringError::InvalidUtf8 => write!(f, "MCAP string contains invalid UTF-8"),
        }
    }
}

impl From<McapStringError> for binrw::Error {
    fn from(err: McapStringError) -> Self {
        binrw::Error::Custom {
            pos: 0, // binrw fills in real offset later
            err: Box::new(err),
        }
    }
}
