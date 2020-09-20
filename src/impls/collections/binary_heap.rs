use std::collections::BinaryHeap;

use crate::Iterable;

impl<T> Iterable for BinaryHeap<T> {
    type C = Self;
    type CC<U> = BinaryHeap<U>;
    type CR<'a> where T: 'a = BinaryHeap<&'a T>;
}

delegate_into_iterator!(BinaryHeap<T>, impl <T>);
delegate_into_iterator!(&'a BinaryHeap<T>, impl <'a, T: 'a>);

delegate_from_iterator!(BinaryHeap<T>, T, impl <T: Ord>);

#[cfg(test)]
mod tests {
    use super::*;

    fn bh(v: Vec<i32>) -> BinaryHeap<i32> {
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
