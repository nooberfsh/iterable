use crate::{Consumer, Iterable, IterableSeq};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyZip<I, C> {
    pub(crate) iterable: I,
    pub(crate) c: C,
}

impl<I, C> Iterable for LazyZip<I, C>
where
    I: Iterable,
    C: Consumer,
{
    type C = I::CC<(I::Item, C::Item)>;
    type CC<U> = I::CC<U>;
    // remove below after `associated_type_defaults` stabilized
    type F = I::CC<(I::Item, C::Item)>;
    type CF<U> = I::CC<U>;
}

impl<I, C> IterableSeq for LazyZip<I, C>
where
    I: IterableSeq,
    C: Consumer,
{
}

impl<I, C> Consumer for LazyZip<I, C>
where
    I: Consumer,
    C: Consumer,
{
    type Item = (I::Item, C::Item);
    type IntoIter = std::iter::Zip<I::IntoIter, C::IntoIter>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().zip(self.c.consume())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![1, 2, 3];
        let s = vec!['a', 'b'];
        let res = collect(v.lazy_zip(s));
        assert_eq!(res, vec![(1, 'a'), (2, 'b')]);
    }
}
