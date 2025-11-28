#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Header = 0x01,
    Footer = 0x02,
    Schema = 0x03,
    Channel = 0x04,
    Message = 0x05,
    Chunk = 0x06,
    ChunkIndex = 0x07,
    Attachment = 0x08,
    Statistics = 0x09,
    Metadata = 0x0A,
    MetadataIndex = 0x0B,
    SummaryOffset = 0x0C,
    Summary = 0x0D,
    Unknown = 0xFF,
}

impl From<u8> for OpCode {
    fn from(v: u8) -> Self {
        use OpCode::*;
        match v {
            0x01 => Header,
            0x02 => Footer,
            0x03 => Schema,
            0x04 => Channel,
            0x05 => Message,
            0x06 => Chunk,
            0x07 => ChunkIndex,
            0x08 => Attachment,
            0x09 => Statistics,
            0x0A => Metadata,
            0x0B => MetadataIndex,
            0x0C => SummaryOffset,
            0x0D => Summary,
            _ => Unknown,
        }
    }
}
