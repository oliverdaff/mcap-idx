use anyhow::{Result, bail};
use tokio::io::{AsyncRead, AsyncReadExt};

/// MCAP v0 magic header (= major version 0)
/// 0x89, 'M', 'C', 'A', 'P', '0', '\r', '\n'
pub const MCAP_MAGIC: [u8; 8] = [0x89, b'M', b'C', b'A', b'P', b'0', 0x0D, 0x0A];

/// Validate the MCAP front-of-file magic header.
/// Consumes the first 8 bytes from the reader.
pub async fn read_magic<R: AsyncRead + Unpin>(reader: &mut R) -> Result<()> {
    let mut magic = [0u8; 8];
    reader.read_exact(&mut magic).await?;

    if magic != MCAP_MAGIC {
        bail!("Invalid MCAP magic header: got {:02x?}", magic);
    }

    Ok(())
}
