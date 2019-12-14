#[derive(PartialEq, Eq)]
pub enum NumChecker {
    Start,
    NotNum,
    Zero,
    OtherNum,
}

impl NumChecker {
    pub fn new() -> NumChecker {
        NumChecker::Start
    }

    pub fn reset(&mut self) {
        *self = NumChecker::Start;
    }

    pub fn numeric(&self) -> bool {
        match *self {
            NumChecker::Start => false,
            NumChecker::NotNum => false,
            NumChecker::Zero => true,
            NumChecker::OtherNum => true,
        }
    }

    pub fn check(&mut self, c: u8) -> bool {
        *self = match *self {
            NumChecker::Start|NumChecker::NotNum => {
                if c == b'0' {
                    NumChecker::Zero
                } else if b'0' <= c && c <= b'9' {
                    NumChecker::OtherNum
                } else {
                    NumChecker::NotNum
                }
            }
            NumChecker::Zero => {
                if b'0' <= c && c <= b'9' {
                    return false;
                } else {
                    NumChecker::NotNum
                }
            }
            NumChecker::OtherNum => {
                if b'0' <= c && c <= b'9' {
                    NumChecker::OtherNum
                } else {
                    NumChecker::NotNum
                }
            }
        };
        true
    }
}

#[cfg(test)]
mod tests {
    use super::NumChecker;

    fn all_checks(s: &[u8]) -> bool {
        let mut num_check = NumChecker::new();
        for &c in s {
            if !num_check.check(c) {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_num_checker() {
        assert!(all_checks(b"test123yes456"));
        assert!(!all_checks(b"test0123yes456"));
        assert!(!all_checks(b"test123yes0456"));
    }
}
