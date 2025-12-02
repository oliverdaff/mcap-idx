use binrw::binrw;

/// MCAP file magic:
/// 0x89 'M' 'C' 'A' 'P' '0' '\r' '\n'
#[binrw]
#[br(little, magic = b"\x89MCAP0\r\n")]
pub struct Magic {
    #[br(calc = b'0')] // The major version literal (0x30)
    pub major_version: u8,
}
