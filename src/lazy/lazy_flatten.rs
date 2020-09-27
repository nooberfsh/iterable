use crate::{Iterable, Consumer};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyFlatten<I> {
    pub(crate) iterable: I,
}

impl<I> Iterable for LazyFlatten<I>
where
    I: Iterable,
    I::Item: Consumer,
{
    type C = I::CC<<I::Item as Consumer>::Item>;
    type CC<U> = I::CC<U>;
}

impl<I> Consumer for LazyFlatten<I>
where
    I: Consumer,
    I::Item: Consumer,
{
    type Item = <I::Item as Consumer>::Item;
    type IntoIter = Iter<I::IntoIter>;
    fn consume(self) -> Self::IntoIter {
        new_iter(self.iterable)
    }
}

pub struct Iter<I>
where
    I: Iterator,
    I::Item: Consumer,
{
    iter: I,
    inner: Option<<I::Item as Consumer>::IntoIter>,
}

pub (super) fn new_iter<C>(c: C) -> Iter<C::IntoIter>
where
    C: Consumer,
    C::Item: Consumer,
{
    Iter {
        iter: c.consume(),
        inner: None,
    }
}

impl<I> Iterator for Iter<I>
where
    I: Iterator,
    I::Item: Consumer,
{
    type Item = <I::Item as Consumer>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.take() {
            None => {
                match self.iter.next() {
                    None => None,
                    Some(d) => {
                        self.inner = Some(d.consume());
                        self.next()
                    }
                }
            }
            Some(mut i) => {
                match i.next() {
                    None => self.next(),
                    d =>  {
                        self.inner = Some(i);
                        d
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![[1,2], [3,4]];
        let res = collect(v.lazy_flatten());
        assert_eq!(res, vec![1, 2, 3, 4]);
    }
}
