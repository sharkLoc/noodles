//! BAM record field readers.

pub(crate) mod cigar;
pub mod data;
mod flags;
mod mapping_quality;
mod position;
mod quality_scores;
pub(crate) mod read_name;
mod reference_sequence_id;
pub(crate) mod sequence;
mod template_length;

pub(crate) use self::{
    cigar::get_cigar, data::get_data, flags::get_flags, mapping_quality::get_mapping_quality,
    position::get_position, quality_scores::get_quality_scores, read_name::get_read_name,
    reference_sequence_id::get_reference_sequence_id, sequence::get_sequence,
};

use std::{
    error, fmt,
    io::{self, Read},
    mem,
};

use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Buf;
use noodles_sam::{self as sam, alignment::Record};

use self::template_length::get_template_length;

pub(crate) fn read_record<R>(
    reader: &mut R,
    header: &sam::Header,
    buf: &mut Vec<u8>,
    record: &mut Record,
) -> io::Result<usize>
where
    R: Read,
{
    let block_size = match read_block_size(reader)? {
        0 => return Ok(0),
        n => n,
    };

    buf.resize(block_size, 0);
    reader.read_exact(buf)?;

    let mut src = &buf[..];
    decode_record(&mut src, header, record)?;

    Ok(block_size)
}

pub(super) fn read_block_size<R>(reader: &mut R) -> io::Result<usize>
where
    R: Read,
{
    match reader.read_u32::<LittleEndian>() {
        Ok(n) => usize::try_from(n).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(0),
        Err(e) => Err(e),
    }
}

/// An error when a raw BAM record fails to parse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The reference sequence ID is invalid.
    InvalidReferenceSequenceId(reference_sequence_id::ParseError),
    /// The position is invalid.
    InvalidPosition(position::ParseError),
    /// The mapping quality is invalid.
    InvalidMappingQuality(mapping_quality::ParseError),
    /// The flags are invalid.
    InvalidFlags(flags::ParseError),
    /// The mate reference sequence ID is invalid.
    InvalidMateReferenceSequenceId(reference_sequence_id::ParseError),
    /// The mate position is invalid.
    InvalidMatePosition(position::ParseError),
    /// The template length is invalid.
    InvalidTemplateLength(template_length::ParseError),
    /// The read name is invalid.
    InvalidReadName(read_name::ParseError),
    /// The CIGAR is invalid.
    InvalidCigar(cigar::ParseError),
    /// The sequence is invalid.
    InvalidSequence(sequence::ParseError),
    /// The quality scores are invalid.
    InvalidQualityScores(quality_scores::ParseError),
    /// The data is invalid.
    InvalidData(data::ParseError),
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::InvalidReferenceSequenceId(e) => Some(e),
            Self::InvalidPosition(e) => Some(e),
            Self::InvalidMappingQuality(e) => Some(e),
            Self::InvalidFlags(e) => Some(e),
            Self::InvalidMateReferenceSequenceId(e) => Some(e),
            Self::InvalidMatePosition(e) => Some(e),
            Self::InvalidTemplateLength(e) => Some(e),
            Self::InvalidReadName(e) => Some(e),
            Self::InvalidCigar(e) => Some(e),
            Self::InvalidSequence(e) => Some(e),
            Self::InvalidQualityScores(e) => Some(e),
            Self::InvalidData(e) => Some(e),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidReferenceSequenceId(_) => write!(f, "invalid reference sequence ID"),
            Self::InvalidPosition(_) => write!(f, "invalid position"),
            Self::InvalidMappingQuality(_) => write!(f, "invalid mapping quality"),
            Self::InvalidFlags(_) => write!(f, "invalid flags"),
            Self::InvalidMateReferenceSequenceId(_) => {
                write!(f, "invalid mate reference sequence ID")
            }
            Self::InvalidMatePosition(_) => write!(f, "invalid mate position"),
            Self::InvalidTemplateLength(_) => write!(f, "invalid template length"),
            Self::InvalidReadName(_) => write!(f, "invalid read name"),
            Self::InvalidCigar(_) => write!(f, "invalid CIGAR"),
            Self::InvalidSequence(_) => write!(f, "invalid sequence"),
            Self::InvalidQualityScores(_) => write!(f, "invalid quality scores"),
            Self::InvalidData(_) => write!(f, "invalid data"),
        }
    }
}

