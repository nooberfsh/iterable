use crate::Iterable;

impl<'a, T: 'a> Iterable for &'a [T] {
    type C = Vec<&'a T>;
    type CC<U> = Vec<U>;
    type CR<'b> where T: 'b = Vec<&'b T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        let v: &[i32] = &[1, 2, 3];
        let res = v.filter(|i| i > &&1);
        assert_eq!(res, vec![&2, &3]);
    }

    #[test]
    fn test_cc() {
        let v: &[i32] = &[1, 2, 3];
        let res = v.map(|i| i + 1);
        assert_eq!(res, vec![2, 3, 4]);
    }

    #[test]
    fn test_c_r() {
        let v: &[i32] = &[1, 2, 3];
        let res = (&v).filter(|i| i > &&1);
        assert_eq!(res, vec![&2, &3]);
    }

    #[test]
    fn test_cc_r() {
        let v: &[i32] = &[1, 2, 3];
        let res = (&v).map(|i| i + 1);
        assert_eq!(res, vec![2, 3, 4]);
    }
}
