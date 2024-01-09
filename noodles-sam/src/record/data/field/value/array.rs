//! SAM record data field array.

use crate::alignment::record::data::field::value::array::Subtype;

/// A SAM record data field array value.
#[derive(Clone, Debug, PartialEq)]
pub enum Array {
    /// An 8-bit integer array (`B:c`).
    Int8(Vec<i8>),
    /// An 8-bit unsigned integer array (`B:C`).
    UInt8(Vec<u8>),
    /// A 16-bit integer array (`B:s`).
    Int16(Vec<i16>),
    /// A 16-bit unsigned integer array (`B:S`).
    UInt16(Vec<u16>),
    /// A 32-bit integer array (`B:i`).
    Int32(Vec<i32>),
    /// A 32-bit unsigned integer array (`B:I`).
    UInt32(Vec<u32>),
    /// A single-precision floating-point array (`B:f`).
    Float(Vec<f32>),
}

impl Array {
    /// Returns the type of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::{
    ///     alignment::record::data::field::value::array::Subtype,
    ///     record::data::field::{value::Array, Value},
    /// };
    ///
    /// assert_eq!(Array::UInt8(vec![0]).subtype(), Subtype::UInt8);
    /// ```
    pub fn subtype(&self) -> Subtype {
        match self {
            Self::Int8(_) => Subtype::Int8,
            Self::UInt8(_) => Subtype::UInt8,
            Self::Int16(_) => Subtype::Int16,
            Self::UInt16(_) => Subtype::UInt16,
            Self::Int32(_) => Subtype::Int32,
            Self::UInt32(_) => Subtype::UInt32,
            Self::Float(_) => Subtype::Float,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtype() {
        assert_eq!(Array::Int8(vec![0]).subtype(), Subtype::Int8);
        assert_eq!(Array::UInt8(vec![0]).subtype(), Subtype::UInt8);
        assert_eq!(Array::Int16(vec![0]).subtype(), Subtype::Int16);
        assert_eq!(Array::UInt16(vec![0]).subtype(), Subtype::UInt16);
        assert_eq!(Array::Int32(vec![0]).subtype(), Subtype::Int32);
        assert_eq!(Array::UInt32(vec![0]).subtype(), Subtype::UInt32);
        assert_eq!(Array::Float(vec![0.0]).subtype(), Subtype::Float);
    }
}
