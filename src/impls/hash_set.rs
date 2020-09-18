use std::collections::HashSet;

use crate::Iterable;

impl<T> Iterable for HashSet<T> {
    type C = Self;
    type CC<U> = HashSet<U>;
    type CR<'a> where T: 'a = HashSet<&'a T>;
}

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
