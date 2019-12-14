mod cmp;
pub mod debian;
pub mod python;
pub mod semver;
mod utils;

use std::cmp::{Ordering, PartialOrd};
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

use cmp::CHAR_ORDER;

/// A version number.
#[derive(Clone, Debug, Hash)]
pub struct Version(String);

impl Deref for Version {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<Version> for Version {
    fn eq(&self, other: &Version) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Version {}

impl PartialOrd<Version> for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        unimplemented!() // TODO: Compare versions
    }
}

pub enum InvalidVersion {
    InvalidCharacter,
    LeadingZero,
}

impl TryFrom<String> for Version {
    type Error = InvalidVersion;

    fn try_from(string: String) -> Result<Version, InvalidVersion> {
        for c in string.bytes() {
            // Check characters are allowed
            if CHAR_ORDER[usize::from(c)] == 255 {
                return Err(InvalidVersion::InvalidCharacter);
            }
        }
        Ok(Version(string))
    }
}

/// A simple version number (only numbers and dots).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleVersion(Version);

impl AsRef<Version> for SimpleVersion {
    fn as_ref(&self) -> &Version {
        &self.0
    }
}
