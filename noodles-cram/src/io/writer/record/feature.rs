use noodles_core::Position;

use crate::container::compression_header::preservation_map::substitution_matrix::Base;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Feature {
    Bases {
        position: Position,
        bases: Vec<u8>,
    },
    Scores {
        position: Position,
        quality_scores: Vec<u8>,
    },
    ReadBase {
        position: Position,
        base: u8,
        quality_score: u8,
    },
    #[allow(dead_code)]
    Substitution {
        position: Position,
        reference_base: Base,
        read_base: Base,
    },
    Insertion {
        position: Position,
        bases: Vec<u8>,
    },
    Deletion {
        position: Position,
        len: usize,
    },
    InsertBase {
        position: Position,
        base: u8,
    },
    QualityScore {
        position: Position,
        quality_score: u8,
    },
    ReferenceSkip {
        position: Position,
        len: usize,
    },
    SoftClip {
        position: Position,
        bases: Vec<u8>,
    },
    Padding {
        position: Position,
        len: usize,
    },
    HardClip {
        position: Position,
        len: usize,
    },
}

impl Feature {
    pub fn code(&self) -> u8 {
        match self {
            Self::Bases { .. } => b'b',
            Self::Scores { .. } => b'q',
            Self::ReadBase { .. } => b'B',
            Self::Substitution { .. } => b'X',
            Self::Insertion { .. } => b'I',
            Self::Deletion { .. } => b'D',
            Self::InsertBase { .. } => b'i',
            Self::QualityScore { .. } => b'Q',
            Self::ReferenceSkip { .. } => b'N',
            Self::SoftClip { .. } => b'S',
            Self::Padding { .. } => b'P',
            Self::HardClip { .. } => b'H',
        }
    }

    pub fn position(&self) -> Position {
        match self {
            Self::Bases { position, .. } => *position,
            Self::Scores { position, .. } => *position,
            Self::ReadBase { position, .. } => *position,
            Self::Substitution { position, .. } => *position,
            Self::Insertion { position, .. } => *position,
            Self::Deletion { position, .. } => *position,
            Self::InsertBase { position, .. } => *position,
            Self::QualityScore { position, .. } => *position,
            Self::ReferenceSkip { position, .. } => *position,
            Self::SoftClip { position, .. } => *position,
            Self::Padding { position, .. } => *position,
            Self::HardClip { position, .. } => *position,
        }
    }
}
