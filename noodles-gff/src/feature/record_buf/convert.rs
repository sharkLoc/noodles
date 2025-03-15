use std::io;

use super::{attributes::field::Value, RecordBuf};
use crate::feature::{record::attributes::field::Value as ValueRef, Record};

impl RecordBuf {
    /// Converts a feature record to a record buffer.
    pub fn try_from_feature_record<R>(record: &R) -> io::Result<Self>
    where
        R: Record,
    {
        let mut builder = Self::builder();

        builder = builder
            .set_reference_sequence_name(record.reference_sequence_name())
            .set_source(record.source())
            .set_type(record.ty())
            .set_start(record.feature_start()?)
            .set_end(record.feature_end()?);

        if let Some(score) = record.score().transpose()? {
            builder = builder.set_score(score);
        }

        builder = builder.set_strand(record.strand()?);

        if let Some(phase) = record.phase().transpose()? {
            builder = builder.set_phase(phase);
        }

        let attributes = record
            .attributes()
            .iter()
            .map(|result| {
                result.and_then(|(k, v)| {
                    let value = match v {
                        ValueRef::String(s) => Value::from(s.as_ref()),
                        ValueRef::Array(values) => Value::Array(
                            values
                                .iter()
                                .map(|result| result.map(String::from))
                                .collect::<io::Result<_>>()?,
                        ),
                    };

                    Ok((k.into(), value))
                })
            })
            .collect::<io::Result<_>>()?;

        builder = builder.set_attributes(attributes);

        Ok(builder.build())
    }
}
