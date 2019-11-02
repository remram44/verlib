use std::cmp::{Ordering, PartialOrd};
use std::fmt::{self, Write};
use std::str::FromStr;

/// Number type used for numeric fields
pub type Number = u32;

/// A single field in a version, either a number of alphanumerical string
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Field {
    Alpha(String),
    Num(Number),
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Field::Alpha(s) => write!(f, "{}", s),
            Field::Num(n) => write!(f, "{}", n),
        }
    }
}

/// A version number, including pre- and post- release parts
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version {
    pub epoch: Number,
    pub version: Vec<u32>,
    pub pre: Vec<Field>,
    pub post: Vec<Field>,
}

/// The version is not supported by semver
pub enum ToSemverError {
    HasEpoch,
    HasPost,
}

impl Version {
    /// Get the version in semver format `version-pre`, if no post-release info
    pub fn to_semver(&self) -> Result<String, ToSemverError> {
        unimplemented!() // TODO: Convert to semver
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!() // TODO: Print version
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        unimplemented!() // TODO: Compare versions
    }
}

/// Error parsing the version string
pub enum ParseVersionError {
}

impl FromStr for Field {
    type Err = ParseVersionError;

    fn from_str(field: &str) -> Result<Field, ParseVersionError> {
        unimplemented!() // TODO: Parse field
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(version: &str) -> Result<Version, ParseVersionError> {
        unimplemented!() // TODO: Parse version
    }
}

pub fn parse_final(version: &str) -> Result<Version, ParseVersionError> {
    unimplemented!() // TODO: Parse simple version
}

pub fn from_semver(version: &str) -> Result<Version, ParseVersionError> {
    unimplemented!() // TODO: Parse semver version
}
