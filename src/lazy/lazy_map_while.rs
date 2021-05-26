use crate::{Consumer, Iterable, IterableSeq};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyMapWhile<I, F> {
    pub(crate) iterable: I,
    pub(crate) f: F,
}

impl<I, F, T> Iterable for LazyMapWhile<I, F>
where
    I: Iterable,
    F: Fn(I::Item) -> Option<T>,
{
    type C = I::CC<T>;
    type CC<U> = I::CC<U>;
}

impl<I, F, T> IterableSeq for LazyMapWhile<I, F>
where
    I: IterableSeq,
    F: Fn(I::Item) -> Option<T>,
{
}

impl<I, F, T> Consumer for LazyMapWhile<I, F>
where
    I: Consumer,
    F: Fn(I::Item) -> Option<T>,
{
    type Item = T;
    type IntoIter = std::iter::MapWhile<I::IntoIter, F>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().map_while(self.f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![1, 2, 3];
        let res = collect(v.lazy_map_while(|x| if x == 2 { None } else { Some(x) }));
        assert_eq!(res, vec![1]);
    }
}
