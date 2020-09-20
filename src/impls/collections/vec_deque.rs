use std::collections::VecDeque;

use crate::Iterable;

impl<T> Iterable for VecDeque<T> {
    type C = Self;
    type CC<U> = VecDeque<U>;
    type CR<'a> where T: 'a = VecDeque<&'a T>;
}

delegate_into_iterator!(VecDeque<T>, impl <T>);
delegate_into_iterator!(&'a VecDeque<T>, impl <'a, T: 'a>);

delegate_from_iterator!(VecDeque<T>, T, impl <T>);

#[cfg(test)]
mod tests {
    use super::*;

    fn vd<T>(v: Vec<T>) -> VecDeque<T> {
        v.into_iter().collect()
    }

    #[test]
    fn test_c() {
        let v = vd(vec![1, 2, 3]);
        let res = v.filter(|i| i > &1);
        assert_eq!(res, vd(vec![2, 3]));
    }

    #[test]
    fn test_cc() {
        let v = vd(vec![1, 2, 3]);
        let res = v.map(|i| i.to_string());
        assert_eq!(res, vd(vec!["1".to_string(), "2".to_string(), "3".to_string()]));
    }

    #[test]
    fn test_c_r() {
        let v = vd(vec![1, 2, 3]);
        let res = (&v).filter(|i| i > &&1);
        assert_eq!(res, vd(vec![&2, &3]));
    }

    #[test]
    fn test_cc_r() {
        let v = vd(vec![1, 2, 3]);
        let res = (&v).map(|i| i.to_string());
        assert_eq!(res, vd(vec!["1".to_string(), "2".to_string(), "3".to_string()]));
    }
}
