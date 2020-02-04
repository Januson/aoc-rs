use std::ops::Range;
use std::iter::FromIterator;
use crate::util::duplicates::DuplicateIterator;
use crate::util::accumulate::AccumulateIterator;

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

trait Validation {
    fn is_valid(&self, password: u64) -> bool;
}

struct NeverDecreases;

impl NeverDecreases {
    fn new() -> NeverDecreases {
        NeverDecreases {}
    }
}

impl Validation for NeverDecreases {
    fn is_valid(&self, password: u64) -> bool {
        let mut chars: Vec<char> = password.to_string().chars().collect();
        chars.sort();
        let pass = String::from_iter(chars);
        pass == password.to_string()
    }
}

struct ContainsRepeat;

impl ContainsRepeat {
    fn new() -> ContainsRepeat {
        ContainsRepeat {}
    }
}

impl Validation for ContainsRepeat {
    fn is_valid(&self, password: u64) -> bool {
        password.to_string().chars()
            .duplicates()
            .peekable()
            .peek()
            .is_some()
    }
}

struct LimitRepeat {
    limit: u8,
}

impl LimitRepeat {
    fn new(limit: u8) -> LimitRepeat {
        LimitRepeat {
            limit
        }
    }
}

impl Validation for LimitRepeat {
    fn is_valid(&self, password: u64) -> bool {
        password.to_string().chars()
            .duplicates()
            .peekable()
            .peek()
            .is_some()
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
        assert_eq!(false, ContainsRepeat::new().is_valid(123));
    }

    #[test]
    fn password_must_be_increasing() {
        assert!(NeverDecreases::new().is_valid(1123));
    }

    #[test]
    fn decreasing_password_is_rejected() {
        assert_eq!(false, NeverDecreases::new().is_valid(11231));
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