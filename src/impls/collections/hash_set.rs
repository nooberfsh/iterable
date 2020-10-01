use std::collections::HashSet;
use std::hash::Hash;

use crate::{Iterable, GrowableProducer};

impl<T> Iterable for HashSet<T> {
    type C = Self;
    type CC<U> = HashSet<U>;

    fn add_one(mut self, a: Self::Item) -> Self::C
    where
        Self::C: GrowableProducer<Self::Item>
    {
        self.grow_one(a);
        self
    }
}

impl<'a, T> Iterable for &'a HashSet<T> {
    type C = HashSet<&'a T>;
    type CC<U> = HashSet<U>;
}

delegate_into_iterator!(HashSet<T>, impl <T>);
delegate_into_iterator!(&'a HashSet<T>, impl <'a, T: 'a>);

delegate_from_iterator!(HashSet<T>, T, impl <T: Eq + Hash>);
delegate_extend!(HashSet<T>, T, impl <T: Eq + Hash>);

#[cfg(test)]
mod tests {
    use maplit::*;

    use super::*;

    #[test]
    fn test_c() {
        let v = hashset![1, 2, 3];
        let res = v.filter(|i| i > &1);
        assert_eq!(res, hashset![2, 3]);
    }

    #[test]
    fn test_cc() {
        let v = hashset![1, 2, 3];
        let res = v.map(|i| i.to_string());
        assert_eq!(
            res,
            hashset!["1".to_string(), "2".to_string(), "3".to_string()]
        );
    }

    #[test]
    fn test_c_r() {
        let v = hashset![1, 2, 3];
        let res = (&v).filter(|i| i > &&1);
        assert_eq!(res, hashset![&2, &3]);
    }

    #[test]
    fn test_cc_r() {
        let v = hashset![1, 2, 3];
        let res = (&v).map(|i| i.to_string());
        assert_eq!(
            res,
            hashset!["1".to_string(), "2".to_string(), "3".to_string()]
        );
    }
}
