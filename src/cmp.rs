use std::cmp::Ordering;

pub const CHAR_ORDER: &'static [u8] = &[
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
fn compare_alpha(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
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

fn position<T, F>(slice: &[T], start: usize, pred: F) -> usize
where F: FnMut(&T) -> bool {
    slice[start ..].iter().position(pred).map(|idx| idx + start).unwrap_or(slice.len())
}

fn parse_int(slice: &[u8]) -> u32 {
   use std::str::from_utf8;
   from_utf8(slice).ok().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0)
}

pub fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let a = a.as_bytes();
    let b = b.as_bytes();

    let mut pos_a = 0;
    let mut pos_b = 0;
    while pos_a < a.len() || pos_b < b.len() {
       // Compare alphabetical sections
       {
           let end_alpha_a = position(a, pos_a, u8::is_ascii_digit);
           let end_alpha_b = position(b, pos_b, u8::is_ascii_digit);
           match compare_alpha(&a[pos_a .. end_alpha_a], &b[pos_b .. end_alpha_b]) {
               Ordering::Equal => {},
               o => return o,
           }
           pos_a = end_alpha_a;
           pos_b = end_alpha_b;
        }

       // Compare numerical sections
       {
           let end_num_a = position(a, pos_a, |c| !c.is_ascii_digit());
           let end_num_b = position(b, pos_b, |c| !c.is_ascii_digit());
           let num_a = parse_int(&a[pos_a .. end_num_a]);
           let num_b = parse_int(&b[pos_b .. end_num_b]);
           match num_a.cmp(&num_b) {
               Ordering::Equal => {},
               o => return o,
           }
           pos_a = end_num_a;
           pos_b = end_num_b;
        }
    }
    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::Version;
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
        assert_eq!(compare_alpha(b"test", b"test"), Ordering::Equal);
        assert_eq!(compare_alpha(b"", b""), Ordering::Equal);
        // Ordering
        assert_eq!(compare_alpha(b"t1", b"t2"), Ordering::Less);
        assert_eq!(compare_alpha(b"t2", b"t1"), Ordering::Greater);
        assert_eq!(compare_alpha(b"t133", b"t2"), Ordering::Less);
        assert_eq!(compare_alpha(b"t2", b"t133"), Ordering::Greater);
        assert_eq!(compare_alpha(b"ta", b"tb"), Ordering::Less);
        assert_eq!(compare_alpha(b"tb", b"ta"), Ordering::Greater);
        assert_eq!(compare_alpha(b"tz", b"test"), Ordering::Greater);
        assert_eq!(compare_alpha(b"test", b"tz"), Ordering::Less);
        // Letters come before numbers
        assert_eq!(compare_alpha(b"test", b"te5t"), Ordering::Less);
        assert_eq!(compare_alpha(b"te5t", b"test"), Ordering::Greater);
        // End comes before all (but tilde)
        assert_eq!(compare_alpha(b"test", b"te"), Ordering::Greater);
        assert_eq!(compare_alpha(b"te", b"test"), Ordering::Less);
        assert_eq!(compare_alpha(b"te-", b"te"), Ordering::Greater);
        assert_eq!(compare_alpha(b"te", b"te-"), Ordering::Less);
        // Tilde comes before end
        assert_eq!(compare_alpha(b"te~", b"te"), Ordering::Less);
        assert_eq!(compare_alpha(b"te", b"te~"), Ordering::Greater);
    }

    #[test]
    fn test_compare_versions() {
        assert!(Version("".into()) == Version("".into()));
        assert!(Version("1.2".into()) == Version("1.2".into()));
        assert!(Version("1.2".into()) < Version("1.2.0".into()));
        assert!(Version("1.3.1".into()) > Version("1.1.3".into()));
        assert!(Version("1.1.3".into()) < Version("1.3.1".into()));
        assert!(Version("1.1~rc1".into()) < Version("1.1".into()));
        assert!(Version("1.1-fix1".into()) > Version("1.1".into()));
    }
}
