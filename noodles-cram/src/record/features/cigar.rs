use std::slice;

use noodles_core::Position;
use noodles_sam::alignment::record::cigar::{op::Kind, Op};

use crate::record::Feature;

/// An iterator over features as CIGAR operations.
pub struct Cigar<'a> {
    features: slice::Iter<'a, Feature>,
    read_length: usize,
    read_position: Position,
    next_op: Option<(Kind, usize)>,
}

impl<'a> Cigar<'a> {
    pub(super) fn new(features: &'a [Feature], read_length: usize) -> Self {
        Self {
            features: features.iter(),
            read_length,
            read_position: Position::MIN,
            next_op: None,
        }
    }

    fn consume_read(&mut self, len: usize) {
        self.read_position = self
            .read_position
            .checked_add(len)
            .expect("attempt to add with overflow");
    }
}

impl<'a> Iterator for Cigar<'a> {
    type Item = Op;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((kind, len)) = self.next_op.take() {
            return Some(Op::new(kind, len));
        }

        let Some(feature) = self.features.next() else {
            if usize::from(self.read_position) <= self.read_length {
                let len = self.read_length - usize::from(self.read_position) + 1;
                self.consume_read(len);
                return Some(Op::new(Kind::Match, len));
            } else {
                return None;
            }
        };

        if feature.position() > self.read_position {
            let len = usize::from(feature.position()) - usize::from(self.read_position);
            self.read_position = feature.position();
            self.next_op = Some((Kind::Match, len));
        }

        let (kind, len) = match feature {
            Feature::Substitution { .. } => (Kind::Match, 1),
            Feature::Insertion { bases, .. } => (Kind::Insertion, bases.len()),
            Feature::Deletion { len, .. } => (Kind::Deletion, *len),
            Feature::InsertBase { .. } => (Kind::Insertion, 1),
            Feature::ReferenceSkip { len, .. } => (Kind::Skip, *len),
            Feature::SoftClip { bases, .. } => (Kind::SoftClip, bases.len()),
            Feature::Padding { len, .. } => (Kind::Pad, *len),
            Feature::HardClip { len, .. } => (Kind::HardClip, *len),
            _ => todo!(),
        };

        if kind.consumes_read() {
            self.consume_read(len);
        }

        match self.next_op.replace((kind, len)) {
            Some((kind, len)) => Some(Op::new(kind, len)),
            None => self.next_op.take().map(|(kind, len)| Op::new(kind, len)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::{feature::substitution, Features};

    #[test]
    fn test_next() -> Result<(), noodles_core::position::TryFromIntError> {
        fn t(features: &Features, read_length: usize, expected: &[Op]) {
            let cigar = Cigar::new(features, read_length);
            let actual: Vec<_> = cigar.collect();
            assert_eq!(actual, expected);
        }

        let features = Features::default();
        t(&features, 4, &[Op::new(Kind::Match, 4)]);

        let features = Features::from(vec![Feature::SoftClip {
            position: Position::try_from(1)?,
            bases: vec![b'A', b'T'],
        }]);
        t(
            &features,
            4,
            &[Op::new(Kind::SoftClip, 2), Op::new(Kind::Match, 2)],
        );

        let features = Features::from(vec![Feature::SoftClip {
            position: Position::try_from(4)?,
            bases: vec![b'G'],
        }]);
        t(
            &features,
            4,
            &[Op::new(Kind::Match, 3), Op::new(Kind::SoftClip, 1)],
        );

        let features = Features::from(vec![Feature::HardClip {
            position: Position::try_from(1)?,
            len: 2,
        }]);
        t(
            &features,
            4,
            &[Op::new(Kind::HardClip, 2), Op::new(Kind::Match, 4)],
        );

        let features = Features::from(vec![
            Feature::SoftClip {
                position: Position::try_from(1)?,
                bases: vec![b'A'],
            },
            Feature::Substitution {
                position: Position::try_from(3)?,
                value: substitution::Value::Code(0),
            },
        ]);
        t(
            &features,
            4,
            &[
                Op::new(Kind::SoftClip, 1),
                Op::new(Kind::Match, 1),
                Op::new(Kind::Match, 1),
                Op::new(Kind::Match, 1),
            ],
        );

        let features = Features::from(vec![Feature::Substitution {
            position: Position::try_from(2)?,
            value: substitution::Value::Code(0),
        }]);
        t(
            &features,
            4,
            &[
                Op::new(Kind::Match, 1),
                Op::new(Kind::Match, 1),
                Op::new(Kind::Match, 2),
            ],
        );

        Ok(())
    }
}
