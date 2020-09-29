use crate::{Iterable, Consumer};
use std::marker::PhantomData;

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyCloned<'a, I, T> {
    pub(crate) iterable: I,
    pub(crate) _marker: PhantomData<&'a T>,
}

impl<'a, I, T> Iterable for LazyCloned<'a, I, T>
where
    T: 'a + Clone,
    I: Iterable<Item = &'a T>,
{
    type C = I::CC<T>;
    type CC<U> = I::CC<U>;
    type F = I::CF<T>;
    type CF<U> = I::CF<U>;
}

impl<'a, I, T> Consumer for LazyCloned<'a, I, T>
where
    T: 'a + Clone,
    I: Consumer<Item = &'a T>,
{
    type Item = T;
    type IntoIter = std::iter::Cloned<I::IntoIter>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;
    use crate::assert_type;

    #[test]
    fn smoke() {
        let v = vec![&1, &2, &3];
        let res = collect(v.lazy_cloned());
        assert_eq!(res, vec![1, 2, 3]);
    }

    #[test]
    fn test_f() {
        let v = [&1, &2, &3];
        let res = v.lazy_cloned().rev();
        assert_type::<[i32; 3]>(res);
    }

    #[test]
    fn test_cf() {
        let v = [&1, &2, &3];
        let res = v.lazy_cloned().map(|x| x.to_string());
        assert_type::<[String; 3]>(res);
    }
}
