use anyhow::{Result, bail};
use tokio::io::{AsyncRead, AsyncReadExt};

use super::opcodes::OpCode;

pub struct McapRecordHeader {
    pub opcode: OpCode,
    pub body_len: u64,
    pub offset: u64,
}

pub struct McapStream<R: AsyncRead + Unpin> {
    reader: R,
    pub offset: u64,
}

impl<R: AsyncRead + Unpin> McapStream<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, offset: 0 }
    }

    pub async fn next(&mut self) -> Result<Option<McapRecordHeader>> {
        // Try reading opcode
        let opcode = match self.reader.read_u8().await {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        // Detect MCAP footer magic
        if opcode == 0x89 {
            let mut rest = [0u8; 7];
            self.reader.read_exact(&mut rest).await?;

            const MAGIC: [u8; 7] = [b'M', b'C', b'A', b'P', b'0', 0x0D, 0x0A];
            if rest == MAGIC {
                return Ok(None);
            } else {
                bail!("unexpected 0x89 – not footer magic");
            }
        }

        let record_start = self.offset;
        self.offset += 1;

        let body_len = self.reader.read_u64_le().await?;
        self.offset += 8;

        Ok(Some(McapRecordHeader {
            opcode: OpCode::from(opcode),
            body_len,
            offset: record_start,
        }))
    }

    pub async fn skip_body(&mut self, rec: &McapRecordHeader) -> Result<()> {
        let mut remaining = rec.body_len;
        let mut buf = [0u8; 65536];

        while remaining > 0 {
            let to_read = remaining.min(buf.len() as u64) as usize;
            let n = self.reader.read(&mut buf[..to_read]).await?;
            if n == 0 {
                bail!("unexpected EOF skipping record body");
            }
            remaining -= n as u64;
            self.offset += n as u64;
        }

        Ok(())
    }

    pub fn reader_mut(&mut self) -> &mut R {
        &mut self.reader
    }
}
