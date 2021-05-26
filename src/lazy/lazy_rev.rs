use crate::{Consumer, Iterable, IterableSeq};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyRev<I> {
    pub(crate) iterable: I,
}

impl<I> Iterable for LazyRev<I>
where
    I: Iterable,
    I::IntoIter: DoubleEndedIterator,
{
    type C = I::C;
    type CC<U> = I::CC<U>;
    type F = I::F;
    type CF<U> = I::CF<U>;
}

impl<I> IterableSeq for LazyRev<I>
where
    I: IterableSeq,
    I::IntoIter: DoubleEndedIterator,
{
}

impl<I> Consumer for LazyRev<I>
where
    I: Consumer,
    I::IntoIter: DoubleEndedIterator,
{
    type Item = I::Item;
    type IntoIter = std::iter::Rev<I::IntoIter>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_type;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![1, 2, 3];
        let res = collect(v.lazy_rev());
        assert_eq!(res, vec![3, 2, 1]);
    }

    #[test]
    fn test_f() {
        let v = [1, 2, 3];
        let res = v.lazy_rev().rev();
        assert_type::<[i32; 3]>(res.clone());
        assert_eq!(res, [1, 2, 3]);
    }

    #[test]
    fn test_cf() {
        let v = [1, 2, 3];
        let res = v.lazy_rev().map(|x| x.to_string());
        assert_type::<[String; 3]>(res);
    }
}
