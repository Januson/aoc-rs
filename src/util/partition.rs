use std::cmp::Eq;
use std::hash::Hash;

pub trait PartitionedIterator<E>: Iterator<Item=E> + Sized where E: Eq + Hash {
    fn partitioned(self, n: u64) -> Partitioned<Self, E> {
        Partitioned {
            iter: self,
            size: n,
        }
    }
}

impl<E, It> PartitionedIterator<E> for It where It: Iterator<Item=E>, E: Eq + Hash {}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Partitioned<It, E> where It: Iterator<Item=E>, E: Eq {
    iter: It,
    size: u64,
}

impl<It, E> Iterator for Partitioned<It, E> where It: Iterator<Item=E>, E: Clone + Eq + Hash {
    type Item = Vec<E>;

    fn next(&mut self) -> Option<Self::Item>{
        let vec: Vec<E> = self.iter.by_ref()
            .take(self.size as usize)
            .collect();
        match &vec.len() {
            0 => None,
            _ => Some(vec)
        }
    }
}

#[test]
fn test_partitions() {
    let v = vec![0, 1, 2, 3, 4, 5];
    let r: Vec<Vec<i32>> = v.into_iter().partitioned(2).collect();
    assert_eq!(r, vec![vec![0, 1], vec![2, 3], vec![4, 5]]);
}

//#[test]
//fn test_duplicates_unwrap() {
//    let v = vec![0usize, 0, 1, 1, 2, 3, 3, 3, 4, 5];
//    let mut i = v.into_iter().duplicates();
//    assert_eq!(i.next(), Some(0));
//    assert_eq!(i.next(), Some(1));
//    assert_eq!(i.next(), Some(3));
//    let r: Vec<_> = i.unwrap().collect();
//    assert_eq!(r, vec![3, 4, 5]);
//}