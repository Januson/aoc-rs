use std::collections::HashMap;
use std::hash::Hash;

pub struct Frequencies<T> {
    frequencies: HashMap<T, u32>,
}

impl<T> Frequencies<T>
where
    T: Eq + Hash,
{
    pub fn get(&self, c: T) -> u32 {
        *self.frequencies.get(&c).unwrap_or(&0)
    }
}

pub fn frequencies<I, T>(input: I) -> Frequencies<T>
where
    I: IntoIterator<Item = T>,
    T: Eq + Hash,
{
    let frequencies = input
        .into_iter()
        .fold(HashMap::new(), |mut acc, c| {
            let counter = acc.entry(c).or_insert(0);
            *counter += 1;
            acc
        });

    Frequencies { frequencies }
}

impl<T> IntoIterator for Frequencies<T>
where
    T: Eq + Hash,
{
    type Item = (T, u32);
    type IntoIter = std::collections::hash_map::IntoIter<T, u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.frequencies.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Frequencies<T>
where
    T: Eq + Hash,
{
    type Item = (&'a T, &'a u32);
    type IntoIter = std::collections::hash_map::Iter<'a, T, u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.frequencies.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequencies() {
        let frequencies = frequencies("aabddd".chars());

        assert_eq!(frequencies.get('a'), 2);
        assert_eq!(frequencies.get('b'), 1);
        assert_eq!(frequencies.get('c'), 0);
        assert_eq!(frequencies.get('d'), 3);
    }
}
