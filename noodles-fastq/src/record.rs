use std::fmt;

/// A FASTQ record.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Record {
    read_name: Vec<u8>,
    sequence: Vec<u8>,
    quality_scores: Vec<u8>,
}

impl Record {
    /// Creates a FASTQ record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_fastq::Record;
    /// let record = Record::new("r0", "AGCT", "NDLS");
    /// ```
    pub fn new<S, T, U>(read_name: S, sequence: T, quality_scores: U) -> Self
    where
        S: Into<Vec<u8>>,
        T: Into<Vec<u8>>,
        U: Into<Vec<u8>>,
    {
        Self {
            read_name: read_name.into(),
            sequence: sequence.into(),
            quality_scores: quality_scores.into(),
        }
    }

    /// Returns the read name of the record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_fastq::Record;
    /// let record = Record::new("r0", "AGCT", "NDLS");
    /// assert_eq!(record.read_name(), b"r0");
    /// ```
    pub fn read_name(&self) -> &[u8] {
        &self.read_name
    }

    pub(crate) fn read_name_mut(&mut self) -> &mut Vec<u8> {
        &mut self.read_name
    }

    /// Returns the sequence of the record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_fastq::Record;
    /// let record = Record::new("r0", "AGCT", "NDLS");
    /// assert_eq!(record.sequence(), b"AGCT");
    /// ```
    pub fn sequence(&self) -> &[u8] {
        &self.sequence
    }

    pub(crate) fn sequence_mut(&mut self) -> &mut Vec<u8> {
        &mut self.sequence
    }

    /// Returns the quality scores of the record.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_fastq::Record;
    /// let record = Record::new("r0", "AGCT", "NDLS");
    /// assert_eq!(record.quality_scores(), b"NDLS");
    /// ```
    pub fn quality_scores(&self) -> &[u8] {
        &self.quality_scores
    }

    pub(crate) fn quality_scores_mut(&mut self) -> &mut Vec<u8> {
        &mut self.quality_scores
    }

    // Truncates all field buffers to 0.
    pub(crate) fn clear(&mut self) {
        self.read_name.clear();
        self.sequence.clear();
        self.quality_scores.clear();
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("@")?;

        for &b in self.read_name() {
            write!(f, "{}", b as char)?;
        }

        writeln!(f)?;

        for &b in self.sequence() {
            write!(f, "{}", b as char)?;
        }

        writeln!(f)?;

        writeln!(f, "+")?;

        for &b in self.quality_scores() {
            write!(f, "{}", b as char)?;
        }

        writeln!(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        let record = Record::new("r0", "ATCG", "NDLS");
        assert_eq!(record.to_string(), "@r0\nATCG\n+\nNDLS\n");
    }

    #[test]
    fn test_clear() {
        let mut record = Record::new("r0", "AGCT", "NDLS");
        record.clear();

        assert!(record.read_name().is_empty());
        assert!(record.sequence().is_empty());
        assert!(record.quality_scores().is_empty());
    }
}
