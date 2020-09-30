use crate::{Iterable, Consumer, IterableSeq};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyFilter<I, F> {
    pub(crate) iterable: I,
    pub(crate) f: F,
}

impl<I, F> Iterable for LazyFilter<I, F>
where
    I: Iterable,
    F: Fn(&I::Item) -> bool,
{
    type C = I::C;
    type CC<U> = I::CC<U>;
}

impl<I, F> IterableSeq for LazyFilter<I, F>
where
    I: IterableSeq,
    F: Fn(&I::Item) -> bool,
{
}

impl<I, F> Consumer for LazyFilter<I, F>
where
    I: Consumer,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;
    type IntoIter = std::iter::Filter<I::IntoIter, F>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().filter(self.f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![1, 2, 3];
        let res = collect(v.lazy_filter(|i| i > &1));
        assert_eq!(res, vec![2, 3]);
    }
}
