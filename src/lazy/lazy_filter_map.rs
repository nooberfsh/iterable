use crate::{Iterable, Consumer};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyFilterMap<I, F> {
    pub(crate) iterable: I,
    pub(crate) f: F,
}

impl<I, F, T> Iterable for LazyFilterMap<I, F>
where
    I: Iterable,
    F: Fn(I::Item) -> Option<T>,
{
    type C = I::CC<T>;
    type CC<U> = I::CC<U>;
}

impl<I, F, T> Consumer for LazyFilterMap<I, F>
where
    I: Consumer,
    F: Fn(I::Item) -> Option<T>,
{
    type Item = T;
    type IntoIter = std::iter::FilterMap<I::IntoIter, F>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().filter_map(self.f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![1, 2, 3];
        let res = collect(v.lazy_filter_map(|i| if i > 1 { Some(i.to_string()) } else { None }));
        assert_eq!(res, vec![2.to_string(), 3.to_string()]);
    }
}
