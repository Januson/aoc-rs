use std::mem::replace;

pub trait AccumulateIterator<E>: Iterator<Item=E> + Sized {
    fn accumulate<F: FnMut(E, E) -> E>(self, f: F) -> Accumulate<Self, E, F> {
        Accumulate {
            iter: self,
            f,
            accum: None,
        }
    }
}

impl<E, It> AccumulateIterator<E> for It where It: Iterator<Item=E> {}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Accumulate<It, E, F> where It: Iterator<Item=E> {
    iter: It,
    f: F,
    accum: Option<E>,
}

impl<It, E, F> Accumulate<It, E, F> where It: Iterator<Item=E> {
    pub fn unwrap(self) -> It {
        let Accumulate { iter, .. } = self;
        iter
    }
}

impl<It, E, F> Iterator for Accumulate<It, E, F> where It: Iterator<Item=E>, F: FnMut(E, E) -> E, E: Clone {
    type Item = E;

    fn next(&mut self) -> Option<E> {
        match replace(&mut self.accum, None) {
            None => match self.iter.next() {
                None => None,
                e @ _ => {
                    self.accum = e;
                    self.accum.clone()
                }
            },
            Some(accum) => match self.iter.next() {
                None => {
                    self.accum = None;
                    None
                },
                Some(rhs) => {
                    self.accum = Some((self.f)(accum, rhs));
                    self.accum.clone()
                }
            }
        }
    }
}

#[test]
fn test_accumulate() {
    let v = vec![0usize, 1, 2, 3, 4];
    let r: Vec<_> = v.into_iter().accumulate(|a,b| a+b).collect();
    assert_eq!(r, vec![0, 1, 3, 6, 10]);
}

#[test]
fn test_accumulate_unwrap() {
    let v = vec![0usize, 1, 2, 3, 4];
    let mut i = v.into_iter().accumulate(|a,b| a+b);
    assert_eq!(i.next(), Some(0));
    assert_eq!(i.next(), Some(1));
    assert_eq!(i.next(), Some(3));
    let r: Vec<_> = i.unwrap().collect();
    assert_eq!(r, vec![3, 4]);
}