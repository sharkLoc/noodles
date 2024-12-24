use std::{error, fmt};

use crate::alignment::record::data::field::Tag;

/// An error when a raw BAM record data field tag fails to parse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// Unexpected EOF.
    UnexpectedEof,
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::UnexpectedEof => None,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof => write!(f, "unexpected EOF"),
        }
    }
}

pub(super) fn parse_tag(src: &mut &[u8]) -> Result<Tag, ParseError> {
    let (buf, rest) = split_first_chunk::<2>(src).ok_or(ParseError::UnexpectedEof)?;
    *src = rest;
    Ok(Tag::new(buf[0], buf[1]))
}

// TODO: Use `slice::split_first_chunk` when the MSRV is raised to or above Rust 1.77.0.
fn split_first_chunk<const N: usize>(src: &[u8]) -> Option<(&[u8; N], &[u8])> {
    if src.len() < N {
        None
    } else {
        // SAFETY: `src.len` >= `N`.
        let (head, tail) = src.split_at(N);
        <&[u8; N]>::try_from(head).ok().map(|chunk| (chunk, tail))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tag() {
        let mut src = &b"NH"[..];
        assert_eq!(parse_tag(&mut src), Ok(Tag::ALIGNMENT_HIT_COUNT));

        let mut src = &b""[..];
        assert_eq!(parse_tag(&mut src), Err(ParseError::UnexpectedEof));
    }
}
