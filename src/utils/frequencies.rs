use std::collections::HashMap;

pub struct Frequencies {
    frequencies: HashMap<char, u32>,
}

impl Frequencies {
    pub fn get(&self, c: char) -> u32 {
        *self.frequencies.get(&c).unwrap_or(&0)
    }
}

pub fn frequencies(text: &str) -> Frequencies {
    let frequencies = text.chars()
        .fold(HashMap::new(), |mut acc, c| {
            let counter = acc.entry(c).or_insert(0);
            *counter += 1;
            acc
        });

    Frequencies { frequencies }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequencies() {
        let frequencies = frequencies("aabddd");

        assert_eq!(frequencies.get('a'), 2);
        assert_eq!(frequencies.get('b'), 1);
        assert_eq!(frequencies.get('c'), 0);
        assert_eq!(frequencies.get('d'), 3);
    }
}
