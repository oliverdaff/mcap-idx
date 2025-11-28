use anyhow::Result;
use tokio::fs::File;
use tokio::io::BufReader;

use mcap_idx::header::parse_header;
use mcap_idx::stream::McapStream;

#[tokio::main]
async fn main() -> Result<()> {
    let file = File::open("./data/nissan_zala_50_zeg_4_0.mcap").await?;
    let mut reader = BufReader::new(file);

    // Read magic
    mcap_idx::magic::read_magic(&mut reader).await?;

    // Set up parser
    let mut stream = McapStream::new(reader);

    // Read header record
    let header_rec = stream.next().await?.expect("missing header record");
    let header = parse_header(stream.reader_mut(), header_rec.body_len).await?;
    println!("Header: {:?}", header);

    // Walk records
    while let Some(rec) = stream.next().await? {
        println!(
            "Record at offset {}: {:?} (body_len = {})",
            rec.offset, rec.opcode, rec.body_len
        );

        stream.skip_body(&rec).await?;
    }

    println!("Done.");
    Ok(())
}
