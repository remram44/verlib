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
    pub fn upstream_version(&self) -> &str {
        unimplemented!() // TODO: Read until last hyphen
    }

    pub fn debian_revision(&self) -> Option<&str> {
        unimplemented!() // TODO: Read from last hyphen
    }
}
