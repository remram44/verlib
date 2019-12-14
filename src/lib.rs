pub mod debian;
pub mod python;
pub mod semver;
mod utils;

use std::cmp::{Ordering, PartialOrd};
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

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

/// Modified string comparison.
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
