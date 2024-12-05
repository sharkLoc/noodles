//! GFF record and fields.

pub mod attributes;
mod builder;
mod convert;
mod field;
mod phase;
pub mod strand;

pub use self::{
    attributes::Attributes, builder::Builder, field::Field, phase::Phase, strand::Strand,
};

use noodles_core::Position;

/// A GFF record.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordBuf {
    reference_sequence_name: String,
    source: String,
    ty: String,
    start: Position,
    end: Position,
    score: Option<f32>,
    strand: Strand,
    phase: Option<Phase>,
    attributes: Attributes,
}

impl RecordBuf {
    /// Returns a builder to create a record from each of its fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    ///
    /// let record = gff::RecordBuf::builder()
    ///     .set_reference_sequence_name(String::from("sq0"))
    ///     .build();
    ///
    /// assert_eq!(record.reference_sequence_name(), "sq0");
    /// ```
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Returns the reference sequence name of the record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    /// let record = gff::RecordBuf::default();
    /// assert_eq!(record.reference_sequence_name(), ".");
    /// ```
    pub fn reference_sequence_name(&self) -> &str {
        &self.reference_sequence_name
    }

    /// Returns the source of the record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    /// let record = gff::RecordBuf::default();
    /// assert_eq!(record.source(), ".");
    /// ```
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the feature type of the record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    /// let record = gff::RecordBuf::default();
    /// assert_eq!(record.ty(), ".");
    /// ```
    pub fn ty(&self) -> &str {
        &self.ty
    }

    /// Returns the start position of the record.
    ///
    /// This position is 1-based, inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_core::Position;
    /// use noodles_gff as gff;
    /// let record = gff::RecordBuf::default();
    /// assert_eq!(record.start(), Position::MIN);
    /// ```
    pub fn start(&self) -> Position {
        self.start
    }

    /// Returns the end position of the record.
    ///
    /// This position is 1-based, inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_core::Position;
    /// use noodles_gff as gff;
    /// let record = gff::RecordBuf::default();
    /// assert_eq!(record.end(), Position::MIN);
    /// ```
    pub fn end(&self) -> Position {
        self.end
    }

    /// Returns the score of the record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    /// let record = gff::RecordBuf::default();
    /// assert!(record.score().is_none());
    /// ```
    pub fn score(&self) -> Option<f32> {
        self.score
    }

    /// Returns the strand of the record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff::{self as gff, record_buf::Strand};
    /// let record = gff::RecordBuf::default();
    /// assert_eq!(record.strand(), Strand::None);
    /// ```
    pub fn strand(&self) -> Strand {
        self.strand
    }

    /// Returns the phase of the record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    /// let record = gff::RecordBuf::default();
    /// assert!(record.phase().is_none());
    /// ```
    pub fn phase(&self) -> Option<Phase> {
        self.phase
    }

    /// Returns the attributes of the record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    /// let record = gff::RecordBuf::default();
    /// assert!(record.attributes().is_empty());
    /// ```
    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }
}

impl Default for RecordBuf {
    fn default() -> Self {
        Builder::new().build()
    }
}
