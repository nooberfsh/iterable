use std::collections::BinaryHeap;

use crate::{Iterable, GrowableProducer};

impl<T> Iterable for BinaryHeap<T> {
    type C = BinaryHeap<T>;
    type CC<U> = BinaryHeap<U>;

    fn add_one(mut self, a: Self::Item) -> Self::C
    where
        Self::C: GrowableProducer<Self::Item>
    {
        self.grow_one(a);
        self
    }
}

impl<'a, T: 'a> Iterable for &'a BinaryHeap<T> {
    type C = BinaryHeap<&'a T>;
    type CC<U> = BinaryHeap<U>;
}

delegate_into_iterator!(BinaryHeap<T>, impl <T>);
delegate_into_iterator!(&'a BinaryHeap<T>, impl <'a, T: 'a>);

delegate_from_iterator!(BinaryHeap<T>, T, impl <T: Ord>);
delegate_extend!(BinaryHeap<T>, T, impl <T: Ord>);

#[cfg(test)]
mod tests {
    use super::*;

    fn bh<T: Ord>(v: Vec<T>) -> BinaryHeap<T> {
        v.into_iter().collect()
    }

    #[test]
    fn test_c() {
        let v = bh(vec![1, 2, 3]);
        let res = v.filter(|i| i > &1).into_sorted_vec();
        assert_eq!(res, vec![2, 3]);
    }

    #[test]
    fn test_cc() {
        let v = bh(vec![1, 2, 3]);
        let res = v.map(|i| i.to_string()).into_sorted_vec();
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    #[test]
    fn test_c_r() {
        let v = bh(vec![1, 2, 3]);
        let res = (&v).filter(|i| i > &&1).into_sorted_vec();
        assert_eq!(res, vec![&2, &3]);
    }

    #[test]
    fn test_cc_r() {
        let v = bh(vec![1, 2, 3]);
        let res = (&v).map(|i| i.to_string()).into_sorted_vec();
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }
}
