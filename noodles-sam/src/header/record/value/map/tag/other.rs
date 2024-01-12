use std::{
    borrow::Borrow,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use super::{ParseError, Standard, LENGTH};

/// A nonstandard tag.
#[derive(Clone, Copy, Debug)]
pub struct Other<S>(pub(super) [u8; LENGTH], pub(super) PhantomData<S>);

impl<S> AsRef<[u8; LENGTH]> for Other<S> {
    fn as_ref(&self) -> &[u8; LENGTH] {
        &self.0
    }
}

impl<S> Borrow<[u8; LENGTH]> for Other<S> {
    fn borrow(&self) -> &[u8; LENGTH] {
        self.as_ref()
    }
}

impl<S> Hash for Other<S> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.hash(state);
    }
}

impl<S> PartialEq for Other<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S> PartialEq<[u8; LENGTH]> for Other<S> {
    fn eq(&self, other: &[u8; LENGTH]) -> bool {
        self.0.eq(other)
    }
}

impl<S> Eq for Other<S> {}

impl<S> fmt::Display for Other<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        char::from(self.0[0]).fmt(f)?;
        char::from(self.0[1]).fmt(f)?;
        Ok(())
    }
}

impl<S> TryFrom<[u8; LENGTH]> for Other<S>
where
    S: Standard,
{
    type Error = ParseError;

    fn try_from(buf: [u8; LENGTH]) -> Result<Self, Self::Error> {
        use super::{is_valid_tag, Tag};

        if !is_valid_tag(buf) {
            return Err(ParseError::Invalid);
        }

        match Tag::from(buf) {
            Tag::Standard(_) => Err(ParseError::Invalid),
            Tag::Other(tag) => Ok(tag),
        }
    }
}
