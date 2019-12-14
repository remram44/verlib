use std::cmp::{Ordering, PartialOrd};
use std::fmt::{self, Write};
use std::str::FromStr;

const CHAR_ORDER: &'static [u8] = &[
    255u8, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 27,
    255, 28, 255, 255, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
    14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 255, 255, 255, 0, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255,
];

/// Modified string comparison
///
/// Compares ASCII strings using the following rules:
///
/// * All the letters sort earlier than all non-letter
/// * Tilde sorts before anything, including the end of the string
///
/// For example, the following strings are in sorted order: `"~~"`, `"~~a"`,
/// `""`, `"a"`.
///
/// See https://www.debian.org/doc/debian-policy/ch-controlfields.html#version
pub fn compare_alpha(a: &str, b: &str) -> std::cmp::Ordering {
    let a = a.as_bytes();
    let b = b.as_bytes();

    // Compare characters using the CHAR_ORDER array
    for (&ca, &cb) in a.iter().zip(b.iter()) {
        let pa = CHAR_ORDER[usize::from(ca)];
        let pb = CHAR_ORDER[usize::from(cb)];
        match pa.cmp(&pb) {
            Ordering::Equal => {},
            o => return o,
        }
    }

    // If the strings are the same size, then they are the same
    if a.len() == b.len() {
        Ordering::Equal
    // Otherwise, the longer string is greater (unless tilde comes next)
    } else if a.len() < b.len() {
        if b[a.len()] == b'~' {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    } else if a.len() > b.len() {
        if a[b.len()] == b'~' {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    } else {
        unreachable!()
    }
}

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
    TooManyFields,
}

impl Version {
    /// Get the version in semver format `version-pre`, if no post-release info
    pub fn to_semver(&self) -> Result<String, ToSemverError> {
        if self.epoch != 0 {
            Err(ToSemverError::HasEpoch)
        } else if !self.post.is_empty() {
            Err(ToSemverError::HasPost)
        } else if self.version.len() > 3 {
            Err(ToSemverError::TooManyFields)
        } else {
            let mut version = String::new();
            for (i, field) in self.version.iter().enumerate() {
                if i > 0 {
                    version.push('.');
                }
                write!(version, "{}", field).unwrap();
            }
            if !self.pre.is_empty() {
                version.push('-');
                for (i, field) in self.pre.iter().enumerate() {
                    if i > 0 {
                        version.push('.');
                    }
                    write!(version, "{}", field).unwrap();
                }
            }
            Ok(version)
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.epoch > 0 {
            write!(f, "{}:", self.epoch)?;
        }
        for (i, field) in self.version.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", field)?;
        }
        if self.version.is_empty() {
            write!(f, "0")?;
        }
        for field in self.pre.iter().chain(self.post.iter()) {
            write!(f, ".{}", field)?;
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::{CHAR_ORDER, compare_alpha};


    struct PrioSetter {
        prio: u8,
        char_order: [u8; 256],
    }

    impl PrioSetter {
        fn new() -> PrioSetter {
            PrioSetter {
                prio: 0,
                char_order: [255u8; 256],
            }
        }

        fn set(&mut self, character: u8) {
            self.char_order[usize::from(character)] = self.prio;
            self.prio += 1;
        }

        fn build(self) -> [u8; 256] {
            self.char_order
        }
    }

    #[test]
    fn test_char_order() {
        // This is how I generated the array
        let mut prios = PrioSetter::new();
        prios.set(b'~');
        for c in b'a' ..= b'z' {
            prios.set(c);
        }
        prios.set(b'+');
        prios.set(b'-');
        for c in b'0' ..= b'9' {
            prios.set(c);
        }
        let prios = prios.build();

        print!("[");
        for c in &prios as &[u8] {
            print!("{}, ", c);
        }
        println!("]");

        assert!(prios.len() == CHAR_ORDER.len());
        assert!(prios.iter().zip(CHAR_ORDER.iter()).all(|(a, b)| a == b));
    }

    #[test]
    fn test_compare_alpha() {
        // Equal
        assert_eq!(compare_alpha("test", "test"), Ordering::Equal);
        assert_eq!(compare_alpha("", ""), Ordering::Equal);
        // Ordering
        assert_eq!(compare_alpha("t1", "t2"), Ordering::Less);
        assert_eq!(compare_alpha("t2", "t1"), Ordering::Greater);
        assert_eq!(compare_alpha("t133", "t2"), Ordering::Less);
        assert_eq!(compare_alpha("t2", "t133"), Ordering::Greater);
        assert_eq!(compare_alpha("ta", "tb"), Ordering::Less);
        assert_eq!(compare_alpha("tb", "ta"), Ordering::Greater);
        assert_eq!(compare_alpha("tz", "test"), Ordering::Greater);
        assert_eq!(compare_alpha("test", "tz"), Ordering::Less);
        // Letters come before numbers
        assert_eq!(compare_alpha("test", "te5t"), Ordering::Less);
        assert_eq!(compare_alpha("te5t", "test"), Ordering::Greater);
        // End comes before all (but tilde)
        assert_eq!(compare_alpha("test", "te"), Ordering::Greater);
        assert_eq!(compare_alpha("te", "test"), Ordering::Less);
        assert_eq!(compare_alpha("te-", "te"), Ordering::Greater);
        assert_eq!(compare_alpha("te", "te-"), Ordering::Less);
        // Tilde comes before end
        assert_eq!(compare_alpha("te~", "te"), Ordering::Less);
        assert_eq!(compare_alpha("te", "te~"), Ordering::Greater);
    }
}
