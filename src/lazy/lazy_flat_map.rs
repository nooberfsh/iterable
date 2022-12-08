use crate::{Consumer, FlattenIter, Iterable, IterableSeq};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyFlatMap<I, F> {
    pub(crate) iterable: I,
    pub(crate) f: F,
}

impl<I, F, T> Iterable for LazyFlatMap<I, F>
where
    I: Iterable,
    F: Fn(I::Item) -> T,
    T: Consumer,
{
    type C = I::CC<<T as Consumer>::Item>;
    type CC<U> = I::CC<U>;
    // remove below after `associated_type_defaults` stabilized
    type F = I::CC<<T as Consumer>::Item>;
    type CF<U> = I::CC<U>;
}

impl<I, F, T> IterableSeq for LazyFlatMap<I, F>
where
    I: IterableSeq,
    F: Fn(I::Item) -> T,
    T: Consumer,
{
}

impl<I, F, T> Consumer for LazyFlatMap<I, F>
where
    I: Consumer,
    F: Fn(I::Item) -> T,
    T: Consumer,
{
    type Item = <T as Consumer>::Item;
    type IntoIter = FlattenMapIter<I::IntoIter, F, T>;
    fn consume(self) -> Self::IntoIter {
        new_flat_map_iter(self.iterable, self.f)
    }
}

pub struct FlattenMapIter<I, F, T>
where
    I: Iterator,
    F: Fn(I::Item) -> T,
    T: Consumer,
{
    inner: FlattenIter<std::iter::Map<I, F>>,
}

fn new_flat_map_iter<C, F, T>(c: C, f: F) -> FlattenMapIter<C::IntoIter, F, T>
where
    C: Consumer,
    F: Fn(C::Item) -> T,
    T: Consumer,
{
    let inner = FlattenIter {
        iter: c.consume().map(f),
        inner: None,
    };
    FlattenMapIter { inner }
}

impl<I, F, T> Iterator for FlattenMapIter<I, F, T>
where
    I: Iterator,
    F: Fn(I::Item) -> T,
    T: Consumer,
{
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![1, 2, 3];
        let res = collect(v.lazy_flat_map(|i| vec![i, 1]));
        assert_eq!(res, vec![1, 1, 2, 1, 3, 1]);
    }

    #[test]
    fn test_iter() {
        let a = new_flat_map_iter(vec![1, 2, 3], |x| vec![x, 1]);
        let res: Vec<_> = a.collect();
        assert_eq!(res, vec![1, 1, 2, 1, 3, 1])
    }

    #[test]
    fn test_iter2() {
        let a = new_flat_map_iter(vec![1, 2, 3], |x| if x == 2 { vec![] } else { vec![x, 1] });
        let res: Vec<_> = a.collect();
        assert_eq!(res, vec![1, 1, 3, 1])
    }

    #[test]
    fn test_iter3() {
        let a = new_flat_map_iter(vec![], |x: i32| vec![x, 1]);
        let res: Vec<_> = a.collect();
        assert_eq!(res, vec![]);
    }
}
