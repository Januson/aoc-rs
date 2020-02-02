use std::ops::Range;
use std::iter::FromIterator;
use crate::util::duplicates::DuplicateIterator;

struct StringRange {
    origin: String,
}

#[derive(Debug, PartialEq)]
struct Password {
    value: u64,
}

impl StringRange {
    fn new(origin: String) -> StringRange {
        StringRange {
            origin,
        }
    }
}

impl Into<Range<u64>> for StringRange {
    fn into(self) -> Range<u64> {
        let mut origin = self.origin.split('-')
            .map(|n| n.parse::<u64>().unwrap());
        match origin.next() {
            Some(start) => {
                match origin.next() {
                    Some(end) => Range { start: start, end: end },
                    None => panic!("No start!"),
                }
            },
            None => panic!("No start!"),
        }
    }
}

impl Password {
    fn new(value: u64) -> Password {
        Password {
            value,
        }
    }

    fn is_valid(&self) -> bool {
        self.contains_repeat() && self.never_decreases()
    }

    fn contains_repeat(&self) -> bool {
        self.value.to_string().chars()
            .duplicates()
            .peekable()
            .peek()
            .is_some()
    }

    fn never_decreases(&self) -> bool {
        let mut chars: Vec<char> = self.value.to_string().chars().collect();
        chars.sort();
        let pass = String::from_iter(chars);
        pass == self.value.to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn string_into_range() {
        let str_range = StringRange::new("2-5".to_string());
        let range: Range<u64> = str_range.into();

        assert_eq!(range, Range {start: 2, end: 5});
    }

    #[test]
    fn password_must_have_repeating_digit() {
        assert_eq!(false, Password::new(123).is_valid());
    }

    #[test]
    fn password_must_be_increasing() {
        assert!(Password::new(1123).never_decreases());
    }

    #[test]
    fn decreasing_password_is_rejected() {
        assert!(!Password::new(11231).never_decreases());
    }

    #[test]
    fn first_part() {
        let str_range = StringRange::new("248345-746315".to_string());
        let range: Range<u64> = str_range.into();
        let passwords = range
            .map(|n| Password::new(n))
            .filter(|p| p.is_valid())
            .count();

        assert_eq!(passwords, 1019);
    }
}