pub(crate) fn decode_record<B>(
    src: &mut B,
    header: &sam::Header,
    record: &mut Record,
) -> io::Result<()>
where
    B: Buf,
{
    let n_ref = header.reference_sequences().len();

    *record.reference_sequence_id_mut() = get_reference_sequence_id(src, n_ref)
        .map_err(ParseError::InvalidReferenceSequenceId)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    *record.alignment_start_mut() = get_position(src)
        .map_err(ParseError::InvalidPosition)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let l_read_name = read_name::get_length(src)
        .map_err(ParseError::InvalidReadName)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    *record.mapping_quality_mut() = get_mapping_quality(src)
        .map_err(ParseError::InvalidMappingQuality)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Discard bin.
    src.advance(mem::size_of::<u16>());

    let n_cigar_op = cigar::get_op_count(src)
        .map_err(ParseError::InvalidCigar)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    *record.flags_mut() = get_flags(src)
        .map_err(ParseError::InvalidFlags)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let l_seq = sequence::get_length(src)
        .map_err(ParseError::InvalidSequence)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    *record.mate_reference_sequence_id_mut() = get_reference_sequence_id(src, n_ref)
        .map_err(ParseError::InvalidMateReferenceSequenceId)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    *record.mate_alignment_start_mut() = get_position(src)
        .map_err(ParseError::InvalidMatePosition)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    *record.template_length_mut() = get_template_length(src)
        .map_err(ParseError::InvalidTemplateLength)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    get_read_name(src, record.read_name_mut(), l_read_name)
        .map_err(ParseError::InvalidReadName)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    get_cigar(src, record.cigar_mut(), n_cigar_op)
        .map_err(ParseError::InvalidCigar)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    get_sequence(src, record.sequence_mut(), l_seq)
        .map_err(ParseError::InvalidSequence)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    get_quality_scores(src, record.quality_scores_mut(), l_seq)
        .map_err(ParseError::InvalidQualityScores)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    get_data(src, record.data_mut())
        .map_err(ParseError::InvalidData)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    cigar::resolve(header, record)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_block_size() -> io::Result<()> {
        let data = [0x08, 0x00, 0x00, 0x00];
        let mut reader = &data[..];
        assert_eq!(read_block_size(&mut reader)?, 8);

        let data = [];
        let mut reader = &data[..];
        assert_eq!(read_block_size(&mut reader)?, 0);

        Ok(())
    }

    #[test]
    fn test_read_record() -> io::Result<()> {
        let data = [
            0x22, 0x00, 0x00, 0x00, // block_size = 34
            0xff, 0xff, 0xff, 0xff, // ref_id = -1
            0xff, 0xff, 0xff, 0xff, // pos = -1
            0x02, // l_read_name = 2
            0xff, // mapq = 255
            0x48, 0x12, // bin = 4680
            0x00, 0x00, // n_cigar_op = 0
            0x04, 0x00, // flag = 4
            0x00, 0x00, 0x00, 0x00, // l_seq = 0
            0xff, 0xff, 0xff, 0xff, // next_ref_id = -1
            0xff, 0xff, 0xff, 0xff, // next_pos = -1
            0x00, 0x00, 0x00, 0x00, // tlen = 0
            0x2a, 0x00, // read_name = "*\x00"
        ];

        let mut reader = &data[..];
        let header = sam::Header::default();
        let mut buf = Vec::new();
        let mut record = Record::default();
        let block_size = read_record(&mut reader, &header, &mut buf, &mut record)?;

        assert_eq!(block_size, 34);
        assert_eq!(record, Record::default());

        Ok(())
    }

    #[test]
    fn test_decode_record_with_invalid_l_read_name() {
        let data = vec![
            0xff, 0xff, 0xff, 0xff, // ref_id = -1
            0xff, 0xff, 0xff, 0xff, // pos = -1
            0x00, // l_read_name = 0
        ];
        let mut src = &data[..];

        let header = sam::Header::default();
        let mut record = Record::default();

        assert!(matches!(
            decode_record(&mut src, &header, &mut record),
            Err(e) if e.kind() == io::ErrorKind::InvalidData
        ));
    }
}
