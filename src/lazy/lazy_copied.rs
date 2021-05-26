use crate::{Consumer, Iterable, IterableSeq};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyCopied<I> {
    pub(crate) iterable: I,
}

impl<'a, I, T> Iterable for LazyCopied<I>
where
    T: 'a + Copy,
    I: Iterable<Item = &'a T>,
{
    type C = I::CC<T>;
    type CC<U> = I::CC<U>;
    type F = I::CF<T>;
    type CF<U> = I::CF<U>;
}

impl<'a, I, T> IterableSeq for LazyCopied<I>
where
    T: 'a + Copy,
    I: IterableSeq<Item = &'a T>,
{
}

impl<'a, I, T> Consumer for LazyCopied<I>
where
    T: 'a + Copy,
    I: Consumer<Item = &'a T>,
{
    type Item = T;
    type IntoIter = std::iter::Copied<I::IntoIter>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_type;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![&1, &2, &3];
        let res = collect(v.lazy_copied());
        assert_eq!(res, vec![1, 2, 3]);
    }

    #[test]
    fn test_f() {
        let v = [&1, &2, &3];
        let res = v.lazy_copied().rev();
        assert_type::<[i32; 3]>(res);
    }

    #[test]
    fn test_cf() {
        let v = [&1, &2, &3];
        let res = v.lazy_copied().map(|x| x.to_string());
        assert_type::<[String; 3]>(res);
    }
}
