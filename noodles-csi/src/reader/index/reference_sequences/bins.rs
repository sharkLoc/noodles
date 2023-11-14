mod chunks;

use std::{
    error, fmt,
    io::{self, Read},
    num,
};

use byteorder::{LittleEndian, ReadBytesExt};
use indexmap::IndexMap;
use noodles_bgzf as bgzf;

use self::chunks::read_chunks;
use super::read_metadata;
use crate::index::reference_sequence::{Bin, Metadata};

/// An error returned when CSI reference sequence bins fail to be read.
#[derive(Debug)]
pub enum ReadError {
    /// An I/O error.
    Io(io::Error),
    /// The bin count is invalid.
    InvalidBinCount(num::TryFromIntError),
    /// A bin ID is invalid.
    InvalidBinId(num::TryFromIntError),
    /// A bin is duplicated.
    DuplicateBin(usize),
    /// Bin chunks are invalid.
    InvalidChunks(chunks::ReadError),
}

impl error::Error for ReadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ReadError::Io(e) => Some(e),
            ReadError::InvalidBinCount(e) => Some(e),
            ReadError::InvalidBinId(e) => Some(e),
            ReadError::DuplicateBin(_) => None,
            ReadError::InvalidChunks(e) => Some(e),
        }
    }
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadError::Io(_) => write!(f, "I/O error"),
            ReadError::InvalidBinCount(_) => write!(f, "invalid bin count"),
            ReadError::InvalidBinId(_) => write!(f, "invalid bin ID"),
            ReadError::DuplicateBin(id) => write!(f, "duplicate bin: {id}"),
            ReadError::InvalidChunks(_) => write!(f, "invalid chunks"),
        }
    }
}

impl From<io::Error> for ReadError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

pub(super) fn read_bins<R>(
    reader: &mut R,
    depth: u8,
) -> Result<(IndexMap<usize, Bin>, Option<Metadata>), ReadError>
where
    R: Read,
{
    let n_bin = reader
        .read_i32::<LittleEndian>()
        .map_err(ReadError::Io)
        .and_then(|n| usize::try_from(n).map_err(ReadError::InvalidBinCount))?;

    let mut bins = IndexMap::with_capacity(n_bin);

    let metadata_id = Bin::metadata_id(depth);
    let mut metadata = None;

    for _ in 0..n_bin {
        let id = reader
            .read_u32::<LittleEndian>()
            .map_err(ReadError::Io)
            .and_then(|n| usize::try_from(n).map_err(ReadError::InvalidBinId))?;

        let loffset = reader
            .read_u64::<LittleEndian>()
            .map(bgzf::VirtualPosition::from)?;

        if id == metadata_id {
            let m = read_metadata(reader)?;

            if metadata.replace(m).is_some() {
                return Err(ReadError::DuplicateBin(id));
            }
        } else {
            let chunks = read_chunks(reader).map_err(ReadError::InvalidChunks)?;
            let bin = Bin::new(loffset, chunks);

            if bins.insert(id, bin).is_some() {
                return Err(ReadError::DuplicateBin(id));
            }
        }
    }

    Ok((bins, metadata))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_bins() -> Result<(), ReadError> {
        const DEPTH: u8 = 5;

        let data = [
            0x00, 0x00, 0x00, 0x00, // n_bin = 0
        ];
        let mut reader = &data[..];
        let (actual_bins, actual_metadata) = read_bins(&mut reader, DEPTH)?;
        assert!(actual_bins.is_empty());
        assert!(actual_metadata.is_none());

        let data = [
            0x02, 0x00, 0x00, 0x00, // n_bin = 2
            0x00, 0x00, 0x00, 0x00, // bins[0].id = 0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[0].loffset = 0
            0x00, 0x00, 0x00, 0x00, // bins[0].n_chunk = 0
            0x4a, 0x92, 0x00, 0x00, // bins[1].id = 37450
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[1].loffset = 0
            0x02, 0x00, 0x00, 0x00, // bins[1].n_chunk = 2
            0x62, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[1].metadata.ref_beg = 610
            0x3d, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[1].metadata.ref_end = 1597
            0x37, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[1].metadata.n_mapped = 55
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[1].metadata.n_unmapped = 0
        ];
        let mut reader = &data[..];
        let (actual_bins, actual_metadata) = read_bins(&mut reader, DEPTH)?;
        assert_eq!(actual_bins.len(), 1);
        assert!(actual_bins.get(&0).is_some());
        assert!(actual_metadata.is_some());

        let data = [
            0x01, 0x00, 0x00, 0x00, // n_bin = 1
        ];
        let mut reader = &data[..];
        assert!(matches!(
            read_bins(&mut reader, DEPTH),
            Err(ReadError::Io(e)) if e.kind() == io::ErrorKind::UnexpectedEof
        ));

        let data = [
            0x02, 0x00, 0x00, 0x00, // n_bin = 2
            0x00, 0x00, 0x00, 0x00, // bins[0].id = 0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[0].loffset = 0
            0x00, 0x00, 0x00, 0x00, // bins[0].n_chunk = 0
            0x00, 0x00, 0x00, 0x00, // bins[1].id = 0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[1].loffset = 0
            0x00, 0x00, 0x00, 0x00, // bins[1].n_chunk = 0
        ];
        let mut reader = &data[..];
        assert!(matches!(
            read_bins(&mut reader, DEPTH),
            Err(ReadError::DuplicateBin(0))
        ));

        let data = [
            0x02, 0x00, 0x00, 0x00, // n_bin = 2
            0x4a, 0x92, 0x00, 0x00, // bins[0].id = 37450
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[0].loffset = 0
            0x02, 0x00, 0x00, 0x00, // bins[0].n_chunk = 2
            0x62, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[0].metadata.ref_beg = 610
            0x3d, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[0].metadata.ref_end = 1597
            0x37, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[0].metadata.n_mapped = 55
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[0].metadata.n_unmapped = 0
            0x4a, 0x92, 0x00, 0x00, // bins[1].id = 37450
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[1].loffset = 0
            0x02, 0x00, 0x00, 0x00, // bins[1].n_chunk = 2
            0x62, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[1].metadata.ref_beg = 610
            0x3d, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[1].metadata.ref_end = 1597
            0x37, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[1].metadata.n_mapped = 55
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bins[1].metadata.n_unmapped = 0
        ];
        let mut reader = &data[..];
        assert!(matches!(
            read_bins(&mut reader, DEPTH),
            Err(ReadError::DuplicateBin(37450))
        ));

        Ok(())
    }
}
