//! Implement Debian's versioning scheme.
//!
//! This package uses Debian's versioning rules by default, so this is a thin
//! wrapper around `Version`, but it adds some Debian-specific accessors.

use std::ops::Deref;

use crate::Version;

/// A Debian version number.
///
/// This package uses Debian's versioning rules, so this is a thin wrapper
/// around `Version`, but it adds some Debian-specific accessors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebianVersion(Version);

impl Deref for DebianVersion {
    type Target = Version;

    fn deref(&self) -> &Version {
        &self.0
    }
}

impl DebianVersion {
    /// The upstream version of package, e.g. version of the packaged software.
    pub fn upstream_version(&self) -> &str {
        match self.rfind('-') {
            Some(hyphen) => &self[0..hyphen],
            None => &self
        }
    }

    /// The Debian revision, e.g. version of the packaging itself.
    pub fn debian_revision(&self) -> Option<&str> {
        match self.rfind('-') {
            Some(hyphen) => Some(&self[hyphen + 1 ..]),
            None => None
        }
    }
}
