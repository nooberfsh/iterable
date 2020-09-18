use crate::Iterable;

impl<T> Iterable for Vec<T> {
    type C = Self;
    type CC<U> = Vec<U>;
    type CR<'a> where T: 'a = Vec<&'a T>;
}
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