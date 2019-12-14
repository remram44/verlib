use std::convert::TryFrom;
use std::fmt;

use crate::Version;
use crate::utils::NumChecker;

/// "Semantic version" as per semver.org.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemverVersion(String);

impl fmt::Display for SemverVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The version is not supported by semver.
#[derive(Debug, PartialEq, Eq)]
pub enum ToSemverError {
    HasEpoch,
    HasPost,
    TooManyFields,
    LeadingZero,
    InvalidCharacter,
}

trait ToSemver {
    /// Convert if the version is a valid semver.
    fn to_semver(&self) -> Result<SemverVersion, ToSemverError>;

    /// Convert as best as we can.
    fn to_semver_lossy(&self) -> SemverVersion;
}

impl ToSemver for Version {
    /// Convert to a semantic version.
    ///
    /// This will work if the version has maximum two fields, and no
    /// post-release or epoch segment.
    ///
    /// Examples:
    ///
    /// * `1.2` -> `1.2.0`
    /// * `1.2.4~rc1` -> `1.2.4-rc.1`
    fn to_semver(&self) -> Result<SemverVersion, ToSemverError> {
        let mut field = 0;
        let mut num_check = NumChecker::new();
        let mut version = Vec::new();
        let mut read_epoch = false;
        for c in self.0.bytes() {
            if b'0' <= c && c <= b'9' {
                if num_check == NumChecker::NotNum {
                    version.push(b'.');
                    num_check.reset();
                }
                version.push(c);
                if !num_check.check(c) {
                    return Err(ToSemverError::LeadingZero);
                }
            } else if c == b'.' {
                if num_check == NumChecker::Start {
                    // Empty field
                    return Err(ToSemverError::InvalidCharacter);
                }
                if field == 2 {
                    return Err(ToSemverError::TooManyFields);
                }
                version.push(c);
                field += 1;
                num_check.reset();
            } else if c == b'~' {
                if num_check == NumChecker::Start {
                    // Empty field
                    return Err(ToSemverError::InvalidCharacter);
                }
                // Add zeros for the missing fields
                for _ in field .. 2 {
                    version.extend(b".0");
                }
                version.push(b'-');
                field = 3;
                num_check.reset();
            } else if
                c == b':' && field == 0 && !read_epoch && !version.is_empty()
            {
                if &version == b"0" {
                    version.clear();
                    read_epoch = true;
                    num_check.reset();
                } else {
                    return Err(ToSemverError::HasEpoch);
                }
            } else if c == b'-' && num_check != NumChecker::Start {
                return Err(ToSemverError::HasPost);
            } else if b'a' <= c && c <= b'z' {
                if field < 3 {
                    // Alphabetical characters only allowed in pre-release part
                    return Err(ToSemverError::InvalidCharacter);
                }
                if num_check.numeric() {
                    version.push(b'.');
                    num_check.reset();
                }
                version.push(c);
                num_check.check(c);
            } else {
                return Err(ToSemverError::InvalidCharacter);
            }
        }
        if num_check == NumChecker::Start {
            // Empty field
            return Err(ToSemverError::InvalidCharacter);
        }
        // Add zeros for the missing fields
        for _ in field .. 2 {
            version.extend(b".0");
        }
        Ok(SemverVersion(String::from_utf8(version).unwrap()))
    }

    /// Convert to a semantic version, removing incompatible information
    ///
    /// This will convert try to encode additional version fields (semver only
    /// allows 3) and pre-release and post-release information in semver's
    /// pre-release field (bumping the patch version if needed).
    ///
    /// Examples:
    ///
    /// * `1.2` -> `1.2.0`
    /// * `1.2.3.1` -> `1.2.3+patch.1`
    /// * `1.2.4~rc1` -> `1.2.4-rc.1`
    /// * `2:1.2.2` -> `1.2.2`
    fn to_semver_lossy(&self) -> SemverVersion {
        unimplemented!() // TODO: Version to SimpleVersion lossy
            // Put post-release info in local version, etc
    }
}

impl<'a> TryFrom<&'a Version> for SemverVersion {
    type Error = ToSemverError;

    fn try_from(version: &'a Version) -> Result<SemverVersion, ToSemverError> {
        version.to_semver()
    }
}

impl From<SemverVersion> for Version {
    fn from(semver: SemverVersion) -> Version {
        Version(semver.0.replace("-", "~"))
    }
}

#[cfg(test)]
mod tests {
    use crate::Version;
    use super::{SemverVersion, ToSemver, ToSemverError};

    #[test]
    fn test_to_semver() {
        assert_eq!(
            Version("1.2.3".into()).to_semver(),
            Ok(SemverVersion("1.2.3".into())),
        );
        assert_eq!(
            Version("1.2.3.4".into()).to_semver(),
            Err(ToSemverError::TooManyFields),
        );
        assert_eq!(
            Version("1.2".into()).to_semver(),
            Ok(SemverVersion("1.2.0".into())),
        );
        assert_eq!(
            Version("8".into()).to_semver(),
            Ok(SemverVersion("8.0.0".into())),
        );
        assert_eq!(
            Version("0:1.2.3".into()).to_semver(),
            Ok(SemverVersion("1.2.3".into())),
        );
        assert_eq!(
            Version("1:1.2.3".into()).to_semver(),
            Err(ToSemverError::HasEpoch),
        );
        assert_eq!(
            Version("0:0:1.2.3".into()).to_semver(),
            Err(ToSemverError::InvalidCharacter),
        );
        assert_eq!(
            Version(":1.2.3".into()).to_semver(),
            Err(ToSemverError::InvalidCharacter),
        );
        assert_eq!(
            Version("1.02".into()).to_semver(),
            Err(ToSemverError::LeadingZero),
        );
        assert_eq!(
            Version(".2".into()).to_semver(),
            Err(ToSemverError::InvalidCharacter),
        );
        assert_eq!(
            Version("-2".into()).to_semver(),
            Err(ToSemverError::InvalidCharacter),
        );
        assert_eq!(
            Version("~2".into()).to_semver(),
            Err(ToSemverError::InvalidCharacter),
        );
        assert_eq!(
            Version("1.2~rc1".into()).to_semver(),
            Ok(SemverVersion("1.2.0-rc.1".into())),
        );
        assert_eq!(
            Version("1.2~0ubuntu3".into()).to_semver(),
            Ok(SemverVersion("1.2.0-0.ubuntu.3".into())),
        );
        assert_eq!(
            Version("1.-2".into()).to_semver(),
            Err(ToSemverError::InvalidCharacter),
        );
    }
}
