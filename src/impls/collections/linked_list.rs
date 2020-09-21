use std::collections::LinkedList;

use crate::Iterable;

impl<T> Iterable for LinkedList<T> {
    type C = Self;
    type CC<U> = LinkedList<U>;
    type CR<'a> where T: 'a = LinkedList<&'a T>;
}

delegate_into_iterator!(LinkedList<T>, impl <T>);
delegate_into_iterator!(&'a LinkedList<T>, impl <'a, T: 'a>);

delegate_from_iterator!(LinkedList<T>, T, impl <T>);

#[cfg(test)]
mod tests {
    use super::*;

    fn ll<T>(v: Vec<T>) -> LinkedList<T> {
        v.into_iter().collect()
    }

    #[test]
    fn test_c() {
        let v = ll(vec![1, 2, 3]);
        let res = v.filter(|i| i > &1);
        assert_eq!(res, ll(vec![2, 3]));
    }

    #[test]
    fn test_cc() {
        let v = ll(vec![1, 2, 3]);
        let res = v.map(|i| i.to_string());
        assert_eq!(res, ll(vec!["1".to_string(), "2".to_string(), "3".to_string()]));
    }

    #[test]
    fn test_c_r() {
        let v = ll(vec![1, 2, 3]);
        let res = (&v).filter(|i| i > &&1);
        assert_eq!(res, ll(vec![&2, &3]));
    }

    #[test]
    fn test_cc_r() {
        let v = ll(vec![1, 2, 3]);
        let res = (&v).map(|i| i.to_string());
        assert_eq!(res, ll(vec!["1".to_string(), "2".to_string(), "3".to_string()]));
    }
}