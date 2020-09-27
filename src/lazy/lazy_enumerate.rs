use crate::{Iterable, Consumer};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyEnumerate<I> {
    pub(crate) iterable: I,
}

impl<I> Iterable for LazyEnumerate<I>
where
    I: Iterable,
{
    type C = I::CC<(usize, I::Item)>;
    type CC<U> = I::CC<U>;
    type F = I::CF<(usize, I::Item)>;
    type CF<U> = I::CF<U>;
}

impl<I> Consumer for LazyEnumerate<I>
where
    I: Consumer,
{
    type Item = (usize, I::Item);
    type IntoIter = std::iter::Enumerate<I::IntoIter>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().enumerate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;
    use crate::assert_type;

    #[test]
    fn smoke() {
        let v = vec![1, 2, 3];
        let res = collect(v.lazy_enumerate());
        assert_eq!(res, vec![(0, 1), (1, 2), (2, 3)]);
    }

    #[test]
    fn test_f() {
        let v = [1, 2, 3];
        let res = v.lazy_enumerate().rev();
        assert_type::<[(usize, i32); 3]>(res);
    }

    #[test]
    fn test_cf() {
        let v = [1, 2, 3];
        let res = v.lazy_enumerate().map(|(_, s)| s.to_string());
        assert_type::<[String; 3]>(res);
    }
}
