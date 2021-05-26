use crate::{Consumer, Iterable, IterableSeq};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazySkip<I> {
    pub(crate) iterable: I,
    pub(crate) n: usize,
}

impl<I> Iterable for LazySkip<I>
where
    I: Iterable,
{
    type C = I::C;
    type CC<U> = I::CC<U>;
}

impl<I> IterableSeq for LazySkip<I> where I: IterableSeq {}

impl<I> Consumer for LazySkip<I>
where
    I: Consumer,
{
    type Item = I::Item;
    type IntoIter = std::iter::Skip<I::IntoIter>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().skip(self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![1, 2, 3];
        let res = collect(v.lazy_skip(1));
        assert_eq!(res, vec![2, 3]);
    }
}
