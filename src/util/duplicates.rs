use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;

pub trait DuplicateIterator<E>: Iterator<Item=E> + Sized where E: Eq + Hash {
    fn duplicates(self) -> Duplicates<Self, E> {
        Duplicates {
            iter: self,
            seen: HashSet::new(),
        }
    }
}

impl<E, It> DuplicateIterator<E> for It where It: Iterator<Item=E>, E: Eq + Hash {}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Duplicates<It, E> where It: Iterator<Item=E>, E: Eq {
    iter: It,
    seen: HashSet<E>,
}

impl<It, E> Duplicates<It, E> where It: Iterator<Item=E>, E: Eq {
    pub fn unwrap(self) -> It {
        let Duplicates { iter, .. } = self;
        iter
    }
}

impl<It, E> Iterator for Duplicates<It, E> where It: Iterator<Item=E>, E: Clone + Eq + Hash {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next();
            match next {
                Some(next) => {
                    if self.seen.contains(&next) {
                        break Some(next)
                    }
                    self.seen.insert(next);
                    continue;
                },
                None => break next,
            };
        }
    }
}

#[test]
fn test_duplicates() {
    let v = vec![0usize, 1, 2, 2, 4, 0];
    let r: Vec<_> = v.into_iter().duplicates().collect();
    assert_eq!(r, vec![2, 0]);
}

#[test]
fn test_duplicates_unwrap() {
    let v = vec![0usize, 0, 1, 1, 2, 3, 3, 3, 4, 5];
    let mut i = v.into_iter().duplicates();
    assert_eq!(i.next(), Some(0));
    assert_eq!(i.next(), Some(1));
    assert_eq!(i.next(), Some(3));
    let r: Vec<_> = i.unwrap().collect();
    assert_eq!(r, vec![3, 4, 5]);
}