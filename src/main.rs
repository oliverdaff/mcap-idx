use anyhow::{Result, bail};
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;

use tokio::fs::File;
use tokio::io::BufReader;

pub struct McapStream<R: AsyncRead + Unpin> {
    reader: R,
    offset: u64,
}

impl<R: AsyncRead + Unpin> McapStream<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, offset: 0 }
    }

    pub async fn next_record(&mut self) -> Result<Option<Record>> {
        let opcode = self.reader.read_u8().await?;

        // Detect footer magic
        if opcode == 0x89 {
            let mut rest = [0u8; 7];
            self.reader.read_exact(&mut rest).await?;
            const MAGIC: [u8; 7] = [b'M', b'C', b'A', b'P', b'0', 0x0D, 0x0A];

            if rest == MAGIC {
                // End of file
                return Ok(None);
            } else {
                bail!("Unexpected 0x89 in stream — not valid MCAP record or footer");
            }
        }

        let record_start = self.offset;
        self.offset += 1;

        // Read 8 byte le length
        let body_len = self.reader.read_u64_le().await?;
        self.offset += 8;

        Ok(Some(Record {
            opcode,
            body_len,
            offset: record_start,
        }))
    }

    pub async fn skip_body(&mut self, rec: &Record) -> Result<()> {
        let mut remaining = rec.body_len;
        let mut buf = [0u8; 64 * 1024];

        while remaining > 0 {
            let take = remaining.min(buf.len() as u64) as usize;
            let n = self.reader.read(&mut buf[..take]).await?;

            if n == 0 {
                bail!("Unexpected EOF while skipping MCAP record body");
            }
            remaining -= n as u64;
            self.offset += n as u64;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub opcode: u8,
    pub body_len: u64,
    pub offset: u64,
}

pub async fn index_mcap<R: AsyncRead + Unpin>(mut m: McapStream<R>) -> Result<()> {
    while let Some(rec) = m.next_record().await? {
        match rec.opcode {
            0x05 => {
                // Message
                m.skip_body(&rec).await?;
            }
            0x06 => {
                // Chunk
                m.skip_body(&rec).await?;
            }
            _ => {
                m.skip_body(&rec).await?;
            }
        }
    }

    Ok(())
}

pub async fn read_magic<R: AsyncRead + Unpin>(reader: &mut R) -> Result<()> {
    let mut magic = [0u8; 8];
    reader.read_exact(&mut magic).await?;

    const MAGIC: [u8; 8] = [0x89, b'M', b'C', b'A', b'P', b'0', 0x0D, 0x0A];

    if magic != MAGIC {
        bail!("Invalid MCAP magic header: got {:02x?}", magic);
    }

    Ok(())
}

#[derive(Debug)]
pub struct McapFileHeader {
    pub profile: String,
    pub library: String,
    // more fields exist but these two are required
}

pub async fn parse_header_record<R: AsyncRead + Unpin>(
    reader: &mut R,
    body_len: u64,
) -> Result<McapFileHeader> {
    async fn read_string<R: AsyncRead + Unpin>(
        reader: &mut R,
        remaining: &mut u64,
    ) -> Result<String> {
        let len = reader.read_u32_le().await? as usize;
        *remaining -= 4;

        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf).await?;
        *remaining -= len as u64;

        Ok(String::from_utf8(buf)?)
    }

    let mut remaining = body_len;

    // Required MCAP header fields
    let profile = read_string(reader, &mut remaining).await?;
    let library = read_string(reader, &mut remaining).await?;

    // Skip any extra unknown key/value fields
    if remaining > 0 {
        let mut tmp = vec![0u8; remaining as usize];
        reader.read_exact(&mut tmp).await?;
    }

    Ok(McapFileHeader { profile, library })
}

#[tokio::main]
async fn main() -> Result<()> {
    let path = "./data/nissan_zala_50_zeg_4_0.mcap";

    let file = File::open(path).await?;
    let mut reader = BufReader::new(file);

    //
    // 1. Validate magic
    read_magic(&mut reader).await?;
    println!("Magic OK");

    // 2. Build stream AFTER consuming magic
    let mut stream = McapStream::new(reader);

    // 3. First record must be a header
    let header_rec = stream
        .next_record()
        .await?
        .expect("MCAP missing header record");

    if header_rec.opcode != 0x01 {
        panic!(
            "Expected header record (opcode 0x01), got: {:x}",
            header_rec.opcode
        );
    }

    // 4. Parse the header record body
    let file_header = parse_header_record(&mut stream.reader, header_rec.body_len).await?;
    stream.offset += header_rec.body_len; // manual bookkeeping

    println!("Header: {:?}", file_header);

    // 5. Resume normal record loop
    while let Some(rec) = stream.next_record().await? {
        println!(
            "Record at offset {}: {:?} (body_len = {})",
            rec.offset, rec.opcode, rec.body_len,
        );

        stream.skip_body(&rec).await?;
    }

    println!("Done.");
    Ok(())
}
