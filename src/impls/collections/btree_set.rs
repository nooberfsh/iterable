use std::collections::BTreeSet;

use crate::{Iterable, GrowableProducer};

impl<T> Iterable for BTreeSet<T> {
    type C = Self;
    type CC<U> = BTreeSet<U>;

    fn add_one(mut self, a: Self::Item) -> Self::C
    where
        Self::C: GrowableProducer<Self::Item>
    {
        self.grow_one(a);
        self
    }
}

impl<'a, T: 'a> Iterable for &'a BTreeSet<T> {
    type C = BTreeSet<&'a T>;
    type CC<U> = BTreeSet<U>;
}

delegate_into_iterator!(BTreeSet<T>, impl <T>);
delegate_into_iterator!(&'a BTreeSet<T>, impl <'a, T: 'a>);

delegate_from_iterator!(BTreeSet<T>, T, impl <T: Ord>);
delegate_extend!(BTreeSet<T>, T, impl <T: Ord>);

#[cfg(test)]
mod tests {
    use maplit::*;

    use super::*;

    #[test]
    fn test_c() {
        let v = btreeset![1, 2, 3];
        let res = v.filter(|i| i > &1);
        assert_eq!(res, btreeset![2, 3]);
    }

    #[test]
    fn test_cc() {
        let v = btreeset![1, 2, 3];
        let res = v.map(|i| i.to_string());
        assert_eq!(
            res,
            btreeset!["1".to_string(), "2".to_string(), "3".to_string()]
        );
    }

    #[test]
    fn test_c_r() {
        let v = btreeset![1, 2, 3];
        let res = (&v).filter(|i| i > &&1);
        assert_eq!(res, btreeset![&2, &3]);
    }

    #[test]
    fn test_cc_r() {
        let v = btreeset![1, 2, 3];
        let res = (&v).map(|i| i.to_string());
        assert_eq!(
            res,
            btreeset!["1".to_string(), "2".to_string(), "3".to_string()]
        );
    }
}
