use crate::{Iterable, Consumer};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyMap<I, F> {
    pub(crate) iterable: I,
    pub(crate) f: F,
}

impl<I, F, T> Iterable for LazyMap<I, F>
where
    I: Iterable,
    F: Fn(I::Item) -> T,
{
    type C = I::CC<T>;
    type CC<U> = I::CC<U>;
    type F = I::CF<T>;
    type CF<U> = I::CF<U>;
}

impl<I, F, T> Consumer for LazyMap<I, F>
where
        I: Consumer,
        F: Fn(I::Item) -> T,
{
    type Item = T;
    type IntoIter = std::iter::Map<I::IntoIter, F>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().map(self.f)
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
        let res = collect(v.lazy_map(|i| i.to_string()));
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    #[test]
    fn test_f() {
        let v = [1, 2, 3];
        let res = v.lazy_map(|i| i.to_string()).rev();
        assert_type::<[String; 3]>(res);
    }

    #[test]
    fn test_cf() {
        let v = [1, 2, 3];
        let res = v.lazy_map(|i| i.to_string()).map(|s| s.parse::<u32>().unwrap());
        assert_type::<[u32; 3]>(res);
    }
}
