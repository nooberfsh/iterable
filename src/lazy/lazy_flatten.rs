use crate::{Consumer, Iterable, IterableSeq};

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
    // remove below after `associated_type_defaults` stabilized
    type F = I::CC<<I::Item as Consumer>::Item>;
    type CF<U> = I::CC<U>;
}

impl<I> IterableSeq for LazyFlatten<I>
where
    I: IterableSeq,
    I::Item: Consumer,
{
}

impl<I> Consumer for LazyFlatten<I>
where
    I: Consumer,
    I::Item: Consumer,
{
    type Item = <I::Item as Consumer>::Item;
    type IntoIter = FlattenIter<I::IntoIter>;
    fn consume(self) -> Self::IntoIter {
        new_flatten_iter(self.iterable)
    }
}

pub struct FlattenIter<I>
where
    I: Iterator,
    I::Item: Consumer,
{
    pub(super) iter: I,
    pub(super) inner: Option<<I::Item as Consumer>::IntoIter>,
}

pub(super) fn new_flatten_iter<C>(c: C) -> FlattenIter<C::IntoIter>
where
    C: Consumer,
    C::Item: Consumer,
{
    FlattenIter {
        iter: c.consume(),
        inner: None,
    }
}

impl<I> Iterator for FlattenIter<I>
where
    I: Iterator,
    I::Item: Consumer,
{
    type Item = <I::Item as Consumer>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.take() {
            None => match self.iter.next() {
                None => None,
                Some(d) => {
                    self.inner = Some(d.consume());
                    self.next()
                }
            },
            Some(mut i) => match i.next() {
                None => self.next(),
                d => {
                    self.inner = Some(i);
                    d
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![[1, 2], [3, 4]];
        let res = collect(v.lazy_flatten());
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_iter() {
        let a = new_flatten_iter(vec![[1, 1], [2, 1], [3, 1]]);
        let res: Vec<_> = a.collect();
        assert_eq!(res, vec![1, 1, 2, 1, 3, 1])
    }

    #[test]
    fn test_iter2() {
        let a = new_flatten_iter(vec![vec![1, 1], vec![], vec![3, 1]]);
        let res: Vec<_> = a.collect();
        assert_eq!(res, vec![1, 1, 3, 1])
    }

    #[test]
    fn test_iter3() {
        let e: Vec<[i32; 10]> = vec![];
        let a = new_flatten_iter(e);
        let res: Vec<_> = a.collect();
        assert_eq!(res, vec![]);
    }
}
