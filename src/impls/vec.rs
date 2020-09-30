use std::cmp::Ordering;

use crate::{Iterable, IterableSeq};

impl<T> Iterable for Vec<T> {
    type C = Self;
    type CC<U> = Vec<U>;

    fn rev(mut self) -> Self::F {
        self.reverse();
        self
    }
}

impl<T> IterableSeq for Vec<T> {
    fn sorted(mut self) -> Self::F
    where
        T: Ord
    {
        self.sort();
        self
    }

    fn sorted_by<F>(mut self, f: F) -> Self::F
    where
        F: Fn(&Self::Item, &Self::Item) -> Ordering,
    {
        self.sort_by(f);
        self
    }

    fn sorted_by_key<K, F>(mut self, f: F) -> Self::F
    where
        K: Ord,
        F: Fn(&Self::Item) -> K,
    {
        self.sort_by_key(f);
        self
    }
}

impl<'a, T: 'a> Iterable for &'a Vec<T> {
    type C = Vec<&'a T>;
    type CC<U> = Vec<U>;
}

impl<'a, T: 'a> IterableSeq for &'a Vec<T> {}

delegate_into_iterator!(Vec<T>, impl <T>);
delegate_into_iterator!(&'a Vec<T>, impl <'a, T: 'a>);

delegate_from_iterator!(Vec<T>, T, impl <T>);
delegate_extend!(Vec<T>, T, impl <T>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        let v = vec![1, 2, 3];
        let res = v.filter(|i| i > &1);
        assert_eq!(res, vec![2, 3]);
    }

    #[test]
    fn test_cc() {
        let v = vec![1, 2, 3];
        let res = v.map(|i| i.to_string());
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    #[test]
    fn test_c_r() {
        let v = vec![1, 2, 3];
        let res = (&v).filter(|i| i > &&1);
        assert_eq!(res, vec![&2, &3]);
    }

    #[test]
    fn test_cc_r() {
        let v = vec![1, 2, 3];
        let res = (&v).map(|i| i.to_string());
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }
}
