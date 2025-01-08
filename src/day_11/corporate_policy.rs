#[derive(Clone, Debug, PartialEq, Eq)]
struct Password(Vec<char>);

impl Password {
    fn new(s: &str) -> Self {
        Password(s.chars().collect())
    }

    fn increment(&mut self) {
        let mut index = self.0.len() - 1;
        loop {
            if self.0[index] == 'z' {
                self.0[index] = 'a';
                index -= 1;
            } else {
                self.0[index] = (self.0[index] as u8 + 1) as char;
                break;
            }
        }
    }

    fn is_valid(&self) -> bool {
        let straight = one_increasing_straight(&self.0);
        let blacklisted = !contains_blacklisted(&self.0);
        let two_pairs = contains_two_pairs(&self.0);

        straight && blacklisted && two_pairs
    }

    fn next(&mut self) -> Self {
        loop {
            self.increment();
            if self.is_valid() {
                return self.clone();
            }
        }
    }

    fn to_string(&self) -> String {
        self.0.iter().collect()
    }
}

fn one_increasing_straight(chars: &[char]) -> bool {
    chars.windows(3).any(|window| {
        let a = window[0] as u8;
        let b = window[1] as u8;
        let c = window[2] as u8;

        a + 1 == b && b + 1 == c
    })
}

const BLACKLIST: [char; 3] = ['i', 'o', 'l'];

fn contains_blacklisted(chars: &[char]) -> bool {
    chars.iter().any(|char| {
        BLACKLIST.contains(&char)
    })
}

fn contains_two_pairs(chars: &[char]) -> bool {
    let mut pairs = 0;
    let mut index = 0;
    while index < chars.len() - 1 {
        if chars[index] == chars[index + 1] {
            pairs += 1;
            index += 2;
        } else {
            index += 1;
        }
    }

    pairs >= 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/year_2015/day_11/input.txt");

        let mut password = Password::new(input);

        assert_eq!(password.next().to_string(), "hepxxyzz");
    }

    #[test]
    fn test_incrementing() {
        let mut password = Password::new("azy");

        password.increment();
        assert_eq!(password, Password::new("azz"));

        password.increment();
        assert_eq!(password, Password::new("baa"));
    }

    #[test]
    fn test_increasing_straight() {
        assert!(!one_increasing_straight(&vec!['a', 'a', 'b', 'a']));
        assert!(one_increasing_straight(&vec!['a', 'a', 'b', 'c']));
    }

    #[test]
    fn test_blacklisting() {
        assert!(!contains_blacklisted(&vec!['a', 'a', 'b', 'a']));
        assert!(contains_blacklisted(&vec!['a', 'a', 'i', 'a']));
        assert!(contains_blacklisted(&vec!['a', 'a', 'o', 'a']));
        assert!(contains_blacklisted(&vec!['a', 'a', 'l', 'a']));
    }

    #[test]
    fn test_pairs() {
        assert!(!contains_two_pairs(&vec!['a', 'a', 'b', 'a']));
        assert!(!contains_two_pairs(&vec!['a', 'a', 'a', 'b']));
        assert!(contains_two_pairs(&vec!['a', 'a', 'a', 'a']));
        assert!(contains_two_pairs(&vec!['a', 'a', 'b', 'a', 'a']));
    }

    #[test]
    fn test_validity() {
        assert!(!Password::new("hijklmmn").is_valid());
        assert!(!Password::new("abbceffg").is_valid());
        assert!(!Password::new("abbcegjk").is_valid());
        assert!(!Password::new("abcdefgh").is_valid());
        assert!(!Password::new("ghijklmn").is_valid());
        assert!(Password::new("gghggjkmnpqrs").is_valid());
    }
}
