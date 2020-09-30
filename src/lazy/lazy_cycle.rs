use crate::{Iterable, Consumer, IterableSeq};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyCycle<I> {
    pub(crate) iterable: I,
}

impl<I> Iterable for LazyCycle<I>
where
    I: Iterable,
    I::IntoIter: Clone,
{
    type C = I::C;
    type CC<U> = I::CC<U>;
}

impl<I> IterableSeq for LazyCycle<I>
where
    I: IterableSeq,
    I::IntoIter: Clone,
{
}

impl<I> Consumer for LazyCycle<I>
where
    I: Consumer,
    I::IntoIter: Clone,
{
    type Item = I::Item;
    type IntoIter = std::iter::Cycle<I::IntoIter>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().cycle()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let v = vec![1, 2, 3];
        let res = v.lazy_cycle().take(6);
        assert_eq!(res, vec![1, 2, 3, 1, 2, 3]);
    }
}
