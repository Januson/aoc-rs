use std::str::FromStr;
use crate::utils::frequencies::frequencies;
use crate::utils::partition::partition;

fn contains_vowels(s: &str, n: u32) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let frequencies = frequencies(s);

    let total_vowels: u32 = vowels.iter()
        .map(|vowel| frequencies.get(*vowel))
        .sum();

    total_vowels >= n
}

fn contains_repeated_letter(s: &str, n: u32) -> bool {
    let chars: Vec<_> = s.chars().collect();
    let found_repeated_char = partition(&chars, n as usize, 1).into_iter()
        .map(|pair| pair[0] == pair[1])
        .any(|x| x);

    found_repeated_char
}

fn contains_blacklisted(s: &str) -> bool {
    let blacklist = ["ab", "cd", "pq", "xy"];

    blacklist.iter()
        .any(|blacklisted| s.contains(blacklisted))
}

struct NaughtyList {
    names: Vec<String>,
}

impl FromStr for NaughtyList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let names = s.lines()
            .map(|line| line.to_string())
            .collect();

        Ok(NaughtyList { names })
    }
}

impl NaughtyList {
    fn nice(&self) -> Vec<&str> {
        self.names.iter()
            .filter(|name| contains_vowels(name, 3))
            .filter(|name| contains_repeated_letter(name, 2))
            .filter(|name| !contains_blacklisted(name))
            .map(|name| name.as_str())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/year_2015/day_05/input.txt");

        let naughty_list = NaughtyList::from_str(input).unwrap();

        assert_eq!(naughty_list.nice().len(), 255);
    }

    #[test]
    fn test_contains_vowels() {
        assert!(contains_vowels("aei", 1));
        assert!(contains_vowels("aei", 3));
        assert!(contains_vowels("xazegov", 3));
        assert!(contains_vowels("aeiouaeiouaeiou", 3));

        assert!(!contains_vowels("aa", 3));
        assert!(!contains_vowels("xazegov", 5));
    }

    #[test]
    fn test_contains_repeated_letter() {
        assert!(contains_repeated_letter("xx", 2));
        assert!(contains_repeated_letter("abcdde", 2));
        assert!(contains_repeated_letter("aabbccdd", 2));
        assert!(contains_repeated_letter("aabbccddd", 3));
    }

    #[test]
    fn test_contains_blacklisted_sequence() {
        assert!(contains_blacklisted("xxy"));
    }

    #[test]
    fn test_nice_detection() {
        let strings = vec![
            "ugknbfddgicrmopn".to_string(),
            "aaa".to_string(),
            "jchzalrnumimnmhp".to_string(),
            "haegwjzuvuyypxyu".to_string(),
            "dvszwmarrgswjxmb".to_string(),
        ];

        let naughty_list = NaughtyList { names: strings };

        assert_eq!(naughty_list.nice(), vec!["ugknbfddgicrmopn", "aaa"]);
    }
}
