//! Creates a new BAM file.
//!
//! This writes a SAM header, reference sequences, and one unmapped record to stdout.
//!
//! Verify the output by piping to `samtools view -h --no-PG`.

use noodles_bam as bam;
use noodles_sam::{
    self as sam,
    header::{Program, ReferenceSequence},
};
use tokio::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut writer = bam::AsyncWriter::new(stdout);

    let header = sam::Header::builder()
        .set_header(Default::default())
        .add_reference_sequence(ReferenceSequence::new("sq0", 8)?)
        .add_program(Program::new("noodles-bam"))
        .add_comment("an example BAM written by noodles-bam")
        .build();

    writer.write_header(&header).await?;
    writer
        .write_reference_sequences(header.reference_sequences())
        .await?;

    let record = bam::Record::default();
    writer.write_record(&record).await?;

    writer.shutdown().await?;

    Ok(())
}
