//! Version manipulation library.
//!
//! This library provides a way to parse version identifier, and convert them
//! between multiple standards such as
//! [semver.org's Semantic Versioning](https://semver.org/),
//! [Python's PEP-440](https://www.python.org/dev/peps/pep-0440/), etc.
//!
//! # Why not stick with semantic versioning?
//!
//! Semantic versioning has the benefit of being a standard, and is
//! (sort-of) used by some ecosystems, but has many short-comings. For example,
//! it does not support post-releases. It also has some (minor?)
//! incompatibilities with some (most?) standards already in use before its
//! inception, such as Git (output of `git describe`), Debian, Fedora, Python,
//! Conda, etc. A lot of important features were disregarded in the creation of
//! semver (such as post-releases or third-party packaging), and some
//! incompatible choices were made (having the dash automatically indicate a
//! pre-release).
//!
//! Fortunately most of the other versioning schemes are still used when
//! appropriate.
//!
//! This library intends to support most of the version numbers actually in the
//! wild and to provide conversion utilities where possible.
//!
//! # Functionality
//!
//! This library is based around Debian's versioning scheme, which is both very
//! expressive (used by the Debian and Ubuntu communities to version millions
//! of packages of all kinds of software), very compatible (accepts any version
//! string under the sun), and proved (Debian has been around for a while).
//!
//! It can also represent and "import" version numbers from foreign schemes,
//! such semver, PEP-440, etc.
//!
//! If you want to parse random version numbers that you can't assume follow
//! semver, this library is probably what you want.

mod cmp;
pub mod debian;
pub mod python;
pub mod semver;
mod utils;

use std::cmp::{Ordering, PartialOrd};
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

use cmp::{CHAR_ORDER, compare_versions};

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
        compare_versions(&self.0, &other.0)
    }
}

impl Version {
    pub fn epoch() -> u32 {
        unimplemented!() // TODO: Read epoch (default to 0)
    }
}

/// Error for the version parser.
pub enum InvalidVersion {
    /// The version contains invalid characters.
    InvalidCharacter,
    /// The version number contains numeric fields with a leading zero.
    LeadingZero,
    #[doc(hidden)]
    __Nonexhaustive,
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

/// A simple "old-school" version number (only numbers and dots).
///
/// This is of special interest because it should be compatible and unambiguous
/// in all versioning schemes (expect for semver's requirement that there be
/// exactly 3 fields).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleVersion(Version);

impl AsRef<Version> for SimpleVersion {
    fn as_ref(&self) -> &Version {
        &self.0
    }
}
