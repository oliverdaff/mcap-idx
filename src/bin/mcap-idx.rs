use std::io::Cursor;

use anyhow::Result;
use clap::Parser;
use tokio::fs::File;
use tokio::io::BufReader;

use binrw::BinRead;
use tokio::io::AsyncReadExt;

/// Simple streaming MCAP parser / indexer.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the MCAP file to parse.
    path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.path).await?;
    let mut reader = BufReader::new(file);

    // MCAP magic is always 8 bytes.
    let mut magic = [0u8; 8];
    reader.read_exact(&mut magic).await?;

    let mut cursor = Cursor::new(magic);

    // Validate the magic bytes.
    mcap_idx::magic::Magic::read(&mut cursor)?;

    println!("Done.");
    Ok(())
}
