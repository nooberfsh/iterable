use crate::{Iterable, Consumer};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazySkipWhile<I, F> {
    pub(crate) iterable: I,
    pub(crate) f: F,
}

impl<I, F> Iterable for LazySkipWhile<I, F>
where
    I: Iterable,
    F: Fn(&I::Item) -> bool,
{
    type C = I::C;
    type CC<U> = I::CC<U>;
}

impl<I, F> Consumer for LazySkipWhile<I, F>
where
    I: Consumer,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;
    type IntoIter = std::iter::SkipWhile<I::IntoIter, F>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().skip_while(self.f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![1, 2, 3];
        let res = collect(v.lazy_skip_while(|i| i < &3));
        assert_eq!(res, vec![3]);
    }
}
