use crate::{Consumer, Iterable, IterableSeq};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyChain<I, C> {
    pub(crate) iterable: I,
    pub(crate) c: C,
}

impl<I, C> Iterable for LazyChain<I, C>
where
    I: Iterable,
    C: Consumer<Item = I::Item>,
{
    type C = I::C;
    type CC<U> = I::CC<U>;
    // remove below after `associated_type_defaults` stabilized
    type F = I::C;
    type CF<U> = I::CC<U>;
}

impl<I, C> IterableSeq for LazyChain<I, C>
where
    I: IterableSeq,
    C: Consumer<Item = I::Item>,
{
}

impl<I, C> Consumer for LazyChain<I, C>
where
    I: Consumer,
    C: Consumer<Item = I::Item>,
{
    type Item = I::Item;
    type IntoIter = std::iter::Chain<I::IntoIter, C::IntoIter>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().chain(self.c.consume())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![1, 2, 3];
        let s = vec![4, 5, 6];
        let res = collect(v.lazy_chain(s));
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6]);
    }
}
