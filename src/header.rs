use anyhow::Result;
use tokio::io::{AsyncRead, AsyncReadExt};

#[derive(Debug)]
pub struct McapFileHeader {
    pub profile: String,
    pub library: String,
}

async fn read_string<R: AsyncRead + Unpin>(reader: &mut R, remaining: &mut u64) -> Result<String> {
    let len = reader.read_u32_le().await? as usize;
    *remaining -= 4;

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await?;
    *remaining -= len as u64;

    Ok(String::from_utf8(buf)?)
}

pub async fn parse_header<R: AsyncRead + Unpin>(
    reader: &mut R,
    body_len: u64,
) -> Result<McapFileHeader> {
    let mut remaining = body_len;

    let profile = read_string(reader, &mut remaining).await?;
    let library = read_string(reader, &mut remaining).await?;

    // Skip any vendor fields
    if remaining > 0 {
        let mut tmp = vec![0u8; remaining as usize];
        reader.read_exact(&mut tmp).await?;
    }

    Ok(McapFileHeader { profile, library })
}
